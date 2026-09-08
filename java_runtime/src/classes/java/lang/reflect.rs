use alloc::{boxed::Box, format, string::String as RustString, vec, vec::Vec};

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangClass};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Class, Object},
};

pub struct Array;

impl Array {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/reflect/Array",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "newInstance",
                "(Ljava/lang/Class;[I)Ljava/lang/Object;",
                Self::new_instance,
                MethodAccessFlags::STATIC,
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn new_instance(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        component: ClassInstanceRef<Class>,
        dimensions: ClassInstanceRef<jvm::Array<i32>>,
    ) -> Result<ClassInstanceRef<Object>> {
        if component.is_null() || dimensions.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Array.newInstance").await);
        }
        let dim_len = jvm.array_length(&dimensions).await?;
        if dim_len == 0 || dim_len > 255 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid dimensions").await);
        }
        let dims: Vec<i32> = jvm.load_array(&dimensions, 0, dim_len).await?;
        if dims.iter().any(|d| *d < 0) {
            return Err(jvm.exception("java/lang/NegativeArraySizeException", "").await);
        }
        let rust_class = JavaLangClass::to_rust_class(jvm, &component).await?;
        let element = component_descriptor(&rust_class.name());
        let array = create_nd_array(jvm, &element, &dims).await?;
        Ok(array.into())
    }
}

fn component_descriptor(name: &str) -> RustString {
    match name {
        "java/lang/Byte" | "byte" | "B" => "B".into(),
        "java/lang/Character" | "char" | "C" => "C".into(),
        "java/lang/Short" | "short" | "S" => "S".into(),
        "java/lang/Integer" | "int" | "I" => "I".into(),
        "java/lang/Long" | "long" | "J" => "J".into(),
        "java/lang/Float" | "float" | "F" => "F".into(),
        "java/lang/Double" | "double" | "D" => "D".into(),
        "java/lang/Boolean" | "boolean" | "Z" => "Z".into(),
        other if other.starts_with('[') => other.into(),
        other => format!("L{other};"),
    }
}

#[async_recursion::async_recursion]
async fn create_nd_array(jvm: &Jvm, element: &str, dims: &[i32]) -> Result<alloc::boxed::Box<dyn jvm::ClassInstance>> {
    let length = dims[0] as usize;
    let nested = "[".repeat(dims.len() - 1);
    let component = format!("{nested}{element}");
    let mut array = jvm.instantiate_array(&component, length).await?;
    if dims.len() > 1 {
        for i in 0..length {
            let inner = create_nd_array(jvm, element, &dims[1..]).await?;
            jvm.store_array(&mut array, i, vec![inner]).await?;
        }
    }
    Ok(array)
}
