#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{Jvm, Result};

impl DeviceControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/nokia/mid/ui/DeviceControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("flashLights", "(J)V", Self::flash_lights, MethodAccessFlags::STATIC),
                JavaMethodProto::new("setLights", "(II)V", Self::set_lights, MethodAccessFlags::STATIC),
                JavaMethodProto::new("startVibra", "(IJ)V", Self::start_vibra, MethodAccessFlags::STATIC),
                JavaMethodProto::new("stopVibra", "()V", Self::stop_vibra, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn flash_lights(_: &Jvm, context: &mut RuntimeContext, _duration_ms: i64) -> Result<()> {
        context.set_lights(0, 100);
        Ok(())
    }

    pub(super) async fn set_lights(_: &Jvm, context: &mut RuntimeContext, num: i32, level: i32) -> Result<()> {
        context.set_lights(num, level);
        Ok(())
    }

    pub(super) async fn start_vibra(_: &Jvm, context: &mut RuntimeContext, freq: i32, duration_ms: i64) -> Result<()> {
        context.start_vibra(freq, duration_ms);
        Ok(())
    }

    pub(super) async fn stop_vibra(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        context.stop_vibra();
        Ok(())
    }
}
