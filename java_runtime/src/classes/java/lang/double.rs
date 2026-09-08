use alloc::{format, string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.Double
pub struct Double;

impl Double {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Double",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(D)V", Self::init, Default::default()),
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, Default::default()),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, Default::default()),
                JavaMethodProto::new("intValue", "()I", Self::int_value, Default::default()),
                JavaMethodProto::new("longValue", "()J", Self::long_value, Default::default()),
                JavaMethodProto::new("floatValue", "()F", Self::float_value, Default::default()),
                JavaMethodProto::new("doubleValue", "()D", Self::double_value, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("toString", "(D)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
                JavaMethodProto::new("parseDouble", "(Ljava/lang/String;)D", Self::parse_double, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(D)Ljava/lang/Double;", Self::value_of, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Double;",
                    Self::value_of_string,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("isNaN", "()Z", Self::is_nan_instance, Default::default()),
                JavaMethodProto::new("isNaN", "(D)Z", Self::is_nan, MethodAccessFlags::STATIC),
                JavaMethodProto::new("isInfinite", "()Z", Self::is_infinite_instance, Default::default()),
                JavaMethodProto::new("isInfinite", "(D)Z", Self::is_infinite, MethodAccessFlags::STATIC),
                JavaMethodProto::new("doubleToLongBits", "(D)J", Self::double_to_long_bits, MethodAccessFlags::STATIC),
                JavaMethodProto::new("longBitsToDouble", "(J)D", Self::long_bits_to_double, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("value", "D", Default::default()),
                JavaFieldProto::new("NaN", "D", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("POSITIVE_INFINITY", "D", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("NEGATIVE_INFINITY", "D", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MIN_VALUE", "D", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MAX_VALUE", "D", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Double", "NaN", "D", f64::NAN).await?;
        jvm.put_static_field("java/lang/Double", "POSITIVE_INFINITY", "D", f64::INFINITY).await?;
        jvm.put_static_field("java/lang/Double", "NEGATIVE_INFINITY", "D", f64::NEG_INFINITY)
            .await?;
        jvm.put_static_field("java/lang/Double", "MIN_VALUE", "D", f64::MIN_POSITIVE).await?;
        jvm.put_static_field("java/lang/Double", "MAX_VALUE", "D", f64::MAX).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "D", value).await
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        Ok(jvm.get_field::<f64>(&this, "value", "D").await? as i8)
    }
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        Ok(jvm.get_field::<f64>(&this, "value", "D").await? as i16)
    }
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<f64>(&this, "value", "D").await? as i32)
    }
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(jvm.get_field::<f64>(&this, "value", "D").await? as i64)
    }
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(jvm.get_field::<f64>(&this, "value", "D").await? as f32)
    }
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        jvm.get_field(&this, "value", "D").await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        if other.is_null() || other.class_definition().name() != "java/lang/Double" {
            return Ok(false);
        }
        let a: f64 = jvm.get_field(&this, "value", "D").await?;
        let b: f64 = jvm.get_field(&other, "value", "D").await?;
        Ok(a.to_bits() == b.to_bits())
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let bits = jvm.get_field::<f64>(&this, "value", "D").await?.to_bits();
        Ok((bits ^ (bits >> 32)) as i32)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: f64 = jvm.get_field(&this, "value", "D").await?;
        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: f64) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn parse_double(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<f64> {
        if s.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let rust = JavaLangString::to_rust_string(jvm, &s).await?;
        match rust.trim().parse::<f64>() {
            Ok(v) => Ok(v),
            Err(_) => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{rust}\""))
                .await),
        }
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: f64) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Double", "(D)V", (value,)).await?.into())
    }

    async fn value_of_string(jvm: &Jvm, context: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let value = Self::parse_double(jvm, context, s).await?;
        Self::value_of(jvm, context, value).await
    }

    async fn is_nan_instance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(jvm.get_field::<f64>(&this, "value", "D").await?.is_nan())
    }

    async fn is_nan(_: &Jvm, _: &mut RuntimeContext, value: f64) -> Result<bool> {
        Ok(value.is_nan())
    }

    async fn is_infinite_instance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(jvm.get_field::<f64>(&this, "value", "D").await?.is_infinite())
    }

    async fn is_infinite(_: &Jvm, _: &mut RuntimeContext, value: f64) -> Result<bool> {
        Ok(value.is_infinite())
    }

    async fn double_to_long_bits(_: &Jvm, _: &mut RuntimeContext, value: f64) -> Result<i64> {
        Ok(value.to_bits() as i64)
    }

    async fn long_bits_to_double(_: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<f64> {
        Ok(f64::from_bits(value as u64))
    }
}
