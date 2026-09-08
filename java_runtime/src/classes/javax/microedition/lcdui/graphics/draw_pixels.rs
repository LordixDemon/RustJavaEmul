use alloc::vec;

use jvm::{Array, ClassInstanceRef, JavaValue, Jvm, Result};

use crate::{
    RuntimeContext,
    classes::javax::microedition::lcdui::{Image, invalidate_gpu_image, publish_gpu_image_to_screen},
};

use super::{
    TARGET_IMAGE_DESC, TARGET_IMAGE_FIELD,
    raster::{
        anchor_xy, blit_i32, clipped_rect, compose, i32_pixels, i32_pixels_mut, named_i32, rasterize_text, raw_i32_array, store_raw_i32_range,
        text_anchor_xy, transform_point, transformed_size,
    },
};

pub struct OffscreenTarget {
    pub(super) translate_x: i32,
    pub(super) translate_y: i32,
    pub(super) clip_x: i32,
    pub(super) clip_y: i32,
    pub(super) clip_w: i32,
    pub(super) clip_h: i32,
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) pixels: ClassInstanceRef<Array<i32>>,
}

impl super::Graphics {
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn draw_region(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        src_x: i32,
        src_y: i32,
        width: i32,
        height: i32,
        transform: i32,
        dest_x: i32,
        dest_y: i32,
        anchor: i32,
    ) -> Result<()> {
        tracing::trace!(
            "javax.microedition.lcdui.Graphics::drawRegion({this:?}, {image:?}, {src_x:?}, {src_y:?}, {width:?}, {height:?}, {transform:?}, {dest_x:?}, {dest_y:?}, {anchor:?})"
        );

        if image.is_null() || width <= 0 || height <= 0 {
            return Ok(());
        }

        let (image_width, image_height, pixel_array) = Image::pixels(jvm, &image).await?;
        let source_pixels = raw_i32_array(jvm, &pixel_array, jvm.array_length(&pixel_array).await?).await?;
        let (draw_width, draw_height) = transformed_size(width, height, transform);

        let source_region_valid = src_x >= 0
            && src_y >= 0
            && i64::from(src_x) + i64::from(width) <= i64::from(image_width)
            && i64::from(src_y) + i64::from(height) <= i64::from(image_height);
        if transform == 0 && source_region_valid {
            let (x, y) = anchor_xy(dest_x, dest_y, draw_width, draw_height, anchor);
            if Self::draw_gpu_image_to_screen(jvm, context, &this, &pixel_array, x, y, draw_width, draw_height, src_x, src_y).await? {
                return Ok(());
            }

            return Self::draw_pixels_strided(
                jvm,
                context,
                &this,
                x,
                y,
                draw_width,
                draw_height,
                &source_pixels,
                image_width,
                src_x,
                src_y,
                true,
            )
            .await;
        }

        let mut pixels = vec![0; (draw_width * draw_height) as usize];

        for y in 0..height {
            for x in 0..width {
                let src_index = ((src_y + y) * image_width + src_x + x) as usize;
                if src_index >= source_pixels.len() {
                    continue;
                }
                let (tx, ty) = transform_point(x, y, width, height, transform);
                if tx >= 0 && ty >= 0 && tx < draw_width && ty < draw_height {
                    pixels[(ty * draw_width + tx) as usize] = source_pixels[src_index];
                }
            }
        }

        let (x, y) = anchor_xy(dest_x, dest_y, draw_width, draw_height, anchor);
        Self::draw_pixels(jvm, context, &this, x, y, draw_width, draw_height, &pixels, true).await
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn draw_rgb(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        rgb_data: ClassInstanceRef<Array<i32>>,
        offset: i32,
        scan_length: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        process_alpha: bool,
    ) -> Result<()> {
        tracing::trace!(
            "javax.microedition.lcdui.Graphics::drawRGB({this:?}, {rgb_data:?}, {offset:?}, {scan_length:?}, {x:?}, {y:?}, {width:?}, {height:?}, {process_alpha:?})"
        );

        if width <= 0 || height <= 0 {
            return Ok(());
        }

        if let Some(rgb) = i32_pixels(&rgb_data) {
            return Self::draw_pixels_strided(
                jvm,
                context,
                &this,
                x,
                y,
                width,
                height,
                rgb,
                scan_length.max(width),
                offset.max(0),
                0,
                process_alpha,
            )
            .await;
        }
        let rgb_len = jvm.array_length(&rgb_data).await?;
        let rgb = raw_i32_array(jvm, &rgb_data, rgb_len).await?;
        Self::draw_pixels_strided(
            jvm,
            context,
            &this,
            x,
            y,
            width,
            height,
            &rgb,
            scan_length.max(width),
            offset.max(0),
            0,
            process_alpha,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn draw_pixels(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        process_alpha: bool,
    ) -> Result<()> {
        Self::draw_pixels_strided(jvm, context, this, x, y, width, height, pixels, width, 0, 0, process_alpha).await
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn replace_pixels(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
    ) -> Result<()> {
        if width <= 0 || height <= 0 {
            return Ok(());
        }

        let (translate_x, translate_y) = Self::translation(this);
        let x = x + translate_x;
        let y = y + translate_y;
        if let Some(mut target) = Self::offscreen_target(this) {
            let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) =
                clipped_rect(x, y, width, height, target.clip_x, target.clip_y, target.clip_w, target.clip_h);
            if draw_w <= 0 || draw_h <= 0 {
                return Ok(());
            }
            invalidate_gpu_image(&target.pixels);
            if let Some(dest) = i32_pixels_mut(&mut target.pixels) {
                blit_i32(
                    dest,
                    target.width,
                    pixels,
                    width,
                    draw_x,
                    draw_y,
                    draw_w,
                    draw_h,
                    src_offset_x,
                    src_offset_y,
                    false,
                );
                return Ok(());
            }
            let draw_w_usize = draw_w as usize;
            for row in 0..draw_h {
                let src_start = ((src_offset_y + row) * width + src_offset_x) as usize;
                let Some(src_row) = pixels.get(src_start..src_start + draw_w_usize) else {
                    return Ok(());
                };
                let dst_start = ((draw_y + row) * target.width + draw_x) as usize;
                store_raw_i32_range(jvm, &mut target.pixels, dst_start, src_row).await?;
            }
            return Ok(());
        }

        let (clip_x, clip_y, clip_w, clip_h) = Self::clip(this);
        let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) = clipped_rect(x, y, width, height, clip_x, clip_y, clip_w, clip_h);
        if draw_w <= 0 || draw_h <= 0 {
            return Ok(());
        }

        context.screen_replace_pixels_strided(draw_x, draw_y, draw_w, draw_h, pixels, width, src_offset_x, src_offset_y);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn draw_pixels_strided(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        source_width: i32,
        source_x: i32,
        source_y: i32,
        process_alpha: bool,
    ) -> Result<()> {
        if width <= 0 || height <= 0 || source_width <= 0 || source_x < 0 || source_y < 0 {
            return Ok(());
        }

        let (translate_x, translate_y) = Self::translation(this);
        let x = x + translate_x;
        let y = y + translate_y;
        if let Some(mut target) = Self::offscreen_target(this) {
            let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) =
                clipped_rect(x, y, width, height, target.clip_x, target.clip_y, target.clip_w, target.clip_h);
            if draw_w <= 0 || draw_h <= 0 {
                return Ok(());
            }
            invalidate_gpu_image(&target.pixels);
            if let Some(dest) = i32_pixels_mut(&mut target.pixels) {
                blit_i32(
                    dest,
                    target.width,
                    pixels,
                    source_width,
                    draw_x,
                    draw_y,
                    draw_w,
                    draw_h,
                    source_x + src_offset_x,
                    source_y + src_offset_y,
                    process_alpha,
                );
                return Ok(());
            }
        } else {
            let (clip_x, clip_y, clip_w, clip_h) = Self::clip(this);
            let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) = clipped_rect(x, y, width, height, clip_x, clip_y, clip_w, clip_h);
            if draw_w <= 0 || draw_h <= 0 {
                return Ok(());
            }

            tracing::info!(
                target: "rustjava_render",
                "graphics.drawPixels target=screen rect={}x{}+{}+{} src={}+{} srcWidth={} alpha={} clip={}x{}+{}+{}",
                draw_w,
                draw_h,
                draw_x,
                draw_y,
                source_x + src_offset_x,
                source_y + src_offset_y,
                source_width,
                process_alpha,
                clip_w,
                clip_h,
                clip_x,
                clip_y
            );
            context.screen_draw_pixels_strided(
                draw_x,
                draw_y,
                draw_w,
                draw_h,
                pixels,
                source_width,
                source_x + src_offset_x,
                source_y + src_offset_y,
                process_alpha,
            );
            return Ok(());
        }

        let target: ClassInstanceRef<Image> = match this.get_named_field(TARGET_IMAGE_FIELD, TARGET_IMAGE_DESC) {
            Some(JavaValue::Object(instance)) => instance.into(),
            _ => None.into(),
        };
        let (target_width, target_height, mut target_pixels_array) = Image::pixels(jvm, &target).await?;
        let (clip_x, clip_y, clip_w, clip_h) = Self::clip(this);
        let clip_x2 = (clip_x + clip_w).min(target_width);
        let clip_y2 = (clip_y + clip_h).min(target_height);
        let clip_x = clip_x.max(0);
        let clip_y = clip_y.max(0);
        let clip_w = (clip_x2 - clip_x).max(0);
        let clip_h = (clip_y2 - clip_y).max(0);
        let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) = clipped_rect(x, y, width, height, clip_x, clip_y, clip_w, clip_h);
        if draw_w <= 0 || draw_h <= 0 {
            return Ok(());
        }
        invalidate_gpu_image(&target_pixels_array);

        let mut target_pixels = raw_i32_array(jvm, &target_pixels_array, (target_width * target_height) as usize).await?;

        for row in 0..draw_h {
            let dst_y = draw_y + row;
            if dst_y < 0 || dst_y >= target_height || dst_y < clip_y || dst_y >= clip_y + clip_h {
                continue;
            }
            let src_row_start = (source_y + src_offset_y + row) * source_width + source_x + src_offset_x;
            for col in 0..draw_w {
                let dst_x = draw_x + col;
                if dst_x < 0 || dst_x >= target_width || dst_x < clip_x || dst_x >= clip_x + clip_w {
                    continue;
                }
                let src = pixels[(src_row_start + col) as usize];
                let dst_index = (dst_y * target_width + dst_x) as usize;
                target_pixels[dst_index] = compose(target_pixels[dst_index], src, process_alpha);
            }
        }

        store_raw_i32_range(jvm, &mut target_pixels_array, 0, &target_pixels).await
    }

    pub(super) fn argb_color(this: &ClassInstanceRef<Self>) -> i32 {
        (0xff00_0000u32 as i32) | (named_i32(this, "color") & 0x00ff_ffff)
    }

    pub(super) async fn draw_text(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        text: &str,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        let color = Self::argb_color(this);
        let metrics = Self::text_metrics(jvm, this).await?;
        let (width, height, pixels) = rasterize_text(text, color, metrics);
        let (x, y) = text_anchor_xy(x, y, width, height, metrics.baseline, anchor);
        Self::draw_pixels(jvm, context, this, x, y, width, height, &pixels, true).await
    }

    pub(super) fn translation(this: &ClassInstanceRef<Self>) -> (i32, i32) {
        (named_i32(this, "translateX"), named_i32(this, "translateY"))
    }

    pub(super) fn clip(this: &ClassInstanceRef<Self>) -> (i32, i32, i32, i32) {
        (
            named_i32(this, "clipX"),
            named_i32(this, "clipY"),
            named_i32(this, "clipW"),
            named_i32(this, "clipH"),
        )
    }

    pub(super) fn offscreen_target(this: &ClassInstanceRef<Self>) -> Option<OffscreenTarget> {
        let target: ClassInstanceRef<Image> = match this.get_named_field(TARGET_IMAGE_FIELD, TARGET_IMAGE_DESC) {
            Some(JavaValue::Object(instance)) => instance.into(),
            _ => return None,
        };
        if target.is_null() {
            return None;
        }
        let (width, height, pixels) = Image::pixels_fast(&target)?;
        let (translate_x, translate_y) = Self::translation(this);
        let (clip_x, clip_y, clip_w, clip_h) = Self::clip(this);
        let clip_x2 = (clip_x + clip_w).min(width);
        let clip_y2 = (clip_y + clip_h).min(height);
        let clip_x = clip_x.max(0);
        let clip_y = clip_y.max(0);
        Some(OffscreenTarget {
            translate_x,
            translate_y,
            clip_x,
            clip_y,
            clip_w: (clip_x2 - clip_x).max(0),
            clip_h: (clip_y2 - clip_y).max(0),
            width,
            height,
            pixels,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn draw_gpu_image_to_screen(
        _jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        pixels: &ClassInstanceRef<Array<i32>>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        source_x: i32,
        source_y: i32,
    ) -> Result<bool> {
        if Self::offscreen_target(this).is_some() {
            return Ok(false);
        }

        let (translate_x, translate_y) = Self::translation(this);
        let x = x + translate_x;
        let y = y + translate_y;
        let (clip_x, clip_y, clip_w, clip_h) = Self::clip(this);
        let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) = clipped_rect(x, y, width, height, clip_x, clip_y, clip_w, clip_h);
        if draw_w <= 0 || draw_h <= 0 {
            return Ok(false);
        }

        Ok(publish_gpu_image_to_screen(
            pixels,
            context.screen_width(),
            context.screen_height(),
            draw_x,
            draw_y,
            source_x + src_offset_x,
            source_y + src_offset_y,
            draw_w,
            draw_h,
            (clip_x, clip_y, clip_w, clip_h),
        ))
    }
}
