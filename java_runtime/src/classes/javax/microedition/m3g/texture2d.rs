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

impl Texture2D {
    pub const FUNC_ADD: i32 = 224;
    pub const FUNC_BLEND: i32 = 225;
    pub const FUNC_DECAL: i32 = 226;
    pub const FUNC_MODULATE: i32 = 227;
    pub const FUNC_REPLACE: i32 = 228;
    pub const WRAP_CLAMP: i32 = 240;
    pub const WRAP_REPEAT: i32 = 241;
    pub const FILTER_BASE_LEVEL: i32 = 208;
    pub const FILTER_LINEAR: i32 = 209;
    pub const FILTER_NEAREST: i32 = 210;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Texture2D",
            parent_class: Some("javax/microedition/m3g/Transformable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init_empty, Default::default()),
                JavaMethodProto::new("<init>", "(Ljavax/microedition/m3g/Image2D;)V", Self::init, Default::default()),
                JavaMethodProto::new("getBlendColor", "()I", Self::get_blend_color, Default::default()),
                JavaMethodProto::new("getBlending", "()I", Self::get_blending, Default::default()),
                JavaMethodProto::new("getImage", "()Ljavax/microedition/m3g/Image2D;", Self::get_image, Default::default()),
                JavaMethodProto::new("getImageFilter", "()I", Self::get_image_filter, Default::default()),
                JavaMethodProto::new("getLevelFilter", "()I", Self::get_level_filter, Default::default()),
                JavaMethodProto::new("getWrappingS", "()I", Self::get_wrapping_s, Default::default()),
                JavaMethodProto::new("getWrappingT", "()I", Self::get_wrapping_t, Default::default()),
                JavaMethodProto::new("setBlendColor", "(I)V", Self::set_blend_color, Default::default()),
                JavaMethodProto::new("setBlending", "(I)V", Self::set_blending, Default::default()),
                JavaMethodProto::new("setFiltering", "(II)V", Self::set_filtering, Default::default()),
                JavaMethodProto::new("setImage", "(Ljavax/microedition/m3g/Image2D;)V", Self::set_image, Default::default()),
                JavaMethodProto::new("setWrapping", "(II)V", Self::set_wrapping, Default::default()),
            ],
            fields: vec![
                static_int_field("FILTER_BASE_LEVEL"),
                static_int_field("FILTER_LINEAR"),
                static_int_field("FILTER_NEAREST"),
                static_int_field("FUNC_ADD"),
                static_int_field("FUNC_BLEND"),
                static_int_field("FUNC_DECAL"),
                static_int_field("FUNC_MODULATE"),
                static_int_field("FUNC_REPLACE"),
                static_int_field("WRAP_CLAMP"),
                static_int_field("WRAP_REPEAT"),
                JavaFieldProto::new("image", "Ljavax/microedition/m3g/Image2D;", Default::default()),
                JavaFieldProto::new("blendColor", "I", Default::default()),
                JavaFieldProto::new("blending", "I", Default::default()),
                JavaFieldProto::new("wrappingS", "I", Default::default()),
                JavaFieldProto::new("wrappingT", "I", Default::default()),
                JavaFieldProto::new("levelFilter", "I", Default::default()),
                JavaFieldProto::new("imageFilter", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/Texture2D",
            &[
                ("FILTER_BASE_LEVEL", Self::FILTER_BASE_LEVEL),
                ("FILTER_LINEAR", Self::FILTER_LINEAR),
                ("FILTER_NEAREST", Self::FILTER_NEAREST),
                ("FUNC_ADD", Self::FUNC_ADD),
                ("FUNC_BLEND", Self::FUNC_BLEND),
                ("FUNC_DECAL", Self::FUNC_DECAL),
                ("FUNC_MODULATE", Self::FUNC_MODULATE),
                ("FUNC_REPLACE", Self::FUNC_REPLACE),
                ("WRAP_CLAMP", Self::WRAP_CLAMP),
                ("WRAP_REPEAT", Self::WRAP_REPEAT),
            ],
        )
        .await
    }

    pub(super) async fn init_empty(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/m3g/Transformable", "<init>", "()V", ())
            .await?;
        Self::set_default_fields(jvm, &mut this).await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, image: ClassInstanceRef<Image2D>) -> Result<()> {
        if image.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Texture2D image").await);
        }
        Self::ensure_power_of_two_image(jvm, &image).await?;
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/m3g/Transformable", "<init>", "()V", ())
            .await?;
        Self::set_default_fields(jvm, &mut this).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await
    }

    pub(super) async fn get_blend_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "blendColor", "I").await
    }

    pub(super) async fn get_blending(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "blending", "I").await
    }

    pub(super) async fn get_image(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Image2D>> {
        jvm.get_field(&this, "image", "Ljavax/microedition/m3g/Image2D;").await
    }

    pub(super) async fn get_image_filter(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "imageFilter", "I").await
    }

    pub(super) async fn get_level_filter(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "levelFilter", "I").await
    }

    pub(super) async fn get_wrapping_s(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "wrappingS", "I").await
    }

    pub(super) async fn get_wrapping_t(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "wrappingT", "I").await
    }

    pub(super) async fn set_blend_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, color: i32) -> Result<()> {
        jvm.put_field(&mut this, "blendColor", "I", color).await
    }

    pub(super) async fn set_blending(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, blending: i32) -> Result<()> {
        jvm.put_field(&mut this, "blending", "I", blending).await
    }

    pub(super) async fn set_filtering(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        level_filter: i32,
        image_filter: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "levelFilter", "I", level_filter).await?;
        jvm.put_field(&mut this, "imageFilter", "I", image_filter).await
    }

    pub(super) async fn set_image(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image2D>,
    ) -> Result<()> {
        if image.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Texture2D.setImage").await);
        }
        Self::ensure_power_of_two_image(jvm, &image).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await
    }

    pub(super) async fn ensure_power_of_two_image(jvm: &Jvm, image: &ClassInstanceRef<Image2D>) -> Result<()> {
        let width: i32 = jvm.get_field(image, "width", "I").await.unwrap_or(0);
        let height: i32 = jvm.get_field(image, "height", "I").await.unwrap_or(0);
        if !is_texture_dimension(width) || !is_texture_dimension(height) {
            return Err(jvm
                .exception(
                    "java/lang/IllegalArgumentException",
                    "Texture2D image size must be a positive power of two",
                )
                .await);
        }
        Ok(())
    }

    pub(super) async fn set_wrapping(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, wrap_s: i32, wrap_t: i32) -> Result<()> {
        jvm.put_field(&mut this, "wrappingS", "I", wrap_s).await?;
        jvm.put_field(&mut this, "wrappingT", "I", wrap_t).await
    }

    pub(super) async fn set_default_fields(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(this, "image", "Ljavax/microedition/m3g/Image2D;", null_ref::<Image2D>())
            .await?;
        jvm.put_field(this, "blendColor", "I", 0).await?;
        jvm.put_field(this, "blending", "I", Self::FUNC_MODULATE).await?;
        jvm.put_field(this, "wrappingS", "I", Self::WRAP_REPEAT).await?;
        jvm.put_field(this, "wrappingT", "I", Self::WRAP_REPEAT).await?;
        jvm.put_field(this, "levelFilter", "I", Self::FILTER_BASE_LEVEL).await?;
        jvm.put_field(this, "imageFilter", "I", Self::FILTER_NEAREST).await
    }
}

pub(super) fn is_texture_dimension(value: i32) -> bool {
    value > 0 && (value & (value - 1)) == 0
}
