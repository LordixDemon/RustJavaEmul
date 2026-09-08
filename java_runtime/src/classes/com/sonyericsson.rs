use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

pub struct Device;

impl Device {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/sonyericsson/device/Device",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("vibrate", "(I)V", Self::vibrate, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_property,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn vibrate(_: &Jvm, context: &mut RuntimeContext, duration: i32) -> Result<()> {
        context.start_vibra(100, duration as i64);
        Ok(())
    }
    async fn get_property(jvm: &Jvm, _: &mut RuntimeContext, key: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        jvm.invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await
    }
}

pub mod device {
    pub use super::Device;
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![Device]
}
