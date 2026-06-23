use std::{collections::BTreeMap, env, fs::File, io::Read, path::Path};

use anyhow::{Context, bail};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

pub enum StartType<'a> {
    Jar(&'a Path),
    JarBytes { name: &'a Path, bytes: &'a [u8] },
    Class(&'a Path),
}

pub(crate) enum EntryPoint {
    MainClass(String),
    Midlet(String),
}

pub(crate) fn screen_size_for_start_type(start_type: &StartType<'_>) -> anyhow::Result<(usize, usize)> {
    if let Some(value) = env::var_os("RUSTJAVA_SCREEN") {
        let value = value.to_string_lossy();
        return parse_screen_size(&value).with_context(|| format!("parse RUSTJAVA_SCREEN={value}"));
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

pub(crate) fn read_jar_manifest(jar_path: &Path) -> anyhow::Result<BTreeMap<String, String>> {
    let file = File::open(jar_path).with_context(|| format!("open jar {}", jar_path.display()))?;
    let mut zip = zip::ZipArchive::new(file).with_context(|| format!("read jar {}", jar_path.display()))?;
    let mut manifest = zip.by_name("META-INF/MANIFEST.MF").context("read META-INF/MANIFEST.MF")?;

    let mut raw_manifest = String::new();
    manifest.read_to_string(&mut raw_manifest).context("decode manifest as UTF-8")?;
    parse_manifest(&raw_manifest)
}

pub(crate) fn read_jar_manifest_bytes(bytes: &[u8]) -> anyhow::Result<BTreeMap<String, String>> {
    let cursor = std::io::Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(cursor).context("read in-memory jar")?;
    let mut manifest = zip.by_name("META-INF/MANIFEST.MF").context("read META-INF/MANIFEST.MF")?;

    let mut raw_manifest = String::new();
    manifest.read_to_string(&mut raw_manifest).context("decode manifest as UTF-8")?;
    parse_manifest(&raw_manifest)
}

pub(crate) async fn get_jar_entry_point(jvm: &Jvm, jar_path: &Path) -> Result<EntryPoint> {
    let filename = JavaLangString::from_rust_string(jvm, jar_path.to_str().unwrap()).await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (filename,)).await?;
    let jar_file = jvm.new_class("java/util/jar/JarFile", "(Ljava/io/File;)V", (file,)).await?;

    let manifest = jvm.invoke_virtual(&jar_file, "getManifest", "()Ljava/util/jar/Manifest;", ()).await?;
    let attributes = jvm
        .invoke_virtual(&manifest, "getMainAttributes", "()Ljava/util/jar/Attributes;", ())
        .await?;

    if let Some(main_class) = manifest_value(jvm, &attributes, "Main-Class").await? {
        return Ok(EntryPoint::MainClass(main_class));
    }

    if let Some(midlet) = manifest_value(jvm, &attributes, "MIDlet-1").await? {
        let Some(class_name) = midlet.splitn(3, ',').nth(2).map(str::trim).filter(|value| !value.is_empty()) else {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "Malformed MIDlet-1 manifest entry")
                .await);
        };

        return Ok(EntryPoint::Midlet(class_name.to_string()));
    }

    Err(jvm
        .exception("java/lang/IllegalArgumentException", "Jar manifest has neither Main-Class nor MIDlet-1")
        .await)
}

fn parse_screen_size(value: &str) -> anyhow::Result<(usize, usize)> {
    let Some((width, height)) = value.split_once(['x', 'X']) else {
        bail!("expected WIDTHxHEIGHT");
    };

    let width = width.trim().parse::<usize>().context("parse screen width")?;
    let height = height.trim().parse::<usize>().context("parse screen height")?;
    if width < 80 || height < 80 || width > 1000 || height > 1000 {
        bail!("screen size is outside the supported range: {width}x{height}");
    }

    Ok((width, height))
}

fn screen_size_from_text(text: &str) -> Option<(usize, usize)> {
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

fn scan_jar_image_dimensions(jar_path: &Path) -> anyhow::Result<Vec<(u32, u32)>> {
    let file = File::open(jar_path).with_context(|| format!("open jar {}", jar_path.display()))?;
    let mut zip = zip::ZipArchive::new(file).with_context(|| format!("read jar {}", jar_path.display()))?;
    let mut dimensions = Vec::new();

    for index in 0..zip.len() {
        let mut entry = zip.by_index(index).with_context(|| format!("read jar entry #{index}"))?;
        if entry.is_dir() {
            continue;
        }

        let entry_name = entry
            .name()
            .map(|name| name.into_owned())
            .unwrap_or_else(|error| format!("<undecodable name: {error}>"));
        let mut data = Vec::with_capacity(entry.size().min(2 * 1024 * 1024) as usize);
        entry.read_to_end(&mut data).with_context(|| format!("read jar entry {entry_name}"))?;
        collect_image_dimensions(&data, &mut dimensions);
    }

    Ok(dimensions)
}

fn scan_jar_image_dimensions_from_bytes(bytes: &[u8]) -> anyhow::Result<Vec<(u32, u32)>> {
    let cursor = std::io::Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(cursor).context("read in-memory jar")?;
    let mut dimensions = Vec::new();

    for index in 0..zip.len() {
        let mut entry = zip.by_index(index).with_context(|| format!("read jar entry #{index}"))?;
        if entry.is_dir() {
            continue;
        }

        let entry_name = entry
            .name()
            .map(|name| name.into_owned())
            .unwrap_or_else(|error| format!("<undecodable name: {error}>"));
        let mut data = Vec::with_capacity(entry.size().min(2 * 1024 * 1024) as usize);
        entry.read_to_end(&mut data).with_context(|| format!("read jar entry {entry_name}"))?;
        collect_image_dimensions(&data, &mut dimensions);
    }

    Ok(dimensions)
}

fn collect_image_dimensions(data: &[u8], dimensions: &mut Vec<(u32, u32)>) {
    collect_png_dimensions(data, dimensions);
    if let Some(size) = jpeg_dimensions(data) {
        dimensions.push(size);
    }
}

fn collect_png_dimensions(data: &[u8], dimensions: &mut Vec<(u32, u32)>) {
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

fn jpeg_dimensions(data: &[u8]) -> Option<(u32, u32)> {
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

fn screen_size_from_image_dimensions(dimensions: &[(u32, u32)]) -> Option<(usize, usize)> {
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

async fn manifest_value(
    jvm: &Jvm,
    attributes: &ClassInstanceRef<java_runtime::classes::java::util::jar::Attributes>,
    name: &str,
) -> Result<Option<String>> {
    let value: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(
            attributes,
            "getValue",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (JavaLangString::from_rust_string(jvm, name).await?,),
        )
        .await?;

    if value.is_null() {
        Ok(None)
    } else {
        Ok(Some(JavaLangString::to_rust_string(jvm, &value).await?))
    }
}

fn parse_manifest(raw_manifest: &str) -> anyhow::Result<BTreeMap<String, String>> {
    let mut unfolded: Vec<String> = Vec::new();
    for line in raw_manifest.replace("\r\n", "\n").replace('\r', "\n").split('\n') {
        if line.is_empty() {
            continue;
        }

        if let Some(continuation) = line.strip_prefix(' ') {
            let Some(last) = unfolded.last_mut() else {
                bail!("manifest continuation without a previous header");
            };
            last.push_str(continuation);
        } else {
            unfolded.push(line.to_string());
        }
    }

    let mut values = BTreeMap::new();
    for line in unfolded {
        let Some((key, value)) = line.split_once(':') else {
            bail!("manifest header is missing ':' separator: {line}");
        };

        values.insert(key.trim().to_string(), value.trim_start().to_string());
    }

    Ok(values)
}
