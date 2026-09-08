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

impl Material {
    pub const AMBIENT: i32 = 1024;
    pub const DIFFUSE: i32 = 2048;
    pub const EMISSIVE: i32 = 4096;
    pub const SPECULAR: i32 = 8192;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Material",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getColor", "(I)I", Self::get_color, Default::default()),
                JavaMethodProto::new("getShininess", "()F", Self::get_shininess, Default::default()),
                JavaMethodProto::new(
                    "isVertexColorTrackingEnabled",
                    "()Z",
                    Self::is_vertex_color_tracking_enabled,
                    Default::default(),
                ),
                JavaMethodProto::new("setColor", "(II)V", Self::set_color, Default::default()),
                JavaMethodProto::new("setShininess", "(F)V", Self::set_shininess, Default::default()),
                JavaMethodProto::new(
                    "setVertexColorTrackingEnable",
                    "(Z)V",
                    Self::set_vertex_color_tracking_enable,
                    Default::default(),
                ),
            ],
            fields: vec![
                static_int_field("AMBIENT"),
                static_int_field("DIFFUSE"),
                static_int_field("EMISSIVE"),
                static_int_field("SPECULAR"),
                JavaFieldProto::new("ambientColor", "I", Default::default()),
                JavaFieldProto::new("diffuseColor", "I", Default::default()),
                JavaFieldProto::new("emissiveColor", "I", Default::default()),
                JavaFieldProto::new("specularColor", "I", Default::default()),
                JavaFieldProto::new("shininess", "F", Default::default()),
                JavaFieldProto::new("vertexColorTracking", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/Material",
            &[
                ("AMBIENT", Self::AMBIENT),
                ("DIFFUSE", Self::DIFFUSE),
                ("EMISSIVE", Self::EMISSIVE),
                ("SPECULAR", Self::SPECULAR),
            ],
        )
        .await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "ambientColor", "I", 0x0033_3333).await?;
        jvm.put_field(&mut this, "diffuseColor", "I", 0xffff_ffffu32 as i32).await?;
        jvm.put_field(&mut this, "emissiveColor", "I", 0).await?;
        jvm.put_field(&mut this, "specularColor", "I", 0).await?;
        jvm.put_field(&mut this, "shininess", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "vertexColorTracking", "Z", false).await
    }

    pub(super) async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: i32) -> Result<i32> {
        let field = if target & Self::AMBIENT != 0 {
            "ambientColor"
        } else if target & Self::DIFFUSE != 0 {
            "diffuseColor"
        } else if target & Self::EMISSIVE != 0 {
            "emissiveColor"
        } else if target & Self::SPECULAR != 0 {
            "specularColor"
        } else {
            "diffuseColor"
        };
        jvm.get_field(&this, field, "I").await
    }

    pub(super) async fn get_shininess(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "shininess", "F").await
    }

    pub(super) async fn is_vertex_color_tracking_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "vertexColorTracking", "Z").await
    }

    pub(super) async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, target: i32, color: i32) -> Result<()> {
        if target & Self::AMBIENT != 0 {
            jvm.put_field(&mut this, "ambientColor", "I", ensure_opaque(color)).await?;
        }
        if target & Self::DIFFUSE != 0 {
            jvm.put_field(&mut this, "diffuseColor", "I", color).await?;
        }
        if target & Self::EMISSIVE != 0 {
            jvm.put_field(&mut this, "emissiveColor", "I", ensure_opaque(color)).await?;
        }
        if target & Self::SPECULAR != 0 {
            jvm.put_field(&mut this, "specularColor", "I", ensure_opaque(color)).await?;
        }
        Ok(())
    }

    pub(super) async fn set_shininess(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, shininess: f32) -> Result<()> {
        jvm.put_field(&mut this, "shininess", "F", shininess).await
    }

    pub(super) async fn set_vertex_color_tracking_enable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        enabled: bool,
    ) -> Result<()> {
        jvm.put_field(&mut this, "vertexColorTracking", "Z", enabled).await
    }
}
