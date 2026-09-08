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

impl Fog {
    pub const EXPONENTIAL: i32 = 80;
    pub const LINEAR: i32 = 81;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Fog",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getColor", "()I", Self::get_color, Default::default()),
                JavaMethodProto::new("getDensity", "()F", Self::get_density, Default::default()),
                JavaMethodProto::new("getFarDistance", "()F", Self::get_far_distance, Default::default()),
                JavaMethodProto::new("getMode", "()I", Self::get_mode, Default::default()),
                JavaMethodProto::new("getNearDistance", "()F", Self::get_near_distance, Default::default()),
                JavaMethodProto::new("setColor", "(I)V", Self::set_color, Default::default()),
                JavaMethodProto::new("setDensity", "(F)V", Self::set_density, Default::default()),
                JavaMethodProto::new("setLinear", "(FF)V", Self::set_linear, Default::default()),
                JavaMethodProto::new("setMode", "(I)V", Self::set_mode, Default::default()),
            ],
            fields: vec![
                static_int_field("EXPONENTIAL"),
                static_int_field("LINEAR"),
                JavaFieldProto::new("color", "I", Default::default()),
                JavaFieldProto::new("density", "F", Default::default()),
                JavaFieldProto::new("mode", "I", Default::default()),
                JavaFieldProto::new("near", "F", Default::default()),
                JavaFieldProto::new("far", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/Fog",
            &[("EXPONENTIAL", Self::EXPONENTIAL), ("LINEAR", Self::LINEAR)],
        )
        .await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "color", "I", 0).await?;
        jvm.put_field(&mut this, "density", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "mode", "I", Self::LINEAR).await?;
        jvm.put_field(&mut this, "near", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "far", "F", 1.0f32).await
    }
    pub(super) async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "color", "I").await
    }
    pub(super) async fn get_density(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "density", "F").await
    }
    pub(super) async fn get_far_distance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "far", "F").await
    }
    pub(super) async fn get_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "mode", "I").await
    }
    pub(super) async fn get_near_distance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "near", "F").await
    }
    pub(super) async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "color", "I", value).await
    }
    pub(super) async fn set_density(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, density: f32) -> Result<()> {
        jvm.put_field(&mut this, "density", "F", density).await
    }
    pub(super) async fn set_linear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, near: f32, far: f32) -> Result<()> {
        jvm.put_field(&mut this, "mode", "I", Self::LINEAR).await?;
        jvm.put_field(&mut this, "near", "F", near).await?;
        jvm.put_field(&mut this, "far", "F", far).await
    }
    pub(super) async fn set_mode(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, mode: i32) -> Result<()> {
        let mode = match mode {
            Self::EXPONENTIAL | Self::LINEAR => mode,
            _ => Self::LINEAR,
        };
        jvm.put_field(&mut this, "mode", "I", mode).await
    }
}
