use super::super::{
    AffineTrans, FigureLayout,
    constants::{PROJECTION_PARALLEL_SIZE, PROJECTION_PERSPECTIVE_FOV, PROJECTION_PERSPECTIVE_WH},
    diagnostics::V3FrameDrawStats,
    gpu_scene::{gpu_scene_enabled, gpu_scene_rejection_reason, invalidate_gpu_image, publish_gpu_scene},
    math::{identity_matrix, sin_cos_mc},
    raster::{SceneTri, rasterize_triangle, rasterize_triangle_into_pixels},
    scene::ProjectionParams,
    storage::{get_affine_matrix, load_int_field},
    texture::NativeTexture,
};
use super::prepare::ScreenTri;
#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeContext,
    classes::javax::microedition::lcdui::{Graphics, Image},
};
#[allow(unused_imports)]
use alloc::{vec, vec::Vec};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

pub(crate) async fn draw_scene_triangles(
    jvm: &Jvm,
    context: &mut RuntimeContext,
    graphics_ref: &ClassInstanceRef<Graphics>,
    textures: &[Option<NativeTexture>],
    triangles: &[SceneTri],
) -> Result<V3FrameDrawStats> {
    let mut stats = V3FrameDrawStats::default();
    let setup_start_ms = context.now();
    let (clip_x, clip_y, clip_w, clip_h) = graphics_clip(jvm, context, graphics_ref).await?;
    let target_image: ClassInstanceRef<Image> = if graphics_ref.is_null() {
        ClassInstanceRef::new(None)
    } else {
        jvm.get_field(graphics_ref, "targetImage", "Ljavax/microedition/lcdui/Image;").await?
    };
    stats.setup_ms = context.now().saturating_sub(setup_start_ms);
    stats.batched = target_image.is_null() && can_batch_screen_triangles(triangles);
    stats.target_image = !target_image.is_null();

    if target_image.is_null() {
        let screen_width = context.screen_width();
        let screen_height = context.screen_height();
        if gpu_scene_enabled() && publish_gpu_scene(screen_width, screen_height, (clip_x, clip_y, clip_w, clip_h), textures, triangles) {
            stats.batched = true;
            stats.gpu_direct = true;
            stats.rasterized_triangles = triangles.len();
            stats.draw_calls = 1;
            return Ok(stats);
        }
        stats.gpu_reject = gpu_scene_rejection_reason(screen_width, screen_height, (clip_x, clip_y, clip_w, clip_h), triangles);

        if stats.batched {
            draw_batched_screen_triangles(
                jvm,
                context,
                graphics_ref,
                textures,
                triangles,
                clip_x,
                clip_y,
                clip_w,
                clip_h,
                &mut stats,
            )
            .await?;
        } else {
            draw_screen_triangles_individually(
                jvm,
                context,
                graphics_ref,
                textures,
                triangles,
                clip_x,
                clip_y,
                clip_w,
                clip_h,
                &mut stats,
            )
            .await?;
        }
    } else {
        let load_start_ms = context.now();
        let (target_width, target_height, mut target_pixels_array) = Image::pixels(jvm, &target_image).await?;
        if target_width > 0 && target_height > 0 {
            invalidate_gpu_image(&target_pixels_array);

            let expected = (target_width * target_height) as usize;
            let count = jvm.array_length(&target_pixels_array).await?.min(expected);
            let mut target_pixels: Vec<i32> = jvm.load_array(&target_pixels_array, 0, count).await?;
            if target_pixels.len() < expected {
                target_pixels.resize(expected, 0);
            }
            stats.setup_ms += context.now().saturating_sub(load_start_ms);

            let mut changed = false;
            let mut dirty_bounds = None;
            let raster_start_ms = context.now();
            for scene_tri in triangles {
                let (tri_clip_x, tri_clip_y, tri_clip_w, tri_clip_h) = effective_clip((clip_x, clip_y, clip_w, clip_h), scene_tri.clip);
                let tri_changed = rasterize_triangle_into_pixels(
                    &scene_tri.tri,
                    textures,
                    &mut target_pixels,
                    target_width,
                    target_height,
                    tri_clip_x,
                    tri_clip_y,
                    tri_clip_w,
                    tri_clip_h,
                    scene_tri.effect_transparency,
                );
                if tri_changed {
                    stats.rasterized_triangles += 1;
                    update_dirty_bounds(
                        &mut dirty_bounds,
                        triangle_bounds(
                            &scene_tri.tri,
                            target_width,
                            target_height,
                            tri_clip_x,
                            tri_clip_y,
                            tri_clip_w,
                            tri_clip_h,
                        ),
                    );
                }
                changed |= tri_changed;
            }
            stats.raster_ms += context.now().saturating_sub(raster_start_ms);

            if changed {
                let store_start_ms = context.now();
                let submitted = store_target_pixels(jvm, &mut target_pixels_array, target_width, target_height, target_pixels, dirty_bounds).await?;
                stats.store_ms += context.now().saturating_sub(store_start_ms);
                stats.draw_calls += 1;
                stats.submitted_pixels += submitted;
            }
        }
    }

    Ok(stats)
}

pub(crate) async fn store_target_pixels(
    jvm: &Jvm,
    target_pixels_array: &mut ClassInstanceRef<Array<i32>>,
    target_width: i32,
    target_height: i32,
    target_pixels: Vec<i32>,
    dirty_bounds: Option<(i32, i32, i32, i32)>,
) -> Result<usize> {
    let expected = (target_width as usize).saturating_mul(target_height as usize);
    let Some((min_x, min_y, max_x, max_y)) = dirty_bounds else {
        jvm.store_array(target_pixels_array, 0, target_pixels).await?;
        return Ok(expected);
    };

    let dirty_width = (max_x - min_x + 1).max(0) as usize;
    let dirty_height = (max_y - min_y + 1).max(0) as usize;
    let dirty_area = dirty_width.saturating_mul(dirty_height);
    if dirty_area == 0 || dirty_area.saturating_mul(4) >= expected {
        jvm.store_array(target_pixels_array, 0, target_pixels).await?;
        return Ok(expected);
    }

    let target_width_usize = target_width as usize;
    for row in min_y..=max_y {
        let row_start = row as usize * target_width_usize + min_x as usize;
        let row_end = row_start + dirty_width;
        let Some(row_pixels) = target_pixels.get(row_start..row_end) else {
            continue;
        };
        jvm.store_array(target_pixels_array, row_start, row_pixels.iter().copied()).await?;
    }

    Ok(dirty_area)
}

pub(crate) fn update_dirty_bounds(bounds: &mut Option<(i32, i32, i32, i32)>, changed: Option<(i32, i32, i32, i32)>) {
    let Some((min_x, min_y, max_x, max_y)) = changed else {
        return;
    };
    match bounds {
        Some((current_min_x, current_min_y, current_max_x, current_max_y)) => {
            *current_min_x = (*current_min_x).min(min_x);
            *current_min_y = (*current_min_y).min(min_y);
            *current_max_x = (*current_max_x).max(max_x);
            *current_max_y = (*current_max_y).max(max_y);
        }
        None => *bounds = Some((min_x, min_y, max_x, max_y)),
    }
}

pub(crate) fn triangle_bounds(
    tri: &ScreenTri,
    target_width: i32,
    target_height: i32,
    clip_x: i32,
    clip_y: i32,
    clip_w: i32,
    clip_h: i32,
) -> Option<(i32, i32, i32, i32)> {
    if target_width <= 0 || target_height <= 0 || clip_w <= 0 || clip_h <= 0 {
        return None;
    }

    let min_x = tri.vertices.iter().map(|vertex| vertex.x).min()?.max(clip_x).max(0);
    let max_x = tri
        .vertices
        .iter()
        .map(|vertex| vertex.x)
        .max()?
        .min(clip_x + clip_w - 1)
        .min(target_width - 1);
    let min_y = tri.vertices.iter().map(|vertex| vertex.y).min()?.max(clip_y).max(0);
    let max_y = tri
        .vertices
        .iter()
        .map(|vertex| vertex.y)
        .max()?
        .min(clip_y + clip_h - 1)
        .min(target_height - 1);

    (min_x <= max_x && min_y <= max_y).then_some((min_x, min_y, max_x, max_y))
}

pub(crate) fn can_batch_screen_triangles(_triangles: &[SceneTri]) -> bool {
    true
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn draw_batched_screen_triangles(
    jvm: &Jvm,
    context: &mut RuntimeContext,
    graphics_ref: &ClassInstanceRef<Graphics>,
    textures: &[Option<NativeTexture>],
    triangles: &[SceneTri],
    clip_x: i32,
    clip_y: i32,
    clip_w: i32,
    clip_h: i32,
    stats: &mut V3FrameDrawStats,
) -> Result<()> {
    let target_width = context.screen_width();
    let target_height = context.screen_height();
    if target_width <= 0 || target_height <= 0 {
        return Ok(());
    }

    let mut target_pixels = vec![0; (target_width * target_height) as usize];
    let mut changed = false;
    let raster_start_ms = context.now();
    for scene_tri in triangles {
        let (tri_clip_x, tri_clip_y, tri_clip_w, tri_clip_h) = effective_clip((clip_x, clip_y, clip_w, clip_h), scene_tri.clip);
        let tri_changed = rasterize_triangle_into_pixels(
            &scene_tri.tri,
            textures,
            &mut target_pixels,
            target_width,
            target_height,
            tri_clip_x,
            tri_clip_y,
            tri_clip_w,
            tri_clip_h,
            scene_tri.effect_transparency,
        );
        if tri_changed {
            stats.rasterized_triangles += 1;
        }
        changed |= tri_changed;
    }
    stats.raster_ms += context.now().saturating_sub(raster_start_ms);

    if changed {
        let upload_start_ms = context.now();
        Graphics::draw_pixels_strided(
            jvm,
            context,
            graphics_ref,
            0,
            0,
            target_width,
            target_height,
            &target_pixels,
            target_width,
            0,
            0,
            true,
        )
        .await?;
        stats.upload_ms += context.now().saturating_sub(upload_start_ms);
        stats.draw_calls += 1;
        stats.submitted_pixels += (target_width * target_height) as usize;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn draw_screen_triangles_individually(
    jvm: &Jvm,
    context: &mut RuntimeContext,
    graphics_ref: &ClassInstanceRef<Graphics>,
    textures: &[Option<NativeTexture>],
    triangles: &[SceneTri],
    clip_x: i32,
    clip_y: i32,
    clip_w: i32,
    clip_h: i32,
    stats: &mut V3FrameDrawStats,
) -> Result<()> {
    for scene_tri in triangles {
        let (tri_clip_x, tri_clip_y, tri_clip_w, tri_clip_h) = effective_clip((clip_x, clip_y, clip_w, clip_h), scene_tri.clip);
        let raster_start_ms = context.now();
        if let Some((x, y, width, height, pixels)) = rasterize_triangle(
            &scene_tri.tri,
            textures,
            tri_clip_x,
            tri_clip_y,
            tri_clip_w,
            tri_clip_h,
            scene_tri.effect_transparency,
        ) {
            stats.raster_ms += context.now().saturating_sub(raster_start_ms);
            stats.rasterized_triangles += 1;
            stats.submitted_pixels += (width * height).max(0) as usize;
            let upload_start_ms = context.now();
            Graphics::draw_pixels(jvm, context, graphics_ref, x, y, width, height, &pixels, true).await?;
            stats.upload_ms += context.now().saturating_sub(upload_start_ms);
            stats.draw_calls += 1;
        } else {
            stats.raster_ms += context.now().saturating_sub(raster_start_ms);
        }
    }

    Ok(())
}

pub(crate) fn effective_clip(base: (i32, i32, i32, i32), override_clip: Option<(i32, i32, i32, i32)>) -> (i32, i32, i32, i32) {
    override_clip.unwrap_or(base)
}

pub(crate) async fn graphics_clip(jvm: &Jvm, context: &mut RuntimeContext, graphics: &ClassInstanceRef<Graphics>) -> Result<(i32, i32, i32, i32)> {
    if graphics.is_null() {
        return Ok((0, 0, context.screen_width(), context.screen_height()));
    }

    Ok((
        jvm.get_field(graphics, "clipX", "I").await?,
        jvm.get_field(graphics, "clipY", "I").await?,
        jvm.get_field(graphics, "clipW", "I").await?,
        jvm.get_field(graphics, "clipH", "I").await?,
    ))
}

pub(crate) async fn projection_params(
    jvm: &Jvm,
    context: &mut RuntimeContext,
    layout: &ClassInstanceRef<FigureLayout>,
    x: i32,
    y: i32,
) -> Result<ProjectionParams> {
    if layout.is_null() {
        return Ok(ProjectionParams {
            perspective: false,
            near: 1,
            far: 32767,
            scale_x: 4096,
            scale_y: 4096,
            center_x: context.screen_width() / 2 + x,
            center_y: context.screen_height() / 2 + y,
        });
    }

    let center_x: i32 = jvm.get_field(layout, "centerX", "I").await?;
    let center_y: i32 = jvm.get_field(layout, "centerY", "I").await?;
    let mode: i32 = jvm.get_field(layout, "projectionMode", "I").await?;
    let near: i32 = jvm.get_field(layout, "near", "I").await?;
    let far: i32 = jvm.get_field(layout, "far", "I").await?;

    if mode == PROJECTION_PERSPECTIVE_FOV {
        let angle: i32 = jvm.get_field(layout, "perspective", "I").await?;
        let (sin, cos) = sin_cos_mc(angle / 2);
        let scale = if sin.abs() < 8 {
            context.screen_width()
        } else {
            ((context.screen_width() as i64 * cos as i64) / (2 * sin as i64)) as i32
        }
        .abs()
        .max(1);
        return Ok(ProjectionParams {
            perspective: true,
            near: near.max(1),
            far: far.max(near + 1),
            scale_x: scale,
            scale_y: scale,
            center_x: center_x + x,
            center_y: center_y + y,
        });
    }

    if mode == PROJECTION_PERSPECTIVE_WH {
        let width: i32 = jvm.get_field(layout, "perspective", "I").await?;
        let height: i32 = jvm.get_field(layout, "projection", "I").await?;
        let near = near.max(1);
        let scale_x = if width > 0 {
            (((context.screen_width() as i64) << 12) * near as i64 / width as i64) as i32
        } else {
            context.screen_width()
        };
        let scale_y = if height > 0 {
            (((context.screen_height() as i64) << 12) * near as i64 / height as i64) as i32
        } else {
            scale_x
        };
        return Ok(ProjectionParams {
            perspective: true,
            near,
            far: far.max(near + 1),
            scale_x,
            scale_y,
            center_x: center_x + x,
            center_y: center_y + y,
        });
    }

    if mode == PROJECTION_PARALLEL_SIZE {
        let width: i32 = jvm.get_field(layout, "parallelWidth", "I").await?;
        let height: i32 = jvm.get_field(layout, "parallelHeight", "I").await?;
        if width > 0 && height > 0 {
            return Ok(ProjectionParams {
                perspective: false,
                near: 1,
                far: 32767,
                scale_x: (((context.screen_width() as i64) << 12) / width as i64) as i32,
                scale_y: (((context.screen_height() as i64) << 12) / height as i64) as i32,
                center_x: center_x + x,
                center_y: center_y + y,
            });
        }
    }

    Ok(ProjectionParams {
        perspective: false,
        near: 1,
        far: 32767,
        scale_x: jvm.get_field(layout, "scaleX", "I").await?,
        scale_y: jvm.get_field(layout, "scaleY", "I").await?,
        center_x: center_x + x,
        center_y: center_y + y,
    })
}

pub(crate) async fn layout_matrix(jvm: &Jvm, layout: &ClassInstanceRef<FigureLayout>) -> Result<[i32; 12]> {
    if layout.is_null() {
        return Ok(identity_matrix());
    }

    let matrix = load_int_field(jvm, layout, "affineMatrix").await?;
    if matrix.len() >= 12 {
        let mut result = [0; 12];
        result.copy_from_slice(&matrix[..12]);
        return Ok(result);
    }

    let affine: ClassInstanceRef<AffineTrans> = jvm.get_field(layout, "affineTrans", "Lcom/mascotcapsule/micro3d/v3/AffineTrans;").await?;
    if affine.is_null() {
        Ok(identity_matrix())
    } else {
        get_affine_matrix(jvm, &affine).await
    }
}
