use alloc::{string::String as AllocString, vec::Vec};

pub fn filesystem_path_candidates(path: &str) -> Vec<AllocString> {
    let mut out = Vec::new();
    let mut push = |value: AllocString| {
        if !value.is_empty() && !out.iter().any(|existing| existing == &value) {
            out.push(value);
        }
    };

    push(AllocString::from(path));
    let unix = path.replace('\\', "/");
    push(unix.clone());
    if let Some(rest) = unix.strip_prefix('/') {
        push(AllocString::from(rest));
    }
    if let Some(rest) = unix.strip_prefix("./") {
        push(AllocString::from(rest));
        if let Some(r2) = rest.strip_prefix('/') {
            push(AllocString::from(r2));
        }
    }
    out
}

pub fn resource_path_candidates(path: &str) -> Vec<AllocString> {
    let mut out = filesystem_path_candidates(path);
    let unix = path.replace('\\', "/");
    let trimmed = unix.trim_start_matches('/');
    if let Some((_, name)) = trimmed.rsplit_once('/') {
        if !name.is_empty() && !out.iter().any(|existing| existing == name) {
            out.push(AllocString::from(name));
        }
    }
    out
}
