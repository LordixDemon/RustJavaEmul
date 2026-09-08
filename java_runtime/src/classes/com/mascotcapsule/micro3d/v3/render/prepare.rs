use super::super::{
    Effect3D, Figure, FigureLayout,
    constants::{BONE_STRIDE, MAT_BLEND_MASK, MAT_COLORKEY, MAT_DOUBLE_FACE, QUAD_C_STRIDE, QUAD_T_STRIDE, TRI_C_STRIDE, TRI_T_STRIDE},
    math::{fixed_mul3, sin_cos_mc},
    raster::{PreparedFigure, PreparedSource, push_render_tri},
    scene::{
        ProjectedVertex, ProjectionParams, apply_active_material_mask, collect_render_triangles, mark_implicit_indexed_color_keys,
        project_figure_vertices, projected_bounds, transformed_bounds, triangle_in_depth_range,
    },
    storage::{load_figure_textures, load_native_texture, load_runtime_figure},
    texture::Texture,
};
#[allow(unused_imports)]
use super::*;
use crate::RuntimeContext;
#[allow(unused_imports)]
use alloc::{vec, vec::Vec};
use jvm::{ClassInstanceRef, Jvm, Result};

pub(crate) async fn prepare_native_figure(
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

pub(crate) fn empty_prepared_figure(source: PreparedSource) -> PreparedFigure {
    PreparedFigure {
        textures: Vec::new(),
        triangles: Vec::new(),
        effect_transparency: false,
        clip: None,
        source,
        prepare_ms: 0,
    }
}

pub(crate) struct PrimitiveRenderState {
    pub(super) projection: ProjectionParams,
    pub(super) view: [i32; 12],
    pub(super) effect_transparency: bool,
}

pub(crate) async fn primitive_render_state(
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
pub(crate) async fn prepare_primitives(
    jvm: &Jvm,
    context: &mut RuntimeContext,
    texture_ref: &ClassInstanceRef<Texture>,
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
pub(crate) async fn prepare_primitives_with_state(
    jvm: &Jvm,
    texture_ref: &ClassInstanceRef<Texture>,
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
    pub const PRIMITIVE_POINTS: i32 = 0x0100_0000;
    pub const PRIMITIVE_LINES: i32 = 0x0200_0000;
    pub const PRIMITIVE_TRIANGLES: i32 = 0x0300_0000;
    pub const PRIMITIVE_QUADS: i32 = 0x0400_0000;
    pub const PRIMITIVE_POINT_SPRITES: i32 = 0x0500_0000;
    pub const PATTR_COLORKEY: i32 = 0x10;
    pub const PATTR_BLEND_MASK: i32 = 0x60;
    pub const PDATA_COLOR_MASK: i32 = 0x0c00;
    pub const PDATA_TEXTURE_COORD: i32 = 0x3000;
    pub const PDATA_SPRITE_PARAMS_MASK: i32 = 0x3000;
    pub const PDATA_POINT_SPRITE_PARAMS_PER_CMD: i32 = 0x1000;

    pub(super) fn primitive_material(command: i32) -> i32 {
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

pub(crate) fn primitive_color(colors: &[i32], color_offset: usize, primitive: usize, color_type: i32, default: i32) -> i32 {
    pub const PDATA_COLOR_PER_COMMAND: i32 = 0x0400;
    pub const PDATA_COLOR_PER_FACE: i32 = 0x0800;

    let color = match color_type {
        PDATA_COLOR_PER_COMMAND => colors.get(color_offset).copied().unwrap_or(default),
        PDATA_COLOR_PER_FACE => colors.get(color_offset + primitive).copied().unwrap_or(default),
        _ => default,
    };
    argb_color(color)
}

pub(crate) fn argb_color(color: i32) -> i32 {
    if (color as u32 >> 24) == 0 {
        (0xff00_0000u32 as i32) | (color & 0x00ff_ffff)
    } else {
        color
    }
}

pub(crate) type ScreenTri = super::super::raster::RenderTri;

#[allow(clippy::too_many_arguments)]
pub(crate) fn push_screen_rect(
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

pub(crate) fn push_screen_line(triangles: &mut Vec<ScreenTri>, a: ProjectedVertex, b: ProjectedVertex, mat: i32, color: i32) {
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
pub(crate) fn push_screen_sprite(
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

pub(crate) fn push_screen_quad(
    triangles: &mut Vec<ScreenTri>,
    vertices: [(i32, i32, i32, i32); 4],
    z: i32,
    mat: i32,
    texture: Option<usize>,
    color: i32,
) {
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

pub(crate) fn projected_sprite_axes(sprite_w: i32, sprite_h: i32, flags: i32, z: i32, projection: &ProjectionParams) -> (i32, i32, i32, i32) {
    pub const POINT_SPRITE_PIXEL_SIZE: i32 = 1;
    pub const POINT_SPRITE_NO_PERS: i32 = 2;

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

pub(crate) fn project_primitive_vertices(
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
