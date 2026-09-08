use alloc::{sync::Arc, vec::Vec};
use core::{
    hash::{Hash, Hasher},
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use jvm::{Array, ClassInstanceRef};
use parking_lot::Mutex;

use super::{constants::MAT_BLEND_MASK, raster::SceneTri, texture::NativeTexture};

static V3_GPU_SCENE_ENABLED: AtomicBool = AtomicBool::new(false);
static V3_GPU_FRAME_GENERATION: AtomicU64 = AtomicU64::new(1);
static V3_GPU_FRAME: Mutex<Option<Arc<V3GpuFrame>>> = Mutex::new(None);
static V3_GPU_IMAGE_FRAMES: Mutex<Vec<(u64, Arc<V3GpuFrame>)>> = Mutex::new(Vec::new());

#[derive(Clone)]
pub struct V3GpuFrame {
    pub generation: u64,
    pub width: i32,
    pub height: i32,
    pub textures: Vec<Option<V3GpuTexture>>,
    pub triangles: Vec<V3GpuTriangle>,
}

#[derive(Clone)]
pub struct V3GpuTexture {
    pub width: i32,
    pub height: i32,
    pub rgba: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct V3GpuVertex {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub u: i32,
    pub v: i32,
}

#[derive(Clone)]
pub struct V3GpuTriangle {
    pub texture: Option<usize>,
    pub color: i32,
    pub blend_mode: u8,
    pub vertices: [V3GpuVertex; 3],
}

fn register_lcdui_gpu_hooks() {
    crate::classes::javax::microedition::lcdui::register_gpu_image_hooks(invalidate_gpu_image, publish_gpu_image_to_screen);
}

pub fn set_gpu_scene_enabled(enabled: bool) {
    register_lcdui_gpu_hooks();
    V3_GPU_SCENE_ENABLED.store(enabled, Ordering::Relaxed);
    if !enabled {
        *V3_GPU_FRAME.lock() = None;
        V3_GPU_IMAGE_FRAMES.lock().clear();
        crate::classes::javax::microedition::m3g::clear_m3g_gpu_frames();
    }
}

pub fn gpu_scene_enabled() -> bool {
    V3_GPU_SCENE_ENABLED.load(Ordering::Relaxed)
}

pub fn latest_gpu_frame_after(generation: u64) -> Option<Arc<V3GpuFrame>> {
    let frame = V3_GPU_FRAME.lock();
    let frame = frame.as_ref()?;
    (frame.generation > generation).then(|| frame.clone())
}

pub(super) fn gpu_scene_rejection_reason(width: i32, height: i32, base_clip: (i32, i32, i32, i32), triangles: &[SceneTri]) -> Option<&'static str> {
    if !gpu_scene_enabled() {
        return Some("disabled");
    }
    if width <= 0 || height <= 0 {
        return Some("size");
    }
    if triangles.is_empty() {
        return Some("empty");
    }
    if base_clip != (0, 0, width, height) {
        return Some("clip");
    }
    triangles.iter().find_map(|scene_tri| gpu_tri_rejection_reason(scene_tri, width, height))
}

pub(super) fn publish_gpu_scene(
    width: i32,
    height: i32,
    base_clip: (i32, i32, i32, i32),
    textures: &[Option<NativeTexture>],
    triangles: &[SceneTri],
) -> bool {
    let Some(frame) = build_gpu_frame(width, height, base_clip, textures, triangles) else {
        return false;
    };
    *V3_GPU_FRAME.lock() = Some(Arc::new(frame));
    true
}

#[allow(dead_code)]
pub(super) fn publish_gpu_image_scene(
    pixels: &ClassInstanceRef<Array<i32>>,
    width: i32,
    height: i32,
    base_clip: (i32, i32, i32, i32),
    textures: &[Option<NativeTexture>],
    triangles: &[SceneTri],
) -> bool {
    let Some(key) = image_pixels_key(pixels) else {
        return false;
    };
    let Some(frame) = build_gpu_frame(width, height, base_clip, textures, triangles) else {
        return false;
    };

    let mut frames = V3_GPU_IMAGE_FRAMES.lock();
    if let Some((_, stored_frame)) = frames.iter_mut().find(|(stored_key, _)| *stored_key == key) {
        *stored_frame = Arc::new(frame);
    } else {
        frames.push((key, Arc::new(frame)));
    }
    true
}

#[allow(clippy::too_many_arguments)]
pub fn publish_gpu_image_to_screen(
    pixels: &ClassInstanceRef<Array<i32>>,
    screen_width: i32,
    screen_height: i32,
    dest_x: i32,
    dest_y: i32,
    src_x: i32,
    src_y: i32,
    width: i32,
    height: i32,
    clip: (i32, i32, i32, i32),
) -> bool {
    if !gpu_scene_enabled() || screen_width <= 0 || screen_height <= 0 {
        return false;
    }
    if dest_x != 0 || dest_y != 0 || src_x != 0 || src_y != 0 {
        return false;
    }
    if clip != (0, 0, screen_width, screen_height) {
        return false;
    }

    let Some(key) = image_pixels_key(pixels) else {
        return false;
    };
    let frames = V3_GPU_IMAGE_FRAMES.lock();
    let Some(frame) = frames.iter().find_map(|(stored_key, frame)| (*stored_key == key).then(|| frame.clone())) else {
        return false;
    };
    if frame.width != width || frame.height != height || frame.width != screen_width || frame.height != screen_height {
        return false;
    }

    *V3_GPU_FRAME.lock() = Some(frame);
    true
}

pub fn invalidate_gpu_image(pixels: &ClassInstanceRef<Array<i32>>) {
    let Some(key) = image_pixels_key(pixels) else {
        return;
    };
    V3_GPU_IMAGE_FRAMES.lock().retain(|(stored_key, _)| *stored_key != key);
}

fn build_gpu_frame(
    width: i32,
    height: i32,
    base_clip: (i32, i32, i32, i32),
    textures: &[Option<NativeTexture>],
    triangles: &[SceneTri],
) -> Option<V3GpuFrame> {
    if !gpu_scene_enabled() || width <= 0 || height <= 0 || triangles.is_empty() {
        return None;
    }
    if base_clip != (0, 0, width, height) {
        return None;
    }
    if !triangles.iter().all(|scene_tri| can_gpu_draw_scene_tri(scene_tri, width, height)) {
        return None;
    }

    Some(V3GpuFrame {
        generation: V3_GPU_FRAME_GENERATION.fetch_add(1, Ordering::Relaxed),
        width,
        height,
        textures: textures.iter().map(convert_texture).collect(),
        triangles: triangles
            .iter()
            .map(|scene_tri| V3GpuTriangle {
                texture: scene_tri.tri.texture,
                color: scene_tri.tri.color,
                blend_mode: gpu_blend_mode(scene_tri),
                vertices: scene_tri.tri.vertices.clone().map(|vertex| V3GpuVertex {
                    x: vertex.x,
                    y: vertex.y,
                    z: vertex.z,
                    u: vertex.u,
                    v: vertex.v,
                }),
            })
            .collect(),
    })
}

fn can_gpu_draw_scene_tri(scene_tri: &SceneTri, width: i32, height: i32) -> bool {
    gpu_tri_rejection_reason(scene_tri, width, height).is_none()
}

fn gpu_tri_rejection_reason(scene_tri: &SceneTri, width: i32, height: i32) -> Option<&'static str> {
    if scene_tri.effect_transparency {
        return Some("effect-alpha");
    }
    if (scene_tri.tri.mat & MAT_BLEND_MASK) != 0 {
        return Some("mat-blend");
    }
    if scene_tri.clip.is_some_and(|clip| clip != (0, 0, width, height)) {
        return Some("tri-clip");
    }
    None
}

fn gpu_blend_mode(scene_tri: &SceneTri) -> u8 {
    if scene_tri.effect_transparency {
        ((scene_tri.tri.mat & MAT_BLEND_MASK) >> 1).clamp(0, 3) as u8
    } else {
        0
    }
}

fn convert_texture(texture: &Option<NativeTexture>) -> Option<V3GpuTexture> {
    let texture = texture.as_ref()?;
    if texture.width <= 0 || texture.height <= 0 {
        return None;
    }

    let width = texture.width;
    let height = texture.height;
    let texel_count = (width as usize).saturating_mul(height as usize);
    let mut rgba = Vec::with_capacity(texel_count.saturating_mul(4));
    let color_key = texture.color_key & 0x00ff_ffff;

    for index in 0..texel_count {
        let color = if !texture.indices.is_empty() && !texture.palette.is_empty() {
            let palette_index = texture.indices.get(index).copied().unwrap_or(0);
            if palette_index == 0 {
                0
            } else {
                texture.palette.get(palette_index as usize).copied().unwrap_or(0)
            }
        } else {
            texture.pixels.get(index).copied().unwrap_or(0)
        };

        let alpha = if (color as u32 >> 24) == 0 || (color & 0x00ff_ffff) == color_key {
            0
        } else {
            0xff
        };
        rgba.push(((color >> 16) & 0xff) as u8);
        rgba.push(((color >> 8) & 0xff) as u8);
        rgba.push((color & 0xff) as u8);
        rgba.push(alpha);
    }

    Some(V3GpuTexture { width, height, rgba })
}

fn image_pixels_key(pixels: &ClassInstanceRef<Array<i32>>) -> Option<u64> {
    let instance = pixels.instance.as_ref()?;
    let mut hasher = ObjectIdentityHasher::default();
    instance.hash(&mut hasher);
    Some(hasher.finish())
}

#[derive(Default)]
struct ObjectIdentityHasher(u64);

impl Hasher for ObjectIdentityHasher {
    fn write(&mut self, bytes: &[u8]) {
        if self.0 == 0 {
            self.0 = 0xcbf2_9ce4_8422_2325;
        }
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn write_u8(&mut self, i: u8) {
        self.write(&[i]);
    }

    fn write_usize(&mut self, i: usize) {
        self.write(&i.to_ne_bytes());
    }

    fn finish(&self) -> u64 {
        self.0
    }
}
