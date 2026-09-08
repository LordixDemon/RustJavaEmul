#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

impl AlertType {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/AlertType",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("playSound", "(Ljavax/microedition/lcdui/Display;)Z", Self::play_sound, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "INFO",
                    "Ljavax/microedition/lcdui/AlertType;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "WARNING",
                    "Ljavax/microedition/lcdui/AlertType;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "ERROR",
                    "Ljavax/microedition/lcdui/AlertType;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "ALARM",
                    "Ljavax/microedition/lcdui/AlertType;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "CONFIRMATION",
                    "Ljavax/microedition/lcdui/AlertType;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        for name in ["INFO", "WARNING", "ERROR", "ALARM", "CONFIRMATION"] {
            let value = jvm.new_class("javax/microedition/lcdui/AlertType", "()V", ()).await?;
            jvm.put_static_field("javax/microedition/lcdui/AlertType", name, "Ljavax/microedition/lcdui/AlertType;", value)
                .await?;
        }
        Ok(())
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn play_sound(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<Object>,
    ) -> Result<bool> {
        tracing::debug!("javax.microedition.lcdui.AlertType::playSound({this:?})");
        Ok(true)
    }
}
