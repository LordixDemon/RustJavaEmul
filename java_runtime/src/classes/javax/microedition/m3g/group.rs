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

impl Group {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Group",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("addChild", "(Ljavax/microedition/m3g/Node;)V", Self::add_child, Default::default()),
                JavaMethodProto::new("getChild", "(I)Ljavax/microedition/m3g/Node;", Self::get_child, Default::default()),
                JavaMethodProto::new("getChildCount", "()I", Self::get_child_count, Default::default()),
                JavaMethodProto::new(
                    "pick",
                    "(IFFFFFFLjavax/microedition/m3g/RayIntersection;)Z",
                    Self::pick_3d,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "pick",
                    "(IFFLjavax/microedition/m3g/Camera;Ljavax/microedition/m3g/RayIntersection;)Z",
                    Self::pick_2d,
                    Default::default(),
                ),
                JavaMethodProto::new("removeChild", "(Ljavax/microedition/m3g/Node;)V", Self::remove_child, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("children", "[Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("childCount", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await?;
        let children = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", 0).await?;
        jvm.put_field(&mut this, "children", "[Ljavax/microedition/m3g/Node;", children).await?;
        jvm.put_field(&mut this, "childCount", "I", 0).await
    }

    pub(super) async fn add_child(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, child: ClassInstanceRef<Node>) -> Result<()> {
        let mut group: ClassInstanceRef<Group> = ClassInstanceRef::new(this.instance.clone());
        Self::push_child(jvm, &mut group, child).await
    }

    pub(super) async fn get_child(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Node>> {
        let count: i32 = jvm.get_field(&this, "childCount", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "M3G child index").await);
        }
        let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm.get_field(&this, "children", "[Ljavax/microedition/m3g/Node;").await?;
        Ok(jvm
            .load_array(&children, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or_else(null_ref))
    }

    pub(super) async fn get_child_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "childCount", "I").await
    }

    pub(super) async fn pick_3d(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mask: i32,
        ox: f32,
        oy: f32,
        oz: f32,
        dx: f32,
        dy: f32,
        dz: f32,
        ri: ClassInstanceRef<RayIntersection>,
    ) -> Result<bool> {
        let ray = [ox, oy, oz, dx, dy, dz];
        let Some(hit) = Self::pick_ray(jvm, cast_ref::<Group, Node>(&this), mask, [ox, oy, oz], [dx, dy, dz]).await? else {
            return Ok(false);
        };
        if !ri.is_null() {
            let mut ri = ri;
            RayIntersection::fill(jvm, &mut ri, hit.node.clone(), &hit, ray).await?;
        }
        Ok(true)
    }

    pub(super) async fn pick_2d(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mask: i32,
        x: f32,
        y: f32,
        camera: ClassInstanceRef<Camera>,
        ri: ClassInstanceRef<RayIntersection>,
    ) -> Result<bool> {
        if camera.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Group.pick camera").await);
        }
        let fovy = jvm.get_field::<f32>(&camera, "fovy", "F").await.unwrap_or(45.0);
        let aspect = jvm.get_field::<f32>(&camera, "aspect", "F").await.unwrap_or(1.0).max(0.001);
        let half_y = (fovy * core::f32::consts::PI / 360.0).tan();
        let direction = normalize3([x * half_y * aspect, y * half_y, 1.0]).unwrap_or([0.0, 0.0, 1.0]);
        let ray = [0.0, 0.0, 0.0, direction[0], direction[1], direction[2]];
        let Some(hit) = Self::pick_ray(jvm, cast_ref::<Group, Node>(&this), mask, [0.0, 0.0, 0.0], direction).await? else {
            return Ok(false);
        };
        if !ri.is_null() {
            let mut ri = ri;
            RayIntersection::fill(jvm, &mut ri, hit.node.clone(), &hit, ray).await?;
        }
        Ok(true)
    }

    pub(super) async fn remove_child(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        child: ClassInstanceRef<Node>,
    ) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "childCount", "I").await?;
        let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm.get_field(&this, "children", "[Ljavax/microedition/m3g/Node;").await?;
        let mut values: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, count.max(0) as usize).await?;
        let removed = values.iter().any(|candidate| same_instance(candidate, &child));
        values.retain(|candidate| !same_instance(candidate, &child));
        let mut new_array = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", values.len()).await?;
        jvm.store_array(&mut new_array, 0, values.clone()).await?;
        jvm.put_field(&mut this, "children", "[Ljavax/microedition/m3g/Node;", new_array).await?;
        jvm.put_field(&mut this, "childCount", "I", values.len() as i32).await?;
        if removed {
            let mut child = child;
            let user_id = jvm
                .get_field::<i32>(&cast_ref::<Node, Object3D>(&child), "userID", "I")
                .await
                .unwrap_or(0);
            tracing::debug!(
                target: "rustjava_m3g",
                "m3g.Group.removeChild group={} child={} childUserID={}",
                this.class_definition().name(),
                child.class_definition().name(),
                user_id
            );
            jvm.put_field(&mut child, "parent", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
                .await?;
        }
        Ok(())
    }

    pub(super) async fn push_child(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, mut child: ClassInstanceRef<Node>) -> Result<()> {
        if child.is_null() {
            return Ok(());
        }
        let count: i32 = jvm.get_field(this, "childCount", "I").await?;
        let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm.get_field(this, "children", "[Ljavax/microedition/m3g/Node;").await?;
        let mut values: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, count.max(0) as usize).await?;
        values.push(child.clone());
        let mut new_array = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", values.len()).await?;
        jvm.store_array(&mut new_array, 0, values).await?;
        jvm.put_field(this, "children", "[Ljavax/microedition/m3g/Node;", new_array).await?;
        jvm.put_field(this, "childCount", "I", count + 1).await?;
        jvm.put_field(&mut child, "parent", "Ljavax/microedition/m3g/Node;", cast_ref::<Group, Node>(this))
            .await
    }

    pub(super) async fn pick_ray(
        jvm: &Jvm,
        root: ClassInstanceRef<Node>,
        mask: i32,
        origin: [f32; 3],
        direction: [f32; 3],
    ) -> Result<Option<M3gPickHit>> {
        let Some(direction) = normalize3(direction) else {
            return Ok(None);
        };
        let mut best: Option<M3gPickHit> = None;
        let mut stack = vec![(root, identity_matrix(), false, true, 1.0f32)];
        while let Some((node, parent_matrix, apply_local_transform, parent_picking_enabled, parent_alpha)) = stack.pop() {
            if node.is_null() {
                continue;
            }
            let picking_enabled = jvm.get_field::<bool>(&node, "pickingEnabled", "Z").await.unwrap_or(true);
            let subtree_picking_enabled = parent_picking_enabled && picking_enabled;
            let world_matrix = if apply_local_transform {
                let local_matrix = Transformable::local_matrix(jvm, &cast_ref::<Node, Transformable>(&node))
                    .await
                    .unwrap_or_else(|_| identity_matrix());
                multiply_matrix(parent_matrix, local_matrix)
            } else {
                parent_matrix
            };

            let class_name = node.class_definition().name().to_string();
            if subtree_picking_enabled && is_mesh_class(&class_name) {
                let scope = jvm.get_field::<i32>(&node, "scope", "I").await.unwrap_or(-1);
                if (scope & mask) != 0 {
                    if let Some(hit) = Self::pick_mesh(jvm, cast_ref::<Node, Mesh>(&node), node.clone(), world_matrix, origin, direction).await? {
                        if best.as_ref().is_none_or(|best| hit.distance < best.distance) {
                            best = Some(hit);
                        }
                    }
                }
            }

            if subtree_picking_enabled && class_name == "javax/microedition/m3g/Sprite3D" {
                let scope = jvm.get_field::<i32>(&node, "scope", "I").await.unwrap_or(-1);
                if (scope & mask) != 0 {
                    if let Some(hit) = Self::pick_sprite(jvm, node.clone(), world_matrix, origin, direction).await? {
                        if best.as_ref().is_none_or(|best| hit.distance < best.distance) {
                            best = Some(hit);
                        }
                    }
                }
            }

            if subtree_picking_enabled && (class_name == "javax/microedition/m3g/Group" || class_name == "javax/microedition/m3g/World") {
                let group: ClassInstanceRef<Group> = cast_ref(&node);
                let child_count: i32 = jvm.get_field(&group, "childCount", "I").await.unwrap_or(0);
                if child_count > 0 {
                    let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> =
                        jvm.get_field(&group, "children", "[Ljavax/microedition/m3g/Node;").await?;
                    let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
                    for child in children.into_iter().rev() {
                        stack.push((child, world_matrix, true, subtree_picking_enabled, parent_alpha));
                    }
                }
            }

            if subtree_picking_enabled {
                push_skinned_skeleton(jvm, &node, &mut stack, world_matrix, subtree_picking_enabled, parent_alpha).await?;
            }
        }
        Ok(best)
    }

    pub(super) async fn pick_mesh(
        jvm: &Jvm,
        mesh: ClassInstanceRef<Mesh>,
        node: ClassInstanceRef<Node>,
        world_matrix: [f32; 16],
        origin: [f32; 3],
        direction: [f32; 3],
    ) -> Result<Option<M3gPickHit>> {
        let vertex_buffer: ClassInstanceRef<VertexBuffer> = jvm.get_field(&mesh, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;").await?;
        if vertex_buffer.is_null() {
            return Ok(None);
        }
        let positions = Graphics3D::deformed_positions(jvm, &mesh, &vertex_buffer).await?;
        if positions.is_empty() {
            return Ok(None);
        }
        let world_positions: Vec<[f32; 3]> = positions
            .iter()
            .copied()
            .map(|position| transform_point(world_matrix, position))
            .collect();

        let tex_coords_ref: ClassInstanceRef<VertexArray> = jvm
            .get_field(&vertex_buffer, "texCoords0", "Ljavax/microedition/m3g/VertexArray;")
            .await?;
        let tex_scale = jvm.get_field::<f32>(&vertex_buffer, "texScale", "F").await.unwrap_or(1.0);
        let tex_bias = Graphics3D::float_array_field(jvm, &vertex_buffer, "texBias", 3, 0.0).await?;
        let tex_coords = Graphics3D::vertex_array_vec2(jvm, &tex_coords_ref, tex_scale, &tex_bias).await?;
        let tex_coords_ref1: ClassInstanceRef<VertexArray> = jvm
            .get_field(&vertex_buffer, "texCoords1", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let tex_scale1 = jvm.get_field::<f32>(&vertex_buffer, "texScale1", "F").await.unwrap_or(1.0);
        let tex_bias1 = Graphics3D::float_array_field(jvm, &vertex_buffer, "texBias1", 3, 0.0).await?;
        let tex_coords1 = Graphics3D::vertex_array_vec2(jvm, &tex_coords_ref1, tex_scale1, &tex_bias1).await?;

        let submesh_count: i32 = jvm.get_field(&mesh, "submeshCount", "I").await.unwrap_or(0);
        if submesh_count <= 0 {
            return Ok(None);
        }
        let index_buffers: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> =
            jvm.get_field(&mesh, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;").await?;
        let index_buffers: Vec<ClassInstanceRef<IndexBuffer>> = jvm.load_array(&index_buffers, 0, submesh_count as usize).await?;
        let mut best: Option<M3gPickHit> = None;
        for (submesh, index_buffer) in index_buffers.into_iter().enumerate() {
            if index_buffer.is_null() {
                continue;
            }
            let triangles = Graphics3D::triangle_indices(jvm, cast_ref::<IndexBuffer, TriangleStripArray>(&index_buffer)).await?;
            for [i0, i1, i2] in triangles.iter().copied() {
                let (Some(p0), Some(p1), Some(p2)) = (
                    world_positions.get(i0).copied(),
                    world_positions.get(i1).copied(),
                    world_positions.get(i2).copied(),
                ) else {
                    continue;
                };
                let Some(triangle_hit) = ray_triangle_intersection(origin, direction, p0, p1, p2) else {
                    continue;
                };
                let uv0 = tex_coords.get(i0).copied().unwrap_or([0.0, 0.0]);
                let uv1 = tex_coords.get(i1).copied().unwrap_or(uv0);
                let uv2 = tex_coords.get(i2).copied().unwrap_or(uv0);
                let uv10 = tex_coords1.get(i0).copied().unwrap_or([0.0, 0.0]);
                let uv11 = tex_coords1.get(i1).copied().unwrap_or(uv10);
                let uv12 = tex_coords1.get(i2).copied().unwrap_or(uv10);
                let w = 1.0 - triangle_hit.u - triangle_hit.v;
                let texture_s = uv0[0] * w + uv1[0] * triangle_hit.u + uv2[0] * triangle_hit.v;
                let texture_t = uv0[1] * w + uv1[1] * triangle_hit.u + uv2[1] * triangle_hit.v;
                let hit = M3gPickHit {
                    node: node.clone(),
                    distance: triangle_hit.distance,
                    submesh_index: submesh as i32,
                    texture_s,
                    texture_t,
                    texture_s1: uv10[0] * w + uv11[0] * triangle_hit.u + uv12[0] * triangle_hit.v,
                    texture_t1: uv10[1] * w + uv11[1] * triangle_hit.u + uv12[1] * triangle_hit.v,
                    normal: triangle_hit.normal,
                };
                if best.as_ref().is_none_or(|best| hit.distance < best.distance) {
                    best = Some(hit);
                }
            }
        }
        Ok(best)
    }

    pub(super) async fn pick_sprite(
        _jvm: &Jvm,
        node: ClassInstanceRef<Node>,
        world_matrix: [f32; 16],
        origin: [f32; 3],
        direction: [f32; 3],
    ) -> Result<Option<M3gPickHit>> {
        let corners = [
            transform_point(world_matrix, [-0.5, -0.5, 0.0]),
            transform_point(world_matrix, [0.5, -0.5, 0.0]),
            transform_point(world_matrix, [0.5, 0.5, 0.0]),
            transform_point(world_matrix, [-0.5, 0.5, 0.0]),
        ];
        let uvs = [[0.0f32, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
        let mut best: Option<M3gPickHit> = None;
        for [i0, i1, i2] in [[0usize, 1, 2], [0, 2, 3]] {
            let Some(triangle_hit) = ray_triangle_intersection(origin, direction, corners[i0], corners[i1], corners[i2]) else {
                continue;
            };
            let w = 1.0 - triangle_hit.u - triangle_hit.v;
            let hit = M3gPickHit {
                node: node.clone(),
                distance: triangle_hit.distance,
                submesh_index: 0,
                texture_s: uvs[i0][0] * w + uvs[i1][0] * triangle_hit.u + uvs[i2][0] * triangle_hit.v,
                texture_t: uvs[i0][1] * w + uvs[i1][1] * triangle_hit.u + uvs[i2][1] * triangle_hit.v,
                texture_s1: 0.0,
                texture_t1: 0.0,
                normal: triangle_hit.normal,
            };
            if best.as_ref().is_none_or(|best| hit.distance < best.distance) {
                best = Some(hit);
            }
        }
        Ok(best)
    }
}
