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

impl MorphingMesh {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/MorphingMesh",
            parent_class: Some("javax/microedition/m3g/Mesh"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;)V",
                    Self::init_single,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/IndexBuffer;[Ljavax/microedition/m3g/Appearance;)V",
                    Self::init_array,
                    Default::default(),
                ),
                JavaMethodProto::new("getMorphTargetCount", "()I", Self::get_target_count, Default::default()),
                JavaMethodProto::new(
                    "getMorphTarget",
                    "(I)Ljavax/microedition/m3g/VertexBuffer;",
                    Self::get_target,
                    Default::default(),
                ),
                JavaMethodProto::new("getWeights", "([F)V", Self::get_weights, Default::default()),
                JavaMethodProto::new("setWeights", "([F)V", Self::set_weights, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("targets", "[Ljavax/microedition/m3g/VertexBuffer;", Default::default()),
                JavaFieldProto::new("weights", "[F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/Mesh", "<init>", "()V", ()).await
    }

    pub(super) async fn init_single(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        targets: ClassInstanceRef<Array<ClassInstanceRef<VertexBuffer>>>,
        index_buffer: ClassInstanceRef<IndexBuffer>,
        appearance: ClassInstanceRef<Appearance>,
    ) -> Result<()> {
        let mesh: ClassInstanceRef<Mesh> = cast_ref(&this);
        Mesh::init_single(jvm, context, mesh, vertex_buffer, index_buffer, appearance).await?;
        Self::store_targets(jvm, &mut this, targets).await
    }

    pub(super) async fn init_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        targets: ClassInstanceRef<Array<ClassInstanceRef<VertexBuffer>>>,
        index_buffers: ClassInstanceRef<Array<IndexBuffer>>,
        appearances: ClassInstanceRef<Array<Appearance>>,
    ) -> Result<()> {
        let mesh: ClassInstanceRef<Mesh> = cast_ref(&this);
        Mesh::init_array(jvm, context, mesh, vertex_buffer, index_buffers, appearances).await?;
        Self::store_targets(jvm, &mut this, targets).await
    }

    pub(super) async fn store_targets(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        targets: ClassInstanceRef<Array<ClassInstanceRef<VertexBuffer>>>,
    ) -> Result<()> {
        if targets.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "MorphingMesh targets").await);
        }
        let count = jvm.array_length(&targets).await?;
        for index in 0..count {
            let target: ClassInstanceRef<VertexBuffer> = jvm.load_array(&targets, index, 1).await?.into_iter().next().unwrap_or_else(null_ref);
            if target.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "MorphingMesh target").await);
            }
        }
        jvm.put_field(this, "targets", "[Ljavax/microedition/m3g/VertexBuffer;", targets.clone())
            .await?;
        let weights = jvm.instantiate_array("F", count).await?;
        jvm.put_field(this, "weights", "[F", weights).await
    }

    pub(super) async fn get_target_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let targets: ClassInstanceRef<Array<ClassInstanceRef<VertexBuffer>>> =
            jvm.get_field(&this, "targets", "[Ljavax/microedition/m3g/VertexBuffer;").await?;
        if targets.is_null() {
            return Ok(0);
        }
        Ok(jvm.array_length(&targets).await? as i32)
    }

    pub(super) async fn get_target(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
    ) -> Result<ClassInstanceRef<VertexBuffer>> {
        let targets: ClassInstanceRef<Array<ClassInstanceRef<VertexBuffer>>> =
            jvm.get_field(&this, "targets", "[Ljavax/microedition/m3g/VertexBuffer;").await?;
        if targets.is_null() || index < 0 {
            return Ok(null_ref());
        }
        let count = jvm.array_length(&targets).await? as i32;
        if index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "morph target").await);
        }
        Ok(jvm
            .load_array(&targets, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or_else(null_ref))
    }

    pub(super) async fn get_weights(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut out: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "MorphingMesh.getWeights").await);
        }
        let weights: ClassInstanceRef<Array<f32>> = jvm.get_field(&this, "weights", "[F").await?;
        if weights.is_null() {
            return Ok(());
        }
        let count = jvm.array_length(&weights).await?;
        if jvm.array_length(&out).await? < count {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "weights array too small").await);
        }
        let values: Vec<f32> = jvm.load_array(&weights, 0, count).await?;
        jvm.store_array(&mut out, 0, values).await
    }

    pub(super) async fn set_weights(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        weights: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if weights.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "MorphingMesh.setWeights").await);
        }
        let targets: ClassInstanceRef<Array<ClassInstanceRef<VertexBuffer>>> =
            jvm.get_field(&this, "targets", "[Ljavax/microedition/m3g/VertexBuffer;").await?;
        let expected = if targets.is_null() { 0 } else { jvm.array_length(&targets).await? };
        if jvm.array_length(&weights).await? < expected {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "MorphingMesh weights length").await);
        }
        jvm.put_field(&mut this, "weights", "[F", weights).await
    }
}
