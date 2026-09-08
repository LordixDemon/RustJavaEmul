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

impl IndexBuffer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/IndexBuffer",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getIndexCount", "()I", Self::get_index_count, Default::default()),
                JavaMethodProto::new("getIndices", "([I)V", Self::get_indices, Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await
    }

    pub(super) async fn get_index_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(super::triangle_strip_array::triangle_strip_explicit_indices(jvm, &cast_ref(&this))
            .await?
            .len() as i32)
    }

    pub(super) async fn get_indices(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut out: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "IndexBuffer.getIndices").await);
        }
        let indices = super::triangle_strip_array::triangle_strip_explicit_indices(jvm, &cast_ref(&this)).await?;
        if jvm.array_length(&out).await? < indices.len() {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "indices array too small").await);
        }
        jvm.store_array(&mut out, 0, indices).await
    }
}
