#[allow(unused_imports)]
use super::super::super::common::*;
#[allow(unused_imports)]
use super::super::super::math::*;
#[allow(unused_imports)]
use super::super::super::prelude::*;
#[allow(unused_imports)]
use super::super::super::raw_arrays::*;
#[allow(unused_imports)]
use super::super::super::render::*;
#[allow(unused_imports)]
use super::super::super::types::*;

impl super::super::super::Graphics3D {
    pub(crate) async fn render_mesh(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mesh: ClassInstanceRef<Mesh>,
        frame: u64,
        world_matrix: [f32; 16],
        view_matrix: [f32; 16],
        projection: [f32; 16],
        _fovy: f32,
        _aspect: f32,
        near: f32,
        far: f32,
        depth_enabled: bool,
        depth_range_near: f32,
        depth_range_far: f32,
        viewport_w: i32,
        viewport_h: i32,
        pixels: &mut [i32],
        depth: &mut [f32],
        scene_lighting: &M3gSceneLighting,
        mesh_scope: i32,
        alpha_factor: f32,
        mut gpu: Option<&mut super::super::super::gpu_scene::M3gGpuBuilder>,
    ) -> Result<M3gMeshRenderStats> {
        let mut stats = M3gMeshRenderStats::default();
        let vertex_start_ms = context.now();
        let user_id = jvm.get_field::<i32>(&cast_ref::<Mesh, Object3D>(&mesh), "userID", "I").await.unwrap_or(0);
        let vertex_buffer: ClassInstanceRef<VertexBuffer> = jvm.get_field(&mesh, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;").await?;
        if vertex_buffer.is_null() {
            tracing::debug!(target: "rustjava_m3g", "m3g.render.mesh frame={} userID={} skipped=noVertexBuffer", frame, user_id);
            return Ok(stats);
        }

        let (positions, normals) = Self::deformed_mesh_attributes(jvm, &mesh, &vertex_buffer).await?;
        stats.vertices = positions.len();
        if positions.is_empty() {
            tracing::debug!(target: "rustjava_m3g", "m3g.render.mesh frame={} userID={} skipped=noPositions", frame, user_id);
            return Ok(stats);
        }

        let tex_coords_ref: ClassInstanceRef<VertexArray> = jvm
            .get_field(&vertex_buffer, "texCoords0", "Ljavax/microedition/m3g/VertexArray;")
            .await?;
        let tex_scale = jvm.get_field::<f32>(&vertex_buffer, "texScale", "F").await.unwrap_or(1.0);
        let tex_bias = Self::float_array_field(jvm, &vertex_buffer, "texBias", 3, 0.0).await?;
        let tex_coords = Self::vertex_array_vec2(jvm, &tex_coords_ref, tex_scale, &tex_bias).await?;
        let tex_coords_ref1: ClassInstanceRef<VertexArray> = jvm
            .get_field(&vertex_buffer, "texCoords1", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let tex_scale1 = jvm.get_field::<f32>(&vertex_buffer, "texScale1", "F").await.unwrap_or(1.0);
        let tex_bias1 = Self::float_array_field(jvm, &vertex_buffer, "texBias1", 3, 0.0).await?;
        let tex_coords1 = Self::vertex_array_vec2(jvm, &tex_coords_ref1, tex_scale1, &tex_bias1).await?;

        let default_color = ensure_opaque(jvm.get_field(&vertex_buffer, "defaultColor", "I").await.unwrap_or(0x00ff_ffff));
        let colors_ref: ClassInstanceRef<VertexArray> = jvm.get_field(&vertex_buffer, "colors", "Ljavax/microedition/m3g/VertexArray;").await?;
        let colors = Self::vertex_array_colors(jvm, &colors_ref, default_color).await?;
        let has_vertex_colors = !colors.is_empty();
        let alpha_bits = ((alpha_factor.clamp(0.0, 1.0) * 255.0) as u32).min(255);

        let model_view = multiply_matrix(view_matrix, world_matrix);
        let mut camera_vertices = Vec::with_capacity(positions.len());
        for (index, position) in positions.iter().copied().enumerate() {
            let uv = tex_coords.get(index).copied().unwrap_or([0.0, 0.0]);
            let uv1 = tex_coords1.get(index).copied().unwrap_or([0.0, 0.0]);
            let mut color = colors.get(index).copied().unwrap_or(default_color);
            color = super::apply_alpha_factor(color, alpha_bits);
            camera_vertices.push(camera_vertex(position, uv, uv1, color, model_view));
        }
        stats.projected_vertices = camera_vertices
            .iter()
            .filter(|vertex| vertex.is_some_and(|vertex| vertex.depth > near))
            .count();
        let vertex_done_ms = context.now();
        stats.vertex_ms = vertex_done_ms.saturating_sub(vertex_start_ms);

        let submesh_count: i32 = jvm.get_field(&mesh, "submeshCount", "I").await.unwrap_or(0);
        if submesh_count <= 0 {
            tracing::debug!(
                target: "rustjava_m3g",
                "m3g.render.mesh frame={} userID={} vertices={} projected={} skipped=noSubmeshes",
                frame,
                user_id,
                stats.vertices,
                stats.projected_vertices
            );
            return Ok(stats);
        }
        stats.submeshes = submesh_count as usize;
        let index_buffers: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> =
            jvm.get_field(&mesh, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;").await?;
        let appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> =
            jvm.get_field(&mesh, "appearances", "[Ljavax/microedition/m3g/Appearance;").await?;
        let index_buffers: Vec<ClassInstanceRef<IndexBuffer>> = jvm.load_array(&index_buffers, 0, submesh_count as usize).await?;
        let appearances: Vec<ClassInstanceRef<Appearance>> = jvm.load_array(&appearances, 0, submesh_count as usize).await?;

        let mut draw_items = Vec::new();
        for (submesh, index_buffer) in index_buffers.into_iter().enumerate() {
            if index_buffer.is_null() {
                continue;
            }
            let appearance_state = if let Some(appearance) = appearances.get(submesh) {
                Self::appearance_state(jvm, appearance, default_color, scene_lighting).await?
            } else {
                default_appearance_state(default_color)
            };
            if appearance_state.texture.is_some() || appearance_state.texture1.is_some() {
                stats.textured_submeshes += 1;
            }
            draw_items.push(M3gSubmeshDraw {
                sort_key: appearance_sort_key(&appearance_state),
                sequence: submesh,
                index_buffer,
                appearance: appearance_state,
            });
        }
        draw_items.sort_by_key(|item| (item.sort_key, item.sequence));
        let appearance_done_ms = context.now();
        stats.appearance_ms = appearance_done_ms.saturating_sub(vertex_done_ms);

        for draw_item in draw_items {
            let triangles = Self::triangle_indices(jvm, cast_ref::<IndexBuffer, TriangleStripArray>(&draw_item.index_buffer)).await?;
            stats.candidate_triangles += triangles.len();
            for [i0, i1, i2] in triangles.iter().copied() {
                let Some(mut v0) = camera_vertices.get(i0).and_then(|v| *v) else {
                    continue;
                };
                let Some(mut v1) = camera_vertices.get(i1).and_then(|v| *v) else {
                    continue;
                };
                let Some(mut v2) = camera_vertices.get(i2).and_then(|v| *v) else {
                    continue;
                };
                let lit = |index: usize, vertex: &mut M3gCameraVertex| {
                    let camera_pos = [vertex.x, vertex.y, -vertex.depth];
                    let normal = normals
                        .get(index)
                        .copied()
                        .and_then(|normal| normalize3(transform_direction(model_view, normal)));
                    let source = if has_vertex_colors {
                        vertex.color
                    } else {
                        super::apply_alpha_factor(draw_item.appearance.base_color, alpha_bits)
                    };
                    vertex.color = shade_vertex(camera_pos, normal, source, &draw_item.appearance, scene_lighting, mesh_scope);
                };
                lit(i0, &mut v0);
                lit(i1, &mut v1);
                lit(i2, &mut v2);
                let clipped = clip_triangle_near(v0, v1, v2, near);
                if clipped.len() < 3 {
                    continue;
                }

                for triangle_index in 1..clipped.len() - 1 {
                    let Some(v0) = project_clip_vertex(clipped[0], projection, viewport_w, viewport_h) else {
                        continue;
                    };
                    let Some(v1) = project_clip_vertex(clipped[triangle_index], projection, viewport_w, viewport_h) else {
                        continue;
                    };
                    let Some(v2) = project_clip_vertex(clipped[triangle_index + 1], projection, viewport_w, viewport_h) else {
                        continue;
                    };
                    if let Some(gpu) = gpu.as_mut() {
                        if gpu.push_projected(v0, v1, v2, &draw_item.appearance) {
                            stats.rasterized_triangles += 1;
                            stats.pixels += 1;
                        }
                        continue;
                    }
                    let drawn_pixels = rasterize_triangle(
                        pixels,
                        depth,
                        viewport_w,
                        viewport_h,
                        v0,
                        v1,
                        v2,
                        &draw_item.appearance,
                        depth_enabled,
                        near,
                        far,
                        depth_range_near,
                        depth_range_far,
                        &mut stats.dirty,
                    );
                    if drawn_pixels > 0 {
                        stats.rasterized_triangles += 1;
                        stats.pixels += drawn_pixels;
                    }
                }
            }
        }
        stats.raster_ms = context.now().saturating_sub(appearance_done_ms);
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.render.mesh frame={} userID={} vertices={} projected={} submeshes={} texturedSubmeshes={} candidateTriangles={} rasterizedTriangles={} pixels={}",
            frame,
            user_id,
            stats.vertices,
            stats.projected_vertices,
            stats.submeshes,
            stats.textured_submeshes,
            stats.candidate_triangles,
            stats.rasterized_triangles,
            stats.pixels
        );
        Ok(stats)
    }

    pub(crate) async fn deformed_positions(
        jvm: &Jvm,
        mesh: &ClassInstanceRef<Mesh>,
        vertex_buffer: &ClassInstanceRef<VertexBuffer>,
    ) -> Result<Vec<[f32; 3]>> {
        Ok(Self::deformed_mesh_attributes(jvm, mesh, vertex_buffer).await?.0)
    }

    pub(crate) async fn deformed_mesh_attributes(
        jvm: &Jvm,
        mesh: &ClassInstanceRef<Mesh>,
        vertex_buffer: &ClassInstanceRef<VertexBuffer>,
    ) -> Result<(Vec<[f32; 3]>, Vec<[f32; 3]>)> {
        let positions_ref: ClassInstanceRef<VertexArray> = jvm.get_field(vertex_buffer, "positions", "Ljavax/microedition/m3g/VertexArray;").await?;
        let position_scale = jvm.get_field::<f32>(vertex_buffer, "positionScale", "F").await.unwrap_or(1.0);
        let position_bias = Self::float_array_field(jvm, vertex_buffer, "positionBias", 3, 0.0).await?;
        let base_positions = Self::vertex_array_vec3(jvm, &positions_ref, position_scale, &position_bias).await?;
        let normals_ref: ClassInstanceRef<VertexArray> = jvm
            .get_field(vertex_buffer, "normals", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let base_normals = Self::vertex_array_vec3(jvm, &normals_ref, 1.0, &[0.0, 0.0, 0.0]).await?;
        let mut positions = (*base_positions).clone();
        let mut normals = (*base_normals).clone();
        Self::apply_morph_targets(jvm, mesh, &base_positions, &base_normals, &mut positions, &mut normals).await?;
        Self::apply_skinning(jvm, mesh, &mut positions, &mut normals).await?;
        Ok((positions, normals))
    }

    pub(crate) async fn apply_morph_targets(
        jvm: &Jvm,
        mesh: &ClassInstanceRef<Mesh>,
        base_positions: &[[f32; 3]],
        base_normals: &[[f32; 3]],
        positions: &mut [[f32; 3]],
        normals: &mut [[f32; 3]],
    ) -> Result<()> {
        if mesh.class_definition().name() != "javax/microedition/m3g/MorphingMesh" {
            return Ok(());
        }
        let targets: ClassInstanceRef<Array<ClassInstanceRef<VertexBuffer>>> = jvm
            .get_field(mesh, "targets", "[Ljavax/microedition/m3g/VertexBuffer;")
            .await
            .unwrap_or_else(|_| null_ref());
        let weights: ClassInstanceRef<Array<f32>> = jvm.get_field(mesh, "weights", "[F").await.unwrap_or_else(|_| null_ref());
        if targets.is_null() || weights.is_null() {
            return Ok(());
        }
        let count = jvm.array_length(&targets).await?.min(jvm.array_length(&weights).await?);
        let weight_values: Vec<f32> = jvm.load_array(&weights, 0, count).await?;
        let target_buffers: Vec<ClassInstanceRef<VertexBuffer>> = jvm.load_array(&targets, 0, count).await?;
        for (target, weight) in target_buffers.into_iter().zip(weight_values) {
            if target.is_null() || weight == 0.0 {
                continue;
            }
            let positions_ref: ClassInstanceRef<VertexArray> = jvm.get_field(&target, "positions", "Ljavax/microedition/m3g/VertexArray;").await?;
            let scale = jvm.get_field::<f32>(&target, "positionScale", "F").await.unwrap_or(1.0);
            let bias = Self::float_array_field(jvm, &target, "positionBias", 3, 0.0).await?;
            let target_positions = Self::vertex_array_vec3(jvm, &positions_ref, scale, &bias).await?;
            for (index, position) in positions.iter_mut().enumerate() {
                if let (Some(base), Some(target_position)) = (base_positions.get(index), target_positions.get(index)) {
                    position[0] += weight * (target_position[0] - base[0]);
                    position[1] += weight * (target_position[1] - base[1]);
                    position[2] += weight * (target_position[2] - base[2]);
                }
            }
            let normals_ref: ClassInstanceRef<VertexArray> = jvm
                .get_field(&target, "normals", "Ljavax/microedition/m3g/VertexArray;")
                .await
                .unwrap_or_else(|_| null_ref());
            if !normals_ref.is_null() && !normals.is_empty() {
                let target_normals = Self::vertex_array_vec3(jvm, &normals_ref, 1.0, &[0.0, 0.0, 0.0]).await?;
                for (index, normal) in normals.iter_mut().enumerate() {
                    if let (Some(base), Some(target_normal)) = (base_normals.get(index), target_normals.get(index)) {
                        normal[0] += weight * (target_normal[0] - base[0]);
                        normal[1] += weight * (target_normal[1] - base[1]);
                        normal[2] += weight * (target_normal[2] - base[2]);
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) async fn apply_skinning(jvm: &Jvm, mesh: &ClassInstanceRef<Mesh>, positions: &mut [[f32; 3]], normals: &mut [[f32; 3]]) -> Result<()> {
        if mesh.class_definition().name() != "javax/microedition/m3g/SkinnedMesh" {
            return Ok(());
        }
        let skinned: ClassInstanceRef<SkinnedMesh> = cast_ref(mesh);
        let bones = SkinnedMesh::load_nodes(jvm, &skinned).await?;
        if bones.is_empty() {
            return Ok(());
        }
        let weights = SkinnedMesh::load_i32s(jvm, &skinned, "boneWeights").await?;
        let firsts = SkinnedMesh::load_i32s(jvm, &skinned, "boneFirst").await?;
        let counts = SkinnedMesh::load_i32s(jvm, &skinned, "boneCount").await?;
        let binds = SkinnedMesh::load_f32s(jvm, &skinned, "boneBind").await?;
        let mut influences: Vec<Vec<(f32, [f32; 16])>> = vec![Vec::new(); positions.len()];
        for (index, bone) in bones.iter().enumerate() {
            if bone.is_null() {
                continue;
            }
            let weight = weights.get(index).copied().unwrap_or(0) as f32;
            if weight <= 0.0 {
                continue;
            }
            let first = firsts.get(index).copied().unwrap_or(0).max(0) as usize;
            let count = counts.get(index).copied().unwrap_or(0).max(0) as usize;
            let bind_start = index * 16;
            let mut bind = identity_matrix();
            if bind_start + 16 <= binds.len() {
                bind.copy_from_slice(&binds[bind_start..bind_start + 16]);
            }
            let Some(bind_inverse) = invert_matrix(bind) else {
                continue;
            };
            let current = SkinnedMesh::bone_to_mesh_matrix(jvm, &skinned, bone).await?;
            let skin = multiply_matrix(current, bind_inverse);
            let last = (first + count).min(positions.len());
            for vertex in first..last {
                influences[vertex].push((weight, skin));
            }
        }
        let source_positions = positions.to_vec();
        let source_normals = normals.to_vec();
        for (vertex, slots) in influences.into_iter().enumerate() {
            if slots.is_empty() {
                continue;
            }
            let mut ranked = slots;
            ranked.sort_by(|left, right| right.0.partial_cmp(&left.0).unwrap_or(core::cmp::Ordering::Equal));
            ranked.truncate(M3G_MAX_TRANSFORMS_PER_VERTEX as usize);
            let weight_sum: f32 = ranked.iter().map(|(weight, _)| *weight).sum();
            if weight_sum <= f32::EPSILON {
                continue;
            }
            let mut position = [0.0; 3];
            let mut normal = [0.0; 3];
            for (weight, matrix) in ranked {
                let scale = weight / weight_sum;
                let transformed = transform_point(matrix, source_positions[vertex]);
                position[0] += transformed[0] * scale;
                position[1] += transformed[1] * scale;
                position[2] += transformed[2] * scale;
                if let Some(source_normal) = source_normals.get(vertex) {
                    let transformed_normal = transform_direction(matrix, *source_normal);
                    normal[0] += transformed_normal[0] * scale;
                    normal[1] += transformed_normal[1] * scale;
                    normal[2] += transformed_normal[2] * scale;
                }
            }
            positions[vertex] = position;
            if vertex < normals.len() {
                normals[vertex] = normalize3(normal).unwrap_or(normal);
            }
        }
        Ok(())
    }
}
