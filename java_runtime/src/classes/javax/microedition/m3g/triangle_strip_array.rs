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

impl TriangleStripArray {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/TriangleStripArray",
            parent_class: Some("javax/microedition/m3g/IndexBuffer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I[I)V", Self::init_implicit, Default::default()),
                JavaMethodProto::new("<init>", "([I[I)V", Self::init_explicit, Default::default()),
                JavaMethodProto::new("getIndexCount", "()I", Self::get_index_count, Default::default()),
                JavaMethodProto::new("getIndices", "([I)V", Self::get_indices, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("indices", "[I", Default::default()),
                JavaFieldProto::new("stripLengths", "[I", Default::default()),
                JavaFieldProto::new("triangleCache", "[I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/IndexBuffer", "<init>", "()V", ()).await
    }

    pub(super) async fn init_implicit(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        first_index: i32,
        strip_lengths: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        if strip_lengths.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "TriangleStripArray stripLengths").await);
        }
        let lengths_len = jvm.array_length(&strip_lengths).await?;
        let lengths: Vec<i32> = jvm.load_array(&strip_lengths, 0, lengths_len).await?;
        let mut indices = jvm.instantiate_array("I", 1).await?;
        jvm.store_array(&mut indices, 0, vec![first_index]).await?;
        let mut lengths_array = jvm.instantiate_array("I", lengths.len()).await?;
        jvm.store_array(&mut lengths_array, 0, lengths).await?;
        jvm.put_field(&mut this, "indices", "[I", indices).await?;
        jvm.put_field(&mut this, "stripLengths", "[I", lengths_array).await
    }

    pub(super) async fn init_explicit(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        indices: ClassInstanceRef<Array<i32>>,
        strip_lengths: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        if indices.is_null() || strip_lengths.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "TriangleStripArray data").await);
        }
        let indices_len = jvm.array_length(&indices).await?;
        let lengths_len = jvm.array_length(&strip_lengths).await?;
        let indices: Vec<i32> = jvm.load_array(&indices, 0, indices_len).await?;
        let lengths: Vec<i32> = jvm.load_array(&strip_lengths, 0, lengths_len).await?;
        let mut indices_array = jvm.instantiate_array("I", indices.len()).await?;
        jvm.store_array(&mut indices_array, 0, indices).await?;
        let mut lengths_array = jvm.instantiate_array("I", lengths.len()).await?;
        jvm.store_array(&mut lengths_array, 0, lengths).await?;
        jvm.put_field(&mut this, "indices", "[I", indices_array).await?;
        jvm.put_field(&mut this, "stripLengths", "[I", lengths_array).await
    }

    pub(super) async fn get_index_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(triangle_strip_explicit_indices(jvm, &this).await?.len() as i32)
    }

    pub(super) async fn get_indices(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut out: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "TriangleStripArray.getIndices").await);
        }
        let indices = triangle_strip_explicit_indices(jvm, &this).await?;
        if jvm.array_length(&out).await? < indices.len() {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "indices array too small").await);
        }
        jvm.store_array(&mut out, 0, indices).await
    }
}

pub(crate) async fn triangle_strip_explicit_indices(jvm: &Jvm, this: &ClassInstanceRef<TriangleStripArray>) -> Result<Vec<i32>> {
    let indices_array: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "indices", "[I").await.unwrap_or_else(|_| null_ref());
    if indices_array.is_null() {
        return Ok(Vec::new());
    }
    let indices_len = jvm.array_length(&indices_array).await?;
    let indices: Vec<i32> = jvm.load_array(&indices_array, 0, indices_len).await?;
    if indices_len != 1 {
        return Ok(indices);
    }

    let lengths_array: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "stripLengths", "[I").await.unwrap_or_else(|_| null_ref());
    if lengths_array.is_null() {
        return Ok(indices);
    }
    let lengths: Vec<i32> = jvm.load_array(&lengths_array, 0, jvm.array_length(&lengths_array).await?).await?;
    let count = lengths.iter().copied().filter(|length| *length > 0).sum::<i32>().max(0) as usize;
    Ok((0..count).map(|offset| indices[0] + offset as i32).collect())
}
