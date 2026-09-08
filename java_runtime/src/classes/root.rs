use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

pub struct MIDPCanvas;
pub struct ConstsDefines;
pub struct ConstsDefinesHUD;
pub struct ConstsDefinesTitanCinematicSettings;

impl MIDPCanvas {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "MIDPCanvas",
            parent_class: Some("javax/microedition/lcdui/Canvas"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("pointerPressed", "(II)V", Self::pointer_pressed, Default::default()),
                JavaMethodProto::new("pointerReleased", "(II)V", Self::pointer_released, Default::default()),
                JavaMethodProto::new("pointerDragged", "(II)V", Self::pointer_dragged, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/lcdui/Canvas", "<init>", "()V", ()).await
    }

    async fn pointer_pressed(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: i32) -> Result<()> {
        Ok(())
    }

    async fn pointer_released(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: i32) -> Result<()> {
        Ok(())
    }

    async fn pointer_dragged(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: i32) -> Result<()> {
        Ok(())
    }
}

impl ConstsDefines {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ConstsDefines",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
}

impl ConstsDefinesHUD {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ConstsDefines$HUD",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
}

impl ConstsDefinesTitanCinematicSettings {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ConstsDefines$TitanCinematicSettings",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
}

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    vec![
        MIDPCanvas::as_proto,
        ConstsDefines::as_proto,
        ConstsDefinesHUD::as_proto,
        ConstsDefinesTitanCinematicSettings::as_proto,
    ]
}
