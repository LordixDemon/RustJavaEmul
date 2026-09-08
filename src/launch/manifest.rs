use std::{collections::BTreeMap, fs, path::Path};

use anyhow::{Context, bail};
use java_runtime::zip::ZipArchive;

use super::EntryPoint;

pub(crate) fn read_jar_manifest(jar_path: &Path) -> anyhow::Result<BTreeMap<String, String>> {
    let bytes = fs::read(jar_path).with_context(|| format!("open jar {}", jar_path.display()))?;
    let zip = ZipArchive::new(bytes).map_err(|e| anyhow::anyhow!("{e}"))?;
    manifest_from_zip(&zip)
}

pub(crate) fn read_jar_manifest_bytes(bytes: &[u8]) -> anyhow::Result<BTreeMap<String, String>> {
    let zip = ZipArchive::new(bytes).map_err(|e| anyhow::anyhow!("{e}"))?;
    manifest_from_zip(&zip)
}

pub(super) fn manifest_from_zip<T: AsRef<[u8]>>(zip: &ZipArchive<T>) -> anyhow::Result<BTreeMap<String, String>> {
    let bytes = read_zip_manifest(zip).unwrap_or_default();
    let mut values = parse_manifest(&decode_manifest(&bytes))?;
    let entry = entry_point_from_manifest(&values);
    let class_missing = match &entry {
        Some(EntryPoint::Midlet(name)) | Some(EntryPoint::MainClass(name)) => {
            let class_file = format!("{}.class", name.replace('.', "/"));
            !zip_contains_file(zip, &class_file)
        }
        None => true,
    };
    if class_missing {
        if let Some(midlet) = infer_midlet_class(zip) {
            values.insert("MIDlet-1".to_string(), format!(",,{midlet}"));
        }
    }
    Ok(values)
}

pub(super) fn zip_contains_file<T: AsRef<[u8]>>(zip: &ZipArchive<T>, path: &str) -> bool {
    let target = normalize_zip_name(path);
    for index in 0..zip.len() {
        if let Ok(entry) = zip.by_index(index) {
            if let Ok(name) = entry.name() {
                if names_equal(name, &target) {
                    return true;
                }
            }
        }
    }
    false
}

pub(super) fn normalize_zip_name(name: &str) -> String {
    name.trim_start_matches('/').replace('\\', "/")
}

pub(super) fn names_equal(left: &str, right: &str) -> bool {
    let left = normalize_zip_name(left);
    let right = normalize_zip_name(right);
    left == right || left.eq_ignore_ascii_case(&right)
}

pub(super) fn is_manifest_name(name: &str) -> bool {
    let name = normalize_zip_name(name);
    let base = name.rsplit('/').next().unwrap_or(&name);
    base.eq_ignore_ascii_case("MANIFEST.MF") || base.eq_ignore_ascii_case("MANIFEST.TXT") || base.eq_ignore_ascii_case("MANIFEST")
}

pub(super) fn read_zip_manifest<T: AsRef<[u8]>>(zip: &ZipArchive<T>) -> Option<Vec<u8>> {
    let mut manifest_mf = None;
    let mut manifest_any = None;
    for index in 0..zip.len() {
        let Ok(entry) = zip.by_index(index) else {
            continue;
        };
        if entry.is_dir() {
            continue;
        }
        let Ok(entry_name) = entry.name() else {
            continue;
        };
        if !is_manifest_name(entry_name) {
            continue;
        }
        let Ok(raw) = entry.extract() else {
            continue;
        };
        if entry_name.to_ascii_uppercase().ends_with("MANIFEST.MF") {
            return Some(raw);
        }
        if manifest_mf.is_none() && names_equal(entry_name, "META-INF/MANIFEST.MF") {
            manifest_mf = Some(raw);
        } else if manifest_any.is_none() {
            manifest_any = Some(raw);
        }
    }
    manifest_mf.or(manifest_any)
}

pub(super) fn is_midlet_super(name: &str) -> bool {
    let name = name.replace('.', "/");
    name == "javax/microedition/midlet/MIDlet"
        || name.ends_with("/MIDlet")
        || name.ends_with("/Midlet")
        || name.ends_with("MIDlet")
        || name.ends_with("Midlet")
}

pub(super) fn infer_midlet_class<T: AsRef<[u8]>>(zip: &ZipArchive<T>) -> Option<String> {
    let mut exact = Vec::new();
    let mut start_app = Vec::new();
    for index in 0..zip.len() {
        let Ok(entry) = zip.by_index(index) else {
            continue;
        };
        if entry.is_dir() {
            continue;
        }
        let Ok(entry_name) = entry.name() else {
            continue;
        };
        if !entry_name.to_ascii_lowercase().ends_with(".class") {
            continue;
        }
        let Ok(bytes) = entry.extract() else {
            continue;
        };
        let Some(info) = classfile::ClassInfo::parse(&bytes) else {
            continue;
        };
        let this_class = info.this_class.to_string();
        if info.super_class.as_deref().is_some_and(|name| is_midlet_super(name.as_str())) {
            exact.push(this_class.clone());
        }
        if info
            .methods
            .iter()
            .any(|method| method.name.as_str() == "startApp" && method.descriptor.as_str() == "()V")
        {
            start_app.push(this_class);
        }
    }
    exact.into_iter().next().or_else(|| start_app.into_iter().next())
}

pub(super) fn parse_midlet_1(value: &str) -> Option<String> {
    value
        .splitn(3, ',')
        .nth(2)
        .map(str::trim)
        .filter(|class_name| !class_name.is_empty())
        .map(str::to_string)
}

pub(super) fn entry_point_from_manifest(values: &BTreeMap<String, String>) -> Option<EntryPoint> {
    if let Some(main_class) = values.get("Main-Class").map(|value| value.trim()).filter(|value| !value.is_empty()) {
        return Some(EntryPoint::MainClass(main_class.to_string()));
    }
    values.get("MIDlet-1").and_then(|value| parse_midlet_1(value)).map(EntryPoint::Midlet)
}

pub(super) fn decode_manifest(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

pub(super) fn parse_manifest(raw_manifest: &str) -> anyhow::Result<BTreeMap<String, String>> {
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
            continue;
        };
        let key = key.trim();
        if key.is_empty() {
            continue;
        }

        values.insert(key.to_string(), value.trim_start().to_string());
    }

    Ok(values)
}
