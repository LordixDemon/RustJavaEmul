use alloc::{vec, vec::Vec};
use core::hint::black_box;

use super::{
    TextMetrics,
    raster::{clipped_rect, compose, edge, rasterize_text, transform_point, transformed_size},
};

const DEFAULT_SCREEN_WIDTH: i32 = 240;
const DEFAULT_SCREEN_HEIGHT: i32 = 320;
const DEFAULT_FRAMES: u32 = 120;
const DEFAULT_SPRITE_SIZE: i32 = 48;

#[derive(Clone, Copy, Debug)]
pub struct LcdUiBenchmarkConfig {
    pub screen_width: i32,
    pub screen_height: i32,
    pub frames: u32,
    pub sprite_size: i32,
}

impl Default for LcdUiBenchmarkConfig {
    fn default() -> Self {
        Self {
            screen_width: DEFAULT_SCREEN_WIDTH,
            screen_height: DEFAULT_SCREEN_HEIGHT,
            frames: DEFAULT_FRAMES,
            sprite_size: DEFAULT_SPRITE_SIZE,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LcdUiBenchmarkRun {
    pub frames: u32,
    pub ops: u64,
    pub checksum: u64,
}

pub struct LcdUiBenchmark {
    config: LcdUiBenchmarkConfig,
    sprite_opaque: Vec<i32>,
    sprite_alpha: Vec<i32>,
}

impl LcdUiBenchmark {
    pub fn new(config: LcdUiBenchmarkConfig) -> Self {
        let config = normalize_config(config);
        let size = config.sprite_size as usize;
        Self {
            sprite_opaque: synthetic_sprite(size, false),
            sprite_alpha: synthetic_sprite(size, true),
            config,
        }
    }

    pub fn config(&self) -> LcdUiBenchmarkConfig {
        self.config
    }

    pub fn run_fill(&self) -> LcdUiBenchmarkRun {
        let mut run = self.empty_run();
        let mut pixels = self.target();
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            let color = rect_color(frame);
            for (x, y, w, h) in hud_rects(self.config, frame) {
                fill_rect(&mut pixels, self.config.screen_width, self.config.screen_height, x, y, w, h, color);
                ops = ops.saturating_add((w.max(0) as u64).saturating_mul(h.max(0) as u64));
            }
            checksum = checksum_pixels(checksum, &pixels, frame);
            black_box(&pixels);
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    pub fn run_blit_opaque(&self) -> LcdUiBenchmarkRun {
        self.blit(false)
    }

    pub fn run_blit_alpha(&self) -> LcdUiBenchmarkRun {
        self.blit(true)
    }

    pub fn run_text(&self) -> LcdUiBenchmarkRun {
        let mut run = self.empty_run();
        let mut pixels = self.target();
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        let metrics = TextMetrics {
            advance: 6,
            height: 12,
            baseline: 10,
            scale: 1,
        };
        let large = TextMetrics {
            advance: 12,
            height: 16,
            baseline: 13,
            scale: 2,
        };
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            let color = rect_color(frame);
            let (w, h, glyph) = rasterize_text("SCORE 12345 HP 88/100", color, metrics);
            blit(
                &mut pixels,
                self.config.screen_width,
                self.config.screen_height,
                8,
                8,
                w,
                h,
                &glyph,
                w,
                0,
                0,
                true,
            );
            ops = ops.saturating_add((w * h).max(0) as u64);
            let (w, h, glyph) = rasterize_text("PAUSE", color.rotate_left(8), large);
            blit(
                &mut pixels,
                self.config.screen_width,
                self.config.screen_height,
                self.config.screen_width / 2 - w / 2,
                self.config.screen_height / 2 - h / 2,
                w,
                h,
                &glyph,
                w,
                0,
                0,
                true,
            );
            ops = ops.saturating_add((w * h).max(0) as u64);
            checksum = checksum_pixels(checksum, &pixels, frame);
            black_box(&pixels);
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    pub fn run_transform(&self) -> LcdUiBenchmarkRun {
        let mut run = self.empty_run();
        let mut pixels = self.target();
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        let src_w = self.config.sprite_size;
        let src_h = self.config.sprite_size;
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            for transform in 0..8 {
                let (draw_w, draw_h) = transformed_size(src_w, src_h, transform);
                let mut transformed = vec![0; (draw_w * draw_h).max(0) as usize];
                for y in 0..src_h {
                    for x in 0..src_w {
                        let src_index = (y * src_w + x) as usize;
                        let (tx, ty) = transform_point(x, y, src_w, src_h, transform);
                        if tx >= 0 && ty >= 0 && tx < draw_w && ty < draw_h {
                            transformed[(ty * draw_w + tx) as usize] = self.sprite_alpha[src_index];
                        }
                    }
                }
                let x = 4 + (transform % 4) * (draw_w + 4);
                let y = 4 + (transform / 4) * (draw_h + 4) + (frame as i32 % 6);
                blit(
                    &mut pixels,
                    self.config.screen_width,
                    self.config.screen_height,
                    x,
                    y,
                    draw_w,
                    draw_h,
                    &transformed,
                    draw_w,
                    0,
                    0,
                    true,
                );
                ops = ops.saturating_add((draw_w * draw_h).max(0) as u64);
            }
            checksum = checksum_pixels(checksum, &pixels, frame);
            black_box(&pixels);
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    pub fn run_triangle(&self) -> LcdUiBenchmarkRun {
        let mut run = self.empty_run();
        let mut pixels = self.target();
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            let color = rect_color(frame);
            let cx = self.config.screen_width / 2;
            let cy = self.config.screen_height / 2;
            let spin = (frame as i32 * 3) % 40;
            ops = ops.saturating_add(fill_triangle(
                &mut pixels,
                self.config.screen_width,
                self.config.screen_height,
                cx,
                8 + spin,
                12,
                self.config.screen_height - 12,
                self.config.screen_width - 12,
                self.config.screen_height / 2,
                color,
            ));
            ops = ops.saturating_add(fill_triangle(
                &mut pixels,
                self.config.screen_width,
                self.config.screen_height,
                20,
                20,
                cx,
                cy + spin,
                self.config.screen_width - 20,
                24,
                color.rotate_left(8),
            ));
            checksum = checksum_pixels(checksum, &pixels, frame);
            black_box(&pixels);
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    pub fn run_line(&self) -> LcdUiBenchmarkRun {
        let mut run = self.empty_run();
        let mut pixels = self.target();
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            let color = rect_color(frame);
            let w = self.config.screen_width;
            let h = self.config.screen_height;
            for index in 0..24 {
                let x2 = (index * 11 + frame as i32 * 3).rem_euclid(w);
                let y2 = (index * 7 + frame as i32 * 5).rem_euclid(h);
                ops = ops.saturating_add(draw_line(&mut pixels, w, h, w / 2, h / 2, x2, y2, color));
            }
            checksum = checksum_pixels(checksum, &pixels, frame);
            black_box(&pixels);
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    pub fn run_full_frame(&self) -> LcdUiBenchmarkRun {
        let mut run = self.empty_run();
        let mut pixels = self.target();
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        let metrics = TextMetrics {
            advance: 6,
            height: 12,
            baseline: 10,
            scale: 1,
        };
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            let color = rect_color(frame);
            for (x, y, w, h) in hud_rects(self.config, frame) {
                fill_rect(&mut pixels, self.config.screen_width, self.config.screen_height, x, y, w, h, color);
                ops = ops.saturating_add((w.max(0) as u64).saturating_mul(h.max(0) as u64));
            }
            let sprite = if frame % 2 == 0 { &self.sprite_opaque } else { &self.sprite_alpha };
            let size = self.config.sprite_size;
            for index in 0..8 {
                let x = 8 + (index * 23 + frame as i32 * 2).rem_euclid(self.config.screen_width - size).max(0);
                let y = 20 + (index * 17 + frame as i32).rem_euclid(self.config.screen_height - size).max(0);
                blit(
                    &mut pixels,
                    self.config.screen_width,
                    self.config.screen_height,
                    x,
                    y,
                    size,
                    size,
                    sprite,
                    size,
                    0,
                    0,
                    frame % 2 == 1,
                );
                ops = ops.saturating_add((size * size) as u64);
            }
            let (tw, th, glyph) = rasterize_text("HUD 3D READY", color, metrics);
            blit(
                &mut pixels,
                self.config.screen_width,
                self.config.screen_height,
                6,
                self.config.screen_height - th - 6,
                tw,
                th,
                &glyph,
                tw,
                0,
                0,
                true,
            );
            ops = ops.saturating_add((tw * th).max(0) as u64);
            ops = ops.saturating_add(fill_triangle(
                &mut pixels,
                self.config.screen_width,
                self.config.screen_height,
                12,
                40,
                48,
                90,
                80,
                36,
                color,
            ));
            ops = ops.saturating_add(draw_line(
                &mut pixels,
                self.config.screen_width,
                self.config.screen_height,
                0,
                0,
                self.config.screen_width - 1,
                self.config.screen_height - 1,
                color,
            ));
            checksum = checksum_pixels(checksum, &pixels, frame);
            black_box(&pixels);
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    fn blit(&self, alpha: bool) -> LcdUiBenchmarkRun {
        let mut run = self.empty_run();
        let mut pixels = self.target();
        let sprite = if alpha { &self.sprite_alpha } else { &self.sprite_opaque };
        let size = self.config.sprite_size;
        let mut checksum = run.checksum;
        let mut ops = 0u64;
        for frame in 0..self.config.frames {
            fill_background(&mut pixels, frame);
            for index in 0..16 {
                let x = (index * 13 + frame as i32 * 3).rem_euclid(self.config.screen_width - size).max(0);
                let y = (index * 9 + frame as i32 * 2).rem_euclid(self.config.screen_height - size).max(0);
                blit(
                    &mut pixels,
                    self.config.screen_width,
                    self.config.screen_height,
                    x,
                    y,
                    size,
                    size,
                    sprite,
                    size,
                    0,
                    0,
                    alpha,
                );
                ops = ops.saturating_add((size * size) as u64);
            }
            checksum = checksum_pixels(checksum, &pixels, frame);
            black_box(&pixels);
        }
        run.ops = ops;
        run.checksum = checksum;
        run
    }

    fn target(&self) -> Vec<i32> {
        vec![0; (self.config.screen_width as usize).saturating_mul(self.config.screen_height as usize)]
    }

    fn empty_run(&self) -> LcdUiBenchmarkRun {
        LcdUiBenchmarkRun {
            frames: self.config.frames,
            ops: 0,
            checksum: 0xcbf2_9ce4_8422_2325,
        }
    }
}

fn normalize_config(config: LcdUiBenchmarkConfig) -> LcdUiBenchmarkConfig {
    LcdUiBenchmarkConfig {
        screen_width: config.screen_width.clamp(32, 2048),
        screen_height: config.screen_height.clamp(32, 2048),
        frames: config.frames.clamp(1, 10_000),
        sprite_size: config.sprite_size.clamp(8, 256),
    }
}

fn synthetic_sprite(size: usize, alpha: bool) -> Vec<i32> {
    let mut pixels = vec![0; size.saturating_mul(size)];
    for y in 0..size {
        for x in 0..size {
            let edge = x < 2 || y < 2 || x + 2 >= size || y + 2 >= size;
            let a = if !alpha {
                0xff
            } else if edge || (x + y) % 5 == 0 {
                0x00
            } else if (x * 3 + y) % 7 == 0 {
                0x60
            } else {
                0xff
            };
            let r = (x * 9) & 0xff;
            let g = (y * 13) & 0xff;
            let b = ((x * 5) ^ (y * 11)) & 0xff;
            pixels[y * size + x] = ((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32) as i32;
        }
    }
    pixels
}

fn hud_rects(config: LcdUiBenchmarkConfig, frame: u32) -> [(i32, i32, i32, i32); 6] {
    let w = config.screen_width;
    let h = config.screen_height;
    let shift = (frame as i32 % 9) - 4;
    [
        (0, 0, w, 16),
        (0, h - 18, w, 18),
        (4, 20, 36, 36),
        (w - 40, 20, 36, 48),
        (w / 4 + shift, h / 3, w / 2, 12),
        (8, h / 2, w - 16, 8),
    ]
}

fn fill_rect(pixels: &mut [i32], width: i32, height: i32, x: i32, y: i32, w: i32, h: i32, color: i32) {
    let (draw_x, draw_y, draw_w, draw_h, _, _) = clipped_rect(x, y, w, h, 0, 0, width, height);
    if draw_w <= 0 || draw_h <= 0 {
        return;
    }
    for row in 0..draw_h {
        let start = ((draw_y + row) * width + draw_x) as usize;
        if let Some(line) = pixels.get_mut(start..start + draw_w as usize) {
            line.fill(color);
        }
    }
}

fn blit(
    pixels: &mut [i32],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    source: &[i32],
    source_width: i32,
    source_x: i32,
    source_y: i32,
    process_alpha: bool,
) {
    let (draw_x, draw_y, draw_w, draw_h, src_ox, src_oy) = clipped_rect(x, y, w, h, 0, 0, width, height);
    if draw_w <= 0 || draw_h <= 0 || source_width <= 0 {
        return;
    }
    for row in 0..draw_h {
        let src_start = ((source_y + src_oy + row) * source_width + source_x + src_ox) as usize;
        let dst_start = ((draw_y + row) * width + draw_x) as usize;
        let Some(src_row) = source.get(src_start..src_start + draw_w as usize) else {
            continue;
        };
        let Some(dst_row) = pixels.get_mut(dst_start..dst_start + draw_w as usize) else {
            continue;
        };
        if process_alpha {
            for (dst, src) in dst_row.iter_mut().zip(src_row) {
                *dst = compose(*dst, *src, true);
            }
        } else {
            dst_row.copy_from_slice(src_row);
        }
    }
}

fn fill_triangle(pixels: &mut [i32], width: i32, height: i32, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: i32) -> u64 {
    let min_x = x1.min(x2).min(x3).max(0);
    let max_x = x1.max(x2).max(x3).min(width - 1);
    let min_y = y1.min(y2).min(y3).max(0);
    let max_y = y1.max(y2).max(y3).min(height - 1);
    if min_x > max_x || min_y > max_y {
        return 0;
    }
    let mut written = 0u64;
    for y in min_y..=max_y {
        let mut row_start = None;
        let mut row_end = min_x;
        for x in min_x..=max_x {
            let a = edge(x2, y2, x3, y3, x, y);
            let b = edge(x3, y3, x1, y1, x, y);
            let c = edge(x1, y1, x2, y2, x, y);
            if (a >= 0 && b >= 0 && c >= 0) || (a <= 0 && b <= 0 && c <= 0) {
                row_start.get_or_insert(x);
                row_end = x;
            }
        }
        if let Some(row_start) = row_start {
            let span = (row_end - row_start + 1).max(0);
            fill_rect(pixels, width, height, row_start, y, span, 1, color);
            written = written.saturating_add(span as u64);
        }
    }
    written
}

fn draw_line(pixels: &mut [i32], width: i32, height: i32, mut x1: i32, mut y1: i32, x2: i32, y2: i32, color: i32) -> u64 {
    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut written = 0u64;
    loop {
        if x1 >= 0 && y1 >= 0 && x1 < width && y1 < height {
            let index = (y1 * width + x1) as usize;
            if let Some(pixel) = pixels.get_mut(index) {
                *pixel = color;
                written = written.saturating_add(1);
            }
        }
        if x1 == x2 && y1 == y2 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x1 += sx;
        }
        if e2 <= dx {
            err += dx;
            y1 += sy;
        }
    }
    written
}

fn fill_background(pixels: &mut [i32], frame: u32) {
    let r = 12 + frame.wrapping_mul(2) % 20;
    let g = 14 + frame.wrapping_mul(3) % 20;
    let b = 18 + frame.wrapping_mul(5) % 24;
    pixels.fill((0xff00_0000 | (r << 16) | (g << 8) | b) as i32);
}

fn rect_color(frame: u32) -> i32 {
    let r = 40 + frame.wrapping_mul(11) % 180;
    let g = 50 + frame.wrapping_mul(7) % 160;
    let b = 70 + frame.wrapping_mul(13) % 140;
    (0xff00_0000 | (r << 16) | (g << 8) | b) as i32
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
    use super::{LcdUiBenchmark, LcdUiBenchmarkConfig};

    #[test]
    fn lcdui_benchmark_runs_tiny_workload() {
        let benchmark = LcdUiBenchmark::new(LcdUiBenchmarkConfig {
            screen_width: 96,
            screen_height: 128,
            frames: 2,
            sprite_size: 16,
        });
        assert_ne!(benchmark.run_fill().checksum, 0);
        assert_ne!(benchmark.run_blit_opaque().checksum, 0);
        assert_ne!(benchmark.run_blit_alpha().checksum, 0);
        assert_ne!(benchmark.run_text().checksum, 0);
        assert_ne!(benchmark.run_transform().checksum, 0);
        assert_ne!(benchmark.run_triangle().checksum, 0);
        assert_ne!(benchmark.run_line().checksum, 0);
        assert_ne!(benchmark.run_full_frame().checksum, 0);
    }
}
