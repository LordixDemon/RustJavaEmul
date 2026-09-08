use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use png::{BitDepth, ColorType, Encoder};

static SCREENSHOT_SEQ: AtomicU64 = AtomicU64::new(1);

pub(super) struct ScreenshotSink {
    dir: PathBuf,
    every: Option<Duration>,
    last: Instant,
}

impl ScreenshotSink {
    pub(super) fn from_env() -> Self {
        let config = crate::config::get();
        Self {
            dir: config.screenshot_dir.clone(),
            every: config.screenshot_every_secs.map(Duration::from_secs),
            last: Instant::now() - Duration::from_secs(3600),
        }
    }

    pub(super) fn save_lcd(&mut self, reason: &str, width: usize, height: usize, stride: usize, pixels: &[u32]) {
        match save_lcd_png(&self.dir, reason, width, height, stride, pixels) {
            Ok(path) => eprintln!("lcd screenshot {} {}x{} {}", path.display(), width, height, reason),
            Err(error) => eprintln!("lcd screenshot failed: {error:#}"),
        }
        self.last = Instant::now();
    }

    pub(super) fn save_if_due(&mut self, width: usize, height: usize, stride: usize, pixels: &[u32]) {
        let Some(every) = self.every else {
            return;
        };
        if self.last.elapsed() < every {
            return;
        }
        self.save_lcd("interval", width, height, stride, pixels);
    }
}

fn save_lcd_png(dir: &Path, reason: &str, width: usize, height: usize, stride: usize, pixels: &[u32]) -> anyhow::Result<PathBuf> {
    anyhow::ensure!(width > 0 && height > 0 && stride >= width, "invalid lcd size");
    fs::create_dir_all(dir)?;
    let seq = SCREENSHOT_SEQ.fetch_add(1, Ordering::Relaxed);
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
    let path = dir.join(format!("lcd-{stamp}-{seq:04}-{reason}.png"));
    fs::write(&path, encode_lcd_png(width, height, stride, pixels)?)?;
    Ok(path)
}

pub(super) fn encode_lcd_png(width: usize, height: usize, stride: usize, pixels: &[u32]) -> anyhow::Result<Vec<u8>> {
    anyhow::ensure!(width > 0 && height > 0 && stride >= width, "invalid lcd size");
    let rgba = packed_rgba(width, height, stride, pixels);
    let mut out = Vec::new();
    {
        let mut encoder = Encoder::new(&mut out, width as u32, height as u32);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&rgba)?;
    }
    Ok(out)
}

fn packed_rgba(width: usize, height: usize, stride: usize, pixels: &[u32]) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(width.saturating_mul(height).saturating_mul(4));
    for row in 0..height {
        let start = row * stride;
        let end = start + width;
        let row_pixels = pixels.get(start..end).unwrap_or(&[]);
        for pixel in row_pixels {
            let pixel = *pixel;
            rgba.push(((pixel >> 16) & 0xff) as u8);
            rgba.push(((pixel >> 8) & 0xff) as u8);
            rgba.push((pixel & 0xff) as u8);
            let alpha = ((pixel >> 24) & 0xff) as u8;
            rgba.push(if alpha == 0 { 0xff } else { alpha });
        }
        if row_pixels.len() < width {
            rgba.resize(rgba.len() + (width - row_pixels.len()) * 4, 0);
        }
    }
    rgba
}
