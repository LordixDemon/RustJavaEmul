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
    pub(crate) async fn render_sprite3d(
        jvm: &Jvm,
        sprite: ClassInstanceRef<Sprite3D>,
        world_matrix: [f32; 16],
        view: [f32; 16],
        projection: [f32; 16],
        viewport_w: i32,
        viewport_h: i32,
        pixels: &mut [i32],
        depth: &mut [f32],
        depth_enabled: bool,
        near: f32,
        alpha_factor: f32,
        stats: &mut M3gRenderStats,
        gpu: Option<&mut super::super::super::gpu_scene::M3gGpuBuilder>,
    ) -> Result<()> {
        let image2d: ClassInstanceRef<Image2D> = jvm
            .get_field(&sprite, "image", "Ljavax/microedition/m3g/Image2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image2d.is_null() {
            return Ok(());
        }
        let image: ClassInstanceRef<Image> = jvm
            .get_field(&image2d, "image", "Ljavax/microedition/lcdui/Image;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image.is_null() {
            return Ok(());
        }
        let (image_w, image_h, image_pixels) = Self::image_pixels_cached(jvm, &image).await?;
        if image_w <= 0 || image_h <= 0 || image_pixels.is_empty() {
            return Ok(());
        }
        let crop_x = jvm.get_field::<i32>(&sprite, "cropX", "I").await.unwrap_or(0).max(0);
        let crop_y = jvm.get_field::<i32>(&sprite, "cropY", "I").await.unwrap_or(0).max(0);
        let mut crop_w = jvm.get_field::<i32>(&sprite, "cropW", "I").await.unwrap_or(0);
        let mut crop_h = jvm.get_field::<i32>(&sprite, "cropH", "I").await.unwrap_or(0);
        if crop_w <= 0 {
            crop_w = image_w;
        }
        if crop_h <= 0 {
            crop_h = image_h;
        }
        let scaled = jvm.get_field::<bool>(&sprite, "scaled", "Z").await.unwrap_or(false);
        let model_view = multiply_matrix(view, world_matrix);
        let Some(origin) = camera_vertex([0.0, 0.0, 0.0], [0.0, 0.0], [0.0, 0.0], -1, model_view) else {
            return Ok(());
        };
        if origin.depth <= near {
            return Ok(());
        }
        let Some(projected) = project_clip_vertex(origin, projection, viewport_w, viewport_h) else {
            return Ok(());
        };
        let (start_x, start_y, draw_w, draw_h) = if scaled {
            let corners = [[-0.5f32, -0.5, 0.0], [0.5, -0.5, 0.0], [0.5, 0.5, 0.0], [-0.5, 0.5, 0.0]];
            let mut min_x = viewport_w as f32;
            let mut min_y = viewport_h as f32;
            let mut max_x = 0.0f32;
            let mut max_y = 0.0f32;
            let mut any = false;
            for corner in corners {
                let Some(camera) = camera_vertex(corner, [0.0, 0.0], [0.0, 0.0], -1, model_view) else {
                    continue;
                };
                let Some(screen) = project_clip_vertex(camera, projection, viewport_w, viewport_h) else {
                    continue;
                };
                min_x = min_x.min(screen.x);
                min_y = min_y.min(screen.y);
                max_x = max_x.max(screen.x);
                max_y = max_y.max(screen.y);
                any = true;
            }
            if !any {
                return Ok(());
            }
            let start_x = min_x.floor() as i32;
            let start_y = min_y.floor() as i32;
            (
                start_x,
                start_y,
                (max_x.ceil() as i32 - start_x).max(1),
                (max_y.ceil() as i32 - start_y).max(1),
            )
        } else {
            (
                projected.x as i32 - crop_w / 2,
                projected.y as i32 - crop_h / 2,
                crop_w.max(1),
                crop_h.max(1),
            )
        };
        let appearance: ClassInstanceRef<Appearance> = jvm
            .get_field(&sprite, "appearance", "Ljavax/microedition/m3g/Appearance;")
            .await
            .unwrap_or_else(|_| null_ref());
        let mut blending = CompositingMode::REPLACE;
        let mut alpha_threshold = 0.0f32;
        let mut depth_test = true;
        let mut depth_write = true;
        let mut fog = None;
        if !appearance.is_null() {
            let compositing: ClassInstanceRef<CompositingMode> = jvm
                .get_field(&appearance, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;")
                .await
                .unwrap_or_else(|_| null_ref());
            if !compositing.is_null() {
                blending = jvm
                    .get_field::<i32>(&compositing, "blending", "I")
                    .await
                    .unwrap_or(CompositingMode::REPLACE);
                alpha_threshold = jvm.get_field::<f32>(&compositing, "alphaThreshold", "F").await.unwrap_or(0.0);
                depth_test = jvm.get_field::<bool>(&compositing, "depthTest", "Z").await.unwrap_or(true);
                depth_write = jvm.get_field::<bool>(&compositing, "depthWrite", "Z").await.unwrap_or(true);
            }
            let fog_obj: ClassInstanceRef<Fog> = jvm
                .get_field(&appearance, "fog", "Ljavax/microedition/m3g/Fog;")
                .await
                .unwrap_or_else(|_| null_ref());
            if !fog_obj.is_null() {
                fog = Some(M3gFogState {
                    mode: jvm.get_field::<i32>(&fog_obj, "mode", "I").await.unwrap_or(Fog::LINEAR),
                    color: jvm.get_field::<i32>(&fog_obj, "color", "I").await.unwrap_or(0),
                    density: jvm.get_field::<f32>(&fog_obj, "density", "F").await.unwrap_or(1.0).max(0.0),
                    near: jvm.get_field::<f32>(&fog_obj, "near", "F").await.unwrap_or(0.0),
                    far: jvm.get_field::<f32>(&fog_obj, "far", "F").await.unwrap_or(1.0),
                });
            }
        }
        let alpha_bits = ((alpha_factor.clamp(0.0, 1.0) * 255.0) as u32).min(255);
        let sprite_z = origin.depth.max(near);
        if let Some(gpu) = gpu {
            gpu.push_sprite_quad(
                start_x as f32,
                start_y as f32,
                draw_w as f32,
                draw_h as f32,
                crop_x as f32,
                crop_y as f32,
                crop_w as f32,
                crop_h as f32,
                image_w,
                image_h,
                &image_pixels,
                super::apply_alpha_factor(0x00ff_ffffu32 as i32, alpha_bits),
                blending,
                sprite_z,
                depth_test,
                depth_write,
                alpha_threshold,
                fog,
            );
            stats.triangles += 2;
            stats.rendered_meshes += 1;
            stats.dirty.include_rect(
                start_x.max(0),
                start_y.max(0),
                (start_x + draw_w).min(viewport_w),
                (start_y + draw_h).min(viewport_h),
            );
            return Ok(());
        }
        let mut drawn = 0usize;
        for dy in 0..draw_h {
            let dest_y = start_y + dy;
            if dest_y < 0 || dest_y >= viewport_h {
                continue;
            }
            let src_y = crop_y + dy * crop_h / draw_h;
            if src_y < 0 || src_y >= image_h {
                continue;
            }
            for dx in 0..draw_w {
                let dest_x = start_x + dx;
                if dest_x < 0 || dest_x >= viewport_w {
                    continue;
                }
                let src_x = crop_x + dx * crop_w / draw_w;
                if src_x < 0 || src_x >= image_w {
                    continue;
                }
                let mut src = super::apply_alpha_factor(image_pixels[(src_y * image_w + src_x) as usize], alpha_bits);
                let alpha = ((src as u32) >> 24) & 0xff;
                if alpha == 0 || (alpha as f32 / 255.0) < alpha_threshold {
                    continue;
                }
                let index = (dest_y * viewport_w + dest_x) as usize;
                if depth_enabled && depth_test && sprite_z > depth[index] {
                    continue;
                }
                if let Some(fog) = fog {
                    src = apply_distance_fog(src, fog, sprite_z);
                }
                if depth_enabled && depth_write {
                    depth[index] = sprite_z;
                }
                pixels[index] = compose_m3g_pixel(pixels[index], src, blending);
                drawn += 1;
            }
        }
        if drawn > 0 {
            stats.pixels += drawn;
            stats.triangles += 1;
            stats.rendered_meshes += 1;
            stats.dirty.include_rect(
                start_x.max(0),
                start_y.max(0),
                (start_x + draw_w).min(viewport_w),
                (start_y + draw_h).min(viewport_h),
            );
        }
        Ok(())
    }
}
