use alloc::{
    string::{String as RustString, ToString},
    sync::Arc,
    vec::Vec,
};
use core::{
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU64, Ordering},
};

use java_class_proto::JavaFieldProto;
use java_constants::FieldAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};
use parking_lot::Mutex;

use super::types::{Group, Node, Object3D};

pub(super) const M3G_IDENTIFIER: &[u8; 12] = b"\xABJSR184\xBB\r\n\x1A\n";
pub(super) const PNG_IDENTIFIER: &[u8; 8] = b"\x89PNG\r\n\x1A\n";
pub(super) const JPEG_IDENTIFIER: &[u8; 2] = b"\xFF\xD8";
pub(super) static M3G_RENDER_FRAME: AtomicU64 = AtomicU64::new(1);
pub(super) static M3G_RENDER_LAST_END_MS: AtomicU64 = AtomicU64::new(0);
pub(super) static M3G_RENDER_LAST_DIAG_MS: AtomicU64 = AtomicU64::new(0);
pub(super) static M3G_RENDER_DIAGNOSTICS: Mutex<Vec<RustString>> = Mutex::new(Vec::new());
pub(super) static M3G_VERTEX_VALUES_CACHE: Mutex<Vec<VertexValuesCacheEntry>> = Mutex::new(Vec::new());
pub(super) static M3G_VERTEX_VEC3_CACHE: Mutex<Vec<VertexVec3CacheEntry>> = Mutex::new(Vec::new());
pub(super) static M3G_VERTEX_VEC2_CACHE: Mutex<Vec<VertexVec2CacheEntry>> = Mutex::new(Vec::new());
pub(super) static M3G_VERTEX_COLOR_CACHE: Mutex<Vec<VertexColorCacheEntry>> = Mutex::new(Vec::new());
pub(super) static M3G_TRIANGLE_INDEX_CACHE: Mutex<Vec<TriangleIndexCacheEntry>> = Mutex::new(Vec::new());
pub(super) static M3G_IMAGE_PIXEL_CACHE: Mutex<Vec<ImagePixelCacheEntry>> = Mutex::new(Vec::new());

pub(super) const FNV_OFFSET_BASIS_64: u64 = 0xcbf29ce484222325;
pub(super) const FNV_PRIME_64: u64 = 0x100000001b3;
pub(super) const M3G_CACHE_LIMIT: usize = 2048;
pub(super) const M3G_RENDER_DIAGNOSTIC_LIMIT: usize = 16;
pub(super) const M3G_TEXTURE_UNITS: i32 = 2;
pub(super) const M3G_MAX_LIGHTS: i32 = 8;
pub(super) const M3G_MAX_TRANSFORMS_PER_VERTEX: i32 = 4;

pub(super) fn static_int_field(name: &str) -> JavaFieldProto {
    JavaFieldProto::new(name, "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL)
}

pub(super) async fn put_static_ints(jvm: &Jvm, class: &str, values: &[(&str, i32)]) -> Result<()> {
    for (name, value) in values {
        jvm.put_static_field(class, name, "I", *value).await?;
    }
    Ok(())
}

#[derive(Default)]
pub(super) struct M3gIdentityHasher(u64);

impl Hasher for M3gIdentityHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut hash = if self.0 == 0 { FNV_OFFSET_BASIS_64 } else { self.0 };
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(FNV_PRIME_64);
        }
        self.0 = hash;
    }
}

pub(super) struct VertexValuesCacheEntry {
    pub(super) key: u64,
    pub(super) version: i32,
    pub(super) component_count: usize,
    pub(super) vertex_count: usize,
    pub(super) values: Arc<Vec<f32>>,
}

pub(super) struct VertexVec3CacheEntry {
    pub(super) key: u64,
    pub(super) version: i32,
    pub(super) scale_bits: u32,
    pub(super) bias_bits: [u32; 3],
    pub(super) values: Arc<Vec<[f32; 3]>>,
}

pub(super) struct VertexVec2CacheEntry {
    pub(super) key: u64,
    pub(super) version: i32,
    pub(super) scale_bits: u32,
    pub(super) bias_bits: [u32; 2],
    pub(super) values: Arc<Vec<[f32; 2]>>,
}

pub(super) struct VertexColorCacheEntry {
    pub(super) key: u64,
    pub(super) version: i32,
    pub(super) default_color: i32,
    pub(super) values: Arc<Vec<i32>>,
}

pub(super) struct TriangleIndexCacheEntry {
    pub(super) key: u64,
    pub(super) values: Arc<Vec<[usize; 3]>>,
}

pub(super) struct ImagePixelCacheEntry {
    pub(super) key: u64,
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) values: Arc<Vec<i32>>,
}

pub fn latest_render_diagnostics() -> RustString {
    let diagnostics = M3G_RENDER_DIAGNOSTICS.lock();
    if diagnostics.is_empty() {
        return "m3g: waiting for render".to_string();
    }

    let mut output = RustString::new();
    for line in diagnostics.iter() {
        if !output.is_empty() {
            output.push('\n');
        }
        output.push_str(line);
    }
    output
}

pub(super) struct M3gPickHit {
    pub(super) node: ClassInstanceRef<Node>,
    pub(super) distance: f32,
    pub(super) submesh_index: i32,
    pub(super) texture_s: f32,
    pub(super) texture_t: f32,
    pub(super) texture_s1: f32,
    pub(super) texture_t1: f32,
    pub(super) normal: [f32; 3],
}

#[derive(Default)]
pub(super) struct M3gDuplicateMap {
    pub(super) entries: Vec<(ClassInstanceRef<Object3D>, ClassInstanceRef<Object3D>)>,
    pub(super) copied: usize,
    pub(super) shared: usize,
}

impl M3gDuplicateMap {
    pub(super) fn get(&self, source: &ClassInstanceRef<Object3D>) -> Option<ClassInstanceRef<Object3D>> {
        self.entries
            .iter()
            .find(|(candidate, _)| same_instance(candidate, source))
            .map(|(_, duplicate)| duplicate.clone())
    }

    pub(super) fn insert(&mut self, source: &ClassInstanceRef<Object3D>, duplicate: &ClassInstanceRef<Object3D>) {
        self.entries.push((source.clone(), duplicate.clone()));
        self.copied += 1;
    }

    pub(super) fn note_shared(&mut self) {
        self.shared += 1;
    }
}

pub(super) enum M3gSectionData<'a> {
    Borrowed(&'a [u8]),
    Owned(Vec<u8>),
}

impl M3gSectionData<'_> {
    pub(super) fn as_slice(&self) -> &[u8] {
        match self {
            Self::Borrowed(data) => data,
            Self::Owned(data) => data,
        }
    }
}

pub(super) fn decode_m3g_section_data(compression: u8, data: &[u8], inflated_len: usize) -> core::result::Result<M3gSectionData<'_>, &'static str> {
    match compression {
        0 => {
            if data.len() != inflated_len {
                return Err("bad M3G uncompressed section length");
            }
            Ok(M3gSectionData::Borrowed(data))
        }
        1 => {
            let inflated = miniz_oxide::inflate::decompress_to_vec_zlib_with_limit(data, inflated_len).map_err(|_| "bad compressed M3G section")?;
            if inflated.len() != inflated_len {
                return Err("bad M3G inflated section length");
            }
            Ok(M3gSectionData::Owned(inflated))
        }
        _ => Err("unsupported M3G compression scheme"),
    }
}

pub(super) fn read_u32_at(data: &[u8], offset: usize) -> u32 {
    if offset + 4 > data.len() {
        return 0;
    }
    u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

pub(super) fn same_instance<T, U>(left: &ClassInstanceRef<T>, right: &ClassInstanceRef<U>) -> bool {
    match (&left.instance, &right.instance) {
        (Some(left), Some(right)) => left.equals(&**right).unwrap_or(false),
        (None, None) => true,
        _ => false,
    }
}

pub(super) fn is_mesh_class(name: &str) -> bool {
    matches!(
        name,
        "javax/microedition/m3g/Mesh" | "javax/microedition/m3g/SkinnedMesh" | "javax/microedition/m3g/MorphingMesh"
    )
}

pub(super) fn is_skinned_mesh_class(name: &str) -> bool {
    name == "javax/microedition/m3g/SkinnedMesh"
}

pub(super) async fn skinned_mesh_skeleton(jvm: &Jvm, node: &ClassInstanceRef<Node>) -> Result<ClassInstanceRef<Group>> {
    if !is_skinned_mesh_class(&node.class_definition().name()) {
        return Ok(null_ref());
    }
    Ok(jvm
        .get_field(node, "skeleton", "Ljavax/microedition/m3g/Group;")
        .await
        .unwrap_or_else(|_| null_ref()))
}

pub(super) async fn push_skinned_skeleton(
    jvm: &Jvm,
    node: &ClassInstanceRef<Node>,
    stack: &mut Vec<(ClassInstanceRef<Node>, [f32; 16], bool, bool, f32)>,
    world_matrix: [f32; 16],
    enabled: bool,
    alpha: f32,
) -> Result<()> {
    let skeleton = skinned_mesh_skeleton(jvm, node).await?;
    if !skeleton.is_null() {
        stack.push((cast_ref(&skeleton), world_matrix, true, enabled, alpha));
    }
    Ok(())
}

pub(super) fn cast_ref<T, U>(value: &ClassInstanceRef<T>) -> ClassInstanceRef<U> {
    ClassInstanceRef::new(value.instance.clone())
}

pub(super) fn null_ref<T>() -> ClassInstanceRef<T> {
    ClassInstanceRef::new(None)
}

pub(super) fn instance_key<T>(value: &ClassInstanceRef<T>) -> u64 {
    let Some(instance) = &value.instance else {
        return 0;
    };
    let mut hasher = M3gIdentityHasher::default();
    instance.hash(&mut hasher);
    hasher.finish()
}

pub(super) fn push_cache_entry<T>(cache: &mut Vec<T>, entry: T) {
    if cache.len() >= M3G_CACHE_LIMIT {
        cache.remove(0);
    }
    cache.push(entry);
}

pub(super) fn publish_render_diagnostic(now_ms: u64, total_ms: u64, line: RustString) {
    let previous = M3G_RENDER_LAST_DIAG_MS.load(Ordering::Relaxed);
    if total_ms < 50 && now_ms.saturating_sub(previous) < 250 {
        return;
    }
    M3G_RENDER_LAST_DIAG_MS.store(now_ms, Ordering::Relaxed);

    let mut diagnostics = M3G_RENDER_DIAGNOSTICS.lock();
    if diagnostics.len() >= M3G_RENDER_DIAGNOSTIC_LIMIT {
        diagnostics.remove(0);
    }
    diagnostics.push(line);
}
