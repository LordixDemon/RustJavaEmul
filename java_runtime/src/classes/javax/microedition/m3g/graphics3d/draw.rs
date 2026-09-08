#[allow(unused_imports)]
use super::super::common::*;
#[allow(unused_imports)]
use super::super::math::*;
#[allow(unused_imports)]
use super::super::prelude::*;
#[allow(unused_imports)]
use super::super::raw_arrays::*;
#[allow(unused_imports)]
use super::super::render::*;
#[allow(unused_imports)]
use super::super::types::*;

impl super::super::Graphics3D {
    pub(crate) async fn render_world(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        world: ClassInstanceRef<World>,
    ) -> Result<()> {
        tracing::trace!(target: "rustjava_m3g", "m3g.Graphics3D.render(World)");
        if world.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Graphics3D.render(World)").await);
        }
        let _: () = jvm
            .invoke_virtual(
                &cast_ref::<World, Node>(&world),
                "align",
                "(Ljavax/microedition/m3g/Node;)V",
                (null_ref::<Node>(),),
            )
            .await?;
        let background = Self::background_frame(jvm, &this, &world).await?;
        let camera: ClassInstanceRef<Camera> = jvm
            .get_field(&world, "activeCamera", "Ljavax/microedition/m3g/Camera;")
            .await
            .unwrap_or_else(|_| null_ref());
        Self::render_scene(
            jvm,
            context,
            &this,
            cast_ref::<World, Node>(&world),
            identity_matrix(),
            camera,
            None,
            Some(background),
            true,
        )
        .await
    }
    pub(crate) async fn render_node(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        node: ClassInstanceRef<Node>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        tracing::trace!(target: "rustjava_m3g", "m3g.Graphics3D.render(Node) node={node:?}");
        let root_matrix = if transform.is_null() {
            identity_matrix()
        } else {
            Transform::matrix(jvm, &transform).await.unwrap_or_else(|_| identity_matrix())
        };
        let camera: ClassInstanceRef<Camera> = jvm
            .get_field(&this, "camera", "Ljavax/microedition/m3g/Camera;")
            .await
            .unwrap_or_else(|_| null_ref());
        let camera_transform = if jvm.get_field(&this, "cameraTransformSet", "Z").await.unwrap_or(false) {
            Some(Self::camera_transform(jvm, &this).await.unwrap_or_else(|_| identity_matrix()))
        } else {
            None
        };
        Self::render_scene(jvm, context, &this, node, root_matrix, camera, camera_transform, None, false).await
    }
    pub(crate) async fn render_vb(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        index_buffer: ClassInstanceRef<IndexBuffer>,
        appearance: ClassInstanceRef<Appearance>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        Self::render_vb_with_scope(jvm, context, this, vertex_buffer, index_buffer, appearance, transform, -1).await
    }
    pub(crate) async fn render_vb_scoped(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        index_buffer: ClassInstanceRef<IndexBuffer>,
        appearance: ClassInstanceRef<Appearance>,
        transform: ClassInstanceRef<Transform>,
        scope: i32,
    ) -> Result<()> {
        Self::render_vb_with_scope(jvm, context, this, vertex_buffer, index_buffer, appearance, transform, scope).await
    }
    pub(crate) async fn render_vb_with_scope(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        index_buffer: ClassInstanceRef<IndexBuffer>,
        appearance: ClassInstanceRef<Appearance>,
        transform: ClassInstanceRef<Transform>,
        scope: i32,
    ) -> Result<()> {
        let mesh = jvm
            .new_class(
                "javax/microedition/m3g/Mesh",
                "(Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;)V",
                (vertex_buffer, index_buffer, appearance),
            )
            .await?;
        let _: () = jvm.invoke_virtual(&mesh, "setScope", "(I)V", (scope,)).await?;
        Self::render_node(jvm, context, this, mesh.into(), transform).await
    }
}
