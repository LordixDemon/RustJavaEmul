use alloc::{vec, vec::Vec};

use jvm::{Array, ClassInstanceRef, JavaValue, Jvm, Result};

use super::TextMetrics;

pub(super) async fn raw_i32_array(jvm: &Jvm, array: &ClassInstanceRef<Array<i32>>, count: usize) -> Result<Vec<i32>> {
    raw_i32_range(jvm, array, 0, count).await
}

pub(super) async fn raw_i32_range(jvm: &Jvm, array: &ClassInstanceRef<Array<i32>>, offset: usize, count: usize) -> Result<Vec<i32>> {
    if count == 0 {
        return Ok(Vec::new());
    }

    if let Some(pixels) = i32_pixels(array) {
        if offset >= pixels.len() {
            return Ok(Vec::new());
        }
        let count = count.min(pixels.len() - offset);
        return Ok(pixels[offset..offset + count].to_vec());
    }

    let length = jvm.array_length(array).await?;
    if offset >= length {
        return Ok(Vec::new());
    }

    let count = count.min(length - offset);
    let mut values = vec![0; count];
    jvm.array_raw_buffer(array)
        .await?
        .read(offset, bytemuck::cast_slice_mut(values.as_mut_slice()))?;

    Ok(values)
}

pub(super) async fn store_raw_i32_range(jvm: &Jvm, array: &mut ClassInstanceRef<Array<i32>>, offset: usize, values: &[i32]) -> Result<()> {
    if values.is_empty() {
        return Ok(());
    }

    if let Some(pixels) = i32_pixels_mut(array) {
        if offset >= pixels.len() {
            return Ok(());
        }
        let count = values.len().min(pixels.len() - offset);
        pixels[offset..offset + count].copy_from_slice(&values[..count]);
        return Ok(());
    }

    let length = jvm.array_length(array).await?;
    if offset >= length {
        return Ok(());
    }

    let count = values.len().min(length - offset);
    jvm.array_raw_buffer_mut(array)
        .await?
        .write(offset, bytemuck::cast_slice(&values[..count]))
}

pub(super) fn named_i32<T>(instance: &ClassInstanceRef<T>, name: &str) -> i32 {
    match instance.get_named_field(name, "I") {
        Some(JavaValue::Int(value)) => value,
        Some(JavaValue::Boolean(value)) => i32::from(value),
        Some(JavaValue::Byte(value)) => i32::from(value),
        Some(JavaValue::Short(value)) => i32::from(value),
        Some(JavaValue::Char(value)) => i32::from(value),
        _ => 0,
    }
}

pub(super) fn i32_pixels(array: &ClassInstanceRef<Array<i32>>) -> Option<&[i32]> {
    array.as_array_instance()?.i32_slice()
}

pub(super) fn i32_pixels_mut(array: &mut ClassInstanceRef<Array<i32>>) -> Option<&mut [i32]> {
    array.as_array_instance_mut()?.i32_slice_mut()
}

pub(super) fn is_opaque_row(pixels: &[i32]) -> bool {
    pixels.iter().all(|pixel| ((*pixel as u32) >> 24) == 0xff)
}

pub(super) fn edge(ax: i32, ay: i32, bx: i32, by: i32, px: i32, py: i32) -> i32 {
    (px - ax) * (by - ay) - (py - ay) * (bx - ax)
}

pub(super) fn anchor_xy(mut x: i32, mut y: i32, width: i32, height: i32, anchor: i32) -> (i32, i32) {
    if anchor & 1 != 0 {
        x -= width / 2;
    } else if anchor & 8 != 0 {
        x -= width;
    }

    if anchor & 2 != 0 {
        y -= height / 2;
    } else if anchor & 32 != 0 {
        y -= height;
    }

    (x, y)
}

pub(super) fn text_anchor_xy(mut x: i32, mut y: i32, width: i32, height: i32, baseline: i32, anchor: i32) -> (i32, i32) {
    if anchor & 1 != 0 {
        x -= width / 2;
    } else if anchor & 8 != 0 {
        x -= width;
    }

    if anchor & 2 != 0 {
        y -= height / 2;
    } else if anchor & 32 != 0 {
        y -= height;
    } else if anchor & 64 != 0 {
        y -= baseline;
    }

    (x, y)
}

pub(super) fn rasterize_text(text: &str, color: i32, metrics: TextMetrics) -> (i32, i32, Vec<i32>) {
    let width = (text.chars().count() as i32 * metrics.advance).max(1);
    let height = metrics.height.max(1);
    let scale = metrics.scale.max(1);
    let glyph_height = 7 * scale;
    let glyph_y = (height - glyph_height).max(0) / 2;
    let mut pixels = vec![0; (width * height) as usize];

    for (char_index, ch) in text.chars().enumerate() {
        let glyph = glyph_5x7(ch);
        let base_x = char_index as i32 * metrics.advance;
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    let glyph_x = base_x + col * scale;
                    let glyph_y = glyph_y + row as i32 * scale;
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let px = glyph_x + sx;
                            let py = glyph_y + sy;
                            if px >= 0 && px < width && py >= 0 && py < height {
                                pixels[(py * width + px) as usize] = color;
                            }
                        }
                    }
                }
            }
        }
    }

    (width, height, pixels)
}

fn glyph_5x7(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [0x0e, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
        'B' => [0x1e, 0x11, 0x11, 0x1e, 0x11, 0x11, 0x1e],
        'C' => [0x0e, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0e],
        'D' => [0x1e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1e],
        'E' => [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x1f],
        'F' => [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x10],
        'G' => [0x0e, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0f],
        'H' => [0x11, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
        'I' => [0x0e, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0e],
        'J' => [0x07, 0x02, 0x02, 0x02, 0x12, 0x12, 0x0c],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1f],
        'M' => [0x11, 0x1b, 0x15, 0x15, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11],
        'O' => [0x0e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
        'P' => [0x1e, 0x11, 0x11, 0x1e, 0x10, 0x10, 0x10],
        'Q' => [0x0e, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0d],
        'R' => [0x1e, 0x11, 0x11, 0x1e, 0x14, 0x12, 0x11],
        'S' => [0x0f, 0x10, 0x10, 0x0e, 0x01, 0x01, 0x1e],
        'T' => [0x1f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x0a, 0x0a, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1b, 0x11],
        'X' => [0x11, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0a, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1f, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1f],
        '0' => [0x0e, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0e],
        '1' => [0x04, 0x0c, 0x04, 0x04, 0x04, 0x04, 0x0e],
        '2' => [0x0e, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1f],
        '3' => [0x1e, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x1e],
        '4' => [0x02, 0x06, 0x0a, 0x12, 0x1f, 0x02, 0x02],
        '5' => [0x1f, 0x10, 0x10, 0x1e, 0x01, 0x01, 0x1e],
        '6' => [0x0e, 0x10, 0x10, 0x1e, 0x11, 0x11, 0x0e],
        '7' => [0x1f, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0e, 0x11, 0x11, 0x0e, 0x11, 0x11, 0x0e],
        '9' => [0x0e, 0x11, 0x11, 0x0f, 0x01, 0x01, 0x0e],
        ':' => [0x00, 0x04, 0x04, 0x00, 0x04, 0x04, 0x00],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x0c],
        ',' => [0x00, 0x00, 0x00, 0x00, 0x0c, 0x04, 0x08],
        '!' => [0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x04],
        '?' => [0x0e, 0x11, 0x01, 0x02, 0x04, 0x00, 0x04],
        '-' => [0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00],
        '_' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f],
        '/' => [0x01, 0x01, 0x02, 0x04, 0x08, 0x10, 0x10],
        '\\' => [0x10, 0x10, 0x08, 0x04, 0x02, 0x01, 0x01],
        '+' => [0x00, 0x04, 0x04, 0x1f, 0x04, 0x04, 0x00],
        '*' => [0x00, 0x15, 0x0e, 0x1f, 0x0e, 0x15, 0x00],
        '=' => [0x00, 0x00, 0x1f, 0x00, 0x1f, 0x00, 0x00],
        '(' => [0x02, 0x04, 0x08, 0x08, 0x08, 0x04, 0x02],
        ')' => [0x08, 0x04, 0x02, 0x02, 0x02, 0x04, 0x08],
        '[' => [0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x0e],
        ']' => [0x0e, 0x02, 0x02, 0x02, 0x02, 0x02, 0x0e],
        '\'' => [0x04, 0x04, 0x08, 0x00, 0x00, 0x00, 0x00],
        '"' => [0x0a, 0x0a, 0x14, 0x00, 0x00, 0x00, 0x00],
        ' ' => [0; 7],
        _ => [0x1f, 0x11, 0x01, 0x02, 0x04, 0x00, 0x04],
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn clipped_rect(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    clip_x: i32,
    clip_y: i32,
    clip_w: i32,
    clip_h: i32,
) -> (i32, i32, i32, i32, i32, i32) {
    let draw_x = x.max(clip_x);
    let draw_y = y.max(clip_y);
    let end_x = (x + width).min(clip_x + clip_w);
    let end_y = (y + height).min(clip_y + clip_h);

    (draw_x, draw_y, end_x - draw_x, end_y - draw_y, draw_x - x, draw_y - y)
}

pub(super) fn transformed_size(width: i32, height: i32, transform: i32) -> (i32, i32) {
    match transform {
        4..=7 => (height, width),
        _ => (width, height),
    }
}

pub(super) fn transform_point(x: i32, y: i32, width: i32, height: i32, transform: i32) -> (i32, i32) {
    match transform {
        1 => (x, height - 1 - y),             // mirror + rot180
        2 => (width - 1 - x, y),              // mirror
        3 => (width - 1 - x, height - 1 - y), // rot180
        4 => (y, x),                          // mirror + rot270
        5 => (height - 1 - y, x),             // rot90
        6 => (y, width - 1 - x),              // rot270
        7 => (height - 1 - y, width - 1 - x), // mirror + rot90
        _ => (x, y),
    }
}

pub(super) fn compose(dst: i32, src: i32, process_alpha: bool) -> i32 {
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

#[allow(clippy::too_many_arguments)]
pub(super) fn blit_i32(
    dest: &mut [i32],
    dest_width: i32,
    src: &[i32],
    src_width: i32,
    draw_x: i32,
    draw_y: i32,
    draw_w: i32,
    draw_h: i32,
    src_x: i32,
    src_y: i32,
    process_alpha: bool,
) {
    if draw_w <= 0 || draw_h <= 0 || dest_width <= 0 || src_width <= 0 {
        return;
    }
    let draw_w = draw_w as usize;
    for row in 0..draw_h {
        let src_start = ((src_y + row) * src_width + src_x) as usize;
        let dest_start = ((draw_y + row) * dest_width + draw_x) as usize;
        let Some(src_row) = src.get(src_start..src_start + draw_w) else {
            continue;
        };
        let Some(dest_row) = dest.get_mut(dest_start..dest_start + draw_w) else {
            continue;
        };
        if !process_alpha {
            for (dst, src_pixel) in dest_row.iter_mut().zip(src_row) {
                *dst = (0xff00_0000u32 as i32) | (src_pixel & 0x00ff_ffff);
            }
        } else if is_opaque_row(src_row) {
            dest_row.copy_from_slice(src_row);
        } else {
            for (dst, src_pixel) in dest_row.iter_mut().zip(src_row) {
                *dst = compose(*dst, *src_pixel, true);
            }
        }
    }
}
