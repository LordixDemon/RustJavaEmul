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

impl Transform {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Transform",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljavax/microedition/m3g/Transform;)V", Self::init_copy, Default::default()),
                JavaMethodProto::new("get", "([F)V", Self::get, Default::default()),
                JavaMethodProto::new("invert", "()V", Self::invert, Default::default()),
                JavaMethodProto::new(
                    "postMultiply",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::post_multiply,
                    Default::default(),
                ),
                JavaMethodProto::new("set", "([F)V", Self::set, Default::default()),
                JavaMethodProto::new("set", "(Ljavax/microedition/m3g/Transform;)V", Self::set_transform, Default::default()),
                JavaMethodProto::new("setIdentity", "()V", Self::set_identity, Default::default()),
                JavaMethodProto::new("postRotate", "(FFFF)V", Self::post_rotate, Default::default()),
                JavaMethodProto::new("postRotateQuat", "(FFFF)V", Self::post_rotate_quat, Default::default()),
                JavaMethodProto::new("postScale", "(FFF)V", Self::post_scale, Default::default()),
                JavaMethodProto::new("postTranslate", "(FFF)V", Self::post_translate, Default::default()),
                JavaMethodProto::new("transform", "([F)V", Self::transform_float_array, Default::default()),
                JavaMethodProto::new(
                    "transform",
                    "(Ljavax/microedition/m3g/VertexArray;[FZ)V",
                    Self::transform_vertex_array,
                    Default::default(),
                ),
                JavaMethodProto::new("transpose", "()V", Self::transpose, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("matrix", "[F", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::put_matrix(jvm, &mut this, identity_matrix()).await
    }

    pub(super) async fn init_copy(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, source: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform source").await);
        }
        let matrix = Self::matrix(jvm, &source).await?;
        Self::put_matrix(jvm, &mut this, matrix).await
    }

    pub(super) async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut dst: ClassInstanceRef<Array<f32>>) -> Result<()> {
        if dst.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.get").await);
        }
        if jvm.array_length(&dst).await? < 16 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "matrix array too small").await);
        }
        let matrix = Self::matrix(jvm, &this).await?;
        store_raw_f32_array(jvm, &mut dst, &matrix).await
    }

    pub(super) async fn set(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, src: ClassInstanceRef<Array<f32>>) -> Result<()> {
        if src.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.set").await);
        }
        if jvm.array_length(&src).await? < 16 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "matrix array too small").await);
        }
        let matrix = raw_f32_array(jvm, &src, 16).await?;
        Self::put_matrix(jvm, &mut this, matrix_to_array(&matrix)).await
    }

    pub(super) async fn set_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<Self>,
    ) -> Result<()> {
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.set").await);
        }
        let matrix = Self::matrix(jvm, &source).await?;
        Self::put_matrix(jvm, &mut this, matrix).await
    }

    pub(super) async fn set_identity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        Self::put_matrix(jvm, &mut this, identity_matrix()).await
    }

    pub(super) async fn invert(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        let Some(inverted) = invert_matrix(matrix) else {
            return Err(jvm.exception("java/lang/ArithmeticException", "singular transform").await);
        };
        Self::put_matrix(jvm, &mut this, inverted).await
    }

    pub(super) async fn transpose(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        Self::put_matrix(jvm, &mut this, transpose_matrix(matrix)).await
    }

    pub(super) async fn post_multiply(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<()> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.postMultiply").await);
        }
        let matrix = Self::matrix(jvm, &this).await?;
        let other = Self::matrix(jvm, &other).await?;
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, other)).await
    }

    pub(super) async fn post_rotate(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        angle: f32,
        ax: f32,
        ay: f32,
        az: f32,
    ) -> Result<()> {
        if angle != 0.0 && ax == 0.0 && ay == 0.0 && az == 0.0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "zero rotation axis").await);
        }
        let matrix = Self::matrix(jvm, &this).await?;
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, rotation_matrix(angle, ax, ay, az))).await
    }

    pub(super) async fn post_rotate_quat(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        qx: f32,
        qy: f32,
        qz: f32,
        qw: f32,
    ) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        let Some(rotation) = quaternion_matrix(qx, qy, qz, qw) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "zero quaternion").await);
        };
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, rotation)).await
    }

    pub(super) async fn post_scale(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sx: f32, sy: f32, sz: f32) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, scale_matrix(sx, sy, sz))).await
    }

    pub(super) async fn post_translate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, tx: f32, ty: f32, tz: f32) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, translation_matrix(tx, ty, tz))).await
    }

    pub(super) async fn transform_float_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut values: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if values.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.transform").await);
        }
        let len = jvm.array_length(&values).await?;
        if len % 4 != 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "vector array length").await);
        }
        if len == 0 {
            return Ok(());
        }
        let matrix = Self::matrix(jvm, &this).await?;
        let mut data = raw_f32_array(jvm, &values, len).await?;
        for vector in data.chunks_exact_mut(4) {
            let transformed = transform_vec4(matrix, [vector[0], vector[1], vector[2], vector[3]]);
            vector.copy_from_slice(&transformed);
        }
        store_raw_f32_array(jvm, &mut values, &data).await
    }

    pub(super) async fn transform_vertex_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<VertexArray>,
        mut out: ClassInstanceRef<Array<f32>>,
        w: bool,
    ) -> Result<()> {
        if array.is_null() || out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.transform").await);
        }
        let component_count: i32 = jvm.get_field(&array, "componentCount", "I").await?;
        let vertex_count: i32 = jvm.get_field(&array, "vertexCount", "I").await?;
        if component_count == 4 || jvm.array_length(&out).await? < vertex_count.max(0) as usize * 4 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "output array too small").await);
        }

        let matrix = Self::matrix(jvm, &this).await?;
        let components = Graphics3D::vertex_array_components(jvm, &array).await?;
        let mut transformed = Vec::with_capacity(components.len() * 4);
        for vertex in components {
            let value = transform_vec4(
                matrix,
                [
                    vertex.first().copied().unwrap_or(0.0),
                    vertex.get(1).copied().unwrap_or(0.0),
                    vertex.get(2).copied().unwrap_or(0.0),
                    if w { 1.0 } else { 0.0 },
                ],
            );
            transformed.extend_from_slice(&value);
        }
        store_raw_f32_array(jvm, &mut out, &transformed).await
    }

    pub(super) async fn matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        if this.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform").await);
        }
        let matrix: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "matrix", "[F").await?;
        let values = raw_f32_array(jvm, &matrix, 16).await?;
        Ok(matrix_to_array(&values))
    }

    pub(super) async fn put_matrix(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, matrix: [f32; 16]) -> Result<()> {
        if this.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform").await);
        }
        let mut array = jvm.instantiate_array("F", 16).await?;
        store_raw_f32_array(jvm, &mut array, &matrix).await?;
        jvm.put_field(this, "matrix", "[F", array).await
    }
}
