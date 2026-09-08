use alloc::{format, string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.Long
pub struct Long;

impl Long {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Long",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(J)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, Default::default()),
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, Default::default()),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, Default::default()),
                JavaMethodProto::new("intValue", "()I", Self::int_value, Default::default()),
                JavaMethodProto::new("longValue", "()J", Self::long_value, Default::default()),
                JavaMethodProto::new("floatValue", "()F", Self::float_value, Default::default()),
                JavaMethodProto::new("doubleValue", "()D", Self::double_value, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("toString", "(J)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toString", "(JI)Ljava/lang/String;", Self::to_string_radix, MethodAccessFlags::STATIC),
                JavaMethodProto::new("parseLong", "(Ljava/lang/String;)J", Self::parse_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("parseLong", "(Ljava/lang/String;I)J", Self::parse_long_radix, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(J)Ljava/lang/Long;", Self::value_of, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Long;",
                    Self::value_of_string,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("value", "J", Default::default()),
                JavaFieldProto::new("MIN_VALUE", "J", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MAX_VALUE", "J", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Long", "MIN_VALUE", "J", i64::MIN).await?;
        jvm.put_static_field("java/lang/Long", "MAX_VALUE", "J", i64::MAX).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "J", value).await
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as i8)
    }
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as i16)
    }
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as i32)
    }
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        jvm.get_field(&this, "value", "J").await
    }
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as f32)
    }
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as f64)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        if other.is_null() || other.class_definition().name() != "java/lang/Long" {
            return Ok(false);
        }
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? == jvm.get_field::<i64>(&other, "value", "J").await?)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: i64 = jvm.get_field(&this, "value", "J").await?;
        Ok((value ^ (value >> 32)) as i32)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i64 = jvm.get_field(&this, "value", "J").await?;
        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn to_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: i64, radix: i32) -> Result<ClassInstanceRef<String>> {
        let radix = radix.clamp(2, 36) as u32;
        let text = if value < 0 {
            format!("-{}", format_radix((-value) as u64, radix))
        } else {
            format_radix(value as u64, radix)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }

    async fn parse_long(jvm: &Jvm, context: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<i64> {
        Self::parse_long_radix(jvm, context, s, 10).await
    }

    async fn parse_long_radix(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>, radix: i32) -> Result<i64> {
        if s.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        if !(2..=36).contains(&radix) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "radix out of range").await);
        }
        let rust = JavaLangString::to_rust_string(jvm, &s).await?;
        match i64::from_str_radix(&rust, radix as u32) {
            Ok(v) => Ok(v),
            Err(_) => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{rust}\""))
                .await),
        }
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Long", "(J)V", (value,)).await?.into())
    }

    async fn init_string(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()> {
        let value = Self::parse_long(jvm, context, s).await?;
        Self::init(jvm, context, this, value).await
    }

    async fn value_of_string(jvm: &Jvm, context: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let value = Self::parse_long(jvm, context, s).await?;
        Self::value_of(jvm, context, value).await
    }
}

fn format_radix(mut value: u64, radix: u32) -> alloc::string::String {
    if value == 0 {
        return "0".into();
    }
    let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut buf = alloc::vec::Vec::new();
    while value > 0 {
        buf.push(digits[(value % radix as u64) as usize]);
        value /= radix as u64;
    }
    buf.reverse();
    alloc::string::String::from_utf8(buf).unwrap()
}
