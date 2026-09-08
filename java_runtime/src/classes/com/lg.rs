use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

pub struct Vibration;

impl Vibration {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/lg/util/Vibration",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("startVibra", "(I)V", Self::start, MethodAccessFlags::STATIC),
                JavaMethodProto::new("stopVibra", "()V", Self::stop, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn start(_: &Jvm, context: &mut RuntimeContext, duration: i32) -> Result<()> {
        context.start_vibra(100, duration as i64);
        Ok(())
    }
    async fn stop(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        context.stop_vibra();
        Ok(())
    }
}

pub mod util {
    pub use super::Vibration;
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![Vibration]
}
