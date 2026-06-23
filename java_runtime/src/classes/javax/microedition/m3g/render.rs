use alloc::{sync::Arc, vec::Vec};

use jvm::ClassInstanceRef;

use super::{Background, CompositingMode, Image2D, IndexBuffer, PolygonMode, Texture2D, math::transform_point};
#[derive(Clone, Copy)]
pub(super) struct M3gDrawVertex {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) z: f32,
    pub(super) u: f32,
    pub(super) v: f32,
    pub(super) color: i32,
}

#[derive(Clone, Copy)]
pub(super) struct M3gCameraVertex {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) depth: f32,
    pub(super) u: f32,
    pub(super) v: f32,
    pub(super) color: i32,
}

pub(super) struct M3gTexture {
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) pixels: Arc<Vec<i32>>,
    pub(super) format: i32,
    pub(super) blend_color: i32,
    pub(super) blending: i32,
    pub(super) wrap_s: i32,
    pub(super) wrap_t: i32,
    pub(super) image_filter: i32,
    pub(super) transform: [f32; 16],
    pub(super) transform_identity: bool,
}

pub(super) struct M3gAppearanceState {
    pub(super) texture: Option<M3gTexture>,
    pub(super) base_color: i32,
    pub(super) layer: i32,
    pub(super) color_write: bool,
    pub(super) alpha_write: bool,
    pub(super) depth_test: bool,
    pub(super) depth_write: bool,
    pub(super) depth_offset_factor: f32,
    pub(super) depth_offset_units: f32,
    pub(super) alpha_threshold: f32,
    pub(super) blending: i32,
    pub(super) culling: i32,
    pub(super) winding: i32,
    pub(super) perspective_correction: bool,
}

#[derive(Clone, Copy, Default)]
pub(super) struct M3gSceneLighting {
    pub(super) ambient_r: f32,
    pub(super) ambient_g: f32,
    pub(super) ambient_b: f32,
    pub(super) ambient_lights: usize,
}

pub(super) struct M3gBackgroundFrame {
    pub(super) pixels: Vec<i32>,
    pub(super) color_clear: bool,
    pub(super) depth_clear: bool,
    pub(super) process_alpha: bool,
    pub(super) submit_when_empty: bool,
}

pub(super) struct M3gSubmeshDraw {
    pub(super) sort_key: i32,
    pub(super) sequence: usize,
    pub(super) index_buffer: ClassInstanceRef<IndexBuffer>,
    pub(super) appearance: M3gAppearanceState,
}

#[derive(Default)]
pub(super) struct M3gRenderStats {
    pub(super) nodes: usize,
    pub(super) meshes: usize,
    pub(super) rendered_meshes: usize,
    pub(super) vertices: usize,
    pub(super) projected_vertices: usize,
    pub(super) candidate_triangles: usize,
    pub(super) triangles: usize,
    pub(super) pixels: usize,
    pub(super) dirty: M3gDirtyRect,
    pub(super) mesh_vertex_ms: u64,
    pub(super) mesh_appearance_ms: u64,
    pub(super) mesh_raster_ms: u64,
}

#[derive(Default)]
pub(super) struct M3gMeshRenderStats {
    pub(super) submeshes: usize,
    pub(super) vertices: usize,
    pub(super) projected_vertices: usize,
    pub(super) candidate_triangles: usize,
    pub(super) rasterized_triangles: usize,
    pub(super) textured_submeshes: usize,
    pub(super) pixels: usize,
    pub(super) dirty: M3gDirtyRect,
    pub(super) vertex_ms: u64,
    pub(super) appearance_ms: u64,
    pub(super) raster_ms: u64,
}

#[derive(Clone, Copy)]
pub(super) struct M3gDirtyRect {
    pub(super) min_x: i32,
    pub(super) min_y: i32,
    pub(super) max_x: i32,
    pub(super) max_y: i32,
}

impl Default for M3gDirtyRect {
    fn default() -> Self {
        Self {
            min_x: i32::MAX,
            min_y: i32::MAX,
            max_x: i32::MIN,
            max_y: i32::MIN,
        }
    }
}

impl M3gDirtyRect {
    pub(super) fn is_empty(self) -> bool {
        self.min_x > self.max_x || self.min_y > self.max_y
    }

    pub(super) fn include_rect(&mut self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
        self.min_x = self.min_x.min(min_x);
        self.min_y = self.min_y.min(min_y);
        self.max_x = self.max_x.max(max_x);
        self.max_y = self.max_y.max(max_y);
    }

    pub(super) fn include_dirty(&mut self, other: Self) {
        if !other.is_empty() {
            self.include_rect(other.min_x, other.min_y, other.max_x, other.max_y);
        }
    }

    pub(super) fn bounds(self) -> Option<(i32, i32, i32, i32)> {
        if self.is_empty() {
            None
        } else {
            Some((self.min_x, self.min_y, self.max_x - self.min_x + 1, self.max_y - self.min_y + 1))
        }
    }
}

pub(super) fn camera_vertex(position: [f32; 3], uv: [f32; 2], color: i32, model_view: [f32; 16]) -> Option<M3gCameraVertex> {
    let camera = transform_point(model_view, position);
    if !camera[0].is_finite() || !camera[1].is_finite() || !camera[2].is_finite() {
        return None;
    }

    Some(M3gCameraVertex {
        x: camera[0],
        y: camera[1],
        depth: -camera[2],
        u: uv[0],
        v: uv[1],
        color,
    })
}

pub(super) fn clip_triangle_near(v0: M3gCameraVertex, v1: M3gCameraVertex, v2: M3gCameraVertex, near: f32) -> Vec<M3gCameraVertex> {
    let input = [v0, v1, v2];
    let mut output = Vec::with_capacity(4);

    let mut previous = input[2];
    let mut previous_inside = previous.depth >= near;
    for current in input {
        let current_inside = current.depth >= near;
        if current_inside != previous_inside {
            output.push(intersect_near(previous, current, near));
        }
        if current_inside {
            output.push(current);
        }
        previous = current;
        previous_inside = current_inside;
    }

    output
}

fn intersect_near(from: M3gCameraVertex, to: M3gCameraVertex, near: f32) -> M3gCameraVertex {
    let denom = to.depth - from.depth;
    let t = if denom.abs() <= f32::EPSILON {
        0.0
    } else {
        ((near - from.depth) / denom).clamp(0.0, 1.0)
    };

    M3gCameraVertex {
        x: lerp_f32(from.x, to.x, t),
        y: lerp_f32(from.y, to.y, t),
        depth: near,
        u: lerp_f32(from.u, to.u, t),
        v: lerp_f32(from.v, to.v, t),
        color: interpolate_color(from.color, to.color, to.color, 1.0 - t, t, 0.0),
    }
}

fn lerp_f32(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}

pub(super) fn project_camera_vertex(vertex: M3gCameraVertex, fovy: f32, aspect: f32, viewport_w: i32, viewport_h: i32) -> Option<M3gDrawVertex> {
    let depth = vertex.depth;
    if depth <= 0.0001 {
        return None;
    }

    let fovy = if fovy.is_finite() && fovy > 0.0 { fovy } else { 45.0 };
    let focal = 1.0 / (fovy * core::f32::consts::PI / 360.0).tan();
    let ndc_x = (vertex.x * focal / aspect) / depth;
    let ndc_y = (vertex.y * focal) / depth;
    if !ndc_x.is_finite() || !ndc_y.is_finite() {
        return None;
    }

    Some(M3gDrawVertex {
        x: (ndc_x * 0.5 + 0.5) * viewport_w as f32,
        y: (0.5 - ndc_y * 0.5) * viewport_h as f32,
        z: depth,
        u: vertex.u,
        v: vertex.v,
        color: vertex.color,
    })
}

pub(super) fn appearance_sort_key(appearance: &M3gAppearanceState) -> i32 {
    let layer = appearance.layer.clamp(-63, 63) + 63;
    let blended = i32::from(appearance.blending != CompositingMode::REPLACE);
    (layer << 1) | blended
}

pub(super) fn rasterize_triangle(
    pixels: &mut [i32],
    depth: &mut [f32],
    width: i32,
    height: i32,
    v0: M3gDrawVertex,
    v1: M3gDrawVertex,
    v2: M3gDrawVertex,
    appearance: &M3gAppearanceState,
    depth_enabled: bool,
    camera_near: f32,
    camera_far: f32,
    depth_range_near: f32,
    depth_range_far: f32,
    dirty: &mut M3gDirtyRect,
) -> usize {
    let area = edge(v0.x, v0.y, v1.x, v1.y, v2.x, v2.y);
    if area.abs() <= 0.0001 {
        return 0;
    }
    if should_cull_triangle(area, appearance.culling, appearance.winding) {
        return 0;
    }

    let min_x = floor_i32(v0.x.min(v1.x).min(v2.x)).clamp(0, width - 1);
    let max_x = ceil_i32(v0.x.max(v1.x).max(v2.x)).clamp(0, width - 1);
    let min_y = floor_i32(v0.y.min(v1.y).min(v2.y)).clamp(0, height - 1);
    let max_y = ceil_i32(v0.y.max(v1.y).max(v2.y)).clamp(0, height - 1);
    if min_x > max_x || min_y > max_y {
        return 0;
    }

    let inv_area = 1.0 / area;
    let edge_epsilon = 0.0001 * area.abs();
    let positive_area = area > 0.0;
    let max_edge = ((v1.x - v0.x).abs() + (v1.y - v0.y).abs())
        .max((v2.x - v1.x).abs() + (v2.y - v1.y).abs())
        .max((v0.x - v2.x).abs() + (v0.y - v2.y).abs())
        .max(1.0);
    let depth0 = depth_buffer_value(v0.z, camera_near, camera_far, depth_range_near, depth_range_far);
    let depth1 = depth_buffer_value(v1.z, camera_near, camera_far, depth_range_near, depth_range_far);
    let depth2 = depth_buffer_value(v2.z, camera_near, camera_far, depth_range_near, depth_range_far);
    let depth_slope = (depth1 - depth0).abs().max((depth2 - depth0).abs()).max((depth2 - depth1).abs()) / max_edge;
    let depth_bias = appearance.depth_offset_factor * depth_slope * 0.001 + appearance.depth_offset_units * 0.0001;
    let depth_test = depth_enabled && appearance.depth_test;
    let depth_write = depth_enabled && appearance.depth_write;
    let constant_fragment_color = if v0.color == v1.color && v1.color == v2.color {
        Some(v0.color)
    } else {
        None
    };
    let inv_z0 = 1.0 / v0.z.max(0.0001);
    let inv_z1 = 1.0 / v1.z.max(0.0001);
    let inv_z2 = 1.0 / v2.z.max(0.0001);
    let u0_inv_z = v0.u * inv_z0;
    let u1_inv_z = v1.u * inv_z1;
    let u2_inv_z = v2.u * inv_z2;
    let v0_inv_z = v0.v * inv_z0;
    let v1_inv_z = v1.v * inv_z1;
    let v2_inv_z = v2.v * inv_z2;
    let camera_depth_scale = if camera_far > camera_near {
        Some(1.0 / (camera_far - camera_near))
    } else {
        None
    };
    let edge0 = EdgeStep::new(v1.x, v1.y, v2.x, v2.y, min_x, min_y);
    let edge1 = EdgeStep::new(v2.x, v2.y, v0.x, v0.y, min_x, min_y);
    let edge2 = EdgeStep::new(v0.x, v0.y, v1.x, v1.y, min_x, min_y);
    let inv_z_plane = AttrStep::new(inv_z0, inv_z1, inv_z2, edge0, edge1, edge2, inv_area);
    let u_inv_z_plane = AttrStep::new(u0_inv_z, u1_inv_z, u2_inv_z, edge0, edge1, edge2, inv_area);
    let v_inv_z_plane = AttrStep::new(v0_inv_z, v1_inv_z, v2_inv_z, edge0, edge1, edge2, inv_area);
    let u_plane = AttrStep::new(v0.u, v1.u, v2.u, edge0, edge1, edge2, inv_area);
    let v_plane = AttrStep::new(v0.v, v1.v, v2.v, edge0, edge1, edge2, inv_area);
    let z_plane = AttrStep::new(v0.z, v1.z, v2.z, edge0, edge1, edge2, inv_area);
    let mut written = 0;
    let mut row0 = edge0.start;
    let mut row1 = edge1.start;
    let mut row2 = edge2.start;
    let mut row_inv_z = inv_z_plane.start;
    let mut row_u_inv_z = u_inv_z_plane.start;
    let mut row_v_inv_z = v_inv_z_plane.start;
    let mut row_u = u_plane.start;
    let mut row_v = v_plane.start;
    let mut row_z = z_plane.start;
    for y in min_y..=max_y {
        let mut edge_value0 = row0;
        let mut edge_value1 = row1;
        let mut edge_value2 = row2;
        let mut inv_z_value = row_inv_z;
        let mut u_inv_z_value = row_u_inv_z;
        let mut v_inv_z_value = row_v_inv_z;
        let mut u_value = row_u;
        let mut v_value = row_v;
        let mut z_value = row_z;
        for x in min_x..=max_x {
            let pixel_edge0 = edge_value0;
            let pixel_edge1 = edge_value1;
            let pixel_edge2 = edge_value2;
            edge_value0 += edge0.x_step;
            edge_value1 += edge1.x_step;
            edge_value2 += edge2.x_step;
            let inv_z = inv_z_value;
            let u_inv_z = u_inv_z_value;
            let v_inv_z = v_inv_z_value;
            let affine_u = u_value;
            let affine_v = v_value;
            let affine_z = z_value;
            inv_z_value += inv_z_plane.x_step;
            u_inv_z_value += u_inv_z_plane.x_step;
            v_inv_z_value += v_inv_z_plane.x_step;
            u_value += u_plane.x_step;
            v_value += v_plane.x_step;
            z_value += z_plane.x_step;
            let inside = if positive_area {
                pixel_edge0 >= -edge_epsilon && pixel_edge1 >= -edge_epsilon && pixel_edge2 >= -edge_epsilon
            } else {
                pixel_edge0 <= edge_epsilon && pixel_edge1 <= edge_epsilon && pixel_edge2 <= edge_epsilon
            };
            if !inside {
                continue;
            }

            let w0 = pixel_edge0 * inv_area;
            let w1 = pixel_edge1 * inv_area;
            let w2 = pixel_edge2 * inv_area;
            let reciprocal_z = if inv_z.is_finite() && inv_z.abs() > f32::EPSILON {
                Some(1.0 / inv_z)
            } else {
                None
            };
            let camera_z = reciprocal_z.unwrap_or(affine_z);
            let z = depth_buffer_value_fast(camera_z, camera_near, depth_range_near, depth_range_far, camera_depth_scale) + depth_bias;
            let index = (y * width + x) as usize;
            if depth_test && z > depth[index] {
                continue;
            }

            let fragment_color = constant_fragment_color.unwrap_or_else(|| interpolate_color(v0.color, v1.color, v2.color, w0, w1, w2));
            let mut color = if let Some(texture) = appearance.texture.as_ref() {
                let (u, v) = if appearance.perspective_correction {
                    if let Some(reciprocal_z) = reciprocal_z {
                        (u_inv_z * reciprocal_z, v_inv_z * reciprocal_z)
                    } else {
                        (affine_u, affine_v)
                    }
                } else {
                    (affine_u, affine_v)
                };
                blend_texture(texture, fragment_color, sample_texture(texture, u, v))
            } else {
                fragment_color
            };

            let alpha = ((color as u32) >> 24) & 0xff;
            if alpha == 0 || (alpha as f32 / 255.0) < appearance.alpha_threshold {
                continue;
            }

            if depth_write {
                depth[index] = z;
            }
            if appearance.color_write {
                if !appearance.alpha_write {
                    let dst_alpha = (pixels[index] as u32) & 0xff00_0000;
                    let alpha = if dst_alpha == 0 { 0xff00_0000 } else { dst_alpha };
                    color = (alpha | ((color as u32) & 0x00ff_ffff)) as i32;
                }
                pixels[index] = compose_m3g_pixel(pixels[index], color, appearance.blending);
            }
            written += 1;
        }
        row0 += edge0.y_step;
        row1 += edge1.y_step;
        row2 += edge2.y_step;
        row_inv_z += inv_z_plane.y_step;
        row_u_inv_z += u_inv_z_plane.y_step;
        row_v_inv_z += v_inv_z_plane.y_step;
        row_u += u_plane.y_step;
        row_v += v_plane.y_step;
        row_z += z_plane.y_step;
    }
    if written > 0 {
        dirty.include_rect(min_x, min_y, max_x, max_y);
    }
    written
}

#[derive(Clone, Copy)]
struct EdgeStep {
    start: f32,
    x_step: f32,
    y_step: f32,
}

impl EdgeStep {
    fn new(ax: f32, ay: f32, bx: f32, by: f32, min_x: i32, min_y: i32) -> Self {
        let x_step = by - ay;
        let y_step = -(bx - ax);
        let px = min_x as f32 + 0.5;
        let py = min_y as f32 + 0.5;
        Self {
            start: (px - ax) * x_step + (py - ay) * y_step,
            x_step,
            y_step,
        }
    }
}

#[derive(Clone, Copy)]
struct AttrStep {
    start: f32,
    x_step: f32,
    y_step: f32,
}

impl AttrStep {
    fn new(a0: f32, a1: f32, a2: f32, edge0: EdgeStep, edge1: EdgeStep, edge2: EdgeStep, inv_area: f32) -> Self {
        Self {
            start: (a0 * edge0.start + a1 * edge1.start + a2 * edge2.start) * inv_area,
            x_step: (a0 * edge0.x_step + a1 * edge1.x_step + a2 * edge2.x_step) * inv_area,
            y_step: (a0 * edge0.y_step + a1 * edge1.y_step + a2 * edge2.y_step) * inv_area,
        }
    }
}

fn depth_buffer_value(camera_z: f32, camera_near: f32, camera_far: f32, range_near: f32, range_far: f32) -> f32 {
    let normalized = if camera_far > camera_near {
        ((camera_z - camera_near) / (camera_far - camera_near)).clamp(0.0, 1.0)
    } else {
        camera_z.max(0.0)
    };
    range_near + normalized * (range_far - range_near)
}

fn depth_buffer_value_fast(camera_z: f32, camera_near: f32, range_near: f32, range_far: f32, camera_scale: Option<f32>) -> f32 {
    let normalized = if let Some(scale) = camera_scale {
        ((camera_z - camera_near) * scale).clamp(0.0, 1.0)
    } else {
        camera_z.max(0.0)
    };
    range_near + normalized * (range_far - range_near)
}

fn edge(ax: f32, ay: f32, bx: f32, by: f32, px: f32, py: f32) -> f32 {
    (px - ax) * (by - ay) - (py - ay) * (bx - ax)
}

pub(super) fn should_cull_triangle(area: f32, culling: i32, winding: i32) -> bool {
    if culling == PolygonMode::CULL_NONE {
        return false;
    }

    let front_facing = if winding == PolygonMode::WINDING_CW { area < 0.0 } else { area > 0.0 };
    match culling {
        PolygonMode::CULL_FRONT => front_facing,
        _ => !front_facing,
    }
}

pub(super) fn material_base_color(material: Option<(i32, i32, i32)>, default_color: i32, lighting: M3gSceneLighting) -> i32 {
    let Some((ambient, diffuse, emissive)) = material else {
        return default_color;
    };
    if lighting.ambient_lights == 0 {
        return diffuse;
    }

    let alpha = ((diffuse as u32) >> 24) & 0xff;
    let r = (color_channel_f32(ambient, 16) * lighting.ambient_r + color_channel_f32(emissive, 16)).clamp(0.0, 1.0);
    let g = (color_channel_f32(ambient, 8) * lighting.ambient_g + color_channel_f32(emissive, 8)).clamp(0.0, 1.0);
    let b = (color_channel_f32(ambient, 0) * lighting.ambient_b + color_channel_f32(emissive, 0)).clamp(0.0, 1.0);
    pack_argb(alpha, (r * 255.0) as u32, (g * 255.0) as u32, (b * 255.0) as u32)
}

pub(super) fn color_channel_f32(color: i32, shift: u32) -> f32 {
    (((color as u32) >> shift) & 0xff) as f32 / 255.0
}

fn compose_m3g_pixel(dst: i32, src: i32, blending: i32) -> i32 {
    match blending {
        CompositingMode::ALPHA => source_over(dst, src),
        CompositingMode::ALPHA_ADD => alpha_add(dst, src),
        CompositingMode::MODULATE => framebuffer_modulate(dst, src, 1),
        CompositingMode::MODULATE_X2 => framebuffer_modulate(dst, src, 2),
        _ => src,
    }
}

pub(super) fn source_over(dst: i32, src: i32) -> i32 {
    let src = src as u32;
    let dst = dst as u32;
    let src_a = (src >> 24) & 0xff;
    if src_a == 0xff {
        return src as i32;
    }
    if src_a == 0 {
        return dst as i32;
    }

    let dst_a = (dst >> 24) & 0xff;
    let inv_a = 255 - src_a;
    let out_a = src_a + mul_u8(dst_a, inv_a);
    if out_a == 0 {
        return 0;
    }

    let src_r = (src >> 16) & 0xff;
    let src_g = (src >> 8) & 0xff;
    let src_b = src & 0xff;
    let dst_r = (dst >> 16) & 0xff;
    let dst_g = (dst >> 8) & 0xff;
    let dst_b = dst & 0xff;
    let dst_weight = mul_u8(dst_a, inv_a);
    pack_argb(
        out_a,
        ((src_r * src_a + dst_r * dst_weight) / out_a).min(255),
        ((src_g * src_a + dst_g * dst_weight) / out_a).min(255),
        ((src_b * src_a + dst_b * dst_weight) / out_a).min(255),
    )
}

fn alpha_add(dst: i32, src: i32) -> i32 {
    let src = src as u32;
    let dst = dst as u32;
    let src_a = (src >> 24) & 0xff;
    let dst_a = (dst >> 24) & 0xff;
    let src_r = mul_u8((src >> 16) & 0xff, src_a);
    let src_g = mul_u8((src >> 8) & 0xff, src_a);
    let src_b = mul_u8(src & 0xff, src_a);
    pack_argb(
        (src_a + dst_a).min(255),
        (((dst >> 16) & 0xff) + src_r).min(255),
        (((dst >> 8) & 0xff) + src_g).min(255),
        ((dst & 0xff) + src_b).min(255),
    )
}

fn framebuffer_modulate(dst: i32, src: i32, factor: u32) -> i32 {
    let src = src as u32;
    let dst = dst as u32;
    if ((dst >> 24) & 0xff) == 0 {
        return src as i32;
    }
    pack_argb(
        (((src >> 24) & 0xff) * ((dst >> 24) & 0xff) * factor / 255).min(255),
        (((src >> 16) & 0xff) * ((dst >> 16) & 0xff) * factor / 255).min(255),
        (((src >> 8) & 0xff) * ((dst >> 8) & 0xff) * factor / 255).min(255),
        ((src & 0xff) * (dst & 0xff) * factor / 255).min(255),
    )
}

fn floor_i32(value: f32) -> i32 {
    let integer = value as i32;
    if value < integer as f32 { integer - 1 } else { integer }
}

fn ceil_i32(value: f32) -> i32 {
    let integer = value as i32;
    if value > integer as f32 { integer + 1 } else { integer }
}

pub(super) fn sample_texture(texture: &M3gTexture, u: f32, v: f32) -> i32 {
    if texture.width <= 0 || texture.height <= 0 || texture.pixels.is_empty() {
        return 0xffff_ffffu32 as i32;
    }
    let (u, v) = if texture.transform_identity {
        (u, v)
    } else {
        transform_texture_coord(texture.transform, u, v)
    };
    if texture.image_filter == Texture2D::FILTER_LINEAR {
        return sample_texture_linear(texture, u, v);
    }
    let x = texture_coord_to_index(u, texture.width, texture.wrap_s);
    let y = texture_coord_to_index(v, texture.height, texture.wrap_t);
    texture.pixels[(y * texture.width + x) as usize]
}

fn sample_texture_linear(texture: &M3gTexture, u: f32, v: f32) -> i32 {
    let (x0, x1, tx) = texture_coord_to_linear_indices(u, texture.width, texture.wrap_s);
    let (y0, y1, ty) = texture_coord_to_linear_indices(v, texture.height, texture.wrap_t);
    let c00 = texture.pixels[(y0 * texture.width + x0) as usize];
    let c10 = texture.pixels[(y0 * texture.width + x1) as usize];
    let c01 = texture.pixels[(y1 * texture.width + x0) as usize];
    let c11 = texture.pixels[(y1 * texture.width + x1) as usize];
    pack_argb(
        bilinear_channel(c00, c10, c01, c11, tx, ty, 24),
        bilinear_channel(c00, c10, c01, c11, tx, ty, 16),
        bilinear_channel(c00, c10, c01, c11, tx, ty, 8),
        bilinear_channel(c00, c10, c01, c11, tx, ty, 0),
    )
}

fn transform_texture_coord(matrix: [f32; 16], u: f32, v: f32) -> (f32, f32) {
    let s = matrix[0] * u + matrix[1] * v + matrix[3];
    let t = matrix[4] * u + matrix[5] * v + matrix[7];
    let q = matrix[12] * u + matrix[13] * v + matrix[15];
    if q.is_finite() && q.abs() > f32::EPSILON {
        (s / q, t / q)
    } else {
        (s, t)
    }
}

pub(super) fn texture_transform_is_identity(matrix: [f32; 16]) -> bool {
    matrix[0] == 1.0
        && matrix[1] == 0.0
        && matrix[3] == 0.0
        && matrix[4] == 0.0
        && matrix[5] == 1.0
        && matrix[7] == 0.0
        && matrix[12] == 0.0
        && matrix[13] == 0.0
        && matrix[15] == 1.0
}

fn texture_coord_to_index(coord: f32, size: i32, wrapping: i32) -> i32 {
    if size <= 1 {
        return 0;
    }
    if wrapping == Texture2D::WRAP_CLAMP {
        let coord = coord.clamp(0.0, 1.0);
        return floor_i32(coord * (size - 1) as f32 + 0.5).clamp(0, size - 1);
    }
    if is_power_of_two(size) {
        return floor_i32(coord * size as f32) & (size - 1);
    }
    let coord = coord - floor_f32(coord);
    floor_i32(coord * size as f32).clamp(0, size - 1)
}

fn texture_coord_to_linear_indices(coord: f32, size: i32, wrapping: i32) -> (i32, i32, f32) {
    if size <= 1 {
        return (0, 0, 0.0);
    }

    let coord = if wrapping == Texture2D::WRAP_CLAMP {
        coord.clamp(0.0, 1.0)
    } else {
        coord - floor_f32(coord)
    };
    let position = coord * size as f32 - 0.5;
    let x0 = floor_i32(position);
    let t = position - x0 as f32;
    (
        texture_index(x0, size, wrapping),
        texture_index(x0 + 1, size, wrapping),
        t.clamp(0.0, 1.0),
    )
}

fn texture_index(index: i32, size: i32, wrapping: i32) -> i32 {
    if wrapping == Texture2D::WRAP_CLAMP {
        return index.clamp(0, size - 1);
    }
    if is_power_of_two(size) {
        return index & (size - 1);
    }
    let index = index % size;
    if index < 0 { index + size } else { index }
}

fn is_power_of_two(value: i32) -> bool {
    value > 0 && (value & (value - 1)) == 0
}

fn bilinear_channel(c00: i32, c10: i32, c01: i32, c11: i32, tx: f32, ty: f32, shift: u32) -> u32 {
    let c00 = (((c00 as u32) >> shift) & 0xff) as f32;
    let c10 = (((c10 as u32) >> shift) & 0xff) as f32;
    let c01 = (((c01 as u32) >> shift) & 0xff) as f32;
    let c11 = (((c11 as u32) >> shift) & 0xff) as f32;
    let top = c00 + (c10 - c00) * tx;
    let bottom = c01 + (c11 - c01) * tx;
    (top + (bottom - top) * ty).clamp(0.0, 255.0) as u32
}

pub(super) fn background_coord(coord: i32, size: i32, mode: i32) -> Option<i32> {
    if size <= 0 {
        return None;
    }
    if mode == Background::REPEAT {
        let value = coord % size;
        return Some(if value < 0 { value + size } else { value });
    }
    if (0..size).contains(&coord) { Some(coord) } else { None }
}

fn blend_texture(texture: &M3gTexture, fragment: i32, texel: i32) -> i32 {
    let frag = fragment as u32;
    let tex = texel as u32;
    let fa = (frag >> 24) & 0xff;
    let fr = (frag >> 16) & 0xff;
    let fg = (frag >> 8) & 0xff;
    let fb = frag & 0xff;
    let ta = (tex >> 24) & 0xff;
    let tr = (tex >> 16) & 0xff;
    let tg = (tex >> 8) & 0xff;
    let tb = tex & 0xff;

    match texture.blending {
        Texture2D::FUNC_REPLACE => {
            if texture.format == Image2D::ALPHA {
                pack_argb(ta, fr, fg, fb)
            } else if texture.format == Image2D::RGB || texture.format == Image2D::LUMINANCE {
                pack_argb(fa, tr, tg, tb)
            } else {
                pack_argb(ta, tr, tg, tb)
            }
        }
        Texture2D::FUNC_DECAL => pack_argb(fa, lerp_u8(fr, tr, ta), lerp_u8(fg, tg, ta), lerp_u8(fb, tb, ta)),
        Texture2D::FUNC_ADD => pack_argb(
            if texture.format == Image2D::RGB || texture.format == Image2D::LUMINANCE {
                fa
            } else {
                mul_u8(fa, ta)
            },
            (fr + tr).min(255),
            (fg + tg).min(255),
            (fb + tb).min(255),
        ),
        Texture2D::FUNC_BLEND => {
            let blend = texture.blend_color as u32;
            let br = (blend >> 16) & 0xff;
            let bg = (blend >> 8) & 0xff;
            let bb = blend & 0xff;
            pack_argb(
                if texture.format == Image2D::RGB || texture.format == Image2D::LUMINANCE {
                    fa
                } else {
                    mul_u8(fa, ta)
                },
                blend_mode_channel(fr, br, tr),
                blend_mode_channel(fg, bg, tg),
                blend_mode_channel(fb, bb, tb),
            )
        }
        _ => {
            if texture.format == Image2D::ALPHA {
                pack_argb(mul_u8(fa, ta), fr, fg, fb)
            } else if texture.format == Image2D::RGB || texture.format == Image2D::LUMINANCE {
                pack_argb(fa, mul_u8(fr, tr), mul_u8(fg, tg), mul_u8(fb, tb))
            } else {
                pack_argb(mul_u8(fa, ta), mul_u8(fr, tr), mul_u8(fg, tg), mul_u8(fb, tb))
            }
        }
    }
}

fn pack_argb(a: u32, r: u32, g: u32, b: u32) -> i32 {
    ((a << 24) | (r << 16) | (g << 8) | b) as i32
}

fn mul_u8(left: u32, right: u32) -> u32 {
    ((left * right) / 255).min(255)
}

fn lerp_u8(from: u32, to: u32, alpha: u32) -> u32 {
    ((from * (255 - alpha) + to * alpha) / 255).min(255)
}

fn blend_mode_channel(fragment: u32, blend: u32, texture: u32) -> u32 {
    ((fragment * (255 - texture) + blend * texture) / 255).min(255)
}

fn floor_f32(value: f32) -> f32 {
    let integer = value as i32;
    if value < integer as f32 { (integer - 1) as f32 } else { integer as f32 }
}

fn interpolate_color(c0: i32, c1: i32, c2: i32, w0: f32, w1: f32, w2: f32) -> i32 {
    let c0 = c0 as u32;
    let c1 = c1 as u32;
    let c2 = c2 as u32;
    let a = channel(c0, c1, c2, 24, w0, w1, w2);
    let r = channel(c0, c1, c2, 16, w0, w1, w2);
    let g = channel(c0, c1, c2, 8, w0, w1, w2);
    let b = channel(c0, c1, c2, 0, w0, w1, w2);
    ((a << 24) | (r << 16) | (g << 8) | b) as i32
}

fn channel(c0: u32, c1: u32, c2: u32, shift: u32, w0: f32, w1: f32, w2: f32) -> u32 {
    let v0 = ((c0 >> shift) & 0xff) as f32;
    let v1 = ((c1 >> shift) & 0xff) as f32;
    let v2 = ((c2 >> shift) & 0xff) as f32;
    (v0 * w0 + v1 * w1 + v2 * w2).clamp(0.0, 255.0) as u32
}

pub(super) fn clamp_color(value: f32) -> u32 {
    if value < 0.0 {
        (value as i32 as i8 as u8) as u32
    } else {
        value.clamp(0.0, 255.0) as u32
    }
}

pub(super) fn ensure_opaque(color: i32) -> i32 {
    if (color as u32) & 0xff00_0000 == 0 {
        ((color as u32) | 0xff00_0000) as i32
    } else {
        color
    }
}

pub(super) fn decode_m3g_image_pixels(format: i32, width: i32, height: i32, palette: &[u8], pixels: &[u8]) -> Vec<i32> {
    let pixel_count = (width.max(1) * height.max(1)) as usize;
    let stride = m3g_image_pixel_stride(format);
    let mut argb = Vec::with_capacity(pixel_count);

    if !palette.is_empty() {
        for index in pixels.iter().take(pixel_count) {
            let offset = *index as usize * stride;
            argb.push(decode_m3g_pixel(format, palette.get(offset..offset + stride).unwrap_or(&[])));
        }
    } else {
        for index in 0..pixel_count {
            let offset = index * stride;
            argb.push(decode_m3g_pixel(format, pixels.get(offset..offset + stride).unwrap_or(&[])));
        }
    }

    argb.resize(pixel_count, 0xffff_ffffu32 as i32);
    argb
}

fn m3g_image_pixel_stride(format: i32) -> usize {
    match format {
        Image2D::ALPHA | Image2D::LUMINANCE => 1,
        Image2D::LUMINANCE_ALPHA => 2,
        Image2D::RGB => 3,
        Image2D::RGBA => 4,
        _ => 4,
    }
}

fn decode_m3g_pixel(format: i32, bytes: &[u8]) -> i32 {
    match format {
        Image2D::ALPHA => {
            let a = bytes.first().copied().unwrap_or(255) as u32;
            pack_argb(a, 255, 255, 255)
        }
        Image2D::LUMINANCE => {
            let l = bytes.first().copied().unwrap_or(255) as u32;
            pack_argb(255, l, l, l)
        }
        Image2D::LUMINANCE_ALPHA => {
            let l = bytes.first().copied().unwrap_or(255) as u32;
            let a = bytes.get(1).copied().unwrap_or(255) as u32;
            pack_argb(a, l, l, l)
        }
        Image2D::RGB => {
            let r = bytes.first().copied().unwrap_or(255) as u32;
            let g = bytes.get(1).copied().unwrap_or(255) as u32;
            let b = bytes.get(2).copied().unwrap_or(255) as u32;
            pack_argb(255, r, g, b)
        }
        _ => {
            let r = bytes.first().copied().unwrap_or(255) as u32;
            let g = bytes.get(1).copied().unwrap_or(255) as u32;
            let b = bytes.get(2).copied().unwrap_or(255) as u32;
            let a = bytes.get(3).copied().unwrap_or(255) as u32;
            pack_argb(a, r, g, b)
        }
    }
}
