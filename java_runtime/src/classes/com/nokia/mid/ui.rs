use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::javax::microedition::lcdui::Graphics};

// class com.nokia.mid.ui.DirectGraphics
pub struct DirectGraphics;

impl DirectGraphics {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/nokia/mid/ui/DirectGraphics",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("setARGBColor", "(I)V", Self::set_argb_color, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("argbColor", "I", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("com.nokia.mid.ui.DirectGraphics::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn set_argb_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, argb: i32) -> Result<()> {
        tracing::trace!("com.nokia.mid.ui.DirectGraphics::setARGBColor({this:?}, {argb:#x})");

        jvm.put_field(&mut this, "argbColor", "I", argb).await?;

        Ok(())
    }
}

// class com.nokia.mid.ui.DirectUtils
pub struct DirectUtils;

impl DirectUtils {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/nokia/mid/ui/DirectUtils",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "getDirectGraphics",
                "(Ljavax/microedition/lcdui/Graphics;)Lcom/nokia/mid/ui/DirectGraphics;",
                Self::get_direct_graphics,
                MethodAccessFlags::STATIC,
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn get_direct_graphics(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        graphics: ClassInstanceRef<Graphics>,
    ) -> Result<ClassInstanceRef<DirectGraphics>> {
        tracing::debug!("com.nokia.mid.ui.DirectUtils::getDirectGraphics({graphics:?})");

        Ok(jvm.new_class("com/nokia/mid/ui/DirectGraphics", "()V", ()).await?.into())
    }
}

// class com.nokia.mid.ui.DeviceControl
pub struct DeviceControl;

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

    async fn flash_lights(_: &Jvm, _: &mut RuntimeContext, duration_ms: i64) -> Result<()> {
        tracing::trace!("com.nokia.mid.ui.DeviceControl::flashLights({duration_ms})");
        Ok(())
    }

    async fn set_lights(_: &Jvm, _: &mut RuntimeContext, num: i32, level: i32) -> Result<()> {
        tracing::trace!("com.nokia.mid.ui.DeviceControl::setLights({num}, {level})");
        Ok(())
    }

    async fn start_vibra(_: &Jvm, _: &mut RuntimeContext, freq: i32, duration_ms: i64) -> Result<()> {
        tracing::trace!("com.nokia.mid.ui.DeviceControl::startVibra({freq}, {duration_ms})");
        Ok(())
    }

    async fn stop_vibra(_: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::trace!("com.nokia.mid.ui.DeviceControl::stopVibra()");
        Ok(())
    }
}
