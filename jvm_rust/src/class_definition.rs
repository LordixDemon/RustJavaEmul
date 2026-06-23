use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

use hashbrown::{Equivalent, HashMap};
use parking_lot::RwLock;

use classfile::{AttributeInfo, ClassInfo, ConstantPoolReference};
use java_class_proto::JavaClassProto;
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassDefinition, ClassInstance, Field, JavaType, JavaValue, Jvm, Method, Result, runtime::JavaLangString};

use crate::{class_instance::ClassInstanceImpl, field::FieldImpl, method::MethodImpl};

#[derive(Eq, PartialEq, Hash)]
struct MemberKey {
    name: String,
    descriptor: String,
    is_static: bool,
}

struct MemberLookup<'a> {
    name: &'a str,
    descriptor: &'a str,
    is_static: bool,
}

impl Hash for MemberLookup<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.descriptor.hash(state);
        self.is_static.hash(state);
    }
}

impl Equivalent<MemberKey> for MemberLookup<'_> {
    fn equivalent(&self, key: &MemberKey) -> bool {
        self.name == key.name && self.descriptor == key.descriptor && self.is_static == key.is_static
    }
}

struct ClassDefinitionInner {
    name: String,
    super_class_name: Option<String>,
    interfaces: Vec<String>,
    access_flags: ClassAccessFlags,
    method_lookup: HashMap<MemberKey, MethodImpl>,
    fields: Vec<FieldImpl>,
    field_lookup: HashMap<MemberKey, FieldImpl>,
    constant_values: Vec<(FieldImpl, ConstantPoolReference)>,
    storage: RwLock<HashMap<FieldImpl, JavaValue>>, // TODO we should use field offset or something
}

#[derive(Clone)]
pub struct ClassDefinitionImpl {
    inner: Arc<ClassDefinitionInner>,
}

impl ClassDefinitionImpl {
    pub fn new(
        name: &str,
        super_class_name: Option<String>,
        interfaces: Vec<String>,
        access_flags: ClassAccessFlags,
        methods: Vec<MethodImpl>,
        fields: Vec<FieldImpl>,
    ) -> Self {
        Self::with_constant_values(name, super_class_name, interfaces, access_flags, methods, fields, Vec::new())
    }

    #[allow(clippy::too_many_arguments)]
    fn with_constant_values(
        name: &str,
        super_class_name: Option<String>,
        interfaces: Vec<String>,
        access_flags: ClassAccessFlags,
        methods: Vec<MethodImpl>,
        fields: Vec<FieldImpl>,
        constant_values: Vec<(FieldImpl, ConstantPoolReference)>,
    ) -> Self {
        let method_lookup = methods
            .iter()
            .map(|method| {
                (
                    MemberKey {
                        name: method.name(),
                        descriptor: method.descriptor(),
                        is_static: method.access_flags().contains(MethodAccessFlags::STATIC),
                    },
                    method.clone(),
                )
            })
            .collect();
        let field_lookup = fields
            .iter()
            .map(|field| {
                (
                    MemberKey {
                        name: field.name(),
                        descriptor: field.descriptor(),
                        is_static: field.access_flags().contains(FieldAccessFlags::STATIC),
                    },
                    field.clone(),
                )
            })
            .collect();

        Self {
            inner: Arc::new(ClassDefinitionInner {
                name: name.to_string(),
                super_class_name,
                interfaces,
                access_flags,
                method_lookup,
                fields,
                field_lookup,
                constant_values,
                storage: RwLock::new(HashMap::new()),
            }),
        }
    }

    pub fn from_class_proto<C, Context>(proto: JavaClassProto<C>, context: Context) -> Self
    where
        C: ?Sized + 'static + Send,
        Context: Sync + Send + DerefMut + Deref<Target = C> + Clone + 'static,
    {
        let methods = proto
            .methods
            .into_iter()
            .map(|x| MethodImpl::from_method_proto_with_owner(proto.name, x, context.clone()))
            .collect::<Vec<_>>();

        let fields = proto.fields.into_iter().map(FieldImpl::from_field_proto).collect::<Vec<_>>();

        let interfaces = proto.interfaces.into_iter().map(|x| x.to_string()).collect();

        Self::new(
            proto.name,
            proto.parent_class.map(|x| x.to_string()),
            interfaces,
            proto.access_flags,
            methods,
            fields,
        )
    }

    pub fn from_classfile(data: &[u8]) -> Result<Self> {
        let class = ClassInfo::parse(data).unwrap(); // TODO ClassFormatError
        assert_eq!(class.magic, 0xCAFEBABE);

        let mut constant_values = Vec::new();
        let fields = class
            .fields
            .into_iter()
            .map(|field_info| {
                let constant = field_info.attributes.iter().find_map(|x| match x {
                    AttributeInfo::ConstantValue(value) => Some(value.clone()),
                    _ => None,
                });

                let field = FieldImpl::from_field_info(field_info);
                if let Some(x) = constant
                    && field.access_flags().contains(FieldAccessFlags::STATIC)
                {
                    constant_values.push((field.clone(), x));
                }

                field
            })
            .collect::<Vec<_>>();

        let methods = class
            .methods
            .into_iter()
            .map(|method| MethodImpl::from_method_info_with_owner(&class.this_class, method))
            .collect::<Vec<_>>();

        let interfaces = class.interfaces.into_iter().map(|x| x.to_string()).collect();

        Ok(Self::with_constant_values(
            &class.this_class,
            class.super_class.map(|x| x.to_string()),
            interfaces,
            class.access_flags,
            methods,
            fields,
            constant_values,
        ))
    }

    pub fn fields(&self) -> &[FieldImpl] {
        &self.inner.fields
    }
}

#[async_trait::async_trait]
impl ClassDefinition for ClassDefinitionImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn super_class_name(&self) -> Option<String> {
        self.inner.super_class_name.as_ref().map(|x| x.to_string())
    }

    fn interface_names(&self) -> Vec<String> {
        self.inner.interfaces.clone()
    }

    fn access_flags(&self) -> ClassAccessFlags {
        self.inner.access_flags
    }

    async fn instantiate(&self, _: &Jvm) -> Result<Box<dyn ClassInstance>> {
        Ok(Box::new(ClassInstanceImpl::new(self)))
    }

    async fn prepare(&self, jvm: &Jvm) -> Result<()> {
        for (field, constant) in &self.inner.constant_values {
            let value = match constant {
                ConstantPoolReference::Integer(x) => match field.descriptor().as_str() {
                    "Z" => JavaValue::Boolean(*x != 0),
                    "B" => JavaValue::Byte(*x as i8),
                    "C" => JavaValue::Char(*x as u16),
                    "S" => JavaValue::Short(*x as i16),
                    _ => JavaValue::Int(*x),
                },
                ConstantPoolReference::Long(x) => JavaValue::Long(*x),
                ConstantPoolReference::Float(x) => JavaValue::Float(*x),
                ConstantPoolReference::Double(x) => JavaValue::Double(*x),
                ConstantPoolReference::String(x) => JavaValue::Object(Some(JavaLangString::from_rust_string(jvm, x).await?)),
                _ => continue,
            };

            self.inner.storage.write().insert(field.clone(), value);
        }

        Ok(())
    }

    fn method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Box<dyn Method>> {
        self.inner
            .method_lookup
            .get(&MemberLookup { name, descriptor, is_static })
            .map(|x| Box::new(x.clone()) as Box<dyn Method>)
    }

    fn field(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Box<dyn Field>> {
        self.inner
            .field_lookup
            .get(&MemberLookup { name, descriptor, is_static })
            .map(|x| Box::new(x.clone()) as Box<dyn Field>)
    }

    fn fields(&self) -> Vec<Box<dyn Field>> {
        self.inner.fields.iter().map(|x| Box::new(x.clone()) as Box<dyn Field>).collect()
    }

    fn get_static_field(&self, field: &dyn Field) -> Result<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        let storage = self.inner.storage.read();
        let value = storage.get(field);

        if let Some(x) = value {
            Ok(x.clone())
        } else {
            Ok(JavaType::parse(&field.descriptor()).default())
        }
    }

    fn put_static_field(&mut self, field: &dyn Field, value: JavaValue) -> Result<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.inner.storage.write().insert(field.clone(), value);

        Ok(())
    }
}

impl Debug for ClassDefinitionImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Class({})", self.name())
    }
}
