#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::javax::microedition::lcdui::{Graphics, Image},
};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

impl DirectUtils {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/nokia/mid/ui/DirectUtils",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "getDirectGraphics",
                    "(Ljavax/microedition/lcdui/Graphics;)Lcom/nokia/mid/ui/DirectGraphics;",
                    Self::get_direct_graphics,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(II)Ljavax/microedition/lcdui/Image;",
                    Self::create_image,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(III)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_color,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljavax/microedition/lcdui/Image;IIIII)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_region,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "([BII)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_bytes,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn get_direct_graphics(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        graphics: ClassInstanceRef<Graphics>,
    ) -> Result<ClassInstanceRef<DirectGraphics>> {
        tracing::debug!("com.nokia.mid.ui.DirectUtils::getDirectGraphics({graphics:?})");

        Ok(jvm
            .new_class("com/nokia/mid/ui/DirectGraphics", "(Ljavax/microedition/lcdui/Graphics;)V", (graphics,))
            .await?
            .into())
    }

    pub(super) async fn create_image(jvm: &Jvm, _: &mut RuntimeContext, width: i32, height: i32) -> Result<ClassInstanceRef<Image>> {
        jvm.invoke_static(
            "javax/microedition/lcdui/Image",
            "createImage",
            "(II)Ljavax/microedition/lcdui/Image;",
            (width, height),
        )
        .await
    }

    pub(super) async fn create_image_color(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        width: i32,
        height: i32,
        argb: i32,
    ) -> Result<ClassInstanceRef<Image>> {
        let image = Self::create_image(jvm, context, width, height).await?;
        let graphics: ClassInstanceRef<Graphics> = jvm
            .invoke_virtual(&image, "getGraphics", "()Ljavax/microedition/lcdui/Graphics;", ())
            .await?;
        let _: () = jvm.invoke_virtual(&graphics, "setColor", "(I)V", (argb,)).await?;
        let _: () = jvm.invoke_virtual(&graphics, "fillRect", "(IIII)V", (0, 0, width, height)).await?;
        Ok(image)
    }

    pub(super) async fn create_image_region(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        image: ClassInstanceRef<Image>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        transform: i32,
    ) -> Result<ClassInstanceRef<Image>> {
        jvm.invoke_static(
            "javax/microedition/lcdui/Image",
            "createImage",
            "(Ljavax/microedition/lcdui/Image;IIIII)Ljavax/microedition/lcdui/Image;",
            (image, x, y, width, height, transform),
        )
        .await
    }

    pub(super) async fn create_image_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        bytes: ClassInstanceRef<jvm::Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<ClassInstanceRef<Image>> {
        jvm.invoke_static(
            "javax/microedition/lcdui/Image",
            "createImage",
            "([BII)Ljavax/microedition/lcdui/Image;",
            (bytes, offset, length),
        )
        .await
    }
}
