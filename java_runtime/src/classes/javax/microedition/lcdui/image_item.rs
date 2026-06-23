use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{java::lang::String, javax::microedition::lcdui::Image},
};

// class javax.microedition.lcdui.ImageItem
pub struct ImageItem;

impl ImageItem {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/ImageItem",
            parent_class: Some("javax/microedition/lcdui/Item"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljavax/microedition/lcdui/Image;ILjava/lang/String;)V",
                    Self::init,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljavax/microedition/lcdui/Image;ILjava/lang/String;I)V",
                    Self::init_with_appearance,
                    Default::default(),
                ),
                JavaMethodProto::new("getAltText", "()Ljava/lang/String;", Self::get_alt_text, Default::default()),
                JavaMethodProto::new("getAppearanceMode", "()I", Self::get_appearance_mode, Default::default()),
                JavaMethodProto::new("getImage", "()Ljavax/microedition/lcdui/Image;", Self::get_image, Default::default()),
                JavaMethodProto::new("setAltText", "(Ljava/lang/String;)V", Self::set_alt_text, Default::default()),
                JavaMethodProto::new("setImage", "(Ljavax/microedition/lcdui/Image;)V", Self::set_image, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("image", "Ljavax/microedition/lcdui/Image;", Default::default()),
                JavaFieldProto::new("altText", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("appearanceMode", "I", Default::default()),
                JavaFieldProto::new("PLAIN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HYPERLINK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BUTTON", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/ImageItem";
        jvm.put_static_field(class, "PLAIN", "I", 0).await?;
        jvm.put_static_field(class, "HYPERLINK", "I", 1).await?;
        jvm.put_static_field(class, "BUTTON", "I", 2).await
    }

    async fn init(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        image: ClassInstanceRef<Image>,
        layout: i32,
        alt_text: ClassInstanceRef<String>,
    ) -> Result<()> {
        Self::init_with_appearance(jvm, context, this, label, image, layout, alt_text, 0).await
    }

    async fn init_with_appearance(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        image: ClassInstanceRef<Image>,
        layout: i32,
        alt_text: ClassInstanceRef<String>,
        appearance_mode: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.ImageItem::<init>({this:?}, {label:?}, {image:?}, {layout:?}, {alt_text:?}, {appearance_mode:?})");

        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Item", "<init>", "(Ljava/lang/String;)V", (label,))
            .await?;
        jvm.put_field(&mut this, "layout", "I", layout).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "altText", "Ljava/lang/String;", alt_text).await?;
        jvm.put_field(&mut this, "appearanceMode", "I", appearance_mode).await
    }

    async fn get_alt_text(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "altText", "Ljava/lang/String;").await
    }

    async fn get_appearance_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "appearanceMode", "I").await
    }

    async fn get_image(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Image>> {
        jvm.get_field(&this, "image", "Ljavax/microedition/lcdui/Image;").await
    }

    async fn set_alt_text(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, alt_text: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.ImageItem::setAltText({this:?}, {alt_text:?})");

        jvm.put_field(&mut this, "altText", "Ljava/lang/String;", alt_text).await
    }

    async fn set_image(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, image: ClassInstanceRef<Image>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.ImageItem::setImage({this:?}, {image:?})");

        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await
    }
}
