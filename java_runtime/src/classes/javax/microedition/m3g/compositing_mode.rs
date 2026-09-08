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

impl CompositingMode {
    pub const ALPHA: i32 = 64;
    pub const ALPHA_ADD: i32 = 65;
    pub const MODULATE: i32 = 66;
    pub const MODULATE_X2: i32 = 67;
    pub const REPLACE: i32 = 68;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/CompositingMode",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getAlphaThreshold", "()F", Self::get_alpha_threshold, Default::default()),
                JavaMethodProto::new("getBlending", "()I", Self::get_blending, Default::default()),
                JavaMethodProto::new("setAlphaThreshold", "(F)V", Self::set_alpha_threshold, Default::default()),
                JavaMethodProto::new("setAlphaWriteEnable", "(Z)V", Self::set_alpha_write_enable, Default::default()),
                JavaMethodProto::new("setBlending", "(I)V", Self::set_blending, Default::default()),
                JavaMethodProto::new("setColorWriteEnable", "(Z)V", Self::set_color_write_enable, Default::default()),
                JavaMethodProto::new("getDepthOffsetFactor", "()F", Self::get_depth_offset_factor, Default::default()),
                JavaMethodProto::new("getDepthOffsetUnits", "()F", Self::get_depth_offset_units, Default::default()),
                JavaMethodProto::new("isAlphaWriteEnabled", "()Z", Self::is_alpha_write_enabled, Default::default()),
                JavaMethodProto::new("isColorWriteEnabled", "()Z", Self::is_color_write_enabled, Default::default()),
                JavaMethodProto::new("isDepthTestEnabled", "()Z", Self::is_depth_test_enabled, Default::default()),
                JavaMethodProto::new("isDepthWriteEnabled", "()Z", Self::is_depth_write_enabled, Default::default()),
                JavaMethodProto::new("setDepthOffset", "(FF)V", Self::set_depth_offset, Default::default()),
                JavaMethodProto::new("setDepthTestEnable", "(Z)V", Self::set_depth_test_enable, Default::default()),
                JavaMethodProto::new("setDepthWriteEnable", "(Z)V", Self::set_depth_write_enable, Default::default()),
            ],
            fields: vec![
                static_int_field("ALPHA"),
                static_int_field("ALPHA_ADD"),
                static_int_field("MODULATE"),
                static_int_field("MODULATE_X2"),
                static_int_field("REPLACE"),
                JavaFieldProto::new("alphaThreshold", "F", Default::default()),
                JavaFieldProto::new("alphaWrite", "Z", Default::default()),
                JavaFieldProto::new("blending", "I", Default::default()),
                JavaFieldProto::new("colorWrite", "Z", Default::default()),
                JavaFieldProto::new("depthTest", "Z", Default::default()),
                JavaFieldProto::new("depthWrite", "Z", Default::default()),
                JavaFieldProto::new("depthOffsetFactor", "F", Default::default()),
                JavaFieldProto::new("depthOffsetUnits", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/CompositingMode",
            &[
                ("ALPHA", Self::ALPHA),
                ("ALPHA_ADD", Self::ALPHA_ADD),
                ("MODULATE", Self::MODULATE),
                ("MODULATE_X2", Self::MODULATE_X2),
                ("REPLACE", Self::REPLACE),
            ],
        )
        .await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "alphaWrite", "Z", true).await?;
        jvm.put_field(&mut this, "blending", "I", Self::REPLACE).await?;
        jvm.put_field(&mut this, "colorWrite", "Z", true).await?;
        jvm.put_field(&mut this, "depthTest", "Z", true).await?;
        jvm.put_field(&mut this, "depthWrite", "Z", true).await?;
        jvm.put_field(&mut this, "depthOffsetFactor", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "depthOffsetUnits", "F", 0.0f32).await
    }

    pub(super) async fn get_alpha_threshold(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "alphaThreshold", "F").await
    }

    pub(super) async fn get_blending(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "blending", "I").await
    }

    pub(super) async fn set_alpha_threshold(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        jvm.put_field(&mut this, "alphaThreshold", "F", value).await
    }
    pub(super) async fn set_alpha_write_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "alphaWrite", "Z", value).await
    }
    pub(super) async fn set_blending(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "blending", "I", value).await
    }
    pub(super) async fn set_color_write_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "colorWrite", "Z", value).await
    }
    pub(super) async fn get_depth_offset_factor(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "depthOffsetFactor", "F").await
    }
    pub(super) async fn get_depth_offset_units(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "depthOffsetUnits", "F").await
    }
    pub(super) async fn is_alpha_write_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "alphaWrite", "Z").await
    }
    pub(super) async fn is_color_write_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "colorWrite", "Z").await
    }
    pub(super) async fn is_depth_test_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "depthTest", "Z").await
    }
    pub(super) async fn is_depth_write_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "depthWrite", "Z").await
    }
    pub(super) async fn set_depth_offset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, factor: f32, units: f32) -> Result<()> {
        jvm.put_field(&mut this, "depthOffsetFactor", "F", factor).await?;
        jvm.put_field(&mut this, "depthOffsetUnits", "F", units).await
    }
    pub(super) async fn set_depth_test_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "depthTest", "Z", value).await
    }
    pub(super) async fn set_depth_write_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "depthWrite", "Z", value).await
    }
}
