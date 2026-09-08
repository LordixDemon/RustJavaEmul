#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl Light {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/game/Light",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("setLightOn", "()V", Self::on, MethodAccessFlags::STATIC),
                JavaMethodProto::new("setLightOff", "()V", Self::off, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn on(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        context.set_lights(0, 100);
        Ok(())
    }

    pub(super) async fn off(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        context.set_lights(0, 0);
        Ok(())
    }
}

impl Vibrator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/game/Vibrator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("triggerVibrator", "(I)V", Self::trigger, MethodAccessFlags::STATIC),
                JavaMethodProto::new("stopVibrator", "()V", Self::stop, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn trigger(_: &Jvm, context: &mut RuntimeContext, duration: i32) -> Result<()> {
        context.start_vibra(100, duration as i64);
        Ok(())
    }

    pub(super) async fn stop(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        context.stop_vibra();
        Ok(())
    }
}

impl Sound {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/game/Sound",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("playTone", "(II)V", Self::play_tone, MethodAccessFlags::STATIC),
                JavaMethodProto::new("stopTone", "()V", Self::stop, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn play_tone(_: &Jvm, context: &mut RuntimeContext, freq: i32, duration: i32) -> Result<()> {
        context.play_tone(freq, duration, 100);
        Ok(())
    }

    pub(super) async fn stop(_: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        Ok(())
    }
}

impl Melody {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/game/Melody",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init, Default::default()),
                JavaMethodProto::new("play", "()V", Self::play, Default::default()),
                JavaMethodProto::new("stop", "()V", Self::stop, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, _data: ClassInstanceRef<Array<i8>>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn play(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    pub(super) async fn stop(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
}

impl GraphicObject {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/game/GraphicObject",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("setVisible", "(Z)V", Self::set_visible, Default::default()),
                JavaMethodProto::new("getVisible", "()Z", Self::get_visible, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("visible", "Z", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "visible", "Z", true).await
    }

    pub(super) async fn set_visible(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, visible: bool) -> Result<()> {
        jvm.put_field(&mut this, "visible", "Z", visible).await
    }

    pub(super) async fn get_visible(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "visible", "Z").await
    }
}

impl Sprite {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/game/Sprite",
            parent_class: Some("com/siemens/mp/game/GraphicObject"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([BII)V", Self::init, Default::default()),
                JavaMethodProto::new("setPosition", "(II)V", Self::set_position, Default::default()),
                JavaMethodProto::new("getXPosition", "()I", Self::get_x, Default::default()),
                JavaMethodProto::new("getYPosition", "()I", Self::get_y, Default::default()),
                JavaMethodProto::new("setFrame", "(I)V", Self::set_frame, Default::default()),
                JavaMethodProto::new("getFrame", "()I", Self::get_frame, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("x", "I", Default::default()),
                JavaFieldProto::new("y", "I", Default::default()),
                JavaFieldProto::new("frame", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        _pixels: ClassInstanceRef<Array<i8>>,
        _w: i32,
        _h: i32,
    ) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "com/siemens/mp/game/GraphicObject", "<init>", "()V", ())
            .await?;
        jvm.put_field(&mut this, "x", "I", 0).await?;
        jvm.put_field(&mut this, "y", "I", 0).await?;
        jvm.put_field(&mut this, "frame", "I", 0).await
    }

    pub(super) async fn set_position(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        jvm.put_field(&mut this, "x", "I", x).await?;
        jvm.put_field(&mut this, "y", "I", y).await
    }

    pub(super) async fn get_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "x", "I").await
    }

    pub(super) async fn get_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "y", "I").await
    }

    pub(super) async fn set_frame(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, frame: i32) -> Result<()> {
        jvm.put_field(&mut this, "frame", "I", frame).await
    }

    pub(super) async fn get_frame(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "frame", "I").await
    }
}
