use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

use super::{
    Vector3D,
    constants::{EFFECT_3D_CLASS, LIGHT_CLASS},
    texture::Texture,
};

// class com.mascotcapsule.micro3d.v3.Effect3D
pub struct Effect3D;

impl Effect3D {
    const CLASS_NAME: &'static str = EFFECT_3D_CLASS;
    const NORMAL_SHADING: i32 = 0;
    const TOON_SHADING: i32 = 1;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: EFFECT_3D_CLASS,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Lcom/mascotcapsule/micro3d/v3/Light;IZLcom/mascotcapsule/micro3d/v3/Texture;)V",
                    Self::init_with_state,
                    Default::default(),
                ),
                JavaMethodProto::new("getLight", "()Lcom/mascotcapsule/micro3d/v3/Light;", Self::get_light, Default::default()),
                JavaMethodProto::new("setLight", "(Lcom/mascotcapsule/micro3d/v3/Light;)V", Self::set_light, Default::default()),
                JavaMethodProto::new("getShading", "()I", Self::get_shading_type, Default::default()),
                JavaMethodProto::new("getShadingType", "()I", Self::get_shading_type, Default::default()),
                JavaMethodProto::new("setShading", "(I)V", Self::set_shading_type, Default::default()),
                JavaMethodProto::new("setShadingType", "(I)V", Self::set_shading_type, Default::default()),
                JavaMethodProto::new("getThreshold", "()I", Self::get_toon_threshold, Default::default()),
                JavaMethodProto::new("getToonThreshold", "()I", Self::get_toon_threshold, Default::default()),
                JavaMethodProto::new("getThresholdHigh", "()I", Self::get_toon_high, Default::default()),
                JavaMethodProto::new("getToonHigh", "()I", Self::get_toon_high, Default::default()),
                JavaMethodProto::new("getThresholdLow", "()I", Self::get_toon_low, Default::default()),
                JavaMethodProto::new("getToonLow", "()I", Self::get_toon_low, Default::default()),
                JavaMethodProto::new("setThreshold", "(III)V", Self::set_toon_params, Default::default()),
                JavaMethodProto::new("setToonParams", "(III)V", Self::set_toon_params, Default::default()),
                JavaMethodProto::new("isSemiTransparentEnabled", "()Z", Self::is_transparency, Default::default()),
                JavaMethodProto::new("isTransparency", "()Z", Self::is_transparency, Default::default()),
                JavaMethodProto::new("setSemiTransparentEnabled", "(Z)V", Self::set_transparency, Default::default()),
                JavaMethodProto::new("setTransparency", "(Z)V", Self::set_transparency, Default::default()),
                JavaMethodProto::new(
                    "getSphereMap",
                    "()Lcom/mascotcapsule/micro3d/v3/Texture;",
                    Self::get_sphere_texture,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getSphereTexture",
                    "()Lcom/mascotcapsule/micro3d/v3/Texture;",
                    Self::get_sphere_texture,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setSphereMap",
                    "(Lcom/mascotcapsule/micro3d/v3/Texture;)V",
                    Self::set_sphere_texture,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setSphereTexture",
                    "(Lcom/mascotcapsule/micro3d/v3/Texture;)V",
                    Self::set_sphere_texture,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("NORMAL_SHADING", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TOON_SHADING", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("light", "Lcom/mascotcapsule/micro3d/v3/Light;", Default::default()),
                JavaFieldProto::new("shadingType", "I", Default::default()),
                JavaFieldProto::new("transparency", "Z", Default::default()),
                JavaFieldProto::new("sphereTexture", "Lcom/mascotcapsule/micro3d/v3/Texture;", Default::default()),
                JavaFieldProto::new("toonThreshold", "I", Default::default()),
                JavaFieldProto::new("toonHigh", "I", Default::default()),
                JavaFieldProto::new("toonLow", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field(Self::CLASS_NAME, "NORMAL_SHADING", "I", Self::NORMAL_SHADING)
            .await?;
        jvm.put_static_field(Self::CLASS_NAME, "TOON_SHADING", "I", Self::TOON_SHADING).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Effect3D::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::put_state(
            jvm,
            &mut this,
            ClassInstanceRef::<Light>::new(None),
            0,
            true,
            ClassInstanceRef::<Texture>::new(None),
        )
        .await
    }

    async fn init_with_state(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        light: ClassInstanceRef<Light>,
        shading_type: i32,
        transparency: bool,
        sphere_texture: ClassInstanceRef<Texture>,
    ) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Effect3D::<init>({this:?}, {light:?}, {shading_type:?}, {transparency:?}, {sphere_texture:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::put_state(jvm, &mut this, light, shading_type, transparency, sphere_texture).await
    }

    async fn put_state(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        light: ClassInstanceRef<Light>,
        shading_type: i32,
        transparency: bool,
        sphere_texture: ClassInstanceRef<Texture>,
    ) -> Result<()> {
        jvm.put_field(this, "light", "Lcom/mascotcapsule/micro3d/v3/Light;", light).await?;
        jvm.put_field(this, "shadingType", "I", shading_type).await?;
        jvm.put_field(this, "transparency", "Z", transparency).await?;
        jvm.put_field(this, "sphereTexture", "Lcom/mascotcapsule/micro3d/v3/Texture;", sphere_texture)
            .await?;
        jvm.put_field(this, "toonThreshold", "I", 0).await?;
        jvm.put_field(this, "toonHigh", "I", 255).await?;
        jvm.put_field(this, "toonLow", "I", 0).await
    }

    async fn get_light(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Light>> {
        jvm.get_field(&this, "light", "Lcom/mascotcapsule/micro3d/v3/Light;").await
    }

    async fn set_light(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, light: ClassInstanceRef<Light>) -> Result<()> {
        jvm.put_field(&mut this, "light", "Lcom/mascotcapsule/micro3d/v3/Light;", light).await
    }

    async fn set_shading_type(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, shading_type: i32) -> Result<()> {
        jvm.put_field(&mut this, "shadingType", "I", shading_type).await
    }

    async fn get_shading_type(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "shadingType", "I").await
    }

    async fn get_toon_threshold(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "toonThreshold", "I").await
    }

    async fn get_toon_high(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "toonHigh", "I").await
    }

    async fn get_toon_low(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "toonLow", "I").await
    }

    async fn set_toon_params(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, threshold: i32, high: i32, low: i32) -> Result<()> {
        jvm.put_field(&mut this, "toonThreshold", "I", threshold.clamp(0, 255)).await?;
        jvm.put_field(&mut this, "toonHigh", "I", high.clamp(0, 255)).await?;
        jvm.put_field(&mut this, "toonLow", "I", low.clamp(0, 255)).await
    }

    async fn is_transparency(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "transparency", "Z").await
    }

    async fn set_transparency(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, transparency: bool) -> Result<()> {
        jvm.put_field(&mut this, "transparency", "Z", transparency).await
    }

    async fn get_sphere_texture(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Texture>> {
        jvm.get_field(&this, "sphereTexture", "Lcom/mascotcapsule/micro3d/v3/Texture;").await
    }

    async fn set_sphere_texture(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        texture: ClassInstanceRef<Texture>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "sphereTexture", "Lcom/mascotcapsule/micro3d/v3/Texture;", texture)
            .await
    }
}

// class com.mascotcapsule.micro3d.v3.Light
pub struct Light;

impl Light {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: LIGHT_CLASS,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;II)V",
                    Self::init_with_state,
                    Default::default(),
                ),
                JavaMethodProto::new("getAmbIntensity", "()I", Self::get_ambient_intensity, Default::default()),
                JavaMethodProto::new("getAmbientIntensity", "()I", Self::get_ambient_intensity, Default::default()),
                JavaMethodProto::new("setAmbIntensity", "(I)V", Self::set_ambient_intensity, Default::default()),
                JavaMethodProto::new("setAmbientIntensity", "(I)V", Self::set_ambient_intensity, Default::default()),
                JavaMethodProto::new(
                    "getDirection",
                    "()Lcom/mascotcapsule/micro3d/v3/Vector3D;",
                    Self::get_direction,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getParallelLightDirection",
                    "()Lcom/mascotcapsule/micro3d/v3/Vector3D;",
                    Self::get_direction,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setDirection",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::set_direction,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setParallelLightDirection",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::set_direction,
                    Default::default(),
                ),
                JavaMethodProto::new("getDirIntensity", "()I", Self::get_diffuse_intensity, Default::default()),
                JavaMethodProto::new("getParallelLightIntensity", "()I", Self::get_diffuse_intensity, Default::default()),
                JavaMethodProto::new("setDirIntensity", "(I)V", Self::set_diffuse_intensity, Default::default()),
                JavaMethodProto::new("setParallelLightIntensity", "(I)V", Self::set_diffuse_intensity, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("direction", "Lcom/mascotcapsule/micro3d/v3/Vector3D;", Default::default()),
                JavaFieldProto::new("ambientIntensity", "I", Default::default()),
                JavaFieldProto::new("diffuseIntensity", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Light::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let direction = jvm
            .new_class("com/mascotcapsule/micro3d/v3/Vector3D", "(III)V", (0, 0, 4096))
            .await?
            .into();
        Self::put_state(jvm, &mut this, direction, 0, 4096).await
    }

    async fn init_with_state(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        direction: ClassInstanceRef<Vector3D>,
        diffuse_intensity: i32,
        ambient_intensity: i32,
    ) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Light::<init>({this:?}, {direction:?}, {ambient_intensity:?}, {diffuse_intensity:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if direction.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Light direction").await);
        }
        Self::put_state(jvm, &mut this, direction, ambient_intensity, diffuse_intensity).await
    }

    async fn put_state(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        direction: ClassInstanceRef<Vector3D>,
        ambient_intensity: i32,
        diffuse_intensity: i32,
    ) -> Result<()> {
        jvm.put_field(this, "direction", "Lcom/mascotcapsule/micro3d/v3/Vector3D;", direction)
            .await?;
        jvm.put_field(this, "ambientIntensity", "I", ambient_intensity).await?;
        jvm.put_field(this, "diffuseIntensity", "I", diffuse_intensity).await
    }

    async fn get_direction(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Vector3D>> {
        jvm.get_field(&this, "direction", "Lcom/mascotcapsule/micro3d/v3/Vector3D;").await
    }

    async fn set_direction(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, direction: ClassInstanceRef<Vector3D>) -> Result<()> {
        if direction.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Light direction").await);
        }
        jvm.put_field(&mut this, "direction", "Lcom/mascotcapsule/micro3d/v3/Vector3D;", direction)
            .await
    }

    async fn get_ambient_intensity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "ambientIntensity", "I").await
    }

    async fn set_ambient_intensity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, intensity: i32) -> Result<()> {
        jvm.put_field(&mut this, "ambientIntensity", "I", intensity).await
    }

    async fn get_diffuse_intensity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "diffuseIntensity", "I").await
    }

    async fn set_diffuse_intensity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, intensity: i32) -> Result<()> {
        jvm.put_field(&mut this, "diffuseIntensity", "I", intensity).await
    }
}
