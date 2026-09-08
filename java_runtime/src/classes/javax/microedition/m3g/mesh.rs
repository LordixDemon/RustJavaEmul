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

impl Mesh {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Mesh",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;)V",
                    Self::init_single,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/IndexBuffer;[Ljavax/microedition/m3g/Appearance;)V",
                    Self::init_array,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getAppearance",
                    "(I)Ljavax/microedition/m3g/Appearance;",
                    Self::get_appearance,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getIndexBuffer",
                    "(I)Ljavax/microedition/m3g/IndexBuffer;",
                    Self::get_index_buffer,
                    Default::default(),
                ),
                JavaMethodProto::new("getSubmeshCount", "()I", Self::get_submesh_count, Default::default()),
                JavaMethodProto::new(
                    "getVertexBuffer",
                    "()Ljavax/microedition/m3g/VertexBuffer;",
                    Self::get_vertex_buffer,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setAppearance",
                    "(ILjavax/microedition/m3g/Appearance;)V",
                    Self::set_appearance,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", Default::default()),
                JavaFieldProto::new("submeshCount", "I", Default::default()),
                JavaFieldProto::new("indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", Default::default()),
                JavaFieldProto::new("appearances", "[Ljavax/microedition/m3g/Appearance;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await
    }

    pub(super) async fn init_single(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        index_buffer: ClassInstanceRef<IndexBuffer>,
        appearance: ClassInstanceRef<Appearance>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        let mut index_buffers = jvm.instantiate_array("Ljavax/microedition/m3g/IndexBuffer;", 1).await?;
        jvm.store_array(&mut index_buffers, 0, vec![index_buffer]).await?;
        let mut appearances = jvm.instantiate_array("Ljavax/microedition/m3g/Appearance;", 1).await?;
        jvm.store_array(&mut appearances, 0, vec![appearance]).await?;
        jvm.put_field(&mut this, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", vertex_buffer)
            .await?;
        jvm.put_field(&mut this, "submeshCount", "I", 1).await?;
        jvm.put_field(&mut this, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", index_buffers)
            .await?;
        jvm.put_field(&mut this, "appearances", "[Ljavax/microedition/m3g/Appearance;", appearances)
            .await
    }

    pub(super) async fn init_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        index_buffers: ClassInstanceRef<Array<IndexBuffer>>,
        appearances: ClassInstanceRef<Array<Appearance>>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        let count = if index_buffers.is_null() {
            0
        } else {
            jvm.array_length(&index_buffers).await? as i32
        };
        jvm.put_field(&mut this, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", vertex_buffer)
            .await?;
        jvm.put_field(&mut this, "submeshCount", "I", count).await?;
        jvm.put_field(&mut this, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", index_buffers)
            .await?;
        jvm.put_field(&mut this, "appearances", "[Ljavax/microedition/m3g/Appearance;", appearances)
            .await
    }

    pub(super) async fn get_appearance(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
    ) -> Result<ClassInstanceRef<Appearance>> {
        let count: i32 = jvm.get_field(&this, "submeshCount", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "M3G appearance index").await);
        }
        let appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> =
            jvm.get_field(&this, "appearances", "[Ljavax/microedition/m3g/Appearance;").await?;
        Ok(jvm
            .load_array(&appearances, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or_else(null_ref))
    }

    pub(super) async fn get_index_buffer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
    ) -> Result<ClassInstanceRef<IndexBuffer>> {
        let count: i32 = jvm.get_field(&this, "submeshCount", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "M3G index buffer index").await);
        }
        let index_buffers: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> =
            jvm.get_field(&this, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;").await?;
        Ok(jvm
            .load_array(&index_buffers, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or_else(null_ref))
    }

    pub(super) async fn get_submesh_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "submeshCount", "I").await
    }

    pub(super) async fn get_vertex_buffer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<VertexBuffer>> {
        jvm.get_field(&this, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;").await
    }

    pub(super) async fn set_appearance(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        appearance: ClassInstanceRef<Appearance>,
    ) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "submeshCount", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "M3G appearance index").await);
        }
        let mut appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> =
            jvm.get_field(&this, "appearances", "[Ljavax/microedition/m3g/Appearance;").await?;
        jvm.store_array(&mut appearances, index as usize, vec![appearance]).await
    }
}
