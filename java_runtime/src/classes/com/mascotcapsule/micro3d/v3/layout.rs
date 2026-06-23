use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

use super::{
    AffineTrans,
    constants::{
        AFFINE_TRANS_CLASS, FIGURE_LAYOUT_CLASS, PROJECTION_PARALLEL_SCALE, PROJECTION_PARALLEL_SIZE, PROJECTION_PERSPECTIVE_FOV,
        PROJECTION_PERSPECTIVE_WH,
    },
    math::identity_matrix,
    storage::{get_affine_matrix, put_int_array_field},
};

// class com.mascotcapsule.micro3d.v3.FigureLayout
pub struct FigureLayout;

impl FigureLayout {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: FIGURE_LAYOUT_CLASS,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;IIII)V",
                    Self::init_with_state,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getAffineTrans",
                    "()Lcom/mascotcapsule/micro3d/v3/AffineTrans;",
                    Self::get_affine_trans,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setAffineTrans",
                    "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V",
                    Self::set_affine_trans,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setAffineTrans",
                    "([Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V",
                    Self::set_affine_trans_array,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setAffineTransArray",
                    "([Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V",
                    Self::set_affine_trans_array,
                    Default::default(),
                ),
                JavaMethodProto::new("selectAffineTrans", "(I)V", Self::select_affine_trans, Default::default()),
                JavaMethodProto::new("getCenterX", "()I", Self::get_center_x, Default::default()),
                JavaMethodProto::new("getCenterY", "()I", Self::get_center_y, Default::default()),
                JavaMethodProto::new("setCenter", "(II)V", Self::set_center, Default::default()),
                JavaMethodProto::new("setPerspective", "(III)V", Self::set_perspective, Default::default()),
                JavaMethodProto::new("setPerspective", "(IIII)V", Self::set_perspective_with_projection, Default::default()),
                JavaMethodProto::new("getParallelWidth", "()I", Self::get_parallel_width, Default::default()),
                JavaMethodProto::new("getParallelHeight", "()I", Self::get_parallel_height, Default::default()),
                JavaMethodProto::new("setParallelSize", "(II)V", Self::set_parallel_size, Default::default()),
                JavaMethodProto::new("getScaleX", "()I", Self::get_scale_x, Default::default()),
                JavaMethodProto::new("getScaleY", "()I", Self::get_scale_y, Default::default()),
                JavaMethodProto::new("setScale", "(II)V", Self::set_scale, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("affineTrans", "Lcom/mascotcapsule/micro3d/v3/AffineTrans;", Default::default()),
                JavaFieldProto::new("affineArray", "[Lcom/mascotcapsule/micro3d/v3/AffineTrans;", Default::default()),
                JavaFieldProto::new("affineMatrix", "[I", Default::default()),
                JavaFieldProto::new("centerX", "I", Default::default()),
                JavaFieldProto::new("centerY", "I", Default::default()),
                JavaFieldProto::new("near", "I", Default::default()),
                JavaFieldProto::new("far", "I", Default::default()),
                JavaFieldProto::new("perspective", "I", Default::default()),
                JavaFieldProto::new("projection", "I", Default::default()),
                JavaFieldProto::new("projectionMode", "I", Default::default()),
                JavaFieldProto::new("parallelWidth", "I", Default::default()),
                JavaFieldProto::new("parallelHeight", "I", Default::default()),
                JavaFieldProto::new("scaleX", "I", Default::default()),
                JavaFieldProto::new("scaleY", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.FigureLayout::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let affine: ClassInstanceRef<AffineTrans> = jvm.new_class(AFFINE_TRANS_CLASS, "()V", ()).await?.into();
        jvm.put_field(&mut this, "affineTrans", "Lcom/mascotcapsule/micro3d/v3/AffineTrans;", affine.clone())
            .await?;
        let mut affine_array = jvm.instantiate_array("Lcom/mascotcapsule/micro3d/v3/AffineTrans;", 1).await?;
        jvm.store_array(&mut affine_array, 0, [affine]).await?;
        jvm.put_field(&mut this, "affineArray", "[Lcom/mascotcapsule/micro3d/v3/AffineTrans;", affine_array)
            .await?;
        put_int_array_field(jvm, &mut this, "affineMatrix", identity_matrix().to_vec()).await?;
        jvm.put_field(&mut this, "centerX", "I", context.screen_width() / 2).await?;
        jvm.put_field(&mut this, "centerY", "I", context.screen_height() / 2).await?;
        jvm.put_field(&mut this, "near", "I", 1).await?;
        jvm.put_field(&mut this, "far", "I", 4096).await?;
        jvm.put_field(&mut this, "perspective", "I", 512).await?;
        jvm.put_field(&mut this, "projection", "I", 0).await?;
        jvm.put_field(&mut this, "projectionMode", "I", PROJECTION_PARALLEL_SCALE).await?;
        jvm.put_field(&mut this, "parallelWidth", "I", 0).await?;
        jvm.put_field(&mut this, "parallelHeight", "I", 0).await?;
        jvm.put_field(&mut this, "scaleX", "I", 4096).await?;
        jvm.put_field(&mut this, "scaleY", "I", 4096).await
    }

    async fn init_with_state(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        affine: ClassInstanceRef<AffineTrans>,
        scale_x: i32,
        scale_y: i32,
        center_x: i32,
        center_y: i32,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        Self::set_affine_trans(jvm, context, this.clone(), affine).await?;
        Self::set_center(jvm, context, this.clone(), center_x, center_y).await?;
        Self::set_scale(jvm, context, this, scale_x, scale_y).await
    }

    async fn get_affine_trans(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<AffineTrans>> {
        jvm.get_field(&this, "affineTrans", "Lcom/mascotcapsule/micro3d/v3/AffineTrans;").await
    }

    async fn set_affine_trans(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        affine: ClassInstanceRef<AffineTrans>,
    ) -> Result<()> {
        let affine = if affine.is_null() {
            jvm.new_class(AFFINE_TRANS_CLASS, "()V", ()).await?.into()
        } else {
            affine
        };
        let matrix = get_affine_matrix(jvm, &affine).await?;
        jvm.put_field(&mut this, "affineTrans", "Lcom/mascotcapsule/micro3d/v3/AffineTrans;", affine)
            .await?;
        put_int_array_field(jvm, &mut this, "affineMatrix", matrix.to_vec()).await
    }

    async fn set_affine_trans_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<AffineTrans>>,
    ) -> Result<()> {
        if array.is_null() || jvm.array_length(&array).await? == 0 {
            return Err(jvm.exception("java/lang/NullPointerException", "FigureLayout affine array").await);
        }
        jvm.put_field(&mut this, "affineArray", "[Lcom/mascotcapsule/micro3d/v3/AffineTrans;", array)
            .await?;
        Self::select_affine_trans(jvm, context, this, 0).await
    }

    async fn select_affine_trans(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        let array: ClassInstanceRef<Array<AffineTrans>> = jvm.get_field(&this, "affineArray", "[Lcom/mascotcapsule/micro3d/v3/AffineTrans;").await?;
        if array.is_null() || index < 0 || index as usize >= jvm.array_length(&array).await? {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "FigureLayout affine index").await);
        }
        let values: Vec<ClassInstanceRef<AffineTrans>> = jvm.load_array(&array, index as usize, 1).await?;
        let affine = values.into_iter().next().unwrap_or_else(|| ClassInstanceRef::<AffineTrans>::new(None));
        if affine.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "FigureLayout affine").await);
        }
        let matrix = get_affine_matrix(jvm, &affine).await?;
        jvm.put_field(&mut this, "affineTrans", "Lcom/mascotcapsule/micro3d/v3/AffineTrans;", affine)
            .await?;
        put_int_array_field(jvm, &mut this, "affineMatrix", matrix.to_vec()).await
    }

    async fn get_center_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "centerX", "I").await
    }

    async fn get_center_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "centerY", "I").await
    }

    async fn set_center(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        jvm.put_field(&mut this, "centerX", "I", x).await?;
        jvm.put_field(&mut this, "centerY", "I", y).await
    }

    async fn set_perspective(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        near: i32,
        far: i32,
        perspective: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "near", "I", near).await?;
        jvm.put_field(&mut this, "far", "I", far).await?;
        jvm.put_field(&mut this, "perspective", "I", perspective).await?;
        jvm.put_field(&mut this, "projectionMode", "I", PROJECTION_PERSPECTIVE_FOV).await
    }

    async fn set_perspective_with_projection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        near: i32,
        far: i32,
        perspective: i32,
        projection: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "near", "I", near).await?;
        jvm.put_field(&mut this, "far", "I", far).await?;
        jvm.put_field(&mut this, "perspective", "I", perspective).await?;
        jvm.put_field(&mut this, "projection", "I", projection).await?;
        jvm.put_field(&mut this, "projectionMode", "I", PROJECTION_PERSPECTIVE_WH).await
    }

    async fn set_scale(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, scale_x: i32, scale_y: i32) -> Result<()> {
        jvm.put_field(&mut this, "scaleX", "I", scale_x).await?;
        jvm.put_field(&mut this, "scaleY", "I", scale_y).await?;
        jvm.put_field(&mut this, "projectionMode", "I", PROJECTION_PARALLEL_SCALE).await
    }

    async fn get_scale_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "scaleX", "I").await
    }

    async fn get_scale_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "scaleY", "I").await
    }

    async fn get_parallel_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "parallelWidth", "I").await
    }

    async fn get_parallel_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "parallelHeight", "I").await
    }

    async fn set_parallel_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, width: i32, height: i32) -> Result<()> {
        jvm.put_field(&mut this, "parallelWidth", "I", width).await?;
        jvm.put_field(&mut this, "parallelHeight", "I", height).await?;
        jvm.put_field(&mut this, "projectionMode", "I", PROJECTION_PARALLEL_SIZE).await
    }
}
