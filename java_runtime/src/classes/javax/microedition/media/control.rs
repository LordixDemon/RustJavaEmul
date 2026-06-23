use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::ClassAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// interface javax.microedition.media.Control
pub struct Control;

impl Control {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/Control",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

// class javax.microedition.media.control.VolumeControl
pub struct VolumeControl;

impl VolumeControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/control/VolumeControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Control"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getLevel", "()I", Self::get_level, Default::default()),
                JavaMethodProto::new("isMuted", "()Z", Self::is_muted, Default::default()),
                JavaMethodProto::new("setLevel", "(I)I", Self::set_level, Default::default()),
                JavaMethodProto::new("setMute", "(Z)V", Self::set_mute, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("level", "I", Default::default()),
                JavaFieldProto::new("muted", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "level", "I", 100).await?;
        jvm.put_field(&mut this, "muted", "Z", false).await?;

        Ok(())
    }

    async fn get_level(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::getLevel({this:?})");

        jvm.get_field(&this, "level", "I").await
    }

    async fn is_muted(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::isMuted({this:?})");

        jvm.get_field(&this, "muted", "Z").await
    }

    async fn set_level(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, level: i32) -> Result<i32> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::setLevel({this:?}, {level:?})");

        let level = level.clamp(0, 100);
        jvm.put_field(&mut this, "level", "I", level).await?;

        Ok(level)
    }

    async fn set_mute(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, muted: bool) -> Result<()> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::setMute({this:?}, {muted:?})");

        jvm.put_field(&mut this, "muted", "Z", muted).await?;
        Ok(())
    }
}
