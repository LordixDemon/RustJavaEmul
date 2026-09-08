use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};

use crate::RuntimeClassProto;

// abstract class java.lang.Number
pub struct Number;

impl Number {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Number",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("intValue", "()I", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("longValue", "()J", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("floatValue", "()F", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("doubleValue", "()D", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, Default::default()),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &jvm::Jvm, _: &mut crate::RuntimeContext, this: jvm::ClassInstanceRef<Self>) -> jvm::Result<()> {
        tracing::debug!("java.lang.Number::<init>({this:?})");
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Ok(())
    }

    async fn byte_value(jvm: &jvm::Jvm, _: &mut crate::RuntimeContext, this: jvm::ClassInstanceRef<Self>) -> jvm::Result<i8> {
        let value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        Ok(value as i8)
    }

    async fn short_value(jvm: &jvm::Jvm, _: &mut crate::RuntimeContext, this: jvm::ClassInstanceRef<Self>) -> jvm::Result<i16> {
        let value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        Ok(value as i16)
    }
}
