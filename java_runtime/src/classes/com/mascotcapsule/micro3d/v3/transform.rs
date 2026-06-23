use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

use super::{
    constants::{AFFINE_TRANS_CLASS, VECTOR_3D_CLASS},
    math::{cross3, dot3, fixed_mul, fixed_mul3, identity_matrix, mul_matrix, normalize3, sin_cos_mc},
    storage::{get_affine_matrix, get_vector, put_affine_matrix},
};

// class com.mascotcapsule.micro3d.v3.Vector3D
pub struct Vector3D;

impl Vector3D {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: VECTOR_3D_CLASS,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init_default, Default::default()),
                JavaMethodProto::new("<init>", "(III)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::init_copy,
                    Default::default(),
                ),
                JavaMethodProto::new("getX", "()I", Self::get_x, Default::default()),
                JavaMethodProto::new("getY", "()I", Self::get_y, Default::default()),
                JavaMethodProto::new("getZ", "()I", Self::get_z, Default::default()),
                JavaMethodProto::new("set", "(III)V", Self::set, Default::default()),
                JavaMethodProto::new(
                    "set",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::set_from_vector,
                    Default::default(),
                ),
                JavaMethodProto::new("setX", "(I)V", Self::set_x, Default::default()),
                JavaMethodProto::new("setY", "(I)V", Self::set_y, Default::default()),
                JavaMethodProto::new("setZ", "(I)V", Self::set_z, Default::default()),
                JavaMethodProto::new("unit", "()V", Self::unit, Default::default()),
                JavaMethodProto::new(
                    "innerProduct",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)I",
                    Self::inner_product,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "innerProduct",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)I",
                    Self::inner_product_static,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "outerProduct",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::outer_product,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "outerProduct",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)Lcom/mascotcapsule/micro3d/v3/Vector3D;",
                    Self::outer_product_static,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "outerProduct",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::outer_product_static_into,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("x", "I", Default::default()),
                JavaFieldProto::new("y", "I", Default::default()),
                JavaFieldProto::new("z", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32, z: i32) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Vector3D::<init>({this:?}, {x:?}, {y:?}, {z:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::put(jvm, &mut this, x, y, z).await
    }

    async fn init_default(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        Self::init(jvm, context, this, 0, 0, 0).await
    }

    async fn init_copy(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, vector: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if vector.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D copy source").await);
        }
        let (x, y, z) = get_vector(jvm, &vector).await?;
        Self::put(jvm, &mut this, x, y, z).await
    }

    async fn get_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "x", "I").await
    }

    async fn get_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "y", "I").await
    }

    async fn get_z(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "z", "I").await
    }

    async fn set(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32, z: i32) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Vector3D::set({this:?}, {x:?}, {y:?}, {z:?})");

        Self::put(jvm, &mut this, x, y, z).await
    }

    async fn set_from_vector(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, vector: ClassInstanceRef<Self>) -> Result<()> {
        if vector.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D.set source").await);
        }
        let (x, y, z) = get_vector(jvm, &vector).await?;
        Self::put(jvm, &mut this, x, y, z).await
    }

    async fn set_x(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Vector3D::setX({this:?}, {x:?})");

        jvm.put_field(&mut this, "x", "I", x).await
    }

    async fn set_y(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, y: i32) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Vector3D::setY({this:?}, {y:?})");

        jvm.put_field(&mut this, "y", "I", y).await
    }

    async fn set_z(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, z: i32) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Vector3D::setZ({this:?}, {z:?})");

        jvm.put_field(&mut this, "z", "I", z).await
    }

    async fn unit(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let vector = normalize3(get_vector(jvm, &this).await?);
        Self::put(jvm, &mut this, vector.0, vector.1, vector.2).await
    }

    async fn inner_product(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        Self::inner_product_refs(jvm, &this, &other).await
    }

    async fn inner_product_static(jvm: &Jvm, _: &mut RuntimeContext, lhs: ClassInstanceRef<Self>, rhs: ClassInstanceRef<Self>) -> Result<i32> {
        Self::inner_product_refs(jvm, &lhs, &rhs).await
    }

    async fn inner_product_refs(jvm: &Jvm, lhs: &ClassInstanceRef<Self>, rhs: &ClassInstanceRef<Self>) -> Result<i32> {
        if lhs.is_null() || rhs.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D.innerProduct").await);
        }
        Ok(raw_dot3(get_vector(jvm, lhs).await?, get_vector(jvm, rhs).await?))
    }

    async fn outer_product(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<()> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D.outerProduct").await);
        }
        let result = cross3(get_vector(jvm, &this).await?, get_vector(jvm, &other).await?);
        Self::put(jvm, &mut this, result.0, result.1, result.2).await
    }

    async fn outer_product_static(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        lhs: ClassInstanceRef<Self>,
        rhs: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        if lhs.is_null() || rhs.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D.outerProduct").await);
        }
        let result = cross3(get_vector(jvm, &lhs).await?, get_vector(jvm, &rhs).await?);
        let vector: ClassInstanceRef<Self> = jvm.new_class(VECTOR_3D_CLASS, "()V", ()).await?.into();
        Self::set(jvm, context, vector.clone(), result.0, result.1, result.2).await?;
        Ok(vector)
    }

    async fn outer_product_static_into(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        lhs: ClassInstanceRef<Self>,
        rhs: ClassInstanceRef<Self>,
        mut result: ClassInstanceRef<Self>,
    ) -> Result<()> {
        if lhs.is_null() || rhs.is_null() || result.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D.outerProduct").await);
        }
        let value = cross3(get_vector(jvm, &lhs).await?, get_vector(jvm, &rhs).await?);
        Self::put(jvm, &mut result, value.0, value.1, value.2).await
    }

    async fn put(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, x: i32, y: i32, z: i32) -> Result<()> {
        jvm.put_field(this, "x", "I", x).await?;
        jvm.put_field(this, "y", "I", y).await?;
        jvm.put_field(this, "z", "I", z).await
    }
}

fn raw_dot3(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    a.0.wrapping_mul(b.0)
        .wrapping_add(a.1.wrapping_mul(b.1))
        .wrapping_add(a.2.wrapping_mul(b.2))
}

// class com.mascotcapsule.micro3d.v3.AffineTrans
pub struct AffineTrans;

impl AffineTrans {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: AFFINE_TRANS_CLASS,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(IIIIIIIIIIII)V", Self::init_values, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V",
                    Self::init_copy,
                    Default::default(),
                ),
                JavaMethodProto::new("<init>", "([I)V", Self::init_from_array, Default::default()),
                JavaMethodProto::new("<init>", "([[I)V", Self::init_from_array_2d, Default::default()),
                JavaMethodProto::new("<init>", "([II)V", Self::init_from_array_offset, Default::default()),
                JavaMethodProto::new("set", "([I)V", Self::set_from_array, Default::default()),
                JavaMethodProto::new("set", "([[I)V", Self::set_from_array_2d, Default::default()),
                JavaMethodProto::new("set", "([II)V", Self::set_from_array_offset, Default::default()),
                JavaMethodProto::new("set", "(IIIIIIIIIIII)V", Self::set_values, Default::default()),
                JavaMethodProto::new(
                    "set",
                    "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V",
                    Self::set_from_affine,
                    Default::default(),
                ),
                JavaMethodProto::new("setIdentity", "()V", Self::set_identity, Default::default()),
                JavaMethodProto::new("rotationX", "(I)V", Self::rotation_x, Default::default()),
                JavaMethodProto::new("rotationY", "(I)V", Self::rotation_y, Default::default()),
                JavaMethodProto::new("rotationZ", "(I)V", Self::rotation_z, Default::default()),
                JavaMethodProto::new("setRotationX", "(I)V", Self::rotation_x, Default::default()),
                JavaMethodProto::new("setRotationY", "(I)V", Self::rotation_y, Default::default()),
                JavaMethodProto::new("setRotationZ", "(I)V", Self::rotation_z, Default::default()),
                JavaMethodProto::new("mul", "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V", Self::mul, Default::default()),
                JavaMethodProto::new("multiply", "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V", Self::mul, Default::default()),
                JavaMethodProto::new(
                    "mul",
                    "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V",
                    Self::mul_two,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "multiply",
                    "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V",
                    Self::mul_two,
                    Default::default(),
                ),
                JavaMethodProto::new("get", "([I)V", Self::get_to_array, Default::default()),
                JavaMethodProto::new("get", "([II)V", Self::get_to_array_offset, Default::default()),
                JavaMethodProto::new(
                    "transform",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)Lcom/mascotcapsule/micro3d/v3/Vector3D;",
                    Self::transform,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "transPoint",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)Lcom/mascotcapsule/micro3d/v3/Vector3D;",
                    Self::transform,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "rotationV",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;I)V",
                    Self::set_rotation,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setRotation",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;I)V",
                    Self::set_rotation,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "lookAt",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::look_at,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setViewTrans",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::look_at,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "rotate",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::rotate_vector,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "scale",
                    "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)V",
                    Self::scale_vector,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("rotationX", "I", Default::default()),
                JavaFieldProto::new("rotationY", "I", Default::default()),
                JavaFieldProto::new("rotationZ", "I", Default::default()),
                JavaFieldProto::new("m00", "I", Default::default()),
                JavaFieldProto::new("m01", "I", Default::default()),
                JavaFieldProto::new("m02", "I", Default::default()),
                JavaFieldProto::new("m03", "I", Default::default()),
                JavaFieldProto::new("m10", "I", Default::default()),
                JavaFieldProto::new("m11", "I", Default::default()),
                JavaFieldProto::new("m12", "I", Default::default()),
                JavaFieldProto::new("m13", "I", Default::default()),
                JavaFieldProto::new("m20", "I", Default::default()),
                JavaFieldProto::new("m21", "I", Default::default()),
                JavaFieldProto::new("m22", "I", Default::default()),
                JavaFieldProto::new("m23", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.AffineTrans::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let _: () = jvm.invoke_virtual(&this, "setIdentity", "()V", ()).await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn init_values(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        m00: i32,
        m01: i32,
        m02: i32,
        m03: i32,
        m10: i32,
        m11: i32,
        m12: i32,
        m13: i32,
        m20: i32,
        m21: i32,
        m22: i32,
        m23: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::set_values(jvm, context, this, m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23).await
    }

    async fn init_copy(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, source: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::set_from_affine(jvm, context, this, source).await
    }

    async fn init_from_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::set_from_array(jvm, context, this, values).await
    }

    async fn init_from_array_offset(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<i32>>,
        offset: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::set_from_array_offset(jvm, context, this, values, offset).await
    }

    async fn init_from_array_2d(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<Array<i32>>>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::set_from_array_2d(jvm, context, this, values).await
    }

    async fn set_from_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        Self::set_from_array_offset(jvm, context, this, values, 0).await
    }

    async fn set_from_array_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<i32>>,
        offset: i32,
    ) -> Result<()> {
        if values.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.set array").await);
        }

        let offset = offset.max(0) as usize;
        let length = jvm.array_length(&values).await?;
        if length.saturating_sub(offset) < 12 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "AffineTrans array length").await);
        }
        let values = jvm.load_array(&values, offset, 12).await?;
        let mut matrix = identity_matrix();
        matrix[..values.len()].copy_from_slice(&values);
        put_affine_matrix(jvm, &mut this, matrix).await
    }

    async fn set_from_array_2d(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<Array<i32>>>,
    ) -> Result<()> {
        if values.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.set matrix").await);
        }
        if jvm.array_length(&values).await? < 3 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "AffineTrans matrix rows").await);
        }

        let rows: Vec<ClassInstanceRef<Array<i32>>> = jvm.load_array(&values, 0, 3).await?;
        let mut matrix = [0; 12];
        for (row_index, row) in rows.iter().enumerate() {
            if row.is_null() || jvm.array_length(row).await? < 4 {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "AffineTrans matrix row length").await);
            }
            let values: Vec<i32> = jvm.load_array(row, 0, 4).await?;
            matrix[row_index * 4..row_index * 4 + 4].copy_from_slice(&values[..4]);
        }
        put_affine_matrix(jvm, &mut this, matrix).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn set_values(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        m00: i32,
        m01: i32,
        m02: i32,
        m03: i32,
        m10: i32,
        m11: i32,
        m12: i32,
        m13: i32,
        m20: i32,
        m21: i32,
        m22: i32,
        m23: i32,
    ) -> Result<()> {
        put_affine_matrix(jvm, &mut this, [m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23]).await
    }

    async fn set_from_affine(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, source: ClassInstanceRef<Self>) -> Result<()> {
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.set source").await);
        }
        let matrix = get_affine_matrix(jvm, &source).await?;
        put_affine_matrix(jvm, &mut this, matrix).await
    }

    async fn set_identity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "rotationX", "I", 0).await?;
        jvm.put_field(&mut this, "rotationY", "I", 0).await?;
        jvm.put_field(&mut this, "rotationZ", "I", 0).await?;
        put_affine_matrix(jvm, &mut this, identity_matrix()).await
    }

    async fn rotation_x(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: i32) -> Result<()> {
        let (sin, cos) = sin_cos_mc(angle);
        jvm.put_field(&mut this, "rotationX", "I", angle).await?;
        put_affine_matrix(jvm, &mut this, [4096, 0, 0, 0, 0, cos, -sin, 0, 0, sin, cos, 0]).await
    }

    async fn rotation_y(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: i32) -> Result<()> {
        let (sin, cos) = sin_cos_mc(angle);
        jvm.put_field(&mut this, "rotationY", "I", angle).await?;
        put_affine_matrix(jvm, &mut this, [cos, 0, sin, 0, 0, 4096, 0, 0, -sin, 0, cos, 0]).await
    }

    async fn rotation_z(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: i32) -> Result<()> {
        let (sin, cos) = sin_cos_mc(angle);
        jvm.put_field(&mut this, "rotationZ", "I", angle).await?;
        put_affine_matrix(jvm, &mut this, [cos, -sin, 0, 0, sin, cos, 0, 0, 0, 0, 4096, 0]).await
    }

    async fn mul(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.AffineTrans::mul({this:?}, {other:?})");

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.mul").await);
        }

        let lhs = get_affine_matrix(jvm, &this).await?;
        let rhs = get_affine_matrix(jvm, &other).await?;
        put_affine_matrix(jvm, &mut this, mul_matrix(lhs, rhs)).await
    }

    async fn mul_two(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        lhs: ClassInstanceRef<Self>,
        rhs: ClassInstanceRef<Self>,
    ) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.AffineTrans::mul({this:?}, {lhs:?}, {rhs:?})");

        if lhs.is_null() || rhs.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.mul").await);
        }

        let lhs = get_affine_matrix(jvm, &lhs).await?;
        let rhs = get_affine_matrix(jvm, &rhs).await?;
        put_affine_matrix(jvm, &mut this, mul_matrix(lhs, rhs)).await
    }

    async fn get_to_array(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, values: ClassInstanceRef<Array<i32>>) -> Result<()> {
        Self::get_to_array_offset(jvm, context, this, values, 0).await
    }

    async fn get_to_array_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut values: ClassInstanceRef<Array<i32>>,
        offset: i32,
    ) -> Result<()> {
        if values.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.get array").await);
        }
        let offset = offset.max(0) as usize;
        let length = jvm.array_length(&values).await?;
        if length.saturating_sub(offset) < 12 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "AffineTrans.get array length").await);
        }
        jvm.store_array(&mut values, offset, get_affine_matrix(jvm, &this).await?).await
    }

    async fn transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        vector: ClassInstanceRef<Vector3D>,
    ) -> Result<ClassInstanceRef<Vector3D>> {
        if vector.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.transform vector").await);
        }

        let matrix = get_affine_matrix(jvm, &this).await?;
        let (x, y, z) = get_vector(jvm, &vector).await?;
        let result = (
            fixed_mul3(matrix[0], x, matrix[1], y, matrix[2], z) + matrix[3],
            fixed_mul3(matrix[4], x, matrix[5], y, matrix[6], z) + matrix[7],
            fixed_mul3(matrix[8], x, matrix[9], y, matrix[10], z) + matrix[11],
        );
        Ok(jvm.new_class(VECTOR_3D_CLASS, "(III)V", result).await?.into())
    }

    async fn set_rotation(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vector: ClassInstanceRef<Vector3D>,
        angle: i32,
    ) -> Result<()> {
        if vector.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.setRotation vector").await);
        }

        let (x, y, z) = get_vector(jvm, &vector).await?;
        let (sin, cos) = sin_cos_mc(angle);
        let xs = fixed_mul(x, sin);
        let ys = fixed_mul(y, sin);
        let zs = fixed_mul(z, sin);
        let nc = 4096 - cos;
        let xy = fixed_mul(fixed_mul(x, y), nc);
        let yz = fixed_mul(fixed_mul(y, z), nc);
        let zx = fixed_mul(fixed_mul(z, x), nc);
        let xx = fixed_mul(fixed_mul(x, x), nc);
        let yy = fixed_mul(fixed_mul(y, y), nc);
        let zz = fixed_mul(fixed_mul(z, z), nc);
        put_affine_matrix(
            jvm,
            &mut this,
            [
                cos + xx,
                xy - zs,
                zx + ys,
                0,
                zs + xy,
                cos + yy,
                yz - xs,
                0,
                zx - ys,
                xs + yz,
                cos + zz,
                0,
            ],
        )
        .await
    }

    async fn look_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        eye: ClassInstanceRef<Vector3D>,
        center: ClassInstanceRef<Vector3D>,
        up: ClassInstanceRef<Vector3D>,
    ) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.AffineTrans::lookAt({this:?}, {eye:?}, {center:?}, {up:?})");

        if eye.is_null() || center.is_null() || up.is_null() {
            return put_affine_matrix(jvm, &mut this, identity_matrix()).await;
        }

        let eye = get_vector(jvm, &eye).await?;
        let look = get_vector(jvm, &center).await?;
        let up = get_vector(jvm, &up).await?;
        let side = normalize3(cross3(look, up));
        let real_up = normalize3(cross3(look, side));
        let look = normalize3(look);
        let neg_eye = (-eye.0, -eye.1, -eye.2);
        put_affine_matrix(
            jvm,
            &mut this,
            [
                side.0,
                side.1,
                side.2,
                dot3(neg_eye, side),
                real_up.0,
                real_up.1,
                real_up.2,
                dot3(neg_eye, real_up),
                look.0,
                look.1,
                look.2,
                dot3(neg_eye, look),
            ],
        )
        .await
    }

    async fn rotate_vector(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut vector: ClassInstanceRef<Vector3D>) -> Result<()> {
        if vector.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.rotate vector").await);
        }
        let matrix = get_affine_matrix(jvm, &this).await?;
        let (x, y, z) = get_vector(jvm, &vector).await?;
        jvm.put_field(&mut vector, "x", "I", fixed_mul3(matrix[0], x, matrix[4], y, matrix[8], z))
            .await?;
        jvm.put_field(&mut vector, "y", "I", fixed_mul3(matrix[1], x, matrix[5], y, matrix[9], z))
            .await?;
        jvm.put_field(&mut vector, "z", "I", fixed_mul3(matrix[2], x, matrix[6], y, matrix[10], z))
            .await
    }

    async fn scale_vector(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, vector: ClassInstanceRef<Vector3D>) -> Result<()> {
        if vector.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.scale vector").await);
        }
        let (x, y, z) = get_vector(jvm, &vector).await?;
        let mut matrix = get_affine_matrix(jvm, &this).await?;
        matrix[0] = fixed_mul(matrix[0], x);
        matrix[1] = fixed_mul(matrix[1], y);
        matrix[2] = fixed_mul(matrix[2], z);
        matrix[4] = fixed_mul(matrix[4], x);
        matrix[5] = fixed_mul(matrix[5], y);
        matrix[6] = fixed_mul(matrix[6], z);
        matrix[8] = fixed_mul(matrix[8], x);
        matrix[9] = fixed_mul(matrix[9], y);
        matrix[10] = fixed_mul(matrix[10], z);
        put_affine_matrix(jvm, &mut this, matrix).await
    }
}
