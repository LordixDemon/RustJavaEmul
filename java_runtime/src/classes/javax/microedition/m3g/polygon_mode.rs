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

impl PolygonMode {
    pub const CULL_BACK: i32 = 160;
    pub const CULL_FRONT: i32 = 161;
    pub const CULL_NONE: i32 = 162;
    pub const SHADE_FLAT: i32 = 164;
    pub const SHADE_SMOOTH: i32 = 165;
    pub const WINDING_CCW: i32 = 168;
    pub const WINDING_CW: i32 = 169;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/PolygonMode",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getCulling", "()I", Self::get_culling, Default::default()),
                JavaMethodProto::new("getShading", "()I", Self::get_shading, Default::default()),
                JavaMethodProto::new("getWinding", "()I", Self::get_winding, Default::default()),
                JavaMethodProto::new(
                    "isLocalCameraLightingEnabled",
                    "()Z",
                    Self::is_local_camera_lighting_enabled,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "isPerspectiveCorrectionEnabled",
                    "()Z",
                    Self::is_perspective_correction_enabled,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "isTwoSidedLightingEnabled",
                    "()Z",
                    Self::is_two_sided_lighting_enabled,
                    Default::default(),
                ),
                JavaMethodProto::new("setCulling", "(I)V", Self::set_culling, Default::default()),
                JavaMethodProto::new(
                    "setLocalCameraLightingEnable",
                    "(Z)V",
                    Self::set_local_camera_lighting_enable,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setPerspectiveCorrectionEnable",
                    "(Z)V",
                    Self::set_perspective_correction_enable,
                    Default::default(),
                ),
                JavaMethodProto::new("setShading", "(I)V", Self::set_shading, Default::default()),
                JavaMethodProto::new(
                    "setTwoSidedLightingEnable",
                    "(Z)V",
                    Self::set_two_sided_lighting_enable,
                    Default::default(),
                ),
                JavaMethodProto::new("setWinding", "(I)V", Self::set_winding, Default::default()),
            ],
            fields: vec![
                static_int_field("CULL_BACK"),
                static_int_field("CULL_FRONT"),
                static_int_field("CULL_NONE"),
                static_int_field("SHADE_FLAT"),
                static_int_field("SHADE_SMOOTH"),
                static_int_field("WINDING_CCW"),
                static_int_field("WINDING_CW"),
                JavaFieldProto::new("culling", "I", Default::default()),
                JavaFieldProto::new("localCameraLighting", "Z", Default::default()),
                JavaFieldProto::new("perspectiveCorrection", "Z", Default::default()),
                JavaFieldProto::new("shading", "I", Default::default()),
                JavaFieldProto::new("twoSidedLighting", "Z", Default::default()),
                JavaFieldProto::new("winding", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/PolygonMode",
            &[
                ("CULL_BACK", Self::CULL_BACK),
                ("CULL_FRONT", Self::CULL_FRONT),
                ("CULL_NONE", Self::CULL_NONE),
                ("SHADE_FLAT", Self::SHADE_FLAT),
                ("SHADE_SMOOTH", Self::SHADE_SMOOTH),
                ("WINDING_CCW", Self::WINDING_CCW),
                ("WINDING_CW", Self::WINDING_CW),
            ],
        )
        .await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "culling", "I", Self::CULL_BACK).await?;
        jvm.put_field(&mut this, "localCameraLighting", "Z", false).await?;
        jvm.put_field(&mut this, "perspectiveCorrection", "Z", true).await?;
        jvm.put_field(&mut this, "shading", "I", Self::SHADE_SMOOTH).await?;
        jvm.put_field(&mut this, "twoSidedLighting", "Z", false).await?;
        jvm.put_field(&mut this, "winding", "I", Self::WINDING_CCW).await
    }
    pub(super) async fn get_culling(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "culling", "I").await
    }
    pub(super) async fn get_shading(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "shading", "I").await
    }
    pub(super) async fn get_winding(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "winding", "I").await
    }
    pub(super) async fn is_local_camera_lighting_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "localCameraLighting", "Z").await
    }
    pub(super) async fn is_perspective_correction_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "perspectiveCorrection", "Z").await
    }
    pub(super) async fn is_two_sided_lighting_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "twoSidedLighting", "Z").await
    }
    pub(super) async fn set_culling(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "culling", "I", value).await
    }
    pub(super) async fn set_local_camera_lighting_enable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: bool,
    ) -> Result<()> {
        jvm.put_field(&mut this, "localCameraLighting", "Z", value).await
    }
    pub(super) async fn set_perspective_correction_enable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: bool,
    ) -> Result<()> {
        jvm.put_field(&mut this, "perspectiveCorrection", "Z", value).await
    }
    pub(super) async fn set_shading(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "shading", "I", value).await
    }
    pub(super) async fn set_two_sided_lighting_enable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: bool,
    ) -> Result<()> {
        jvm.put_field(&mut this, "twoSidedLighting", "Z", value).await
    }
    pub(super) async fn set_winding(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "winding", "I", value).await
    }
}
