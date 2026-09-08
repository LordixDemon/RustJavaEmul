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

impl Background {
    pub const BORDER: i32 = 32;
    pub const REPEAT: i32 = 33;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Background",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getColor", "()I", Self::get_color, Default::default()),
                JavaMethodProto::new("getCropHeight", "()I", Self::get_crop_height, Default::default()),
                JavaMethodProto::new("getCropWidth", "()I", Self::get_crop_width, Default::default()),
                JavaMethodProto::new("getCropX", "()I", Self::get_crop_x, Default::default()),
                JavaMethodProto::new("getCropY", "()I", Self::get_crop_y, Default::default()),
                JavaMethodProto::new("getImage", "()Ljavax/microedition/m3g/Image2D;", Self::get_image, Default::default()),
                JavaMethodProto::new("getImageModeX", "()I", Self::get_image_mode_x, Default::default()),
                JavaMethodProto::new("getImageModeY", "()I", Self::get_image_mode_y, Default::default()),
                JavaMethodProto::new("isColorClearEnabled", "()Z", Self::is_color_clear_enabled, Default::default()),
                JavaMethodProto::new("isDepthClearEnabled", "()Z", Self::is_depth_clear_enabled, Default::default()),
                JavaMethodProto::new("setColor", "(I)V", Self::set_color, Default::default()),
                JavaMethodProto::new("setColorClearEnable", "(Z)V", Self::set_color_clear_enable, Default::default()),
                JavaMethodProto::new("setCrop", "(IIII)V", Self::set_crop, Default::default()),
                JavaMethodProto::new("setDepthClearEnable", "(Z)V", Self::set_depth_clear_enable, Default::default()),
                JavaMethodProto::new("setImage", "(Ljavax/microedition/m3g/Image2D;)V", Self::set_image, Default::default()),
                JavaMethodProto::new("setImageMode", "(II)V", Self::set_image_mode, Default::default()),
            ],
            fields: vec![
                static_int_field("BORDER"),
                static_int_field("REPEAT"),
                JavaFieldProto::new("color", "I", Default::default()),
                JavaFieldProto::new("image", "Ljavax/microedition/m3g/Image2D;", Default::default()),
                JavaFieldProto::new("colorClear", "Z", Default::default()),
                JavaFieldProto::new("depthClear", "Z", Default::default()),
                JavaFieldProto::new("imageModeX", "I", Default::default()),
                JavaFieldProto::new("imageModeY", "I", Default::default()),
                JavaFieldProto::new("cropX", "I", Default::default()),
                JavaFieldProto::new("cropY", "I", Default::default()),
                JavaFieldProto::new("cropW", "I", Default::default()),
                JavaFieldProto::new("cropH", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/Background",
            &[("BORDER", Self::BORDER), ("REPEAT", Self::REPEAT)],
        )
        .await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "color", "I", 0xff000000u32 as i32).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", null_ref::<Image2D>())
            .await?;
        jvm.put_field(&mut this, "colorClear", "Z", true).await?;
        jvm.put_field(&mut this, "depthClear", "Z", true).await?;
        jvm.put_field(&mut this, "imageModeX", "I", Self::BORDER).await?;
        jvm.put_field(&mut this, "imageModeY", "I", Self::BORDER).await?;
        jvm.put_field(&mut this, "cropX", "I", 0).await?;
        jvm.put_field(&mut this, "cropY", "I", 0).await?;
        jvm.put_field(&mut this, "cropW", "I", 0).await?;
        jvm.put_field(&mut this, "cropH", "I", 0).await
    }

    pub(super) async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "color", "I").await
    }

    pub(super) async fn get_crop_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropH", "I").await
    }

    pub(super) async fn get_crop_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropW", "I").await
    }

    pub(super) async fn get_crop_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropX", "I").await
    }

    pub(super) async fn get_crop_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropY", "I").await
    }

    pub(super) async fn get_image(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Image2D>> {
        jvm.get_field(&this, "image", "Ljavax/microedition/m3g/Image2D;").await
    }

    pub(super) async fn get_image_mode_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "imageModeX", "I").await
    }

    pub(super) async fn get_image_mode_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "imageModeY", "I").await
    }

    pub(super) async fn is_color_clear_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "colorClear", "Z").await
    }

    pub(super) async fn is_depth_clear_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "depthClear", "Z").await
    }

    pub(super) async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, color: i32) -> Result<()> {
        jvm.put_field(&mut this, "color", "I", color).await
    }

    pub(super) async fn set_color_clear_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, enabled: bool) -> Result<()> {
        jvm.put_field(&mut this, "colorClear", "Z", enabled).await
    }

    pub(super) async fn set_crop(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        crop_x: i32,
        crop_y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        if width < 0 || height < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "negative background crop").await);
        }
        jvm.put_field(&mut this, "cropX", "I", crop_x).await?;
        jvm.put_field(&mut this, "cropY", "I", crop_y).await?;
        jvm.put_field(&mut this, "cropW", "I", width).await?;
        jvm.put_field(&mut this, "cropH", "I", height).await
    }

    pub(super) async fn set_depth_clear_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, enabled: bool) -> Result<()> {
        jvm.put_field(&mut this, "depthClear", "Z", enabled).await
    }

    pub(super) async fn set_image(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image2D>,
    ) -> Result<()> {
        let crop = if image.is_null() {
            (0, 0)
        } else {
            (
                jvm.get_field(&image, "width", "I").await.unwrap_or(0),
                jvm.get_field(&image, "height", "I").await.unwrap_or(0),
            )
        };
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
        jvm.put_field(&mut this, "cropX", "I", 0).await?;
        jvm.put_field(&mut this, "cropY", "I", 0).await?;
        jvm.put_field(&mut this, "cropW", "I", crop.0).await?;
        jvm.put_field(&mut this, "cropH", "I", crop.1).await
    }

    pub(super) async fn set_image_mode(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, mode_x: i32, mode_y: i32) -> Result<()> {
        if !matches!(mode_x, Self::BORDER | Self::REPEAT) || !matches!(mode_y, Self::BORDER | Self::REPEAT) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "bad background image mode").await);
        }
        jvm.put_field(&mut this, "imageModeX", "I", mode_x).await?;
        jvm.put_field(&mut this, "imageModeY", "I", mode_y).await
    }
}
