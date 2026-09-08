use alloc::vec::Vec;

use super::super::{Background, Image2D, Texture2D};
use super::raster::floor_i32;
#[allow(unused_imports)]
use super::types::*;

pub(crate) fn sample_texture(texture: &M3gTexture, u: f32, v: f32) -> i32 {
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

pub(crate) fn sample_texture_linear(texture: &M3gTexture, u: f32, v: f32) -> i32 {
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

pub(crate) fn transform_texture_coord(matrix: [f32; 16], u: f32, v: f32) -> (f32, f32) {
    let s = matrix[0] * u + matrix[1] * v + matrix[3];
    let t = matrix[4] * u + matrix[5] * v + matrix[7];
    let q = matrix[12] * u + matrix[13] * v + matrix[15];
    if q.is_finite() && q.abs() > f32::EPSILON {
        (s / q, t / q)
    } else {
        (s, t)
    }
}

pub(crate) fn texture_transform_is_identity(matrix: [f32; 16]) -> bool {
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

pub(crate) fn texture_coord_to_index(coord: f32, size: i32, wrapping: i32) -> i32 {
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

pub(crate) fn texture_coord_to_linear_indices(coord: f32, size: i32, wrapping: i32) -> (i32, i32, f32) {
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

pub(crate) fn texture_index(index: i32, size: i32, wrapping: i32) -> i32 {
    if wrapping == Texture2D::WRAP_CLAMP {
        return index.clamp(0, size - 1);
    }
    if is_power_of_two(size) {
        return index & (size - 1);
    }
    let index = index % size;
    if index < 0 { index + size } else { index }
}

pub(crate) fn is_power_of_two(value: i32) -> bool {
    value > 0 && (value & (value - 1)) == 0
}

pub(crate) fn bilinear_channel(c00: i32, c10: i32, c01: i32, c11: i32, tx: f32, ty: f32, shift: u32) -> u32 {
    let c00 = (((c00 as u32) >> shift) & 0xff) as f32;
    let c10 = (((c10 as u32) >> shift) & 0xff) as f32;
    let c01 = (((c01 as u32) >> shift) & 0xff) as f32;
    let c11 = (((c11 as u32) >> shift) & 0xff) as f32;
    let top = c00 + (c10 - c00) * tx;
    let bottom = c01 + (c11 - c01) * tx;
    (top + (bottom - top) * ty).clamp(0.0, 255.0) as u32
}

pub(crate) fn background_coord(coord: i32, size: i32, mode: i32) -> Option<i32> {
    if size <= 0 {
        return None;
    }
    if mode == Background::REPEAT {
        let value = coord % size;
        return Some(if value < 0 { value + size } else { value });
    }
    if (0..size).contains(&coord) { Some(coord) } else { None }
}

pub(crate) fn blend_texture(texture: &M3gTexture, fragment: i32, texel: i32) -> i32 {
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

pub(crate) fn pack_argb(a: u32, r: u32, g: u32, b: u32) -> i32 {
    ((a << 24) | (r << 16) | (g << 8) | b) as i32
}

pub(crate) fn mul_u8(left: u32, right: u32) -> u32 {
    ((left * right) / 255).min(255)
}

pub(crate) fn lerp_u8(from: u32, to: u32, alpha: u32) -> u32 {
    ((from * (255 - alpha) + to * alpha) / 255).min(255)
}

pub(crate) fn blend_mode_channel(fragment: u32, blend: u32, texture: u32) -> u32 {
    ((fragment * (255 - texture) + blend * texture) / 255).min(255)
}

pub(crate) fn floor_f32(value: f32) -> f32 {
    let integer = value as i32;
    if value < integer as f32 { (integer - 1) as f32 } else { integer as f32 }
}

pub(crate) fn interpolate_color(c0: i32, c1: i32, c2: i32, w0: f32, w1: f32, w2: f32) -> i32 {
    let c0 = c0 as u32;
    let c1 = c1 as u32;
    let c2 = c2 as u32;
    let a = channel(c0, c1, c2, 24, w0, w1, w2);
    let r = channel(c0, c1, c2, 16, w0, w1, w2);
    let g = channel(c0, c1, c2, 8, w0, w1, w2);
    let b = channel(c0, c1, c2, 0, w0, w1, w2);
    ((a << 24) | (r << 16) | (g << 8) | b) as i32
}

pub(crate) fn channel(c0: u32, c1: u32, c2: u32, shift: u32, w0: f32, w1: f32, w2: f32) -> u32 {
    let v0 = ((c0 >> shift) & 0xff) as f32;
    let v1 = ((c1 >> shift) & 0xff) as f32;
    let v2 = ((c2 >> shift) & 0xff) as f32;
    (v0 * w0 + v1 * w1 + v2 * w2).clamp(0.0, 255.0) as u32
}

pub(crate) fn clamp_color(value: f32) -> u32 {
    if value < 0.0 {
        (value as i32 as i8 as u8) as u32
    } else {
        value.clamp(0.0, 255.0) as u32
    }
}

pub(crate) fn ensure_opaque(color: i32) -> i32 {
    if (color as u32) & 0xff00_0000 == 0 {
        ((color as u32) | 0xff00_0000) as i32
    } else {
        color
    }
}

pub(crate) fn decode_m3g_image_pixels(format: i32, width: i32, height: i32, palette: &[u8], pixels: &[u8]) -> Vec<i32> {
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

pub(crate) fn m3g_image_pixel_stride(format: i32) -> usize {
    match format {
        Image2D::ALPHA | Image2D::LUMINANCE => 1,
        Image2D::LUMINANCE_ALPHA => 2,
        Image2D::RGB => 3,
        Image2D::RGBA => 4,
        _ => 4,
    }
}

pub(crate) fn decode_m3g_pixel(format: i32, bytes: &[u8]) -> i32 {
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
