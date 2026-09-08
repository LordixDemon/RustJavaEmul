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

impl Camera {
    pub const GENERIC: i32 = 48;
    pub const PARALLEL: i32 = 49;
    pub const PERSPECTIVE: i32 = 50;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Camera",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getProjection",
                    "(Ljavax/microedition/m3g/Transform;)I",
                    Self::get_projection_transform,
                    Default::default(),
                ),
                JavaMethodProto::new("getProjection", "([F)I", Self::get_projection_params, Default::default()),
                JavaMethodProto::new(
                    "setGeneric",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::set_generic,
                    Default::default(),
                ),
                JavaMethodProto::new("setParallel", "(FFFF)V", Self::set_parallel, Default::default()),
                JavaMethodProto::new("setPerspective", "(FFFF)V", Self::set_perspective, Default::default()),
            ],
            fields: vec![
                static_int_field("GENERIC"),
                static_int_field("PARALLEL"),
                static_int_field("PERSPECTIVE"),
                JavaFieldProto::new("projectionMode", "I", Default::default()),
                JavaFieldProto::new("fovy", "F", Default::default()),
                JavaFieldProto::new("parallelHeight", "F", Default::default()),
                JavaFieldProto::new("aspect", "F", Default::default()),
                JavaFieldProto::new("near", "F", Default::default()),
                JavaFieldProto::new("far", "F", Default::default()),
                JavaFieldProto::new("genericProjection", "[F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/Camera",
            &[
                ("GENERIC", Self::GENERIC),
                ("PARALLEL", Self::PARALLEL),
                ("PERSPECTIVE", Self::PERSPECTIVE),
            ],
        )
        .await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "projectionMode", "I", Self::PERSPECTIVE).await?;
        jvm.put_field(&mut this, "fovy", "F", 45.0f32).await?;
        jvm.put_field(&mut this, "parallelHeight", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "aspect", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "near", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "far", "F", 1000.0f32).await?;
        Self::put_generic_projection(jvm, &mut this, identity_matrix()).await
    }

    pub(super) async fn set_generic(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        if transform.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Camera.setGeneric").await);
        }
        let matrix = Transform::matrix(jvm, &transform).await?;
        jvm.put_field(&mut this, "projectionMode", "I", Self::GENERIC).await?;
        Self::put_generic_projection(jvm, &mut this, matrix).await
    }

    pub(super) async fn set_parallel(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        height: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Result<()> {
        if height <= 0.0 || aspect <= 0.0 || near == far {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid parallel projection").await);
        }
        jvm.put_field(&mut this, "projectionMode", "I", Self::PARALLEL).await?;
        jvm.put_field(&mut this, "parallelHeight", "F", height).await?;
        jvm.put_field(&mut this, "fovy", "F", height).await?;
        jvm.put_field(&mut this, "aspect", "F", aspect).await?;
        jvm.put_field(&mut this, "near", "F", near).await?;
        jvm.put_field(&mut this, "far", "F", far).await
    }

    pub(super) async fn set_perspective(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        fovy: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Result<()> {
        if fovy <= 0.0 || fovy >= 180.0 || aspect <= 0.0 || near <= 0.0 || far <= 0.0 {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "invalid perspective projection")
                .await);
        }
        jvm.put_field(&mut this, "projectionMode", "I", Self::PERSPECTIVE).await?;
        jvm.put_field(&mut this, "fovy", "F", fovy).await?;
        jvm.put_field(&mut this, "aspect", "F", aspect).await?;
        jvm.put_field(&mut this, "near", "F", near).await?;
        jvm.put_field(&mut this, "far", "F", far).await
    }

    pub(super) async fn get_projection_params(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut params: ClassInstanceRef<Array<f32>>,
    ) -> Result<i32> {
        let mode = Self::projection_mode(jvm, &this).await?;
        if !params.is_null() && mode != Self::GENERIC {
            if jvm.array_length(&params).await? < 4 {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "projection params array too small")
                    .await);
            }
            let first = if mode == Self::PARALLEL {
                jvm.get_field::<f32>(&this, "parallelHeight", "F").await.unwrap_or(1.0)
            } else {
                jvm.get_field::<f32>(&this, "fovy", "F").await.unwrap_or(45.0)
            };
            let values = vec![
                first,
                jvm.get_field::<f32>(&this, "aspect", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(&this, "near", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(&this, "far", "F").await.unwrap_or(1000.0),
            ];
            jvm.store_array(&mut params, 0, values).await?;
        }
        Ok(mode)
    }

    pub(super) async fn get_projection_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut transform: ClassInstanceRef<Transform>,
    ) -> Result<i32> {
        let mode = Self::projection_mode(jvm, &this).await?;
        if !transform.is_null() {
            let matrix = Self::projection_matrix(jvm, &this, mode).await?;
            Transform::put_matrix(jvm, &mut transform, matrix).await?;
        }
        Ok(mode)
    }

    pub(super) async fn projection_mode(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field(this, "projectionMode", "I").await.unwrap_or(Self::PERSPECTIVE))
    }

    pub(super) async fn generic_projection(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let matrix: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "genericProjection", "[F").await?;
        let values: Vec<f32> = jvm.load_array(&matrix, 0, 16).await?;
        Ok(matrix_to_array(&values))
    }

    pub(super) async fn put_generic_projection(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, matrix: [f32; 16]) -> Result<()> {
        let mut array = jvm.instantiate_array("F", 16).await?;
        jvm.store_array(&mut array, 0, matrix).await?;
        jvm.put_field(this, "genericProjection", "[F", array).await
    }

    pub(super) async fn projection_matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>, mode: i32) -> Result<[f32; 16]> {
        match mode {
            Self::GENERIC => Self::generic_projection(jvm, this).await,
            Self::PARALLEL => Ok(parallel_projection_matrix(
                jvm.get_field::<f32>(this, "parallelHeight", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "aspect", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "near", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "far", "F").await.unwrap_or(1000.0),
            )),
            _ => Ok(perspective_projection_matrix(
                jvm.get_field::<f32>(this, "fovy", "F").await.unwrap_or(45.0),
                jvm.get_field::<f32>(this, "aspect", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "near", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "far", "F").await.unwrap_or(1000.0),
            )),
        }
    }
}
