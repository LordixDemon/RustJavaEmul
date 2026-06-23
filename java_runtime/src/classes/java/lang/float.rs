use alloc::vec;

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
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(F)V", Self::init, Default::default()),
                JavaMethodProto::new("floatValue", "()F", Self::float_value, Default::default()),
                JavaMethodProto::new("floatToIntBits", "(F)I", Self::float_to_int_bits, MethodAccessFlags::STATIC),
                JavaMethodProto::new("intBitsToFloat", "(I)F", Self::int_bits_to_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("isNaN", "(F)Z", Self::is_nan, MethodAccessFlags::STATIC),
            ],
            fields: vec![JavaFieldProto::new("value", "F", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        tracing::debug!("java.lang.Float::<init>({this:?}, {value:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "F", value).await
    }

    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        tracing::debug!("java.lang.Float::floatValue({this:?})");

        jvm.get_field(&this, "value", "F").await
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
