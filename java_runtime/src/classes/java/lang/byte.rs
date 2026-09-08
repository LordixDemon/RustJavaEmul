use alloc::{format, string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.Byte
pub struct Byte;

impl Byte {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Byte",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(B)V", Self::init, Default::default()),
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, Default::default()),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, Default::default()),
                JavaMethodProto::new("intValue", "()I", Self::int_value, Default::default()),
                JavaMethodProto::new("longValue", "()J", Self::long_value, Default::default()),
                JavaMethodProto::new("floatValue", "()F", Self::float_value, Default::default()),
                JavaMethodProto::new("doubleValue", "()D", Self::double_value, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("toString", "(B)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
                JavaMethodProto::new("parseByte", "(Ljava/lang/String;)B", Self::parse_byte, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(B)Ljava/lang/Byte;", Self::value_of, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("value", "B", Default::default()),
                JavaFieldProto::new("MIN_VALUE", "B", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MAX_VALUE", "B", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE", "Ljava/lang/Class;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Byte", "MIN_VALUE", "B", i8::MIN).await?;
        jvm.put_static_field("java/lang/Byte", "MAX_VALUE", "B", i8::MAX).await?;
        let class = jvm.resolve_class("java/lang/Byte").await?;
        jvm.put_static_field("java/lang/Byte", "TYPE", "Ljava/lang/Class;", class.java_class())
            .await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i8) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "B", value).await
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        jvm.get_field(&this, "value", "B").await
    }
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        Ok(jvm.get_field::<i8>(&this, "value", "B").await? as i16)
    }
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i8>(&this, "value", "B").await? as i32)
    }
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(jvm.get_field::<i8>(&this, "value", "B").await? as i64)
    }
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(jvm.get_field::<i8>(&this, "value", "B").await? as f32)
    }
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        Ok(jvm.get_field::<i8>(&this, "value", "B").await? as f64)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        if other.is_null() || other.class_definition().name() != "java/lang/Byte" {
            return Ok(false);
        }
        Ok(jvm.get_field::<i8>(&this, "value", "B").await? == jvm.get_field::<i8>(&other, "value", "B").await?)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i8>(&this, "value", "B").await? as i32)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i8 = jvm.get_field(&this, "value", "B").await?;
        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i8) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn parse_byte(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<i8> {
        if s.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let rust = JavaLangString::to_rust_string(jvm, &s).await?;
        match rust.parse::<i8>() {
            Ok(v) => Ok(v),
            Err(_) => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{rust}\""))
                .await),
        }
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i8) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Byte", "(B)V", (value,)).await?.into())
    }
}
