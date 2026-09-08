use alloc::{sync::Arc, vec::Vec};

use jvm::ClassInstanceRef;

use super::super::{
    CompositingMode, IndexBuffer, PolygonMode,
    math::{transform_point, transform_vec4},
};
use super::texture::interpolate_color;

#[derive(Clone, Copy)]
pub(crate) struct M3gDrawVertex {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) u: f32,
    pub(crate) v: f32,
    pub(crate) u1: f32,
    pub(crate) v1: f32,
    pub(crate) color: i32,
}

#[derive(Clone, Copy)]
pub(crate) struct M3gCameraVertex {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) depth: f32,
    pub(crate) u: f32,
    pub(crate) v: f32,
    pub(crate) u1: f32,
    pub(crate) v1: f32,
    pub(crate) color: i32,
}

pub(crate) struct M3gTexture {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) pixels: Arc<Vec<i32>>,
    pub(crate) format: i32,
    pub(crate) blend_color: i32,
    pub(crate) blending: i32,
    pub(crate) wrap_s: i32,
    pub(crate) wrap_t: i32,
    pub(crate) image_filter: i32,
    pub(crate) transform: [f32; 16],
    pub(crate) transform_identity: bool,
}

pub(crate) struct M3gAppearanceState {
    pub(crate) texture: Option<M3gTexture>,
    pub(crate) texture1: Option<M3gTexture>,
    pub(crate) base_color: i32,
    pub(crate) layer: i32,
    pub(crate) color_write: bool,
    pub(crate) alpha_write: bool,
    pub(crate) depth_test: bool,
    pub(crate) depth_write: bool,
    pub(crate) depth_offset_factor: f32,
    pub(crate) depth_offset_units: f32,
    pub(crate) alpha_threshold: f32,
    pub(crate) blending: i32,
    pub(crate) culling: i32,
    pub(crate) winding: i32,
    pub(crate) perspective_correction: bool,
    pub(crate) two_sided_lighting: bool,
    pub(crate) local_camera_lighting: bool,
    pub(crate) material: Option<M3gMaterialState>,
    pub(crate) fog: Option<M3gFogState>,
}

#[derive(Clone, Copy)]
pub(crate) struct M3gMaterialState {
    pub(crate) ambient: i32,
    pub(crate) diffuse: i32,
    pub(crate) emissive: i32,
    pub(crate) specular: i32,
    pub(crate) shininess: f32,
    pub(crate) vertex_color_tracking: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct M3gFogState {
    pub(crate) mode: i32,
    pub(crate) color: i32,
    pub(crate) density: f32,
    pub(crate) near: f32,
    pub(crate) far: f32,
}

#[derive(Clone, Copy)]
pub(crate) struct M3gLightState {
    pub(crate) mode: i32,
    pub(crate) color: [f32; 3],
    pub(crate) position: [f32; 3],
    pub(crate) direction: [f32; 3],
    pub(crate) constant: f32,
    pub(crate) linear: f32,
    pub(crate) quadratic: f32,
    pub(crate) spot_cos: f32,
    pub(crate) spot_exponent: f32,
    pub(crate) scope: i32,
}

#[derive(Clone, Default)]
pub(crate) struct M3gSceneLighting {
    pub(crate) ambient_r: f32,
    pub(crate) ambient_g: f32,
    pub(crate) ambient_b: f32,
    pub(crate) ambient_lights: usize,
    pub(crate) lights: Vec<M3gLightState>,
}

pub(crate) struct M3gBackgroundFrame {
    pub(crate) pixels: Vec<i32>,
    pub(crate) color_clear: bool,
    pub(crate) depth_clear: bool,
    pub(crate) process_alpha: bool,
    pub(crate) submit_when_empty: bool,
}

pub(crate) struct M3gSubmeshDraw {
    pub(crate) sort_key: i32,
    pub(crate) sequence: usize,
    pub(crate) index_buffer: ClassInstanceRef<IndexBuffer>,
    pub(crate) appearance: M3gAppearanceState,
}

#[derive(Default)]
pub(crate) struct M3gRenderStats {
    pub(crate) nodes: usize,
    pub(crate) meshes: usize,
    pub(crate) rendered_meshes: usize,
    pub(crate) vertices: usize,
    pub(crate) projected_vertices: usize,
    pub(crate) candidate_triangles: usize,
    pub(crate) triangles: usize,
    pub(crate) pixels: usize,
    pub(crate) dirty: M3gDirtyRect,
    pub(crate) mesh_vertex_ms: u64,
    pub(crate) mesh_appearance_ms: u64,
    pub(crate) mesh_raster_ms: u64,
}

#[derive(Default)]
pub(crate) struct M3gMeshRenderStats {
    pub(crate) submeshes: usize,
    pub(crate) vertices: usize,
    pub(crate) projected_vertices: usize,
    pub(crate) candidate_triangles: usize,
    pub(crate) rasterized_triangles: usize,
    pub(crate) textured_submeshes: usize,
    pub(crate) pixels: usize,
    pub(crate) dirty: M3gDirtyRect,
    pub(crate) vertex_ms: u64,
    pub(crate) appearance_ms: u64,
    pub(crate) raster_ms: u64,
}

#[derive(Clone, Copy)]
pub(crate) struct M3gDirtyRect {
    pub(crate) min_x: i32,
    pub(crate) min_y: i32,
    pub(crate) max_x: i32,
    pub(crate) max_y: i32,
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
    pub(crate) fn is_empty(self) -> bool {
        self.min_x > self.max_x || self.min_y > self.max_y
    }

    pub(crate) fn include_rect(&mut self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
        self.min_x = self.min_x.min(min_x);
        self.min_y = self.min_y.min(min_y);
        self.max_x = self.max_x.max(max_x);
        self.max_y = self.max_y.max(max_y);
    }

    pub(crate) fn include_dirty(&mut self, other: Self) {
        if !other.is_empty() {
            self.include_rect(other.min_x, other.min_y, other.max_x, other.max_y);
        }
    }

    pub(crate) fn bounds(self) -> Option<(i32, i32, i32, i32)> {
        if self.is_empty() {
            None
        } else {
            Some((self.min_x, self.min_y, self.max_x - self.min_x + 1, self.max_y - self.min_y + 1))
        }
    }
}

pub(crate) fn default_appearance_state(base_color: i32) -> M3gAppearanceState {
    M3gAppearanceState {
        texture: None,
        texture1: None,
        base_color,
        layer: 0,
        color_write: true,
        alpha_write: true,
        depth_test: true,
        depth_write: true,
        depth_offset_factor: 0.0,
        depth_offset_units: 0.0,
        alpha_threshold: 0.0,
        blending: CompositingMode::REPLACE,
        culling: PolygonMode::CULL_BACK,
        winding: PolygonMode::WINDING_CCW,
        perspective_correction: true,
        two_sided_lighting: false,
        local_camera_lighting: false,
        material: None,
        fog: None,
    }
}

pub(crate) fn camera_vertex(position: [f32; 3], uv: [f32; 2], uv1: [f32; 2], color: i32, model_view: [f32; 16]) -> Option<M3gCameraVertex> {
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
        u1: uv1[0],
        v1: uv1[1],
        color,
    })
}

pub(crate) fn clip_triangle_near(v0: M3gCameraVertex, v1: M3gCameraVertex, v2: M3gCameraVertex, near: f32) -> Vec<M3gCameraVertex> {
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
        u1: lerp_f32(from.u1, to.u1, t),
        v1: lerp_f32(from.v1, to.v1, t),
        color: interpolate_color(from.color, to.color, to.color, 1.0 - t, t, 0.0),
    }
}

fn lerp_f32(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}

#[allow(dead_code)]
pub(crate) fn project_camera_vertex(vertex: M3gCameraVertex, fovy: f32, aspect: f32, viewport_w: i32, viewport_h: i32) -> Option<M3gDrawVertex> {
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
        u1: vertex.u1,
        v1: vertex.v1,
        color: vertex.color,
    })
}

pub(crate) fn project_clip_vertex(vertex: M3gCameraVertex, projection: [f32; 16], viewport_w: i32, viewport_h: i32) -> Option<M3gDrawVertex> {
    let clip = transform_vec4(projection, [vertex.x, vertex.y, vertex.depth, 1.0]);
    if !clip[3].is_finite() || clip[3].abs() <= 0.0001 {
        return None;
    }
    let ndc_x = clip[0] / clip[3];
    let ndc_y = clip[1] / clip[3];
    if !ndc_x.is_finite() || !ndc_y.is_finite() {
        return None;
    }

    Some(M3gDrawVertex {
        x: (ndc_x * 0.5 + 0.5) * viewport_w as f32,
        y: (0.5 - ndc_y * 0.5) * viewport_h as f32,
        z: vertex.depth,
        u: vertex.u,
        v: vertex.v,
        u1: vertex.u1,
        v1: vertex.v1,
        color: vertex.color,
    })
}

pub(crate) fn appearance_sort_key(appearance: &M3gAppearanceState) -> i32 {
    let layer = appearance.layer.clamp(-63, 63) + 63;
    let blended = i32::from(appearance.blending != CompositingMode::REPLACE);
    (layer << 1) | blended
}
