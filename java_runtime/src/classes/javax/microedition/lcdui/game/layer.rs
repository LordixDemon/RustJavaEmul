#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

impl Layer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/game/Layer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getX", "()I", Self::get_x, Default::default()),
                JavaMethodProto::new("getY", "()I", Self::get_y, Default::default()),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new("isVisible", "()Z", Self::is_visible, Default::default()),
                JavaMethodProto::new("setVisible", "(Z)V", Self::set_visible, Default::default()),
                JavaMethodProto::new("setPosition", "(II)V", Self::set_position, Default::default()),
                JavaMethodProto::new("move", "(II)V", Self::r#move, Default::default()),
                JavaMethodProto::new_abstract(
                    "paint",
                    "(Ljavax/microedition/lcdui/Graphics;)V",
                    java_constants::MethodAccessFlags::ABSTRACT,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("x", "I", Default::default()),
                JavaFieldProto::new("y", "I", Default::default()),
                JavaFieldProto::new("width", "I", Default::default()),
                JavaFieldProto::new("height", "I", Default::default()),
                JavaFieldProto::new("visible", "Z", Default::default()),
            ],
            access_flags: java_constants::ClassAccessFlags::ABSTRACT,
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "visible", "Z", true).await
    }

    pub(super) async fn get_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "x", "I").await
    }
    pub(super) async fn get_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "y", "I").await
    }
    pub(super) async fn get_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "width", "I").await
    }
    pub(super) async fn get_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "height", "I").await
    }
    pub(super) async fn is_visible(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "visible", "Z").await
    }
    pub(super) async fn set_visible(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, visible: bool) -> Result<()> {
        jvm.put_field(&mut this, "visible", "Z", visible).await
    }
    pub(super) async fn set_position(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        jvm.put_field(&mut this, "x", "I", x).await?;
        jvm.put_field(&mut this, "y", "I", y).await
    }
    pub(super) async fn r#move(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, dx: i32, dy: i32) -> Result<()> {
        let x: i32 = jvm.get_field(&this, "x", "I").await?;
        let y: i32 = jvm.get_field(&this, "y", "I").await?;
        jvm.put_field(&mut this, "x", "I", x + dx).await?;
        jvm.put_field(&mut this, "y", "I", y + dy).await
    }
}
