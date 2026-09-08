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

impl SkinnedMesh {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/SkinnedMesh",
            parent_class: Some("javax/microedition/m3g/Mesh"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;Ljavax/microedition/m3g/Group;)V",
                    Self::init_single,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/IndexBuffer;[Ljavax/microedition/m3g/Appearance;Ljavax/microedition/m3g/Group;)V",
                    Self::init_array,
                    Default::default(),
                ),
                JavaMethodProto::new("getSkeleton", "()Ljavax/microedition/m3g/Group;", Self::get_skeleton, Default::default()),
                JavaMethodProto::new(
                    "addTransform",
                    "(Ljavax/microedition/m3g/Node;III)V",
                    Self::add_transform,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getBoneTransform",
                    "(Ljavax/microedition/m3g/Node;Ljavax/microedition/m3g/Transform;)V",
                    Self::get_bone_transform,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getBoneVertices",
                    "(Ljavax/microedition/m3g/Node;[I[F)I",
                    Self::get_bone_vertices,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("skeleton", "Ljavax/microedition/m3g/Group;", Default::default()),
                JavaFieldProto::new("boneNodes", "[Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("boneWeights", "[I", Default::default()),
                JavaFieldProto::new("boneFirst", "[I", Default::default()),
                JavaFieldProto::new("boneCount", "[I", Default::default()),
                JavaFieldProto::new("boneBind", "[F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Mesh", "<init>", "()V", ()).await?;
        Self::clear_bone_storage(jvm, &mut this).await
    }

    pub(super) async fn init_single(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        index_buffer: ClassInstanceRef<IndexBuffer>,
        appearance: ClassInstanceRef<Appearance>,
        skeleton: ClassInstanceRef<Group>,
    ) -> Result<()> {
        let mesh: ClassInstanceRef<Mesh> = cast_ref(&this);
        Mesh::init_single(jvm, context, mesh, vertex_buffer, index_buffer, appearance).await?;
        Self::attach_skeleton(jvm, &mut this, skeleton).await
    }

    pub(super) async fn init_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        index_buffers: ClassInstanceRef<Array<IndexBuffer>>,
        appearances: ClassInstanceRef<Array<Appearance>>,
        skeleton: ClassInstanceRef<Group>,
    ) -> Result<()> {
        let mesh: ClassInstanceRef<Mesh> = cast_ref(&this);
        Mesh::init_array(jvm, context, mesh, vertex_buffer, index_buffers, appearances).await?;
        Self::attach_skeleton(jvm, &mut this, skeleton).await
    }

    pub(super) async fn attach_skeleton(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, skeleton: ClassInstanceRef<Group>) -> Result<()> {
        if skeleton.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "SkinnedMesh skeleton").await);
        }
        let mut skeleton_node: ClassInstanceRef<Node> = cast_ref(&skeleton);
        jvm.put_field(
            &mut skeleton_node,
            "parent",
            "Ljavax/microedition/m3g/Node;",
            cast_ref::<SkinnedMesh, Node>(this),
        )
        .await?;
        jvm.put_field(this, "skeleton", "Ljavax/microedition/m3g/Group;", skeleton).await?;
        Self::clear_bone_storage(jvm, this).await
    }

    pub(super) async fn clear_bone_storage(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let bones = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", 0).await?;
        jvm.put_field(this, "boneNodes", "[Ljavax/microedition/m3g/Node;", bones).await?;
        jvm.put_field(this, "boneWeights", "[I", jvm.instantiate_array("I", 0).await?).await?;
        jvm.put_field(this, "boneFirst", "[I", jvm.instantiate_array("I", 0).await?).await?;
        jvm.put_field(this, "boneCount", "[I", jvm.instantiate_array("I", 0).await?).await?;
        jvm.put_field(this, "boneBind", "[F", jvm.instantiate_array("F", 0).await?).await
    }

    pub(super) async fn get_skeleton(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Group>> {
        jvm.get_field(&this, "skeleton", "Ljavax/microedition/m3g/Group;").await
    }

    pub(super) async fn add_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        bone: ClassInstanceRef<Node>,
        weight: i32,
        first: i32,
        count: i32,
    ) -> Result<()> {
        if bone.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "SkinnedMesh.addTransform").await);
        }
        if weight <= 0 || first < 0 || count <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "SkinnedMesh.addTransform").await);
        }

        let mut bones = Self::load_nodes(jvm, &this).await?;
        let mut weights = Self::load_i32s(jvm, &this, "boneWeights").await?;
        let mut firsts = Self::load_i32s(jvm, &this, "boneFirst").await?;
        let mut counts = Self::load_i32s(jvm, &this, "boneCount").await?;
        let mut binds = Self::load_f32s(jvm, &this, "boneBind").await?;

        bones.push(bone.clone());
        weights.push(weight);
        firsts.push(first);
        counts.push(count);
        let bind = Self::bone_to_mesh_matrix(jvm, &this, &bone).await?;
        binds.extend_from_slice(&bind);

        Self::store_nodes(jvm, &mut this, bones).await?;
        Self::store_i32s(jvm, &mut this, "boneWeights", weights).await?;
        Self::store_i32s(jvm, &mut this, "boneFirst", firsts).await?;
        Self::store_i32s(jvm, &mut this, "boneCount", counts).await?;
        Self::store_f32s(jvm, &mut this, "boneBind", binds).await
    }

    pub(super) async fn get_bone_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        bone: ClassInstanceRef<Node>,
        mut transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        if bone.is_null() || transform.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "SkinnedMesh.getBoneTransform").await);
        }
        let Some(index) = Self::bone_index(jvm, &this, &bone).await? else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "unknown SkinnedMesh bone").await);
        };
        let binds = Self::load_f32s(jvm, &this, "boneBind").await?;
        let start = index * 16;
        let mut matrix = identity_matrix();
        if start + 16 <= binds.len() {
            matrix.copy_from_slice(&binds[start..start + 16]);
        }
        Transform::put_matrix(jvm, &mut transform, matrix).await
    }

    pub(super) async fn get_bone_vertices(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        bone: ClassInstanceRef<Node>,
        mut indices: ClassInstanceRef<Array<i32>>,
        mut weights_out: ClassInstanceRef<Array<f32>>,
    ) -> Result<i32> {
        if bone.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "SkinnedMesh.getBoneVertices").await);
        }
        let bones = Self::load_nodes(jvm, &this).await?;
        let weights = Self::load_i32s(jvm, &this, "boneWeights").await?;
        let firsts = Self::load_i32s(jvm, &this, "boneFirst").await?;
        let counts = Self::load_i32s(jvm, &this, "boneCount").await?;
        let mut by_vertex: Vec<(i32, f32)> = Vec::new();
        for (index, candidate) in bones.iter().enumerate() {
            if !same_instance(candidate, &bone) {
                continue;
            }
            let first = firsts.get(index).copied().unwrap_or(0);
            let count = counts.get(index).copied().unwrap_or(0).max(0);
            let weight = weights.get(index).copied().unwrap_or(0) as f32;
            for vertex in first..first + count {
                if let Some(existing) = by_vertex.iter_mut().find(|(candidate, _)| *candidate == vertex) {
                    existing.1 += weight;
                } else {
                    by_vertex.push((vertex, weight));
                }
            }
        }
        let count = by_vertex.len() as i32;
        if !indices.is_null() {
            if jvm.array_length(&indices).await? < by_vertex.len() {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "bone vertex indices").await);
            }
            jvm.store_array(&mut indices, 0, by_vertex.iter().map(|(vertex, _)| *vertex).collect::<Vec<_>>())
                .await?;
        }
        if !weights_out.is_null() {
            if jvm.array_length(&weights_out).await? < by_vertex.len() {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "bone vertex weights").await);
            }
            jvm.store_array(&mut weights_out, 0, by_vertex.iter().map(|(_, weight)| *weight).collect::<Vec<_>>())
                .await?;
        }
        Ok(count)
    }

    pub(super) async fn bone_index(jvm: &Jvm, this: &ClassInstanceRef<Self>, bone: &ClassInstanceRef<Node>) -> Result<Option<usize>> {
        let bones = Self::load_nodes(jvm, this).await?;
        Ok(bones.iter().position(|candidate| same_instance(candidate, bone)))
    }

    pub(super) async fn bone_to_mesh_matrix(jvm: &Jvm, mesh: &ClassInstanceRef<Self>, bone: &ClassInstanceRef<Node>) -> Result<[f32; 16]> {
        let mesh_node: ClassInstanceRef<Node> = cast_ref(mesh);
        let (source_root, source_world) = Node::root_and_world_matrix(jvm, bone).await?;
        let (target_root, target_world) = Node::root_and_world_matrix(jvm, &mesh_node).await?;
        if same_instance(&source_root, &target_root)
            && let Some(target_inverse) = invert_matrix(target_world)
        {
            return Ok(multiply_matrix(target_inverse, source_world));
        }
        Transformable::local_matrix(jvm, &cast_ref(bone)).await
    }

    pub(super) async fn load_nodes(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<ClassInstanceRef<Node>>> {
        let array: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm
            .get_field(this, "boneNodes", "[Ljavax/microedition/m3g/Node;")
            .await
            .unwrap_or_else(|_| null_ref());
        if array.is_null() {
            return Ok(Vec::new());
        }
        let count = jvm.array_length(&array).await?;
        jvm.load_array(&array, 0, count).await
    }

    pub(super) async fn store_nodes(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, values: Vec<ClassInstanceRef<Node>>) -> Result<()> {
        let mut array = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", values.len()).await?;
        jvm.store_array(&mut array, 0, values).await?;
        jvm.put_field(this, "boneNodes", "[Ljavax/microedition/m3g/Node;", array).await
    }

    pub(super) async fn load_i32s(jvm: &Jvm, this: &ClassInstanceRef<Self>, name: &str) -> Result<Vec<i32>> {
        let array: ClassInstanceRef<Array<i32>> = jvm.get_field(this, name, "[I").await.unwrap_or_else(|_| null_ref());
        if array.is_null() {
            return Ok(Vec::new());
        }
        let count = jvm.array_length(&array).await?;
        jvm.load_array(&array, 0, count).await
    }

    pub(super) async fn store_i32s(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, name: &str, values: Vec<i32>) -> Result<()> {
        let mut array = jvm.instantiate_array("I", values.len()).await?;
        jvm.store_array(&mut array, 0, values).await?;
        jvm.put_field(this, name, "[I", array).await
    }

    pub(super) async fn load_f32s(jvm: &Jvm, this: &ClassInstanceRef<Self>, name: &str) -> Result<Vec<f32>> {
        let array: ClassInstanceRef<Array<f32>> = jvm.get_field(this, name, "[F").await.unwrap_or_else(|_| null_ref());
        if array.is_null() {
            return Ok(Vec::new());
        }
        let count = jvm.array_length(&array).await?;
        jvm.load_array(&array, 0, count).await
    }

    pub(super) async fn store_f32s(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, name: &str, values: Vec<f32>) -> Result<()> {
        let mut array = jvm.instantiate_array("F", values.len()).await?;
        jvm.store_array(&mut array, 0, values).await?;
        jvm.put_field(this, name, "[F", array).await
    }
}
