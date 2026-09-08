use super::super::{
    constants::VECTOR_3D_CLASS,
    math::{cross3, normalize3},
    storage::get_vector,
};
#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext};
#[allow(unused_imports)]
use alloc::{vec, vec::Vec};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

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

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32, z: i32) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Vector3D::<init>({this:?}, {x:?}, {y:?}, {z:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::put(jvm, &mut this, x, y, z).await
    }

    pub(super) async fn init_default(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        Self::init(jvm, context, this, 0, 0, 0).await
    }

    pub(super) async fn init_copy(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, vector: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if vector.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D copy source").await);
        }
        let (x, y, z) = get_vector(jvm, &vector).await?;
        Self::put(jvm, &mut this, x, y, z).await
    }

    pub(super) async fn get_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "x", "I").await
    }

    pub(super) async fn get_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "y", "I").await
    }

    pub(super) async fn get_z(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "z", "I").await
    }

    pub(super) async fn set(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32, z: i32) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Vector3D::set({this:?}, {x:?}, {y:?}, {z:?})");

        Self::put(jvm, &mut this, x, y, z).await
    }

    pub(super) async fn set_from_vector(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vector: ClassInstanceRef<Self>,
    ) -> Result<()> {
        if vector.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D.set source").await);
        }
        let (x, y, z) = get_vector(jvm, &vector).await?;
        Self::put(jvm, &mut this, x, y, z).await
    }

    pub(super) async fn set_x(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Vector3D::setX({this:?}, {x:?})");

        jvm.put_field(&mut this, "x", "I", x).await
    }

    pub(super) async fn set_y(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, y: i32) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Vector3D::setY({this:?}, {y:?})");

        jvm.put_field(&mut this, "y", "I", y).await
    }

    pub(super) async fn set_z(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, z: i32) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Vector3D::setZ({this:?}, {z:?})");

        jvm.put_field(&mut this, "z", "I", z).await
    }

    pub(super) async fn unit(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let vector = normalize3(get_vector(jvm, &this).await?);
        Self::put(jvm, &mut this, vector.0, vector.1, vector.2).await
    }

    pub(super) async fn inner_product(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        Self::inner_product_refs(jvm, &this, &other).await
    }

    pub(super) async fn inner_product_static(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        lhs: ClassInstanceRef<Self>,
        rhs: ClassInstanceRef<Self>,
    ) -> Result<i32> {
        Self::inner_product_refs(jvm, &lhs, &rhs).await
    }

    pub(super) async fn inner_product_refs(jvm: &Jvm, lhs: &ClassInstanceRef<Self>, rhs: &ClassInstanceRef<Self>) -> Result<i32> {
        if lhs.is_null() || rhs.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D.innerProduct").await);
        }
        Ok(raw_dot3(get_vector(jvm, lhs).await?, get_vector(jvm, rhs).await?))
    }

    pub(super) async fn outer_product(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<()> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Vector3D.outerProduct").await);
        }
        let result = cross3(get_vector(jvm, &this).await?, get_vector(jvm, &other).await?);
        Self::put(jvm, &mut this, result.0, result.1, result.2).await
    }

    pub(super) async fn outer_product_static(
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

    pub(super) async fn outer_product_static_into(
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

    pub(super) async fn put(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, x: i32, y: i32, z: i32) -> Result<()> {
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
