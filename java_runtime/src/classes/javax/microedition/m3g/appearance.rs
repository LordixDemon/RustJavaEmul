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

impl Appearance {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Appearance",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getCompositingMode",
                    "()Ljavax/microedition/m3g/CompositingMode;",
                    Self::get_compositing_mode,
                    Default::default(),
                ),
                JavaMethodProto::new("getFog", "()Ljavax/microedition/m3g/Fog;", Self::get_fog, Default::default()),
                JavaMethodProto::new(
                    "getMaterial",
                    "()Ljavax/microedition/m3g/Material;",
                    Self::get_material,
                    Default::default(),
                ),
                JavaMethodProto::new("getLayer", "()I", Self::get_layer, Default::default()),
                JavaMethodProto::new(
                    "getPolygonMode",
                    "()Ljavax/microedition/m3g/PolygonMode;",
                    Self::get_polygon_mode,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getTexture",
                    "(I)Ljavax/microedition/m3g/Texture2D;",
                    Self::get_texture,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setCompositingMode",
                    "(Ljavax/microedition/m3g/CompositingMode;)V",
                    Self::set_compositing_mode,
                    Default::default(),
                ),
                JavaMethodProto::new("setFog", "(Ljavax/microedition/m3g/Fog;)V", Self::set_fog, Default::default()),
                JavaMethodProto::new("setLayer", "(I)V", Self::set_layer, Default::default()),
                JavaMethodProto::new(
                    "setMaterial",
                    "(Ljavax/microedition/m3g/Material;)V",
                    Self::set_material,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setPolygonMode",
                    "(Ljavax/microedition/m3g/PolygonMode;)V",
                    Self::set_polygon_mode,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setTexture",
                    "(ILjavax/microedition/m3g/Texture2D;)V",
                    Self::set_texture,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("compositingMode", "Ljavax/microedition/m3g/CompositingMode;", Default::default()),
                JavaFieldProto::new("fog", "Ljavax/microedition/m3g/Fog;", Default::default()),
                JavaFieldProto::new("layer", "I", Default::default()),
                JavaFieldProto::new("polygonMode", "Ljavax/microedition/m3g/PolygonMode;", Default::default()),
                JavaFieldProto::new("material", "Ljavax/microedition/m3g/Material;", Default::default()),
                JavaFieldProto::new("texture0", "Ljavax/microedition/m3g/Texture2D;", Default::default()),
                JavaFieldProto::new("texture1", "Ljavax/microedition/m3g/Texture2D;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await
    }

    pub(super) async fn get_compositing_mode(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<CompositingMode>> {
        jvm.get_field(&this, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;").await
    }

    pub(super) async fn get_fog(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Fog>> {
        jvm.get_field(&this, "fog", "Ljavax/microedition/m3g/Fog;").await
    }

    pub(super) async fn get_material(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Material>> {
        jvm.get_field(&this, "material", "Ljavax/microedition/m3g/Material;").await
    }

    pub(super) async fn get_layer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "layer", "I").await
    }

    pub(super) async fn get_polygon_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<PolygonMode>> {
        jvm.get_field(&this, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;").await
    }

    pub(super) async fn get_texture(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        unit: i32,
    ) -> Result<ClassInstanceRef<Texture2D>> {
        match unit {
            0 => jvm.get_field(&this, "texture0", "Ljavax/microedition/m3g/Texture2D;").await,
            1 if M3G_TEXTURE_UNITS > 1 => jvm.get_field(&this, "texture1", "Ljavax/microedition/m3g/Texture2D;").await,
            _ => Err(jvm.exception("java/lang/IndexOutOfBoundsException", "texture unit").await),
        }
    }

    pub(super) async fn set_compositing_mode(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        mode: ClassInstanceRef<CompositingMode>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;", mode)
            .await
    }

    pub(super) async fn set_fog(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, fog: ClassInstanceRef<Fog>) -> Result<()> {
        jvm.put_field(&mut this, "fog", "Ljavax/microedition/m3g/Fog;", fog).await
    }

    pub(super) async fn set_layer(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, layer: i32) -> Result<()> {
        if !(-63..=63).contains(&layer) {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "bad appearance layer").await);
        }
        jvm.put_field(&mut this, "layer", "I", layer).await
    }

    pub(super) async fn set_material(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        material: ClassInstanceRef<Material>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "material", "Ljavax/microedition/m3g/Material;", material).await
    }

    pub(super) async fn set_polygon_mode(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        mode: ClassInstanceRef<PolygonMode>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;", mode)
            .await
    }

    pub(super) async fn set_texture(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        unit: i32,
        texture: ClassInstanceRef<Texture2D>,
    ) -> Result<()> {
        match unit {
            0 => {
                jvm.put_field(&mut this, "texture0", "Ljavax/microedition/m3g/Texture2D;", texture)
                    .await?;
            }
            1 if M3G_TEXTURE_UNITS > 1 => {
                jvm.put_field(&mut this, "texture1", "Ljavax/microedition/m3g/Texture2D;", texture)
                    .await?;
            }
            _ => return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "texture unit").await),
        }
        Ok(())
    }
}
