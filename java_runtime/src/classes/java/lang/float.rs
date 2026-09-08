use alloc::{string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.lang.Float
pub struct Float;

impl Float {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Float",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, java_constants::MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(F)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, Default::default()),
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, Default::default()),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, Default::default()),
                JavaMethodProto::new("intValue", "()I", Self::int_value, Default::default()),
                JavaMethodProto::new("longValue", "()J", Self::long_value, Default::default()),
                JavaMethodProto::new("floatValue", "()F", Self::float_value, Default::default()),
                JavaMethodProto::new("doubleValue", "()D", Self::double_value, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("floatToIntBits", "(F)I", Self::float_to_int_bits, MethodAccessFlags::STATIC),
                JavaMethodProto::new("intBitsToFloat", "(I)F", Self::int_bits_to_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("isNaN", "(F)Z", Self::is_nan, MethodAccessFlags::STATIC),
                JavaMethodProto::new("isNaN", "()Z", Self::is_nan_instance, Default::default()),
                JavaMethodProto::new("isInfinite", "(F)Z", Self::is_infinite, MethodAccessFlags::STATIC),
                JavaMethodProto::new("isInfinite", "()Z", Self::is_infinite_instance, Default::default()),
                JavaMethodProto::new("parseFloat", "(Ljava/lang/String;)F", Self::parse_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("toString", "(F)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(F)Ljava/lang/Float;", Self::value_of, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Float;",
                    Self::value_of_string,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("value", "F", Default::default()),
                JavaFieldProto::new(
                    "NaN",
                    "F",
                    java_constants::FieldAccessFlags::STATIC | java_constants::FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "POSITIVE_INFINITY",
                    "F",
                    java_constants::FieldAccessFlags::STATIC | java_constants::FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "NEGATIVE_INFINITY",
                    "F",
                    java_constants::FieldAccessFlags::STATIC | java_constants::FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MAX_VALUE",
                    "F",
                    java_constants::FieldAccessFlags::STATIC | java_constants::FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MIN_VALUE",
                    "F",
                    java_constants::FieldAccessFlags::STATIC | java_constants::FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        tracing::debug!("java.lang.Float::<init>({this:?}, {value:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "F", value).await
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Float", "NaN", "F", f32::NAN).await?;
        jvm.put_static_field("java/lang/Float", "POSITIVE_INFINITY", "F", f32::INFINITY).await?;
        jvm.put_static_field("java/lang/Float", "NEGATIVE_INFINITY", "F", f32::NEG_INFINITY)
            .await?;
        jvm.put_static_field("java/lang/Float", "MAX_VALUE", "F", f32::MAX).await?;
        jvm.put_static_field("java/lang/Float", "MIN_VALUE", "F", f32::MIN_POSITIVE).await
    }

    async fn init_string(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        s: jvm::ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<()> {
        let value = Self::parse_float(jvm, context, s).await?;
        Self::init(jvm, context, this, value).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        if other.is_null() || other.class_definition().name() != "java/lang/Float" {
            return Ok(false);
        }
        Ok(jvm.get_field::<f32>(&this, "value", "F").await? == jvm.get_field::<f32>(&other, "value", "F").await?)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<f32>(&this, "value", "F").await?.to_bits() as i32)
    }

    async fn is_nan_instance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(jvm.get_field::<f32>(&this, "value", "F").await?.is_nan())
    }

    async fn is_infinite_instance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(jvm.get_field::<f32>(&this, "value", "F").await?.is_infinite())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<jvm::ClassInstanceRef<crate::classes::java::lang::String>> {
        Ok(jvm::runtime::JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn value_of_string(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        s: jvm::ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<ClassInstanceRef<Self>> {
        let value = Self::parse_float(jvm, context, s).await?;
        Self::value_of(jvm, context, value).await
    }

    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        tracing::debug!("java.lang.Float::floatValue({this:?})");

        jvm.get_field(&this, "value", "F").await
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        Ok(jvm.get_field::<f32>(&this, "value", "F").await? as i8)
    }
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        Ok(jvm.get_field::<f32>(&this, "value", "F").await? as i16)
    }
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<f32>(&this, "value", "F").await? as i32)
    }
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(jvm.get_field::<f32>(&this, "value", "F").await? as i64)
    }
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        Ok(jvm.get_field::<f32>(&this, "value", "F").await? as f64)
    }

    async fn is_infinite(_: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<bool> {
        Ok(value.is_infinite())
    }

    async fn parse_float(jvm: &Jvm, _: &mut RuntimeContext, s: jvm::ClassInstanceRef<crate::classes::java::lang::String>) -> Result<f32> {
        let rust = jvm::runtime::JavaLangString::to_rust_string(jvm, &s).await?;
        match rust.trim().parse::<f32>() {
            Ok(v) => Ok(v),
            Err(_) => Err(jvm.exception("java/lang/NumberFormatException", &rust).await),
        }
    }

    async fn to_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<jvm::ClassInstanceRef<crate::classes::java::lang::String>> {
        let value: f32 = jvm.get_field(&this, "value", "F").await?;
        Ok(jvm::runtime::JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Float", "(F)V", (value,)).await?.into())
    }

    async fn float_to_int_bits(_: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<i32> {
        tracing::trace!("java.lang.Float::floatToIntBits({value:?})");

        Ok(value.to_bits() as i32)
    }

    async fn int_bits_to_float(_: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<f32> {
        tracing::trace!("java.lang.Float::intBitsToFloat({value:?})");

        Ok(f32::from_bits(value as u32))
    }

    async fn is_nan(_: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<bool> {
        tracing::trace!("java.lang.Float::isNaN({value:?})");

        Ok(value.is_nan())
    }
}
