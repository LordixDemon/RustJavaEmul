use alloc::{boxed::Box, string::String as RustString, sync::Arc, vec, vec::Vec};
use core::{
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU64, Ordering},
};

use jvm::{
    Array, ClassInstance, ClassInstanceRef, Jvm, Result,
    runtime::{JavaIoInputStream, JavaLangString},
};
use parking_lot::Mutex;

use crate::{
    RuntimeContext,
    classes::java::{
        io::InputStream,
        lang::{ClassLoader, String},
    },
};

use super::{AffineTrans, Figure, Texture, Vector3D, mbac::NativeFigure, scene::RuntimeFigure, texture::NativeTexture};

static V3_TEXTURE_CACHE: Mutex<Vec<TextureCacheEntry>> = Mutex::new(Vec::new());
static V3_FIGURE_CACHE: Mutex<Vec<FigureCacheEntry>> = Mutex::new(Vec::new());
static V3_TEXTURE_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
static V3_TEXTURE_CACHE_MISSES: AtomicU64 = AtomicU64::new(0);
static V3_FIGURE_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
static V3_FIGURE_CACHE_MISSES: AtomicU64 = AtomicU64::new(0);

const FNV_OFFSET_BASIS_64: u64 = 0xcbf29ce484222325;
const FNV_PRIME_64: u64 = 0x100000001b3;
const V3_TEXTURE_CACHE_LIMIT: usize = 512;
const V3_FIGURE_CACHE_LIMIT: usize = 512;

#[derive(Default)]
struct V3IdentityHasher(u64);

impl Hasher for V3IdentityHasher {
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

struct TextureCacheEntry {
    key: u64,
    width: i32,
    height: i32,
    color_key: i32,
    texture: NativeTexture,
}

#[derive(Clone)]
struct CachedRuntimeFigure {
    vertex_count: usize,
    vertices: Arc<Vec<i16>>,
    poly_c3: Arc<Vec<i16>>,
    poly_c4: Arc<Vec<i16>>,
    poly_t3: Arc<Vec<i16>>,
    poly_t4: Arc<Vec<i16>>,
    colors: Arc<Vec<i32>>,
    patterns: Arc<Vec<i32>>,
    pattern_count: usize,
    pattern_slots: usize,
    bones: Arc<Vec<i32>>,
}

struct FigureCacheEntry {
    key: u64,
    figure: CachedRuntimeFigure,
}

#[derive(Clone, Copy, Default)]
pub(super) struct V3StorageCacheStats {
    pub(super) texture_hits: u64,
    pub(super) texture_misses: u64,
    pub(super) figure_hits: u64,
    pub(super) figure_misses: u64,
}

pub(super) fn storage_cache_stats() -> V3StorageCacheStats {
    V3StorageCacheStats {
        texture_hits: V3_TEXTURE_CACHE_HITS.load(Ordering::Relaxed),
        texture_misses: V3_TEXTURE_CACHE_MISSES.load(Ordering::Relaxed),
        figure_hits: V3_FIGURE_CACHE_HITS.load(Ordering::Relaxed),
        figure_misses: V3_FIGURE_CACHE_MISSES.load(Ordering::Relaxed),
    }
}

fn instance_key<T>(value: &ClassInstanceRef<T>) -> u64 {
    let Some(instance) = &value.instance else {
        return 0;
    };
    let mut hasher = V3IdentityHasher::default();
    instance.hash(&mut hasher);
    hasher.finish()
}

fn push_texture_cache_entry(entry: TextureCacheEntry) {
    let mut cache = V3_TEXTURE_CACHE.lock();
    if cache.len() >= V3_TEXTURE_CACHE_LIMIT {
        cache.remove(0);
    }
    cache.push(entry);
}

pub(super) fn invalidate_native_texture_cache<T>(texture: &ClassInstanceRef<T>) {
    let key = instance_key(texture);
    if key == 0 {
        return;
    }
    V3_TEXTURE_CACHE.lock().retain(|entry| entry.key != key);
}

pub(super) fn invalidate_runtime_figure_cache<T>(figure: &ClassInstanceRef<T>) {
    let key = instance_key(figure);
    if key == 0 {
        return;
    }
    V3_FIGURE_CACHE.lock().retain(|entry| entry.key != key);
}

pub(super) async fn put_int_array_field(jvm: &Jvm, this: &mut Box<dyn ClassInstance>, field: &str, values: Vec<i32>) -> Result<()> {
    let mut array = jvm.instantiate_array("I", values.len()).await?;
    jvm.store_array(&mut array, 0, values).await?;
    jvm.put_field(this, field, "[I", array).await
}

pub(super) async fn put_byte_array_field(jvm: &Jvm, this: &mut Box<dyn ClassInstance>, field: &str, values: Vec<i8>) -> Result<()> {
    let mut array = jvm.instantiate_array("B", values.len()).await?;
    jvm.store_array(&mut array, 0, values).await?;
    jvm.put_field(this, field, "[B", array).await
}

async fn put_short_array_field(jvm: &Jvm, this: &mut Box<dyn ClassInstance>, field: &str, values: Vec<i16>) -> Result<()> {
    let mut array = jvm.instantiate_array("S", values.len()).await?;
    jvm.store_array(&mut array, 0, values).await?;
    jvm.put_field(this, field, "[S", array).await
}

pub(super) async fn put_native_figure(jvm: &Jvm, this: &mut ClassInstanceRef<Figure>, figure: NativeFigure) -> Result<()> {
    jvm.put_field(this, "vertexCount", "I", figure.vertex_count).await?;
    put_short_array_field(jvm, this, "vertices", figure.vertices).await?;
    put_short_array_field(jvm, this, "polyC3", figure.poly_c3).await?;
    put_short_array_field(jvm, this, "polyC4", figure.poly_c4).await?;
    put_short_array_field(jvm, this, "polyT3", figure.poly_t3).await?;
    put_short_array_field(jvm, this, "polyT4", figure.poly_t4).await?;
    put_int_array_field(jvm, this, "colors", figure.colors).await?;
    put_int_array_field(jvm, this, "patterns", figure.patterns).await?;
    put_int_array_field(jvm, this, "bones", figure.bones).await?;
    jvm.put_field(this, "patternCount", "I", figure.pattern_count).await?;
    jvm.put_field(this, "patternSlots", "I", figure.pattern_slots).await?;
    jvm.put_field(this, "numPolyC3", "I", figure.num_poly_c3).await?;
    jvm.put_field(this, "numPolyC4", "I", figure.num_poly_c4).await?;
    jvm.put_field(this, "numPolyT3", "I", figure.num_poly_t3).await?;
    jvm.put_field(this, "numPolyT4", "I", figure.num_poly_t4).await?;
    jvm.put_field(this, "allMatsOr", "I", figure.all_mats_or).await?;
    jvm.put_field(this, "allMatsAnd", "I", figure.all_mats_and).await
}

pub(super) async fn load_byte_array(jvm: &Jvm, array: &ClassInstanceRef<Array<i8>>) -> Result<Vec<u8>> {
    if array.is_null() {
        return Ok(Vec::new());
    }

    let length = jvm.array_length(array).await?;
    let mut bytes = vec![0; length];
    jvm.array_raw_buffer(array).await?.read(0, &mut bytes)?;
    Ok(bytes)
}

pub(super) async fn load_resource_bytes_with_extensions(
    jvm: &Jvm,
    _: &mut RuntimeContext,
    name: &ClassInstanceRef<String>,
    extensions: &[&str],
) -> Result<Vec<u8>> {
    if name.is_null() {
        return Ok(Vec::new());
    }

    let name = JavaLangString::to_rust_string(jvm, name).await?;
    let resource_name = name.trim_start_matches('/');
    let loader: ClassInstanceRef<ClassLoader> = jvm
        .invoke_static("java/lang/ClassLoader", "getSystemClassLoader", "()Ljava/lang/ClassLoader;", ())
        .await?;

    let mut tried = Vec::new();
    if let Some(bytes) = try_load_resource(jvm, &loader, resource_name, &mut tried).await? {
        return Ok(bytes);
    }

    if !has_resource_extension(resource_name) {
        for extension in extensions {
            let mut candidate = RustString::from(resource_name);
            candidate.push_str(extension);

            if let Some(bytes) = try_load_resource(jvm, &loader, &candidate, &mut tried).await? {
                tracing::debug!("Mascot Capsule resource fallback: {resource_name} -> {candidate}");
                return Ok(bytes);
            }
        }
    }

    tracing::warn!("resource not found for Mascot Capsule object: {resource_name}; tried={tried:?}");
    Ok(Vec::new())
}

async fn try_load_resource(
    jvm: &Jvm,
    loader: &ClassInstanceRef<ClassLoader>,
    resource_name: &str,
    tried: &mut Vec<RustString>,
) -> Result<Option<Vec<u8>>> {
    tried.push(RustString::from(resource_name));
    let stream: ClassInstanceRef<InputStream> = jvm
        .invoke_virtual(
            loader,
            "getResourceAsStream",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
            (JavaLangString::from_rust_string(jvm, resource_name).await?,),
        )
        .await?;

    if stream.is_null() {
        return Ok(None);
    }

    Ok(Some(JavaIoInputStream::read_until_end(jvm, &stream).await?))
}

fn has_resource_extension(resource_name: &str) -> bool {
    resource_name
        .rsplit('/')
        .next()
        .unwrap_or(resource_name)
        .rsplit('\\')
        .next()
        .unwrap_or(resource_name)
        .contains('.')
}

pub(super) async fn load_runtime_figure(jvm: &Jvm, figure: &ClassInstanceRef<Figure>) -> Result<RuntimeFigure> {
    let key = instance_key(figure);
    let cached = if key == 0 {
        None
    } else {
        V3_FIGURE_CACHE
            .lock()
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| entry.figure.clone())
    };

    let cached = if let Some(cached) = cached {
        V3_FIGURE_CACHE_HITS.fetch_add(1, Ordering::Relaxed);
        cached
    } else {
        V3_FIGURE_CACHE_MISSES.fetch_add(1, Ordering::Relaxed);
        let cached = CachedRuntimeFigure {
            vertex_count: jvm.get_field::<i32>(figure, "vertexCount", "I").await?.max(0) as usize,
            vertices: Arc::new(load_short_field(jvm, figure, "vertices").await?),
            poly_c3: Arc::new(load_short_field(jvm, figure, "polyC3").await?),
            poly_c4: Arc::new(load_short_field(jvm, figure, "polyC4").await?),
            poly_t3: Arc::new(load_short_field(jvm, figure, "polyT3").await?),
            poly_t4: Arc::new(load_short_field(jvm, figure, "polyT4").await?),
            colors: Arc::new(load_int_field(jvm, figure, "colors").await?),
            patterns: Arc::new(load_int_field(jvm, figure, "patterns").await?),
            pattern_count: jvm.get_field::<i32>(figure, "patternCount", "I").await?.max(1) as usize,
            pattern_slots: jvm.get_field::<i32>(figure, "patternSlots", "I").await?.max(1) as usize,
            bones: Arc::new(load_int_field(jvm, figure, "bones").await?),
        };
        if key != 0 {
            let mut cache = V3_FIGURE_CACHE.lock();
            if cache.len() >= V3_FIGURE_CACHE_LIMIT {
                cache.remove(0);
            }
            cache.push(FigureCacheEntry { key, figure: cached.clone() });
        }
        cached
    };

    let posture_bones = load_int_field(jvm, figure, "postureBones").await?;
    let selected_pattern = jvm.get_field(figure, "selectedPattern", "I").await?;
    let texture_index = jvm.get_field(figure, "textureIndex", "I").await?;
    Ok(RuntimeFigure {
        vertex_count: cached.vertex_count,
        vertices: cached.vertices,
        poly_c3: cached.poly_c3,
        poly_c4: cached.poly_c4,
        poly_t3: cached.poly_t3,
        poly_t4: cached.poly_t4,
        colors: cached.colors,
        patterns: cached.patterns,
        pattern_count: cached.pattern_count,
        pattern_slots: cached.pattern_slots,
        bones: cached.bones,
        posture_bones,
        selected_pattern,
        texture_index,
    })
}

async fn load_short_field<T>(jvm: &Jvm, object: &ClassInstanceRef<T>, field: &str) -> Result<Vec<i16>> {
    let array: ClassInstanceRef<Array<i16>> = jvm.get_field(object, field, "[S").await?;
    if array.is_null() {
        return Ok(Vec::new());
    }
    let length = jvm.array_length(&array).await?;
    jvm.load_array(&array, 0, length).await
}

pub(super) async fn load_int_field<T>(jvm: &Jvm, object: &ClassInstanceRef<T>, field: &str) -> Result<Vec<i32>> {
    let array: ClassInstanceRef<Array<i32>> = jvm.get_field(object, field, "[I").await?;
    if array.is_null() {
        return Ok(Vec::new());
    }
    let length = jvm.array_length(&array).await?;
    jvm.load_array(&array, 0, length).await
}

pub(super) async fn load_figure_textures(jvm: &Jvm, figure: &ClassInstanceRef<Figure>) -> Result<Vec<Option<NativeTexture>>> {
    let texture_index: i32 = jvm.get_field(figure, "textureIndex", "I").await?;
    if texture_index >= 0 {
        let textures: ClassInstanceRef<Array<Texture>> = jvm.get_field(figure, "textures", "[Lcom/mascotcapsule/micro3d/v3/Texture;").await?;
        if !textures.is_null() {
            let length = jvm.array_length(&textures).await?;
            if (texture_index as usize) < length {
                let refs: Vec<ClassInstanceRef<Texture>> = jvm.load_array(&textures, texture_index as usize, 1).await?;
                let texture = refs.into_iter().next().unwrap_or_else(|| ClassInstanceRef::<Texture>::new(None));
                return Ok(vec![load_native_texture(jvm, &texture).await?]);
            }
        }

        let texture: ClassInstanceRef<Texture> = jvm.get_field(figure, "texture", "Lcom/mascotcapsule/micro3d/v3/Texture;").await?;
        return Ok(vec![load_native_texture(jvm, &texture).await?]);
    }

    let textures: ClassInstanceRef<Array<Texture>> = jvm.get_field(figure, "textures", "[Lcom/mascotcapsule/micro3d/v3/Texture;").await?;
    if textures.is_null() {
        return Ok(Vec::new());
    }

    let length = jvm.array_length(&textures).await?;
    let refs: Vec<ClassInstanceRef<Texture>> = jvm.load_array(&textures, 0, length).await?;
    let mut result = Vec::with_capacity(refs.len());
    for texture in refs {
        result.push(load_native_texture(jvm, &texture).await?);
    }
    Ok(result)
}

pub(super) async fn load_native_texture(jvm: &Jvm, texture: &ClassInstanceRef<Texture>) -> Result<Option<NativeTexture>> {
    if texture.is_null() {
        return Ok(None);
    }
    let width: i32 = jvm.get_field(texture, "width", "I").await?;
    let height: i32 = jvm.get_field(texture, "height", "I").await?;
    let color_key: i32 = jvm.get_field(texture, "colorKey", "I").await?;
    if width <= 0 || height <= 0 {
        return Ok(None);
    }

    let key = instance_key(texture);
    if key != 0 {
        let cache = V3_TEXTURE_CACHE.lock();
        if let Some(entry) = cache
            .iter()
            .find(|entry| entry.key == key && entry.width == width && entry.height == height && entry.color_key == color_key)
        {
            V3_TEXTURE_CACHE_HITS.fetch_add(1, Ordering::Relaxed);
            return Ok(Some(entry.texture.clone()));
        }
    }

    V3_TEXTURE_CACHE_MISSES.fetch_add(1, Ordering::Relaxed);
    let pixels: ClassInstanceRef<Array<i32>> = jvm.get_field(texture, "pixels", "[I").await?;
    let indices: ClassInstanceRef<Array<i8>> = jvm.get_field(texture, "indices", "[B").await?;
    let palette: ClassInstanceRef<Array<i32>> = jvm.get_field(texture, "palette", "[I").await?;
    if pixels.is_null() {
        return Ok(None);
    }
    let expected = (width * height) as usize;
    let count = jvm.array_length(&pixels).await?.min(expected);
    let mut argb = jvm.load_array(&pixels, 0, count).await?;
    if argb.len() < expected {
        argb.resize(expected, 0xff00_0000u32 as i32);
    }
    let indexed_pixels = if indices.is_null() {
        Vec::new()
    } else {
        let count = jvm.array_length(&indices).await?.min(expected);
        jvm.load_array(&indices, 0, count)
            .await?
            .into_iter()
            .map(|value: i8| value as u8)
            .collect()
    };
    let palette = if palette.is_null() {
        Vec::new()
    } else {
        let length = jvm.array_length(&palette).await?;
        jvm.load_array(&palette, 0, length).await?
    };
    let native = NativeTexture {
        width,
        height,
        pixels: Arc::new(argb),
        indices: Arc::new(indexed_pixels),
        palette: Arc::new(palette),
        color_key,
    };
    if key != 0 {
        push_texture_cache_entry(TextureCacheEntry {
            key,
            width,
            height,
            color_key,
            texture: native.clone(),
        });
    }
    Ok(Some(native))
}

pub(super) async fn put_affine_matrix(jvm: &Jvm, this: &mut ClassInstanceRef<AffineTrans>, matrix: [i32; 12]) -> Result<()> {
    for (name, value) in [
        ("m00", matrix[0]),
        ("m01", matrix[1]),
        ("m02", matrix[2]),
        ("m03", matrix[3]),
        ("m10", matrix[4]),
        ("m11", matrix[5]),
        ("m12", matrix[6]),
        ("m13", matrix[7]),
        ("m20", matrix[8]),
        ("m21", matrix[9]),
        ("m22", matrix[10]),
        ("m23", matrix[11]),
    ] {
        jvm.put_field(this, name, "I", value).await?;
    }
    Ok(())
}

pub(super) async fn get_affine_matrix(jvm: &Jvm, this: &ClassInstanceRef<AffineTrans>) -> Result<[i32; 12]> {
    Ok([
        jvm.get_field(this, "m00", "I").await?,
        jvm.get_field(this, "m01", "I").await?,
        jvm.get_field(this, "m02", "I").await?,
        jvm.get_field(this, "m03", "I").await?,
        jvm.get_field(this, "m10", "I").await?,
        jvm.get_field(this, "m11", "I").await?,
        jvm.get_field(this, "m12", "I").await?,
        jvm.get_field(this, "m13", "I").await?,
        jvm.get_field(this, "m20", "I").await?,
        jvm.get_field(this, "m21", "I").await?,
        jvm.get_field(this, "m22", "I").await?,
        jvm.get_field(this, "m23", "I").await?,
    ])
}

pub(super) async fn get_vector(jvm: &Jvm, value: &ClassInstanceRef<Vector3D>) -> Result<(i32, i32, i32)> {
    Ok((
        jvm.get_field(value, "x", "I").await?,
        jvm.get_field(value, "y", "I").await?,
        jvm.get_field(value, "z", "I").await?,
    ))
}
