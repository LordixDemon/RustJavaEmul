use alloc::{vec, vec::Vec};

use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeContext,
    classes::javax::microedition::lcdui::{Graphics, Image},
};

use super::{
    AffineTrans, Effect3D, Figure, FigureLayout,
    constants::{
        BONE_STRIDE, MAT_BLEND_MASK, MAT_COLORKEY, MAT_DOUBLE_FACE, PROJECTION_PARALLEL_SIZE, PROJECTION_PERSPECTIVE_FOV, PROJECTION_PERSPECTIVE_WH,
        QUAD_C_STRIDE, QUAD_T_STRIDE, TRI_C_STRIDE, TRI_T_STRIDE,
    },
    diagnostics::V3FrameDrawStats,
    gpu_scene::{gpu_scene_enabled, gpu_scene_rejection_reason, invalidate_gpu_image, publish_gpu_image_scene, publish_gpu_scene},
    math::{fixed_mul3, identity_matrix, sin_cos_mc},
    raster::{PreparedFigure, PreparedSource, SceneTri, push_render_tri, rasterize_triangle, rasterize_triangle_into_pixels},
    scene::{
        ProjectedVertex, ProjectionParams, apply_active_material_mask, collect_render_triangles, mark_implicit_indexed_color_keys,
        project_figure_vertices, projected_bounds, transformed_bounds, triangle_in_depth_range,
    },
    storage::{get_affine_matrix, load_figure_textures, load_int_field, load_native_texture, load_runtime_figure},
    texture::NativeTexture,
};

#[allow(clippy::too_many_arguments)]
pub(super) async fn prepare_native_figure(
    jvm: &Jvm,
    context: &mut RuntimeContext,
    figure_ref: &ClassInstanceRef<Figure>,
    x: i32,
    y: i32,
    layout_ref: &ClassInstanceRef<FigureLayout>,
    effect_ref: &ClassInstanceRef<Effect3D>,
) -> Result<PreparedFigure> {
    if figure_ref.is_null() {
        return Ok(empty_prepared_figure(PreparedSource::Figure));
    }

    let figure = load_runtime_figure(jvm, figure_ref).await?;
    if figure.vertex_count == 0 || figure.vertices.len() < figure.vertex_count * 3 {
        return Ok(empty_prepared_figure(PreparedSource::Figure));
    }

    let textures = load_figure_textures(jvm, figure_ref).await?;
    let projection = projection_params(jvm, context, layout_ref, x, y).await?;
    let view = layout_matrix(jvm, layout_ref).await?;
    let projected = project_figure_vertices(&figure, view, &projection);
    let mut triangles = collect_render_triangles(&figure, &projected, &textures);
    if projection.perspective {
        triangles.retain(|tri| triangle_in_depth_range(tri, &projection));
    }
    mark_implicit_indexed_color_keys(&mut triangles, &textures);
    let effect_transparency = if effect_ref.is_null() {
        false
    } else {
        jvm.get_field(effect_ref, "transparency", "Z").await?
    };
    apply_active_material_mask(&mut triangles, effect_transparency);

    if tracing::enabled!(target: "java_runtime::classes::com::mascotcapsule::micro3d::v3", tracing::Level::DEBUG) {
        let transformed = transformed_bounds(&figure, view);
        let (min_x, min_y, max_x, max_y, min_z, max_z) = projected_bounds(&projected);
        tracing::debug!(
            "MCv3 prepareFigure vertices={} c3={} c4={} t3={} t4={} patterns={} selected={:#x} bones={} posture={} textures={} triangles={} transformed=({},{},{}..{},{},{}) projected=({},{}..{},{} z={}..{}) view={:?} perspective={} center=({}, {}) scale=({}, {})",
            figure.vertex_count,
            figure.poly_c3.len() / TRI_C_STRIDE,
            figure.poly_c4.len() / QUAD_C_STRIDE,
            figure.poly_t3.len() / TRI_T_STRIDE,
            figure.poly_t4.len() / QUAD_T_STRIDE,
            figure.pattern_count,
            figure.selected_pattern,
            figure.bones.len() / BONE_STRIDE,
            figure.posture_bones.len() / 12,
            textures.iter().filter(|texture| texture.is_some()).count(),
            triangles.len(),
            transformed.0,
            transformed.1,
            transformed.2,
            transformed.3,
            transformed.4,
            transformed.5,
            min_x,
            min_y,
            max_x,
            max_y,
            min_z,
            max_z,
            view,
            projection.perspective,
            projection.center_x,
            projection.center_y,
            projection.scale_x,
            projection.scale_y,
        );
    }

    Ok(PreparedFigure {
        textures,
        triangles,
        effect_transparency,
        clip: None,
        source: PreparedSource::Figure,
        prepare_ms: 0,
    })
}

fn empty_prepared_figure(source: PreparedSource) -> PreparedFigure {
    PreparedFigure {
        textures: Vec::new(),
        triangles: Vec::new(),
        effect_transparency: false,
        clip: None,
        source,
        prepare_ms: 0,
    }
}

pub(super) struct PrimitiveRenderState {
    pub(super) projection: ProjectionParams,
    pub(super) view: [i32; 12],
    pub(super) effect_transparency: bool,
}

pub(super) async fn primitive_render_state(
    jvm: &Jvm,
    context: &mut RuntimeContext,
    layout_ref: &ClassInstanceRef<FigureLayout>,
    effect_ref: &ClassInstanceRef<Effect3D>,
    x: i32,
    y: i32,
) -> Result<PrimitiveRenderState> {
    Ok(PrimitiveRenderState {
        projection: projection_params(jvm, context, layout_ref, x, y).await?,
        view: layout_matrix(jvm, layout_ref).await?,
        effect_transparency: if effect_ref.is_null() {
            false
        } else {
            jvm.get_field(effect_ref, "transparency", "Z").await?
        },
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn prepare_primitives(
    jvm: &Jvm,
    context: &mut RuntimeContext,
    texture_ref: &ClassInstanceRef<super::Texture>,
    x: i32,
    y: i32,
    layout_ref: &ClassInstanceRef<FigureLayout>,
    effect_ref: &ClassInstanceRef<Effect3D>,
    command: i32,
    num_primitives: i32,
    vertex_coords: &[i32],
    vertex_offset: usize,
    texture_coords: &[i32],
    texture_offset: usize,
    colors: &[i32],
    color_offset: usize,
) -> Result<PreparedFigure> {
    let state = primitive_render_state(jvm, context, layout_ref, effect_ref, x, y).await?;
    prepare_primitives_with_state(
        jvm,
        texture_ref,
        &state,
        command,
        num_primitives,
        vertex_coords,
        vertex_offset,
        texture_coords,
        texture_offset,
        colors,
        color_offset,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn prepare_primitives_with_state(
    jvm: &Jvm,
    texture_ref: &ClassInstanceRef<super::Texture>,
    state: &PrimitiveRenderState,
    command: i32,
    num_primitives: i32,
    vertex_coords: &[i32],
    vertex_offset: usize,
    texture_coords: &[i32],
    texture_offset: usize,
    colors: &[i32],
    color_offset: usize,
) -> Result<PreparedFigure> {
    const PRIMITIVE_POINTS: i32 = 0x0100_0000;
    const PRIMITIVE_LINES: i32 = 0x0200_0000;
    const PRIMITIVE_TRIANGLES: i32 = 0x0300_0000;
    const PRIMITIVE_QUADS: i32 = 0x0400_0000;
    const PRIMITIVE_POINT_SPRITES: i32 = 0x0500_0000;
    const PATTR_COLORKEY: i32 = 0x10;
    const PATTR_BLEND_MASK: i32 = 0x60;
    const PDATA_COLOR_MASK: i32 = 0x0c00;
    const PDATA_TEXTURE_COORD: i32 = 0x3000;
    const PDATA_SPRITE_PARAMS_MASK: i32 = 0x3000;
    const PDATA_POINT_SPRITE_PARAMS_PER_CMD: i32 = 0x1000;

    fn primitive_material(command: i32) -> i32 {
        let mut mat = 0;
        if (command & PATTR_COLORKEY) != 0 {
            mat |= MAT_COLORKEY;
        }
        mat | (((command & PATTR_BLEND_MASK) >> 4) & MAT_BLEND_MASK)
    }

    let primitive_type = command & 0xff00_0000u32 as i32;
    let num_primitives = num_primitives.max(0) as usize;
    if num_primitives == 0 {
        return Ok(empty_prepared_figure(PreparedSource::Primitive));
    }

    let vertices_per_primitive = match primitive_type {
        PRIMITIVE_POINTS | PRIMITIVE_POINT_SPRITES => 1,
        PRIMITIVE_LINES => 2,
        PRIMITIVE_TRIANGLES => 3,
        PRIMITIVE_QUADS => 4,
        _ => return Ok(empty_prepared_figure(PreparedSource::Primitive)),
    };
    let vertex_count = num_primitives.saturating_mul(vertices_per_primitive);
    if vertex_offset.saturating_add(vertex_count.saturating_mul(3)) > vertex_coords.len() {
        return Ok(empty_prepared_figure(PreparedSource::Primitive));
    }

    let has_uv = ((command & PDATA_TEXTURE_COORD) != 0 || primitive_type == PRIMITIVE_POINT_SPRITES) && !texture_ref.is_null();
    let textures = if has_uv {
        vec![load_native_texture(jvm, texture_ref).await?]
    } else {
        Vec::new()
    };
    if has_uv && textures.first().is_none_or(|texture| texture.is_none()) {
        return Ok(empty_prepared_figure(PreparedSource::Primitive));
    }

    let projection = state.projection;
    let view = state.view;
    let projected = project_primitive_vertices(vertex_coords, vertex_offset, vertex_count, view, &projection);
    let mat = primitive_material(command);
    let color_type = command & PDATA_COLOR_MASK;
    let mut triangles = Vec::with_capacity(num_primitives * 2);

    match primitive_type {
        PRIMITIVE_POINTS => {
            if color_type == 0 {
                return Ok(empty_prepared_figure(PreparedSource::Primitive));
            }
            for primitive in 0..num_primitives {
                let Some(point) = projected.get(primitive).copied() else {
                    continue;
                };
                if projection.perspective && (point.z < projection.near || point.z > projection.far) {
                    continue;
                }
                let color = primitive_color(colors, color_offset, primitive, color_type, 0x00ff_ffff);
                push_screen_rect(
                    &mut triangles,
                    point.x,
                    point.y,
                    1,
                    1,
                    point.z,
                    mat | MAT_DOUBLE_FACE,
                    None,
                    color,
                    (0, 0, 0, 0),
                );
            }
        }
        PRIMITIVE_LINES => {
            if color_type == 0 {
                return Ok(empty_prepared_figure(PreparedSource::Primitive));
            }
            for primitive in 0..num_primitives {
                let base = primitive * 2;
                let Some(a) = projected.get(base).copied() else {
                    continue;
                };
                let Some(b) = projected.get(base + 1).copied() else {
                    continue;
                };
                if projection.perspective && ((a.z < projection.near && b.z < projection.near) || (a.z > projection.far && b.z > projection.far)) {
                    continue;
                }
                let color = primitive_color(colors, color_offset, primitive, color_type, 0x00ff_ffff);
                push_screen_line(&mut triangles, a, b, mat | MAT_DOUBLE_FACE, color);
            }
        }
        PRIMITIVE_POINT_SPRITES => {
            let mode = command & PDATA_SPRITE_PARAMS_MASK;
            if mode == 0 || textures.first().is_none_or(|texture| texture.is_none()) {
                return Ok(empty_prepared_figure(PreparedSource::Primitive));
            }

            let mut command_params = [0; 8];
            if mode == PDATA_POINT_SPRITE_PARAMS_PER_CMD {
                for (dst, src) in command_params
                    .iter_mut()
                    .zip(texture_coords.get(texture_offset..texture_offset + 8).unwrap_or(&[]))
                {
                    *dst = *src;
                }
            }

            for primitive in 0..num_primitives {
                let Some(center) = projected.get(primitive).copied() else {
                    continue;
                };
                if projection.perspective && (center.z < projection.near || center.z > projection.far) {
                    continue;
                }

                let params = if mode == PDATA_POINT_SPRITE_PARAMS_PER_CMD {
                    command_params
                } else {
                    let mut values = [0; 8];
                    let offset = texture_offset + primitive * 8;
                    for (dst, src) in values.iter_mut().zip(texture_coords.get(offset..offset + 8).unwrap_or(&[])) {
                        *dst = *src;
                    }
                    values
                };

                let [sprite_w, sprite_h, angle, u0, v0, u1, v1, flags] = params;
                if sprite_w <= 0 || sprite_h <= 0 || u1 <= u0 || v1 <= v0 {
                    continue;
                }
                let (width_x, width_y, height_x, height_y) = projected_sprite_axes(sprite_w, sprite_h, flags, center.z, &projection);
                if width_x <= 0 || width_y <= 0 || height_x <= 0 || height_y <= 0 {
                    continue;
                }
                push_screen_sprite(
                    &mut triangles,
                    center.x,
                    center.y,
                    center.z,
                    width_x,
                    width_y,
                    height_x,
                    height_y,
                    angle,
                    mat | MAT_DOUBLE_FACE,
                    Some(0),
                    (u0, v0, u1, v1),
                );
            }
        }
        PRIMITIVE_TRIANGLES | PRIMITIVE_QUADS => {
            let is_quad = primitive_type == PRIMITIVE_QUADS;
            for primitive in 0..num_primitives {
                let base = primitive * vertices_per_primitive;
                let color = primitive_color(colors, color_offset, primitive, color_type, 0x00d0_d0d0);
                let texture = has_uv.then_some(0);
                let uv = |vertex: usize| -> (i32, i32) {
                    if !has_uv {
                        return (0, 0);
                    }
                    let offset = texture_offset + (base + vertex) * 2;
                    (
                        texture_coords.get(offset).copied().unwrap_or(0),
                        texture_coords.get(offset + 1).copied().unwrap_or(0),
                    )
                };

                let (u0, v0) = uv(0);
                let (u1, v1) = uv(1);
                let (u2, v2) = uv(2);
                push_render_tri(
                    &projected,
                    &mut triangles,
                    mat,
                    texture,
                    color,
                    [(base, u0, v0), (base + 1, u1, v1), (base + 2, u2, v2)],
                );

                if is_quad {
                    let (u3, v3) = uv(3);
                    push_render_tri(
                        &projected,
                        &mut triangles,
                        mat,
                        texture,
                        color,
                        [(base + 2, u2, v2), (base + 1, u1, v1), (base + 3, u3, v3)],
                    );
                }
            }
        }
        _ => {}
    }

    Ok(PreparedFigure {
        textures,
        triangles,
        effect_transparency: state.effect_transparency,
        clip: None,
        source: PreparedSource::Primitive,
        prepare_ms: 0,
    })
}

fn primitive_color(colors: &[i32], color_offset: usize, primitive: usize, color_type: i32, default: i32) -> i32 {
    const PDATA_COLOR_PER_COMMAND: i32 = 0x0400;
    const PDATA_COLOR_PER_FACE: i32 = 0x0800;

    let color = match color_type {
        PDATA_COLOR_PER_COMMAND => colors.get(color_offset).copied().unwrap_or(default),
        PDATA_COLOR_PER_FACE => colors.get(color_offset + primitive).copied().unwrap_or(default),
        _ => default,
    };
    argb_color(color)
}

fn argb_color(color: i32) -> i32 {
    if (color as u32 >> 24) == 0 {
        (0xff00_0000u32 as i32) | (color & 0x00ff_ffff)
    } else {
        color
    }
}

type ScreenTri = super::raster::RenderTri;

#[allow(clippy::too_many_arguments)]
fn push_screen_rect(
    triangles: &mut Vec<ScreenTri>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    z: i32,
    mat: i32,
    texture: Option<usize>,
    color: i32,
    uv: (i32, i32, i32, i32),
) {
    let x0 = x;
    let y0 = y;
    let x1 = x + width.max(1);
    let y1 = y + height.max(1);
    push_screen_quad(
        triangles,
        [(x0, y0, uv.0, uv.1), (x1, y0, uv.2, uv.1), (x1, y1, uv.2, uv.3), (x0, y1, uv.0, uv.3)],
        z,
        mat,
        texture,
        color,
    );
}

fn push_screen_line(triangles: &mut Vec<ScreenTri>, a: ProjectedVertex, b: ProjectedVertex, mat: i32, color: i32) {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let (ox, oy) = if dx.abs() >= dy.abs() { (0, 1) } else { (1, 0) };
    let z = (a.z + b.z) / 2;
    push_screen_quad(
        triangles,
        [(a.x, a.y, 0, 0), (b.x, b.y, 0, 0), (b.x + ox, b.y + oy, 0, 0), (a.x + ox, a.y + oy, 0, 0)],
        z,
        mat,
        None,
        color,
    );
}

#[allow(clippy::too_many_arguments)]
fn push_screen_sprite(
    triangles: &mut Vec<ScreenTri>,
    x: i32,
    y: i32,
    z: i32,
    width_x: i32,
    width_y: i32,
    height_x: i32,
    height_y: i32,
    angle: i32,
    mat: i32,
    texture: Option<usize>,
    uv: (i32, i32, i32, i32),
) {
    let (sin, cos) = sin_cos_mc(angle);
    let x1 = x - ((cos * width_x) >> 13) + ((sin * height_x) >> 13);
    let y1 = y - ((cos * height_y) >> 13) - ((sin * width_y) >> 13);
    let x2 = x + ((cos * width_x) >> 13) + ((sin * height_x) >> 13);
    let y2 = y - ((cos * height_y) >> 13) + ((sin * width_y) >> 13);
    let x3 = x + ((cos * width_x) >> 13) - ((sin * height_x) >> 13);
    let y3 = y + ((cos * height_y) >> 13) + ((sin * width_y) >> 13);
    let x4 = x - ((cos * width_x) >> 13) - ((sin * height_x) >> 13);
    let y4 = y + ((cos * height_y) >> 13) - ((sin * width_y) >> 13);

    push_screen_quad(
        triangles,
        [(x1, y1, uv.0, uv.1), (x2, y2, uv.2, uv.1), (x3, y3, uv.2, uv.3), (x4, y4, uv.0, uv.3)],
        z,
        mat,
        texture,
        0xffd0_d0d0u32 as i32,
    );
}

fn push_screen_quad(triangles: &mut Vec<ScreenTri>, vertices: [(i32, i32, i32, i32); 4], z: i32, mat: i32, texture: Option<usize>, color: i32) {
    let projected = [
        ProjectedVertex {
            x: vertices[0].0,
            y: vertices[0].1,
            z,
        },
        ProjectedVertex {
            x: vertices[1].0,
            y: vertices[1].1,
            z,
        },
        ProjectedVertex {
            x: vertices[2].0,
            y: vertices[2].1,
            z,
        },
        ProjectedVertex {
            x: vertices[3].0,
            y: vertices[3].1,
            z,
        },
    ];
    push_render_tri(
        &projected,
        triangles,
        mat,
        texture,
        color,
        [
            (0, vertices[0].2, vertices[0].3),
            (1, vertices[1].2, vertices[1].3),
            (2, vertices[2].2, vertices[2].3),
        ],
    );
    push_render_tri(
        &projected,
        triangles,
        mat,
        texture,
        color,
        [
            (0, vertices[0].2, vertices[0].3),
            (2, vertices[2].2, vertices[2].3),
            (3, vertices[3].2, vertices[3].3),
        ],
    );
}

fn projected_sprite_axes(sprite_w: i32, sprite_h: i32, flags: i32, z: i32, projection: &ProjectionParams) -> (i32, i32, i32, i32) {
    const POINT_SPRITE_PIXEL_SIZE: i32 = 1;
    const POINT_SPRITE_NO_PERS: i32 = 2;

    let size_in_pixels = (flags & POINT_SPRITE_PIXEL_SIZE) != 0;
    let no_perspective = (flags & POINT_SPRITE_NO_PERS) != 0;
    if projection.perspective {
        if size_in_pixels {
            if no_perspective {
                (sprite_w, sprite_w, sprite_h, sprite_h)
            } else {
                let z = z.max(1);
                (
                    (projection.near * sprite_w) / z,
                    (projection.near * sprite_w) / z,
                    (projection.near * sprite_h) / z,
                    (projection.near * sprite_h) / z,
                )
            }
        } else if no_perspective {
            let near = projection.near.max(1);
            (
                (sprite_w * projection.scale_x) / near,
                (sprite_w * projection.scale_y) / near,
                (sprite_h * projection.scale_x) / near,
                (sprite_h * projection.scale_y) / near,
            )
        } else {
            let z = z.max(1);
            (
                (sprite_w * projection.scale_x) / z,
                (sprite_w * projection.scale_y) / z,
                (sprite_h * projection.scale_x) / z,
                (sprite_h * projection.scale_y) / z,
            )
        }
    } else if size_in_pixels {
        (sprite_w, sprite_w, sprite_h, sprite_h)
    } else {
        (
            (sprite_w * projection.scale_x) >> 12,
            (sprite_w * projection.scale_y) >> 12,
            (sprite_h * projection.scale_x) >> 12,
            (sprite_h * projection.scale_y) >> 12,
        )
    }
}

fn project_primitive_vertices(
    vertex_coords: &[i32],
    vertex_offset: usize,
    vertex_count: usize,
    matrix: [i32; 12],
    projection: &ProjectionParams,
) -> Vec<ProjectedVertex> {
    let mut projected = Vec::with_capacity(vertex_count);
    for vertex in 0..vertex_count {
        let base = vertex_offset + vertex * 3;
        let vx = vertex_coords[base];
        let vy = vertex_coords[base + 1];
        let vz = vertex_coords[base + 2];
        let tx = fixed_mul3(matrix[0], vx, matrix[1], vy, matrix[2], vz) + matrix[3];
        let ty = fixed_mul3(matrix[4], vx, matrix[5], vy, matrix[6], vz) + matrix[7];
        let tz = fixed_mul3(matrix[8], vx, matrix[9], vy, matrix[10], vz) + matrix[11];

        let (sx, sy) = if projection.perspective {
            let z = tz.max(projection.near);
            (
                ((tx as i64 * projection.scale_x as i64) / z as i64) as i32 + projection.center_x,
                ((ty as i64 * projection.scale_y as i64) / z as i64) as i32 + projection.center_y,
            )
        } else {
            (
                (((tx as i64 * projection.scale_x as i64) + 2048) >> 12) as i32 + projection.center_x,
                (((ty as i64 * projection.scale_y as i64) + 2048) >> 12) as i32 + projection.center_y,
            )
        };

        projected.push(ProjectedVertex {
            x: sx.clamp(-32768, 32767),
            y: sy.clamp(-32768, 32767),
            z: tz,
        });
    }
    projected
}

pub(super) async fn draw_scene_triangles(
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
            if gpu_scene_enabled()
                && publish_gpu_image_scene(
                    &target_pixels_array,
                    target_width,
                    target_height,
                    (clip_x, clip_y, clip_w, clip_h),
                    textures,
                    triangles,
                )
            {
                stats.gpu_direct = true;
                stats.rasterized_triangles = triangles.len();
                stats.draw_calls = 1;
                return Ok(stats);
            }
            stats.gpu_reject = gpu_scene_rejection_reason(target_width, target_height, (clip_x, clip_y, clip_w, clip_h), triangles);
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

async fn store_target_pixels(
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

fn update_dirty_bounds(bounds: &mut Option<(i32, i32, i32, i32)>, changed: Option<(i32, i32, i32, i32)>) {
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

fn triangle_bounds(
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

fn can_batch_screen_triangles(triangles: &[SceneTri]) -> bool {
    triangles.iter().all(|scene_tri| !scene_tri.effect_transparency)
}

#[allow(clippy::too_many_arguments)]
async fn draw_batched_screen_triangles(
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
            false,
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
async fn draw_screen_triangles_individually(
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

fn effective_clip(base: (i32, i32, i32, i32), override_clip: Option<(i32, i32, i32, i32)>) -> (i32, i32, i32, i32) {
    override_clip.unwrap_or(base)
}

async fn graphics_clip(jvm: &Jvm, context: &mut RuntimeContext, graphics: &ClassInstanceRef<Graphics>) -> Result<(i32, i32, i32, i32)> {
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

async fn projection_params(
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

async fn layout_matrix(jvm: &Jvm, layout: &ClassInstanceRef<FigureLayout>) -> Result<[i32; 12]> {
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
