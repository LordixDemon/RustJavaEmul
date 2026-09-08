use std::path::Path;

use anyhow::{Context, bail};

use super::StartType;

pub(crate) fn screen_size_for_start_type(start_type: &StartType<'_>) -> anyhow::Result<(usize, usize)> {
    let config = crate::config::get();
    if let Some(error) = &config.screen_parse_error {
        bail!("{error}");
    }
    if let Some(size) = config.screen {
        return Ok(size);
    }

    if let StartType::Jar(jar_path) = start_type {
        if let Some(size) = jar_path.file_name().and_then(|name| screen_size_from_text(&name.to_string_lossy())) {
            return Ok(size);
        }

        let image_dimensions = scan_jar_image_dimensions(jar_path)?;
        if let Some(size) = screen_size_from_image_dimensions(&image_dimensions) {
            return Ok(size);
        }
    }
    if let StartType::JarBytes { name, bytes } = start_type {
        if let Some(size) = name.file_name().and_then(|name| screen_size_from_text(&name.to_string_lossy())) {
            return Ok(size);
        }

        let image_dimensions = scan_jar_image_dimensions_from_bytes(bytes)?;
        if let Some(size) = screen_size_from_image_dimensions(&image_dimensions) {
            return Ok(size);
        }
    }

    Ok((240, 320))
}

pub(super) fn screen_size_from_text(text: &str) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        if *byte != b'x' && *byte != b'X' {
            continue;
        }

        let mut left = index;
        while left > 0 && bytes[left - 1].is_ascii_digit() {
            left -= 1;
        }

        let mut right = index + 1;
        while right < bytes.len() && bytes[right].is_ascii_digit() {
            right += 1;
        }

        if left == index || right == index + 1 {
            continue;
        }

        let width = text[left..index].parse::<usize>().ok()?;
        let height = text[index + 1..right].parse::<usize>().ok()?;
        if width >= 80 && height >= 80 && width <= 1000 && height <= 1000 {
            return Some((width, height));
        }
    }

    None
}

pub(super) fn scan_jar_image_dimensions(jar_path: &Path) -> anyhow::Result<Vec<(u32, u32)>> {
    let bytes = std::fs::read(jar_path).with_context(|| format!("open jar {}", jar_path.display()))?;
    scan_jar_image_dimensions_from_bytes(&bytes)
}

pub(super) fn scan_jar_image_dimensions_from_bytes(bytes: &[u8]) -> anyhow::Result<Vec<(u32, u32)>> {
    let zip = java_runtime::zip::ZipArchive::new(bytes).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut dimensions = Vec::new();

    for index in 0..zip.len() {
        let Ok(entry) = zip.by_index(index) else {
            continue;
        };
        if entry.is_dir() {
            continue;
        }

        let Ok(data) = entry.extract() else {
            continue;
        };
        collect_image_dimensions(&data, &mut dimensions);
    }

    Ok(dimensions)
}

pub(super) fn collect_image_dimensions(data: &[u8], dimensions: &mut Vec<(u32, u32)>) {
    collect_png_dimensions(data, dimensions);
    if let Some(size) = jpeg_dimensions(data) {
        dimensions.push(size);
    }
}

pub(super) fn collect_png_dimensions(data: &[u8], dimensions: &mut Vec<(u32, u32)>) {
    const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

    if data.len() < 24 {
        return;
    }

    for offset in 0..=data.len() - 24 {
        if &data[offset..offset + 8] != PNG_SIGNATURE || &data[offset + 12..offset + 16] != b"IHDR" {
            continue;
        }

        let width = u32::from_be_bytes(data[offset + 16..offset + 20].try_into().unwrap());
        let height = u32::from_be_bytes(data[offset + 20..offset + 24].try_into().unwrap());
        if width >= 16 && height >= 16 && width <= 1000 && height <= 1000 {
            dimensions.push((width, height));
        }
    }
}

pub(super) fn jpeg_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 4 || data[0] != 0xff || data[1] != 0xd8 {
        return None;
    }

    let mut index = 2usize;
    while index + 4 <= data.len() {
        while index < data.len() && data[index] != 0xff {
            index += 1;
        }
        while index < data.len() && data[index] == 0xff {
            index += 1;
        }
        if index >= data.len() {
            return None;
        }

        let marker = data[index];
        index += 1;
        if marker == 0xd9 || marker == 0xda {
            return None;
        }
        if matches!(marker, 0x01 | 0xd0..=0xd7) {
            continue;
        }
        if index + 2 > data.len() {
            return None;
        }

        let length = u16::from_be_bytes([data[index], data[index + 1]]) as usize;
        if length < 2 || index + length > data.len() {
            return None;
        }

        if matches!(marker, 0xc0..=0xc3 | 0xc5..=0xc7 | 0xc9..=0xcb | 0xcd..=0xcf) {
            if length < 7 {
                return None;
            }
            let height = u16::from_be_bytes([data[index + 3], data[index + 4]]) as u32;
            let width = u16::from_be_bytes([data[index + 5], data[index + 6]]) as u32;
            if width >= 16 && height >= 16 && width <= 1000 && height <= 1000 {
                return Some((width, height));
            }
            return None;
        }

        index += length;
    }

    None
}

pub(super) fn screen_size_from_image_dimensions(dimensions: &[(u32, u32)]) -> Option<(usize, usize)> {
    let dimensions = dimensions
        .iter()
        .copied()
        .filter(|(width, height)| *width >= 80 || *height >= 80)
        .collect::<Vec<_>>();

    if dimensions
        .iter()
        .any(|(width, height)| (*width >= 240 && *height >= 320) || (*width >= 320 && *height >= 240))
    {
        return Some((240, 320));
    }
    if dimensions.iter().any(|(_, height)| *height == 220) {
        return Some((176, 220));
    }
    if dimensions.iter().any(|(_, height)| *height == 208) {
        return Some((176, 208));
    }
    if dimensions.iter().any(|(_, height)| *height == 160) {
        return Some((128, 160));
    }

    None
}
