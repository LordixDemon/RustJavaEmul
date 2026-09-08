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

impl Sprite3D {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Sprite3D",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(ZLjavax/microedition/m3g/Image2D;Ljavax/microedition/m3g/Appearance;)V",
                    Self::init,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getAppearance",
                    "()Ljavax/microedition/m3g/Appearance;",
                    Self::get_appearance,
                    Default::default(),
                ),
                JavaMethodProto::new("getCropHeight", "()I", Self::get_crop_height, Default::default()),
                JavaMethodProto::new("getCropWidth", "()I", Self::get_crop_width, Default::default()),
                JavaMethodProto::new("getCropX", "()I", Self::get_crop_x, Default::default()),
                JavaMethodProto::new("getCropY", "()I", Self::get_crop_y, Default::default()),
                JavaMethodProto::new("getImage", "()Ljavax/microedition/m3g/Image2D;", Self::get_image, Default::default()),
                JavaMethodProto::new("isScaled", "()Z", Self::is_scaled, Default::default()),
                JavaMethodProto::new(
                    "setAppearance",
                    "(Ljavax/microedition/m3g/Appearance;)V",
                    Self::set_appearance,
                    Default::default(),
                ),
                JavaMethodProto::new("setCrop", "(IIII)V", Self::set_crop, Default::default()),
                JavaMethodProto::new("setImage", "(Ljavax/microedition/m3g/Image2D;)V", Self::set_image, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("scaled", "Z", Default::default()),
                JavaFieldProto::new("image", "Ljavax/microedition/m3g/Image2D;", Default::default()),
                JavaFieldProto::new("appearance", "Ljavax/microedition/m3g/Appearance;", Default::default()),
                JavaFieldProto::new("cropX", "I", Default::default()),
                JavaFieldProto::new("cropY", "I", Default::default()),
                JavaFieldProto::new("cropW", "I", Default::default()),
                JavaFieldProto::new("cropH", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        scaled: bool,
        image: ClassInstanceRef<Image2D>,
        appearance: ClassInstanceRef<Appearance>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "scaled", "Z", scaled).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
        jvm.put_field(&mut this, "appearance", "Ljavax/microedition/m3g/Appearance;", appearance)
            .await?;
        jvm.put_field(&mut this, "cropX", "I", 0).await?;
        jvm.put_field(&mut this, "cropY", "I", 0).await?;
        jvm.put_field(&mut this, "cropW", "I", 0).await?;
        jvm.put_field(&mut this, "cropH", "I", 0).await
    }

    pub(super) async fn get_appearance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Appearance>> {
        jvm.get_field(&this, "appearance", "Ljavax/microedition/m3g/Appearance;").await
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

    pub(super) async fn is_scaled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "scaled", "Z").await
    }

    pub(super) async fn set_appearance(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        appearance: ClassInstanceRef<Appearance>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "appearance", "Ljavax/microedition/m3g/Appearance;", appearance)
            .await
    }

    pub(super) async fn set_crop(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "cropX", "I", x).await?;
        jvm.put_field(&mut this, "cropY", "I", y).await?;
        jvm.put_field(&mut this, "cropW", "I", width).await?;
        jvm.put_field(&mut this, "cropH", "I", height).await
    }

    pub(super) async fn set_image(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image2D>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await
    }
}
