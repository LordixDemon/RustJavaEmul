use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{
    Array, ClassInstanceRef, Jvm, Result,
    runtime::{JavaIoInputStream, JavaLangString},
};

use crate::{
    DecodedImage, RuntimeClassProto, RuntimeContext,
    classes::{
        java::{io::InputStream, lang::String},
        javax::microedition::lcdui::Graphics,
    },
};

// class javax.microedition.lcdui.Image
pub struct Image;

impl Image {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Image",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(II)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "createImage",
                    "(II)Ljavax/microedition/lcdui/Image;",
                    Self::create_image,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "([BII)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_from_bytes,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljava/lang/String;)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_from_resource,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljava/io/InputStream;)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_from_stream,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljavax/microedition/lcdui/Image;)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_from_image,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljavax/microedition/lcdui/Image;IIIII)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_from_region,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createRGBImage",
                    "([IIIZ)Ljavax/microedition/lcdui/Image;",
                    Self::create_rgb_image,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getGraphics",
                    "()Ljavax/microedition/lcdui/Graphics;",
                    Self::get_graphics,
                    Default::default(),
                ),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new("isMutable", "()Z", Self::is_mutable, Default::default()),
                JavaMethodProto::new("getRGB", "([IIIIIII)V", Self::get_rgb, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("width", "I", Default::default()),
                JavaFieldProto::new("height", "I", Default::default()),
                JavaFieldProto::new("mutable", "Z", Default::default()),
                JavaFieldProto::new("argb", "[I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, width: i32, height: i32) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Image::<init>({this:?}, {width:?}, {height:?})");

        let width = width.max(1);
        let height = height.max(1);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "width", "I", width).await?;
        jvm.put_field(&mut this, "height", "I", height).await?;
        jvm.put_field(&mut this, "mutable", "Z", true).await?;

        let argb = vec![0; (width * height) as usize];
        Self::put_pixels(jvm, &mut this, argb).await?;

        Ok(())
    }

    async fn create_image(jvm: &Jvm, _: &mut RuntimeContext, width: i32, height: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.lcdui.Image::createImage({width:?}, {height:?})");

        Ok(jvm.new_class("javax/microedition/lcdui/Image", "(II)V", (width, height)).await?.into())
    }

    async fn create_image_from_bytes(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        image_data: ClassInstanceRef<Array<i8>>,
        image_offset: i32,
        image_length: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.lcdui.Image::createImage({image_data:?}, {image_offset:?}, {image_length:?})");

        let mut data = vec![0; image_length.max(0) as usize];
        jvm.array_raw_buffer(&image_data).await?.read(image_offset.max(0) as _, &mut data)?;

        if let Some(decoded) = context.decode_image(&data) {
            Self::from_decoded(jvm, decoded).await
        } else {
            let (width, height) = png_dimensions(&data).unwrap_or((1, 1));
            Ok(jvm
                .new_class("javax/microedition/lcdui/Image", "(II)V", (width as i32, height as i32))
                .await?
                .into())
        }
    }

    async fn create_image_from_resource(jvm: &Jvm, context: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let name_str = JavaLangString::to_rust_string(jvm, &name).await?;
        tracing::debug!("javax.microedition.lcdui.Image::createImage({name_str:?})");

        let resource_name = name_str.trim_start_matches('/');
        let loader = jvm
            .invoke_static("java/lang/ClassLoader", "getSystemClassLoader", "()Ljava/lang/ClassLoader;", ())
            .await?;
        let stream: ClassInstanceRef<InputStream> = jvm
            .invoke_virtual(
                &loader,
                "getResourceAsStream",
                "(Ljava/lang/String;)Ljava/io/InputStream;",
                (JavaLangString::from_rust_string(jvm, resource_name).await?,),
            )
            .await?;

        if stream.is_null() {
            tracing::warn!("resource image not found: {resource_name}");
            return Ok(jvm.new_class("javax/microedition/lcdui/Image", "(II)V", (1, 1)).await?.into());
        }

        let data = JavaIoInputStream::read_until_end(jvm, &stream).await?;
        if let Some(decoded) = context.decode_image(&data) {
            Self::from_decoded(jvm, decoded).await
        } else {
            let (width, height) = png_dimensions(&data).unwrap_or((1, 1));
            Ok(jvm
                .new_class("javax/microedition/lcdui/Image", "(II)V", (width as i32, height as i32))
                .await?
                .into())
        }
    }

    async fn create_image_from_stream(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        stream: ClassInstanceRef<InputStream>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.lcdui.Image::createImage({stream:?})");

        if stream.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "image stream").await);
        }

        let data = JavaIoInputStream::read_until_end(jvm, &stream).await?;
        if let Some(decoded) = context.decode_image(&data) {
            Self::from_decoded(jvm, decoded).await
        } else {
            let (width, height) = png_dimensions(&data).unwrap_or((1, 1));
            Ok(jvm
                .new_class("javax/microedition/lcdui/Image", "(II)V", (width as i32, height as i32))
                .await?
                .into())
        }
    }

    async fn create_image_from_image(jvm: &Jvm, _: &mut RuntimeContext, image: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.lcdui.Image::createImage({image:?})");

        if image.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "image is null").await);
        }
        let (width, height, pixels) = Self::pixels(jvm, &image).await?;
        let pixels = jvm.load_array(&pixels, 0, (width * height) as usize).await?;
        Self::from_argb_with_mutability(jvm, width, height, pixels, false).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn create_image_from_region(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        image: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        transform: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.lcdui.Image::createImage({image:?}, {x:?}, {y:?}, {width:?}, {height:?}, {transform:?})");

        if image.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "image is null").await);
        }
        if width <= 0 || height <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid image region size").await);
        }

        let (source_width, source_height, source_array) = Self::pixels(jvm, &image).await?;
        if x < 0 || y < 0 || x + width > source_width || y + height > source_height {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "image region out of bounds").await);
        }

        let source_pixels: Vec<i32> = jvm.load_array(&source_array, 0, (source_width * source_height) as usize).await?;
        let (draw_width, draw_height) = transformed_size(width, height, transform);
        let mut pixels = vec![0; (draw_width * draw_height) as usize];
        for src_y in 0..height {
            for src_x in 0..width {
                let src_index = ((y + src_y) * source_width + x + src_x) as usize;
                let (dst_x, dst_y) = transform_point(src_x, src_y, width, height, transform);
                if dst_x >= 0 && dst_y >= 0 && dst_x < draw_width && dst_y < draw_height {
                    pixels[(dst_y * draw_width + dst_x) as usize] = source_pixels[src_index];
                }
            }
        }

        Self::from_argb_with_mutability(jvm, draw_width, draw_height, pixels, false).await
    }

    async fn create_rgb_image(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        rgb: ClassInstanceRef<Array<i32>>,
        width: i32,
        height: i32,
        process_alpha: bool,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.lcdui.Image::createRGBImage({rgb:?}, {width:?}, {height:?}, {process_alpha:?})");

        let width = width.max(1);
        let height = height.max(1);
        let mut argb: Vec<i32> = jvm.load_array(&rgb, 0, (width * height) as usize).await?;
        if !process_alpha {
            for pixel in &mut argb {
                *pixel |= 0xff00_0000u32 as i32;
            }
        }

        Self::from_argb_with_mutability(jvm, width, height, argb, false).await
    }

    async fn get_graphics(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Graphics>> {
        tracing::debug!("javax.microedition.lcdui.Image::getGraphics({this:?})");

        Ok(jvm
            .new_class("javax/microedition/lcdui/Graphics", "(Ljavax/microedition/lcdui/Image;)V", (this,))
            .await?
            .into())
    }

    async fn get_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.lcdui.Image::getWidth({this:?})");

        jvm.get_field(&this, "width", "I").await
    }

    async fn get_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.lcdui.Image::getHeight({this:?})");

        jvm.get_field(&this, "height", "I").await
    }

    async fn is_mutable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "mutable", "Z").await
    }

    #[allow(clippy::too_many_arguments)]
    async fn get_rgb(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut rgb_data: ClassInstanceRef<Array<i32>>,
        offset: i32,
        scan_length: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        tracing::trace!(
            "javax.microedition.lcdui.Image::getRGB({this:?}, {rgb_data:?}, {offset:?}, {scan_length:?}, {x:?}, {y:?}, {width:?}, {height:?})"
        );

        if width <= 0 || height <= 0 {
            return Ok(());
        }

        let image_width: i32 = jvm.get_field(&this, "width", "I").await?;
        let pixels: ClassInstanceRef<Array<i32>> = jvm.get_field(&this, "argb", "[I").await?;
        for row in 0..height {
            let src_offset = ((y + row) * image_width + x) as usize;
            let dst_offset = offset + row * scan_length;
            if dst_offset < 0 {
                continue;
            }
            let row_pixels: Vec<i32> = jvm.load_array(&pixels, src_offset, width as usize).await?;
            jvm.store_array(&mut rgb_data, dst_offset as usize, row_pixels).await?;
        }

        Ok(())
    }

    async fn from_decoded(jvm: &Jvm, decoded: DecodedImage) -> Result<ClassInstanceRef<Self>> {
        Self::from_argb_with_mutability(jvm, decoded.width, decoded.height, decoded.argb, false).await
    }

    pub(crate) async fn from_argb(jvm: &Jvm, width: i32, height: i32, argb: Vec<i32>) -> Result<ClassInstanceRef<Self>> {
        Self::from_argb_with_mutability(jvm, width, height, argb, false).await
    }

    async fn from_argb_with_mutability(jvm: &Jvm, width: i32, height: i32, argb: Vec<i32>, mutable: bool) -> Result<ClassInstanceRef<Self>> {
        let mut image: ClassInstanceRef<Self> = jvm.new_class("javax/microedition/lcdui/Image", "(II)V", (width, height)).await?.into();
        jvm.put_field(&mut image, "mutable", "Z", mutable).await?;
        Self::put_pixels(jvm, &mut image, argb).await?;

        Ok(image)
    }

    pub async fn pixels(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<(i32, i32, ClassInstanceRef<Array<i32>>)> {
        let width = jvm.get_field(this, "width", "I").await?;
        let height = jvm.get_field(this, "height", "I").await?;
        let pixels = jvm.get_field(this, "argb", "[I").await?;

        Ok((width, height, pixels))
    }

    async fn put_pixels(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, argb: Vec<i32>) -> Result<()> {
        let mut pixels = jvm.instantiate_array("I", argb.len()).await?;
        jvm.store_array(&mut pixels, 0, argb).await?;
        jvm.put_field(this, "argb", "[I", pixels).await
    }
}

fn transformed_size(width: i32, height: i32, transform: i32) -> (i32, i32) {
    match transform {
        4..=7 => (height, width),
        _ => (width, height),
    }
}

fn transform_point(x: i32, y: i32, width: i32, height: i32, transform: i32) -> (i32, i32) {
    match transform {
        1 => (x, height - 1 - y),
        2 => (width - 1 - x, y),
        3 => (width - 1 - x, height - 1 - y),
        4 => (y, x),
        5 => (height - 1 - y, x),
        6 => (y, width - 1 - x),
        7 => (height - 1 - y, width - 1 - x),
        _ => (x, y),
    }
}

fn png_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

    if data.len() < 24 || &data[..8] != PNG_SIGNATURE || &data[12..16] != b"IHDR" {
        return None;
    }

    let width = u32::from_be_bytes(data[16..20].try_into().ok()?);
    let height = u32::from_be_bytes(data[20..24].try_into().ok()?);

    Some((width.max(1), height.max(1)))
}
