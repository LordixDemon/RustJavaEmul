use alloc::{boxed::Box, format, string::String, vec::Vec};

use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{JavaError, JavaValue, Jvm};

use crate::method::{MethodBody, MethodImpl};

pub struct JavaClassProto<C>
where
    C: ?Sized + Send,
{
    pub name: &'static str,
    pub parent_class: Option<&'static str>,
    pub interfaces: Vec<&'static str>,
    pub methods: Vec<JavaMethodProto<C>>,
    pub fields: Vec<JavaFieldProto>,
    pub access_flags: ClassAccessFlags,
}

pub struct JavaFieldProto {
    pub name: String,
    pub descriptor: String,
    pub access_flags: FieldAccessFlags,
}

impl JavaFieldProto {
    pub fn new(name: &str, descriptor: &str, access_flag: FieldAccessFlags) -> Self {
        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            access_flags: access_flag,
        }
    }
}

pub struct JavaMethodProto<C>
where
    C: ?Sized + Send,
{
    pub name: String,
    pub descriptor: String,
    pub body: Box<dyn MethodBody<JavaError, C>>,
    pub access_flags: MethodAccessFlags,
}

impl<C> JavaMethodProto<C>
where
    C: ?Sized + Send,
{
    pub fn new<M, F, R, P>(name: &str, descriptor: &str, method: M, flag: MethodAccessFlags) -> Self
    where
        M: MethodImpl<F, C, R, JavaError, P>,
    {
        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            body: method.into_body(),
            access_flags: flag,
        }
    }

    pub fn new_abstract(name: &str, descriptor: &str, flag: MethodAccessFlags) -> Self {
        struct AbstractCall {
            name: String,
            descriptor: String,
        }

        #[async_trait::async_trait]
        impl<C> MethodBody<JavaError, C> for AbstractCall
        where
            C: ?Sized + Send,
        {
            async fn call(&self, jvm: &Jvm, _: &mut C, _: Box<[JavaValue]>) -> Result<JavaValue, JavaError> {
                Err(jvm
                    .exception(
                        "java/lang/AbstractMethodError",
                        &format!("Abstract {}{} method called", self.name, self.descriptor),
                    )
                    .await)
            }
        }

        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            body: Box::new(AbstractCall {
                name: name.into(),
                descriptor: descriptor.into(),
            }),
            access_flags: flag,
        }
    }

    pub fn new_stub(name: &str, descriptor: &str, flag: MethodAccessFlags) -> Self {
        struct StubCall {
            descriptor: String,
        }

        #[async_trait::async_trait]
        impl<C> MethodBody<JavaError, C> for StubCall
        where
            C: ?Sized + Send,
        {
            async fn call(&self, jvm: &Jvm, _: &mut C, _: Box<[JavaValue]>) -> Result<JavaValue, JavaError> {
                let ret = self.descriptor.rsplit_once(')').map(|(_, r)| r).unwrap_or("V");
                match ret.as_bytes().first().copied() {
                    Some(b'V') => Ok(JavaValue::Void),
                    Some(b'Z') => Ok(JavaValue::Boolean(false)),
                    Some(b'B') => Ok(JavaValue::Byte(0)),
                    Some(b'C') => Ok(JavaValue::Char(0)),
                    Some(b'S') => Ok(JavaValue::Short(0)),
                    Some(b'I') => Ok(JavaValue::Int(0)),
                    Some(b'J') => Ok(JavaValue::Long(0)),
                    Some(b'F') => Ok(JavaValue::Float(0.0)),
                    Some(b'D') => Ok(JavaValue::Double(0.0)),
                    Some(b'[') => {
                        let elem_type = &ret[1..];
                        if let Ok(array) = jvm.instantiate_array(elem_type, 0).await {
                            Ok(JavaValue::Object(Some(array)))
                        } else {
                            Ok(JavaValue::Object(None))
                        }
                    }
                    Some(b'L') => {
                        let class_name = &ret[1..ret.len().saturating_sub(1)];
                        if class_name == "java/lang/String" {
                            if let Ok(s) = jvm::runtime::JavaLangString::from_rust_string(jvm, "").await {
                                Ok(JavaValue::Object(Some(s)))
                            } else {
                                Ok(JavaValue::Object(None))
                            }
                        } else {
                            Ok(JavaValue::Object(None))
                        }
                    }
                    _ => Ok(JavaValue::Object(None)),
                }
            }
        }

        Self {
            name: name.into(),
            descriptor: descriptor.into(),
            body: Box::new(StubCall {
                descriptor: descriptor.into(),
            }),
            access_flags: flag,
        }
    }
}
