mod bind;
mod cache;
mod draw;
mod lights;
mod scene;
mod state;

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

impl super::Graphics3D {
    pub const ANTIALIAS: i32 = 2;
    pub const DITHER: i32 = 4;
    pub const TRUE_COLOR: i32 = 8;
    pub const OVERWRITE: i32 = 16;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Graphics3D",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "addLight",
                    "(Ljavax/microedition/m3g/Light;Ljavax/microedition/m3g/Transform;)I",
                    Self::add_light,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getInstance",
                    "()Ljavax/microedition/m3g/Graphics3D;",
                    Self::get_instance,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("bindTarget", "(Ljava/lang/Object;)V", Self::bind_target, Default::default()),
                JavaMethodProto::new(
                    "bindTarget",
                    "(Ljava/lang/Object;ZI)V",
                    Self::bind_target_with_options,
                    Default::default(),
                ),
                JavaMethodProto::new("clear", "(Ljavax/microedition/m3g/Background;)V", Self::clear, Default::default()),
                JavaMethodProto::new(
                    "getCamera",
                    "(Ljavax/microedition/m3g/Transform;)Ljavax/microedition/m3g/Camera;",
                    Self::get_camera,
                    Default::default(),
                ),
                JavaMethodProto::new("getDepthRangeFar", "()F", Self::get_depth_range_far, Default::default()),
                JavaMethodProto::new("getDepthRangeNear", "()F", Self::get_depth_range_near, Default::default()),
                JavaMethodProto::new("getHints", "()I", Self::get_hints, Default::default()),
                JavaMethodProto::new(
                    "getLight",
                    "(ILjavax/microedition/m3g/Transform;)Ljavax/microedition/m3g/Light;",
                    Self::get_light,
                    Default::default(),
                ),
                JavaMethodProto::new("getLightCount", "()I", Self::get_light_count, Default::default()),
                JavaMethodProto::new(
                    "getProperties",
                    "()Ljava/util/Hashtable;",
                    Self::get_properties,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getTarget", "()Ljava/lang/Object;", Self::get_target, Default::default()),
                JavaMethodProto::new("getViewportHeight", "()I", Self::get_viewport_height, Default::default()),
                JavaMethodProto::new("getViewportWidth", "()I", Self::get_viewport_width, Default::default()),
                JavaMethodProto::new("getViewportX", "()I", Self::get_viewport_x, Default::default()),
                JavaMethodProto::new("getViewportY", "()I", Self::get_viewport_y, Default::default()),
                JavaMethodProto::new("isDepthBufferEnabled", "()Z", Self::is_depth_buffer_enabled, Default::default()),
                JavaMethodProto::new("releaseTarget", "()V", Self::release_target, Default::default()),
                JavaMethodProto::new("render", "(Ljavax/microedition/m3g/World;)V", Self::render_world, Default::default()),
                JavaMethodProto::new(
                    "render",
                    "(Ljavax/microedition/m3g/Node;Ljavax/microedition/m3g/Transform;)V",
                    Self::render_node,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "render",
                    "(Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;Ljavax/microedition/m3g/Transform;)V",
                    Self::render_vb,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "render",
                    "(Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;Ljavax/microedition/m3g/Transform;I)V",
                    Self::render_vb_scoped,
                    Default::default(),
                ),
                JavaMethodProto::new("resetLights", "()V", Self::reset_lights, Default::default()),
                JavaMethodProto::new(
                    "setCamera",
                    "(Ljavax/microedition/m3g/Camera;Ljavax/microedition/m3g/Transform;)V",
                    Self::set_camera,
                    Default::default(),
                ),
                JavaMethodProto::new("setDepthRange", "(FF)V", Self::set_depth_range, Default::default()),
                JavaMethodProto::new(
                    "setLight",
                    "(ILjavax/microedition/m3g/Light;Ljavax/microedition/m3g/Transform;)V",
                    Self::set_light,
                    Default::default(),
                ),
                JavaMethodProto::new("setViewport", "(IIII)V", Self::set_viewport, Default::default()),
            ],
            fields: vec![
                static_int_field("ANTIALIAS"),
                static_int_field("DITHER"),
                static_int_field("TRUE_COLOR"),
                static_int_field("OVERWRITE"),
                JavaFieldProto::new("target", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("viewportX", "I", Default::default()),
                JavaFieldProto::new("viewportY", "I", Default::default()),
                JavaFieldProto::new("viewportW", "I", Default::default()),
                JavaFieldProto::new("viewportH", "I", Default::default()),
                JavaFieldProto::new("camera", "Ljavax/microedition/m3g/Camera;", Default::default()),
                JavaFieldProto::new("cameraTransform", "[F", Default::default()),
                JavaFieldProto::new("cameraTransformSet", "Z", Default::default()),
                JavaFieldProto::new("colorBuffer", "[I", Default::default()),
                JavaFieldProto::new("colorBufferValid", "Z", Default::default()),
                JavaFieldProto::new("depthBuffer", "[F", Default::default()),
                JavaFieldProto::new("depthEnabled", "Z", Default::default()),
                JavaFieldProto::new("depthRangeNear", "F", Default::default()),
                JavaFieldProto::new("depthRangeFar", "F", Default::default()),
                JavaFieldProto::new("hints", "I", Default::default()),
                JavaFieldProto::new("lights", "[Ljavax/microedition/m3g/Light;", Default::default()),
                JavaFieldProto::new("lightTransforms", "[Ljavax/microedition/m3g/Transform;", Default::default()),
                JavaFieldProto::new("instance", "Ljavax/microedition/m3g/Graphics3D;", FieldAccessFlags::STATIC),
            ],
            access_flags: Default::default(),
        }
    }
    pub(crate) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/Graphics3D",
            &[
                ("ANTIALIAS", Self::ANTIALIAS),
                ("DITHER", Self::DITHER),
                ("TRUE_COLOR", Self::TRUE_COLOR),
                ("OVERWRITE", Self::OVERWRITE),
            ],
        )
        .await?;
        jvm.put_static_field(
            "javax/microedition/m3g/Graphics3D",
            "instance",
            "Ljavax/microedition/m3g/Graphics3D;",
            null_ref::<Self>(),
        )
        .await
    }
    pub(crate) async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "viewportX", "I", 0).await?;
        jvm.put_field(&mut this, "viewportY", "I", 0).await?;
        jvm.put_field(&mut this, "viewportW", "I", context.screen_width()).await?;
        jvm.put_field(&mut this, "viewportH", "I", context.screen_height()).await?;
        jvm.put_field(&mut this, "camera", "Ljavax/microedition/m3g/Camera;", null_ref::<Camera>())
            .await?;
        Self::put_camera_transform(jvm, &mut this, identity_matrix()).await?;
        jvm.put_field(&mut this, "cameraTransformSet", "Z", false).await?;
        jvm.put_field(&mut this, "depthEnabled", "Z", true).await?;
        jvm.put_field(&mut this, "depthRangeNear", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "depthRangeFar", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "hints", "I", 0).await?;
        let lights = jvm.instantiate_array("Ljavax/microedition/m3g/Light;", 0).await?;
        let light_transforms = jvm.instantiate_array("Ljavax/microedition/m3g/Transform;", 0).await?;
        jvm.put_field(&mut this, "lights", "[Ljavax/microedition/m3g/Light;", lights).await?;
        jvm.put_field(&mut this, "lightTransforms", "[Ljavax/microedition/m3g/Transform;", light_transforms)
            .await?;
        Self::clear_framebuffers(jvm, &mut this).await
    }
    pub(crate) async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let existing: ClassInstanceRef<Self> = jvm
            .get_static_field("javax/microedition/m3g/Graphics3D", "instance", "Ljavax/microedition/m3g/Graphics3D;")
            .await
            .unwrap_or_else(|_| null_ref());
        if !existing.is_null() {
            return Ok(existing);
        }
        let created: ClassInstanceRef<Self> = jvm.new_class("javax/microedition/m3g/Graphics3D", "()V", ()).await?.into();
        let raced: ClassInstanceRef<Self> = jvm
            .get_static_field("javax/microedition/m3g/Graphics3D", "instance", "Ljavax/microedition/m3g/Graphics3D;")
            .await
            .unwrap_or_else(|_| null_ref());
        if !raced.is_null() {
            return Ok(raced);
        }
        jvm.put_static_field(
            "javax/microedition/m3g/Graphics3D",
            "instance",
            "Ljavax/microedition/m3g/Graphics3D;",
            created.clone(),
        )
        .await?;
        Ok(created)
    }
}
