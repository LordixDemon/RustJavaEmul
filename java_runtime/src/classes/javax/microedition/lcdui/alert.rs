use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::lang::String,
        javax::microedition::lcdui::{AlertType, Graphics, Image},
    },
};

// class javax.microedition.lcdui.Alert
pub struct Alert;

impl Alert {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Alert",
            parent_class: Some("javax/microedition/lcdui/Displayable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;Ljavax/microedition/lcdui/Image;Ljavax/microedition/lcdui/AlertType;)V",
                    Self::init_full,
                    Default::default(),
                ),
                JavaMethodProto::new("getString", "()Ljava/lang/String;", Self::get_string, Default::default()),
                JavaMethodProto::new("getTimeout", "()I", Self::get_timeout, Default::default()),
                JavaMethodProto::new("setString", "(Ljava/lang/String;)V", Self::set_string, Default::default()),
                JavaMethodProto::new("setTimeout", "(I)V", Self::set_timeout, Default::default()),
                JavaMethodProto::new("setType", "(Ljavax/microedition/lcdui/AlertType;)V", Self::set_type, Default::default()),
                JavaMethodProto::new("showNotify", "()V", Self::show_notify, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("title", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("text", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("timeout", "I", Default::default()),
                JavaFieldProto::new("alertType", "Ljavax/microedition/lcdui/AlertType;", Default::default()),
                JavaFieldProto::new("FOREVER", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "DISMISS_COMMAND",
                    "Ljavax/microedition/lcdui/Command;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("javax/microedition/lcdui/Alert", "FOREVER", "I", -2).await?;
        let label = JavaLangString::from_rust_string(jvm, "Done").await?;
        let command = jvm
            .new_class("javax/microedition/lcdui/Command", "(Ljava/lang/String;II)V", (label, 4, 0))
            .await?;
        jvm.put_static_field(
            "javax/microedition/lcdui/Alert",
            "DISMISS_COMMAND",
            "Ljavax/microedition/lcdui/Command;",
            command,
        )
        .await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, title: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Alert::<init>({this:?}, {title:?})");

        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Displayable", "<init>", "()V", ())
            .await?;
        jvm.put_field(&mut this, "title", "Ljava/lang/String;", title).await?;
        jvm.put_field(&mut this, "text", "Ljava/lang/String;", ClassInstanceRef::<String>::new(None))
            .await?;
        jvm.put_field(&mut this, "timeout", "I", 2000).await
    }

    async fn init_full(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        title: ClassInstanceRef<String>,
        text: ClassInstanceRef<String>,
        _image: ClassInstanceRef<Image>,
        alert_type: ClassInstanceRef<AlertType>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone(), title).await?;
        jvm.put_field(&mut this, "text", "Ljava/lang/String;", text).await?;
        jvm.put_field(&mut this, "alertType", "Ljavax/microedition/lcdui/AlertType;", alert_type)
            .await
    }

    async fn set_type(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, alert_type: ClassInstanceRef<AlertType>) -> Result<()> {
        jvm.put_field(&mut this, "alertType", "Ljavax/microedition/lcdui/AlertType;", alert_type)
            .await
    }

    async fn get_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "text", "Ljava/lang/String;").await
    }

    async fn get_timeout(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "timeout", "I").await
    }

    async fn set_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, text: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "text", "Ljava/lang/String;", text).await
    }

    async fn set_timeout(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, timeout: i32) -> Result<()> {
        jvm.put_field(&mut this, "timeout", "I", timeout).await
    }

    async fn show_notify(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let graphics: ClassInstanceRef<Graphics> = jvm.new_class("javax/microedition/lcdui/Graphics", "()V", ()).await?.into();
        let width = context.screen_width();
        let height = context.screen_height();
        let _: () = jvm.invoke_virtual(&graphics, "setColor", "(I)V", (0x101820,)).await?;
        let _: () = jvm.invoke_virtual(&graphics, "fillRect", "(IIII)V", (0, 0, width, height)).await?;
        let _: () = jvm.invoke_virtual(&graphics, "setColor", "(I)V", (0xffffff,)).await?;

        let title: ClassInstanceRef<String> = jvm.get_field(&this, "title", "Ljava/lang/String;").await?;
        if !title.is_null() {
            let _: () = jvm
                .invoke_virtual(&graphics, "drawString", "(Ljava/lang/String;III)V", (title, width / 2, 22, 17))
                .await?;
        }

        let text: ClassInstanceRef<String> = jvm.get_field(&this, "text", "Ljava/lang/String;").await?;
        if !text.is_null() {
            let message = JavaLangString::to_rust_string(jvm, &text).await?;
            for (line, chunk) in message.as_bytes().chunks(28).take(8).enumerate() {
                let line_text = JavaLangString::from_rust_string(jvm, core::str::from_utf8(chunk).unwrap_or("")).await?;
                let _: () = jvm
                    .invoke_virtual(
                        &graphics,
                        "drawString",
                        "(Ljava/lang/String;III)V",
                        (line_text, 16, 56 + line as i32 * 18, 20),
                    )
                    .await?;
            }
        }

        context.screen_present();
        Ok(())
    }
}
