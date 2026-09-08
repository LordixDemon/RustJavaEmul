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

mod lighting;
mod mesh;
mod sprite;

impl super::super::Graphics3D {
    pub(crate) async fn render_scene(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        root: ClassInstanceRef<Node>,
        root_matrix: [f32; 16],
        camera: ClassInstanceRef<Camera>,
        camera_transform: Option<[f32; 16]>,
        background: Option<M3gBackgroundFrame>,
        use_world_lights: bool,
    ) -> Result<()> {
        if root.is_null() {
            return Ok(());
        }

        let target: ClassInstanceRef<Object> = jvm.get_field(this, "target", "Ljava/lang/Object;").await?;
        if target.is_null() {
            return Ok(());
        }

        let viewport_x: i32 = jvm.get_field(this, "viewportX", "I").await?;
        let viewport_y: i32 = jvm.get_field(this, "viewportY", "I").await?;
        let viewport_w: i32 = jvm.get_field(this, "viewportW", "I").await?;
        let viewport_h: i32 = jvm.get_field(this, "viewportH", "I").await?;
        if viewport_w <= 0 || viewport_h <= 0 {
            return Ok(());
        }
        let render_start_ms = context.now();

        let full_frame_changed = background.as_ref().is_some_and(|background| background.color_clear);
        let pixel_count = (viewport_w * viewport_h) as usize;
        let (stored_pixels, _stored_pixels_valid) = Self::load_color_buffer(jvm, this, pixel_count).await?;
        let (mut pixels, process_alpha, submit_when_empty, depth_clear) = match background {
            Some(background) if background.color_clear => (
                background.pixels,
                background.process_alpha,
                background.submit_when_empty,
                background.depth_clear,
            ),
            Some(background) => (stored_pixels, true, background.submit_when_empty, background.depth_clear),
            None => (stored_pixels, true, false, false),
        };
        if pixels.len() != pixel_count {
            pixels.resize(pixel_count, 0);
        }
        let mut depth = Self::load_depth_buffer(jvm, this, pixel_count, depth_clear).await?;
        let graphics: ClassInstanceRef<Graphics> = cast_ref(&target);
        let view = if let Some(transform) = camera_transform {
            invert_affine_matrix(transform).unwrap_or_else(identity_matrix)
        } else {
            Self::camera_view_matrix(jvm, &camera).await.unwrap_or_else(|_| identity_matrix())
        };
        let depth_enabled: bool = jvm.get_field(this, "depthEnabled", "Z").await.unwrap_or(true);
        let depth_range_near = jvm.get_field::<f32>(this, "depthRangeNear", "F").await.unwrap_or(0.0).clamp(0.0, 1.0);
        let depth_range_far = jvm.get_field::<f32>(this, "depthRangeFar", "F").await.unwrap_or(1.0).clamp(0.0, 1.0);
        let (fovy, aspect, near, far, projection) = if camera.is_null() {
            let fovy = 45.0;
            let aspect = viewport_w as f32 / viewport_h as f32;
            let near = 1.0;
            let far = 1000.0;
            (fovy, aspect, near, far, perspective_projection_matrix(fovy, aspect, near, far))
        } else {
            let fovy = jvm.get_field::<f32>(&camera, "fovy", "F").await.unwrap_or(45.0);
            let aspect = jvm
                .get_field(&camera, "aspect", "F")
                .await
                .unwrap_or(viewport_w as f32 / viewport_h as f32)
                .max(0.001);
            let near = jvm.get_field::<f32>(&camera, "near", "F").await.unwrap_or(1.0);
            let far = jvm.get_field::<f32>(&camera, "far", "F").await.unwrap_or(1000.0);
            let mode = Camera::projection_mode(jvm, &camera).await.unwrap_or(Camera::PERSPECTIVE);
            let projection = Camera::projection_matrix(jvm, &camera, mode)
                .await
                .unwrap_or_else(|_| perspective_projection_matrix(fovy, aspect, near.max(0.001), far.max(near + 0.001)));
            (fovy, aspect, near, far, projection)
        };
        let camera_scope = if camera.is_null() {
            -1
        } else {
            jvm.get_field::<i32>(&cast_ref::<Camera, Node>(&camera), "scope", "I").await.unwrap_or(-1)
        };
        let prepare_done_ms = context.now();
        let frame = M3G_RENDER_FRAME.fetch_add(1, Ordering::Relaxed);
        let root_class = root.class_definition().name().to_string();
        let root_user_id = jvm.get_field::<i32>(&cast_ref::<Node, Object3D>(&root), "userID", "I").await.unwrap_or(0);
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.render.begin frame={} root={} userID={} viewport={}x{}+{}+{} camera={} fovy={:.3} aspect={:.3} near={:.3} far={:.3} depth={} depthRange={:.3}..{:.3} alphaSubmit={}",
            frame,
            root_class,
            root_user_id,
            viewport_w,
            viewport_h,
            viewport_x,
            viewport_y,
            if camera.is_null() { "null" } else { "active" },
            fovy,
            aspect,
            near,
            far,
            depth_enabled,
            depth_range_near,
            depth_range_far,
            process_alpha
        );

        let lighting_start_ms = context.now();
        let scene_lighting = if use_world_lights {
            Self::collect_scene_lighting(jvm, root.clone(), root_matrix, view).await?
        } else {
            Self::collect_g3d_lights(jvm, this, view).await?
        };
        let lighting_done_ms = context.now();
        let mut stats = M3gRenderStats::default();
        let target_image: ClassInstanceRef<Image> = jvm
            .get_field(&graphics, "targetImage", "Ljavax/microedition/lcdui/Image;")
            .await
            .unwrap_or_else(|_| null_ref());
        let mut gpu_builder = if target_image.is_null() && super::super::gpu_scene::m3g_gpu_enabled() {
            let mut builder = super::super::gpu_scene::M3gGpuBuilder::new(
                viewport_w,
                viewport_h,
                viewport_x,
                viewport_y,
                near,
                far,
                depth_range_near,
                depth_range_far,
                depth_enabled,
                depth_clear,
            );
            if full_frame_changed {
                if let Some(&first) = pixels.first() {
                    if pixels.iter().all(|&pixel| pixel == first) {
                        builder.push_clear_color(first);
                    } else {
                        builder.push_background_image(viewport_w, viewport_h, &pixels);
                    }
                }
            }
            Some(builder)
        } else {
            None
        };
        let meshes_start_ms = lighting_done_ms;
        let mut stack = vec![(root, root_matrix, false, true, 1.0f32)];
        while let Some((node, parent_matrix, apply_local_transform, parent_rendering_enabled, parent_alpha)) = stack.pop() {
            if node.is_null() {
                continue;
            }
            stats.nodes += 1;

            let rendering_enabled = jvm.get_field::<bool>(&node, "renderingEnabled", "Z").await.unwrap_or(true);
            let subtree_rendering_enabled = parent_rendering_enabled && rendering_enabled;
            let alpha_factor = (parent_alpha * jvm.get_field::<f32>(&node, "alphaFactor", "F").await.unwrap_or(1.0)).clamp(0.0, 1.0);
            let world_matrix = if apply_local_transform {
                let local_matrix = Self::node_local_matrix(jvm, &node).await.unwrap_or_else(|_| identity_matrix());
                multiply_matrix(parent_matrix, local_matrix)
            } else {
                parent_matrix
            };
            let class_name = node.class_definition().name().to_string();

            if subtree_rendering_enabled && is_mesh_class(&class_name) {
                let mesh_scope = jvm.get_field::<i32>(&node, "scope", "I").await.unwrap_or(-1);
                if (mesh_scope & camera_scope) != 0 {
                    stats.meshes += 1;
                    let mesh_stats = Self::render_mesh(
                        jvm,
                        context,
                        cast_ref::<Node, Mesh>(&node),
                        frame,
                        world_matrix,
                        view,
                        projection,
                        fovy,
                        aspect,
                        near,
                        far,
                        depth_enabled,
                        depth_range_near,
                        depth_range_far,
                        viewport_w,
                        viewport_h,
                        &mut pixels,
                        &mut depth,
                        &scene_lighting,
                        mesh_scope,
                        alpha_factor,
                        gpu_builder.as_mut(),
                    )
                    .await?;
                    stats.vertices += mesh_stats.vertices;
                    stats.projected_vertices += mesh_stats.projected_vertices;
                    stats.candidate_triangles += mesh_stats.candidate_triangles;
                    stats.triangles += mesh_stats.rasterized_triangles;
                    stats.pixels += mesh_stats.pixels;
                    stats.dirty.include_dirty(mesh_stats.dirty);
                    stats.mesh_vertex_ms = stats.mesh_vertex_ms.saturating_add(mesh_stats.vertex_ms);
                    stats.mesh_appearance_ms = stats.mesh_appearance_ms.saturating_add(mesh_stats.appearance_ms);
                    stats.mesh_raster_ms = stats.mesh_raster_ms.saturating_add(mesh_stats.raster_ms);
                    if mesh_stats.rasterized_triangles > 0 {
                        stats.rendered_meshes += 1;
                    }
                }
                push_skinned_skeleton(jvm, &node, &mut stack, world_matrix, subtree_rendering_enabled, alpha_factor).await?;
            }

            if subtree_rendering_enabled && class_name == "javax/microedition/m3g/Sprite3D" {
                Self::render_sprite3d(
                    jvm,
                    cast_ref::<Node, Sprite3D>(&node),
                    world_matrix,
                    view,
                    projection,
                    viewport_w,
                    viewport_h,
                    &mut pixels,
                    &mut depth,
                    depth_enabled,
                    near,
                    alpha_factor,
                    &mut stats,
                    gpu_builder.as_mut(),
                )
                .await?;
            }

            if subtree_rendering_enabled && (class_name == "javax/microedition/m3g/Group" || class_name == "javax/microedition/m3g/World") {
                let group: ClassInstanceRef<Group> = cast_ref(&node);
                let child_count: i32 = jvm.get_field(&group, "childCount", "I").await.unwrap_or(0);
                if child_count > 0 {
                    let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> =
                        jvm.get_field(&group, "children", "[Ljavax/microedition/m3g/Node;").await?;
                    let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
                    for child in children.into_iter().rev() {
                        stack.push((child, world_matrix, true, subtree_rendering_enabled, alpha_factor));
                    }
                }
            }
        }
        let meshes_done_ms = context.now();

        let gpu_published = gpu_builder.map(|builder| builder.finish(full_frame_changed)).unwrap_or(false);
        let submitted = stats.triangles > 0 || submit_when_empty || gpu_published;
        let draw_start_ms = meshes_done_ms;
        if gpu_published && full_frame_changed {
            Graphics::replace_pixels(
                jvm,
                context,
                &graphics,
                viewport_x,
                viewport_y,
                viewport_w,
                viewport_h,
                &vec![0; pixel_count],
            )
            .await?;
        } else if submitted && !gpu_published {
            if full_frame_changed || submit_when_empty || stats.dirty.is_empty() {
                Graphics::draw_pixels(
                    jvm,
                    context,
                    &graphics,
                    viewport_x,
                    viewport_y,
                    viewport_w,
                    viewport_h,
                    &pixels,
                    process_alpha,
                )
                .await?;
            } else if let Some((dirty_x, dirty_y, dirty_w, dirty_h)) = stats.dirty.bounds() {
                Graphics::draw_pixels_strided(
                    jvm,
                    context,
                    &graphics,
                    viewport_x + dirty_x,
                    viewport_y + dirty_y,
                    dirty_w,
                    dirty_h,
                    &pixels,
                    viewport_w,
                    dirty_x,
                    dirty_y,
                    process_alpha,
                )
                .await?;
            }
        }
        let draw_done_ms = context.now();
        let buffers_changed = stats.triangles > 0 || full_frame_changed || depth_clear;
        if buffers_changed && !gpu_published {
            Self::store_color_buffer(jvm, this, &pixels).await?;
            Self::store_depth_buffer(jvm, this, &depth).await?;
        }
        let now_ms = context.now();
        let prepare_ms = prepare_done_ms.saturating_sub(render_start_ms);
        let lighting_ms = lighting_done_ms.saturating_sub(lighting_start_ms);
        let meshes_ms = meshes_done_ms.saturating_sub(meshes_start_ms);
        let draw_ms = draw_done_ms.saturating_sub(draw_start_ms);
        let store_ms = now_ms.saturating_sub(draw_done_ms);
        let total_ms = now_ms.saturating_sub(render_start_ms);
        publish_render_diagnostic(
            now_ms,
            total_ms,
            format!(
                "f={frame} gpu={} total={total_ms}ms prep={prepare_ms} light={lighting_ms} mesh={meshes_ms} draw={draw_ms} store={store_ms} | meshParts v={} app={} rast={} | nodes={} meshes={}/{} verts={}/{} tri={}/{} px={} dirty={:?}",
                gpu_published,
                stats.mesh_vertex_ms,
                stats.mesh_appearance_ms,
                stats.mesh_raster_ms,
                stats.nodes,
                stats.rendered_meshes,
                stats.meshes,
                stats.projected_vertices,
                stats.vertices,
                stats.triangles,
                stats.candidate_triangles,
                stats.pixels,
                stats.dirty.bounds()
            ),
        );
        let previous_ms = M3G_RENDER_LAST_END_MS.swap(now_ms, Ordering::Relaxed);
        if previous_ms != 0 {
            let gap_ms = now_ms.saturating_sub(previous_ms);
            if gap_ms >= 50 {
                tracing::warn!(
                    target: "rustjava_m3g",
                    "m3g.render.slow_gap gapMs={} frame={} nodes={} meshes={} renderedMeshes={} triangles={} pixels={} submitted={} dirty={:?}",
                    gap_ms,
                    frame,
                    stats.nodes,
                    stats.meshes,
                    stats.rendered_meshes,
                    stats.triangles,
                    stats.pixels,
                    submitted,
                    stats.dirty.bounds()
                );
            }
        }
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.render.end frame={} gpu={} nodes={} meshes={} renderedMeshes={} triangles={} pixels={} submitted={} buffersChanged={} dirty={:?}",
            frame,
            gpu_published,
            stats.nodes,
            stats.meshes,
            stats.rendered_meshes,
            stats.triangles,
            stats.pixels,
            submitted,
            buffers_changed,
            stats.dirty.bounds()
        );
        Ok(())
    }
}

pub(super) fn apply_alpha_factor(color: i32, alpha_bits: u32) -> i32 {
    let color = color as u32;
    let alpha = (((color >> 24) & 0xff) * alpha_bits / 255).min(255);
    ((alpha << 24) | (color & 0x00ff_ffff)) as i32
}
