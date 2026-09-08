use alloc::{sync::Arc, vec, vec::Vec};

use super::{
    constants::{
        BONE_STRIDE, IMPLICIT_COLOR_KEY_EDGE_ZERO_RATIO_NUMERATOR, IMPLICIT_COLOR_KEY_MAX_TEXELS, IMPLICIT_COLOR_KEY_MIN_OPAQUE_TEXELS,
        IMPLICIT_COLOR_KEY_MIN_ZERO_RATIO_NUMERATOR, IMPLICIT_COLOR_KEY_ZERO_RATIO_DENOMINATOR, IMPLICIT_COLOR_KEY_ZERO_RATIO_NUMERATOR,
        MAT_BLEND_MASK, MAT_COLORKEY, MAT_LIGHTING, MAT_MASK, MAT_SPECULAR, PATTERN_STRIDE, QUAD_C_STRIDE, QUAD_T_STRIDE, TRI_C_STRIDE, TRI_T_STRIDE,
    },
    math::{fixed_mul3, identity_matrix, mul_matrix},
    mbac::unpack_uv,
    raster::{RenderTri, push_render_tri},
    texture::NativeTexture,
};

pub(super) struct RuntimeFigure {
    pub(super) vertex_count: usize,
    pub(super) vertices: Arc<Vec<i16>>,
    pub(super) poly_c3: Arc<Vec<i16>>,
    pub(super) poly_c4: Arc<Vec<i16>>,
    pub(super) poly_t3: Arc<Vec<i16>>,
    pub(super) poly_t4: Arc<Vec<i16>>,
    pub(super) colors: Arc<Vec<i32>>,
    pub(super) patterns: Arc<Vec<i32>>,
    pub(super) pattern_count: usize,
    pub(super) pattern_slots: usize,
    pub(super) bones: Arc<Vec<i32>>,
    pub(super) posture_bones: Vec<i32>,
    pub(super) selected_pattern: i32,
    pub(super) texture_index: i32,
}

#[derive(Clone, Copy, Default)]
pub(super) struct ProjectedVertex {
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) z: i32,
}

#[derive(Clone, Copy)]
pub(super) struct ProjectionParams {
    pub(super) perspective: bool,
    pub(super) near: i32,
    pub(super) far: i32,
    pub(super) scale_x: i32,
    pub(super) scale_y: i32,
    pub(super) center_x: i32,
    pub(super) center_y: i32,
}

pub(super) fn triangle_in_depth_range(tri: &RenderTri, projection: &ProjectionParams) -> bool {
    !tri.vertices.iter().all(|vertex| vertex.z < projection.near) && !tri.vertices.iter().all(|vertex| vertex.z > projection.far)
}

pub(super) fn projected_bounds(projected: &[ProjectedVertex]) -> (i32, i32, i32, i32, i32, i32) {
    let Some(first) = projected.first() else {
        return (0, 0, 0, 0, 0, 0);
    };

    let mut min_x = first.x;
    let mut min_y = first.y;
    let mut max_x = first.x;
    let mut max_y = first.y;
    let mut min_z = first.z;
    let mut max_z = first.z;
    for vertex in projected.iter().skip(1) {
        min_x = min_x.min(vertex.x);
        min_y = min_y.min(vertex.y);
        max_x = max_x.max(vertex.x);
        max_y = max_y.max(vertex.y);
        min_z = min_z.min(vertex.z);
        max_z = max_z.max(vertex.z);
    }

    (min_x, min_y, max_x, max_y, min_z, max_z)
}

pub(super) fn project_figure_vertices(figure: &RuntimeFigure, view: [i32; 12], projection: &ProjectionParams) -> Vec<ProjectedVertex> {
    let mut projected = vec![ProjectedVertex::default(); figure.vertex_count];
    if figure.bones.len() >= BONE_STRIDE {
        let bone_count = figure.bones.len() / BONE_STRIDE;
        let mut global = vec![identity_matrix(); bone_count];
        for bone_index in 0..bone_count {
            let offset = bone_index * BONE_STRIDE;
            let start = figure.bones[offset].max(0) as usize;
            let count = figure.bones[offset + 1].max(0) as usize;
            let parent = figure.bones[offset + 2];
            let local = runtime_bone_local_matrix(figure, bone_index, offset);
            let matrix = if parent >= 0 {
                let parent = parent as usize;
                if parent < global.len() {
                    mul_matrix(global[parent], local)
                } else {
                    mul_matrix(view, local)
                }
            } else {
                mul_matrix(view, local)
            };
            global[bone_index] = matrix;
            project_vertex_range(figure, start, count, matrix, projection, &mut projected);
        }
    } else {
        project_vertex_range(figure, 0, figure.vertex_count, view, projection, &mut projected);
    }

    projected
}

pub(super) fn transformed_bounds(figure: &RuntimeFigure, view: [i32; 12]) -> (i32, i32, i32, i32, i32, i32) {
    let mut bounds: Option<(i32, i32, i32, i32, i32, i32)> = None;
    if figure.bones.len() >= BONE_STRIDE {
        let bone_count = figure.bones.len() / BONE_STRIDE;
        let mut global = vec![identity_matrix(); bone_count];
        for bone_index in 0..bone_count {
            let offset = bone_index * BONE_STRIDE;
            let start = figure.bones[offset].max(0) as usize;
            let count = figure.bones[offset + 1].max(0) as usize;
            let parent = figure.bones[offset + 2];
            let local = runtime_bone_local_matrix(figure, bone_index, offset);
            let matrix = if parent >= 0 {
                let parent = parent as usize;
                if parent < global.len() {
                    mul_matrix(global[parent], local)
                } else {
                    mul_matrix(view, local)
                }
            } else {
                mul_matrix(view, local)
            };
            global[bone_index] = matrix;
            update_transform_bounds(figure, start, count, matrix, &mut bounds);
        }
    } else {
        update_transform_bounds(figure, 0, figure.vertex_count, view, &mut bounds);
    }

    bounds.unwrap_or((0, 0, 0, 0, 0, 0))
}

fn update_transform_bounds(
    figure: &RuntimeFigure,
    start: usize,
    count: usize,
    matrix: [i32; 12],
    bounds: &mut Option<(i32, i32, i32, i32, i32, i32)>,
) {
    let end = (start + count).min(figure.vertex_count);
    for index in start..end {
        let base = index * 3;
        if base + 2 >= figure.vertices.len() {
            break;
        }
        let vx = figure.vertices[base] as i32;
        let vy = figure.vertices[base + 1] as i32;
        let vz = figure.vertices[base + 2] as i32;
        let tx = fixed_mul3(matrix[0], vx, matrix[1], vy, matrix[2], vz) + matrix[3];
        let ty = fixed_mul3(matrix[4], vx, matrix[5], vy, matrix[6], vz) + matrix[7];
        let tz = fixed_mul3(matrix[8], vx, matrix[9], vy, matrix[10], vz) + matrix[11];
        match bounds {
            Some((min_x, min_y, min_z, max_x, max_y, max_z)) => {
                *min_x = (*min_x).min(tx);
                *min_y = (*min_y).min(ty);
                *min_z = (*min_z).min(tz);
                *max_x = (*max_x).max(tx);
                *max_y = (*max_y).max(ty);
                *max_z = (*max_z).max(tz);
            }
            None => *bounds = Some((tx, ty, tz, tx, ty, tz)),
        }
    }
}

fn runtime_bone_local_matrix(figure: &RuntimeFigure, bone_index: usize, offset: usize) -> [i32; 12] {
    let posture_offset = bone_index * 12;
    if posture_offset + 12 <= figure.posture_bones.len() {
        let mut matrix = [0; 12];
        matrix.copy_from_slice(&figure.posture_bones[posture_offset..posture_offset + 12]);
        return matrix;
    }

    let mut matrix = [0; 12];
    matrix.copy_from_slice(&figure.bones[offset + 3..offset + 15]);
    matrix
}

fn project_vertex_range(
    figure: &RuntimeFigure,
    start: usize,
    count: usize,
    matrix: [i32; 12],
    projection: &ProjectionParams,
    projected: &mut [ProjectedVertex],
) {
    let end = (start + count).min(figure.vertex_count).min(projected.len());
    for (index, projected_vertex) in projected.iter_mut().enumerate().take(end).skip(start) {
        let base = index * 3;
        if base + 2 >= figure.vertices.len() {
            break;
        }
        let vx = figure.vertices[base] as i32;
        let vy = figure.vertices[base + 1] as i32;
        let vz = figure.vertices[base + 2] as i32;
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

        *projected_vertex = ProjectedVertex {
            x: sx.clamp(-32768, 32767),
            y: sy.clamp(-32768, 32767),
            z: tz,
        };
    }
}

pub(super) fn collect_render_triangles(figure: &RuntimeFigure, projected: &[ProjectedVertex], textures: &[Option<NativeTexture>]) -> Vec<RenderTri> {
    let mut triangles = Vec::new();
    for pattern in 0..figure.pattern_count {
        let pattern_mask = if pattern == 0 { 0 } else { 1i32 << pattern };
        if (pattern_mask & figure.selected_pattern) != pattern_mask {
            continue;
        }

        for slot in 0..figure.pattern_slots {
            let Some([start3, start4, count3, count4]) = pattern_entry(figure, pattern, slot) else {
                continue;
            };

            if slot == 0 {
                submit_color_tris(figure, projected, start3, count3, &mut triangles);
                submit_color_quads(figure, projected, start4, count4, &mut triangles);
            } else {
                let texture_index = if figure.texture_index >= 0 {
                    if slot != 1 {
                        continue;
                    }
                    figure.texture_index as usize
                } else {
                    slot - 1
                };
                if texture_index >= textures.len() || textures[texture_index].is_none() {
                    continue;
                }
                submit_tex_tris(figure, projected, start3, count3, texture_index, &mut triangles);
                submit_tex_quads(figure, projected, start4, count4, texture_index, &mut triangles);
            }
        }
    }
    triangles
}

pub(super) fn mark_implicit_indexed_color_keys(triangles: &mut [RenderTri], textures: &[Option<NativeTexture>]) {
    for tri in triangles {
        tri.implicit_color_key = should_use_implicit_indexed_color_key(tri, textures);
    }
}

pub(super) fn apply_active_material_mask(triangles: &mut [RenderTri], effect_transparency: bool) {
    let mut material_mask = MAT_MASK & !MAT_LIGHTING & !MAT_SPECULAR;
    if !effect_transparency {
        material_mask &= !MAT_BLEND_MASK;
    }

    for tri in triangles {
        tri.mat &= material_mask;
    }
}

fn should_use_implicit_indexed_color_key(tri: &RenderTri, textures: &[Option<NativeTexture>]) -> bool {
    if (tri.mat & MAT_COLORKEY) != 0 || (tri.mat & (MAT_LIGHTING | MAT_BLEND_MASK)) == 0 {
        return false;
    }

    let Some(texture) = tri.texture.and_then(|index| textures.get(index)).and_then(|texture| texture.as_ref()) else {
        return false;
    };
    if texture.width <= 0 || texture.height <= 0 || texture.indices.is_empty() || texture.palette.is_empty() || !is_dark_rgb(texture.palette[0]) {
        return false;
    }

    let min_u = tri.vertices.iter().map(|vertex| vertex.u).min().unwrap_or(0).clamp(0, 255);
    let max_u = tri.vertices.iter().map(|vertex| vertex.u).max().unwrap_or(0).clamp(0, 255);
    let min_v = tri.vertices.iter().map(|vertex| vertex.v).min().unwrap_or(0).clamp(0, 255);
    let max_v = tri.vertices.iter().map(|vertex| vertex.v).max().unwrap_or(0).clamp(0, 255);
    let x0 = uv_to_texel(min_u, texture.width);
    let x1 = uv_to_texel(max_u, texture.width);
    let y0 = uv_to_texel(min_v, texture.height);
    let y1 = uv_to_texel(max_v, texture.height);
    if x0 > x1 || y0 > y1 {
        return false;
    }

    let rect_w = (x1 - x0 + 1) as usize;
    let rect_h = (y1 - y0 + 1) as usize;
    let total = rect_w.saturating_mul(rect_h);
    if total == 0 || total > IMPLICIT_COLOR_KEY_MAX_TEXELS {
        return false;
    }

    let texture_width = texture.width as usize;
    let mut zero_count = 0usize;
    let mut edge_count = 0usize;
    let mut edge_zero_count = 0usize;
    for y in y0 as usize..=y1 as usize {
        let row = y.saturating_mul(texture_width);
        for x in x0 as usize..=x1 as usize {
            let is_zero = texture.indices.get(row + x).copied().unwrap_or(0) == 0;
            if is_zero {
                zero_count += 1;
            }
            if x == x0 as usize || x == x1 as usize || y == y0 as usize || y == y1 as usize {
                edge_count += 1;
                if is_zero {
                    edge_zero_count += 1;
                }
            }
        }
    }

    let opaque_count = total.saturating_sub(zero_count);
    if opaque_count < IMPLICIT_COLOR_KEY_MIN_OPAQUE_TEXELS {
        return false;
    }

    let zero_dominant =
        zero_count.saturating_mul(IMPLICIT_COLOR_KEY_ZERO_RATIO_DENOMINATOR) >= total.saturating_mul(IMPLICIT_COLOR_KEY_ZERO_RATIO_NUMERATOR);
    let edge_background = edge_count > 0
        && zero_count.saturating_mul(IMPLICIT_COLOR_KEY_ZERO_RATIO_DENOMINATOR) >= total.saturating_mul(IMPLICIT_COLOR_KEY_MIN_ZERO_RATIO_NUMERATOR)
        && edge_zero_count.saturating_mul(IMPLICIT_COLOR_KEY_ZERO_RATIO_DENOMINATOR)
            >= edge_count.saturating_mul(IMPLICIT_COLOR_KEY_EDGE_ZERO_RATIO_NUMERATOR);

    zero_dominant || edge_background
}

fn uv_to_texel(uv: i32, size: i32) -> i32 {
    uv.clamp(0, (size - 1).max(0))
}

fn is_dark_rgb(color: i32) -> bool {
    let rgb = color as u32;
    ((rgb >> 16) & 0xff) <= 3 && ((rgb >> 8) & 0xff) <= 3 && (rgb & 0xff) <= 3
}

fn pattern_entry(figure: &RuntimeFigure, pattern: usize, slot: usize) -> Option<[usize; 4]> {
    let base = (pattern * figure.pattern_slots + slot) * PATTERN_STRIDE;
    if base + 3 >= figure.patterns.len() {
        return None;
    }
    Some([
        figure.patterns[base].max(0) as usize,
        figure.patterns[base + 1].max(0) as usize,
        figure.patterns[base + 2].max(0) as usize,
        figure.patterns[base + 3].max(0) as usize,
    ])
}

fn submit_color_tris(figure: &RuntimeFigure, projected: &[ProjectedVertex], start: usize, count: usize, triangles: &mut Vec<RenderTri>) {
    let stride = TRI_C_STRIDE;
    let begin = start.saturating_mul(stride);
    let end = (begin + count.saturating_mul(stride)).min(figure.poly_c3.len());
    let mut offset = begin;
    while offset + stride <= end {
        let mat = figure.poly_c3[offset] as i32;
        let color_idx = (figure.poly_c3[offset + 1] as usize).min(figure.colors.len().saturating_sub(1));
        let color = figure.colors.get(color_idx).copied().unwrap_or(0xffd0_d0d0u32 as i32);
        let v0 = figure.poly_c3[offset + 2] as u16 as usize;
        let v1 = figure.poly_c3[offset + 3] as u16 as usize;
        let v2 = figure.poly_c3[offset + 4] as u16 as usize;
        push_render_tri(projected, triangles, mat, None, color, [(v0, 0, 0), (v1, 0, 0), (v2, 0, 0)]);
        offset += stride;
    }
}

fn submit_color_quads(figure: &RuntimeFigure, projected: &[ProjectedVertex], start: usize, count: usize, triangles: &mut Vec<RenderTri>) {
    let stride = QUAD_C_STRIDE;
    let begin = start.saturating_mul(stride);
    let end = (begin + count.saturating_mul(stride)).min(figure.poly_c4.len());
    let mut offset = begin;
    while offset + stride <= end {
        let mat = figure.poly_c4[offset] as i32;
        let color_idx = (figure.poly_c4[offset + 1] as usize).min(figure.colors.len().saturating_sub(1));
        let color = figure.colors.get(color_idx).copied().unwrap_or(0xffd0_d0d0u32 as i32);
        let v0 = figure.poly_c4[offset + 2] as u16 as usize;
        let v1 = figure.poly_c4[offset + 3] as u16 as usize;
        let v2 = figure.poly_c4[offset + 4] as u16 as usize;
        let v3 = figure.poly_c4[offset + 5] as u16 as usize;
        push_render_tri(projected, triangles, mat, None, color, [(v0, 0, 0), (v1, 0, 0), (v2, 0, 0)]);
        push_render_tri(projected, triangles, mat, None, color, [(v2, 0, 0), (v1, 0, 0), (v3, 0, 0)]);
        offset += stride;
    }
}

fn submit_tex_tris(
    figure: &RuntimeFigure,
    projected: &[ProjectedVertex],
    start: usize,
    count: usize,
    texture: usize,
    triangles: &mut Vec<RenderTri>,
) {
    let stride = TRI_T_STRIDE;
    let begin = start.saturating_mul(stride);
    let end = (begin + count.saturating_mul(stride)).min(figure.poly_t3.len());
    let mut offset = begin;
    while offset + stride <= end {
        let mat = figure.poly_t3[offset] as i32;
        let v0 = figure.poly_t3[offset + 1] as u16 as usize;
        let v1 = figure.poly_t3[offset + 2] as u16 as usize;
        let v2 = figure.poly_t3[offset + 3] as u16 as usize;
        let (u0, v0t) = unpack_uv(figure.poly_t3[offset + 4]);
        let (u1, v1t) = unpack_uv(figure.poly_t3[offset + 5]);
        let (u2, v2t) = unpack_uv(figure.poly_t3[offset + 6]);
        push_render_tri(
            projected,
            triangles,
            mat,
            Some(texture),
            0xffd0_d0d0u32 as i32,
            [(v0, u0, v0t), (v1, u1, v1t), (v2, u2, v2t)],
        );
        offset += stride;
    }
}

fn submit_tex_quads(
    figure: &RuntimeFigure,
    projected: &[ProjectedVertex],
    start: usize,
    count: usize,
    texture: usize,
    triangles: &mut Vec<RenderTri>,
) {
    let stride = QUAD_T_STRIDE;
    let begin = start.saturating_mul(stride);
    let end = (begin + count.saturating_mul(stride)).min(figure.poly_t4.len());
    let mut offset = begin;
    while offset + stride <= end {
        let mat = figure.poly_t4[offset] as i32;
        let v0 = figure.poly_t4[offset + 1] as u16 as usize;
        let v1 = figure.poly_t4[offset + 2] as u16 as usize;
        let v2 = figure.poly_t4[offset + 3] as u16 as usize;
        let v3 = figure.poly_t4[offset + 4] as u16 as usize;
        let (u0, v0t) = unpack_uv(figure.poly_t4[offset + 5]);
        let (u1, v1t) = unpack_uv(figure.poly_t4[offset + 6]);
        let (u2, v2t) = unpack_uv(figure.poly_t4[offset + 7]);
        let (u3, v3t) = unpack_uv(figure.poly_t4[offset + 8]);
        push_render_tri(
            projected,
            triangles,
            mat,
            Some(texture),
            0xffd0_d0d0u32 as i32,
            [(v0, u0, v0t), (v1, u1, v1t), (v2, u2, v2t)],
        );
        push_render_tri(
            projected,
            triangles,
            mat,
            Some(texture),
            0xffd0_d0d0u32 as i32,
            [(v2, u2, v2t), (v1, u1, v1t), (v3, u3, v3t)],
        );
        offset += stride;
    }
}
