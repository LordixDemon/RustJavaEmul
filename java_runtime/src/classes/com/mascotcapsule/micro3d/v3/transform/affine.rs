use super::super::{
    constants::{AFFINE_TRANS_CLASS, VECTOR_3D_CLASS},
    math::{cross3, dot3, fixed_mul, fixed_mul3, identity_matrix, mul_matrix, normalize3, sin_cos_mc},
    storage::{get_affine_matrix, get_vector, put_affine_matrix},
};
#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext};
#[allow(unused_imports)]
use alloc::{vec, vec::Vec};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

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

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.AffineTrans::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let _: () = jvm.invoke_virtual(&this, "setIdentity", "()V", ()).await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn init_values(
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

    pub(super) async fn init_copy(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<Self>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::set_from_affine(jvm, context, this, source).await
    }

    pub(super) async fn init_from_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::set_from_array(jvm, context, this, values).await
    }

    pub(super) async fn init_from_array_offset(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<i32>>,
        offset: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::set_from_array_offset(jvm, context, this, values, offset).await
    }

    pub(super) async fn init_from_array_2d(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<Array<i32>>>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::set_from_array_2d(jvm, context, this, values).await
    }

    pub(super) async fn set_from_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        Self::set_from_array_offset(jvm, context, this, values, 0).await
    }

    pub(super) async fn set_from_array_offset(
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

    pub(super) async fn set_from_array_2d(
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
    pub(super) async fn set_values(
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

    pub(super) async fn set_from_affine(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<Self>,
    ) -> Result<()> {
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.set source").await);
        }
        let matrix = get_affine_matrix(jvm, &source).await?;
        put_affine_matrix(jvm, &mut this, matrix).await
    }

    pub(super) async fn set_identity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "rotationX", "I", 0).await?;
        jvm.put_field(&mut this, "rotationY", "I", 0).await?;
        jvm.put_field(&mut this, "rotationZ", "I", 0).await?;
        put_affine_matrix(jvm, &mut this, identity_matrix()).await
    }

    pub(super) async fn rotation_x(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: i32) -> Result<()> {
        let (sin, cos) = sin_cos_mc(angle);
        jvm.put_field(&mut this, "rotationX", "I", angle).await?;
        put_affine_matrix(jvm, &mut this, [4096, 0, 0, 0, 0, cos, -sin, 0, 0, sin, cos, 0]).await
    }

    pub(super) async fn rotation_y(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: i32) -> Result<()> {
        let (sin, cos) = sin_cos_mc(angle);
        jvm.put_field(&mut this, "rotationY", "I", angle).await?;
        put_affine_matrix(jvm, &mut this, [cos, 0, sin, 0, 0, 4096, 0, 0, -sin, 0, cos, 0]).await
    }

    pub(super) async fn rotation_z(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: i32) -> Result<()> {
        let (sin, cos) = sin_cos_mc(angle);
        jvm.put_field(&mut this, "rotationZ", "I", angle).await?;
        put_affine_matrix(jvm, &mut this, [cos, -sin, 0, 0, sin, cos, 0, 0, 0, 0, 4096, 0]).await
    }

    pub(super) async fn mul(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.AffineTrans::mul({this:?}, {other:?})");

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AffineTrans.mul").await);
        }

        let lhs = get_affine_matrix(jvm, &this).await?;
        let rhs = get_affine_matrix(jvm, &other).await?;
        put_affine_matrix(jvm, &mut this, mul_matrix(lhs, rhs)).await
    }

    pub(super) async fn mul_two(
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

    pub(super) async fn get_to_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        values: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        Self::get_to_array_offset(jvm, context, this, values, 0).await
    }

    pub(super) async fn get_to_array_offset(
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

    pub(super) async fn transform(
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

    pub(super) async fn set_rotation(
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

    pub(super) async fn look_at(
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

    pub(super) async fn rotate_vector(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut vector: ClassInstanceRef<Vector3D>,
    ) -> Result<()> {
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

    pub(super) async fn scale_vector(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vector: ClassInstanceRef<Vector3D>,
    ) -> Result<()> {
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
