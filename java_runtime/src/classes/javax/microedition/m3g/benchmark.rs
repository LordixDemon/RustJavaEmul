use alloc::{sync::Arc, vec, vec::Vec};
use core::hint::black_box;

use super::{
    CompositingMode, Fog, Image2D, Light, PolygonMode, Texture2D,
    math::{identity_matrix, invert_matrix, multiply_matrix, rotation_matrix, scale_matrix, transform_point, translation_matrix},
    render::{
        M3gAppearanceState, M3gDrawVertex, M3gFogState, M3gLightState, M3gMaterialState, M3gSceneLighting, M3gTexture, camera_vertex,
        clip_triangle_near, default_appearance_state, rasterize_triangle, sample_texture, shade_vertex,
    },
};

const DEFAULT_SCREEN_WIDTH: i32 = 240;
const DEFAULT_SCREEN_HEIGHT: i32 = 320;
const DEFAULT_GRID_SIZE: usize = 18;
const DEFAULT_FRAMES: u32 = 120;
const DEFAULT_TEXTURE_SIZE: usize = 64;

#[derive(Clone, Copy, Debug)]
pub struct M3gBenchmarkConfig {
    pub screen_width: i32,
    pub screen_height: i32,
    pub grid_size: usize,
    pub frames: u32,
    pub texture_size: usize,
}

impl Default for M3gBenchmarkConfig {
    fn default() -> Self {
        Self {
            screen_width: DEFAULT_SCREEN_WIDTH,
            screen_height: DEFAULT_SCREEN_HEIGHT,
            grid_size: DEFAULT_GRID_SIZE,
            frames: DEFAULT_FRAMES,
            texture_size: DEFAULT_TEXTURE_SIZE,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct M3gBenchmarkRun {
    pub frames: u32,
    pub ops: u64,
    pub checksum: u64,
}

pub struct M3gBenchmark {
    config: M3gBenchmarkConfig,
    texture_nearest: M3gTexture,
    texture_linear: M3gTexture,
    texture_unit1: M3gTexture,
    lighting: M3gSceneLighting,
}

impl M3gBenchmark {
    pub fn new(config: M3gBenchmarkConfig) -> Self {
        let config = normalize_config(config);
        Self {
            texture_nearest: synthetic_texture(config.texture_size, Texture2D::FILTER_NEAREST, Texture2D::FUNC_MODULATE),
            texture_linear: synthetic_texture(config.texture_size, Texture2D::FILTER_LINEAR, Texture2D::FUNC_DECAL),
            texture_unit1: synthetic_texture(config.texture_size, Texture2D::FILTER_NEAREST, Texture2D::FUNC_ADD),
            lighting: scene_lighting(),
            config,
        }
    }

    pub fn config(&self) -> M3gBenchmarkConfig {
        self.config
    }

    pub fn triangle_count(&self) -> usize {
        self.config.grid_size.saturating_mul(self.config.grid_size).saturating_mul(2)
    }

    pub fn vertex_count(&self) -> usize {
        (self.config.grid_size + 1).saturating_mul(self.config.grid_size + 1)
    }

    pub fn run_math(&self) -> M3gBenchmarkRun {
        let mut run = self.empty_run();
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        for frame in 0..self.config.frames {
            let angle = (frame as f32) * 3.0;
            let rotate = rotation_matrix(angle, 0.15, 0.85, 0.35);
            let scale = scale_matrix(1.05, 0.95, 1.1);
            let translate = translation_matrix(8.0, -4.0, 24.0 + frame as f32);
            let model = multiply_matrix(translate, multiply_matrix(rotate, scale));
            let inverted = invert_matrix(model).unwrap_or(identity_matrix());
            checksum = mix_f32(checksum, model[0]);
            checksum = mix_f32(checksum, inverted[15]);
            for vertex in grid_points(self.config.grid_size, frame) {
                let transformed = transform_point(model, vertex);
                checksum = mix_f32(checksum, transformed[0] + transformed[1] + transformed[2]);
                ops = ops.saturating_add(1);
            }
            black_box(&model);
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    pub fn run_lighting(&self) -> M3gBenchmarkRun {
        let mut run = self.empty_run();
        let mut appearance = default_appearance_state(0xffc8c8c8u32 as i32);
        appearance.material = Some(M3gMaterialState {
            ambient: 0xff303030u32 as i32,
            diffuse: 0xffd0a060u32 as i32,
            emissive: 0xff101010u32 as i32,
            specular: 0xffffffffu32 as i32,
            shininess: 48.0,
            vertex_color_tracking: false,
        });
        appearance.two_sided_lighting = true;
        appearance.local_camera_lighting = true;
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        for frame in 0..self.config.frames {
            for (index, position) in grid_points(self.config.grid_size, frame).into_iter().enumerate() {
                let normal = normalize_or_up([(index % 7) as f32 * 0.1 - 0.3, 0.4, 1.0 - (index % 5) as f32 * 0.15]);
                let color = shade_vertex(position, Some(normal), 0xffff8080u32 as i32, &appearance, &self.lighting, -1);
                checksum = mix(checksum, color as u32 as u64);
                ops = ops.saturating_add(1);
            }
        }
        black_box(&appearance);
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    pub fn run_texture(&self) -> M3gBenchmarkRun {
        let mut run = self.empty_run();
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        let samples = self.config.texture_size.saturating_mul(4).max(16);
        for frame in 0..self.config.frames {
            for y in 0..samples {
                for x in 0..samples {
                    let u = (x as f32 + frame as f32 * 0.01) / samples as f32;
                    let v = (y as f32 + frame as f32 * 0.013) / samples as f32;
                    checksum = mix(checksum, sample_texture(&self.texture_nearest, u, v) as u32 as u64);
                    checksum = mix(checksum, sample_texture(&self.texture_linear, u + 0.5, v + 0.25) as u32 as u64);
                    ops = ops.saturating_add(2);
                }
            }
        }
        black_box(&self.texture_nearest);
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    pub fn run_clip(&self) -> M3gBenchmarkRun {
        let mut run = self.empty_run();
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        let model_view = translation_matrix(0.0, 0.0, -8.0);
        for frame in 0..self.config.frames {
            for tri in screen_triangles(self.config, frame) {
                let Some(v0) = camera_vertex(
                    [tri.0.x, tri.0.y, tri.0.z],
                    [tri.0.u, tri.0.v],
                    [tri.0.u1, tri.0.v1],
                    tri.0.color,
                    model_view,
                ) else {
                    continue;
                };
                let Some(v1) = camera_vertex(
                    [tri.1.x, tri.1.y, tri.1.z],
                    [tri.1.u, tri.1.v],
                    [tri.1.u1, tri.1.v1],
                    tri.1.color,
                    model_view,
                ) else {
                    continue;
                };
                let Some(v2) = camera_vertex(
                    [tri.2.x, tri.2.y, tri.2.z],
                    [tri.2.u, tri.2.v],
                    [tri.2.u1, tri.2.v1],
                    tri.2.color,
                    model_view,
                ) else {
                    continue;
                };
                let clipped = clip_triangle_near(v0, v1, v2, 1.0);
                checksum = mix(checksum, clipped.len() as u64);
                if let Some(first) = clipped.first() {
                    checksum = mix_f32(checksum, first.depth);
                }
                ops = ops.saturating_add(1);
            }
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    pub fn run_raster_flat(&self) -> M3gBenchmarkRun {
        self.raster_with(self.flat_appearance(), false)
    }

    pub fn run_raster_textured(&self) -> M3gBenchmarkRun {
        self.raster_with(self.textured_appearance(false), false)
    }

    pub fn run_raster_blend(&self) -> M3gBenchmarkRun {
        let mut appearance = self.textured_appearance(false);
        appearance.blending = CompositingMode::ALPHA;
        self.raster_with(appearance, false)
    }

    pub fn run_raster_fog(&self) -> M3gBenchmarkRun {
        let mut appearance = self.textured_appearance(true);
        appearance.fog = Some(M3gFogState {
            mode: Fog::LINEAR,
            color: 0xff102030u32 as i32,
            density: 0.08,
            near: 1.0,
            far: 24.0,
        });
        self.raster_with(appearance, false)
    }

    pub fn run_full_frame(&self) -> M3gBenchmarkRun {
        let mut appearance = self.textured_appearance(true);
        appearance.material = Some(M3gMaterialState {
            ambient: 0xff202020u32 as i32,
            diffuse: 0xffe0c080u32 as i32,
            emissive: 0,
            specular: 0xff808080u32 as i32,
            shininess: 16.0,
            vertex_color_tracking: true,
        });
        appearance.fog = Some(M3gFogState {
            mode: Fog::EXPONENTIAL,
            color: 0xff081018u32 as i32,
            density: 0.04,
            near: 1.0,
            far: 32.0,
        });
        appearance.blending = CompositingMode::MODULATE;
        self.raster_with(appearance, true)
    }

    fn raster_with(&self, appearance: M3gAppearanceState, relight: bool) -> M3gBenchmarkRun {
        let mut run = self.empty_run();
        let width = self.config.screen_width;
        let height = self.config.screen_height;
        let mut pixels = vec![0; (width as usize).saturating_mul(height as usize)];
        let mut depth = vec![1.0f32; pixels.len()];
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            depth.fill(1.0);
            let mut dirty = Default::default();
            for mut tri in screen_triangles(self.config, frame) {
                if relight {
                    for vertex in [&mut tri.0, &mut tri.1, &mut tri.2] {
                        vertex.color = shade_vertex(
                            [vertex.x * 0.01, vertex.y * 0.01, vertex.z],
                            Some([0.0, 0.0, 1.0]),
                            vertex.color,
                            &appearance,
                            &self.lighting,
                            -1,
                        );
                    }
                }
                let written = rasterize_triangle(
                    &mut pixels,
                    &mut depth,
                    width,
                    height,
                    tri.0,
                    tri.1,
                    tri.2,
                    &appearance,
                    true,
                    1.0,
                    32.0,
                    0.0,
                    1.0,
                    &mut dirty,
                );
                ops = ops.saturating_add(1);
                checksum = mix(checksum, written as u64);
            }
            checksum = checksum_pixels(checksum, &pixels, frame);
            black_box((&pixels, &depth));
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    fn flat_appearance(&self) -> M3gAppearanceState {
        let mut appearance = default_appearance_state(0xff80a0c0u32 as i32);
        appearance.culling = PolygonMode::CULL_NONE;
        appearance.perspective_correction = false;
        appearance
    }

    fn textured_appearance(&self, two_units: bool) -> M3gAppearanceState {
        let mut appearance = default_appearance_state(0xffffffffu32 as i32);
        appearance.culling = PolygonMode::CULL_NONE;
        appearance.texture = Some(clone_texture(&self.texture_nearest));
        if two_units {
            appearance.texture1 = Some(clone_texture(&self.texture_unit1));
        }
        appearance.perspective_correction = true;
        appearance
    }

    fn empty_run(&self) -> M3gBenchmarkRun {
        M3gBenchmarkRun {
            frames: self.config.frames,
            ops: 0,
            checksum: 0xcbf2_9ce4_8422_2325,
        }
    }
}

fn normalize_config(config: M3gBenchmarkConfig) -> M3gBenchmarkConfig {
    M3gBenchmarkConfig {
        screen_width: config.screen_width.clamp(32, 2048),
        screen_height: config.screen_height.clamp(32, 2048),
        grid_size: config.grid_size.clamp(2, 64),
        frames: config.frames.clamp(1, 10_000),
        texture_size: config.texture_size.clamp(2, 256).next_power_of_two(),
    }
}

fn synthetic_texture(size: usize, filter: i32, blending: i32) -> M3gTexture {
    let mut pixels = vec![0; size.saturating_mul(size)];
    for y in 0..size {
        for x in 0..size {
            let a = if (x + y) % 11 == 0 { 0x80 } else { 0xff };
            let r = (x * 13) & 0xff;
            let g = (y * 17) & 0xff;
            let b = ((x * 5) ^ (y * 9)) & 0xff;
            pixels[y * size + x] = ((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32) as i32;
        }
    }
    M3gTexture {
        width: size as i32,
        height: size as i32,
        pixels: Arc::new(pixels),
        format: Image2D::RGBA,
        blend_color: 0xff224466u32 as i32,
        blending,
        wrap_s: Texture2D::WRAP_REPEAT,
        wrap_t: Texture2D::WRAP_CLAMP,
        image_filter: filter,
        transform: identity_matrix(),
        transform_identity: true,
    }
}

fn clone_texture(texture: &M3gTexture) -> M3gTexture {
    M3gTexture {
        width: texture.width,
        height: texture.height,
        pixels: texture.pixels.clone(),
        format: texture.format,
        blend_color: texture.blend_color,
        blending: texture.blending,
        wrap_s: texture.wrap_s,
        wrap_t: texture.wrap_t,
        image_filter: texture.image_filter,
        transform: texture.transform,
        transform_identity: texture.transform_identity,
    }
}

fn scene_lighting() -> M3gSceneLighting {
    M3gSceneLighting {
        ambient_r: 0.15,
        ambient_g: 0.16,
        ambient_b: 0.18,
        ambient_lights: 1,
        lights: vec![
            M3gLightState {
                mode: Light::DIRECTIONAL,
                color: [1.0, 0.95, 0.85],
                position: [0.0, 0.0, 0.0],
                direction: [0.2, -0.4, -0.9],
                constant: 1.0,
                linear: 0.0,
                quadratic: 0.0,
                spot_cos: 0.0,
                spot_exponent: 0.0,
                scope: -1,
            },
            M3gLightState {
                mode: Light::OMNI,
                color: [0.4, 0.6, 1.0],
                position: [4.0, 6.0, 8.0],
                direction: [0.0, 0.0, -1.0],
                constant: 1.0,
                linear: 0.05,
                quadratic: 0.01,
                spot_cos: 0.0,
                spot_exponent: 0.0,
                scope: -1,
            },
            M3gLightState {
                mode: Light::SPOT,
                color: [1.0, 0.3, 0.2],
                position: [-3.0, 2.0, 5.0],
                direction: [0.3, -0.2, -1.0],
                constant: 1.0,
                linear: 0.02,
                quadratic: 0.0,
                spot_cos: 0.7,
                spot_exponent: 8.0,
                scope: -1,
            },
        ],
    }
}

fn grid_points(grid: usize, frame: u32) -> Vec<[f32; 3]> {
    let mut points = Vec::with_capacity((grid + 1).saturating_mul(grid + 1));
    let half = grid as f32 * 0.5;
    for y in 0..=grid {
        for x in 0..=grid {
            let wave = ((x * 17 + y * 31 + frame as usize * 3) % 11) as f32 - 5.0;
            points.push([(x as f32 - half) * 2.0, (y as f32 - half) * 2.0, 6.0 + wave * 0.35]);
        }
    }
    points
}

fn screen_triangles(config: M3gBenchmarkConfig, frame: u32) -> Vec<(M3gDrawVertex, M3gDrawVertex, M3gDrawVertex)> {
    let grid = config.grid_size;
    let cell_w = config.screen_width as f32 / grid as f32;
    let cell_h = config.screen_height as f32 / grid as f32;
    let mut triangles = Vec::with_capacity(grid.saturating_mul(grid).saturating_mul(2));
    let shift = ((frame % 7) as f32) * 0.4;
    for y in 0..grid {
        for x in 0..grid {
            let x0 = x as f32 * cell_w + shift;
            let y0 = y as f32 * cell_h;
            let x1 = x0 + cell_w;
            let y1 = y0 + cell_h;
            let z = 2.5 + ((x + y + frame as usize) % 5) as f32 * 0.4;
            let color = vertex_color(x, y, frame);
            let v00 = draw_vertex(x0, y0, z, 0.0, 0.0, color);
            let v10 = draw_vertex(x1, y0, z + 0.1, 1.0, 0.0, color.rotate_left(8));
            let v01 = draw_vertex(x0, y1, z + 0.05, 0.0, 1.0, color.rotate_left(16));
            let v11 = draw_vertex(x1, y1, z + 0.2, 1.0, 1.0, color);
            triangles.push((v00, v10, v01));
            triangles.push((v10, v11, v01));
        }
    }
    triangles
}

fn draw_vertex(x: f32, y: f32, z: f32, u: f32, v: f32, color: i32) -> M3gDrawVertex {
    M3gDrawVertex {
        x,
        y,
        z,
        u,
        v,
        u1: 1.0 - u,
        v1: v * 0.5,
        color,
    }
}

fn vertex_color(x: usize, y: usize, frame: u32) -> i32 {
    let r = (40 + (x * 17 + frame as usize) % 180) as u32;
    let g = (30 + (y * 13 + frame as usize * 3) % 180) as u32;
    let b = (50 + ((x + y) * 11) % 160) as u32;
    let a = if (x + y) % 5 == 0 { 0x90 } else { 0xff };
    ((a << 24) | (r << 16) | (g << 8) | b) as i32
}

fn fill_background(pixels: &mut [i32], frame: u32) {
    let r = 8 + frame.wrapping_mul(2) % 24;
    let g = 10 + frame.wrapping_mul(3) % 24;
    let b = 16 + frame.wrapping_mul(5) % 32;
    pixels.fill((0xff00_0000 | (r << 16) | (g << 8) | b) as i32);
}

fn checksum_pixels(mut checksum: u64, pixels: &[i32], frame: u32) -> u64 {
    checksum = mix(checksum, frame as u64);
    checksum = mix(checksum, pixels.len() as u64);
    let step = (pixels.len() / 128).max(1);
    for pixel in pixels.iter().step_by(step).take(128) {
        checksum = mix(checksum, *pixel as u32 as u64);
    }
    checksum
}

fn normalize_or_up(value: [f32; 3]) -> [f32; 3] {
    let len = (value[0] * value[0] + value[1] * value[1] + value[2] * value[2]).sqrt();
    if len <= f32::EPSILON {
        [0.0, 0.0, 1.0]
    } else {
        [value[0] / len, value[1] / len, value[2] / len]
    }
}

fn mix_f32(state: u64, value: f32) -> u64 {
    mix(state, value.to_bits() as u64)
}

fn mix(state: u64, value: u64) -> u64 {
    let value = value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    (state ^ value).rotate_left(27).wrapping_mul(0x94d0_49bb_1331_11eb)
}

#[cfg(test)]
mod tests {
    use super::{M3gBenchmark, M3gBenchmarkConfig};

    #[test]
    fn m3g_benchmark_runs_tiny_workload() {
        let benchmark = M3gBenchmark::new(M3gBenchmarkConfig {
            screen_width: 96,
            screen_height: 128,
            grid_size: 4,
            frames: 2,
            texture_size: 16,
        });
        assert!(benchmark.triangle_count() > 0);
        assert_ne!(benchmark.run_math().checksum, 0);
        assert_ne!(benchmark.run_lighting().checksum, 0);
        assert_ne!(benchmark.run_texture().checksum, 0);
        assert_ne!(benchmark.run_clip().checksum, 0);
        assert_ne!(benchmark.run_raster_flat().checksum, 0);
        assert_ne!(benchmark.run_raster_textured().checksum, 0);
        assert_ne!(benchmark.run_raster_blend().checksum, 0);
        assert_ne!(benchmark.run_raster_fog().checksum, 0);
        assert_ne!(benchmark.run_full_frame().checksum, 0);
    }
}
