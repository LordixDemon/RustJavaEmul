use alloc::{sync::Arc, vec, vec::Vec};
use core::hint::black_box;

use super::{
    constants::{MAT_BLEND_MASK, MAT_COLORKEY, MAT_DOUBLE_FACE},
    math::sin_cos_mc,
    raster::{RenderTri, rasterize_triangle_into_pixels},
    scene::{
        ProjectionParams, RuntimeFigure, apply_active_material_mask, collect_render_triangles, mark_implicit_indexed_color_keys,
        project_figure_vertices, triangle_in_depth_range,
    },
    texture::NativeTexture,
};

const DEFAULT_SCREEN_WIDTH: i32 = 240;
const DEFAULT_SCREEN_HEIGHT: i32 = 320;
const DEFAULT_GRID_SIZE: usize = 18;
const DEFAULT_FRAMES: u32 = 120;
const DEFAULT_TEXTURE_SIZE: usize = 64;

#[derive(Clone, Copy, Debug)]
pub struct V3BenchmarkConfig {
    pub screen_width: i32,
    pub screen_height: i32,
    pub grid_size: usize,
    pub frames: u32,
    pub texture_size: usize,
}

impl Default for V3BenchmarkConfig {
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
pub struct V3BenchmarkRun {
    pub frames: u32,
    pub vertices_per_frame: usize,
    pub source_quads: usize,
    pub triangles: u64,
    pub changed_triangles: u64,
    pub checksum: u64,
}

pub struct V3Benchmark {
    config: V3BenchmarkConfig,
    figure: RuntimeFigure,
    textures: Vec<Option<NativeTexture>>,
    projection: ProjectionParams,
}

pub struct V3PreparedBenchmarkFrame {
    triangles: Vec<RenderTri>,
    checksum: u64,
}

impl V3PreparedBenchmarkFrame {
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    pub fn checksum(&self) -> u64 {
        self.checksum
    }
}

impl V3Benchmark {
    pub fn new(config: V3BenchmarkConfig) -> Self {
        let config = normalize_config(config);
        let figure = synthetic_figure(config.grid_size);
        let textures = vec![Some(synthetic_texture(config.texture_size))];
        let projection = ProjectionParams {
            perspective: true,
            near: 64,
            far: 4096,
            scale_x: config.screen_width.max(1),
            scale_y: config.screen_width.max(1),
            center_x: config.screen_width / 2,
            center_y: config.screen_height / 2,
        };

        Self {
            config,
            figure,
            textures,
            projection,
        }
    }

    pub fn config(&self) -> V3BenchmarkConfig {
        self.config
    }

    pub fn source_quads(&self) -> usize {
        self.config.grid_size.saturating_mul(self.config.grid_size)
    }

    pub fn vertex_count(&self) -> usize {
        self.figure.vertex_count
    }

    pub fn prepare_frame(&self, frame_index: u32) -> V3PreparedBenchmarkFrame {
        let projected = project_figure_vertices(&self.figure, frame_view_matrix(frame_index), &self.projection);
        let mut triangles = collect_render_triangles(&self.figure, &projected, &self.textures);
        if self.projection.perspective {
            triangles.retain(|tri| triangle_in_depth_range(tri, &self.projection));
        }
        mark_implicit_indexed_color_keys(&mut triangles, &self.textures);
        apply_active_material_mask(&mut triangles, true);
        triangles.sort_by(|a, b| b.z.cmp(&a.z));

        let checksum = checksum_triangles(0, &triangles);
        black_box(&triangles);
        V3PreparedBenchmarkFrame { triangles, checksum }
    }

    pub fn run_geometry(&self) -> V3BenchmarkRun {
        let mut run = self.empty_run();
        for frame in 0..self.config.frames {
            let prepared = self.prepare_frame(frame);
            run.triangles = run.triangles.saturating_add(prepared.triangles.len() as u64);
            run.checksum = mix(run.checksum, prepared.checksum);
            black_box(&prepared);
        }
        run
    }

    pub fn run_raster(&self, prepared: &V3PreparedBenchmarkFrame) -> V3BenchmarkRun {
        let mut run = self.empty_run();
        run.triangles = (prepared.triangles.len() as u64).saturating_mul(self.config.frames as u64);
        run.checksum = mix(run.checksum, prepared.checksum);

        let mut pixels = vec![0; target_len(self.config)];
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            for tri in &prepared.triangles {
                if rasterize_triangle_into_pixels(
                    tri,
                    &self.textures,
                    &mut pixels,
                    self.config.screen_width,
                    self.config.screen_height,
                    0,
                    0,
                    self.config.screen_width,
                    self.config.screen_height,
                    true,
                ) {
                    run.changed_triangles = run.changed_triangles.saturating_add(1);
                }
            }
            run.checksum = checksum_pixels(run.checksum, &pixels, frame);
            black_box(&pixels);
        }
        run
    }

    pub fn run_full_frame(&self) -> V3BenchmarkRun {
        let mut run = self.empty_run();
        let mut pixels = vec![0; target_len(self.config)];
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            let prepared = self.prepare_frame(frame);
            run.triangles = run.triangles.saturating_add(prepared.triangles.len() as u64);
            run.checksum = mix(run.checksum, prepared.checksum);
            for tri in &prepared.triangles {
                if rasterize_triangle_into_pixels(
                    tri,
                    &self.textures,
                    &mut pixels,
                    self.config.screen_width,
                    self.config.screen_height,
                    0,
                    0,
                    self.config.screen_width,
                    self.config.screen_height,
                    true,
                ) {
                    run.changed_triangles = run.changed_triangles.saturating_add(1);
                }
            }
            run.checksum = checksum_pixels(run.checksum, &pixels, frame);
            black_box(&pixels);
        }
        run
    }

    fn empty_run(&self) -> V3BenchmarkRun {
        V3BenchmarkRun {
            frames: self.config.frames,
            vertices_per_frame: self.figure.vertex_count,
            source_quads: self.source_quads(),
            triangles: 0,
            changed_triangles: 0,
            checksum: 0xcbf2_9ce4_8422_2325,
        }
    }
}

fn normalize_config(config: V3BenchmarkConfig) -> V3BenchmarkConfig {
    V3BenchmarkConfig {
        screen_width: config.screen_width.clamp(32, 2048),
        screen_height: config.screen_height.clamp(32, 2048),
        grid_size: config.grid_size.clamp(2, 64),
        frames: config.frames.clamp(1, 10_000),
        texture_size: config.texture_size.clamp(2, 256).next_power_of_two(),
    }
}

fn synthetic_figure(grid: usize) -> RuntimeFigure {
    let vertex_count = (grid + 1).saturating_mul(grid + 1);
    let mut vertices = Vec::with_capacity(vertex_count.saturating_mul(3));
    let cell = 36i32;
    let half = grid as i32 / 2;
    for y in 0..=grid {
        for x in 0..=grid {
            let vx = (x as i32 - half) * cell;
            let vy = (y as i32 - half) * cell;
            let wave = (((x * 17 + y * 31) % 11) as i32 - 5) * 7;
            vertices.push(vx as i16);
            vertices.push(vy as i16);
            vertices.push(wave as i16);
        }
    }

    let mut poly_c4 = Vec::new();
    let mut poly_t4 = Vec::with_capacity(grid.saturating_mul(grid).saturating_mul(9));
    let colors = vec![
        0xff48_8cc8u32 as i32,
        0xffd8_b858u32 as i32,
        0xff70_a050u32 as i32,
        0xffc8_6858u32 as i32,
        0xffb0_b0b0u32 as i32,
        0xff58_b8b8u32 as i32,
    ];

    for y in 0..grid {
        for x in 0..grid {
            let row = grid + 1;
            let v0 = (y * row + x) as i16;
            let v1 = (y * row + x + 1) as i16;
            let v2 = ((y + 1) * row + x) as i16;
            let v3 = ((y + 1) * row + x + 1) as i16;
            let blend = match (x + y) & 3 {
                0 => 0,
                1 => 0x02,
                2 => 0x04,
                _ => 0x06,
            } & MAT_BLEND_MASK;
            let mat = MAT_DOUBLE_FACE | MAT_COLORKEY | blend;
            poly_t4.extend_from_slice(&[
                mat as i16,
                v0,
                v1,
                v2,
                v3,
                pack_uv(0, 0),
                pack_uv(255, 0),
                pack_uv(0, 255),
                pack_uv(255, 255),
            ]);

            if (x + y) % 3 == 0 {
                let color = ((x + y) % colors.len()) as i16;
                poly_c4.extend_from_slice(&[(MAT_DOUBLE_FACE | blend) as i16, color, v0, v1, v2, v3]);
            }
        }
    }

    let color_quads = poly_c4.len() / 6;
    let texture_quads = poly_t4.len() / 9;
    RuntimeFigure {
        vertex_count,
        vertices: Arc::new(vertices),
        poly_c3: Arc::new(Vec::new()),
        poly_c4: Arc::new(poly_c4),
        poly_t3: Arc::new(Vec::new()),
        poly_t4: Arc::new(poly_t4),
        colors: Arc::new(colors),
        patterns: Arc::new(vec![0, 0, 0, color_quads as i32, 0, 0, 0, texture_quads as i32]),
        pattern_count: 1,
        pattern_slots: 2,
        bones: Arc::new(Vec::new()),
        posture_bones: Vec::new(),
        selected_pattern: 0,
        texture_index: -1,
    }
}

fn synthetic_texture(size: usize) -> NativeTexture {
    let mut palette = vec![0xff00_0000u32 as i32; 256];
    for (index, color) in palette.iter_mut().enumerate().skip(1) {
        let r = ((index * 37) & 0xff) as u32;
        let g = ((index * 67) & 0xff) as u32;
        let b = ((index * 97) & 0xff) as u32;
        *color = (0xff00_0000 | (r << 16) | (g << 8) | b) as i32;
    }

    let mut indices = vec![0; size.saturating_mul(size)];
    for y in 0..size {
        for x in 0..size {
            let edge = x == 0 || y == 0 || x + 1 == size || y + 1 == size;
            let transparent = edge || ((x * 13 + y * 7) % 29 == 0);
            indices[y * size + x] = if transparent { 0 } else { ((x * 5 + y * 3) % 255 + 1) as u8 };
        }
    }

    NativeTexture {
        width: size as i32,
        height: size as i32,
        pixels: Arc::new(Vec::new()),
        indices: Arc::new(indices),
        palette: Arc::new(palette),
        color_key: 0xff00_0000u32 as i32,
    }
}

fn frame_view_matrix(frame: u32) -> [i32; 12] {
    let angle = ((frame as i32) * 37).rem_euclid(4096);
    let (sin, cos) = sin_cos_mc(angle);
    [cos, 0, sin, 0, 0, 4096, 0, 0, -sin, 0, cos, 1240]
}

fn pack_uv(u: i32, v: i32) -> i16 {
    (((u.clamp(0, 255) << 8) | v.clamp(0, 255)) as u16) as i16
}

fn target_len(config: V3BenchmarkConfig) -> usize {
    (config.screen_width as usize).saturating_mul(config.screen_height as usize)
}

fn fill_background(pixels: &mut [i32], frame: u32) {
    let r = 16 + frame.wrapping_mul(3) % 48;
    let g = 20 + frame.wrapping_mul(5) % 48;
    let b = 24 + frame.wrapping_mul(7) % 48;
    let color = (0xff00_0000 | (r << 16) | (g << 8) | b) as i32;
    pixels.fill(color);
}

fn checksum_triangles(mut checksum: u64, triangles: &[RenderTri]) -> u64 {
    checksum = mix(checksum, triangles.len() as u64);
    let step = (triangles.len() / 32).max(1);
    for tri in triangles.iter().step_by(step) {
        checksum = mix(checksum, tri.z as u32 as u64);
        checksum = mix(checksum, tri.mat as u32 as u64);
        checksum = mix(checksum, tri.color as u32 as u64);
        for vertex in &tri.vertices {
            checksum = mix(checksum, vertex.x as u32 as u64);
            checksum = mix(checksum, vertex.y as u32 as u64);
            checksum = mix(checksum, vertex.z as u32 as u64);
            checksum = mix(checksum, vertex.u as u32 as u64);
            checksum = mix(checksum, vertex.v as u32 as u64);
        }
    }
    checksum
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

fn mix(state: u64, value: u64) -> u64 {
    let value = value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    (state ^ value).rotate_left(27).wrapping_mul(0x94d0_49bb_1331_11eb)
}

#[cfg(test)]
mod tests {
    use super::{V3Benchmark, V3BenchmarkConfig};

    #[test]
    fn v3_benchmark_runs_tiny_workload() {
        let benchmark = V3Benchmark::new(V3BenchmarkConfig {
            screen_width: 96,
            screen_height: 128,
            grid_size: 4,
            frames: 2,
            texture_size: 16,
        });
        let prepared = benchmark.prepare_frame(0);
        assert!(prepared.triangle_count() > 0);

        let geometry = benchmark.run_geometry();
        let raster = benchmark.run_raster(&prepared);
        let full = benchmark.run_full_frame();
        assert_ne!(geometry.checksum, 0);
        assert_ne!(raster.checksum, 0);
        assert_ne!(full.checksum, 0);
    }
}
