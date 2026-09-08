use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use std::io::{Cursor, Write};

use java_runtime::{DecodedImage, RuntimeScreen};
use jvm::ClassInstance;

use super::{RuntimeImpl, window::input_diag_enabled};
use crate::profile;

#[cfg_attr(not(feature = "desktop-window"), allow(dead_code))]
pub(crate) struct ScreenState {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) front_pixels: Vec<u32>,
    pub(crate) back_pixels: Vec<u32>,
    pub(crate) generation: u64,
    pub(crate) present_requested: bool,
}

impl ScreenState {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            front_pixels: vec![0; width * height],
            back_pixels: vec![0; width * height],
            generation: 0,
            present_requested: false,
        }
    }

    fn draw_pixels(&mut self, x: i32, y: i32, width: i32, height: i32, pixels: &[i32], process_alpha: bool) {
        self.draw_pixels_strided(x, y, width, height, pixels, width, 0, 0, process_alpha);
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_pixels_strided(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        source_width: i32,
        source_x: i32,
        source_y: i32,
        process_alpha: bool,
    ) {
        if width <= 0 || height <= 0 || source_width <= 0 {
            return;
        }

        let dst_x0 = x.max(0);
        let dst_y0 = y.max(0);
        let dst_x1 = (x + width).min(self.width as i32);
        let dst_y1 = (y + height).min(self.height as i32);
        if dst_x0 >= dst_x1 || dst_y0 >= dst_y1 {
            return;
        }

        let copy_width = (dst_x1 - dst_x0) as usize;
        let copy_height = (dst_y1 - dst_y0) as usize;
        let src_x0 = source_x + dst_x0 - x;
        let src_y0 = source_y + dst_y0 - y;
        if src_x0 < 0 || src_y0 < 0 {
            return;
        }

        let dst_x0 = dst_x0 as usize;
        let dst_y0 = dst_y0 as usize;
        let src_x0 = src_x0 as usize;
        let src_y0 = src_y0 as usize;
        let source_width = source_width as usize;

        tracing::debug!(
            target: "rustjava_render",
            "screen.drawPixels rect={}x{}+{}+{} source={}x?+{}+{} alpha={}",
            copy_width,
            copy_height,
            dst_x0,
            dst_y0,
            source_width,
            src_x0,
            src_y0,
            process_alpha
        );

        if !process_alpha
            && dst_x0 == 0
            && dst_y0 == 0
            && copy_width == self.width
            && copy_height == self.height
            && source_width == self.width
            && src_x0 == 0
            && src_y0 == 0
        {
            let total = self.width * self.height;
            if let Some(src_slice) = pixels.get(..total) {
                for (dst, src) in self.back_pixels[..total].iter_mut().zip(src_slice) {
                    *dst = (*src as u32) | 0xff00_0000;
                }
                return;
            }
        }

        for row in 0..copy_height {
            let src_start = (src_y0 + row) * source_width + src_x0;
            let src_end = src_start + copy_width;
            let Some(src_row) = pixels.get(src_start..src_end) else {
                return;
            };
            let dst_start = (dst_y0 + row) * self.width + dst_x0;
            let dst_row = &mut self.back_pixels[dst_start..dst_start + copy_width];

            if process_alpha {
                for (dst, src) in dst_row.iter_mut().zip(src_row) {
                    *dst = compose_screen(*dst, *src, true);
                }
            } else {
                for (dst, src) in dst_row.iter_mut().zip(src_row) {
                    *dst = (*src as u32) | 0xff00_0000;
                }
            }
        }
    }

    fn replace_pixels_strided(&mut self, x: i32, y: i32, width: i32, height: i32, pixels: &[i32], source_width: i32, source_x: i32, source_y: i32) {
        if width <= 0 || height <= 0 || source_width <= 0 {
            return;
        }

        let dst_x0 = x.max(0);
        let dst_y0 = y.max(0);
        let dst_x1 = (x + width).min(self.width as i32);
        let dst_y1 = (y + height).min(self.height as i32);
        if dst_x0 >= dst_x1 || dst_y0 >= dst_y1 {
            return;
        }

        let copy_width = (dst_x1 - dst_x0) as usize;
        let copy_height = (dst_y1 - dst_y0) as usize;
        let src_x0 = source_x + dst_x0 - x;
        let src_y0 = source_y + dst_y0 - y;
        if src_x0 < 0 || src_y0 < 0 {
            return;
        }

        let dst_x0 = dst_x0 as usize;
        let dst_y0 = dst_y0 as usize;
        let src_x0 = src_x0 as usize;
        let src_y0 = src_y0 as usize;
        let source_width = source_width as usize;

        for row in 0..copy_height {
            let src_start = (src_y0 + row) * source_width + src_x0;
            let src_end = src_start + copy_width;
            let Some(src_row) = pixels.get(src_start..src_end) else {
                return;
            };
            let dst_start = (dst_y0 + row) * self.width + dst_x0;
            let dst_row = &mut self.back_pixels[dst_start..dst_start + copy_width];
            for (dst, src) in dst_row.iter_mut().zip(src_row) {
                *dst = *src as u32;
            }
        }
    }

    fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, argb: i32) {
        if width <= 0 || height <= 0 {
            return;
        }

        let x0 = x.max(0) as usize;
        let y0 = y.max(0) as usize;
        let x1 = (x + width).min(self.width as i32).max(0) as usize;
        let y1 = (y + height).min(self.height as i32).max(0) as usize;
        if x0 >= x1 || y0 >= y1 {
            return;
        }

        let alpha = ((argb as u32) >> 24) & 0xff;
        if alpha == 0 {
            return;
        }

        let color = argb as u32 & 0x00ff_ffff;
        tracing::debug!(
            target: "rustjava_render",
            "screen.fillRect rect={}x{}+{}+{} alpha={} color={:#08x}",
            x1 - x0,
            y1 - y0,
            x0,
            y0,
            alpha,
            color
        );

        for py in y0..y1 {
            let row = &mut self.back_pixels[py * self.width + x0..py * self.width + x1];
            if alpha == 0xff {
                row.fill(argb as u32);
            } else {
                for dst in row {
                    *dst = compose_screen(*dst, argb, true);
                }
            }
        }
    }

    fn present(&mut self) {
        self.front_pixels.copy_from_slice(&self.back_pixels);
        self.generation += 1;
        if tracing::enabled!(target: "rustjava_render", tracing::Level::INFO) {
            tracing::info!(
                target: "rustjava_render",
                "screen.present gen={} size={}x{} hash={:#018x}",
                self.generation,
                self.width,
                self.height,
                pixel_hash(&self.front_pixels)
            );
        }
        self.present_requested = true;
    }

    #[cfg_attr(not(feature = "desktop-window"), allow(dead_code))]
    pub(crate) fn publish_pending_present(&mut self) {
        self.present_requested = false;
    }
}

impl<T> RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    pub fn screen_dimensions(&self) -> (usize, usize) {
        let screen = self.screen.lock();
        (screen.width, screen.height)
    }

    pub fn copy_presented_frame(&self, last_generation: u64, output: &mut Vec<u32>) -> Option<(usize, usize, u64)> {
        let mut screen = self.screen.try_lock()?;
        screen.publish_pending_present();
        if screen.generation == last_generation {
            return None;
        }

        output.resize(screen.front_pixels.len(), 0);
        output.copy_from_slice(&screen.front_pixels);
        Some((screen.width, screen.height, screen.generation))
    }
}

#[async_trait::async_trait]
impl<T> RuntimeScreen for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    fn decode_image(&self, data: &[u8]) -> Option<DecodedImage> {
        let _timer = profile::timer(&profile::DECODE_IMAGE);
        match decode_png(data) {
            Ok(img) => Some(img),
            Err(err) => {
                tracing::debug!("decode_png failed: {err:#}");
                decode_jpeg(data).or_else(|_| decode_bmp(data)).ok()
            }
        }
    }

    fn screen_width(&self) -> i32 {
        self.screen.lock().width as i32
    }

    fn screen_height(&self) -> i32 {
        self.screen.lock().height as i32
    }

    fn screen_draw_pixels(&self, x: i32, y: i32, width: i32, height: i32, pixels: &[i32], process_alpha: bool) {
        let _timer = profile::timer(&profile::SCREEN_DRAW_PIXELS);
        self.screen.lock().draw_pixels(x, y, width, height, pixels, process_alpha);
    }

    fn screen_draw_pixels_strided(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        source_width: i32,
        source_x: i32,
        source_y: i32,
        process_alpha: bool,
    ) {
        let _timer = profile::timer(&profile::SCREEN_DRAW_PIXELS);
        self.screen
            .lock()
            .draw_pixels_strided(x, y, width, height, pixels, source_width, source_x, source_y, process_alpha);
    }

    fn screen_replace_pixels_strided(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        source_width: i32,
        source_x: i32,
        source_y: i32,
    ) {
        let _timer = profile::timer(&profile::SCREEN_DRAW_PIXELS);
        self.screen
            .lock()
            .replace_pixels_strided(x, y, width, height, pixels, source_width, source_x, source_y);
    }

    fn screen_fill_rect(&self, x: i32, y: i32, width: i32, height: i32, argb: i32) {
        let _timer = profile::timer(&profile::SCREEN_FILL_RECT);
        self.screen.lock().fill_rect(x, y, width, height, argb);
    }

    fn screen_present(&self) {
        let _timer = profile::timer(&profile::SCREEN_PRESENT);
        self.screen.lock().present();
    }

    fn set_current_displayable(&self, displayable: Option<Box<dyn ClassInstance>>) {
        if input_diag_enabled() {
            match &displayable {
                Some(displayable) => eprintln!("[display] setCurrent {}", displayable.class_definition().name()),
                None => eprintln!("[display] setCurrent <none>"),
            }
        }
        *self.current_displayable.lock() = displayable;
    }

    fn is_current_displayable(&self, displayable: &dyn ClassInstance) -> bool {
        self.current_displayable
            .lock()
            .as_deref()
            .is_some_and(|current| current.equals(displayable).unwrap_or(false))
    }

    fn game_key_states(&self) -> i32 {
        self.game_key_states.load(Ordering::Relaxed)
    }

    fn target_frame_rate(&self) -> u32 {
        crate::config::get().game_fps
    }
}

fn decode_png(data: &[u8]) -> anyhow::Result<DecodedImage> {
    let mut decoder = png::Decoder::new(Cursor::new(data));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info()?;
    let output_buffer_size = reader
        .output_buffer_size()
        .ok_or_else(|| anyhow::anyhow!("PNG output buffer size is unknown"))?;
    let mut buf = vec![0; output_buffer_size];
    let info = reader.next_frame(&mut buf)?;
    let bytes = &buf[..info.buffer_size()];

    let mut argb = Vec::with_capacity(info.width as usize * info.height as usize);
    match info.color_type {
        png::ColorType::Rgb => {
            for px in bytes.chunks_exact(3) {
                argb.push(argb_pixel(255, px[0], px[1], px[2]));
            }
        }
        png::ColorType::Rgba => {
            for px in bytes.chunks_exact(4) {
                argb.push(argb_pixel(px[3], px[0], px[1], px[2]));
            }
        }
        png::ColorType::Grayscale => {
            for &v in bytes {
                argb.push(argb_pixel(255, v, v, v));
            }
        }
        png::ColorType::GrayscaleAlpha => {
            for px in bytes.chunks_exact(2) {
                argb.push(argb_pixel(px[1], px[0], px[0], px[0]));
            }
        }
        png::ColorType::Indexed => anyhow::bail!("indexed PNG was not expanded"),
    }

    Ok(DecodedImage {
        width: info.width as i32,
        height: info.height as i32,
        argb,
    })
}

fn decode_jpeg(data: &[u8]) -> anyhow::Result<DecodedImage> {
    if data.len() < 2 || data[0] != 0xff || data[1] != 0xd8 {
        anyhow::bail!("not a JPEG image");
    }
    let mut decoder = jpeg_decoder::Decoder::new(Cursor::new(data));
    let pixels = decoder.decode()?;
    let info = decoder.info().ok_or_else(|| anyhow::anyhow!("JPEG info missing"))?;
    let mut argb = Vec::with_capacity(info.width as usize * info.height as usize);
    match info.pixel_format {
        jpeg_decoder::PixelFormat::L8 => {
            for &v in &pixels {
                argb.push(argb_pixel(255, v, v, v));
            }
        }
        jpeg_decoder::PixelFormat::RGB24 => {
            for px in pixels.chunks_exact(3) {
                argb.push(argb_pixel(255, px[0], px[1], px[2]));
            }
        }
        jpeg_decoder::PixelFormat::CMYK32 => {
            for px in pixels.chunks_exact(4) {
                let k = 255u16.saturating_sub(px[3] as u16);
                let r = (255u16.saturating_sub(px[0] as u16) * k / 255) as u8;
                let g = (255u16.saturating_sub(px[1] as u16) * k / 255) as u8;
                let b = (255u16.saturating_sub(px[2] as u16) * k / 255) as u8;
                argb.push(argb_pixel(255, r, g, b));
            }
        }
        jpeg_decoder::PixelFormat::L16 => anyhow::bail!("unsupported JPEG L16"),
    }
    Ok(DecodedImage {
        width: info.width as i32,
        height: info.height as i32,
        argb,
    })
}

fn decode_bmp_rle8(data: &[u8], offset: usize, width: usize, height: usize, top_down: bool, palette: &[i32]) -> anyhow::Result<DecodedImage> {
    let mut rows = vec![vec![0i32; width]; height];
    let mut x = 0usize;
    let mut y = 0usize;
    let mut i = offset;
    while i < data.len() && y < height {
        let count = data[i];
        i += 1;
        if i >= data.len() {
            break;
        }
        if count == 0 {
            match data[i] {
                0 => {
                    x = 0;
                    y += 1;
                    i += 1;
                }
                1 => break,
                2 => {
                    if i + 2 >= data.len() {
                        break;
                    }
                    x = x.saturating_add(data[i + 1] as usize);
                    y = y.saturating_add(data[i + 2] as usize);
                    i += 3;
                }
                n => {
                    i += 1;
                    for _ in 0..n {
                        if i >= data.len() || x >= width || y >= height {
                            break;
                        }
                        rows[y][x] = *palette.get(data[i] as usize).unwrap_or(&0);
                        x += 1;
                        i += 1;
                    }
                    if n % 2 == 1 {
                        i += 1;
                    }
                }
            }
        } else {
            let color = *palette.get(data[i] as usize).unwrap_or(&0);
            i += 1;
            for _ in 0..count {
                if x >= width {
                    break;
                }
                rows[y][x] = color;
                x += 1;
            }
        }
    }
    let mut argb = Vec::with_capacity(width * height);
    for row in 0..height {
        let src = if top_down { row } else { height - 1 - row };
        argb.extend_from_slice(&rows[src]);
    }
    Ok(DecodedImage {
        width: width as i32,
        height: height as i32,
        argb,
    })
}

fn decode_bmp_rle4(data: &[u8], offset: usize, width: usize, height: usize, top_down: bool, palette: &[i32]) -> anyhow::Result<DecodedImage> {
    decode_bmp_rle8(data, offset, width, height, top_down, palette)
}
fn decode_bmp(data: &[u8]) -> anyhow::Result<DecodedImage> {
    if data.len() < 54 || &data[..2] != b"BM" {
        anyhow::bail!("not a BMP image");
    }

    let pixel_offset = read_u32_le(data, 10)? as usize;
    let dib_size = read_u32_le(data, 14)? as usize;
    if dib_size < 40 || data.len() < 14 + dib_size {
        anyhow::bail!("unsupported BMP DIB header");
    }

    let width = read_i32_le(data, 18)?;
    let raw_height = read_i32_le(data, 22)?;
    let planes = read_u16_le(data, 26)?;
    let bits_per_pixel = read_u16_le(data, 28)?;
    let compression = read_u32_le(data, 30)?;
    let colors_used = read_u32_le(data, 46).unwrap_or(0) as usize;

    if width <= 0 || raw_height == 0 || planes != 1 {
        anyhow::bail!("unsupported BMP format");
    }

    let top_down = raw_height < 0;
    let width = width as usize;
    let height = raw_height.unsigned_abs() as usize;
    let palette = bmp_palette(data, dib_size, bits_per_pixel, colors_used)?;

    if compression == 1 {
        return decode_bmp_rle8(data, pixel_offset, width, height, top_down, &palette);
    }
    if compression == 2 {
        return decode_bmp_rle4(data, pixel_offset, width, height, top_down, &palette);
    }
    if compression != 0 {
        anyhow::bail!("unsupported BMP format");
    }

    let row_stride = ((width * bits_per_pixel as usize).div_ceil(32)) * 4;

    if pixel_offset + row_stride.saturating_mul(height) > data.len() {
        anyhow::bail!("truncated BMP pixels");
    }

    let mut argb = Vec::with_capacity(width * height);
    for y in 0..height {
        let src_y = if top_down { y } else { height - 1 - y };
        let row_offset = pixel_offset + src_y * row_stride;
        match bits_per_pixel {
            32 => {
                for x in 0..width {
                    let offset = row_offset + x * 4;
                    let b = data[offset];
                    let g = data[offset + 1];
                    let r = data[offset + 2];
                    let a = data[offset + 3];
                    argb.push(argb_pixel(if a == 0 { 255 } else { a }, r, g, b));
                }
            }
            24 => {
                for x in 0..width {
                    let offset = row_offset + x * 3;
                    argb.push(argb_pixel(255, data[offset + 2], data[offset + 1], data[offset]));
                }
            }
            8 => {
                for x in 0..width {
                    let index = data[row_offset + x] as usize;
                    argb.push(*palette.get(index).unwrap_or(&0xffff_00ffu32.cast_signed()));
                }
            }
            4 => {
                for x in 0..width {
                    let packed = data[row_offset + x / 2];
                    let index = if x % 2 == 0 { packed >> 4 } else { packed & 0x0f } as usize;
                    argb.push(*palette.get(index).unwrap_or(&0xffff_00ffu32.cast_signed()));
                }
            }
            1 => {
                for x in 0..width {
                    let packed = data[row_offset + x / 8];
                    let index = ((packed >> (7 - (x % 8))) & 1) as usize;
                    argb.push(*palette.get(index).unwrap_or(&0xffff_00ffu32.cast_signed()));
                }
            }
            _ => anyhow::bail!("unsupported BMP bit depth {bits_per_pixel}"),
        }
    }

    Ok(DecodedImage {
        width: width as i32,
        height: height as i32,
        argb,
    })
}

fn bmp_palette(data: &[u8], dib_size: usize, bits_per_pixel: u16, colors_used: usize) -> anyhow::Result<Vec<i32>> {
    if bits_per_pixel > 8 {
        return Ok(Vec::new());
    }

    let max_colors = 1usize << bits_per_pixel;
    let colors = if colors_used == 0 { max_colors } else { colors_used.min(max_colors) };
    let palette_offset = 14 + dib_size;
    let palette_bytes = colors * 4;
    if palette_offset + palette_bytes > data.len() {
        anyhow::bail!("truncated BMP palette");
    }

    let mut palette = Vec::with_capacity(colors);
    for index in 0..colors {
        let offset = palette_offset + index * 4;
        palette.push(argb_pixel(255, data[offset + 2], data[offset + 1], data[offset]));
    }

    Ok(palette)
}

fn read_u16_le(data: &[u8], offset: usize) -> anyhow::Result<u16> {
    let Some(bytes) = data.get(offset..offset + 2) else {
        anyhow::bail!("truncated u16");
    };
    Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
}

fn read_u32_le(data: &[u8], offset: usize) -> anyhow::Result<u32> {
    let Some(bytes) = data.get(offset..offset + 4) else {
        anyhow::bail!("truncated u32");
    };
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

fn read_i32_le(data: &[u8], offset: usize) -> anyhow::Result<i32> {
    let Some(bytes) = data.get(offset..offset + 4) else {
        anyhow::bail!("truncated i32");
    };
    Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
}

fn argb_pixel(a: u8, r: u8, g: u8, b: u8) -> i32 {
    ((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32) as i32
}

fn compose_screen(dst: u32, src: i32, process_alpha: bool) -> u32 {
    let src = src as u32;
    if !process_alpha {
        return src | 0xff00_0000;
    }

    let alpha = (src >> 24) & 0xff;
    if alpha == 0xff {
        return src;
    }
    if alpha == 0 {
        return dst;
    }

    let inv = 255 - alpha;
    let r = (((src >> 16) & 0xff) * alpha + ((dst >> 16) & 0xff) * inv) / 255;
    let g = (((src >> 8) & 0xff) * alpha + ((dst >> 8) & 0xff) * inv) / 255;
    let b = ((src & 0xff) * alpha + (dst & 0xff) * inv) / 255;

    0xff00_0000 | (r << 16) | (g << 8) | b
}

#[cfg_attr(not(feature = "desktop-window"), allow(dead_code))]
fn pixel_hash(pixels: &[u32]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for pixel in pixels {
        hash ^= *pixel as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
