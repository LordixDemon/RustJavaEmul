use alloc::{vec, vec::Vec};

use super::{
    constants::{MAT_BLEND_MASK, MAT_COLORKEY, MAT_DOUBLE_FACE, MAT_ZSORT_MASK, MAT_ZSORT_NEAR},
    scene::ProjectedVertex,
    texture::NativeTexture,
};

#[derive(Clone)]
pub(super) struct RenderVertex {
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) z: i32,
    pub(super) u: i32,
    pub(super) v: i32,
}

pub(super) struct RenderTri {
    pub(super) z: i32,
    pub(super) mat: i32,
    pub(super) texture: Option<usize>,
    pub(super) implicit_color_key: bool,
    pub(super) color: i32,
    pub(super) vertices: [RenderVertex; 3],
}

pub(super) struct PreparedFigure {
    pub(super) textures: Vec<Option<NativeTexture>>,
    pub(super) triangles: Vec<RenderTri>,
    pub(super) effect_transparency: bool,
    pub(super) clip: Option<(i32, i32, i32, i32)>,
    pub(super) source: PreparedSource,
    pub(super) prepare_ms: u64,
}

pub(super) struct SceneTri {
    pub(super) tri: RenderTri,
    pub(super) effect_transparency: bool,
    pub(super) clip: Option<(i32, i32, i32, i32)>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum PreparedSource {
    Figure,
    Primitive,
}

pub(super) fn push_render_tri(
    projected: &[ProjectedVertex],
    triangles: &mut Vec<RenderTri>,
    mat: i32,
    texture: Option<usize>,
    color: i32,
    vertices: [(usize, i32, i32); 3],
) {
    if vertices.iter().any(|(index, _, _)| *index >= projected.len()) {
        return;
    }
    let p0 = projected[vertices[0].0];
    let p1 = projected[vertices[1].0];
    let p2 = projected[vertices[2].0];
    if (mat & MAT_DOUBLE_FACE) == 0 {
        let cross = (p1.x - p0.x) as i64 * (p2.y - p1.y) as i64 - (p1.y - p0.y) as i64 * (p2.x - p1.x) as i64;
        if cross <= 0 {
            return;
        }
    }

    let z = if (mat & MAT_ZSORT_MASK) == 0 {
        (p0.z + p1.z + p2.z) / 3
    } else if (mat & MAT_ZSORT_NEAR) != 0 {
        p0.z.min(p1.z).min(p2.z)
    } else {
        p0.z.max(p1.z).max(p2.z)
    };

    triangles.push(RenderTri {
        z,
        mat,
        texture,
        implicit_color_key: false,
        color,
        vertices: [
            RenderVertex {
                x: p0.x,
                y: p0.y,
                z: p0.z,
                u: vertices[0].1,
                v: vertices[0].2,
            },
            RenderVertex {
                x: p1.x,
                y: p1.y,
                z: p1.z,
                u: vertices[1].1,
                v: vertices[1].2,
            },
            RenderVertex {
                x: p2.x,
                y: p2.y,
                z: p2.z,
                u: vertices[2].1,
                v: vertices[2].2,
            },
        ],
    });
}

pub(super) fn rasterize_triangle(
    tri: &RenderTri,
    textures: &[Option<NativeTexture>],
    clip_x: i32,
    clip_y: i32,
    clip_w: i32,
    clip_h: i32,
    effect_transparency: bool,
) -> Option<(i32, i32, i32, i32, Vec<i32>)> {
    if clip_w <= 0 || clip_h <= 0 {
        return None;
    }

    let x0 = tri.vertices[0].x;
    let y0 = tri.vertices[0].y;
    let x1 = tri.vertices[1].x;
    let y1 = tri.vertices[1].y;
    let x2 = tri.vertices[2].x;
    let y2 = tri.vertices[2].y;
    let min_x = x0.min(x1).min(x2).max(clip_x).max(0);
    let max_x = x0.max(x1).max(x2).min(clip_x + clip_w - 1);
    let min_y = y0.min(y1).min(y2).max(clip_y).max(0);
    let max_y = y0.max(y1).max(y2).min(clip_y + clip_h - 1);
    if min_x > max_x || min_y > max_y {
        return None;
    }

    let area = edge_i64(x0, y0, x1, y1, x2, y2);
    if area == 0 {
        return None;
    }

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut pixels = vec![0; (width * height) as usize];
    let texture = tri.texture.and_then(|index| textures.get(index)).and_then(|x| x.as_ref());
    let use_color_key = (tri.mat & MAT_COLORKEY) != 0 || tri.implicit_color_key;
    let blend_mode = triangle_blend_mode(tri, effect_transparency);
    let mut has_pixels = false;
    let edge_walk = EdgeWalk::new(tri, min_x, min_y);
    let texture_walk = TextureWalk::new(tri, &edge_walk);

    for (row, py) in (min_y..=max_y).enumerate() {
        let [mut w0, mut w1, mut w2] = edge_walk.row_start(row as i32);
        let [mut u_num, mut v_num] = texture_walk.row_start(row as i32);
        for px in min_x..=max_x {
            let inside = if area > 0 {
                w0 >= 0 && w1 >= 0 && w2 >= 0
            } else {
                w0 <= 0 && w1 <= 0 && w2 <= 0
            };
            if inside {
                let color = if let Some(texture) = texture {
                    sample_texture(texture, u_num, v_num, area, use_color_key)
                } else {
                    tri.color
                };
                let color = fallback_blend_color(color, blend_mode);
                if color != 0 {
                    let index = ((py - min_y) * width + (px - min_x)) as usize;
                    pixels[index] = color;
                    has_pixels = true;
                }
            }

            w0 += edge_walk.dx[0];
            w1 += edge_walk.dx[1];
            w2 += edge_walk.dx[2];
            u_num += texture_walk.dx[0];
            v_num += texture_walk.dx[1];
        }
    }

    if has_pixels { Some((min_x, min_y, width, height, pixels)) } else { None }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn rasterize_triangle_into_pixels(
    tri: &RenderTri,
    textures: &[Option<NativeTexture>],
    target_pixels: &mut [i32],
    target_width: i32,
    target_height: i32,
    clip_x: i32,
    clip_y: i32,
    clip_w: i32,
    clip_h: i32,
    effect_transparency: bool,
) -> bool {
    if clip_w <= 0 || clip_h <= 0 || target_width <= 0 || target_height <= 0 {
        return false;
    }
    let target_len = (target_width as usize).saturating_mul(target_height as usize);
    if target_pixels.len() < target_len {
        return false;
    }

    let x0 = tri.vertices[0].x;
    let y0 = tri.vertices[0].y;
    let x1 = tri.vertices[1].x;
    let y1 = tri.vertices[1].y;
    let x2 = tri.vertices[2].x;
    let y2 = tri.vertices[2].y;
    let min_x = x0.min(x1).min(x2).max(clip_x).max(0);
    let max_x = x0.max(x1).max(x2).min(clip_x + clip_w - 1).min(target_width - 1);
    let min_y = y0.min(y1).min(y2).max(clip_y).max(0);
    let max_y = y0.max(y1).max(y2).min(clip_y + clip_h - 1).min(target_height - 1);
    if min_x > max_x || min_y > max_y {
        return false;
    }

    let area = edge_i64(x0, y0, x1, y1, x2, y2);
    if area == 0 {
        return false;
    }

    let texture = tri.texture.and_then(|index| textures.get(index)).and_then(|x| x.as_ref());
    let use_color_key = (tri.mat & MAT_COLORKEY) != 0 || tri.implicit_color_key;
    let blend_mode = triangle_blend_mode(tri, effect_transparency);
    let mut changed = false;
    let edge_walk = EdgeWalk::new(tri, min_x, min_y);
    let texture_walk = TextureWalk::new(tri, &edge_walk);

    for (row, py) in (min_y..=max_y).enumerate() {
        let row_start = (py * target_width) as usize;
        let [mut w0, mut w1, mut w2] = edge_walk.row_start(row as i32);
        let [mut u_num, mut v_num] = texture_walk.row_start(row as i32);
        for px in min_x..=max_x {
            let inside = if area > 0 {
                w0 >= 0 && w1 >= 0 && w2 >= 0
            } else {
                w0 <= 0 && w1 <= 0 && w2 <= 0
            };
            if inside {
                let color = if let Some(texture) = texture {
                    sample_texture(texture, u_num, v_num, area, use_color_key)
                } else {
                    tri.color
                };
                if (color as u32 >> 24) != 0 {
                    let index = row_start + px as usize;
                    if write_blended_pixel(&mut target_pixels[index], color, blend_mode) {
                        changed = true;
                    }
                }
            }

            w0 += edge_walk.dx[0];
            w1 += edge_walk.dx[1];
            w2 += edge_walk.dx[2];
            u_num += texture_walk.dx[0];
            v_num += texture_walk.dx[1];
        }
    }

    changed
}

#[inline]
fn triangle_blend_mode(tri: &RenderTri, effect_transparency: bool) -> i32 {
    if effect_transparency { (tri.mat & MAT_BLEND_MASK) >> 1 } else { 0 }
}

#[inline]
fn fallback_blend_color(color: i32, blend_mode: i32) -> i32 {
    if blend_mode == 1 && (color as u32 >> 24) != 0 {
        return (0x8000_0000u32 as i32) | (color & 0x00ff_ffff);
    }

    color
}

#[inline]
fn write_blended_pixel(dst: &mut i32, src: i32, blend_mode: i32) -> bool {
    let src_alpha = (src as u32) >> 24;
    if src_alpha == 0 {
        return false;
    }

    if blend_mode == 0 && src_alpha == 0xff {
        *dst = src;
        return true;
    }

    let blended = blend_argb(*dst, src, blend_mode);
    if blended == *dst {
        return false;
    }

    *dst = blended;
    true
}

#[inline]
fn blend_argb(dst: i32, src: i32, blend_mode: i32) -> i32 {
    if blend_mode == 0 {
        return compose_argb(dst, src, true);
    }

    let src_alpha = ((src as u32) >> 24) & 0xff;
    if src_alpha == 0 {
        return dst;
    }

    let src = src as u32;
    let dst = dst as u32;
    match blend_mode {
        1 => {
            let rgb = ((dst & src) + (((dst ^ src) >> 1) & 0x007f_7f7f)) & 0x00ff_ffff;
            (0xff00_0000 | rgb) as i32
        }
        2 => {
            let r = (((dst >> 16) & 0xff) + ((src >> 16) & 0xff)).min(0xff);
            let g = (((dst >> 8) & 0xff) + ((src >> 8) & 0xff)).min(0xff);
            let b = ((dst & 0xff) + (src & 0xff)).min(0xff);
            (0xff00_0000 | (r << 16) | (g << 8) | b) as i32
        }
        3 => {
            let r = ((dst >> 16) & 0xff).saturating_sub((src >> 16) & 0xff);
            let g = ((dst >> 8) & 0xff).saturating_sub((src >> 8) & 0xff);
            let b = (dst & 0xff).saturating_sub(src & 0xff);
            (0xff00_0000 | (r << 16) | (g << 8) | b) as i32
        }
        _ => compose_argb(dst as i32, src as i32, true),
    }
}

#[inline]
fn compose_argb(dst: i32, src: i32, process_alpha: bool) -> i32 {
    if !process_alpha {
        return (0xff00_0000u32 as i32) | (src & 0x00ff_ffff);
    }

    let alpha = ((src as u32) >> 24) & 0xff;
    if alpha == 0xff {
        return src;
    }
    if alpha == 0 {
        return dst;
    }

    let inv = 255 - alpha;
    let src = src as u32;
    let dst = dst as u32;
    let r = (((src >> 16) & 0xff) * alpha + ((dst >> 16) & 0xff) * inv) / 255;
    let g = (((src >> 8) & 0xff) * alpha + ((dst >> 8) & 0xff) * inv) / 255;
    let b = ((src & 0xff) * alpha + (dst & 0xff) * inv) / 255;

    (0xff00_0000 | (r << 16) | (g << 8) | b) as i32
}

#[inline]
fn edge_i64(ax: i32, ay: i32, bx: i32, by: i32, px: i32, py: i32) -> i64 {
    (px - ax) as i64 * (by - ay) as i64 - (py - ay) as i64 * (bx - ax) as i64
}

struct EdgeWalk {
    start: [i64; 3],
    dx: [i64; 3],
    dy: [i64; 3],
}

impl EdgeWalk {
    fn new(tri: &RenderTri, start_x: i32, start_y: i32) -> Self {
        let x0 = tri.vertices[0].x;
        let y0 = tri.vertices[0].y;
        let x1 = tri.vertices[1].x;
        let y1 = tri.vertices[1].y;
        let x2 = tri.vertices[2].x;
        let y2 = tri.vertices[2].y;

        Self {
            start: [
                edge_i64(x1, y1, x2, y2, start_x, start_y),
                edge_i64(x2, y2, x0, y0, start_x, start_y),
                edge_i64(x0, y0, x1, y1, start_x, start_y),
            ],
            dx: [(y2 - y1) as i64, (y0 - y2) as i64, (y1 - y0) as i64],
            dy: [(x1 - x2) as i64, (x2 - x0) as i64, (x0 - x1) as i64],
        }
    }

    #[inline]
    fn row_start(&self, row: i32) -> [i64; 3] {
        let row = row as i64;
        [
            self.start[0] + self.dy[0] * row,
            self.start[1] + self.dy[1] * row,
            self.start[2] + self.dy[2] * row,
        ]
    }
}

struct TextureWalk {
    start: [i64; 2],
    dx: [i64; 2],
    dy: [i64; 2],
}

impl TextureWalk {
    fn new(tri: &RenderTri, edge: &EdgeWalk) -> Self {
        let u0 = tri.vertices[0].u as i64;
        let u1 = tri.vertices[1].u as i64;
        let u2 = tri.vertices[2].u as i64;
        let v0 = tri.vertices[0].v as i64;
        let v1 = tri.vertices[1].v as i64;
        let v2 = tri.vertices[2].v as i64;

        Self {
            start: [
                edge.start[0] * u0 + edge.start[1] * u1 + edge.start[2] * u2,
                edge.start[0] * v0 + edge.start[1] * v1 + edge.start[2] * v2,
            ],
            dx: [
                edge.dx[0] * u0 + edge.dx[1] * u1 + edge.dx[2] * u2,
                edge.dx[0] * v0 + edge.dx[1] * v1 + edge.dx[2] * v2,
            ],
            dy: [
                edge.dy[0] * u0 + edge.dy[1] * u1 + edge.dy[2] * u2,
                edge.dy[0] * v0 + edge.dy[1] * v1 + edge.dy[2] * v2,
            ],
        }
    }

    #[inline]
    fn row_start(&self, row: i32) -> [i64; 2] {
        let row = row as i64;
        [self.start[0] + self.dy[0] * row, self.start[1] + self.dy[1] * row]
    }
}

#[inline]
fn sample_texture(texture: &NativeTexture, u_num: i64, v_num: i64, area: i64, use_color_key: bool) -> i32 {
    let u = (u_num / area).clamp(0, 255) as i32;
    let v = (v_num / area).clamp(0, 255) as i32;
    let tx = (u * (texture.width - 1).max(0) / 255).clamp(0, texture.width - 1);
    let ty = (v * (texture.height - 1).max(0) / 255).clamp(0, texture.height - 1);
    let index = (ty * texture.width + tx) as usize;
    if !texture.indices.is_empty() && !texture.palette.is_empty() {
        let palette_index = texture.indices.get(index).copied().unwrap_or(0);
        if use_color_key && palette_index == 0 {
            return 0;
        }
        return texture.palette.get(palette_index as usize).copied().unwrap_or(0) | 0xff00_0000u32 as i32;
    }

    let mut color = texture.pixels.get(index).copied().unwrap_or(0);
    if use_color_key && (color & 0x00ff_ffff) == (texture.color_key & 0x00ff_ffff) {
        return 0;
    }
    if (color as u32 >> 24) == 0 {
        return 0;
    }
    color |= 0xff00_0000u32 as i32;
    color
}
