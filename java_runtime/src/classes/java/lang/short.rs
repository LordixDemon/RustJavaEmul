use alloc::{format, string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.Short
pub struct Short;

impl Short {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Short",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(S)V", Self::init, Default::default()),
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, Default::default()),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, Default::default()),
                JavaMethodProto::new("intValue", "()I", Self::int_value, Default::default()),
                JavaMethodProto::new("longValue", "()J", Self::long_value, Default::default()),
                JavaMethodProto::new("floatValue", "()F", Self::float_value, Default::default()),
                JavaMethodProto::new("doubleValue", "()D", Self::double_value, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("toString", "(S)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
                JavaMethodProto::new("parseShort", "(Ljava/lang/String;)S", Self::parse_short, MethodAccessFlags::STATIC),
                JavaMethodProto::new("parseShort", "(Ljava/lang/String;I)S", Self::parse_short_radix, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(S)Ljava/lang/Short;", Self::value_of, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("value", "S", Default::default()),
                JavaFieldProto::new("MIN_VALUE", "S", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MAX_VALUE", "S", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Short", "MIN_VALUE", "S", i16::MIN).await?;
        jvm.put_static_field("java/lang/Short", "MAX_VALUE", "S", i16::MAX).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i16) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "S", value).await
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as i8)
    }
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        jvm.get_field(&this, "value", "S").await
    }
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as i32)
    }
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as i64)
    }
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as f32)
    }
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as f64)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        if other.is_null() || other.class_definition().name() != "java/lang/Short" {
            return Ok(false);
        }
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? == jvm.get_field::<i16>(&other, "value", "S").await?)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as i32)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i16 = jvm.get_field(&this, "value", "S").await?;
        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i16) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn parse_short(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<i16> {
        if s.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let rust = JavaLangString::to_rust_string(jvm, &s).await?;
        match rust.parse::<i16>() {
            Ok(v) => Ok(v),
            Err(_) => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{rust}\""))
                .await),
        }
    }

    async fn parse_short_radix(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>, radix: i32) -> Result<i16> {
        if s.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        if !(2..=36).contains(&radix) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "radix out of range").await);
        }
        let rust = JavaLangString::to_rust_string(jvm, &s).await?;
        match i16::from_str_radix(&rust, radix as u32) {
            Ok(v) => Ok(v),
            Err(_) => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{rust}\""))
                .await),
        }
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i16) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Short", "(S)V", (value,)).await?.into())
    }
}
