use alloc::{string::String as RustString, vec, vec::Vec};

use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeContext,
    classes::{
        java::lang::String,
        javax::microedition::lcdui::{Image, invalidate_gpu_image},
    },
};

use super::raster::{anchor_xy, clipped_rect, edge, i32_pixels_mut, raw_i32_array};

impl super::Graphics {
    pub(super) async fn draw_string(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        let string = if string.is_null() {
            "null".into()
        } else {
            JavaLangString::to_rust_string(jvm, &string).await?
        };
        tracing::trace!("javax.microedition.lcdui.Graphics::drawString({this:?}, {string:?}, {x:?}, {y:?}, {anchor:?})");

        Self::draw_text(jvm, context, &this, &string, x, y, anchor).await
    }

    pub(super) async fn draw_substring(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        offset: i32,
        length: i32,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
        }
        let text = JavaLangString::to_rust_string(jvm, &string).await?;
        let chars: Vec<char> = text.chars().collect();
        let valid =
            offset >= 0 && length >= 0 && (offset as usize) <= chars.len() && (length as usize) <= chars.len().saturating_sub(offset as usize);
        if !valid {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid substring range").await);
        }
        let substring: RustString = chars[offset as usize..offset as usize + length as usize].iter().collect();
        Self::draw_text(jvm, context, &this, &substring, x, y, anchor).await
    }

    pub(super) async fn draw_char(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        ch: JavaChar,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        let text = RustString::from_utf16_lossy(&[ch]);
        Self::draw_text(jvm, context, &this, &text, x, y, anchor).await
    }

    pub(super) async fn draw_chars(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        if chars.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "chars is null").await);
        }
        let array_length = jvm.array_length(&chars).await?;
        let valid =
            offset >= 0 && length >= 0 && (offset as usize) <= array_length && (length as usize) <= array_length.saturating_sub(offset as usize);
        if !valid {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid char range").await);
        }

        let chars: Vec<JavaChar> = jvm.load_array(&chars, offset as usize, length as usize).await?;
        let text = RustString::from_utf16_lossy(&chars);
        Self::draw_text(jvm, context, &this, &text, x, y, anchor).await
    }

    pub(super) async fn draw_image(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::drawImage({this:?}, {image:?}, {x:?}, {y:?}, {anchor:?})");

        if image.is_null() {
            return Ok(());
        }

        let (width, height, pixel_array) = Image::pixels(jvm, &image).await?;
        let (x, y) = anchor_xy(x, y, width, height, anchor);
        if Self::draw_gpu_image_to_screen(jvm, context, &this, &pixel_array, x, y, width, height, 0, 0).await? {
            return Ok(());
        }

        if let (Some(src), Some(mut target)) = (super::raster::i32_pixels(&pixel_array), Self::offscreen_target(&this)) {
            let x = x + target.translate_x;
            let y = y + target.translate_y;
            let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) =
                clipped_rect(x, y, width, height, target.clip_x, target.clip_y, target.clip_w, target.clip_h);
            if draw_w > 0 && draw_h > 0 {
                invalidate_gpu_image(&target.pixels);
                if let Some(dest) = i32_pixels_mut(&mut target.pixels) {
                    super::raster::blit_i32(
                        dest,
                        target.width,
                        src,
                        width,
                        draw_x,
                        draw_y,
                        draw_w,
                        draw_h,
                        src_offset_x,
                        src_offset_y,
                        true,
                    );
                    return Ok(());
                }
            }
        }

        let pixels = raw_i32_array(jvm, &pixel_array, (width * height) as usize).await?;
        Self::draw_pixels(jvm, context, &this, x, y, width, height, &pixels, true).await
    }

    pub(super) async fn draw_line(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut x1: i32,
        mut y1: i32,
        mut x2: i32,
        mut y2: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::drawLine({this:?}, {x1:?}, {y1:?}, {x2:?}, {y2:?})");

        let color = Self::argb_color(&this);
        if let Some(mut target) = Self::offscreen_target(&this) {
            x1 += target.translate_x;
            y1 += target.translate_y;
            x2 += target.translate_x;
            y2 += target.translate_y;
            let clip = PixelClip::from_target(&target);
            invalidate_gpu_image(&target.pixels);
            if let Some(pixels) = i32_pixels_mut(&mut target.pixels) {
                plot_line(pixels, clip, x1, y1, x2, y2, color);
                return Ok(());
            }
        }
        if y1 == y2 {
            let x = x1.min(x2);
            let width = (x2 - x1).abs() + 1;
            let pixels = vec![color; width as usize];
            return Self::draw_pixels(jvm, context, &this, x, y1, width, 1, &pixels, true).await;
        }
        if x1 == x2 {
            let y = y1.min(y2);
            let height = (y2 - y1).abs() + 1;
            let pixels = vec![color; height as usize];
            return Self::draw_pixels(jvm, context, &this, x1, y, 1, height, &pixels, true).await;
        }

        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut points = Vec::new();
        loop {
            points.push((x1, y1));
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
        for (x, y) in points {
            Self::draw_pixels(jvm, context, &this, x, y, 1, 1, &[color], true).await?;
        }

        Ok(())
    }

    pub(super) async fn draw_rect(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::drawRect({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        if width < 0 || height < 0 {
            return Ok(());
        }

        let color = Self::argb_color(&this);
        let horizontal_width = width + 1;
        let horizontal_pixels = vec![color; horizontal_width as usize];
        Self::draw_pixels(jvm, context, &this, x, y, horizontal_width, 1, &horizontal_pixels, true).await?;
        if height > 0 {
            Self::draw_pixels(jvm, context, &this, x, y + height, horizontal_width, 1, &horizontal_pixels, true).await?;
        }

        if height > 1 {
            let vertical_pixels = vec![color; (height - 1) as usize];
            Self::draw_pixels(jvm, context, &this, x, y + 1, 1, height - 1, &vertical_pixels, true).await?;
            if width > 0 {
                Self::draw_pixels(jvm, context, &this, x + width, y + 1, 1, height - 1, &vertical_pixels, true).await?;
            }
        }

        Ok(())
    }

    pub(super) async fn fill_rect(
        _jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::fillRect({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        if width <= 0 || height <= 0 {
            return Ok(());
        }

        let color = Self::argb_color(&this);
        if let Some(mut target) = Self::offscreen_target(&this) {
            let x = x + target.translate_x;
            let y = y + target.translate_y;
            let (draw_x, draw_y, draw_w, draw_h, _, _) =
                clipped_rect(x, y, width, height, target.clip_x, target.clip_y, target.clip_w, target.clip_h);
            if draw_w <= 0 || draw_h <= 0 {
                return Ok(());
            }
            invalidate_gpu_image(&target.pixels);
            if let Some(pixels) = i32_pixels_mut(&mut target.pixels) {
                let stride = target.width;
                let draw_w = draw_w as usize;
                for row in 0..draw_h {
                    let start = ((draw_y + row) * stride + draw_x) as usize;
                    if let Some(line) = pixels.get_mut(start..start + draw_w) {
                        line.fill(color);
                    }
                }
                return Ok(());
            }
        }
        let (translate_x, translate_y) = Self::translation(&this);
        let x = x + translate_x;
        let y = y + translate_y;
        let (clip_x, clip_y, clip_w, clip_h) = Self::clip(&this);
        let (draw_x, draw_y, draw_w, draw_h, _, _) = clipped_rect(x, y, width, height, clip_x, clip_y, clip_w, clip_h);
        if draw_w > 0 && draw_h > 0 {
            tracing::info!(
                target: "rustjava_render",
                "graphics.fillRect target=screen rect={}x{}+{}+{} clip={}x{}+{}+{} color={:#08x}",
                draw_w,
                draw_h,
                draw_x,
                draw_y,
                clip_w,
                clip_h,
                clip_x,
                clip_y,
                color & 0x00ff_ffff
            );
            context.screen_fill_rect(draw_x, draw_y, draw_w, draw_h, color);
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn draw_round_rect(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        _arc_width: i32,
        _arc_height: i32,
    ) -> Result<()> {
        Self::draw_rect(jvm, context, this, x, y, width, height).await
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn fill_round_rect(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        _arc_width: i32,
        _arc_height: i32,
    ) -> Result<()> {
        Self::fill_rect(jvm, context, this, x, y, width, height).await
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn draw_arc(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        start_angle: i32,
        arc_angle: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::drawArc({this:?}, {x:?}, {y:?}, {width:?}, {height:?}, {start_angle:?}, {arc_angle:?})");

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn fill_arc(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        start_angle: i32,
        arc_angle: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::fillArc({this:?}, {x:?}, {y:?}, {width:?}, {height:?}, {start_angle:?}, {arc_angle:?})");

        if width <= 0 || height <= 0 {
            return Ok(());
        }

        let color = Self::argb_color(&this);
        let pixels = vec![color; (width * height) as usize];
        Self::draw_pixels(jvm, context, &this, x, y, width, height, &pixels, true).await
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn fill_triangle(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::fillTriangle({this:?}, {x1:?}, {y1:?}, {x2:?}, {y2:?}, {x3:?}, {y3:?})");

        let color = Self::argb_color(&this);
        if let Some(mut target) = Self::offscreen_target(&this) {
            let x1 = x1 + target.translate_x;
            let y1 = y1 + target.translate_y;
            let x2 = x2 + target.translate_x;
            let y2 = y2 + target.translate_y;
            let x3 = x3 + target.translate_x;
            let y3 = y3 + target.translate_y;
            let clip = PixelClip::from_target(&target);
            invalidate_gpu_image(&target.pixels);
            if let Some(pixels) = i32_pixels_mut(&mut target.pixels) {
                fill_triangle_spans(pixels, clip, x1, y1, x2, y2, x3, y3, color);
                return Ok(());
            }
        }
        let min_x = x1.min(x2).min(x3);
        let max_x = x1.max(x2).max(x3);
        let min_y = y1.min(y2).min(y3);
        let max_y = y1.max(y2).max(y3);
        let max_width = max_x - min_x + 1;
        if max_width <= 0 {
            return Ok(());
        }
        let row_pixels = vec![color; max_width as usize];

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
                let width = row_end - row_start + 1;
                Self::draw_pixels(jvm, context, &this, row_start, y, width, 1, &row_pixels[..width as usize], true).await?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct PixelClip {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    width: i32,
    height: i32,
}

impl PixelClip {
    fn from_target(target: &super::OffscreenTarget) -> Self {
        Self {
            x: target.clip_x,
            y: target.clip_y,
            w: target.clip_w,
            h: target.clip_h,
            width: target.width,
            height: target.height,
        }
    }

    fn contains(self, x: i32, y: i32) -> bool {
        x >= self.x && y >= self.y && x < self.x + self.w && y < self.y + self.h && x >= 0 && y >= 0 && x < self.width && y < self.height
    }
}

fn put_clipped(pixels: &mut [i32], clip: PixelClip, x: i32, y: i32, color: i32) {
    if clip.contains(x, y) {
        let index = (y * clip.width + x) as usize;
        if let Some(slot) = pixels.get_mut(index) {
            *slot = color;
        }
    }
}

fn plot_line(pixels: &mut [i32], clip: PixelClip, mut x1: i32, mut y1: i32, x2: i32, y2: i32, color: i32) {
    if y1 == y2 {
        let x = x1.min(x2);
        let width = (x2 - x1).abs() + 1;
        fill_horizontal(pixels, clip, x, y1, width, color);
        return;
    }
    if x1 == x2 {
        let y = y1.min(y2);
        let height = (y2 - y1).abs() + 1;
        for row in 0..height {
            put_clipped(pixels, clip, x1, y + row, color);
        }
        return;
    }
    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        put_clipped(pixels, clip, x1, y1, color);
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
}

fn fill_horizontal(pixels: &mut [i32], clip: PixelClip, x: i32, y: i32, width: i32, color: i32) {
    if y < clip.y || y >= clip.y + clip.h || y < 0 || y >= clip.height || width <= 0 {
        return;
    }
    let start = x.max(clip.x).max(0);
    let end = (x + width).min(clip.x + clip.w).min(clip.width);
    if end <= start {
        return;
    }
    let offset = (y * clip.width + start) as usize;
    let count = (end - start) as usize;
    if let Some(line) = pixels.get_mut(offset..offset + count) {
        line.fill(color);
    }
}

fn fill_triangle_spans(pixels: &mut [i32], clip: PixelClip, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: i32) {
    let mut verts = [(x1, y1), (x2, y2), (x3, y3)];
    verts.sort_by_key(|vertex| vertex.1);
    let (x_top, y_top) = verts[0];
    let (x_mid, y_mid) = verts[1];
    let (x_bot, y_bot) = verts[2];
    if y_top == y_bot {
        let min_x = x_top.min(x_mid).min(x_bot);
        let max_x = x_top.max(x_mid).max(x_bot);
        fill_horizontal(pixels, clip, min_x, y_top, max_x - min_x + 1, color);
        return;
    }

    for y in y_top..=y_bot {
        let x_long = interpolate_x(y_top, x_top, y_bot, x_bot, y);
        let x_short = if y < y_mid || y_mid == y_bot {
            interpolate_x(y_top, x_top, y_mid, x_mid, y)
        } else {
            interpolate_x(y_mid, x_mid, y_bot, x_bot, y)
        };
        let start = x_long.min(x_short);
        let end = x_long.max(x_short);
        fill_horizontal(pixels, clip, start, y, end - start + 1, color);
    }
}

fn interpolate_x(y0: i32, x0: i32, y1: i32, x1: i32, y: i32) -> i32 {
    if y1 == y0 {
        return x0;
    }
    x0 + ((x1 - x0) as i64 * (y - y0) as i64 / (y1 - y0) as i64) as i32
}
