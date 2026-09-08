#[allow(unused_imports)]
use super::common::*;
#[allow(unused_imports)]
use super::math::*;
#[allow(unused_imports)]
use super::prelude::*;
#[allow(unused_imports)]
use super::raw_arrays::*;
#[allow(unused_imports)]
use super::render::*;
#[allow(unused_imports)]
use super::types::*;

impl Image2D {
    pub const ALPHA: i32 = 96;
    pub const LUMINANCE: i32 = 97;
    pub const LUMINANCE_ALPHA: i32 = 98;
    pub const RGB: i32 = 99;
    pub const RGBA: i32 = 100;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Image2D",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init_empty, Default::default()),
                JavaMethodProto::new("<init>", "(ILjava/lang/Object;)V", Self::init_from_image, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(ILjavax/microedition/lcdui/Image;)V",
                    Self::init_from_lcdui_image,
                    Default::default(),
                ),
                JavaMethodProto::new("<init>", "(III)V", Self::init_mutable, Default::default()),
                JavaMethodProto::new("<init>", "(III[B)V", Self::init_from_pixels, Default::default()),
                JavaMethodProto::new("<init>", "(III[B[B)V", Self::init_from_pixels_palette, Default::default()),
                JavaMethodProto::new("getFormat", "()I", Self::get_format, Default::default()),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new("isMutable", "()Z", Self::is_mutable, Default::default()),
                JavaMethodProto::new("set", "(IIII[B)V", Self::set, Default::default()),
            ],
            fields: vec![
                static_int_field("ALPHA"),
                static_int_field("LUMINANCE"),
                static_int_field("LUMINANCE_ALPHA"),
                static_int_field("RGB"),
                static_int_field("RGBA"),
                JavaFieldProto::new("format", "I", Default::default()),
                JavaFieldProto::new("width", "I", Default::default()),
                JavaFieldProto::new("height", "I", Default::default()),
                JavaFieldProto::new("image", "Ljavax/microedition/lcdui/Image;", Default::default()),
                JavaFieldProto::new("mutable", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/Image2D",
            &[
                ("ALPHA", Self::ALPHA),
                ("LUMINANCE", Self::LUMINANCE),
                ("LUMINANCE_ALPHA", Self::LUMINANCE_ALPHA),
                ("RGB", Self::RGB),
                ("RGBA", Self::RGBA),
            ],
        )
        .await
    }

    pub(super) async fn init_empty(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "format", "I", Self::RGBA).await?;
        jvm.put_field(&mut this, "width", "I", 1).await?;
        jvm.put_field(&mut this, "height", "I", 1).await?;
        let image = Image::from_argb(jvm, 1, 1, vec![0xffff_ffffu32 as i32]).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "mutable", "Z", false).await
    }

    pub(super) async fn init_from_image(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        format: i32,
        image_object: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        let image: ClassInstanceRef<Image> = cast_ref(&image_object);
        let width = if image.is_null() {
            1
        } else {
            jvm.get_field(&image, "width", "I").await.unwrap_or(1)
        };
        let height = if image.is_null() {
            1
        } else {
            jvm.get_field(&image, "height", "I").await.unwrap_or(1)
        };
        jvm.put_field(&mut this, "format", "I", format).await?;
        jvm.put_field(&mut this, "width", "I", width).await?;
        jvm.put_field(&mut this, "height", "I", height).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "mutable", "Z", false).await
    }

    pub(super) async fn init_from_lcdui_image(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        format: i32,
        image: ClassInstanceRef<Image>,
    ) -> Result<()> {
        Self::init_from_image(jvm, context, this, format, cast_ref(&image)).await
    }

    pub(super) async fn init_mutable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        format: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        let width = width.max(1);
        let height = height.max(1);
        jvm.put_field(&mut this, "format", "I", format).await?;
        jvm.put_field(&mut this, "width", "I", width).await?;
        jvm.put_field(&mut this, "height", "I", height).await?;
        let image = Image::from_argb(jvm, width, height, vec![0; (width * height) as usize]).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "mutable", "Z", true).await
    }

    pub(super) async fn init_from_pixels(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        format: i32,
        width: i32,
        height: i32,
        pixels: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        Self::init_from_pixel_data(jvm, &mut this, format, width, height, pixels, null_ref()).await
    }

    pub(super) async fn init_from_pixels_palette(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        format: i32,
        width: i32,
        height: i32,
        pixels: ClassInstanceRef<Array<i8>>,
        palette: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        Self::init_from_pixel_data(jvm, &mut this, format, width, height, pixels, palette).await
    }

    pub(super) async fn get_format(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "format", "I").await
    }
    pub(super) async fn get_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "width", "I").await
    }
    pub(super) async fn get_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "height", "I").await
    }

    pub(super) async fn is_mutable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "mutable", "Z").await
    }

    pub(super) async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        if pixels.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Image2D.set").await);
        }
        if !jvm.get_field::<bool>(&this, "mutable", "Z").await.unwrap_or(false) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Image2D is immutable").await);
        }
        let image_width: i32 = jvm.get_field(&this, "width", "I").await?;
        let image_height: i32 = jvm.get_field(&this, "height", "I").await?;
        if x < 0 || y < 0 || width < 0 || height < 0 || x + width > image_width || y + height > image_height {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Image2D.set bounds").await);
        }
        let format: i32 = jvm.get_field(&this, "format", "I").await?;
        let byte_len = jvm.array_length(&pixels).await?;
        let pixel_bytes = raw_u8_array(jvm, &pixels, byte_len).await?;
        let patch = decode_m3g_image_pixels(format, width, height, &[], &pixel_bytes);
        let image: ClassInstanceRef<Image> = jvm
            .get_field(&this, "image", "Ljavax/microedition/lcdui/Image;")
            .await
            .unwrap_or_else(|_| null_ref());
        let mut argb = if image.is_null() {
            vec![0; (image_width * image_height) as usize]
        } else {
            let (_, _, pixels_array) = Image::pixels(jvm, &image).await?;
            raw_i32_array(jvm, &pixels_array, (image_width * image_height) as usize).await?
        };
        for row in 0..height {
            let src = (row * width) as usize;
            let dst = ((y + row) * image_width + x) as usize;
            let count = width as usize;
            argb[dst..dst + count].copy_from_slice(&patch[src..src + count]);
        }
        let image = Image::from_argb(jvm, image_width, image_height, argb).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await
    }

    pub(super) async fn init_from_pixel_data(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        format: i32,
        width: i32,
        height: i32,
        pixels: ClassInstanceRef<Array<i8>>,
        palette: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        if pixels.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Image2D pixels").await);
        }
        let _: () = jvm.invoke_special(this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        let width = width.max(1);
        let height = height.max(1);
        let pixels_len = jvm.array_length(&pixels).await?;
        let pixels = raw_u8_array(jvm, &pixels, pixels_len).await?;
        let palette = if palette.is_null() {
            Vec::new()
        } else {
            let palette_len = jvm.array_length(&palette).await?;
            raw_u8_array(jvm, &palette, palette_len).await?
        };
        let argb = decode_m3g_image_pixels(format, width, height, &palette, &pixels);
        let image = Image::from_argb(jvm, width, height, argb).await?;
        jvm.put_field(this, "format", "I", format).await?;
        jvm.put_field(this, "width", "I", width).await?;
        jvm.put_field(this, "height", "I", height).await?;
        jvm.put_field(this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(this, "mutable", "Z", false).await
    }
}
