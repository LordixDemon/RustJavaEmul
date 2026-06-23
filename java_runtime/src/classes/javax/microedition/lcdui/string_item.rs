use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class javax.microedition.lcdui.StringItem
pub struct StringItem;

impl StringItem {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/StringItem",
            parent_class: Some("javax/microedition/lcdui/Item"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;I)V",
                    Self::init_with_appearance,
                    Default::default(),
                ),
                JavaMethodProto::new("getAppearanceMode", "()I", Self::get_appearance_mode, Default::default()),
                JavaMethodProto::new("getText", "()Ljava/lang/String;", Self::get_text, Default::default()),
                JavaMethodProto::new("setText", "(Ljava/lang/String;)V", Self::set_text, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("text", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("appearanceMode", "I", Default::default()),
                JavaFieldProto::new("PLAIN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HYPERLINK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BUTTON", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/StringItem";
        jvm.put_static_field(class, "PLAIN", "I", 0).await?;
        jvm.put_static_field(class, "HYPERLINK", "I", 1).await?;
        jvm.put_static_field(class, "BUTTON", "I", 2).await
    }

    async fn init(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        text: ClassInstanceRef<String>,
    ) -> Result<()> {
        Self::init_with_appearance(jvm, context, this, label, text, 0).await
    }

    async fn init_with_appearance(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        text: ClassInstanceRef<String>,
        appearance_mode: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.StringItem::<init>({this:?}, {label:?}, {text:?}, {appearance_mode:?})");

        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Item", "<init>", "(Ljava/lang/String;)V", (label,))
            .await?;
        jvm.put_field(&mut this, "text", "Ljava/lang/String;", text).await?;
        jvm.put_field(&mut this, "appearanceMode", "I", appearance_mode).await
    }

    async fn get_appearance_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "appearanceMode", "I").await
    }

    async fn get_text(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "text", "Ljava/lang/String;").await
    }

    async fn set_text(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, text: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.StringItem::setText({this:?}, {text:?})");

        jvm.put_field(&mut this, "text", "Ljava/lang/String;", text).await
    }
}
