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

impl Light {
    pub const AMBIENT: i32 = 128;
    pub const DIRECTIONAL: i32 = 129;
    pub const OMNI: i32 = 130;
    pub const SPOT: i32 = 131;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Light",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getColor", "()I", Self::get_color, Default::default()),
                JavaMethodProto::new("getConstantAttenuation", "()F", Self::get_constant_attenuation, Default::default()),
                JavaMethodProto::new("getIntensity", "()F", Self::get_intensity, Default::default()),
                JavaMethodProto::new("getLinearAttenuation", "()F", Self::get_linear_attenuation, Default::default()),
                JavaMethodProto::new("getMode", "()I", Self::get_mode, Default::default()),
                JavaMethodProto::new("getQuadraticAttenuation", "()F", Self::get_quadratic_attenuation, Default::default()),
                JavaMethodProto::new("getSpotAngle", "()F", Self::get_spot_angle, Default::default()),
                JavaMethodProto::new("getSpotExponent", "()F", Self::get_spot_exponent, Default::default()),
                JavaMethodProto::new("setAttenuation", "(FFF)V", Self::set_attenuation, Default::default()),
                JavaMethodProto::new("setColor", "(I)V", Self::set_color, Default::default()),
                JavaMethodProto::new("setIntensity", "(F)V", Self::set_intensity, Default::default()),
                JavaMethodProto::new("setMode", "(I)V", Self::set_mode, Default::default()),
                JavaMethodProto::new("setSpotAngle", "(F)V", Self::set_spot_angle, Default::default()),
                JavaMethodProto::new("setSpotExponent", "(F)V", Self::set_spot_exponent, Default::default()),
            ],
            fields: vec![
                static_int_field("AMBIENT"),
                static_int_field("DIRECTIONAL"),
                static_int_field("OMNI"),
                static_int_field("SPOT"),
                JavaFieldProto::new("constantAttenuation", "F", Default::default()),
                JavaFieldProto::new("linearAttenuation", "F", Default::default()),
                JavaFieldProto::new("quadraticAttenuation", "F", Default::default()),
                JavaFieldProto::new("color", "I", Default::default()),
                JavaFieldProto::new("intensity", "F", Default::default()),
                JavaFieldProto::new("mode", "I", Default::default()),
                JavaFieldProto::new("spotAngle", "F", Default::default()),
                JavaFieldProto::new("spotExponent", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/Light",
            &[
                ("AMBIENT", Self::AMBIENT),
                ("DIRECTIONAL", Self::DIRECTIONAL),
                ("OMNI", Self::OMNI),
                ("SPOT", Self::SPOT),
            ],
        )
        .await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "constantAttenuation", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "linearAttenuation", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "quadraticAttenuation", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "color", "I", 0x00ff_ffff).await?;
        jvm.put_field(&mut this, "intensity", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "mode", "I", Self::DIRECTIONAL).await?;
        jvm.put_field(&mut this, "spotAngle", "F", 45.0f32).await?;
        jvm.put_field(&mut this, "spotExponent", "F", 0.0f32).await
    }

    pub(super) async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "color", "I").await
    }

    pub(super) async fn get_constant_attenuation(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "constantAttenuation", "F").await
    }

    pub(super) async fn get_intensity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "intensity", "F").await
    }

    pub(super) async fn get_linear_attenuation(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "linearAttenuation", "F").await
    }

    pub(super) async fn get_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "mode", "I").await
    }

    pub(super) async fn get_quadratic_attenuation(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "quadraticAttenuation", "F").await
    }

    pub(super) async fn get_spot_angle(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "spotAngle", "F").await
    }

    pub(super) async fn get_spot_exponent(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "spotExponent", "F").await
    }

    pub(super) async fn set_attenuation(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        constant: f32,
        linear: f32,
        quadratic: f32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "constantAttenuation", "F", constant).await?;
        jvm.put_field(&mut this, "linearAttenuation", "F", linear).await?;
        jvm.put_field(&mut this, "quadraticAttenuation", "F", quadratic).await
    }

    pub(super) async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, color: i32) -> Result<()> {
        jvm.put_field(&mut this, "color", "I", color & 0x00ff_ffff).await
    }

    pub(super) async fn set_intensity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, intensity: f32) -> Result<()> {
        jvm.put_field(&mut this, "intensity", "F", intensity).await
    }

    pub(super) async fn set_mode(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, mode: i32) -> Result<()> {
        let mode = match mode {
            Self::AMBIENT | Self::DIRECTIONAL | Self::OMNI | Self::SPOT => mode,
            _ => Self::DIRECTIONAL,
        };
        jvm.put_field(&mut this, "mode", "I", mode).await
    }

    pub(super) async fn set_spot_angle(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: f32) -> Result<()> {
        jvm.put_field(&mut this, "spotAngle", "F", angle).await
    }

    pub(super) async fn set_spot_exponent(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, exponent: f32) -> Result<()> {
        jvm.put_field(&mut this, "spotExponent", "F", exponent).await
    }
}
