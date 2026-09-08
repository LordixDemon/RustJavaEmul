use alloc::{sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicU64, Ordering};

use parking_lot::Mutex;

use crate::classes::com::mascotcapsule::micro3d::v3::gpu_scene_enabled;

use super::render::{M3gAppearanceState, M3gDrawVertex, M3gFogState, M3gTexture, depth_buffer_value, should_cull_triangle, transform_texture_coord};
use super::{CompositingMode, Image2D, Texture2D};

static M3G_GPU_FRAME_GENERATION: AtomicU64 = AtomicU64::new(1);
static M3G_GPU_FRAME: Mutex<Option<Arc<M3gGpuFrame>>> = Mutex::new(None);

#[derive(Clone)]
pub struct M3gGpuFrame {
    pub generation: u64,
    pub width: i32,
    pub height: i32,
    pub viewport_x: i32,
    pub viewport_y: i32,
    pub clear_depth: bool,
    pub color_clear: bool,
    pub textures: Vec<M3gGpuTexture>,
    pub triangles: Vec<M3gGpuTriangle>,
}

#[derive(Clone)]
pub struct M3gGpuTexture {
    pub width: i32,
    pub height: i32,
    pub rgba: Vec<u8>,
    pub wrap_s: i32,
    pub wrap_t: i32,
    pub filter: i32,
}

#[derive(Clone, Copy)]
pub struct M3gGpuVertex {
    pub x: f32,
    pub y: f32,
    pub depth: f32,
    pub camera_z: f32,
    pub u: f32,
    pub v: f32,
    pub u1: f32,
    pub v1: f32,
    pub color: i32,
}

#[derive(Clone, Copy)]
pub struct M3gGpuTex {
    pub index: Option<usize>,
    pub blending: i32,
    pub format: i32,
    pub blend_color: i32,
}

#[derive(Clone, Copy, Default)]
pub struct M3gGpuFog {
    pub mode: i32,
    pub color: i32,
    pub density: f32,
    pub near: f32,
    pub far: f32,
}

#[derive(Clone, Copy)]
pub struct M3gGpuTriangle {
    pub vertices: [M3gGpuVertex; 3],
    pub tex0: M3gGpuTex,
    pub tex1: M3gGpuTex,
    pub blending: i32,
    pub depth_test: bool,
    pub depth_write: bool,
    pub alpha_threshold: f32,
    pub fog: M3gGpuFog,
}

pub struct M3gGpuBuilder {
    width: i32,
    height: i32,
    viewport_x: i32,
    viewport_y: i32,
    camera_near: f32,
    camera_far: f32,
    depth_range_near: f32,
    depth_range_far: f32,
    depth_enabled: bool,
    clear_depth: bool,
    textures: Vec<M3gGpuTexture>,
    texture_keys: Vec<(usize, usize)>,
    triangles: Vec<M3gGpuTriangle>,
}

impl M3gGpuTex {
    fn none() -> Self {
        Self {
            index: None,
            blending: 0,
            format: 0,
            blend_color: 0,
        }
    }

    fn from_texture(index: usize, texture: &M3gTexture) -> Self {
        Self {
            index: Some(index),
            blending: texture.blending,
            format: texture.format,
            blend_color: texture.blend_color,
        }
    }

    fn replace(index: usize) -> Self {
        Self {
            index: Some(index),
            blending: Texture2D::FUNC_REPLACE,
            format: Image2D::RGBA,
            blend_color: 0,
        }
    }
}

impl M3gGpuFog {
    fn from_state(fog: Option<M3gFogState>) -> Self {
        let Some(fog) = fog else {
            return Self::default();
        };
        Self {
            mode: fog.mode,
            color: fog.color,
            density: fog.density,
            near: fog.near,
            far: fog.far,
        }
    }
}

impl M3gGpuBuilder {
    pub fn new(
        width: i32,
        height: i32,
        viewport_x: i32,
        viewport_y: i32,
        camera_near: f32,
        camera_far: f32,
        depth_range_near: f32,
        depth_range_far: f32,
        depth_enabled: bool,
        clear_depth: bool,
    ) -> Self {
        Self {
            width,
            height,
            viewport_x,
            viewport_y,
            camera_near,
            camera_far,
            depth_range_near,
            depth_range_far,
            depth_enabled,
            clear_depth,
            textures: Vec::new(),
            texture_keys: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn push_clear_color(&mut self, color: i32) {
        self.push_viewport_quad(M3gGpuTex::none(), color, 1.0, 0.0, 0.0, 1.0, 1.0, false, false, 0.0);
    }

    pub fn push_background_image(&mut self, width: i32, height: i32, pixels: &[i32]) {
        if width <= 0 || height <= 0 || pixels.is_empty() {
            return;
        }
        let index = self.push_rgba_texture(
            width,
            height,
            pixels,
            Texture2D::WRAP_CLAMP,
            Texture2D::WRAP_CLAMP,
            Texture2D::FILTER_NEAREST,
        );
        self.push_viewport_quad(
            M3gGpuTex::replace(index),
            0x00ff_ffffu32 as i32,
            1.0,
            0.0,
            0.0,
            1.0,
            1.0,
            false,
            false,
            0.0,
        );
    }

    pub fn push_projected(&mut self, v0: M3gDrawVertex, v1: M3gDrawVertex, v2: M3gDrawVertex, appearance: &M3gAppearanceState) -> bool {
        if !appearance.color_write {
            return false;
        }
        let area = (v1.x - v0.x) * (v2.y - v0.y) - (v1.y - v0.y) * (v2.x - v0.x);
        if area.abs() <= 0.0001 || should_cull_triangle(area, appearance.culling, appearance.winding) {
            return false;
        }
        let tex0 = appearance
            .texture
            .as_ref()
            .map(|texture| M3gGpuTex::from_texture(self.intern_texture(texture), texture));
        let tex1 = appearance
            .texture1
            .as_ref()
            .map(|texture| M3gGpuTex::from_texture(self.intern_texture(texture), texture));
        let map_vertex = |vertex: M3gDrawVertex| {
            let (u, v) = match appearance.texture.as_ref() {
                Some(texture) if !texture.transform_identity => transform_texture_coord(texture.transform, vertex.u, vertex.v),
                _ => (vertex.u, vertex.v),
            };
            let (u1, v1) = match appearance.texture1.as_ref() {
                Some(texture) if !texture.transform_identity => transform_texture_coord(texture.transform, vertex.u1, vertex.v1),
                _ => (vertex.u1, vertex.v1),
            };
            M3gGpuVertex {
                x: vertex.x,
                y: vertex.y,
                depth: depth_buffer_value(vertex.z, self.camera_near, self.camera_far, self.depth_range_near, self.depth_range_far),
                camera_z: vertex.z,
                u,
                v,
                u1,
                v1,
                color: vertex.color,
            }
        };
        self.triangles.push(M3gGpuTriangle {
            vertices: [map_vertex(v0), map_vertex(v1), map_vertex(v2)],
            tex0: tex0.unwrap_or_else(M3gGpuTex::none),
            tex1: tex1.unwrap_or_else(M3gGpuTex::none),
            blending: appearance.blending,
            depth_test: self.depth_enabled && appearance.depth_test,
            depth_write: self.depth_enabled && appearance.depth_write,
            alpha_threshold: appearance.alpha_threshold,
            fog: M3gGpuFog::from_state(appearance.fog),
        });
        true
    }

    pub fn push_sprite_quad(
        &mut self,
        start_x: f32,
        start_y: f32,
        draw_w: f32,
        draw_h: f32,
        crop_x: f32,
        crop_y: f32,
        crop_w: f32,
        crop_h: f32,
        image_w: i32,
        image_h: i32,
        pixels: &Arc<Vec<i32>>,
        color: i32,
        blending: i32,
        depth: f32,
        depth_test: bool,
        depth_write: bool,
        alpha_threshold: f32,
        fog: Option<M3gFogState>,
    ) {
        if image_w <= 0 || image_h <= 0 || draw_w <= 0.0 || draw_h <= 0.0 {
            return;
        }
        let texture = self.intern_arc_pixels(
            image_w,
            image_h,
            pixels,
            Texture2D::WRAP_CLAMP,
            Texture2D::WRAP_CLAMP,
            Texture2D::FILTER_NEAREST,
        );
        let depth_value = depth_buffer_value(depth, self.camera_near, self.camera_far, self.depth_range_near, self.depth_range_far);
        let u0 = crop_x / image_w as f32;
        let v0 = crop_y / image_h as f32;
        let u1 = (crop_x + crop_w) / image_w as f32;
        let v1 = (crop_y + crop_h) / image_h as f32;
        let x1 = start_x + draw_w;
        let y1 = start_y + draw_h;
        let vertex = |x: f32, y: f32, u: f32, v: f32| M3gGpuVertex {
            x,
            y,
            depth: depth_value,
            camera_z: depth,
            u,
            v,
            u1: 0.0,
            v1: 0.0,
            color,
        };
        let quad = [
            vertex(start_x, start_y, u0, v0),
            vertex(x1, start_y, u1, v0),
            vertex(x1, y1, u1, v1),
            vertex(start_x, y1, u0, v1),
        ];
        for vertices in [[quad[0], quad[1], quad[2]], [quad[0], quad[2], quad[3]]] {
            self.triangles.push(M3gGpuTriangle {
                vertices,
                tex0: M3gGpuTex::replace(texture),
                tex1: M3gGpuTex::none(),
                blending,
                depth_test: self.depth_enabled && depth_test,
                depth_write: self.depth_enabled && depth_write,
                alpha_threshold,
                fog: M3gGpuFog::from_state(fog),
            });
        }
    }

    pub fn finish(self, color_clear: bool) -> bool {
        if self.triangles.is_empty() {
            return false;
        }
        let frame = M3gGpuFrame {
            generation: M3G_GPU_FRAME_GENERATION.fetch_add(1, Ordering::Relaxed),
            width: self.width,
            height: self.height,
            viewport_x: self.viewport_x,
            viewport_y: self.viewport_y,
            clear_depth: self.clear_depth,
            color_clear,
            textures: self.textures,
            triangles: self.triangles,
        };
        let mut slot = M3G_GPU_FRAME.lock();
        if color_clear {
            *slot = Some(Arc::new(frame));
        } else if let Some(existing) = slot.as_ref() {
            *slot = Some(Arc::new(merge_frames(existing, frame)));
        } else {
            *slot = Some(Arc::new(frame));
        }
        true
    }

    fn intern_texture(&mut self, texture: &M3gTexture) -> usize {
        self.intern_arc_pixels(
            texture.width,
            texture.height,
            &texture.pixels,
            texture.wrap_s,
            texture.wrap_t,
            texture.image_filter,
        )
    }

    fn intern_arc_pixels(&mut self, width: i32, height: i32, pixels: &Arc<Vec<i32>>, wrap_s: i32, wrap_t: i32, filter: i32) -> usize {
        let key = Arc::as_ptr(pixels) as usize;
        if let Some((_, index)) = self.texture_keys.iter().copied().find(|(stored, _)| *stored == key) {
            return index;
        }
        let index = self.push_rgba_texture(width, height, pixels, wrap_s, wrap_t, filter);
        self.texture_keys.push((key, index));
        index
    }

    fn push_rgba_texture(&mut self, width: i32, height: i32, pixels: &[i32], wrap_s: i32, wrap_t: i32, filter: i32) -> usize {
        let mut rgba = Vec::with_capacity((width.max(0) as usize).saturating_mul(height.max(0) as usize).saturating_mul(4));
        for color in pixels.iter().copied() {
            let color = color as u32;
            rgba.push(((color >> 16) & 0xff) as u8);
            rgba.push(((color >> 8) & 0xff) as u8);
            rgba.push((color & 0xff) as u8);
            rgba.push(((color >> 24) & 0xff) as u8);
        }
        let index = self.textures.len();
        self.textures.push(M3gGpuTexture {
            width,
            height,
            rgba,
            wrap_s,
            wrap_t,
            filter,
        });
        index
    }

    fn push_viewport_quad(
        &mut self,
        tex0: M3gGpuTex,
        color: i32,
        depth: f32,
        u0: f32,
        v0: f32,
        u1: f32,
        v1: f32,
        depth_test: bool,
        depth_write: bool,
        alpha_threshold: f32,
    ) {
        let width = self.width as f32;
        let height = self.height as f32;
        let vertex = |x: f32, y: f32, u: f32, v: f32| M3gGpuVertex {
            x,
            y,
            depth,
            camera_z: 0.0,
            u,
            v,
            u1: 0.0,
            v1: 0.0,
            color,
        };
        let quad = [
            vertex(0.0, 0.0, u0, v0),
            vertex(width, 0.0, u1, v0),
            vertex(width, height, u1, v1),
            vertex(0.0, height, u0, v1),
        ];
        for vertices in [[quad[0], quad[1], quad[2]], [quad[0], quad[2], quad[3]]] {
            self.triangles.push(M3gGpuTriangle {
                vertices,
                tex0,
                tex1: M3gGpuTex::none(),
                blending: CompositingMode::REPLACE,
                depth_test,
                depth_write,
                alpha_threshold,
                fog: M3gGpuFog::default(),
            });
        }
    }
}

fn merge_frames(base: &M3gGpuFrame, extra: M3gGpuFrame) -> M3gGpuFrame {
    let mut textures = base.textures.clone();
    let offset = textures.len();
    textures.extend(extra.textures);
    let mut triangles = base.triangles.clone();
    for mut triangle in extra.triangles {
        if let Some(index) = triangle.tex0.index.as_mut() {
            *index += offset;
        }
        if let Some(index) = triangle.tex1.index.as_mut() {
            *index += offset;
        }
        triangles.push(triangle);
    }
    M3gGpuFrame {
        generation: extra.generation,
        width: extra.width,
        height: extra.height,
        viewport_x: extra.viewport_x,
        viewport_y: extra.viewport_y,
        clear_depth: extra.clear_depth,
        color_clear: base.color_clear || extra.color_clear,
        textures,
        triangles,
    }
}

pub fn latest_m3g_gpu_frame_after(generation: u64) -> Option<Arc<M3gGpuFrame>> {
    let frame = M3G_GPU_FRAME.lock();
    let frame = frame.as_ref()?;
    (frame.generation > generation).then(|| frame.clone())
}

pub fn clear_m3g_gpu_frames() {
    *M3G_GPU_FRAME.lock() = None;
}

pub(super) fn m3g_gpu_enabled() -> bool {
    gpu_scene_enabled()
}
