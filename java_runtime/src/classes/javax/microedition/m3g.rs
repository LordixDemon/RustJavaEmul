mod math;
mod raw_arrays;
mod render;

use alloc::{
    boxed::Box,
    format,
    string::{String as RustString, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};
use core::{
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU64, Ordering},
};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{
    Array, ClassInstance, ClassInstanceRef, JavaError, Jvm, Result,
    runtime::{JavaIoInputStream, JavaLangString},
};
use parking_lot::Mutex;

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::{
            io::InputStream,
            lang::{Object, String as JavaString},
            util::Hashtable,
        },
        javax::microedition::lcdui::{Graphics, Image},
    },
};

use self::{
    math::{
        expand_triangle_strips, identity_matrix, invert_affine_matrix, invert_matrix, matrix_to_array, multiply_matrix, normalize3,
        parallel_projection_matrix, perspective_projection_matrix, quaternion_matrix, ray_triangle_intersection, rotation_matrix,
        rotation_matrix_to_axis_angle, scale_matrix, transform_point, transform_vec4, translation_matrix, transpose_matrix,
    },
    raw_arrays::{raw_f32_array, raw_i16_array, raw_i32_array, raw_u8_array, store_raw_f32_array, store_raw_i32_array},
    render::{
        M3gAppearanceState, M3gBackgroundFrame, M3gMeshRenderStats, M3gRenderStats, M3gSceneLighting, M3gSubmeshDraw, M3gTexture,
        appearance_sort_key, background_coord, camera_vertex, clamp_color, clip_triangle_near, color_channel_f32, decode_m3g_image_pixels,
        ensure_opaque, material_base_color, project_camera_vertex, rasterize_triangle, source_over, texture_transform_is_identity,
    },
};

type Object3DArray = Array<ClassInstanceRef<Object3D>>;

const M3G_IDENTIFIER: &[u8; 12] = b"\xABJSR184\xBB\r\n\x1A\n";
const PNG_IDENTIFIER: &[u8; 8] = b"\x89PNG\r\n\x1A\n";
const JPEG_IDENTIFIER: &[u8; 2] = b"\xFF\xD8";
static M3G_RENDER_FRAME: AtomicU64 = AtomicU64::new(1);
static M3G_RENDER_LAST_END_MS: AtomicU64 = AtomicU64::new(0);
static M3G_RENDER_LAST_DIAG_MS: AtomicU64 = AtomicU64::new(0);
static M3G_RENDER_DIAGNOSTICS: Mutex<Vec<RustString>> = Mutex::new(Vec::new());
static M3G_VERTEX_VALUES_CACHE: Mutex<Vec<VertexValuesCacheEntry>> = Mutex::new(Vec::new());
static M3G_VERTEX_VEC3_CACHE: Mutex<Vec<VertexVec3CacheEntry>> = Mutex::new(Vec::new());
static M3G_VERTEX_VEC2_CACHE: Mutex<Vec<VertexVec2CacheEntry>> = Mutex::new(Vec::new());
static M3G_VERTEX_COLOR_CACHE: Mutex<Vec<VertexColorCacheEntry>> = Mutex::new(Vec::new());
static M3G_TRIANGLE_INDEX_CACHE: Mutex<Vec<TriangleIndexCacheEntry>> = Mutex::new(Vec::new());
static M3G_IMAGE_PIXEL_CACHE: Mutex<Vec<ImagePixelCacheEntry>> = Mutex::new(Vec::new());

const FNV_OFFSET_BASIS_64: u64 = 0xcbf29ce484222325;
const FNV_PRIME_64: u64 = 0x100000001b3;
const M3G_CACHE_LIMIT: usize = 2048;
const M3G_RENDER_DIAGNOSTIC_LIMIT: usize = 16;

#[derive(Default)]
struct M3gIdentityHasher(u64);

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

struct VertexValuesCacheEntry {
    key: u64,
    version: i32,
    component_count: usize,
    vertex_count: usize,
    values: Arc<Vec<f32>>,
}

struct VertexVec3CacheEntry {
    key: u64,
    version: i32,
    scale_bits: u32,
    bias_bits: [u32; 3],
    values: Arc<Vec<[f32; 3]>>,
}

struct VertexVec2CacheEntry {
    key: u64,
    version: i32,
    scale_bits: u32,
    bias_bits: [u32; 2],
    values: Arc<Vec<[f32; 2]>>,
}

struct VertexColorCacheEntry {
    key: u64,
    version: i32,
    default_color: i32,
    values: Arc<Vec<i32>>,
}

struct TriangleIndexCacheEntry {
    key: u64,
    values: Arc<Vec<[usize; 3]>>,
}

struct ImagePixelCacheEntry {
    key: u64,
    width: i32,
    height: i32,
    values: Arc<Vec<i32>>,
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

struct M3gPickHit {
    node: ClassInstanceRef<Node>,
    distance: f32,
    submesh_index: i32,
    texture_s: f32,
    texture_t: f32,
    normal: [f32; 3],
}

#[derive(Default)]
struct M3gDuplicateMap {
    entries: Vec<(ClassInstanceRef<Object3D>, ClassInstanceRef<Object3D>)>,
    copied: usize,
    shared: usize,
}

impl M3gDuplicateMap {
    fn get(&self, source: &ClassInstanceRef<Object3D>) -> Option<ClassInstanceRef<Object3D>> {
        self.entries
            .iter()
            .find(|(candidate, _)| same_instance(candidate, source))
            .map(|(_, duplicate)| duplicate.clone())
    }

    fn insert(&mut self, source: &ClassInstanceRef<Object3D>, duplicate: &ClassInstanceRef<Object3D>) {
        self.entries.push((source.clone(), duplicate.clone()));
        self.copied += 1;
    }

    fn note_shared(&mut self) {
        self.shared += 1;
    }
}

enum M3gSectionData<'a> {
    Borrowed(&'a [u8]),
    Owned(Vec<u8>),
}

impl M3gSectionData<'_> {
    fn as_slice(&self) -> &[u8] {
        match self {
            Self::Borrowed(data) => data,
            Self::Owned(data) => data,
        }
    }
}

fn decode_m3g_section_data(compression: u8, data: &[u8], inflated_len: usize) -> core::result::Result<M3gSectionData<'_>, &'static str> {
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

pub struct Loader;
pub struct AnimationController;
pub struct AnimationTrack;
pub struct KeyframeSequence;
pub struct Object3D;
pub struct Transform;
pub struct Transformable;
pub struct Node;
pub struct Group;
pub struct World;
pub struct Camera;
pub struct Background;
pub struct Appearance;
pub struct CompositingMode;
pub struct Fog;
pub struct PolygonMode;
pub struct Image2D;
pub struct Texture2D;
pub struct Sprite3D;
pub struct Mesh;
pub struct IndexBuffer;
pub struct TriangleStripArray;
pub struct VertexArray;
pub struct VertexBuffer;
pub struct Light;
pub struct Material;
pub struct Graphics3D;
pub struct RayIntersection;

fn cast_ref<T, U>(value: &ClassInstanceRef<T>) -> ClassInstanceRef<U> {
    ClassInstanceRef::new(value.instance.clone())
}

fn null_ref<T>() -> ClassInstanceRef<T> {
    ClassInstanceRef::new(None)
}

fn instance_key<T>(value: &ClassInstanceRef<T>) -> u64 {
    let Some(instance) = &value.instance else {
        return 0;
    };
    let mut hasher = M3gIdentityHasher::default();
    instance.hash(&mut hasher);
    hasher.finish()
}

fn push_cache_entry<T>(cache: &mut Vec<T>, entry: T) {
    if cache.len() >= M3G_CACHE_LIMIT {
        cache.remove(0);
    }
    cache.push(entry);
}

fn publish_render_diagnostic(now_ms: u64, total_ms: u64, line: RustString) {
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

fn object3d_proto(
    name: &'static str,
    parent_class: Option<&'static str>,
    methods: Vec<JavaMethodProto<dyn crate::runtime::Runtime>>,
) -> RuntimeClassProto {
    RuntimeClassProto {
        name,
        parent_class,
        interfaces: vec![],
        methods,
        fields: vec![],
        access_flags: Default::default(),
    }
}

impl Loader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Loader",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "load",
                    "(Ljava/lang/String;)[Ljavax/microedition/m3g/Object3D;",
                    Self::load_from_string,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "load",
                    "([BI)[Ljavax/microedition/m3g/Object3D;",
                    Self::load_from_bytes,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn load_from_string(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<JavaString>,
    ) -> Result<ClassInstanceRef<Object3DArray>> {
        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Loader.load(null)").await);
        }

        let resource_name = JavaLangString::to_rust_string(jvm, &name).await?;
        tracing::info!(target: "rustjava_m3g", "m3g.loader.load resource={resource_name:?}");
        let objects = Self::load_resource_objects(jvm, context, &resource_name, &mut Vec::new()).await?;
        Self::object_array(jvm, objects).await
    }

    async fn load_from_bytes(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        data: ClassInstanceRef<Array<i8>>,
        offset: i32,
    ) -> Result<ClassInstanceRef<Object3DArray>> {
        if data.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Loader.load(null, offset)").await);
        }
        if offset < 0 {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "negative offset").await);
        }

        let length = jvm.array_length(&data).await?;
        if offset as usize >= length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "offset outside data").await);
        }

        let bytes_i8: Vec<i8> = jvm.load_array(&data, offset as usize, length - offset as usize).await?;
        let bytes: Vec<u8> = bytes_i8.into_iter().map(|b| b as u8).collect();
        let objects = Self::load_bytes_objects(jvm, context, "ByteArray", &bytes, &mut Vec::new()).await?;
        Self::object_array(jvm, objects).await
    }

    #[async_recursion::async_recursion]
    async fn load_resource_objects(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        resource_name: &str,
        history: &mut Vec<RustString>,
    ) -> Result<Vec<ClassInstanceRef<Object3D>>> {
        let normalized = resource_name.trim_start_matches('/').to_string();
        if history.iter().any(|name| name == &normalized) {
            return Err(jvm.exception("java/io/IOException", "M3G reference loop").await);
        }
        history.push(normalized.clone());
        let bytes = Self::read_resource(jvm, &normalized).await?;
        let result = Self::load_bytes_objects(jvm, context, &normalized, &bytes, history).await;
        history.pop();
        result
    }

    async fn load_bytes_objects(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        resource_name: &str,
        bytes: &[u8],
        history: &mut Vec<RustString>,
    ) -> Result<Vec<ClassInstanceRef<Object3D>>> {
        if bytes.starts_with(M3G_IDENTIFIER) {
            return M3gFileLoader::new(jvm, context, resource_name, history).parse(bytes).await;
        }

        if bytes.starts_with(PNG_IDENTIFIER) || bytes.starts_with(JPEG_IDENTIFIER) {
            let image = if resource_name == "ByteArray" {
                let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
                let signed: Vec<i8> = bytes.iter().map(|byte| *byte as i8).collect();
                jvm.store_array(&mut byte_array, 0, signed).await?;
                let image: ClassInstanceRef<Image> = jvm
                    .invoke_static(
                        "javax/microedition/lcdui/Image",
                        "createImage",
                        "([BII)Ljavax/microedition/lcdui/Image;",
                        (byte_array, 0, bytes.len() as i32),
                    )
                    .await?;
                image
            } else {
                let name = JavaLangString::from_rust_string(jvm, resource_name).await?;
                jvm.invoke_static(
                    "javax/microedition/lcdui/Image",
                    "createImage",
                    "(Ljava/lang/String;)Ljavax/microedition/lcdui/Image;",
                    (name,),
                )
                .await?
            };

            let image2d: ClassInstanceRef<Image2D> = jvm
                .new_class(
                    "javax/microedition/m3g/Image2D",
                    "(ILjava/lang/Object;)V",
                    (Image2D::RGBA, cast_ref::<Image, Object>(&image)),
                )
                .await?
                .into();
            return Ok(vec![cast_ref(&image2d)]);
        }

        Err(jvm
            .exception("java/io/IOException", &format!("M3G resource not recognized: {resource_name}"))
            .await)
    }

    async fn read_resource(jvm: &Jvm, resource_name: &str) -> Result<Vec<u8>> {
        let class_loader = jvm
            .invoke_static("java/lang/ClassLoader", "getSystemClassLoader", "()Ljava/lang/ClassLoader;", ())
            .await?;
        let stream: ClassInstanceRef<InputStream> = jvm
            .invoke_virtual(
                &class_loader,
                "getResourceAsStream",
                "(Ljava/lang/String;)Ljava/io/InputStream;",
                (JavaLangString::from_rust_string(jvm, resource_name).await?,),
            )
            .await?;

        if stream.is_null() {
            return Err(jvm
                .exception("java/io/IOException", &format!("resource not found: {resource_name}"))
                .await);
        }

        JavaIoInputStream::read_until_end(jvm, &stream).await
    }

    async fn object_array(jvm: &Jvm, objects: Vec<ClassInstanceRef<Object3D>>) -> Result<ClassInstanceRef<Object3DArray>> {
        let mut array = jvm.instantiate_array("Ljavax/microedition/m3g/Object3D;", objects.len()).await?;
        jvm.store_array(&mut array, 0, objects).await?;
        Ok(array.into())
    }
}

impl AnimationController {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/AnimationController",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getActiveIntervalEnd", "()I", Self::get_active_interval_end, Default::default()),
                JavaMethodProto::new("getActiveIntervalStart", "()I", Self::get_active_interval_start, Default::default()),
                JavaMethodProto::new("getPosition", "(I)F", Self::get_position, Default::default()),
                JavaMethodProto::new("getRefWorldTime", "()I", Self::get_ref_world_time, Default::default()),
                JavaMethodProto::new("getSpeed", "()F", Self::get_speed, Default::default()),
                JavaMethodProto::new("getWeight", "()F", Self::get_weight, Default::default()),
                JavaMethodProto::new("setActiveInterval", "(II)V", Self::set_active_interval, Default::default()),
                JavaMethodProto::new("setPosition", "(FI)V", Self::set_position, Default::default()),
                JavaMethodProto::new("setSpeed", "(FI)V", Self::set_speed, Default::default()),
                JavaMethodProto::new("setWeight", "(F)V", Self::set_weight, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("speed", "F", Default::default()),
                JavaFieldProto::new("weight", "F", Default::default()),
                JavaFieldProto::new("activeIntervalStart", "I", Default::default()),
                JavaFieldProto::new("activeIntervalEnd", "I", Default::default()),
                JavaFieldProto::new("refSequenceTime", "F", Default::default()),
                JavaFieldProto::new("refWorldTime", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "speed", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "weight", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "activeIntervalStart", "I", 0).await?;
        jvm.put_field(&mut this, "activeIntervalEnd", "I", i32::MAX).await?;
        jvm.put_field(&mut this, "refSequenceTime", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "refWorldTime", "I", 0).await
    }

    async fn get_active_interval_end(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "activeIntervalEnd", "I").await
    }

    async fn get_active_interval_start(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "activeIntervalStart", "I").await
    }

    async fn get_position(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, world_time: i32) -> Result<f32> {
        Self::sequence_time(jvm, &this, world_time).await
    }

    async fn get_ref_world_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "refWorldTime", "I").await
    }

    async fn get_speed(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "speed", "F").await
    }

    async fn get_weight(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "weight", "F").await
    }

    async fn set_active_interval(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, start: i32, end: i32) -> Result<()> {
        if end < start {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "AnimationController active interval")
                .await);
        }
        jvm.put_field(&mut this, "activeIntervalStart", "I", start).await?;
        jvm.put_field(&mut this, "activeIntervalEnd", "I", end).await
    }

    async fn set_position(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sequence_time: f32, world_time: i32) -> Result<()> {
        jvm.put_field(&mut this, "refSequenceTime", "F", sequence_time).await?;
        jvm.put_field(&mut this, "refWorldTime", "I", world_time).await
    }

    async fn set_speed(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, speed: f32, world_time: i32) -> Result<()> {
        let sequence_time = Self::sequence_time(jvm, &this, world_time).await.unwrap_or(0.0);
        jvm.put_field(&mut this, "speed", "F", speed).await?;
        jvm.put_field(&mut this, "refSequenceTime", "F", sequence_time).await?;
        jvm.put_field(&mut this, "refWorldTime", "I", world_time).await
    }

    async fn set_weight(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, weight: f32) -> Result<()> {
        if weight < 0.0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "AnimationController weight").await);
        }
        jvm.put_field(&mut this, "weight", "F", weight).await
    }

    async fn is_active(jvm: &Jvm, this: &ClassInstanceRef<Self>, world_time: i32) -> Result<bool> {
        if this.is_null() {
            return Ok(false);
        }
        let start: i32 = jvm.get_field(this, "activeIntervalStart", "I").await.unwrap_or(0);
        let end: i32 = jvm.get_field(this, "activeIntervalEnd", "I").await.unwrap_or(i32::MAX);
        let weight: f32 = jvm.get_field(this, "weight", "F").await.unwrap_or(1.0);
        Ok(world_time >= start && world_time < end && weight > 0.0)
    }

    async fn sequence_time(jvm: &Jvm, this: &ClassInstanceRef<Self>, world_time: i32) -> Result<f32> {
        if this.is_null() {
            return Ok(world_time as f32);
        }
        let ref_sequence_time: f32 = jvm.get_field(this, "refSequenceTime", "F").await.unwrap_or(0.0);
        let ref_world_time: i32 = jvm.get_field(this, "refWorldTime", "I").await.unwrap_or(0);
        let speed: f32 = jvm.get_field(this, "speed", "F").await.unwrap_or(1.0);
        Ok(ref_sequence_time + (world_time - ref_world_time) as f32 * speed)
    }
}

impl AnimationTrack {
    const ALPHA: i32 = 256;
    const AMBIENT_COLOR: i32 = 257;
    const COLOR: i32 = 258;
    const CROP: i32 = 259;
    const DENSITY: i32 = 260;
    const DIFFUSE_COLOR: i32 = 261;
    const EMISSIVE_COLOR: i32 = 262;
    const FAR_DISTANCE: i32 = 263;
    const FIELD_OF_VIEW: i32 = 264;
    const INTENSITY: i32 = 265;
    const MORPH_WEIGHTS: i32 = 266;
    const NEAR_DISTANCE: i32 = 267;
    const ORIENTATION: i32 = 268;
    const PICKABILITY: i32 = 269;
    const SCALE: i32 = 270;
    const SHININESS: i32 = 271;
    const SPECULAR_COLOR: i32 = 272;
    const SPOT_ANGLE: i32 = 273;
    const SPOT_EXPONENT: i32 = 274;
    const TRANSLATION: i32 = 275;
    const VISIBILITY: i32 = 276;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/AnimationTrack",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/m3g/KeyframeSequence;I)V",
                    Self::init_with_sequence,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getController",
                    "()Ljavax/microedition/m3g/AnimationController;",
                    Self::get_controller,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getKeyframeSequence",
                    "()Ljavax/microedition/m3g/KeyframeSequence;",
                    Self::get_keyframe_sequence,
                    Default::default(),
                ),
                JavaMethodProto::new("getTargetProperty", "()I", Self::get_target_property, Default::default()),
                JavaMethodProto::new(
                    "setController",
                    "(Ljavax/microedition/m3g/AnimationController;)V",
                    Self::set_controller,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("ALPHA", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("AMBIENT_COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("CROP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DENSITY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DIFFUSE_COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("EMISSIVE_COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FAR_DISTANCE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FIELD_OF_VIEW", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INTENSITY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MORPH_WEIGHTS", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("NEAR_DISTANCE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ORIENTATION", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PICKABILITY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SCALE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SHININESS", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SPECULAR_COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SPOT_ANGLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SPOT_EXPONENT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRANSLATION", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("VISIBILITY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("sequence", "Ljavax/microedition/m3g/KeyframeSequence;", Default::default()),
                JavaFieldProto::new("controller", "Ljavax/microedition/m3g/AnimationController;", Default::default()),
                JavaFieldProto::new("targetProperty", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/m3g/AnimationTrack";
        for (name, value) in [
            ("ALPHA", Self::ALPHA),
            ("AMBIENT_COLOR", Self::AMBIENT_COLOR),
            ("COLOR", Self::COLOR),
            ("CROP", Self::CROP),
            ("DENSITY", Self::DENSITY),
            ("DIFFUSE_COLOR", Self::DIFFUSE_COLOR),
            ("EMISSIVE_COLOR", Self::EMISSIVE_COLOR),
            ("FAR_DISTANCE", Self::FAR_DISTANCE),
            ("FIELD_OF_VIEW", Self::FIELD_OF_VIEW),
            ("INTENSITY", Self::INTENSITY),
            ("MORPH_WEIGHTS", Self::MORPH_WEIGHTS),
            ("NEAR_DISTANCE", Self::NEAR_DISTANCE),
            ("ORIENTATION", Self::ORIENTATION),
            ("PICKABILITY", Self::PICKABILITY),
            ("SCALE", Self::SCALE),
            ("SHININESS", Self::SHININESS),
            ("SPECULAR_COLOR", Self::SPECULAR_COLOR),
            ("SPOT_ANGLE", Self::SPOT_ANGLE),
            ("SPOT_EXPONENT", Self::SPOT_EXPONENT),
            ("TRANSLATION", Self::TRANSLATION),
            ("VISIBILITY", Self::VISIBILITY),
        ] {
            jvm.put_static_field(class, name, "I", value).await?;
        }
        Ok(())
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "sequence",
            "Ljavax/microedition/m3g/KeyframeSequence;",
            null_ref::<KeyframeSequence>(),
        )
        .await?;
        jvm.put_field(
            &mut this,
            "controller",
            "Ljavax/microedition/m3g/AnimationController;",
            null_ref::<AnimationController>(),
        )
        .await?;
        jvm.put_field(&mut this, "targetProperty", "I", 0).await
    }

    async fn init_with_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<KeyframeSequence>,
        property: i32,
    ) -> Result<()> {
        if sequence.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AnimationTrack sequence").await);
        }
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;", sequence)
            .await?;
        jvm.put_field(
            &mut this,
            "controller",
            "Ljavax/microedition/m3g/AnimationController;",
            null_ref::<AnimationController>(),
        )
        .await?;
        jvm.put_field(&mut this, "targetProperty", "I", property).await
    }

    async fn get_controller(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<AnimationController>> {
        jvm.get_field(&this, "controller", "Ljavax/microedition/m3g/AnimationController;").await
    }

    async fn get_keyframe_sequence(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<KeyframeSequence>> {
        jvm.get_field(&this, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;").await
    }

    async fn get_target_property(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "targetProperty", "I").await
    }

    async fn set_controller(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        controller: ClassInstanceRef<AnimationController>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "controller", "Ljavax/microedition/m3g/AnimationController;", controller)
            .await
    }
}

impl KeyframeSequence {
    const LINEAR: i32 = 176;
    const SLERP: i32 = 177;
    const SPLINE: i32 = 178;
    const SQUAD: i32 = 179;
    const STEP: i32 = 180;
    const CONSTANT: i32 = 192;
    const LOOP: i32 = 193;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/KeyframeSequence",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(III)V", Self::init_with_counts, Default::default()),
                JavaMethodProto::new("getComponentCount", "()I", Self::get_component_count, Default::default()),
                JavaMethodProto::new("getDuration", "()I", Self::get_duration, Default::default()),
                JavaMethodProto::new("getInterpolationType", "()I", Self::get_interpolation_type, Default::default()),
                JavaMethodProto::new("getKeyframe", "(I[F)I", Self::get_keyframe, Default::default()),
                JavaMethodProto::new("getKeyframeCount", "()I", Self::get_keyframe_count, Default::default()),
                JavaMethodProto::new("getRepeatMode", "()I", Self::get_repeat_mode, Default::default()),
                JavaMethodProto::new("getValidRangeFirst", "()I", Self::get_valid_range_first, Default::default()),
                JavaMethodProto::new("getValidRangeLast", "()I", Self::get_valid_range_last, Default::default()),
                JavaMethodProto::new("setDuration", "(I)V", Self::set_duration, Default::default()),
                JavaMethodProto::new("setKeyframe", "(II[F)V", Self::set_keyframe, Default::default()),
                JavaMethodProto::new("setRepeatMode", "(I)V", Self::set_repeat_mode, Default::default()),
                JavaMethodProto::new("setValidRange", "(II)V", Self::set_valid_range, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("LINEAR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SLERP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SPLINE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SQUAD", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STEP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("CONSTANT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LOOP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("interpolationType", "I", Default::default()),
                JavaFieldProto::new("repeatMode", "I", Default::default()),
                JavaFieldProto::new("duration", "I", Default::default()),
                JavaFieldProto::new("validRangeFirst", "I", Default::default()),
                JavaFieldProto::new("validRangeLast", "I", Default::default()),
                JavaFieldProto::new("componentCount", "I", Default::default()),
                JavaFieldProto::new("keyframeCount", "I", Default::default()),
                JavaFieldProto::new("keyframeTimes", "[I", Default::default()),
                JavaFieldProto::new("keyframeValues", "[F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/m3g/KeyframeSequence";
        for (name, value) in [
            ("LINEAR", Self::LINEAR),
            ("SLERP", Self::SLERP),
            ("SPLINE", Self::SPLINE),
            ("SQUAD", Self::SQUAD),
            ("STEP", Self::STEP),
            ("CONSTANT", Self::CONSTANT),
            ("LOOP", Self::LOOP),
        ] {
            jvm.put_static_field(class, name, "I", value).await?;
        }
        Ok(())
    }

    async fn init_with_counts(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        keyframe_count: i32,
        component_count: i32,
        interpolation: i32,
    ) -> Result<()> {
        if keyframe_count <= 0 || component_count <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "KeyframeSequence size").await);
        }
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        let times = vec![0; keyframe_count as usize];
        let values = vec![0.0; (keyframe_count * component_count) as usize];
        Self::put_data(
            jvm,
            &mut this,
            interpolation,
            Self::CONSTANT,
            0,
            0,
            keyframe_count - 1,
            component_count,
            times,
            values,
        )
        .await
    }

    async fn get_component_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "componentCount", "I").await
    }

    async fn get_duration(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "duration", "I").await
    }

    async fn get_interpolation_type(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "interpolationType", "I").await
    }

    async fn get_keyframe(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        mut values: ClassInstanceRef<Array<f32>>,
    ) -> Result<i32> {
        let keyframe_count: i32 = jvm.get_field(&this, "keyframeCount", "I").await?;
        let component_count: i32 = jvm.get_field(&this, "componentCount", "I").await?;
        if index < 0 || index >= keyframe_count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "KeyframeSequence index").await);
        }
        if values.is_null() || jvm.array_length(&values).await? < component_count as usize {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "KeyframeSequence values").await);
        }
        let times = Self::times(jvm, &this).await?;
        let all_values = Self::values(jvm, &this).await?;
        let start = index as usize * component_count as usize;
        store_raw_f32_array(jvm, &mut values, &all_values[start..start + component_count as usize]).await?;
        Ok(times[index as usize])
    }

    async fn get_keyframe_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "keyframeCount", "I").await
    }

    async fn get_repeat_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "repeatMode", "I").await
    }

    async fn get_valid_range_first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "validRangeFirst", "I").await
    }

    async fn get_valid_range_last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "validRangeLast", "I").await
    }

    async fn set_duration(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, duration: i32) -> Result<()> {
        if duration < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "KeyframeSequence duration").await);
        }
        jvm.put_field(&mut this, "duration", "I", duration).await
    }

    async fn set_keyframe(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        index: i32,
        time: i32,
        values: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        let keyframe_count: i32 = jvm.get_field(&this, "keyframeCount", "I").await?;
        let component_count: i32 = jvm.get_field(&this, "componentCount", "I").await?;
        if index < 0 || index >= keyframe_count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "KeyframeSequence index").await);
        }
        if values.is_null() || jvm.array_length(&values).await? < component_count as usize {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "KeyframeSequence values").await);
        }

        let mut times = Self::times(jvm, &this).await?;
        let mut all_values = Self::values(jvm, &this).await?;
        let new_values = raw_f32_array(jvm, &values, component_count as usize).await?;
        times[index as usize] = time;
        let start = index as usize * component_count as usize;
        all_values[start..start + component_count as usize].copy_from_slice(&new_values);
        Self::put_keyframes(jvm, &mut this, times, all_values).await
    }

    async fn set_repeat_mode(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, repeat_mode: i32) -> Result<()> {
        if repeat_mode != Self::CONSTANT && repeat_mode != Self::LOOP {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "KeyframeSequence repeat mode").await);
        }
        jvm.put_field(&mut this, "repeatMode", "I", repeat_mode).await
    }

    async fn set_valid_range(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, first: i32, last: i32) -> Result<()> {
        let keyframe_count: i32 = jvm.get_field(&this, "keyframeCount", "I").await?;
        if first < 0 || last < first || last >= keyframe_count {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "KeyframeSequence valid range").await);
        }
        jvm.put_field(&mut this, "validRangeFirst", "I", first).await?;
        jvm.put_field(&mut this, "validRangeLast", "I", last).await
    }

    async fn put_data(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        interpolation: i32,
        repeat_mode: i32,
        duration: i32,
        valid_first: i32,
        valid_last: i32,
        component_count: i32,
        times: Vec<i32>,
        values: Vec<f32>,
    ) -> Result<()> {
        jvm.put_field(this, "interpolationType", "I", interpolation).await?;
        jvm.put_field(this, "repeatMode", "I", repeat_mode).await?;
        jvm.put_field(this, "duration", "I", duration).await?;
        jvm.put_field(this, "validRangeFirst", "I", valid_first).await?;
        jvm.put_field(this, "validRangeLast", "I", valid_last).await?;
        jvm.put_field(this, "componentCount", "I", component_count).await?;
        jvm.put_field(this, "keyframeCount", "I", times.len() as i32).await?;
        Self::put_keyframes(jvm, this, times, values).await
    }

    async fn put_keyframes(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, times: Vec<i32>, values: Vec<f32>) -> Result<()> {
        let mut time_array = jvm.instantiate_array("I", times.len()).await?;
        store_raw_i32_array(jvm, &mut time_array, &times).await?;
        let mut value_array = jvm.instantiate_array("F", values.len()).await?;
        store_raw_f32_array(jvm, &mut value_array, &values).await?;
        jvm.put_field(this, "keyframeTimes", "[I", time_array).await?;
        jvm.put_field(this, "keyframeValues", "[F", value_array).await
    }

    async fn times(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<i32>> {
        let keyframe_count: i32 = jvm.get_field(this, "keyframeCount", "I").await?;
        let times: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "keyframeTimes", "[I").await?;
        raw_i32_array(jvm, &times, keyframe_count.max(0) as usize).await
    }

    async fn values(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<f32>> {
        let keyframe_count: i32 = jvm.get_field(this, "keyframeCount", "I").await?;
        let component_count: i32 = jvm.get_field(this, "componentCount", "I").await?;
        let values: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "keyframeValues", "[F").await?;
        raw_f32_array(jvm, &values, (keyframe_count.max(0) * component_count.max(0)) as usize).await
    }

    async fn sample(jvm: &Jvm, this: &ClassInstanceRef<Self>, sequence_time: f32) -> Result<Option<Vec<f32>>> {
        if this.is_null() {
            return Ok(None);
        }
        let keyframe_count: i32 = jvm.get_field(this, "keyframeCount", "I").await.unwrap_or(0);
        let component_count: i32 = jvm.get_field(this, "componentCount", "I").await.unwrap_or(0);
        if keyframe_count <= 0 || component_count <= 0 {
            return Ok(None);
        }
        let duration: i32 = jvm.get_field(this, "duration", "I").await.unwrap_or(0);
        let repeat_mode: i32 = jvm.get_field(this, "repeatMode", "I").await.unwrap_or(Self::CONSTANT);
        let interpolation: i32 = jvm.get_field(this, "interpolationType", "I").await.unwrap_or(Self::LINEAR);
        let times = Self::times(jvm, this).await?;
        let values = Self::values(jvm, this).await?;
        if times.is_empty() || values.len() < component_count as usize {
            return Ok(None);
        }

        let mut time = sequence_time;
        if repeat_mode == Self::LOOP && duration > 0 {
            let duration = duration as f32;
            time %= duration;
            if time < 0.0 {
                time += duration;
            }
        }

        if time <= times[0] as f32 || keyframe_count == 1 {
            return Ok(Some(values[0..component_count as usize].to_vec()));
        }

        let last_index = keyframe_count as usize - 1;
        if time >= times[last_index] as f32 {
            let start = last_index * component_count as usize;
            return Ok(Some(values[start..start + component_count as usize].to_vec()));
        }

        let mut upper = 1usize;
        while upper < times.len() && time > times[upper] as f32 {
            upper += 1;
        }
        let lower = upper.saturating_sub(1);
        let lower_time = times[lower] as f32;
        let upper_time = times[upper] as f32;
        let span = (upper_time - lower_time).max(1.0);
        let t = if interpolation == Self::STEP {
            0.0
        } else {
            ((time - lower_time) / span).clamp(0.0, 1.0)
        };

        let component_count = component_count as usize;
        let lower_start = lower * component_count;
        let upper_start = upper * component_count;
        let mut result = Vec::with_capacity(component_count);
        for component in 0..component_count {
            let a = values[lower_start + component];
            let b = values[upper_start + component];
            result.push(a + (b - a) * t);
        }
        Ok(Some(result))
    }
}

impl Object3D {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Object3D",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "addAnimationTrack",
                    "(Ljavax/microedition/m3g/AnimationTrack;)V",
                    Self::add_animation_track,
                    Default::default(),
                ),
                JavaMethodProto::new("animate", "(I)I", Self::animate, Default::default()),
                JavaMethodProto::new("duplicate", "()Ljavax/microedition/m3g/Object3D;", Self::duplicate, Default::default()),
                JavaMethodProto::new("find", "(I)Ljavax/microedition/m3g/Object3D;", Self::find, Default::default()),
                JavaMethodProto::new(
                    "getAnimationTrack",
                    "(I)Ljavax/microedition/m3g/AnimationTrack;",
                    Self::get_animation_track,
                    Default::default(),
                ),
                JavaMethodProto::new("getAnimationTrackCount", "()I", Self::get_animation_track_count, Default::default()),
                JavaMethodProto::new("getUserID", "()I", Self::get_user_id, Default::default()),
                JavaMethodProto::new(
                    "removeAnimationTrack",
                    "(Ljavax/microedition/m3g/AnimationTrack;)V",
                    Self::remove_animation_track,
                    Default::default(),
                ),
                JavaMethodProto::new("setUserID", "(I)V", Self::set_user_id, Default::default()),
                JavaMethodProto::new("setUserObject", "(Ljava/lang/Object;)V", Self::set_user_object, Default::default()),
                JavaMethodProto::new("getUserObject", "()Ljava/lang/Object;", Self::get_user_object, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("userID", "I", Default::default()),
                JavaFieldProto::new("userObject", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", Default::default()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "userID", "I", 0).await?;
        jvm.put_field(&mut this, "userObject", "Ljava/lang/Object;", null_ref::<Object>()).await?;
        let animation_tracks = jvm.instantiate_array("Ljavax/microedition/m3g/AnimationTrack;", 0).await?;
        jvm.put_field(&mut this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", animation_tracks)
            .await
    }

    async fn duplicate(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        if this.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Object3D.duplicate").await);
        }

        let class_name = this.class_definition().name().to_string();
        let mut map = M3gDuplicateMap::default();
        let duplicate = Self::duplicate_object(jvm, &this, &mut map).await?;
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.Object3D.duplicate class={} copied={} shared={}",
            class_name,
            map.copied,
            map.shared
        );
        Ok(duplicate)
    }

    async fn find(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, user_id: i32) -> Result<ClassInstanceRef<Self>> {
        let mut stack = vec![this];
        let mut visited = Vec::new();
        while let Some(current) = stack.pop() {
            if current.is_null() {
                continue;
            }
            if visited.iter().any(|seen| same_instance(seen, &current)) {
                continue;
            }
            visited.push(current.clone());

            let current_id: i32 = jvm.get_field(&current, "userID", "I").await?;
            if current_id == user_id {
                tracing::debug!(
                    target: "rustjava_m3g",
                    "m3g.Object3D.find userID={} hit class={} visited={}",
                    user_id,
                    current.class_definition().name(),
                    visited.len()
                );
                return Ok(current);
            }

            Self::push_find_references(jvm, &current, &mut stack).await?;
        }

        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.Object3D.find userID={} miss visited={}",
            user_id,
            visited.len()
        );
        Ok(null_ref())
    }

    async fn add_animation_track(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        track: ClassInstanceRef<AnimationTrack>,
    ) -> Result<()> {
        if track.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Object3D.addAnimationTrack").await);
        }
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(&this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await?;
        let count = jvm.array_length(&tracks).await?;
        let mut values: Vec<ClassInstanceRef<AnimationTrack>> = jvm.load_array(&tracks, 0, count).await?;
        if values.iter().any(|candidate| same_instance(candidate, &track)) {
            return Ok(());
        }
        values.push(track);
        Self::store_animation_tracks(jvm, &mut this, values).await
    }

    async fn animate(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, world_time: i32) -> Result<i32> {
        if this.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Object3D.animate").await);
        }

        let mut stack = vec![this];
        let mut visited = Vec::new();
        while let Some(current) = stack.pop() {
            if current.is_null() || visited.iter().any(|seen| same_instance(seen, &current)) {
                continue;
            }
            visited.push(current.clone());
            Self::animate_one(jvm, &current, world_time).await?;
            Self::push_find_references(jvm, &current, &mut stack).await?;
        }

        Ok(0)
    }

    async fn animate_one(jvm: &Jvm, object: &ClassInstanceRef<Self>, world_time: i32) -> Result<()> {
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(object, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await
            .unwrap_or_else(|_| null_ref());
        if tracks.is_null() {
            return Ok(());
        }

        let count = jvm.array_length(&tracks).await?;
        if count == 0 {
            return Ok(());
        }

        let tracks: Vec<ClassInstanceRef<AnimationTrack>> = jvm.load_array(&tracks, 0, count).await?;
        for track in tracks {
            if track.is_null() {
                continue;
            }
            let controller: ClassInstanceRef<AnimationController> = jvm
                .get_field(&track, "controller", "Ljavax/microedition/m3g/AnimationController;")
                .await
                .unwrap_or_else(|_| null_ref());
            if !AnimationController::is_active(jvm, &controller, world_time).await? {
                continue;
            }

            let sequence_time = AnimationController::sequence_time(jvm, &controller, world_time).await?;
            let sequence: ClassInstanceRef<KeyframeSequence> = jvm
                .get_field(&track, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;")
                .await
                .unwrap_or_else(|_| null_ref());
            let Some(values) = KeyframeSequence::sample(jvm, &sequence, sequence_time).await? else {
                continue;
            };
            let property: i32 = jvm.get_field(&track, "targetProperty", "I").await.unwrap_or(0);
            Self::apply_animation_values(jvm, object, property, &values).await?;
        }
        Ok(())
    }

    async fn apply_animation_values(jvm: &Jvm, object: &ClassInstanceRef<Self>, property: i32, values: &[f32]) -> Result<()> {
        match property {
            AnimationTrack::ALPHA if !values.is_empty() => {
                if let Some(instance) = object.instance.as_deref()
                    && jvm.is_instance(instance, "javax/microedition/m3g/Node")
                {
                    let mut node = cast_ref::<Object3D, Node>(object);
                    jvm.put_field(&mut node, "alphaFactor", "F", values[0].clamp(0.0, 1.0)).await?;
                }
            }
            AnimationTrack::ORIENTATION if values.len() >= 4 => {
                if let Some(instance) = object.instance.as_deref()
                    && jvm.is_instance(instance, "javax/microedition/m3g/Transformable")
                {
                    let mut transformable = cast_ref::<Object3D, Transformable>(object);
                    Transformable::set_orientation_fields(jvm, &mut transformable, values[0], values[1], values[2], values[3]).await?;
                }
            }
            AnimationTrack::PICKABILITY if !values.is_empty() => {
                if let Some(instance) = object.instance.as_deref()
                    && jvm.is_instance(instance, "javax/microedition/m3g/Node")
                {
                    let mut node = cast_ref::<Object3D, Node>(object);
                    jvm.put_field(&mut node, "pickingEnabled", "Z", values[0] >= 0.5).await?;
                }
            }
            AnimationTrack::SCALE if values.len() >= 3 => {
                if let Some(instance) = object.instance.as_deref()
                    && jvm.is_instance(instance, "javax/microedition/m3g/Transformable")
                {
                    let mut transformable = cast_ref::<Object3D, Transformable>(object);
                    Transformable::set_scale_fields(jvm, &mut transformable, values[0], values[1], values[2]).await?;
                }
            }
            AnimationTrack::TRANSLATION if values.len() >= 3 => {
                if let Some(instance) = object.instance.as_deref()
                    && jvm.is_instance(instance, "javax/microedition/m3g/Transformable")
                {
                    let mut transformable = cast_ref::<Object3D, Transformable>(object);
                    Transformable::set_translation_fields(jvm, &mut transformable, values[0], values[1], values[2]).await?;
                }
            }
            AnimationTrack::VISIBILITY if !values.is_empty() => {
                if let Some(instance) = object.instance.as_deref()
                    && jvm.is_instance(instance, "javax/microedition/m3g/Node")
                {
                    let mut node = cast_ref::<Object3D, Node>(object);
                    jvm.put_field(&mut node, "renderingEnabled", "Z", values[0] >= 0.5).await?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn get_animation_track(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
    ) -> Result<ClassInstanceRef<AnimationTrack>> {
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(&this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await?;
        let count = jvm.array_length(&tracks).await?;
        if index < 0 || index as usize >= count {
            return Err(jvm
                .exception("java/lang/IndexOutOfBoundsException", "Object3D animation track index")
                .await);
        }
        Ok(jvm
            .load_array(&tracks, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or_else(null_ref))
    }

    async fn get_animation_track_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(&this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await?;
        Ok(jvm.array_length(&tracks).await? as i32)
    }

    async fn remove_animation_track(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        track: ClassInstanceRef<AnimationTrack>,
    ) -> Result<()> {
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(&this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await?;
        let count = jvm.array_length(&tracks).await?;
        let mut values: Vec<ClassInstanceRef<AnimationTrack>> = jvm.load_array(&tracks, 0, count).await?;
        values.retain(|candidate| !same_instance(candidate, &track));
        Self::store_animation_tracks(jvm, &mut this, values).await
    }

    async fn store_animation_tracks(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, values: Vec<ClassInstanceRef<AnimationTrack>>) -> Result<()> {
        let mut array = jvm.instantiate_array("Ljavax/microedition/m3g/AnimationTrack;", values.len()).await?;
        jvm.store_array(&mut array, 0, values).await?;
        jvm.put_field(this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", array)
            .await
    }

    async fn copy_object3d_fields(jvm: &Jvm, source: &ClassInstanceRef<Object3D>, target: &mut ClassInstanceRef<Object3D>) -> Result<()> {
        let user_id = jvm.get_field(source, "userID", "I").await.unwrap_or(0);
        let user_object: ClassInstanceRef<Object> = jvm
            .get_field(source, "userObject", "Ljava/lang/Object;")
            .await
            .unwrap_or_else(|_| null_ref());
        let animation_tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(source, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await
            .unwrap_or_else(|_| null_ref());
        jvm.put_field(target, "userID", "I", user_id).await?;
        jvm.put_field(target, "userObject", "Ljava/lang/Object;", user_object).await?;
        jvm.put_field(target, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", animation_tracks)
            .await
    }

    async fn copy_transformable_fields(
        jvm: &Jvm,
        source: &ClassInstanceRef<Transformable>,
        target: &mut ClassInstanceRef<Transformable>,
    ) -> Result<()> {
        let source_object = cast_ref(source);
        let mut target_object = cast_ref(target);
        Self::copy_object3d_fields(jvm, &source_object, &mut target_object).await?;
        Self::copy_f32_field(jvm, source, target, "translationX", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "translationY", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "translationZ", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "scaleX", 1.0).await?;
        Self::copy_f32_field(jvm, source, target, "scaleY", 1.0).await?;
        Self::copy_f32_field(jvm, source, target, "scaleZ", 1.0).await?;
        Self::copy_f32_field(jvm, source, target, "orientationAngle", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "orientationX", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "orientationY", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "orientationZ", 1.0).await?;
        let matrix = Transformable::generic_matrix(jvm, source).await.unwrap_or_else(|_| identity_matrix());
        Transformable::put_generic_matrix(jvm, target, matrix).await
    }

    async fn copy_node_fields(jvm: &Jvm, source: &ClassInstanceRef<Node>, target: &mut ClassInstanceRef<Node>) -> Result<()> {
        let source_transformable = cast_ref(source);
        let mut target_transformable = cast_ref(target);
        Self::copy_transformable_fields(jvm, &source_transformable, &mut target_transformable).await?;
        let rendering_enabled = jvm.get_field(source, "renderingEnabled", "Z").await.unwrap_or(true);
        let picking_enabled = jvm.get_field(source, "pickingEnabled", "Z").await.unwrap_or(true);
        let alpha_factor = jvm.get_field::<f32>(source, "alphaFactor", "F").await.unwrap_or(1.0);
        let scope = jvm.get_field(source, "scope", "I").await.unwrap_or(-1);
        let z_reference: ClassInstanceRef<Node> = jvm
            .get_field(source, "zReference", "Ljavax/microedition/m3g/Node;")
            .await
            .unwrap_or_else(|_| null_ref());
        let y_reference: ClassInstanceRef<Node> = jvm
            .get_field(source, "yReference", "Ljavax/microedition/m3g/Node;")
            .await
            .unwrap_or_else(|_| null_ref());
        let z_target = jvm.get_field(source, "zTarget", "I").await.unwrap_or(Node::NONE);
        let y_target = jvm.get_field(source, "yTarget", "I").await.unwrap_or(Node::NONE);
        jvm.put_field(target, "parent", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(target, "renderingEnabled", "Z", rendering_enabled).await?;
        jvm.put_field(target, "pickingEnabled", "Z", picking_enabled).await?;
        jvm.put_field(target, "alphaFactor", "F", alpha_factor).await?;
        jvm.put_field(target, "scope", "I", scope).await?;
        jvm.put_field(target, "zReference", "Ljavax/microedition/m3g/Node;", z_reference).await?;
        jvm.put_field(target, "yReference", "Ljavax/microedition/m3g/Node;", y_reference).await?;
        jvm.put_field(target, "zTarget", "I", z_target).await?;
        jvm.put_field(target, "yTarget", "I", y_target).await
    }

    async fn copy_group_fields(jvm: &Jvm, source: &ClassInstanceRef<Group>, target: &mut ClassInstanceRef<Group>) -> Result<()> {
        let source_node = cast_ref(source);
        let mut target_node = cast_ref(target);
        Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;
        let children = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", 0).await?;
        jvm.put_field(target, "children", "[Ljavax/microedition/m3g/Node;", children).await?;
        jvm.put_field(target, "childCount", "I", 0).await
    }

    async fn duplicate_group_children(
        jvm: &Jvm,
        source: &ClassInstanceRef<Group>,
        target: &mut ClassInstanceRef<Group>,
        map: &mut M3gDuplicateMap,
    ) -> Result<()> {
        let child_count: i32 = jvm.get_field(source, "childCount", "I").await.unwrap_or(0);
        if child_count <= 0 {
            return Ok(());
        }

        let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm.get_field(source, "children", "[Ljavax/microedition/m3g/Node;").await?;
        let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
        for child in children {
            if child.is_null() {
                continue;
            }
            let source_object = cast_ref(&child);
            let duplicate = Self::duplicate_object(jvm, &source_object, map).await?;
            Group::push_child(jvm, target, cast_ref::<Object3D, Node>(&duplicate)).await?;
        }
        Ok(())
    }

    async fn duplicate_typed_ref<T>(jvm: &Jvm, source: ClassInstanceRef<T>, map: &mut M3gDuplicateMap) -> Result<ClassInstanceRef<T>> {
        if source.is_null() {
            return Ok(null_ref());
        }
        let source_object = cast_ref(&source);
        let duplicate = Self::duplicate_object(jvm, &source_object, map).await?;
        Ok(cast_ref(&duplicate))
    }

    async fn new_object<T>(jvm: &Jvm, class_name: &str) -> Result<ClassInstanceRef<T>> {
        Ok(jvm.new_class(class_name, "()V", ()).await?.into())
    }

    #[async_recursion::async_recursion]
    async fn duplicate_object(jvm: &Jvm, source: &ClassInstanceRef<Object3D>, map: &mut M3gDuplicateMap) -> Result<ClassInstanceRef<Object3D>> {
        if source.is_null() {
            return Ok(null_ref());
        }
        if let Some(duplicate) = map.get(source) {
            return Ok(duplicate);
        }

        let class_name = source.class_definition().name().to_string();
        match class_name.as_str() {
            "javax/microedition/m3g/World" => {
                let mut target: ClassInstanceRef<World> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);

                let source_group = cast_ref(source);
                let mut target_group = cast_ref(&target);
                Self::copy_group_fields(jvm, &source_group, &mut target_group).await?;
                Self::duplicate_group_children(jvm, &source_group, &mut target_group, map).await?;

                let camera: ClassInstanceRef<Camera> = jvm
                    .get_field(&cast_ref::<Object3D, World>(source), "activeCamera", "Ljavax/microedition/m3g/Camera;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let background: ClassInstanceRef<Background> = jvm
                    .get_field(&cast_ref::<Object3D, World>(source), "background", "Ljavax/microedition/m3g/Background;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let camera = Self::duplicate_typed_ref(jvm, camera, map).await?;
                let background = Self::duplicate_typed_ref(jvm, background, map).await?;
                jvm.put_field(&mut target, "activeCamera", "Ljavax/microedition/m3g/Camera;", camera)
                    .await?;
                jvm.put_field(&mut target, "background", "Ljavax/microedition/m3g/Background;", background)
                    .await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Group" => {
                let mut target: ClassInstanceRef<Group> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                let source_group = cast_ref(source);
                Self::copy_group_fields(jvm, &source_group, &mut target).await?;
                Self::duplicate_group_children(jvm, &source_group, &mut target, map).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Mesh" => {
                let mut target: ClassInstanceRef<Mesh> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                let source_mesh: ClassInstanceRef<Mesh> = cast_ref(source);
                let source_node = cast_ref(&source_mesh);
                let mut target_node = cast_ref(&target);
                Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;

                let vertex_buffer: ClassInstanceRef<VertexBuffer> = jvm
                    .get_field(&source_mesh, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let submesh_count: i32 = jvm.get_field(&source_mesh, "submeshCount", "I").await.unwrap_or(0);
                let count = submesh_count.max(0) as usize;

                let mut index_array = jvm.instantiate_array("Ljavax/microedition/m3g/IndexBuffer;", count).await?;
                let source_indices: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> = jvm
                    .get_field(&source_mesh, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                if !source_indices.is_null() && count > 0 {
                    let indices: Vec<ClassInstanceRef<IndexBuffer>> = jvm.load_array(&source_indices, 0, count).await?;
                    jvm.store_array(&mut index_array, 0, indices).await?;
                }

                let mut appearance_array = jvm.instantiate_array("Ljavax/microedition/m3g/Appearance;", count).await?;
                let source_appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> = jvm
                    .get_field(&source_mesh, "appearances", "[Ljavax/microedition/m3g/Appearance;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                if !source_appearances.is_null() && count > 0 {
                    let appearances: Vec<ClassInstanceRef<Appearance>> = jvm.load_array(&source_appearances, 0, count).await?;
                    let mut duplicates = Vec::with_capacity(appearances.len());
                    for appearance in appearances {
                        duplicates.push(Self::duplicate_typed_ref(jvm, appearance, map).await?);
                    }
                    jvm.store_array(&mut appearance_array, 0, duplicates).await?;
                }

                jvm.put_field(&mut target, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", vertex_buffer)
                    .await?;
                jvm.put_field(&mut target, "submeshCount", "I", submesh_count).await?;
                jvm.put_field(&mut target, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", index_array)
                    .await?;
                jvm.put_field(&mut target, "appearances", "[Ljavax/microedition/m3g/Appearance;", appearance_array)
                    .await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Appearance" => {
                let mut target: ClassInstanceRef<Appearance> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;

                let source_appearance: ClassInstanceRef<Appearance> = cast_ref(source);
                let compositing: ClassInstanceRef<CompositingMode> = jvm
                    .get_field(&source_appearance, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let fog: ClassInstanceRef<Fog> = jvm
                    .get_field(&source_appearance, "fog", "Ljavax/microedition/m3g/Fog;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let polygon: ClassInstanceRef<PolygonMode> = jvm
                    .get_field(&source_appearance, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let material: ClassInstanceRef<Material> = jvm
                    .get_field(&source_appearance, "material", "Ljavax/microedition/m3g/Material;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let texture: ClassInstanceRef<Texture2D> = jvm
                    .get_field(&source_appearance, "texture0", "Ljavax/microedition/m3g/Texture2D;")
                    .await
                    .unwrap_or_else(|_| null_ref());

                Self::copy_i32_field(jvm, &source_appearance, &mut target, "layer", "I", 0).await?;
                jvm.put_field(
                    &mut target,
                    "compositingMode",
                    "Ljavax/microedition/m3g/CompositingMode;",
                    Self::duplicate_typed_ref(jvm, compositing, map).await?,
                )
                .await?;
                jvm.put_field(
                    &mut target,
                    "fog",
                    "Ljavax/microedition/m3g/Fog;",
                    Self::duplicate_typed_ref(jvm, fog, map).await?,
                )
                .await?;
                jvm.put_field(
                    &mut target,
                    "polygonMode",
                    "Ljavax/microedition/m3g/PolygonMode;",
                    Self::duplicate_typed_ref(jvm, polygon, map).await?,
                )
                .await?;
                jvm.put_field(
                    &mut target,
                    "material",
                    "Ljavax/microedition/m3g/Material;",
                    Self::duplicate_typed_ref(jvm, material, map).await?,
                )
                .await?;
                jvm.put_field(
                    &mut target,
                    "texture0",
                    "Ljavax/microedition/m3g/Texture2D;",
                    Self::duplicate_typed_ref(jvm, texture, map).await?,
                )
                .await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Camera" => {
                let mut target: ClassInstanceRef<Camera> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                let source_camera: ClassInstanceRef<Camera> = cast_ref(source);
                let source_node = cast_ref(&source_camera);
                let mut target_node = cast_ref(&target);
                Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;
                Self::copy_i32_field(jvm, &source_camera, &mut target, "projectionMode", "I", Camera::PERSPECTIVE).await?;
                Self::copy_f32_field(jvm, &source_camera, &mut target, "fovy", 45.0).await?;
                Self::copy_f32_field(jvm, &source_camera, &mut target, "parallelHeight", 1.0).await?;
                Self::copy_f32_field(jvm, &source_camera, &mut target, "aspect", 1.0).await?;
                Self::copy_f32_field(jvm, &source_camera, &mut target, "near", 1.0).await?;
                Self::copy_f32_field(jvm, &source_camera, &mut target, "far", 1000.0).await?;
                let projection = Camera::generic_projection(jvm, &source_camera)
                    .await
                    .unwrap_or_else(|_| identity_matrix());
                Camera::put_generic_projection(jvm, &mut target, projection).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Background" => {
                let mut target: ClassInstanceRef<Background> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
                let source_background: ClassInstanceRef<Background> = cast_ref(source);
                Self::copy_i32_field(jvm, &source_background, &mut target, "color", "I", 0xff000000u32 as i32).await?;
                Self::copy_bool_field(jvm, &source_background, &mut target, "colorClear", true).await?;
                Self::copy_bool_field(jvm, &source_background, &mut target, "depthClear", true).await?;
                Self::copy_i32_field(jvm, &source_background, &mut target, "imageModeX", "I", 32).await?;
                Self::copy_i32_field(jvm, &source_background, &mut target, "imageModeY", "I", 32).await?;
                Self::copy_i32_field(jvm, &source_background, &mut target, "cropX", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_background, &mut target, "cropY", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_background, &mut target, "cropW", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_background, &mut target, "cropH", "I", 0).await?;
                let image: ClassInstanceRef<Image2D> = jvm
                    .get_field(&source_background, "image", "Ljavax/microedition/m3g/Image2D;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                jvm.put_field(&mut target, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/CompositingMode" => {
                let mut target: ClassInstanceRef<CompositingMode> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
                let source_mode: ClassInstanceRef<CompositingMode> = cast_ref(source);
                Self::copy_f32_field(jvm, &source_mode, &mut target, "alphaThreshold", 0.0).await?;
                Self::copy_bool_field(jvm, &source_mode, &mut target, "alphaWrite", true).await?;
                Self::copy_i32_field(jvm, &source_mode, &mut target, "blending", "I", CompositingMode::REPLACE).await?;
                Self::copy_bool_field(jvm, &source_mode, &mut target, "colorWrite", true).await?;
                Self::copy_bool_field(jvm, &source_mode, &mut target, "depthTest", true).await?;
                Self::copy_bool_field(jvm, &source_mode, &mut target, "depthWrite", true).await?;
                Self::copy_f32_field(jvm, &source_mode, &mut target, "depthOffsetFactor", 0.0).await?;
                Self::copy_f32_field(jvm, &source_mode, &mut target, "depthOffsetUnits", 0.0).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Fog" => {
                let mut target: ClassInstanceRef<Fog> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
                let source_fog: ClassInstanceRef<Fog> = cast_ref(source);
                Self::copy_i32_field(jvm, &source_fog, &mut target, "color", "I", 0).await?;
                Self::copy_f32_field(jvm, &source_fog, &mut target, "density", 1.0).await?;
                Self::copy_i32_field(jvm, &source_fog, &mut target, "mode", "I", Fog::LINEAR).await?;
                Self::copy_f32_field(jvm, &source_fog, &mut target, "near", 0.0).await?;
                Self::copy_f32_field(jvm, &source_fog, &mut target, "far", 0.0).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/PolygonMode" => {
                let mut target: ClassInstanceRef<PolygonMode> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
                let source_mode: ClassInstanceRef<PolygonMode> = cast_ref(source);
                Self::copy_i32_field(jvm, &source_mode, &mut target, "culling", "I", 160).await?;
                Self::copy_bool_field(jvm, &source_mode, &mut target, "localCameraLighting", false).await?;
                Self::copy_bool_field(jvm, &source_mode, &mut target, "perspectiveCorrection", true).await?;
                Self::copy_i32_field(jvm, &source_mode, &mut target, "shading", "I", PolygonMode::SHADE_SMOOTH).await?;
                Self::copy_bool_field(jvm, &source_mode, &mut target, "twoSidedLighting", false).await?;
                Self::copy_i32_field(jvm, &source_mode, &mut target, "winding", "I", 168).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Texture2D" => {
                let mut target: ClassInstanceRef<Texture2D> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                let source_texture: ClassInstanceRef<Texture2D> = cast_ref(source);
                let source_transformable = cast_ref(&source_texture);
                let mut target_transformable = cast_ref(&target);
                let mut duplicate_object = duplicate.clone();
                Self::copy_object3d_fields(jvm, source, &mut duplicate_object).await?;
                Self::copy_transformable_fields(jvm, &source_transformable, &mut target_transformable).await?;
                Self::copy_i32_field(jvm, &source_texture, &mut target, "blendColor", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_texture, &mut target, "blending", "I", Texture2D::FUNC_MODULATE).await?;
                Self::copy_i32_field(jvm, &source_texture, &mut target, "wrappingS", "I", Texture2D::WRAP_REPEAT).await?;
                Self::copy_i32_field(jvm, &source_texture, &mut target, "wrappingT", "I", Texture2D::WRAP_REPEAT).await?;
                Self::copy_i32_field(jvm, &source_texture, &mut target, "levelFilter", "I", Texture2D::FILTER_BASE_LEVEL).await?;
                Self::copy_i32_field(jvm, &source_texture, &mut target, "imageFilter", "I", Texture2D::FILTER_NEAREST).await?;
                let image: ClassInstanceRef<Image2D> = jvm
                    .get_field(&source_texture, "image", "Ljavax/microedition/m3g/Image2D;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                jvm.put_field(&mut target, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Sprite3D" => {
                let source_sprite: ClassInstanceRef<Sprite3D> = cast_ref(source);
                let scaled = jvm.get_field(&source_sprite, "scaled", "Z").await.unwrap_or(false);
                let image: ClassInstanceRef<Image2D> = jvm
                    .get_field(&source_sprite, "image", "Ljavax/microedition/m3g/Image2D;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let appearance: ClassInstanceRef<Appearance> = jvm
                    .get_field(&source_sprite, "appearance", "Ljavax/microedition/m3g/Appearance;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let appearance = Self::duplicate_typed_ref(jvm, appearance, map).await?;
                let mut target: ClassInstanceRef<Sprite3D> = jvm
                    .new_class(
                        &class_name,
                        "(ZLjavax/microedition/m3g/Image2D;Ljavax/microedition/m3g/Appearance;)V",
                        (scaled, image.clone(), appearance.clone()),
                    )
                    .await?
                    .into();
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                let source_node = cast_ref(&source_sprite);
                let mut target_node = cast_ref(&target);
                Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;
                jvm.put_field(&mut target, "scaled", "Z", scaled).await?;
                jvm.put_field(&mut target, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
                jvm.put_field(&mut target, "appearance", "Ljavax/microedition/m3g/Appearance;", appearance)
                    .await?;
                Self::copy_i32_field(jvm, &source_sprite, &mut target, "cropX", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_sprite, &mut target, "cropY", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_sprite, &mut target, "cropW", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_sprite, &mut target, "cropH", "I", 0).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Image2D" => {
                let mut target: ClassInstanceRef<Image2D> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
                let source_image: ClassInstanceRef<Image2D> = cast_ref(source);
                Self::copy_i32_field(jvm, &source_image, &mut target, "format", "I", Image2D::RGBA).await?;
                Self::copy_i32_field(jvm, &source_image, &mut target, "width", "I", 1).await?;
                Self::copy_i32_field(jvm, &source_image, &mut target, "height", "I", 1).await?;
                Self::copy_bool_field(jvm, &source_image, &mut target, "mutable", false).await?;
                let image: ClassInstanceRef<Image> = jvm
                    .get_field(&source_image, "image", "Ljavax/microedition/lcdui/Image;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                jvm.put_field(&mut target, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Light" => {
                let mut target: ClassInstanceRef<Light> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                let source_node = cast_ref(source);
                let mut target_node = cast_ref(&target);
                Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;
                let source_light: ClassInstanceRef<Light> = cast_ref(source);
                Self::copy_f32_field(jvm, &source_light, &mut target, "constantAttenuation", 1.0).await?;
                Self::copy_f32_field(jvm, &source_light, &mut target, "linearAttenuation", 0.0).await?;
                Self::copy_f32_field(jvm, &source_light, &mut target, "quadraticAttenuation", 0.0).await?;
                Self::copy_i32_field(jvm, &source_light, &mut target, "color", "I", 0x00ff_ffff).await?;
                Self::copy_f32_field(jvm, &source_light, &mut target, "intensity", 1.0).await?;
                Self::copy_i32_field(jvm, &source_light, &mut target, "mode", "I", Light::DIRECTIONAL).await?;
                Self::copy_f32_field(jvm, &source_light, &mut target, "spotAngle", 45.0).await?;
                Self::copy_f32_field(jvm, &source_light, &mut target, "spotExponent", 0.0).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/Material" | "javax/microedition/m3g/IndexBuffer" => {
                let mut target: ClassInstanceRef<Object3D> = jvm.new_class(&class_name, "()V", ()).await?.into();
                map.insert(source, &target);
                Self::copy_object3d_fields(jvm, source, &mut target).await?;
                if class_name == "javax/microedition/m3g/Material" {
                    let source_material: ClassInstanceRef<Material> = cast_ref(source);
                    let mut target_material: ClassInstanceRef<Material> = cast_ref(&target);
                    Self::copy_i32_field(jvm, &source_material, &mut target_material, "ambientColor", "I", 0x0033_3333).await?;
                    Self::copy_i32_field(jvm, &source_material, &mut target_material, "diffuseColor", "I", 0xffff_ffffu32 as i32).await?;
                    Self::copy_i32_field(jvm, &source_material, &mut target_material, "emissiveColor", "I", 0).await?;
                    Self::copy_i32_field(jvm, &source_material, &mut target_material, "specularColor", "I", 0).await?;
                    Self::copy_f32_field(jvm, &source_material, &mut target_material, "shininess", 0.0).await?;
                    Self::copy_bool_field(jvm, &source_material, &mut target_material, "vertexColorTracking", false).await?;
                }
                Ok(target)
            }
            "javax/microedition/m3g/TriangleStripArray" => {
                let mut target: ClassInstanceRef<TriangleStripArray> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
                let source_array: ClassInstanceRef<TriangleStripArray> = cast_ref(source);
                let indices: ClassInstanceRef<Array<i32>> = jvm.get_field(&source_array, "indices", "[I").await.unwrap_or_else(|_| null_ref());
                let lengths: ClassInstanceRef<Array<i32>> = jvm.get_field(&source_array, "stripLengths", "[I").await.unwrap_or_else(|_| null_ref());
                jvm.put_field(&mut target, "indices", "[I", indices).await?;
                jvm.put_field(&mut target, "stripLengths", "[I", lengths).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/VertexArray" => {
                let mut target: ClassInstanceRef<VertexArray> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
                let source_array: ClassInstanceRef<VertexArray> = cast_ref(source);
                Self::copy_i32_field(jvm, &source_array, &mut target, "componentSize", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_array, &mut target, "componentCount", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_array, &mut target, "vertexCount", "I", 0).await?;
                Self::copy_i32_field(jvm, &source_array, &mut target, "version", "I", 0).await?;
                let byte_data: ClassInstanceRef<Array<i8>> = jvm.get_field(&source_array, "byteData", "[B").await.unwrap_or_else(|_| null_ref());
                let short_data: ClassInstanceRef<Array<i16>> = jvm.get_field(&source_array, "shortData", "[S").await.unwrap_or_else(|_| null_ref());
                jvm.put_field(&mut target, "byteData", "[B", byte_data).await?;
                jvm.put_field(&mut target, "shortData", "[S", short_data).await?;
                Ok(duplicate)
            }
            "javax/microedition/m3g/VertexBuffer" => {
                let mut target: ClassInstanceRef<VertexBuffer> = Self::new_object(jvm, &class_name).await?;
                let duplicate = cast_ref(&target);
                map.insert(source, &duplicate);
                Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
                let source_buffer: ClassInstanceRef<VertexBuffer> = cast_ref(source);
                Self::copy_i32_field(jvm, &source_buffer, &mut target, "defaultColor", "I", 0x00ff_ffff).await?;
                Self::copy_f32_field(jvm, &source_buffer, &mut target, "positionScale", 1.0).await?;
                Self::copy_f32_field(jvm, &source_buffer, &mut target, "texScale", 1.0).await?;
                let positions: ClassInstanceRef<VertexArray> = jvm
                    .get_field(&source_buffer, "positions", "Ljavax/microedition/m3g/VertexArray;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let normals: ClassInstanceRef<VertexArray> = jvm
                    .get_field(&source_buffer, "normals", "Ljavax/microedition/m3g/VertexArray;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let colors: ClassInstanceRef<VertexArray> = jvm
                    .get_field(&source_buffer, "colors", "Ljavax/microedition/m3g/VertexArray;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let tex_coords: ClassInstanceRef<VertexArray> = jvm
                    .get_field(&source_buffer, "texCoords0", "Ljavax/microedition/m3g/VertexArray;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                let position_bias: ClassInstanceRef<Array<f32>> =
                    jvm.get_field(&source_buffer, "positionBias", "[F").await.unwrap_or_else(|_| null_ref());
                let tex_bias: ClassInstanceRef<Array<f32>> = jvm.get_field(&source_buffer, "texBias", "[F").await.unwrap_or_else(|_| null_ref());
                jvm.put_field(&mut target, "positions", "Ljavax/microedition/m3g/VertexArray;", positions)
                    .await?;
                jvm.put_field(&mut target, "normals", "Ljavax/microedition/m3g/VertexArray;", normals)
                    .await?;
                jvm.put_field(&mut target, "colors", "Ljavax/microedition/m3g/VertexArray;", colors)
                    .await?;
                jvm.put_field(&mut target, "texCoords0", "Ljavax/microedition/m3g/VertexArray;", tex_coords)
                    .await?;
                jvm.put_field(&mut target, "positionBias", "[F", position_bias).await?;
                jvm.put_field(&mut target, "texBias", "[F", tex_bias).await?;
                Ok(duplicate)
            }
            _ => {
                tracing::debug!(target: "rustjava_m3g", "m3g.Object3D.duplicate share unsupported class={class_name}");
                map.entries.push((source.clone(), source.clone()));
                map.note_shared();
                Ok(source.clone())
            }
        }
    }

    async fn copy_i32_field<S, T>(
        jvm: &Jvm,
        source: &ClassInstanceRef<S>,
        target: &mut ClassInstanceRef<T>,
        name: &str,
        descriptor: &str,
        default: i32,
    ) -> Result<()> {
        let value = jvm.get_field(source, name, descriptor).await.unwrap_or(default);
        jvm.put_field(target, name, descriptor, value).await
    }

    async fn copy_f32_field<S, T>(jvm: &Jvm, source: &ClassInstanceRef<S>, target: &mut ClassInstanceRef<T>, name: &str, default: f32) -> Result<()> {
        let value = jvm.get_field(source, name, "F").await.unwrap_or(default);
        jvm.put_field(target, name, "F", value).await
    }

    async fn copy_bool_field<S, T>(
        jvm: &Jvm,
        source: &ClassInstanceRef<S>,
        target: &mut ClassInstanceRef<T>,
        name: &str,
        default: bool,
    ) -> Result<()> {
        let value = jvm.get_field(source, name, "Z").await.unwrap_or(default);
        jvm.put_field(target, name, "Z", value).await
    }

    async fn push_find_references(jvm: &Jvm, current: &ClassInstanceRef<Object3D>, stack: &mut Vec<ClassInstanceRef<Object3D>>) -> Result<()> {
        let class_name = current.class_definition().name().to_string();
        if class_name == "javax/microedition/m3g/Group" || class_name == "javax/microedition/m3g/World" {
            let child_count: i32 = jvm.get_field(current, "childCount", "I").await.unwrap_or(0);
            if child_count > 0 {
                let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> =
                    jvm.get_field(current, "children", "[Ljavax/microedition/m3g/Node;").await?;
                let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
                stack.extend(children.iter().rev().map(cast_ref::<Node, Object3D>));
            }
        }

        match class_name.as_str() {
            "javax/microedition/m3g/World" => {
                Self::push_object_field::<Camera>(jvm, current, "activeCamera", "Ljavax/microedition/m3g/Camera;", stack).await?;
                Self::push_object_field::<Background>(jvm, current, "background", "Ljavax/microedition/m3g/Background;", stack).await?;
            }
            "javax/microedition/m3g/Background" => {
                Self::push_object_field::<Image2D>(jvm, current, "image", "Ljavax/microedition/m3g/Image2D;", stack).await?;
            }
            "javax/microedition/m3g/Appearance" => {
                Self::push_object_field::<CompositingMode>(jvm, current, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;", stack)
                    .await?;
                Self::push_object_field::<Fog>(jvm, current, "fog", "Ljavax/microedition/m3g/Fog;", stack).await?;
                Self::push_object_field::<PolygonMode>(jvm, current, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;", stack).await?;
                Self::push_object_field::<Material>(jvm, current, "material", "Ljavax/microedition/m3g/Material;", stack).await?;
                Self::push_object_field::<Texture2D>(jvm, current, "texture0", "Ljavax/microedition/m3g/Texture2D;", stack).await?;
            }
            "javax/microedition/m3g/Texture2D" => {
                Self::push_object_field::<Image2D>(jvm, current, "image", "Ljavax/microedition/m3g/Image2D;", stack).await?;
            }
            "javax/microedition/m3g/Sprite3D" => {
                Self::push_object_field::<Image2D>(jvm, current, "image", "Ljavax/microedition/m3g/Image2D;", stack).await?;
                Self::push_object_field::<Appearance>(jvm, current, "appearance", "Ljavax/microedition/m3g/Appearance;", stack).await?;
            }
            "javax/microedition/m3g/Mesh" => {
                Self::push_object_field::<VertexBuffer>(jvm, current, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", stack).await?;
                let count: i32 = jvm.get_field(current, "submeshCount", "I").await.unwrap_or(0);
                if count > 0 {
                    let index_buffers: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> =
                        jvm.get_field(current, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;").await?;
                    if !index_buffers.is_null() {
                        let values: Vec<ClassInstanceRef<IndexBuffer>> = jvm.load_array(&index_buffers, 0, count as usize).await?;
                        stack.extend(values.iter().rev().map(cast_ref::<IndexBuffer, Object3D>));
                    }
                    let appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> =
                        jvm.get_field(current, "appearances", "[Ljavax/microedition/m3g/Appearance;").await?;
                    if !appearances.is_null() {
                        let values: Vec<ClassInstanceRef<Appearance>> = jvm.load_array(&appearances, 0, count as usize).await?;
                        stack.extend(values.iter().rev().map(cast_ref::<Appearance, Object3D>));
                    }
                }
            }
            "javax/microedition/m3g/VertexBuffer" => {
                Self::push_object_field::<VertexArray>(jvm, current, "positions", "Ljavax/microedition/m3g/VertexArray;", stack).await?;
                Self::push_object_field::<VertexArray>(jvm, current, "normals", "Ljavax/microedition/m3g/VertexArray;", stack).await?;
                Self::push_object_field::<VertexArray>(jvm, current, "colors", "Ljavax/microedition/m3g/VertexArray;", stack).await?;
                Self::push_object_field::<VertexArray>(jvm, current, "texCoords0", "Ljavax/microedition/m3g/VertexArray;", stack).await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn push_object_field<T>(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        name: &str,
        descriptor: &str,
        stack: &mut Vec<ClassInstanceRef<Object3D>>,
    ) -> Result<()> {
        let value: ClassInstanceRef<T> = jvm.get_field(source, name, descriptor).await.unwrap_or_else(|_| null_ref());
        if !value.is_null() {
            stack.push(cast_ref(&value));
        }
        Ok(())
    }

    async fn get_user_id(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "userID", "I").await
    }

    async fn set_user_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, user_id: i32) -> Result<()> {
        jvm.put_field(&mut this, "userID", "I", user_id).await
    }

    async fn set_user_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        user_object: ClassInstanceRef<Object>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "userObject", "Ljava/lang/Object;", user_object).await
    }

    async fn get_user_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "userObject", "Ljava/lang/Object;").await
    }
}

impl Transform {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Transform",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljavax/microedition/m3g/Transform;)V", Self::init_copy, Default::default()),
                JavaMethodProto::new("get", "([F)V", Self::get, Default::default()),
                JavaMethodProto::new("invert", "()V", Self::invert, Default::default()),
                JavaMethodProto::new(
                    "postMultiply",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::post_multiply,
                    Default::default(),
                ),
                JavaMethodProto::new("set", "([F)V", Self::set, Default::default()),
                JavaMethodProto::new("set", "(Ljavax/microedition/m3g/Transform;)V", Self::set_transform, Default::default()),
                JavaMethodProto::new("setIdentity", "()V", Self::set_identity, Default::default()),
                JavaMethodProto::new("postRotate", "(FFFF)V", Self::post_rotate, Default::default()),
                JavaMethodProto::new("postRotateQuat", "(FFFF)V", Self::post_rotate_quat, Default::default()),
                JavaMethodProto::new("postScale", "(FFF)V", Self::post_scale, Default::default()),
                JavaMethodProto::new("postTranslate", "(FFF)V", Self::post_translate, Default::default()),
                JavaMethodProto::new("transform", "([F)V", Self::transform_float_array, Default::default()),
                JavaMethodProto::new(
                    "transform",
                    "(Ljavax/microedition/m3g/VertexArray;[FZ)V",
                    Self::transform_vertex_array,
                    Default::default(),
                ),
                JavaMethodProto::new("transpose", "()V", Self::transpose, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("matrix", "[F", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::put_matrix(jvm, &mut this, identity_matrix()).await
    }

    async fn init_copy(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, source: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform source").await);
        }
        let matrix = Self::matrix(jvm, &source).await?;
        Self::put_matrix(jvm, &mut this, matrix).await
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut dst: ClassInstanceRef<Array<f32>>) -> Result<()> {
        if dst.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.get").await);
        }
        if jvm.array_length(&dst).await? < 16 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "matrix array too small").await);
        }
        let matrix = Self::matrix(jvm, &this).await?;
        store_raw_f32_array(jvm, &mut dst, &matrix).await
    }

    async fn set(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, src: ClassInstanceRef<Array<f32>>) -> Result<()> {
        if src.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.set").await);
        }
        if jvm.array_length(&src).await? < 16 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "matrix array too small").await);
        }
        let matrix = raw_f32_array(jvm, &src, 16).await?;
        Self::put_matrix(jvm, &mut this, matrix_to_array(&matrix)).await
    }

    async fn set_transform(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, source: ClassInstanceRef<Self>) -> Result<()> {
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.set").await);
        }
        let matrix = Self::matrix(jvm, &source).await?;
        Self::put_matrix(jvm, &mut this, matrix).await
    }

    async fn set_identity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        Self::put_matrix(jvm, &mut this, identity_matrix()).await
    }

    async fn invert(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        let Some(inverted) = invert_matrix(matrix) else {
            return Err(jvm.exception("java/lang/ArithmeticException", "singular transform").await);
        };
        Self::put_matrix(jvm, &mut this, inverted).await
    }

    async fn transpose(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        Self::put_matrix(jvm, &mut this, transpose_matrix(matrix)).await
    }

    async fn post_multiply(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<()> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.postMultiply").await);
        }
        let matrix = Self::matrix(jvm, &this).await?;
        let other = Self::matrix(jvm, &other).await?;
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, other)).await
    }

    async fn post_rotate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: f32, ax: f32, ay: f32, az: f32) -> Result<()> {
        if angle != 0.0 && ax == 0.0 && ay == 0.0 && az == 0.0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "zero rotation axis").await);
        }
        let matrix = Self::matrix(jvm, &this).await?;
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, rotation_matrix(angle, ax, ay, az))).await
    }

    async fn post_rotate_quat(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, qx: f32, qy: f32, qz: f32, qw: f32) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        let Some(rotation) = quaternion_matrix(qx, qy, qz, qw) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "zero quaternion").await);
        };
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, rotation)).await
    }

    async fn post_scale(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sx: f32, sy: f32, sz: f32) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, scale_matrix(sx, sy, sz))).await
    }

    async fn post_translate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, tx: f32, ty: f32, tz: f32) -> Result<()> {
        let matrix = Self::matrix(jvm, &this).await?;
        Self::put_matrix(jvm, &mut this, multiply_matrix(matrix, translation_matrix(tx, ty, tz))).await
    }

    async fn transform_float_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut values: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if values.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.transform").await);
        }
        let len = jvm.array_length(&values).await?;
        if len % 4 != 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "vector array length").await);
        }
        if len == 0 {
            return Ok(());
        }
        let matrix = Self::matrix(jvm, &this).await?;
        let mut data = raw_f32_array(jvm, &values, len).await?;
        for vector in data.chunks_exact_mut(4) {
            let transformed = transform_vec4(matrix, [vector[0], vector[1], vector[2], vector[3]]);
            vector.copy_from_slice(&transformed);
        }
        store_raw_f32_array(jvm, &mut values, &data).await
    }

    async fn transform_vertex_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<VertexArray>,
        mut out: ClassInstanceRef<Array<f32>>,
        w: bool,
    ) -> Result<()> {
        if array.is_null() || out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transform.transform").await);
        }
        let component_count: i32 = jvm.get_field(&array, "componentCount", "I").await?;
        let vertex_count: i32 = jvm.get_field(&array, "vertexCount", "I").await?;
        if component_count == 4 || jvm.array_length(&out).await? < vertex_count.max(0) as usize * 4 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "output array too small").await);
        }

        let matrix = Self::matrix(jvm, &this).await?;
        let components = Graphics3D::vertex_array_components(jvm, &array).await?;
        let mut transformed = Vec::with_capacity(components.len() * 4);
        for vertex in components {
            let value = transform_vec4(
                matrix,
                [
                    vertex.first().copied().unwrap_or(0.0),
                    vertex.get(1).copied().unwrap_or(0.0),
                    vertex.get(2).copied().unwrap_or(0.0),
                    if w { 1.0 } else { 0.0 },
                ],
            );
            transformed.extend_from_slice(&value);
        }
        store_raw_f32_array(jvm, &mut out, &transformed).await
    }

    async fn matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let matrix: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "matrix", "[F").await?;
        let values = raw_f32_array(jvm, &matrix, 16).await?;
        Ok(matrix_to_array(&values))
    }

    async fn put_matrix(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, matrix: [f32; 16]) -> Result<()> {
        let mut array = jvm.instantiate_array("F", 16).await?;
        store_raw_f32_array(jvm, &mut array, &matrix).await?;
        jvm.put_field(this, "matrix", "[F", array).await
    }
}

impl Transformable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Transformable",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getCompositeTransform",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::get_composite_transform,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getTransform",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::get_transform,
                    Default::default(),
                ),
                JavaMethodProto::new("getOrientation", "([F)V", Self::get_orientation, Default::default()),
                JavaMethodProto::new("getScale", "([F)V", Self::get_scale, Default::default()),
                JavaMethodProto::new("getTranslation", "([F)V", Self::get_translation, Default::default()),
                JavaMethodProto::new("postRotate", "(FFFF)V", Self::post_rotate, Default::default()),
                JavaMethodProto::new("preRotate", "(FFFF)V", Self::pre_rotate, Default::default()),
                JavaMethodProto::new("scale", "(FFF)V", Self::scale, Default::default()),
                JavaMethodProto::new("setOrientation", "(FFFF)V", Self::set_orientation, Default::default()),
                JavaMethodProto::new("setScale", "(FFF)V", Self::set_scale, Default::default()),
                JavaMethodProto::new(
                    "setTransform",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::set_transform,
                    Default::default(),
                ),
                JavaMethodProto::new("setTranslation", "(FFF)V", Self::set_translation, Default::default()),
                JavaMethodProto::new("translate", "(FFF)V", Self::translate, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("translationX", "F", Default::default()),
                JavaFieldProto::new("translationY", "F", Default::default()),
                JavaFieldProto::new("translationZ", "F", Default::default()),
                JavaFieldProto::new("scaleX", "F", Default::default()),
                JavaFieldProto::new("scaleY", "F", Default::default()),
                JavaFieldProto::new("scaleZ", "F", Default::default()),
                JavaFieldProto::new("orientationAngle", "F", Default::default()),
                JavaFieldProto::new("orientationX", "F", Default::default()),
                JavaFieldProto::new("orientationY", "F", Default::default()),
                JavaFieldProto::new("orientationZ", "F", Default::default()),
                JavaFieldProto::new("transform", "[F", Default::default()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        Self::set_translation_fields(jvm, &mut this, 0.0, 0.0, 0.0).await?;
        Self::set_scale_fields(jvm, &mut this, 1.0, 1.0, 1.0).await?;
        Self::set_orientation_fields(jvm, &mut this, 0.0, 0.0, 0.0, 1.0).await?;
        let mut transform = jvm.instantiate_array("F", 16).await?;
        store_raw_f32_array(jvm, &mut transform, &identity_matrix()).await?;
        jvm.put_field(&mut this, "transform", "[F", transform).await
    }

    async fn get_composite_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut dst: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        let matrix = Self::local_matrix(jvm, &this).await?;
        Transform::put_matrix(jvm, &mut dst, matrix).await
    }

    async fn get_transform(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut dst: ClassInstanceRef<Transform>) -> Result<()> {
        let matrix = Self::generic_matrix(jvm, &this).await?;
        Transform::put_matrix(jvm, &mut dst, matrix).await
    }

    async fn get_orientation(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut dst: ClassInstanceRef<Array<f32>>) -> Result<()> {
        if dst.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transformable.getOrientation").await);
        }
        if jvm.array_length(&dst).await? < 4 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "orientation array too small").await);
        }
        let values: Vec<f32> = vec![
            jvm.get_field::<f32>(&this, "orientationAngle", "F").await?,
            jvm.get_field::<f32>(&this, "orientationX", "F").await?,
            jvm.get_field::<f32>(&this, "orientationY", "F").await?,
            jvm.get_field::<f32>(&this, "orientationZ", "F").await?,
        ];
        store_raw_f32_array(jvm, &mut dst, &values).await
    }

    async fn get_scale(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut dst: ClassInstanceRef<Array<f32>>) -> Result<()> {
        if dst.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transformable.getScale").await);
        }
        if jvm.array_length(&dst).await? < 3 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "scale array too small").await);
        }
        let values: Vec<f32> = vec![
            jvm.get_field::<f32>(&this, "scaleX", "F").await?,
            jvm.get_field::<f32>(&this, "scaleY", "F").await?,
            jvm.get_field::<f32>(&this, "scaleZ", "F").await?,
        ];
        store_raw_f32_array(jvm, &mut dst, &values).await
    }

    async fn get_translation(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut dst: ClassInstanceRef<Array<f32>>) -> Result<()> {
        if dst.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transformable.getTranslation").await);
        }
        if jvm.array_length(&dst).await? < 3 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "translation array too small").await);
        }
        let values: Vec<f32> = vec![
            jvm.get_field::<f32>(&this, "translationX", "F").await?,
            jvm.get_field::<f32>(&this, "translationY", "F").await?,
            jvm.get_field::<f32>(&this, "translationZ", "F").await?,
        ];
        store_raw_f32_array(jvm, &mut dst, &values).await
    }

    async fn post_rotate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: f32, ax: f32, ay: f32, az: f32) -> Result<()> {
        let matrix = Self::orientation_matrix(jvm, &this).await?;
        let (angle, ax, ay, az) = rotation_matrix_to_axis_angle(multiply_matrix(matrix, rotation_matrix(angle, ax, ay, az)));
        Self::set_orientation_fields(jvm, &mut this, angle, ax, ay, az).await
    }

    async fn pre_rotate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: f32, ax: f32, ay: f32, az: f32) -> Result<()> {
        let matrix = Self::orientation_matrix(jvm, &this).await?;
        let (angle, ax, ay, az) = rotation_matrix_to_axis_angle(multiply_matrix(rotation_matrix(angle, ax, ay, az), matrix));
        Self::set_orientation_fields(jvm, &mut this, angle, ax, ay, az).await
    }

    async fn scale(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sx: f32, sy: f32, sz: f32) -> Result<()> {
        let old_x: f32 = jvm.get_field(&this, "scaleX", "F").await?;
        let old_y: f32 = jvm.get_field(&this, "scaleY", "F").await?;
        let old_z: f32 = jvm.get_field(&this, "scaleZ", "F").await?;
        Self::set_scale_fields(jvm, &mut this, old_x * sx, old_y * sy, old_z * sz).await
    }

    async fn set_orientation(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        angle: f32,
        ax: f32,
        ay: f32,
        az: f32,
    ) -> Result<()> {
        Self::set_orientation_fields(jvm, &mut this, angle, ax, ay, az).await
    }

    async fn set_scale(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sx: f32, sy: f32, sz: f32) -> Result<()> {
        Self::set_scale_fields(jvm, &mut this, sx, sy, sz).await
    }

    async fn set_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        let matrix = Transform::matrix(jvm, &transform).await?;
        Self::put_generic_matrix(jvm, &mut this, matrix).await
    }

    async fn set_translation(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, tx: f32, ty: f32, tz: f32) -> Result<()> {
        Self::set_translation_fields(jvm, &mut this, tx, ty, tz).await
    }

    async fn translate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, tx: f32, ty: f32, tz: f32) -> Result<()> {
        let old_x: f32 = jvm.get_field(&this, "translationX", "F").await?;
        let old_y: f32 = jvm.get_field(&this, "translationY", "F").await?;
        let old_z: f32 = jvm.get_field(&this, "translationZ", "F").await?;
        Self::set_translation_fields(jvm, &mut this, old_x + tx, old_y + ty, old_z + tz).await
    }

    async fn set_translation_fields(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, tx: f32, ty: f32, tz: f32) -> Result<()> {
        jvm.put_field(this, "translationX", "F", tx).await?;
        jvm.put_field(this, "translationY", "F", ty).await?;
        jvm.put_field(this, "translationZ", "F", tz).await
    }

    async fn set_scale_fields(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, sx: f32, sy: f32, sz: f32) -> Result<()> {
        jvm.put_field(this, "scaleX", "F", sx).await?;
        jvm.put_field(this, "scaleY", "F", sy).await?;
        jvm.put_field(this, "scaleZ", "F", sz).await
    }

    async fn set_orientation_fields(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, angle: f32, ax: f32, ay: f32, az: f32) -> Result<()> {
        jvm.put_field(this, "orientationAngle", "F", angle).await?;
        jvm.put_field(this, "orientationX", "F", ax).await?;
        jvm.put_field(this, "orientationY", "F", ay).await?;
        jvm.put_field(this, "orientationZ", "F", az).await
    }

    async fn generic_matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let matrix: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "transform", "[F").await?;
        let values = raw_f32_array(jvm, &matrix, 16).await?;
        Ok(matrix_to_array(&values))
    }

    async fn put_generic_matrix(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, matrix: [f32; 16]) -> Result<()> {
        let mut array = jvm.instantiate_array("F", 16).await?;
        store_raw_f32_array(jvm, &mut array, &matrix).await?;
        jvm.put_field(this, "transform", "[F", array).await
    }

    async fn orientation_matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let angle: f32 = jvm.get_field(this, "orientationAngle", "F").await?;
        let ax: f32 = jvm.get_field(this, "orientationX", "F").await?;
        let ay: f32 = jvm.get_field(this, "orientationY", "F").await?;
        let az: f32 = jvm.get_field(this, "orientationZ", "F").await?;
        Ok(rotation_matrix(angle, ax, ay, az))
    }

    async fn local_matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let tx: f32 = jvm.get_field(this, "translationX", "F").await?;
        let ty: f32 = jvm.get_field(this, "translationY", "F").await?;
        let tz: f32 = jvm.get_field(this, "translationZ", "F").await?;
        let sx: f32 = jvm.get_field(this, "scaleX", "F").await?;
        let sy: f32 = jvm.get_field(this, "scaleY", "F").await?;
        let sz: f32 = jvm.get_field(this, "scaleZ", "F").await?;
        let orientation = Self::orientation_matrix(jvm, this).await?;
        let generic = Self::generic_matrix(jvm, this).await?;

        Ok(multiply_matrix(
            multiply_matrix(multiply_matrix(translation_matrix(tx, ty, tz), orientation), scale_matrix(sx, sy, sz)),
            generic,
        ))
    }
}

impl Node {
    const NONE: i32 = 144;
    const ORIGIN: i32 = 145;
    const X_AXIS: i32 = 146;
    const Y_AXIS: i32 = 147;
    const Z_AXIS: i32 = 148;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Node",
            parent_class: Some("javax/microedition/m3g/Transformable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("align", "(Ljavax/microedition/m3g/Node;)V", Self::align, Default::default()),
                JavaMethodProto::new(
                    "getAlignmentReference",
                    "(I)Ljavax/microedition/m3g/Node;",
                    Self::get_alignment_reference,
                    Default::default(),
                ),
                JavaMethodProto::new("getAlignmentTarget", "(I)I", Self::get_alignment_target, Default::default()),
                JavaMethodProto::new("getAlphaFactor", "()F", Self::get_alpha_factor, Default::default()),
                JavaMethodProto::new("getParent", "()Ljavax/microedition/m3g/Node;", Self::get_parent, Default::default()),
                JavaMethodProto::new("getScope", "()I", Self::get_scope, Default::default()),
                JavaMethodProto::new(
                    "getTransformTo",
                    "(Ljavax/microedition/m3g/Node;Ljavax/microedition/m3g/Transform;)Z",
                    Self::get_transform_to,
                    Default::default(),
                ),
                JavaMethodProto::new("isPickingEnabled", "()Z", Self::is_picking_enabled, Default::default()),
                JavaMethodProto::new("isRenderingEnabled", "()Z", Self::is_rendering_enabled, Default::default()),
                JavaMethodProto::new(
                    "setAlignment",
                    "(Ljavax/microedition/m3g/Node;ILjavax/microedition/m3g/Node;I)V",
                    Self::set_alignment,
                    Default::default(),
                ),
                JavaMethodProto::new("setAlphaFactor", "(F)V", Self::set_alpha_factor, Default::default()),
                JavaMethodProto::new("setPickingEnable", "(Z)V", Self::set_picking_enable, Default::default()),
                JavaMethodProto::new("setRenderingEnable", "(Z)V", Self::set_rendering_enable, Default::default()),
                JavaMethodProto::new("setScope", "(I)V", Self::set_scope, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("parent", "Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("renderingEnabled", "Z", Default::default()),
                JavaFieldProto::new("pickingEnabled", "Z", Default::default()),
                JavaFieldProto::new("alphaFactor", "F", Default::default()),
                JavaFieldProto::new("scope", "I", Default::default()),
                JavaFieldProto::new("zReference", "Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("yReference", "Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("zTarget", "I", Default::default()),
                JavaFieldProto::new("yTarget", "I", Default::default()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/m3g/Transformable", "<init>", "()V", ())
            .await?;
        jvm.put_field(&mut this, "parent", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(&mut this, "renderingEnabled", "Z", true).await?;
        jvm.put_field(&mut this, "pickingEnabled", "Z", true).await?;
        jvm.put_field(&mut this, "alphaFactor", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "scope", "I", -1).await?;
        jvm.put_field(&mut this, "zReference", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(&mut this, "yReference", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(&mut this, "zTarget", "I", Self::NONE).await?;
        jvm.put_field(&mut this, "yTarget", "I", Self::NONE).await
    }

    async fn align(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, _reference: ClassInstanceRef<Self>) -> Result<()> {
        let z_target = jvm.get_field::<i32>(&this, "zTarget", "I").await.unwrap_or(Self::NONE);
        let y_target = jvm.get_field::<i32>(&this, "yTarget", "I").await.unwrap_or(Self::NONE);
        if z_target == Self::NONE && y_target == Self::NONE {
            return Ok(());
        }
        // Full M3G alignment changes orientation from target axes and references.
        // Keeping stored alignment state lets games query it; transform solving is handled by getTransformTo.
        Ok(())
    }

    async fn get_alignment_reference(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, axis: i32) -> Result<ClassInstanceRef<Self>> {
        match axis {
            Self::Y_AXIS => jvm.get_field(&this, "yReference", "Ljavax/microedition/m3g/Node;").await,
            Self::Z_AXIS => jvm.get_field(&this, "zReference", "Ljavax/microedition/m3g/Node;").await,
            _ => Err(jvm.exception("java/lang/IllegalArgumentException", "alignment axis").await),
        }
    }

    async fn get_alignment_target(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, axis: i32) -> Result<i32> {
        match axis {
            Self::Y_AXIS => jvm.get_field(&this, "yTarget", "I").await,
            Self::Z_AXIS => jvm.get_field(&this, "zTarget", "I").await,
            _ => Err(jvm.exception("java/lang/IllegalArgumentException", "alignment axis").await),
        }
    }

    async fn get_alpha_factor(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "alphaFactor", "F").await
    }

    async fn get_parent(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        jvm.get_field(&this, "parent", "Ljavax/microedition/m3g/Node;").await
    }

    async fn get_scope(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "scope", "I").await
    }

    async fn get_transform_to(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Self>,
        mut transform: ClassInstanceRef<Transform>,
    ) -> Result<bool> {
        if target.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Node.getTransformTo target").await);
        }
        let (source_root, source_world) = Self::root_and_world_matrix(jvm, &this).await?;
        let (target_root, target_world) = Self::root_and_world_matrix(jvm, &target).await?;
        if !same_instance(&source_root, &target_root) {
            return Ok(false);
        }
        let Some(target_inverse) = invert_matrix(target_world) else {
            return Ok(false);
        };
        if !transform.is_null() {
            Transform::put_matrix(jvm, &mut transform, multiply_matrix(target_inverse, source_world)).await?;
        }
        Ok(true)
    }

    async fn is_picking_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "pickingEnabled", "Z").await
    }

    async fn is_rendering_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "renderingEnabled", "Z").await
    }

    async fn set_alignment(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        z_reference: ClassInstanceRef<Self>,
        z_target: i32,
        y_reference: ClassInstanceRef<Self>,
        y_target: i32,
    ) -> Result<()> {
        Self::validate_alignment_target(jvm, z_target).await?;
        Self::validate_alignment_target(jvm, y_target).await?;
        jvm.put_field(&mut this, "zReference", "Ljavax/microedition/m3g/Node;", z_reference)
            .await?;
        jvm.put_field(&mut this, "yReference", "Ljavax/microedition/m3g/Node;", y_reference)
            .await?;
        jvm.put_field(&mut this, "zTarget", "I", z_target).await?;
        jvm.put_field(&mut this, "yTarget", "I", y_target).await
    }

    async fn set_alpha_factor(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, alpha_factor: f32) -> Result<()> {
        jvm.put_field(&mut this, "alphaFactor", "F", alpha_factor.clamp(0.0, 1.0)).await
    }

    async fn set_picking_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, enabled: bool) -> Result<()> {
        jvm.put_field(&mut this, "pickingEnabled", "Z", enabled).await
    }

    async fn set_rendering_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, enabled: bool) -> Result<()> {
        let user_id = jvm.get_field::<i32>(&cast_ref::<Node, Object3D>(&this), "userID", "I").await.unwrap_or(0);
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.Node.setRenderingEnable class={} userID={} enabled={}",
            this.class_definition().name(),
            user_id,
            enabled
        );
        jvm.put_field(&mut this, "renderingEnabled", "Z", enabled).await
    }

    async fn set_scope(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, scope: i32) -> Result<()> {
        jvm.put_field(&mut this, "scope", "I", scope).await
    }

    async fn validate_alignment_target(jvm: &Jvm, target: i32) -> Result<()> {
        match target {
            Self::NONE | Self::ORIGIN | Self::X_AXIS | Self::Y_AXIS | Self::Z_AXIS => Ok(()),
            _ => Err(jvm.exception("java/lang/IllegalArgumentException", "alignment target").await),
        }
    }

    async fn root_and_world_matrix(jvm: &Jvm, node: &ClassInstanceRef<Self>) -> Result<(ClassInstanceRef<Self>, [f32; 16])> {
        let mut chain = Vec::new();
        let mut current = node.clone();
        while !current.is_null() {
            chain.push(current.clone());
            current = jvm
                .get_field(&current, "parent", "Ljavax/microedition/m3g/Node;")
                .await
                .unwrap_or_else(|_| null_ref());
        }
        let root = chain.last().cloned().unwrap_or_else(null_ref);
        let mut world = identity_matrix();
        for node in chain.into_iter().rev() {
            let local = Transformable::local_matrix(jvm, &cast_ref::<Node, Transformable>(&node))
                .await
                .unwrap_or_else(|_| identity_matrix());
            world = multiply_matrix(world, local);
        }
        Ok((root, world))
    }
}

impl Group {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Group",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("addChild", "(Ljavax/microedition/m3g/Node;)V", Self::add_child, Default::default()),
                JavaMethodProto::new("getChild", "(I)Ljavax/microedition/m3g/Node;", Self::get_child, Default::default()),
                JavaMethodProto::new("getChildCount", "()I", Self::get_child_count, Default::default()),
                JavaMethodProto::new(
                    "pick",
                    "(IFFFFFFLjavax/microedition/m3g/RayIntersection;)Z",
                    Self::pick_3d,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "pick",
                    "(IFFLjavax/microedition/m3g/Camera;Ljavax/microedition/m3g/RayIntersection;)Z",
                    Self::pick_2d,
                    Default::default(),
                ),
                JavaMethodProto::new("removeChild", "(Ljavax/microedition/m3g/Node;)V", Self::remove_child, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("children", "[Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("childCount", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await?;
        let children = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", 0).await?;
        jvm.put_field(&mut this, "children", "[Ljavax/microedition/m3g/Node;", children).await?;
        jvm.put_field(&mut this, "childCount", "I", 0).await
    }

    async fn add_child(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, child: ClassInstanceRef<Node>) -> Result<()> {
        let mut group: ClassInstanceRef<Group> = ClassInstanceRef::new(this.instance.clone());
        Self::push_child(jvm, &mut group, child).await
    }

    async fn get_child(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Node>> {
        let count: i32 = jvm.get_field(&this, "childCount", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "M3G child index").await);
        }
        let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm.get_field(&this, "children", "[Ljavax/microedition/m3g/Node;").await?;
        Ok(jvm
            .load_array(&children, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or_else(null_ref))
    }

    async fn get_child_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "childCount", "I").await
    }

    async fn pick_3d(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mask: i32,
        ox: f32,
        oy: f32,
        oz: f32,
        dx: f32,
        dy: f32,
        dz: f32,
        ri: ClassInstanceRef<RayIntersection>,
    ) -> Result<bool> {
        let ray = [ox, oy, oz, dx, dy, dz];
        let Some(hit) = Self::pick_ray(jvm, cast_ref::<Group, Node>(&this), mask, [ox, oy, oz], [dx, dy, dz]).await? else {
            return Ok(false);
        };
        if !ri.is_null() {
            let mut ri = ri;
            RayIntersection::fill(jvm, &mut ri, hit.node.clone(), &hit, ray).await?;
        }
        Ok(true)
    }

    async fn pick_2d(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mask: i32,
        x: f32,
        y: f32,
        camera: ClassInstanceRef<Camera>,
        ri: ClassInstanceRef<RayIntersection>,
    ) -> Result<bool> {
        if camera.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Group.pick camera").await);
        }
        let fovy = jvm.get_field::<f32>(&camera, "fovy", "F").await.unwrap_or(45.0);
        let aspect = jvm.get_field::<f32>(&camera, "aspect", "F").await.unwrap_or(1.0).max(0.001);
        let half_y = (fovy * core::f32::consts::PI / 360.0).tan();
        let direction = normalize3([x * half_y * aspect, y * half_y, 1.0]).unwrap_or([0.0, 0.0, 1.0]);
        let ray = [0.0, 0.0, 0.0, direction[0], direction[1], direction[2]];
        let Some(hit) = Self::pick_ray(jvm, cast_ref::<Group, Node>(&this), mask, [0.0, 0.0, 0.0], direction).await? else {
            return Ok(false);
        };
        if !ri.is_null() {
            let mut ri = ri;
            RayIntersection::fill(jvm, &mut ri, hit.node.clone(), &hit, ray).await?;
        }
        Ok(true)
    }

    async fn remove_child(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, child: ClassInstanceRef<Node>) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "childCount", "I").await?;
        let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm.get_field(&this, "children", "[Ljavax/microedition/m3g/Node;").await?;
        let mut values: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, count.max(0) as usize).await?;
        let removed = values.iter().any(|candidate| same_instance(candidate, &child));
        values.retain(|candidate| !same_instance(candidate, &child));
        let mut new_array = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", values.len()).await?;
        jvm.store_array(&mut new_array, 0, values.clone()).await?;
        jvm.put_field(&mut this, "children", "[Ljavax/microedition/m3g/Node;", new_array).await?;
        jvm.put_field(&mut this, "childCount", "I", values.len() as i32).await?;
        if removed {
            let mut child = child;
            let user_id = jvm
                .get_field::<i32>(&cast_ref::<Node, Object3D>(&child), "userID", "I")
                .await
                .unwrap_or(0);
            tracing::debug!(
                target: "rustjava_m3g",
                "m3g.Group.removeChild group={} child={} childUserID={}",
                this.class_definition().name(),
                child.class_definition().name(),
                user_id
            );
            jvm.put_field(&mut child, "parent", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
                .await?;
        }
        Ok(())
    }

    async fn push_child(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, mut child: ClassInstanceRef<Node>) -> Result<()> {
        if child.is_null() {
            return Ok(());
        }
        let count: i32 = jvm.get_field(this, "childCount", "I").await?;
        let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm.get_field(this, "children", "[Ljavax/microedition/m3g/Node;").await?;
        let mut values: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, count.max(0) as usize).await?;
        values.push(child.clone());
        let mut new_array = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", values.len()).await?;
        jvm.store_array(&mut new_array, 0, values).await?;
        jvm.put_field(this, "children", "[Ljavax/microedition/m3g/Node;", new_array).await?;
        jvm.put_field(this, "childCount", "I", count + 1).await?;
        jvm.put_field(&mut child, "parent", "Ljavax/microedition/m3g/Node;", cast_ref::<Group, Node>(this))
            .await
    }

    async fn pick_ray(jvm: &Jvm, root: ClassInstanceRef<Node>, mask: i32, origin: [f32; 3], direction: [f32; 3]) -> Result<Option<M3gPickHit>> {
        let Some(direction) = normalize3(direction) else {
            return Ok(None);
        };
        let mut best: Option<M3gPickHit> = None;
        let mut stack = vec![(root, identity_matrix(), false, true)];
        while let Some((node, parent_matrix, apply_local_transform, parent_picking_enabled)) = stack.pop() {
            if node.is_null() {
                continue;
            }
            let picking_enabled = jvm.get_field::<bool>(&node, "pickingEnabled", "Z").await.unwrap_or(true);
            let subtree_picking_enabled = parent_picking_enabled && picking_enabled;
            let world_matrix = if apply_local_transform {
                let local_matrix = Transformable::local_matrix(jvm, &cast_ref::<Node, Transformable>(&node))
                    .await
                    .unwrap_or_else(|_| identity_matrix());
                multiply_matrix(parent_matrix, local_matrix)
            } else {
                parent_matrix
            };

            let class_name = node.class_definition().name().to_string();
            if subtree_picking_enabled && class_name == "javax/microedition/m3g/Mesh" {
                let scope = jvm.get_field::<i32>(&node, "scope", "I").await.unwrap_or(-1);
                if (scope & mask) != 0 {
                    if let Some(hit) = Self::pick_mesh(jvm, cast_ref::<Node, Mesh>(&node), node.clone(), world_matrix, origin, direction).await? {
                        if best.as_ref().is_none_or(|best| hit.distance < best.distance) {
                            best = Some(hit);
                        }
                    }
                }
            }

            if subtree_picking_enabled && (class_name == "javax/microedition/m3g/Group" || class_name == "javax/microedition/m3g/World") {
                let group: ClassInstanceRef<Group> = cast_ref(&node);
                let child_count: i32 = jvm.get_field(&group, "childCount", "I").await.unwrap_or(0);
                if child_count > 0 {
                    let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> =
                        jvm.get_field(&group, "children", "[Ljavax/microedition/m3g/Node;").await?;
                    let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
                    for child in children.into_iter().rev() {
                        stack.push((child, world_matrix, true, subtree_picking_enabled));
                    }
                }
            }
        }
        Ok(best)
    }

    async fn pick_mesh(
        jvm: &Jvm,
        mesh: ClassInstanceRef<Mesh>,
        node: ClassInstanceRef<Node>,
        world_matrix: [f32; 16],
        origin: [f32; 3],
        direction: [f32; 3],
    ) -> Result<Option<M3gPickHit>> {
        let vertex_buffer: ClassInstanceRef<VertexBuffer> = jvm.get_field(&mesh, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;").await?;
        if vertex_buffer.is_null() {
            return Ok(None);
        }
        let positions_ref: ClassInstanceRef<VertexArray> = jvm.get_field(&vertex_buffer, "positions", "Ljavax/microedition/m3g/VertexArray;").await?;
        let position_scale = jvm.get_field::<f32>(&vertex_buffer, "positionScale", "F").await.unwrap_or(1.0);
        let position_bias = Graphics3D::float_array_field(jvm, &vertex_buffer, "positionBias", 3, 0.0).await?;
        let positions = Graphics3D::vertex_array_vec3(jvm, &positions_ref, position_scale, &position_bias).await?;
        if positions.is_empty() {
            return Ok(None);
        }
        let world_positions: Vec<[f32; 3]> = positions
            .iter()
            .copied()
            .map(|position| transform_point(world_matrix, position))
            .collect();

        let tex_coords_ref: ClassInstanceRef<VertexArray> = jvm
            .get_field(&vertex_buffer, "texCoords0", "Ljavax/microedition/m3g/VertexArray;")
            .await?;
        let tex_scale = jvm.get_field::<f32>(&vertex_buffer, "texScale", "F").await.unwrap_or(1.0);
        let tex_bias = Graphics3D::float_array_field(jvm, &vertex_buffer, "texBias", 3, 0.0).await?;
        let tex_coords = Graphics3D::vertex_array_vec2(jvm, &tex_coords_ref, tex_scale, &tex_bias).await?;

        let submesh_count: i32 = jvm.get_field(&mesh, "submeshCount", "I").await.unwrap_or(0);
        if submesh_count <= 0 {
            return Ok(None);
        }
        let index_buffers: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> =
            jvm.get_field(&mesh, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;").await?;
        let index_buffers: Vec<ClassInstanceRef<IndexBuffer>> = jvm.load_array(&index_buffers, 0, submesh_count as usize).await?;
        let mut best: Option<M3gPickHit> = None;
        for (submesh, index_buffer) in index_buffers.into_iter().enumerate() {
            if index_buffer.is_null() {
                continue;
            }
            let triangles = Graphics3D::triangle_indices(jvm, cast_ref::<IndexBuffer, TriangleStripArray>(&index_buffer)).await?;
            for [i0, i1, i2] in triangles.iter().copied() {
                let (Some(p0), Some(p1), Some(p2)) = (
                    world_positions.get(i0).copied(),
                    world_positions.get(i1).copied(),
                    world_positions.get(i2).copied(),
                ) else {
                    continue;
                };
                let Some(triangle_hit) = ray_triangle_intersection(origin, direction, p0, p1, p2) else {
                    continue;
                };
                let uv0 = tex_coords.get(i0).copied().unwrap_or([0.0, 0.0]);
                let uv1 = tex_coords.get(i1).copied().unwrap_or(uv0);
                let uv2 = tex_coords.get(i2).copied().unwrap_or(uv0);
                let w = 1.0 - triangle_hit.u - triangle_hit.v;
                let texture_s = uv0[0] * w + uv1[0] * triangle_hit.u + uv2[0] * triangle_hit.v;
                let texture_t = uv0[1] * w + uv1[1] * triangle_hit.u + uv2[1] * triangle_hit.v;
                let hit = M3gPickHit {
                    node: node.clone(),
                    distance: triangle_hit.distance,
                    submesh_index: submesh as i32,
                    texture_s,
                    texture_t,
                    normal: triangle_hit.normal,
                };
                if best.as_ref().is_none_or(|best| hit.distance < best.distance) {
                    best = Some(hit);
                }
            }
        }
        Ok(best)
    }
}

impl RayIntersection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/RayIntersection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getDistance", "()F", Self::get_distance, Default::default()),
                JavaMethodProto::new(
                    "getIntersected",
                    "()Ljavax/microedition/m3g/Node;",
                    Self::get_intersected,
                    Default::default(),
                ),
                JavaMethodProto::new("getNormalX", "()F", Self::get_normal_x, Default::default()),
                JavaMethodProto::new("getNormalY", "()F", Self::get_normal_y, Default::default()),
                JavaMethodProto::new("getNormalZ", "()F", Self::get_normal_z, Default::default()),
                JavaMethodProto::new("getRay", "([F)V", Self::get_ray, Default::default()),
                JavaMethodProto::new("getSubmeshIndex", "()I", Self::get_submesh_index, Default::default()),
                JavaMethodProto::new("getTextureS", "(I)F", Self::get_texture_s, Default::default()),
                JavaMethodProto::new("getTextureT", "(I)F", Self::get_texture_t, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("intersected", "Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("distance", "F", Default::default()),
                JavaFieldProto::new("submeshIndex", "I", Default::default()),
                JavaFieldProto::new("textureS", "[F", Default::default()),
                JavaFieldProto::new("textureT", "[F", Default::default()),
                JavaFieldProto::new("normal", "[F", Default::default()),
                JavaFieldProto::new("ray", "[F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "intersected", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(&mut this, "distance", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "submeshIndex", "I", 0).await?;
        let texture_s = jvm.instantiate_array("F", 2).await?;
        let texture_t = jvm.instantiate_array("F", 2).await?;
        let mut normal = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut normal, 0, vec![0.0f32, 0.0, 1.0]).await?;
        let mut ray = jvm.instantiate_array("F", 6).await?;
        jvm.store_array(&mut ray, 0, vec![0.0f32, 0.0, 0.0, 0.0, 0.0, 1.0]).await?;
        jvm.put_field(&mut this, "textureS", "[F", texture_s).await?;
        jvm.put_field(&mut this, "textureT", "[F", texture_t).await?;
        jvm.put_field(&mut this, "normal", "[F", normal).await?;
        jvm.put_field(&mut this, "ray", "[F", ray).await
    }

    async fn get_distance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "distance", "F").await
    }

    async fn get_intersected(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Node>> {
        jvm.get_field(&this, "intersected", "Ljavax/microedition/m3g/Node;").await
    }

    async fn get_normal_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Self::array_value(jvm, &this, "normal", 0).await
    }

    async fn get_normal_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Self::array_value(jvm, &this, "normal", 1).await
    }

    async fn get_normal_z(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Self::array_value(jvm, &this, "normal", 2).await
    }

    async fn get_ray(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut out: ClassInstanceRef<Array<f32>>) -> Result<()> {
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "RayIntersection.getRay").await);
        }
        if jvm.array_length(&out).await? < 6 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "ray array too small").await);
        }
        let ray: ClassInstanceRef<Array<f32>> = jvm.get_field(&this, "ray", "[F").await?;
        let values: Vec<f32> = jvm.load_array(&ray, 0, 6).await?;
        jvm.store_array(&mut out, 0, values).await
    }

    async fn get_submesh_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "submeshIndex", "I").await
    }

    async fn get_texture_s(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<f32> {
        Self::texture_value(jvm, &this, "textureS", index).await
    }

    async fn get_texture_t(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<f32> {
        Self::texture_value(jvm, &this, "textureT", index).await
    }

    async fn fill(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, intersected: ClassInstanceRef<Node>, hit: &M3gPickHit, ray: [f32; 6]) -> Result<()> {
        jvm.put_field(this, "intersected", "Ljavax/microedition/m3g/Node;", intersected).await?;
        jvm.put_field(this, "distance", "F", hit.distance).await?;
        jvm.put_field(this, "submeshIndex", "I", hit.submesh_index).await?;
        let mut texture_s = jvm.instantiate_array("F", 2).await?;
        jvm.store_array(&mut texture_s, 0, vec![hit.texture_s, 0.0]).await?;
        let mut texture_t = jvm.instantiate_array("F", 2).await?;
        jvm.store_array(&mut texture_t, 0, vec![hit.texture_t, 0.0]).await?;
        let mut normal = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut normal, 0, hit.normal.to_vec()).await?;
        let mut ray_array = jvm.instantiate_array("F", 6).await?;
        jvm.store_array(&mut ray_array, 0, ray).await?;
        jvm.put_field(this, "textureS", "[F", texture_s).await?;
        jvm.put_field(this, "textureT", "[F", texture_t).await?;
        jvm.put_field(this, "normal", "[F", normal).await?;
        jvm.put_field(this, "ray", "[F", ray_array).await
    }

    async fn texture_value(jvm: &Jvm, this: &ClassInstanceRef<Self>, field: &str, index: i32) -> Result<f32> {
        if !(0..2).contains(&index) {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "texture unit").await);
        }
        Self::array_value(jvm, this, field, index as usize).await
    }

    async fn array_value(jvm: &Jvm, this: &ClassInstanceRef<Self>, field: &str, index: usize) -> Result<f32> {
        let array: ClassInstanceRef<Array<f32>> = jvm.get_field(this, field, "[F").await?;
        Ok(jvm.load_array(&array, index, 1).await?.into_iter().next().unwrap_or(0.0))
    }
}

impl World {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/World",
            parent_class: Some("javax/microedition/m3g/Group"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getActiveCamera",
                    "()Ljavax/microedition/m3g/Camera;",
                    Self::get_active_camera,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getBackground",
                    "()Ljavax/microedition/m3g/Background;",
                    Self::get_background,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setActiveCamera",
                    "(Ljavax/microedition/m3g/Camera;)V",
                    Self::set_active_camera,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setBackground",
                    "(Ljavax/microedition/m3g/Background;)V",
                    Self::set_background,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("activeCamera", "Ljavax/microedition/m3g/Camera;", Default::default()),
                JavaFieldProto::new("background", "Ljavax/microedition/m3g/Background;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Group", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "activeCamera", "Ljavax/microedition/m3g/Camera;", null_ref::<Camera>())
            .await?;
        jvm.put_field(&mut this, "background", "Ljavax/microedition/m3g/Background;", null_ref::<Background>())
            .await
    }

    async fn get_active_camera(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Camera>> {
        jvm.get_field(&this, "activeCamera", "Ljavax/microedition/m3g/Camera;").await
    }

    async fn get_background(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Background>> {
        jvm.get_field(&this, "background", "Ljavax/microedition/m3g/Background;").await
    }

    async fn set_active_camera(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, camera: ClassInstanceRef<Camera>) -> Result<()> {
        jvm.put_field(&mut this, "activeCamera", "Ljavax/microedition/m3g/Camera;", camera).await
    }

    async fn set_background(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        background: ClassInstanceRef<Background>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "background", "Ljavax/microedition/m3g/Background;", background)
            .await
    }
}

impl Camera {
    const GENERIC: i32 = 48;
    const PARALLEL: i32 = 49;
    const PERSPECTIVE: i32 = 50;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Camera",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getProjection",
                    "(Ljavax/microedition/m3g/Transform;)I",
                    Self::get_projection_transform,
                    Default::default(),
                ),
                JavaMethodProto::new("getProjection", "([F)I", Self::get_projection_params, Default::default()),
                JavaMethodProto::new(
                    "setGeneric",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::set_generic,
                    Default::default(),
                ),
                JavaMethodProto::new("setParallel", "(FFFF)V", Self::set_parallel, Default::default()),
                JavaMethodProto::new("setPerspective", "(FFFF)V", Self::set_perspective, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("projectionMode", "I", Default::default()),
                JavaFieldProto::new("fovy", "F", Default::default()),
                JavaFieldProto::new("parallelHeight", "F", Default::default()),
                JavaFieldProto::new("aspect", "F", Default::default()),
                JavaFieldProto::new("near", "F", Default::default()),
                JavaFieldProto::new("far", "F", Default::default()),
                JavaFieldProto::new("genericProjection", "[F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "projectionMode", "I", Self::PERSPECTIVE).await?;
        jvm.put_field(&mut this, "fovy", "F", 45.0f32).await?;
        jvm.put_field(&mut this, "parallelHeight", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "aspect", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "near", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "far", "F", 1000.0f32).await?;
        Self::put_generic_projection(jvm, &mut this, identity_matrix()).await
    }

    async fn set_generic(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, transform: ClassInstanceRef<Transform>) -> Result<()> {
        if transform.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Camera.setGeneric").await);
        }
        let matrix = Transform::matrix(jvm, &transform).await?;
        jvm.put_field(&mut this, "projectionMode", "I", Self::GENERIC).await?;
        Self::put_generic_projection(jvm, &mut this, matrix).await
    }

    async fn set_parallel(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        height: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Result<()> {
        if height <= 0.0 || aspect <= 0.0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid parallel projection").await);
        }
        jvm.put_field(&mut this, "projectionMode", "I", Self::PARALLEL).await?;
        jvm.put_field(&mut this, "parallelHeight", "F", height).await?;
        jvm.put_field(&mut this, "fovy", "F", height).await?;
        jvm.put_field(&mut this, "aspect", "F", aspect).await?;
        jvm.put_field(&mut this, "near", "F", near).await?;
        jvm.put_field(&mut this, "far", "F", far).await
    }

    async fn set_perspective(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        fovy: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Result<()> {
        if fovy <= 0.0 || fovy >= 180.0 || aspect <= 0.0 || near <= 0.0 || far <= 0.0 {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "invalid perspective projection")
                .await);
        }
        jvm.put_field(&mut this, "projectionMode", "I", Self::PERSPECTIVE).await?;
        jvm.put_field(&mut this, "fovy", "F", fovy).await?;
        jvm.put_field(&mut this, "aspect", "F", aspect).await?;
        jvm.put_field(&mut this, "near", "F", near).await?;
        jvm.put_field(&mut this, "far", "F", far).await
    }

    async fn get_projection_params(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut params: ClassInstanceRef<Array<f32>>,
    ) -> Result<i32> {
        let mode = Self::projection_mode(jvm, &this).await?;
        if !params.is_null() && mode != Self::GENERIC {
            if jvm.array_length(&params).await? < 4 {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "projection params array too small")
                    .await);
            }
            let first = if mode == Self::PARALLEL {
                jvm.get_field::<f32>(&this, "parallelHeight", "F").await.unwrap_or(1.0)
            } else {
                jvm.get_field::<f32>(&this, "fovy", "F").await.unwrap_or(45.0)
            };
            let values = vec![
                first,
                jvm.get_field::<f32>(&this, "aspect", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(&this, "near", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(&this, "far", "F").await.unwrap_or(1000.0),
            ];
            jvm.store_array(&mut params, 0, values).await?;
        }
        Ok(mode)
    }

    async fn get_projection_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut transform: ClassInstanceRef<Transform>,
    ) -> Result<i32> {
        let mode = Self::projection_mode(jvm, &this).await?;
        if !transform.is_null() {
            let matrix = Self::projection_matrix(jvm, &this, mode).await?;
            Transform::put_matrix(jvm, &mut transform, matrix).await?;
        }
        Ok(mode)
    }

    async fn projection_mode(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field(this, "projectionMode", "I").await.unwrap_or(Self::PERSPECTIVE))
    }

    async fn generic_projection(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let matrix: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "genericProjection", "[F").await?;
        let values: Vec<f32> = jvm.load_array(&matrix, 0, 16).await?;
        Ok(matrix_to_array(&values))
    }

    async fn put_generic_projection(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, matrix: [f32; 16]) -> Result<()> {
        let mut array = jvm.instantiate_array("F", 16).await?;
        jvm.store_array(&mut array, 0, matrix).await?;
        jvm.put_field(this, "genericProjection", "[F", array).await
    }

    async fn projection_matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>, mode: i32) -> Result<[f32; 16]> {
        match mode {
            Self::GENERIC => Self::generic_projection(jvm, this).await,
            Self::PARALLEL => Ok(parallel_projection_matrix(
                jvm.get_field::<f32>(this, "parallelHeight", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "aspect", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "near", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "far", "F").await.unwrap_or(1000.0),
            )),
            _ => Ok(perspective_projection_matrix(
                jvm.get_field::<f32>(this, "fovy", "F").await.unwrap_or(45.0),
                jvm.get_field::<f32>(this, "aspect", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "near", "F").await.unwrap_or(1.0),
                jvm.get_field::<f32>(this, "far", "F").await.unwrap_or(1000.0),
            )),
        }
    }
}

impl Background {
    const BORDER: i32 = 32;
    const REPEAT: i32 = 33;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Background",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getColor", "()I", Self::get_color, Default::default()),
                JavaMethodProto::new("getCropHeight", "()I", Self::get_crop_height, Default::default()),
                JavaMethodProto::new("getCropWidth", "()I", Self::get_crop_width, Default::default()),
                JavaMethodProto::new("getCropX", "()I", Self::get_crop_x, Default::default()),
                JavaMethodProto::new("getCropY", "()I", Self::get_crop_y, Default::default()),
                JavaMethodProto::new("getImage", "()Ljavax/microedition/m3g/Image2D;", Self::get_image, Default::default()),
                JavaMethodProto::new("getImageModeX", "()I", Self::get_image_mode_x, Default::default()),
                JavaMethodProto::new("getImageModeY", "()I", Self::get_image_mode_y, Default::default()),
                JavaMethodProto::new("isColorClearEnabled", "()Z", Self::is_color_clear_enabled, Default::default()),
                JavaMethodProto::new("isDepthClearEnabled", "()Z", Self::is_depth_clear_enabled, Default::default()),
                JavaMethodProto::new("setColor", "(I)V", Self::set_color, Default::default()),
                JavaMethodProto::new("setColorClearEnable", "(Z)V", Self::set_color_clear_enable, Default::default()),
                JavaMethodProto::new("setCrop", "(IIII)V", Self::set_crop, Default::default()),
                JavaMethodProto::new("setDepthClearEnable", "(Z)V", Self::set_depth_clear_enable, Default::default()),
                JavaMethodProto::new("setImage", "(Ljavax/microedition/m3g/Image2D;)V", Self::set_image, Default::default()),
                JavaMethodProto::new("setImageMode", "(II)V", Self::set_image_mode, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("color", "I", Default::default()),
                JavaFieldProto::new("image", "Ljavax/microedition/m3g/Image2D;", Default::default()),
                JavaFieldProto::new("colorClear", "Z", Default::default()),
                JavaFieldProto::new("depthClear", "Z", Default::default()),
                JavaFieldProto::new("imageModeX", "I", Default::default()),
                JavaFieldProto::new("imageModeY", "I", Default::default()),
                JavaFieldProto::new("cropX", "I", Default::default()),
                JavaFieldProto::new("cropY", "I", Default::default()),
                JavaFieldProto::new("cropW", "I", Default::default()),
                JavaFieldProto::new("cropH", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "color", "I", 0xff000000u32 as i32).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", null_ref::<Image2D>())
            .await?;
        jvm.put_field(&mut this, "colorClear", "Z", true).await?;
        jvm.put_field(&mut this, "depthClear", "Z", true).await?;
        jvm.put_field(&mut this, "imageModeX", "I", Self::BORDER).await?;
        jvm.put_field(&mut this, "imageModeY", "I", Self::BORDER).await?;
        jvm.put_field(&mut this, "cropX", "I", 0).await?;
        jvm.put_field(&mut this, "cropY", "I", 0).await?;
        jvm.put_field(&mut this, "cropW", "I", 0).await?;
        jvm.put_field(&mut this, "cropH", "I", 0).await
    }

    async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "color", "I").await
    }

    async fn get_crop_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropH", "I").await
    }

    async fn get_crop_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropW", "I").await
    }

    async fn get_crop_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropX", "I").await
    }

    async fn get_crop_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropY", "I").await
    }

    async fn get_image(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Image2D>> {
        jvm.get_field(&this, "image", "Ljavax/microedition/m3g/Image2D;").await
    }

    async fn get_image_mode_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "imageModeX", "I").await
    }

    async fn get_image_mode_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "imageModeY", "I").await
    }

    async fn is_color_clear_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "colorClear", "Z").await
    }

    async fn is_depth_clear_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "depthClear", "Z").await
    }

    async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, color: i32) -> Result<()> {
        jvm.put_field(&mut this, "color", "I", color).await
    }

    async fn set_color_clear_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, enabled: bool) -> Result<()> {
        jvm.put_field(&mut this, "colorClear", "Z", enabled).await
    }

    async fn set_crop(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        crop_x: i32,
        crop_y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        if width < 0 || height < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "negative background crop").await);
        }
        jvm.put_field(&mut this, "cropX", "I", crop_x).await?;
        jvm.put_field(&mut this, "cropY", "I", crop_y).await?;
        jvm.put_field(&mut this, "cropW", "I", width).await?;
        jvm.put_field(&mut this, "cropH", "I", height).await
    }

    async fn set_depth_clear_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, enabled: bool) -> Result<()> {
        jvm.put_field(&mut this, "depthClear", "Z", enabled).await
    }

    async fn set_image(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, image: ClassInstanceRef<Image2D>) -> Result<()> {
        let crop = if image.is_null() {
            (0, 0)
        } else {
            (
                jvm.get_field(&image, "width", "I").await.unwrap_or(0),
                jvm.get_field(&image, "height", "I").await.unwrap_or(0),
            )
        };
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
        jvm.put_field(&mut this, "cropX", "I", 0).await?;
        jvm.put_field(&mut this, "cropY", "I", 0).await?;
        jvm.put_field(&mut this, "cropW", "I", crop.0).await?;
        jvm.put_field(&mut this, "cropH", "I", crop.1).await
    }

    async fn set_image_mode(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, mode_x: i32, mode_y: i32) -> Result<()> {
        if !matches!(mode_x, Self::BORDER | Self::REPEAT) || !matches!(mode_y, Self::BORDER | Self::REPEAT) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "bad background image mode").await);
        }
        jvm.put_field(&mut this, "imageModeX", "I", mode_x).await?;
        jvm.put_field(&mut this, "imageModeY", "I", mode_y).await
    }
}

impl Appearance {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Appearance",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getCompositingMode",
                    "()Ljavax/microedition/m3g/CompositingMode;",
                    Self::get_compositing_mode,
                    Default::default(),
                ),
                JavaMethodProto::new("getFog", "()Ljavax/microedition/m3g/Fog;", Self::get_fog, Default::default()),
                JavaMethodProto::new(
                    "getMaterial",
                    "()Ljavax/microedition/m3g/Material;",
                    Self::get_material,
                    Default::default(),
                ),
                JavaMethodProto::new("getLayer", "()I", Self::get_layer, Default::default()),
                JavaMethodProto::new(
                    "getPolygonMode",
                    "()Ljavax/microedition/m3g/PolygonMode;",
                    Self::get_polygon_mode,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getTexture",
                    "(I)Ljavax/microedition/m3g/Texture2D;",
                    Self::get_texture,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setCompositingMode",
                    "(Ljavax/microedition/m3g/CompositingMode;)V",
                    Self::set_compositing_mode,
                    Default::default(),
                ),
                JavaMethodProto::new("setFog", "(Ljavax/microedition/m3g/Fog;)V", Self::set_fog, Default::default()),
                JavaMethodProto::new("setLayer", "(I)V", Self::set_layer, Default::default()),
                JavaMethodProto::new(
                    "setMaterial",
                    "(Ljavax/microedition/m3g/Material;)V",
                    Self::set_material,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setPolygonMode",
                    "(Ljavax/microedition/m3g/PolygonMode;)V",
                    Self::set_polygon_mode,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setTexture",
                    "(ILjavax/microedition/m3g/Texture2D;)V",
                    Self::set_texture,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("compositingMode", "Ljavax/microedition/m3g/CompositingMode;", Default::default()),
                JavaFieldProto::new("fog", "Ljavax/microedition/m3g/Fog;", Default::default()),
                JavaFieldProto::new("layer", "I", Default::default()),
                JavaFieldProto::new("polygonMode", "Ljavax/microedition/m3g/PolygonMode;", Default::default()),
                JavaFieldProto::new("material", "Ljavax/microedition/m3g/Material;", Default::default()),
                JavaFieldProto::new("texture0", "Ljavax/microedition/m3g/Texture2D;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await
    }

    async fn get_compositing_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<CompositingMode>> {
        jvm.get_field(&this, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;").await
    }

    async fn get_fog(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Fog>> {
        jvm.get_field(&this, "fog", "Ljavax/microedition/m3g/Fog;").await
    }

    async fn get_material(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Material>> {
        jvm.get_field(&this, "material", "Ljavax/microedition/m3g/Material;").await
    }

    async fn get_layer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "layer", "I").await
    }

    async fn get_polygon_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<PolygonMode>> {
        jvm.get_field(&this, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;").await
    }

    async fn get_texture(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, unit: i32) -> Result<ClassInstanceRef<Texture2D>> {
        if unit != 0 {
            return Ok(null_ref());
        }
        jvm.get_field(&this, "texture0", "Ljavax/microedition/m3g/Texture2D;").await
    }

    async fn set_compositing_mode(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        mode: ClassInstanceRef<CompositingMode>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;", mode)
            .await
    }

    async fn set_fog(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, fog: ClassInstanceRef<Fog>) -> Result<()> {
        jvm.put_field(&mut this, "fog", "Ljavax/microedition/m3g/Fog;", fog).await
    }

    async fn set_layer(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, layer: i32) -> Result<()> {
        if !(-63..=63).contains(&layer) {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "bad appearance layer").await);
        }
        jvm.put_field(&mut this, "layer", "I", layer).await
    }

    async fn set_material(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, material: ClassInstanceRef<Material>) -> Result<()> {
        jvm.put_field(&mut this, "material", "Ljavax/microedition/m3g/Material;", material).await
    }

    async fn set_polygon_mode(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        mode: ClassInstanceRef<PolygonMode>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;", mode)
            .await
    }

    async fn set_texture(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        unit: i32,
        texture: ClassInstanceRef<Texture2D>,
    ) -> Result<()> {
        if unit == 0 {
            jvm.put_field(&mut this, "texture0", "Ljavax/microedition/m3g/Texture2D;", texture)
                .await?;
        }
        Ok(())
    }
}

impl CompositingMode {
    const ALPHA: i32 = 64;
    const ALPHA_ADD: i32 = 65;
    const MODULATE: i32 = 66;
    const MODULATE_X2: i32 = 67;
    const REPLACE: i32 = 68;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/CompositingMode",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getAlphaThreshold", "()F", Self::get_alpha_threshold, Default::default()),
                JavaMethodProto::new("getBlending", "()I", Self::get_blending, Default::default()),
                JavaMethodProto::new("setAlphaThreshold", "(F)V", Self::set_alpha_threshold, Default::default()),
                JavaMethodProto::new("setAlphaWriteEnable", "(Z)V", Self::set_alpha_write_enable, Default::default()),
                JavaMethodProto::new("setBlending", "(I)V", Self::set_blending, Default::default()),
                JavaMethodProto::new("setColorWriteEnable", "(Z)V", Self::set_color_write_enable, Default::default()),
                JavaMethodProto::new("getDepthOffsetFactor", "()F", Self::get_depth_offset_factor, Default::default()),
                JavaMethodProto::new("getDepthOffsetUnits", "()F", Self::get_depth_offset_units, Default::default()),
                JavaMethodProto::new("isAlphaWriteEnabled", "()Z", Self::is_alpha_write_enabled, Default::default()),
                JavaMethodProto::new("isColorWriteEnabled", "()Z", Self::is_color_write_enabled, Default::default()),
                JavaMethodProto::new("isDepthTestEnabled", "()Z", Self::is_depth_test_enabled, Default::default()),
                JavaMethodProto::new("isDepthWriteEnabled", "()Z", Self::is_depth_write_enabled, Default::default()),
                JavaMethodProto::new("setDepthOffset", "(FF)V", Self::set_depth_offset, Default::default()),
                JavaMethodProto::new("setDepthTestEnable", "(Z)V", Self::set_depth_test_enable, Default::default()),
                JavaMethodProto::new("setDepthWriteEnable", "(Z)V", Self::set_depth_write_enable, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("alphaThreshold", "F", Default::default()),
                JavaFieldProto::new("alphaWrite", "Z", Default::default()),
                JavaFieldProto::new("blending", "I", Default::default()),
                JavaFieldProto::new("colorWrite", "Z", Default::default()),
                JavaFieldProto::new("depthTest", "Z", Default::default()),
                JavaFieldProto::new("depthWrite", "Z", Default::default()),
                JavaFieldProto::new("depthOffsetFactor", "F", Default::default()),
                JavaFieldProto::new("depthOffsetUnits", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "alphaWrite", "Z", true).await?;
        jvm.put_field(&mut this, "blending", "I", Self::REPLACE).await?;
        jvm.put_field(&mut this, "colorWrite", "Z", true).await?;
        jvm.put_field(&mut this, "depthTest", "Z", true).await?;
        jvm.put_field(&mut this, "depthWrite", "Z", true).await?;
        jvm.put_field(&mut this, "depthOffsetFactor", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "depthOffsetUnits", "F", 0.0f32).await
    }

    async fn get_alpha_threshold(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "alphaThreshold", "F").await
    }

    async fn get_blending(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "blending", "I").await
    }

    async fn set_alpha_threshold(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        jvm.put_field(&mut this, "alphaThreshold", "F", value).await
    }
    async fn set_alpha_write_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "alphaWrite", "Z", value).await
    }
    async fn set_blending(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "blending", "I", value).await
    }
    async fn set_color_write_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "colorWrite", "Z", value).await
    }
    async fn get_depth_offset_factor(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "depthOffsetFactor", "F").await
    }
    async fn get_depth_offset_units(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "depthOffsetUnits", "F").await
    }
    async fn is_alpha_write_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "alphaWrite", "Z").await
    }
    async fn is_color_write_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "colorWrite", "Z").await
    }
    async fn is_depth_test_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "depthTest", "Z").await
    }
    async fn is_depth_write_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "depthWrite", "Z").await
    }
    async fn set_depth_offset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, factor: f32, units: f32) -> Result<()> {
        jvm.put_field(&mut this, "depthOffsetFactor", "F", factor).await?;
        jvm.put_field(&mut this, "depthOffsetUnits", "F", units).await
    }
    async fn set_depth_test_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "depthTest", "Z", value).await
    }
    async fn set_depth_write_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "depthWrite", "Z", value).await
    }
}

impl Fog {
    const EXPONENTIAL: i32 = 80;
    const LINEAR: i32 = 81;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Fog",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getColor", "()I", Self::get_color, Default::default()),
                JavaMethodProto::new("getDensity", "()F", Self::get_density, Default::default()),
                JavaMethodProto::new("getFarDistance", "()F", Self::get_far_distance, Default::default()),
                JavaMethodProto::new("getMode", "()I", Self::get_mode, Default::default()),
                JavaMethodProto::new("getNearDistance", "()F", Self::get_near_distance, Default::default()),
                JavaMethodProto::new("setColor", "(I)V", Self::set_color, Default::default()),
                JavaMethodProto::new("setDensity", "(F)V", Self::set_density, Default::default()),
                JavaMethodProto::new("setLinear", "(FF)V", Self::set_linear, Default::default()),
                JavaMethodProto::new("setMode", "(I)V", Self::set_mode, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("color", "I", Default::default()),
                JavaFieldProto::new("density", "F", Default::default()),
                JavaFieldProto::new("mode", "I", Default::default()),
                JavaFieldProto::new("near", "F", Default::default()),
                JavaFieldProto::new("far", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "color", "I", 0).await?;
        jvm.put_field(&mut this, "density", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "mode", "I", Self::LINEAR).await?;
        jvm.put_field(&mut this, "near", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "far", "F", 1.0f32).await
    }
    async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "color", "I").await
    }
    async fn get_density(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "density", "F").await
    }
    async fn get_far_distance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "far", "F").await
    }
    async fn get_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "mode", "I").await
    }
    async fn get_near_distance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "near", "F").await
    }
    async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "color", "I", value).await
    }
    async fn set_density(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, density: f32) -> Result<()> {
        jvm.put_field(&mut this, "density", "F", density).await
    }
    async fn set_linear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, near: f32, far: f32) -> Result<()> {
        jvm.put_field(&mut this, "mode", "I", Self::LINEAR).await?;
        jvm.put_field(&mut this, "near", "F", near).await?;
        jvm.put_field(&mut this, "far", "F", far).await
    }
    async fn set_mode(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, mode: i32) -> Result<()> {
        let mode = match mode {
            Self::EXPONENTIAL | Self::LINEAR => mode,
            _ => Self::LINEAR,
        };
        jvm.put_field(&mut this, "mode", "I", mode).await
    }
}

impl PolygonMode {
    pub const CULL_BACK: i32 = 160;
    pub const CULL_FRONT: i32 = 161;
    pub const CULL_NONE: i32 = 162;
    pub const SHADE_FLAT: i32 = 164;
    pub const SHADE_SMOOTH: i32 = 165;
    pub const WINDING_CCW: i32 = 168;
    pub const WINDING_CW: i32 = 169;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/PolygonMode",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getCulling", "()I", Self::get_culling, Default::default()),
                JavaMethodProto::new("getShading", "()I", Self::get_shading, Default::default()),
                JavaMethodProto::new("getWinding", "()I", Self::get_winding, Default::default()),
                JavaMethodProto::new(
                    "isLocalCameraLightingEnabled",
                    "()Z",
                    Self::is_local_camera_lighting_enabled,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "isPerspectiveCorrectionEnabled",
                    "()Z",
                    Self::is_perspective_correction_enabled,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "isTwoSidedLightingEnabled",
                    "()Z",
                    Self::is_two_sided_lighting_enabled,
                    Default::default(),
                ),
                JavaMethodProto::new("setCulling", "(I)V", Self::set_culling, Default::default()),
                JavaMethodProto::new(
                    "setLocalCameraLightingEnable",
                    "(Z)V",
                    Self::set_local_camera_lighting_enable,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setPerspectiveCorrectionEnable",
                    "(Z)V",
                    Self::set_perspective_correction_enable,
                    Default::default(),
                ),
                JavaMethodProto::new("setShading", "(I)V", Self::set_shading, Default::default()),
                JavaMethodProto::new(
                    "setTwoSidedLightingEnable",
                    "(Z)V",
                    Self::set_two_sided_lighting_enable,
                    Default::default(),
                ),
                JavaMethodProto::new("setWinding", "(I)V", Self::set_winding, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("culling", "I", Default::default()),
                JavaFieldProto::new("localCameraLighting", "Z", Default::default()),
                JavaFieldProto::new("perspectiveCorrection", "Z", Default::default()),
                JavaFieldProto::new("shading", "I", Default::default()),
                JavaFieldProto::new("twoSidedLighting", "Z", Default::default()),
                JavaFieldProto::new("winding", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "culling", "I", Self::CULL_BACK).await?;
        jvm.put_field(&mut this, "localCameraLighting", "Z", false).await?;
        jvm.put_field(&mut this, "perspectiveCorrection", "Z", true).await?;
        jvm.put_field(&mut this, "shading", "I", Self::SHADE_SMOOTH).await?;
        jvm.put_field(&mut this, "twoSidedLighting", "Z", false).await?;
        jvm.put_field(&mut this, "winding", "I", Self::WINDING_CCW).await
    }
    async fn get_culling(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "culling", "I").await
    }
    async fn get_shading(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "shading", "I").await
    }
    async fn get_winding(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "winding", "I").await
    }
    async fn is_local_camera_lighting_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "localCameraLighting", "Z").await
    }
    async fn is_perspective_correction_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "perspectiveCorrection", "Z").await
    }
    async fn is_two_sided_lighting_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "twoSidedLighting", "Z").await
    }
    async fn set_culling(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "culling", "I", value).await
    }
    async fn set_local_camera_lighting_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "localCameraLighting", "Z", value).await
    }
    async fn set_perspective_correction_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "perspectiveCorrection", "Z", value).await
    }
    async fn set_shading(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "shading", "I", value).await
    }
    async fn set_two_sided_lighting_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "twoSidedLighting", "Z", value).await
    }
    async fn set_winding(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "winding", "I", value).await
    }
}

impl Image2D {
    pub const ALPHA: i32 = 96;
    pub const LUMINANCE: i32 = 97;
    pub const LUMINANCE_ALPHA: i32 = 98;
    pub const RGB: i32 = 99;
    pub const RGBA: i32 = 100;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Image2D",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init_empty, Default::default()),
                JavaMethodProto::new("<init>", "(ILjava/lang/Object;)V", Self::init_from_image, Default::default()),
                JavaMethodProto::new("<init>", "(III)V", Self::init_mutable, Default::default()),
                JavaMethodProto::new("<init>", "(III[B)V", Self::init_from_pixels, Default::default()),
                JavaMethodProto::new("<init>", "(III[B[B)V", Self::init_from_pixels_palette, Default::default()),
                JavaMethodProto::new("getFormat", "()I", Self::get_format, Default::default()),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new("isMutable", "()Z", Self::is_mutable, Default::default()),
                JavaMethodProto::new("set", "(IIII[B)V", Self::set, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("format", "I", Default::default()),
                JavaFieldProto::new("width", "I", Default::default()),
                JavaFieldProto::new("height", "I", Default::default()),
                JavaFieldProto::new("image", "Ljavax/microedition/lcdui/Image;", Default::default()),
                JavaFieldProto::new("mutable", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init_empty(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "format", "I", Self::RGBA).await?;
        jvm.put_field(&mut this, "width", "I", 1).await?;
        jvm.put_field(&mut this, "height", "I", 1).await?;
        let image = Image::from_argb(jvm, 1, 1, vec![0xffff_ffffu32 as i32]).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "mutable", "Z", false).await
    }

    async fn init_from_image(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        format: i32,
        image_object: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        let image: ClassInstanceRef<Image> = cast_ref(&image_object);
        let width = if image.is_null() {
            1
        } else {
            jvm.get_field(&image, "width", "I").await.unwrap_or(1)
        };
        let height = if image.is_null() {
            1
        } else {
            jvm.get_field(&image, "height", "I").await.unwrap_or(1)
        };
        jvm.put_field(&mut this, "format", "I", format).await?;
        jvm.put_field(&mut this, "width", "I", width).await?;
        jvm.put_field(&mut this, "height", "I", height).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "mutable", "Z", false).await
    }

    async fn init_mutable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, format: i32, width: i32, height: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        let width = width.max(1);
        let height = height.max(1);
        jvm.put_field(&mut this, "format", "I", format).await?;
        jvm.put_field(&mut this, "width", "I", width).await?;
        jvm.put_field(&mut this, "height", "I", height).await?;
        let image = Image::from_argb(jvm, width, height, vec![0; (width * height) as usize]).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "mutable", "Z", true).await
    }

    async fn init_from_pixels(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        format: i32,
        width: i32,
        height: i32,
        pixels: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        Self::init_from_pixel_data(jvm, &mut this, format, width, height, pixels, null_ref()).await
    }

    async fn init_from_pixels_palette(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        format: i32,
        width: i32,
        height: i32,
        pixels: ClassInstanceRef<Array<i8>>,
        palette: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        Self::init_from_pixel_data(jvm, &mut this, format, width, height, pixels, palette).await
    }

    async fn get_format(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "format", "I").await
    }
    async fn get_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "width", "I").await
    }
    async fn get_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "height", "I").await
    }

    async fn is_mutable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "mutable", "Z").await
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        if pixels.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Image2D.set").await);
        }
        if !jvm.get_field::<bool>(&this, "mutable", "Z").await.unwrap_or(false) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Image2D is immutable").await);
        }
        let image_width: i32 = jvm.get_field(&this, "width", "I").await?;
        let image_height: i32 = jvm.get_field(&this, "height", "I").await?;
        if x < 0 || y < 0 || width < 0 || height < 0 || x + width > image_width || y + height > image_height {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Image2D.set bounds").await);
        }
        let format: i32 = jvm.get_field(&this, "format", "I").await?;
        let byte_len = jvm.array_length(&pixels).await?;
        let pixel_bytes = raw_u8_array(jvm, &pixels, byte_len).await?;
        let patch = decode_m3g_image_pixels(format, width, height, &[], &pixel_bytes);
        let image: ClassInstanceRef<Image> = jvm
            .get_field(&this, "image", "Ljavax/microedition/lcdui/Image;")
            .await
            .unwrap_or_else(|_| null_ref());
        let mut argb = if image.is_null() {
            vec![0; (image_width * image_height) as usize]
        } else {
            let (_, _, pixels_array) = Image::pixels(jvm, &image).await?;
            raw_i32_array(jvm, &pixels_array, (image_width * image_height) as usize).await?
        };
        for row in 0..height {
            let src = (row * width) as usize;
            let dst = ((y + row) * image_width + x) as usize;
            let count = width as usize;
            argb[dst..dst + count].copy_from_slice(&patch[src..src + count]);
        }
        let image = Image::from_argb(jvm, image_width, image_height, argb).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await
    }

    async fn init_from_pixel_data(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        format: i32,
        width: i32,
        height: i32,
        pixels: ClassInstanceRef<Array<i8>>,
        palette: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        if pixels.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Image2D pixels").await);
        }
        let _: () = jvm.invoke_special(this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        let width = width.max(1);
        let height = height.max(1);
        let pixels_len = jvm.array_length(&pixels).await?;
        let pixels = raw_u8_array(jvm, &pixels, pixels_len).await?;
        let palette = if palette.is_null() {
            Vec::new()
        } else {
            let palette_len = jvm.array_length(&palette).await?;
            raw_u8_array(jvm, &palette, palette_len).await?
        };
        let argb = decode_m3g_image_pixels(format, width, height, &palette, &pixels);
        let image = Image::from_argb(jvm, width, height, argb).await?;
        jvm.put_field(this, "format", "I", format).await?;
        jvm.put_field(this, "width", "I", width).await?;
        jvm.put_field(this, "height", "I", height).await?;
        jvm.put_field(this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(this, "mutable", "Z", false).await
    }
}

impl Texture2D {
    const FUNC_ADD: i32 = 224;
    const FUNC_BLEND: i32 = 225;
    const FUNC_DECAL: i32 = 226;
    const FUNC_MODULATE: i32 = 227;
    const FUNC_REPLACE: i32 = 228;
    const WRAP_CLAMP: i32 = 240;
    const WRAP_REPEAT: i32 = 241;
    const FILTER_BASE_LEVEL: i32 = 208;
    const FILTER_NEAREST: i32 = 209;
    const FILTER_LINEAR: i32 = 210;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Texture2D",
            parent_class: Some("javax/microedition/m3g/Transformable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init_empty, Default::default()),
                JavaMethodProto::new("<init>", "(Ljavax/microedition/m3g/Image2D;)V", Self::init, Default::default()),
                JavaMethodProto::new("getBlendColor", "()I", Self::get_blend_color, Default::default()),
                JavaMethodProto::new("getBlending", "()I", Self::get_blending, Default::default()),
                JavaMethodProto::new("getImage", "()Ljavax/microedition/m3g/Image2D;", Self::get_image, Default::default()),
                JavaMethodProto::new("getImageFilter", "()I", Self::get_image_filter, Default::default()),
                JavaMethodProto::new("getLevelFilter", "()I", Self::get_level_filter, Default::default()),
                JavaMethodProto::new("getWrappingS", "()I", Self::get_wrapping_s, Default::default()),
                JavaMethodProto::new("getWrappingT", "()I", Self::get_wrapping_t, Default::default()),
                JavaMethodProto::new("setBlendColor", "(I)V", Self::set_blend_color, Default::default()),
                JavaMethodProto::new("setBlending", "(I)V", Self::set_blending, Default::default()),
                JavaMethodProto::new("setFiltering", "(II)V", Self::set_filtering, Default::default()),
                JavaMethodProto::new("setImage", "(Ljavax/microedition/m3g/Image2D;)V", Self::set_image, Default::default()),
                JavaMethodProto::new("setWrapping", "(II)V", Self::set_wrapping, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("image", "Ljavax/microedition/m3g/Image2D;", Default::default()),
                JavaFieldProto::new("blendColor", "I", Default::default()),
                JavaFieldProto::new("blending", "I", Default::default()),
                JavaFieldProto::new("wrappingS", "I", Default::default()),
                JavaFieldProto::new("wrappingT", "I", Default::default()),
                JavaFieldProto::new("levelFilter", "I", Default::default()),
                JavaFieldProto::new("imageFilter", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init_empty(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/m3g/Transformable", "<init>", "()V", ())
            .await?;
        Self::set_default_fields(jvm, &mut this).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, image: ClassInstanceRef<Image2D>) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/m3g/Transformable", "<init>", "()V", ())
            .await?;
        Self::set_default_fields(jvm, &mut this).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await
    }

    async fn get_blend_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "blendColor", "I").await
    }

    async fn get_blending(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "blending", "I").await
    }

    async fn get_image(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Image2D>> {
        jvm.get_field(&this, "image", "Ljavax/microedition/m3g/Image2D;").await
    }

    async fn get_image_filter(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "imageFilter", "I").await
    }

    async fn get_level_filter(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "levelFilter", "I").await
    }

    async fn get_wrapping_s(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "wrappingS", "I").await
    }

    async fn get_wrapping_t(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "wrappingT", "I").await
    }

    async fn set_blend_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, color: i32) -> Result<()> {
        jvm.put_field(&mut this, "blendColor", "I", color).await
    }

    async fn set_blending(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, blending: i32) -> Result<()> {
        jvm.put_field(&mut this, "blending", "I", blending).await
    }

    async fn set_filtering(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, level_filter: i32, image_filter: i32) -> Result<()> {
        jvm.put_field(&mut this, "levelFilter", "I", level_filter).await?;
        jvm.put_field(&mut this, "imageFilter", "I", image_filter).await
    }

    async fn set_image(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, image: ClassInstanceRef<Image2D>) -> Result<()> {
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await
    }

    async fn set_wrapping(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, wrap_s: i32, wrap_t: i32) -> Result<()> {
        jvm.put_field(&mut this, "wrappingS", "I", wrap_s).await?;
        jvm.put_field(&mut this, "wrappingT", "I", wrap_t).await
    }

    async fn set_default_fields(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(this, "image", "Ljavax/microedition/m3g/Image2D;", null_ref::<Image2D>())
            .await?;
        jvm.put_field(this, "blendColor", "I", 0).await?;
        jvm.put_field(this, "blending", "I", Self::FUNC_MODULATE).await?;
        jvm.put_field(this, "wrappingS", "I", Self::WRAP_REPEAT).await?;
        jvm.put_field(this, "wrappingT", "I", Self::WRAP_REPEAT).await?;
        jvm.put_field(this, "levelFilter", "I", Self::FILTER_BASE_LEVEL).await?;
        jvm.put_field(this, "imageFilter", "I", Self::FILTER_NEAREST).await
    }
}

impl Sprite3D {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Sprite3D",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(ZLjavax/microedition/m3g/Image2D;Ljavax/microedition/m3g/Appearance;)V",
                    Self::init,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getAppearance",
                    "()Ljavax/microedition/m3g/Appearance;",
                    Self::get_appearance,
                    Default::default(),
                ),
                JavaMethodProto::new("getCropHeight", "()I", Self::get_crop_height, Default::default()),
                JavaMethodProto::new("getCropWidth", "()I", Self::get_crop_width, Default::default()),
                JavaMethodProto::new("getCropX", "()I", Self::get_crop_x, Default::default()),
                JavaMethodProto::new("getCropY", "()I", Self::get_crop_y, Default::default()),
                JavaMethodProto::new("getImage", "()Ljavax/microedition/m3g/Image2D;", Self::get_image, Default::default()),
                JavaMethodProto::new("isScaled", "()Z", Self::is_scaled, Default::default()),
                JavaMethodProto::new(
                    "setAppearance",
                    "(Ljavax/microedition/m3g/Appearance;)V",
                    Self::set_appearance,
                    Default::default(),
                ),
                JavaMethodProto::new("setCrop", "(IIII)V", Self::set_crop, Default::default()),
                JavaMethodProto::new("setImage", "(Ljavax/microedition/m3g/Image2D;)V", Self::set_image, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("scaled", "Z", Default::default()),
                JavaFieldProto::new("image", "Ljavax/microedition/m3g/Image2D;", Default::default()),
                JavaFieldProto::new("appearance", "Ljavax/microedition/m3g/Appearance;", Default::default()),
                JavaFieldProto::new("cropX", "I", Default::default()),
                JavaFieldProto::new("cropY", "I", Default::default()),
                JavaFieldProto::new("cropW", "I", Default::default()),
                JavaFieldProto::new("cropH", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        scaled: bool,
        image: ClassInstanceRef<Image2D>,
        appearance: ClassInstanceRef<Appearance>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "scaled", "Z", scaled).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
        jvm.put_field(&mut this, "appearance", "Ljavax/microedition/m3g/Appearance;", appearance)
            .await?;
        jvm.put_field(&mut this, "cropX", "I", 0).await?;
        jvm.put_field(&mut this, "cropY", "I", 0).await?;
        jvm.put_field(&mut this, "cropW", "I", 0).await?;
        jvm.put_field(&mut this, "cropH", "I", 0).await
    }

    async fn get_appearance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Appearance>> {
        jvm.get_field(&this, "appearance", "Ljavax/microedition/m3g/Appearance;").await
    }

    async fn get_crop_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropH", "I").await
    }

    async fn get_crop_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropW", "I").await
    }

    async fn get_crop_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropX", "I").await
    }

    async fn get_crop_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cropY", "I").await
    }

    async fn get_image(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Image2D>> {
        jvm.get_field(&this, "image", "Ljavax/microedition/m3g/Image2D;").await
    }

    async fn is_scaled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "scaled", "Z").await
    }

    async fn set_appearance(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        appearance: ClassInstanceRef<Appearance>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "appearance", "Ljavax/microedition/m3g/Appearance;", appearance)
            .await
    }

    async fn set_crop(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        jvm.put_field(&mut this, "cropX", "I", x).await?;
        jvm.put_field(&mut this, "cropY", "I", y).await?;
        jvm.put_field(&mut this, "cropW", "I", width).await?;
        jvm.put_field(&mut this, "cropH", "I", height).await
    }

    async fn set_image(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, image: ClassInstanceRef<Image2D>) -> Result<()> {
        jvm.put_field(&mut this, "image", "Ljavax/microedition/m3g/Image2D;", image).await
    }
}

impl Mesh {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Mesh",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;)V",
                    Self::init_single,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getAppearance",
                    "(I)Ljavax/microedition/m3g/Appearance;",
                    Self::get_appearance,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getIndexBuffer",
                    "(I)Ljavax/microedition/m3g/IndexBuffer;",
                    Self::get_index_buffer,
                    Default::default(),
                ),
                JavaMethodProto::new("getSubmeshCount", "()I", Self::get_submesh_count, Default::default()),
                JavaMethodProto::new(
                    "getVertexBuffer",
                    "()Ljavax/microedition/m3g/VertexBuffer;",
                    Self::get_vertex_buffer,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setAppearance",
                    "(ILjavax/microedition/m3g/Appearance;)V",
                    Self::set_appearance,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", Default::default()),
                JavaFieldProto::new("submeshCount", "I", Default::default()),
                JavaFieldProto::new("indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", Default::default()),
                JavaFieldProto::new("appearances", "[Ljavax/microedition/m3g/Appearance;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await
    }

    async fn init_single(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vertex_buffer: ClassInstanceRef<VertexBuffer>,
        index_buffer: ClassInstanceRef<IndexBuffer>,
        appearance: ClassInstanceRef<Appearance>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        let mut index_buffers = jvm.instantiate_array("Ljavax/microedition/m3g/IndexBuffer;", 1).await?;
        jvm.store_array(&mut index_buffers, 0, vec![index_buffer]).await?;
        let mut appearances = jvm.instantiate_array("Ljavax/microedition/m3g/Appearance;", 1).await?;
        jvm.store_array(&mut appearances, 0, vec![appearance]).await?;
        jvm.put_field(&mut this, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", vertex_buffer)
            .await?;
        jvm.put_field(&mut this, "submeshCount", "I", 1).await?;
        jvm.put_field(&mut this, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", index_buffers)
            .await?;
        jvm.put_field(&mut this, "appearances", "[Ljavax/microedition/m3g/Appearance;", appearances)
            .await
    }

    async fn get_appearance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Appearance>> {
        let count: i32 = jvm.get_field(&this, "submeshCount", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "M3G appearance index").await);
        }
        let appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> =
            jvm.get_field(&this, "appearances", "[Ljavax/microedition/m3g/Appearance;").await?;
        Ok(jvm
            .load_array(&appearances, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or_else(null_ref))
    }

    async fn get_index_buffer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<IndexBuffer>> {
        let count: i32 = jvm.get_field(&this, "submeshCount", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "M3G index buffer index").await);
        }
        let index_buffers: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> =
            jvm.get_field(&this, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;").await?;
        Ok(jvm
            .load_array(&index_buffers, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or_else(null_ref))
    }

    async fn get_submesh_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "submeshCount", "I").await
    }

    async fn get_vertex_buffer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<VertexBuffer>> {
        jvm.get_field(&this, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;").await
    }

    async fn set_appearance(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        appearance: ClassInstanceRef<Appearance>,
    ) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "submeshCount", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "M3G appearance index").await);
        }
        let mut appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> =
            jvm.get_field(&this, "appearances", "[Ljavax/microedition/m3g/Appearance;").await?;
        jvm.store_array(&mut appearances, index as usize, vec![appearance]).await
    }
}

impl IndexBuffer {
    pub fn as_proto() -> RuntimeClassProto {
        object3d_proto(
            "javax/microedition/m3g/IndexBuffer",
            Some("javax/microedition/m3g/Object3D"),
            vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
        )
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await
    }
}

impl TriangleStripArray {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/TriangleStripArray",
            parent_class: Some("javax/microedition/m3g/IndexBuffer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I[I)V", Self::init_implicit, Default::default()),
                JavaMethodProto::new("<init>", "([I[I)V", Self::init_explicit, Default::default()),
                JavaMethodProto::new("getIndexCount", "()I", Self::get_index_count, Default::default()),
                JavaMethodProto::new("getIndices", "([I)V", Self::get_indices, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("indices", "[I", Default::default()),
                JavaFieldProto::new("stripLengths", "[I", Default::default()),
                JavaFieldProto::new("triangleCache", "[I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/IndexBuffer", "<init>", "()V", ()).await
    }

    async fn init_implicit(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        first_index: i32,
        strip_lengths: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        if strip_lengths.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "TriangleStripArray stripLengths").await);
        }
        let lengths_len = jvm.array_length(&strip_lengths).await?;
        let lengths: Vec<i32> = jvm.load_array(&strip_lengths, 0, lengths_len).await?;
        let mut indices = jvm.instantiate_array("I", 1).await?;
        jvm.store_array(&mut indices, 0, vec![first_index]).await?;
        let mut lengths_array = jvm.instantiate_array("I", lengths.len()).await?;
        jvm.store_array(&mut lengths_array, 0, lengths).await?;
        jvm.put_field(&mut this, "indices", "[I", indices).await?;
        jvm.put_field(&mut this, "stripLengths", "[I", lengths_array).await
    }

    async fn init_explicit(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        indices: ClassInstanceRef<Array<i32>>,
        strip_lengths: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        if indices.is_null() || strip_lengths.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "TriangleStripArray data").await);
        }
        let indices_len = jvm.array_length(&indices).await?;
        let lengths_len = jvm.array_length(&strip_lengths).await?;
        let indices: Vec<i32> = jvm.load_array(&indices, 0, indices_len).await?;
        let lengths: Vec<i32> = jvm.load_array(&strip_lengths, 0, lengths_len).await?;
        let mut indices_array = jvm.instantiate_array("I", indices.len()).await?;
        jvm.store_array(&mut indices_array, 0, indices).await?;
        let mut lengths_array = jvm.instantiate_array("I", lengths.len()).await?;
        jvm.store_array(&mut lengths_array, 0, lengths).await?;
        jvm.put_field(&mut this, "indices", "[I", indices_array).await?;
        jvm.put_field(&mut this, "stripLengths", "[I", lengths_array).await
    }

    async fn get_index_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(Self::indices_for_getters(jvm, &this).await?.len() as i32)
    }

    async fn get_indices(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut out: ClassInstanceRef<Array<i32>>) -> Result<()> {
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "TriangleStripArray.getIndices").await);
        }
        let indices = Self::indices_for_getters(jvm, &this).await?;
        if jvm.array_length(&out).await? < indices.len() {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "indices array too small").await);
        }
        jvm.store_array(&mut out, 0, indices).await
    }

    async fn indices_for_getters(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<i32>> {
        let indices_array: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "indices", "[I").await.unwrap_or_else(|_| null_ref());
        if indices_array.is_null() {
            return Ok(Vec::new());
        }
        let indices_len = jvm.array_length(&indices_array).await?;
        let indices: Vec<i32> = jvm.load_array(&indices_array, 0, indices_len).await?;
        if indices_len != 1 {
            return Ok(indices);
        }

        let lengths_array: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "stripLengths", "[I").await.unwrap_or_else(|_| null_ref());
        if lengths_array.is_null() {
            return Ok(indices);
        }
        let lengths: Vec<i32> = jvm.load_array(&lengths_array, 0, jvm.array_length(&lengths_array).await?).await?;
        let count = lengths.iter().copied().filter(|length| *length > 0).sum::<i32>().max(0) as usize;
        Ok((0..count).map(|offset| indices[0] + offset as i32).collect())
    }
}

impl VertexArray {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/VertexArray",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(III)V", Self::init_sized, Default::default()),
                JavaMethodProto::new("get", "(II[B)V", Self::get_bytes, Default::default()),
                JavaMethodProto::new("get", "(II[S)V", Self::get_shorts, Default::default()),
                JavaMethodProto::new("getComponentCount", "()I", Self::get_component_count, Default::default()),
                JavaMethodProto::new("getComponentType", "()I", Self::get_component_type, Default::default()),
                JavaMethodProto::new("getVertexCount", "()I", Self::get_vertex_count, Default::default()),
                JavaMethodProto::new("set", "(II[B)V", Self::set_bytes, Default::default()),
                JavaMethodProto::new("set", "(II[S)V", Self::set_shorts, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("componentSize", "I", Default::default()),
                JavaFieldProto::new("componentCount", "I", Default::default()),
                JavaFieldProto::new("vertexCount", "I", Default::default()),
                JavaFieldProto::new("version", "I", Default::default()),
                JavaFieldProto::new("byteData", "[B", Default::default()),
                JavaFieldProto::new("shortData", "[S", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await
    }

    async fn init_sized(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vertex_count: i32,
        component_count: i32,
        component_size: i32,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        if !(1..=65535).contains(&vertex_count) || !(2..=4).contains(&component_count) || (component_size != 1 && component_size != 2) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid vertex array layout").await);
        }
        jvm.put_field(&mut this, "componentSize", "I", component_size).await?;
        jvm.put_field(&mut this, "componentCount", "I", component_count).await?;
        jvm.put_field(&mut this, "vertexCount", "I", vertex_count).await?;
        jvm.put_field(&mut this, "version", "I", 1i32).await?;
        let value_count = (vertex_count * component_count) as usize;
        if component_size == 1 {
            let byte_data = jvm.instantiate_array("B", value_count).await?;
            jvm.put_field(&mut this, "byteData", "[B", byte_data).await?;
            jvm.put_field(&mut this, "shortData", "[S", null_ref::<Array<i16>>()).await
        } else {
            let short_data = jvm.instantiate_array("S", value_count).await?;
            jvm.put_field(&mut this, "byteData", "[B", null_ref::<Array<i8>>()).await?;
            jvm.put_field(&mut this, "shortData", "[S", short_data).await
        }
    }

    async fn get_component_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "componentCount", "I").await
    }

    async fn get_component_type(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "componentSize", "I").await
    }

    async fn get_vertex_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "vertexCount", "I").await
    }

    async fn set_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        values: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        Self::set_array_data(jvm, this, first, count, Some(values), None).await
    }

    async fn set_shorts(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        values: ClassInstanceRef<Array<i16>>,
    ) -> Result<()> {
        Self::set_array_data(jvm, this, first, count, None, Some(values)).await
    }

    async fn get_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        values: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        Self::get_array_data(jvm, this, first, count, Some(values), None).await
    }

    async fn get_shorts(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        values: ClassInstanceRef<Array<i16>>,
    ) -> Result<()> {
        Self::get_array_data(jvm, this, first, count, None, Some(values)).await
    }

    async fn set_array_data(
        jvm: &Jvm,
        mut this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        bytes: Option<ClassInstanceRef<Array<i8>>>,
        shorts: Option<ClassInstanceRef<Array<i16>>>,
    ) -> Result<()> {
        let (component_size, component_count, vertex_count, value_count, offset) = Self::checked_range(jvm, &this, first, count).await?;
        if let Some(values) = bytes {
            if values.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "VertexArray.set").await);
            }
            if component_size != 1 || jvm.array_length(&values).await? < value_count {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "byte vertex data").await);
            }
            let mut data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "byteData", "[B").await?;
            if data.is_null() {
                data = jvm.instantiate_array("B", (vertex_count * component_count) as usize).await?.into();
            }
            let values: Vec<i8> = jvm.load_array(&values, 0, value_count).await?;
            jvm.store_array(&mut data, offset, values).await?;
            jvm.put_field(&mut this, "byteData", "[B", data).await?;
        } else if let Some(values) = shorts {
            if values.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "VertexArray.set").await);
            }
            if component_size != 2 || jvm.array_length(&values).await? < value_count {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "short vertex data").await);
            }
            let mut data: ClassInstanceRef<Array<i16>> = jvm.get_field(&this, "shortData", "[S").await?;
            if data.is_null() {
                data = jvm.instantiate_array("S", (vertex_count * component_count) as usize).await?.into();
            }
            let values: Vec<i16> = jvm.load_array(&values, 0, value_count).await?;
            jvm.store_array(&mut data, offset, values).await?;
            jvm.put_field(&mut this, "shortData", "[S", data).await?;
        } else {
            return Ok(());
        }
        Self::bump_version(jvm, &mut this).await
    }

    async fn bump_version(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let version = jvm.get_field::<i32>(this, "version", "I").await.unwrap_or(0).wrapping_add(1);
        jvm.put_field(this, "version", "I", version).await
    }

    async fn get_array_data(
        jvm: &Jvm,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        bytes: Option<ClassInstanceRef<Array<i8>>>,
        shorts: Option<ClassInstanceRef<Array<i16>>>,
    ) -> Result<()> {
        let (component_size, _component_count, _vertex_count, value_count, offset) = Self::checked_range(jvm, &this, first, count).await?;
        if let Some(mut values) = bytes {
            if values.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "VertexArray.get").await);
            }
            if component_size != 1 || jvm.array_length(&values).await? < value_count {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "byte vertex data").await);
            }
            let data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "byteData", "[B").await?;
            let data: Vec<i8> = if data.is_null() {
                vec![0; value_count]
            } else {
                jvm.load_array(&data, offset, value_count).await?
            };
            jvm.store_array(&mut values, 0, data).await
        } else if let Some(mut values) = shorts {
            if values.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "VertexArray.get").await);
            }
            if component_size != 2 || jvm.array_length(&values).await? < value_count {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "short vertex data").await);
            }
            let data: ClassInstanceRef<Array<i16>> = jvm.get_field(&this, "shortData", "[S").await?;
            let data: Vec<i16> = if data.is_null() {
                vec![0; value_count]
            } else {
                jvm.load_array(&data, offset, value_count).await?
            };
            jvm.store_array(&mut values, 0, data).await
        } else {
            Ok(())
        }
    }

    async fn checked_range(jvm: &Jvm, this: &ClassInstanceRef<Self>, first: i32, count: i32) -> Result<(i32, i32, i32, usize, usize)> {
        let component_size: i32 = jvm.get_field(this, "componentSize", "I").await?;
        let component_count: i32 = jvm.get_field(this, "componentCount", "I").await?;
        let vertex_count: i32 = jvm.get_field(this, "vertexCount", "I").await?;
        if first < 0 || count < 0 || first.saturating_add(count) > vertex_count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "vertex array range").await);
        }
        let value_count = (count * component_count) as usize;
        let offset = (first * component_count) as usize;
        Ok((component_size, component_count, vertex_count, value_count, offset))
    }
}

impl VertexBuffer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/VertexBuffer",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getColors",
                    "()Ljavax/microedition/m3g/VertexArray;",
                    Self::get_colors,
                    Default::default(),
                ),
                JavaMethodProto::new("getDefaultColor", "()I", Self::get_default_color, Default::default()),
                JavaMethodProto::new(
                    "getNormals",
                    "()Ljavax/microedition/m3g/VertexArray;",
                    Self::get_normals,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getPositions",
                    "([F)Ljavax/microedition/m3g/VertexArray;",
                    Self::get_positions,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getTexCoords",
                    "(I[F)Ljavax/microedition/m3g/VertexArray;",
                    Self::get_tex_coords,
                    Default::default(),
                ),
                JavaMethodProto::new("getVertexCount", "()I", Self::get_vertex_count, Default::default()),
                JavaMethodProto::new(
                    "setColors",
                    "(Ljavax/microedition/m3g/VertexArray;)V",
                    Self::set_colors,
                    Default::default(),
                ),
                JavaMethodProto::new("setDefaultColor", "(I)V", Self::set_default_color, Default::default()),
                JavaMethodProto::new(
                    "setNormals",
                    "(Ljavax/microedition/m3g/VertexArray;)V",
                    Self::set_normals,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setPositions",
                    "(Ljavax/microedition/m3g/VertexArray;F[F)V",
                    Self::set_positions,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setTexCoords",
                    "(ILjavax/microedition/m3g/VertexArray;F[F)V",
                    Self::set_tex_coords,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("defaultColor", "I", Default::default()),
                JavaFieldProto::new("positions", "Ljavax/microedition/m3g/VertexArray;", Default::default()),
                JavaFieldProto::new("normals", "Ljavax/microedition/m3g/VertexArray;", Default::default()),
                JavaFieldProto::new("colors", "Ljavax/microedition/m3g/VertexArray;", Default::default()),
                JavaFieldProto::new("texCoords0", "Ljavax/microedition/m3g/VertexArray;", Default::default()),
                JavaFieldProto::new("positionScale", "F", Default::default()),
                JavaFieldProto::new("positionBias", "[F", Default::default()),
                JavaFieldProto::new("texScale", "F", Default::default()),
                JavaFieldProto::new("texBias", "[F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "defaultColor", "I", 0xffff_ffffu32 as i32).await?;
        jvm.put_field(&mut this, "positions", "Ljavax/microedition/m3g/VertexArray;", null_ref::<VertexArray>())
            .await?;
        jvm.put_field(&mut this, "normals", "Ljavax/microedition/m3g/VertexArray;", null_ref::<VertexArray>())
            .await?;
        jvm.put_field(&mut this, "colors", "Ljavax/microedition/m3g/VertexArray;", null_ref::<VertexArray>())
            .await?;
        jvm.put_field(&mut this, "texCoords0", "Ljavax/microedition/m3g/VertexArray;", null_ref::<VertexArray>())
            .await?;
        jvm.put_field(&mut this, "positionScale", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "texScale", "F", 1.0f32).await?;
        let mut position_bias = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut position_bias, 0, vec![0.0f32, 0.0, 0.0]).await?;
        let mut tex_bias = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut tex_bias, 0, vec![0.0f32, 0.0, 0.0]).await?;
        jvm.put_field(&mut this, "positionBias", "[F", position_bias).await?;
        jvm.put_field(&mut this, "texBias", "[F", tex_bias).await
    }

    async fn get_colors(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<VertexArray>> {
        jvm.get_field(&this, "colors", "Ljavax/microedition/m3g/VertexArray;").await
    }

    async fn get_normals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<VertexArray>> {
        jvm.get_field(&this, "normals", "Ljavax/microedition/m3g/VertexArray;").await
    }

    async fn get_positions(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        scale_bias: ClassInstanceRef<Array<f32>>,
    ) -> Result<ClassInstanceRef<VertexArray>> {
        if !scale_bias.is_null() {
            Self::write_scale_bias(jvm, scale_bias, &this, "positionScale", "positionBias", 4).await?;
        }
        jvm.get_field(&this, "positions", "Ljavax/microedition/m3g/VertexArray;").await
    }

    async fn get_tex_coords(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        unit: i32,
        scale_bias: ClassInstanceRef<Array<f32>>,
    ) -> Result<ClassInstanceRef<VertexArray>> {
        if unit != 0 {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "texture unit").await);
        }
        let tex_coords: ClassInstanceRef<VertexArray> = jvm.get_field(&this, "texCoords0", "Ljavax/microedition/m3g/VertexArray;").await?;
        if !scale_bias.is_null() && !tex_coords.is_null() {
            let component_count: i32 = jvm.get_field(&tex_coords, "componentCount", "I").await.unwrap_or(2);
            Self::write_scale_bias(jvm, scale_bias, &this, "texScale", "texBias", (component_count + 1).max(0) as usize).await?;
        }
        Ok(tex_coords)
    }

    async fn get_vertex_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let positions: ClassInstanceRef<VertexArray> = jvm.get_field(&this, "positions", "Ljavax/microedition/m3g/VertexArray;").await?;
        if positions.is_null() {
            Ok(0)
        } else {
            jvm.get_field(&positions, "vertexCount", "I").await
        }
    }

    async fn get_default_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "defaultColor", "I").await
    }

    async fn set_default_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, color: i32) -> Result<()> {
        jvm.put_field(&mut this, "defaultColor", "I", color).await
    }

    async fn set_positions(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        positions: ClassInstanceRef<VertexArray>,
        scale: f32,
        bias: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if !positions.is_null() {
            let component_count: i32 = jvm.get_field(&positions, "componentCount", "I").await?;
            if component_count != 3 {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "position array must have 3 components")
                    .await);
            }
            Self::check_compatible_vertex_count(jvm, &this, &positions).await?;
        }
        let bias_values = Self::read_bias(jvm, &bias, 3).await?;
        let mut bias_array = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut bias_array, 0, bias_values).await?;
        jvm.put_field(&mut this, "positions", "Ljavax/microedition/m3g/VertexArray;", positions)
            .await?;
        jvm.put_field(&mut this, "positionScale", "F", scale).await?;
        jvm.put_field(&mut this, "positionBias", "[F", bias_array).await
    }

    async fn set_tex_coords(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        unit: i32,
        tex_coords: ClassInstanceRef<VertexArray>,
        scale: f32,
        bias: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if unit != 0 {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "texture unit").await);
        }
        let bias_len = if tex_coords.is_null() {
            3
        } else {
            let component_count: i32 = jvm.get_field(&tex_coords, "componentCount", "I").await?;
            if component_count != 2 && component_count != 3 {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "texcoord array must have 2 or 3 components")
                    .await);
            }
            Self::check_compatible_vertex_count(jvm, &this, &tex_coords).await?;
            component_count as usize
        };
        let mut bias_values = Self::read_bias(jvm, &bias, bias_len).await?;
        bias_values.resize(3, 0.0);
        let mut bias_array = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut bias_array, 0, bias_values).await?;
        jvm.put_field(&mut this, "texCoords0", "Ljavax/microedition/m3g/VertexArray;", tex_coords)
            .await?;
        jvm.put_field(&mut this, "texScale", "F", scale).await?;
        jvm.put_field(&mut this, "texBias", "[F", bias_array).await
    }

    async fn set_normals(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, normals: ClassInstanceRef<VertexArray>) -> Result<()> {
        if !normals.is_null() {
            let component_count: i32 = jvm.get_field(&normals, "componentCount", "I").await?;
            if component_count != 3 {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "normal array must have 3 components")
                    .await);
            }
            Self::check_compatible_vertex_count(jvm, &this, &normals).await?;
        }
        jvm.put_field(&mut this, "normals", "Ljavax/microedition/m3g/VertexArray;", normals).await
    }

    async fn set_colors(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, colors: ClassInstanceRef<VertexArray>) -> Result<()> {
        if !colors.is_null() {
            let component_size: i32 = jvm.get_field(&colors, "componentSize", "I").await?;
            let component_count: i32 = jvm.get_field(&colors, "componentCount", "I").await?;
            if component_size != 1 || !(3..=4).contains(&component_count) {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "color array must be byte RGB/RGBA")
                    .await);
            }
            Self::check_compatible_vertex_count(jvm, &this, &colors).await?;
        }
        jvm.put_field(&mut this, "colors", "Ljavax/microedition/m3g/VertexArray;", colors).await
    }

    async fn check_compatible_vertex_count(jvm: &Jvm, this: &ClassInstanceRef<Self>, array: &ClassInstanceRef<VertexArray>) -> Result<()> {
        let positions: ClassInstanceRef<VertexArray> = jvm
            .get_field(this, "positions", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let existing = if positions.is_null() {
            0
        } else {
            jvm.get_field(&positions, "vertexCount", "I").await.unwrap_or(0)
        };
        let incoming: i32 = jvm.get_field(array, "vertexCount", "I").await.unwrap_or(0);
        if existing > 0 && incoming > 0 && existing != incoming {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "vertex count mismatch").await);
        }
        Ok(())
    }

    async fn read_bias(jvm: &Jvm, bias: &ClassInstanceRef<Array<f32>>, min_len: usize) -> Result<Vec<f32>> {
        if bias.is_null() {
            return Ok(vec![0.0; min_len]);
        }
        if jvm.array_length(bias).await? < min_len {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "bias array too small").await);
        }
        jvm.load_array(bias, 0, min_len).await
    }

    async fn write_scale_bias(
        jvm: &Jvm,
        mut dst: ClassInstanceRef<Array<f32>>,
        this: &ClassInstanceRef<Self>,
        scale_field: &str,
        bias_field: &str,
        required_len: usize,
    ) -> Result<()> {
        if jvm.array_length(&dst).await? < required_len {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "scale bias array too small").await);
        }
        let scale = jvm.get_field::<f32>(this, scale_field, "F").await.unwrap_or(1.0);
        let bias: ClassInstanceRef<Array<f32>> = jvm.get_field(this, bias_field, "[F").await.unwrap_or_else(|_| null_ref());
        let mut values = vec![scale];
        if bias.is_null() {
            values.resize(required_len, 0.0);
        } else {
            let mut bias_values: Vec<f32> = jvm
                .load_array(&bias, 0, jvm.array_length(&bias).await?.min(required_len.saturating_sub(1)))
                .await?;
            values.append(&mut bias_values);
            values.resize(required_len, 0.0);
        }
        jvm.store_array(&mut dst, 0, values).await
    }
}

impl Light {
    const AMBIENT: i32 = 128;
    const DIRECTIONAL: i32 = 129;
    const OMNI: i32 = 130;
    const SPOT: i32 = 131;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Light",
            parent_class: Some("javax/microedition/m3g/Node"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getColor", "()I", Self::get_color, Default::default()),
                JavaMethodProto::new("getConstantAttenuation", "()F", Self::get_constant_attenuation, Default::default()),
                JavaMethodProto::new("getIntensity", "()F", Self::get_intensity, Default::default()),
                JavaMethodProto::new("getLinearAttenuation", "()F", Self::get_linear_attenuation, Default::default()),
                JavaMethodProto::new("getMode", "()I", Self::get_mode, Default::default()),
                JavaMethodProto::new("getQuadraticAttenuation", "()F", Self::get_quadratic_attenuation, Default::default()),
                JavaMethodProto::new("getSpotAngle", "()F", Self::get_spot_angle, Default::default()),
                JavaMethodProto::new("getSpotExponent", "()F", Self::get_spot_exponent, Default::default()),
                JavaMethodProto::new("setAttenuation", "(FFF)V", Self::set_attenuation, Default::default()),
                JavaMethodProto::new("setColor", "(I)V", Self::set_color, Default::default()),
                JavaMethodProto::new("setIntensity", "(F)V", Self::set_intensity, Default::default()),
                JavaMethodProto::new("setMode", "(I)V", Self::set_mode, Default::default()),
                JavaMethodProto::new("setSpotAngle", "(F)V", Self::set_spot_angle, Default::default()),
                JavaMethodProto::new("setSpotExponent", "(F)V", Self::set_spot_exponent, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("constantAttenuation", "F", Default::default()),
                JavaFieldProto::new("linearAttenuation", "F", Default::default()),
                JavaFieldProto::new("quadraticAttenuation", "F", Default::default()),
                JavaFieldProto::new("color", "I", Default::default()),
                JavaFieldProto::new("intensity", "F", Default::default()),
                JavaFieldProto::new("mode", "I", Default::default()),
                JavaFieldProto::new("spotAngle", "F", Default::default()),
                JavaFieldProto::new("spotExponent", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Node", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "constantAttenuation", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "linearAttenuation", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "quadraticAttenuation", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "color", "I", 0x00ff_ffff).await?;
        jvm.put_field(&mut this, "intensity", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "mode", "I", Self::DIRECTIONAL).await?;
        jvm.put_field(&mut this, "spotAngle", "F", 45.0f32).await?;
        jvm.put_field(&mut this, "spotExponent", "F", 0.0f32).await
    }

    async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "color", "I").await
    }

    async fn get_constant_attenuation(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "constantAttenuation", "F").await
    }

    async fn get_intensity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "intensity", "F").await
    }

    async fn get_linear_attenuation(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "linearAttenuation", "F").await
    }

    async fn get_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "mode", "I").await
    }

    async fn get_quadratic_attenuation(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "quadraticAttenuation", "F").await
    }

    async fn get_spot_angle(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "spotAngle", "F").await
    }

    async fn get_spot_exponent(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "spotExponent", "F").await
    }

    async fn set_attenuation(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        constant: f32,
        linear: f32,
        quadratic: f32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "constantAttenuation", "F", constant).await?;
        jvm.put_field(&mut this, "linearAttenuation", "F", linear).await?;
        jvm.put_field(&mut this, "quadraticAttenuation", "F", quadratic).await
    }

    async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, color: i32) -> Result<()> {
        jvm.put_field(&mut this, "color", "I", color & 0x00ff_ffff).await
    }

    async fn set_intensity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, intensity: f32) -> Result<()> {
        jvm.put_field(&mut this, "intensity", "F", intensity).await
    }

    async fn set_mode(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, mode: i32) -> Result<()> {
        let mode = match mode {
            Self::AMBIENT | Self::DIRECTIONAL | Self::OMNI | Self::SPOT => mode,
            _ => Self::DIRECTIONAL,
        };
        jvm.put_field(&mut this, "mode", "I", mode).await
    }

    async fn set_spot_angle(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: f32) -> Result<()> {
        jvm.put_field(&mut this, "spotAngle", "F", angle).await
    }

    async fn set_spot_exponent(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, exponent: f32) -> Result<()> {
        jvm.put_field(&mut this, "spotExponent", "F", exponent).await
    }
}

impl Material {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Material",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getColor", "(I)I", Self::get_color, Default::default()),
                JavaMethodProto::new("getShininess", "()F", Self::get_shininess, Default::default()),
                JavaMethodProto::new(
                    "isVertexColorTrackingEnabled",
                    "()Z",
                    Self::is_vertex_color_tracking_enabled,
                    Default::default(),
                ),
                JavaMethodProto::new("setColor", "(II)V", Self::set_color, Default::default()),
                JavaMethodProto::new("setShininess", "(F)V", Self::set_shininess, Default::default()),
                JavaMethodProto::new(
                    "setVertexColorTrackingEnable",
                    "(Z)V",
                    Self::set_vertex_color_tracking_enable,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("ambientColor", "I", Default::default()),
                JavaFieldProto::new("diffuseColor", "I", Default::default()),
                JavaFieldProto::new("emissiveColor", "I", Default::default()),
                JavaFieldProto::new("specularColor", "I", Default::default()),
                JavaFieldProto::new("shininess", "F", Default::default()),
                JavaFieldProto::new("vertexColorTracking", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "ambientColor", "I", 0x0033_3333).await?;
        jvm.put_field(&mut this, "diffuseColor", "I", 0xffff_ffffu32 as i32).await?;
        jvm.put_field(&mut this, "emissiveColor", "I", 0).await?;
        jvm.put_field(&mut this, "specularColor", "I", 0).await?;
        jvm.put_field(&mut this, "shininess", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "vertexColorTracking", "Z", false).await
    }

    async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: i32) -> Result<i32> {
        let field = if target & 1024 != 0 {
            "ambientColor"
        } else if target & 2048 != 0 {
            "diffuseColor"
        } else if target & 4096 != 0 {
            "emissiveColor"
        } else if target & 8192 != 0 {
            "specularColor"
        } else {
            "diffuseColor"
        };
        jvm.get_field(&this, field, "I").await
    }

    async fn get_shininess(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "shininess", "F").await
    }

    async fn is_vertex_color_tracking_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "vertexColorTracking", "Z").await
    }

    async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, target: i32, color: i32) -> Result<()> {
        if target & 1024 != 0 {
            jvm.put_field(&mut this, "ambientColor", "I", ensure_opaque(color)).await?;
        }
        if target & 2048 != 0 {
            jvm.put_field(&mut this, "diffuseColor", "I", color).await?;
        }
        if target & 4096 != 0 {
            jvm.put_field(&mut this, "emissiveColor", "I", ensure_opaque(color)).await?;
        }
        if target & 8192 != 0 {
            jvm.put_field(&mut this, "specularColor", "I", ensure_opaque(color)).await?;
        }
        Ok(())
    }

    async fn set_shininess(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, shininess: f32) -> Result<()> {
        jvm.put_field(&mut this, "shininess", "F", shininess).await
    }

    async fn set_vertex_color_tracking_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, enabled: bool) -> Result<()> {
        jvm.put_field(&mut this, "vertexColorTracking", "Z", enabled).await
    }
}

impl Graphics3D {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Graphics3D",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "addLight",
                    "(Ljavax/microedition/m3g/Light;Ljavax/microedition/m3g/Transform;)I",
                    Self::add_light,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getInstance",
                    "()Ljavax/microedition/m3g/Graphics3D;",
                    Self::get_instance,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("bindTarget", "(Ljava/lang/Object;)V", Self::bind_target, Default::default()),
                JavaMethodProto::new(
                    "bindTarget",
                    "(Ljava/lang/Object;ZI)V",
                    Self::bind_target_with_options,
                    Default::default(),
                ),
                JavaMethodProto::new("clear", "(Ljavax/microedition/m3g/Background;)V", Self::clear, Default::default()),
                JavaMethodProto::new(
                    "getCamera",
                    "(Ljavax/microedition/m3g/Transform;)Ljavax/microedition/m3g/Camera;",
                    Self::get_camera,
                    Default::default(),
                ),
                JavaMethodProto::new("getDepthRangeFar", "()F", Self::get_depth_range_far, Default::default()),
                JavaMethodProto::new("getDepthRangeNear", "()F", Self::get_depth_range_near, Default::default()),
                JavaMethodProto::new("getHints", "()I", Self::get_hints, Default::default()),
                JavaMethodProto::new(
                    "getLight",
                    "(ILjavax/microedition/m3g/Transform;)Ljavax/microedition/m3g/Light;",
                    Self::get_light,
                    Default::default(),
                ),
                JavaMethodProto::new("getLightCount", "()I", Self::get_light_count, Default::default()),
                JavaMethodProto::new(
                    "getProperties",
                    "()Ljava/util/Hashtable;",
                    Self::get_properties,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getTarget", "()Ljava/lang/Object;", Self::get_target, Default::default()),
                JavaMethodProto::new("getViewportHeight", "()I", Self::get_viewport_height, Default::default()),
                JavaMethodProto::new("getViewportWidth", "()I", Self::get_viewport_width, Default::default()),
                JavaMethodProto::new("getViewportX", "()I", Self::get_viewport_x, Default::default()),
                JavaMethodProto::new("getViewportY", "()I", Self::get_viewport_y, Default::default()),
                JavaMethodProto::new("isDepthBufferEnabled", "()Z", Self::is_depth_buffer_enabled, Default::default()),
                JavaMethodProto::new("releaseTarget", "()V", Self::release_target, Default::default()),
                JavaMethodProto::new("render", "(Ljavax/microedition/m3g/World;)V", Self::render_world, Default::default()),
                JavaMethodProto::new(
                    "render",
                    "(Ljavax/microedition/m3g/Node;Ljavax/microedition/m3g/Transform;)V",
                    Self::render_node,
                    Default::default(),
                ),
                JavaMethodProto::new("resetLights", "()V", Self::reset_lights, Default::default()),
                JavaMethodProto::new(
                    "setCamera",
                    "(Ljavax/microedition/m3g/Camera;Ljavax/microedition/m3g/Transform;)V",
                    Self::set_camera,
                    Default::default(),
                ),
                JavaMethodProto::new("setDepthRange", "(FF)V", Self::set_depth_range, Default::default()),
                JavaMethodProto::new(
                    "setLight",
                    "(ILjavax/microedition/m3g/Light;Ljavax/microedition/m3g/Transform;)V",
                    Self::set_light,
                    Default::default(),
                ),
                JavaMethodProto::new("setViewport", "(IIII)V", Self::set_viewport, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("target", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("viewportX", "I", Default::default()),
                JavaFieldProto::new("viewportY", "I", Default::default()),
                JavaFieldProto::new("viewportW", "I", Default::default()),
                JavaFieldProto::new("viewportH", "I", Default::default()),
                JavaFieldProto::new("camera", "Ljavax/microedition/m3g/Camera;", Default::default()),
                JavaFieldProto::new("cameraTransform", "[F", Default::default()),
                JavaFieldProto::new("cameraTransformSet", "Z", Default::default()),
                JavaFieldProto::new("colorBuffer", "[I", Default::default()),
                JavaFieldProto::new("colorBufferValid", "Z", Default::default()),
                JavaFieldProto::new("depthBuffer", "[F", Default::default()),
                JavaFieldProto::new("depthEnabled", "Z", Default::default()),
                JavaFieldProto::new("depthRangeNear", "F", Default::default()),
                JavaFieldProto::new("depthRangeFar", "F", Default::default()),
                JavaFieldProto::new("hints", "I", Default::default()),
                JavaFieldProto::new("lights", "[Ljavax/microedition/m3g/Light;", Default::default()),
                JavaFieldProto::new("lightTransforms", "[Ljavax/microedition/m3g/Transform;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "viewportX", "I", 0).await?;
        jvm.put_field(&mut this, "viewportY", "I", 0).await?;
        jvm.put_field(&mut this, "viewportW", "I", context.screen_width()).await?;
        jvm.put_field(&mut this, "viewportH", "I", context.screen_height()).await?;
        jvm.put_field(&mut this, "camera", "Ljavax/microedition/m3g/Camera;", null_ref::<Camera>())
            .await?;
        Self::put_camera_transform(jvm, &mut this, identity_matrix()).await?;
        jvm.put_field(&mut this, "cameraTransformSet", "Z", false).await?;
        jvm.put_field(&mut this, "depthEnabled", "Z", true).await?;
        jvm.put_field(&mut this, "depthRangeNear", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "depthRangeFar", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "hints", "I", 0).await?;
        let lights = jvm.instantiate_array("Ljavax/microedition/m3g/Light;", 0).await?;
        let light_transforms = jvm.instantiate_array("Ljavax/microedition/m3g/Transform;", 0).await?;
        jvm.put_field(&mut this, "lights", "[Ljavax/microedition/m3g/Light;", lights).await?;
        jvm.put_field(&mut this, "lightTransforms", "[Ljavax/microedition/m3g/Transform;", light_transforms)
            .await?;
        Self::clear_framebuffers(jvm, &mut this).await
    }

    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        jvm.new_class("javax/microedition/m3g/Graphics3D", "()V", ()).await.map(Into::into)
    }

    async fn add_light(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        light: ClassInstanceRef<Light>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<i32> {
        if light.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Graphics3D.addLight").await);
        }
        let mut lights = Self::light_list(jvm, &this).await?;
        let mut transforms = Self::light_transform_list(jvm, &this).await?;
        let index = lights.len() as i32;
        lights.push(light);
        transforms.push(Self::transform_from_optional(jvm, transform).await?);
        Self::store_lights(jvm, &mut this, lights, transforms).await?;
        Ok(index)
    }

    async fn bind_target(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!(target: "rustjava_m3g", "m3g.Graphics3D.bindTarget target={target:?}");
        jvm.put_field(&mut this, "target", "Ljava/lang/Object;", target).await?;
        Self::clear_framebuffers(jvm, &mut this).await
    }

    async fn bind_target_with_options(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Object>,
        depth_buffer: bool,
        hints: i32,
    ) -> Result<()> {
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.Graphics3D.bindTarget target={target:?} depth={depth_buffer:?} hints={hints:?}"
        );
        jvm.put_field(&mut this, "depthEnabled", "Z", depth_buffer).await?;
        jvm.put_field(&mut this, "hints", "I", hints).await?;
        Self::bind_target(jvm, context, this, target).await
    }

    async fn clear(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, background: ClassInstanceRef<Background>) -> Result<()> {
        let target: ClassInstanceRef<Object> = jvm.get_field(&this, "target", "Ljava/lang/Object;").await?;
        if target.is_null() {
            return Ok(());
        }
        let viewport_x: i32 = jvm.get_field(&this, "viewportX", "I").await?;
        let viewport_y: i32 = jvm.get_field(&this, "viewportY", "I").await?;
        let viewport_w: i32 = jvm.get_field(&this, "viewportW", "I").await?;
        let viewport_h: i32 = jvm.get_field(&this, "viewportH", "I").await?;
        if viewport_w <= 0 || viewport_h <= 0 {
            return Ok(());
        }
        let pixel_count = (viewport_w * viewport_h) as usize;
        let mut pixels = vec![0xff00_0000u32 as i32; pixel_count];
        let mut depth_clear = true;
        if !background.is_null() {
            depth_clear = jvm.get_field(&background, "depthClear", "Z").await.unwrap_or(true);
            if jvm.get_field(&background, "colorClear", "Z").await.unwrap_or(true) {
                let color = ensure_opaque(jvm.get_field(&background, "color", "I").await.unwrap_or(0));
                pixels.fill(color);
                Self::paint_background_image(jvm, &background, viewport_w, viewport_h, color, &mut pixels).await?;
            }
        }

        let graphics: ClassInstanceRef<Graphics> = cast_ref(&target);
        Graphics::draw_pixels(jvm, context, &graphics, viewport_x, viewport_y, viewport_w, viewport_h, &pixels, false).await?;
        Self::store_color_buffer(jvm, &this, &pixels).await?;
        if depth_clear {
            let depth = vec![f32::INFINITY; pixel_count];
            Self::store_depth_buffer(jvm, &this, &depth).await?;
        }
        Ok(())
    }

    async fn get_camera(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut transform: ClassInstanceRef<Transform>,
    ) -> Result<ClassInstanceRef<Camera>> {
        if !transform.is_null() {
            let matrix = Self::camera_transform(jvm, &this).await.unwrap_or_else(|_| identity_matrix());
            Transform::put_matrix(jvm, &mut transform, matrix).await?;
        }
        jvm.get_field(&this, "camera", "Ljavax/microedition/m3g/Camera;").await
    }

    async fn get_depth_range_far(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "depthRangeFar", "F").await
    }

    async fn get_depth_range_near(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "depthRangeNear", "F").await
    }

    async fn get_hints(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "hints", "I").await
    }

    async fn get_light(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        mut transform: ClassInstanceRef<Transform>,
    ) -> Result<ClassInstanceRef<Light>> {
        let lights = Self::light_list(jvm, &this).await?;
        if index < 0 || index as usize >= lights.len() {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "light index").await);
        }
        if !transform.is_null() {
            let transforms = Self::light_transform_list(jvm, &this).await?;
            if let Some(stored) = transforms.get(index as usize) {
                let matrix = Transform::matrix(jvm, stored).await.unwrap_or_else(|_| identity_matrix());
                Transform::put_matrix(jvm, &mut transform, matrix).await?;
            }
        }
        Ok(lights[index as usize].clone())
    }

    async fn get_light_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(Self::light_list(jvm, &this).await?.len() as i32)
    }

    async fn get_properties(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Hashtable>> {
        let table: ClassInstanceRef<Hashtable> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
        Self::put_property_i32(jvm, &table, "maxLights", 8).await?;
        Self::put_property_i32(jvm, &table, "maxViewportWidth", 4096).await?;
        Self::put_property_i32(jvm, &table, "maxViewportHeight", 4096).await?;
        Self::put_property_i32(jvm, &table, "maxViewportDimension", 4096).await?;
        Self::put_property_i32(jvm, &table, "maxTextureDimension", 4096).await?;
        Self::put_property_i32(jvm, &table, "maxSpriteCropDimension", 4096).await?;
        Self::put_property_i32(jvm, &table, "numTextureUnits", 1).await?;
        Self::put_property_i32(jvm, &table, "maxTransformsPerVertex", 4).await?;
        Self::put_property_string(jvm, &table, "m3gRelease", "rustjava-0.1.0").await?;
        Ok(table)
    }

    async fn get_target(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "target", "Ljava/lang/Object;").await
    }

    async fn get_viewport_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportH", "I").await
    }

    async fn get_viewport_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportW", "I").await
    }

    async fn get_viewport_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportX", "I").await
    }

    async fn get_viewport_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportY", "I").await
    }

    async fn is_depth_buffer_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "depthEnabled", "Z").await
    }

    async fn release_target(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!(target: "rustjava_m3g", "m3g.Graphics3D.releaseTarget");
        jvm.put_field(&mut this, "target", "Ljava/lang/Object;", null_ref::<Object>()).await?;
        Self::clear_framebuffers(jvm, &mut this).await
    }

    async fn render_world(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, world: ClassInstanceRef<World>) -> Result<()> {
        tracing::trace!(target: "rustjava_m3g", "m3g.Graphics3D.render(World)");
        let background = Self::background_frame(jvm, &this, &world).await?;
        let camera: ClassInstanceRef<Camera> = jvm
            .get_field(&world, "activeCamera", "Ljavax/microedition/m3g/Camera;")
            .await
            .unwrap_or_else(|_| null_ref());
        Self::render_scene(
            jvm,
            context,
            &this,
            cast_ref::<World, Node>(&world),
            identity_matrix(),
            camera,
            None,
            Some(background),
        )
        .await
    }

    async fn render_node(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        node: ClassInstanceRef<Node>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        tracing::trace!(target: "rustjava_m3g", "m3g.Graphics3D.render(Node) node={node:?}");
        let root_matrix = if transform.is_null() {
            identity_matrix()
        } else {
            Transform::matrix(jvm, &transform).await.unwrap_or_else(|_| identity_matrix())
        };
        let camera: ClassInstanceRef<Camera> = jvm
            .get_field(&this, "camera", "Ljavax/microedition/m3g/Camera;")
            .await
            .unwrap_or_else(|_| null_ref());
        let camera_transform = if jvm.get_field(&this, "cameraTransformSet", "Z").await.unwrap_or(false) {
            Some(Self::camera_transform(jvm, &this).await.unwrap_or_else(|_| identity_matrix()))
        } else {
            None
        };
        Self::render_scene(jvm, context, &this, node, root_matrix, camera, camera_transform, None).await
    }

    async fn reset_lights(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let lights = jvm.instantiate_array("Ljavax/microedition/m3g/Light;", 0).await?;
        let transforms = jvm.instantiate_array("Ljavax/microedition/m3g/Transform;", 0).await?;
        jvm.put_field(&mut this, "lights", "[Ljavax/microedition/m3g/Light;", lights).await?;
        jvm.put_field(&mut this, "lightTransforms", "[Ljavax/microedition/m3g/Transform;", transforms)
            .await
    }

    async fn set_camera(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        camera: ClassInstanceRef<Camera>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "camera", "Ljavax/microedition/m3g/Camera;", camera).await?;
        if transform.is_null() {
            Self::put_camera_transform(jvm, &mut this, identity_matrix()).await?;
            jvm.put_field(&mut this, "cameraTransformSet", "Z", false).await
        } else {
            let matrix = Transform::matrix(jvm, &transform).await?;
            tracing::debug!(
                target: "rustjava_m3g",
                "m3g.Graphics3D.setCamera matrix tx={:.3} ty={:.3} tz={:.3} xAxis=({:.3},{:.3},{:.3}) yAxis=({:.3},{:.3},{:.3}) zAxis=({:.3},{:.3},{:.3})",
                matrix[3],
                matrix[7],
                matrix[11],
                matrix[0],
                matrix[4],
                matrix[8],
                matrix[1],
                matrix[5],
                matrix[9],
                matrix[2],
                matrix[6],
                matrix[10]
            );
            Self::put_camera_transform(jvm, &mut this, matrix).await?;
            jvm.put_field(&mut this, "cameraTransformSet", "Z", true).await
        }
    }

    async fn set_depth_range(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, near: f32, far: f32) -> Result<()> {
        if !(0.0..=1.0).contains(&near) || !(0.0..=1.0).contains(&far) || near > far {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid depth range").await);
        }
        jvm.put_field(&mut this, "depthRangeNear", "F", near).await?;
        jvm.put_field(&mut this, "depthRangeFar", "F", far).await
    }

    async fn set_light(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        index: i32,
        light: ClassInstanceRef<Light>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        let mut lights = Self::light_list(jvm, &this).await?;
        if index < 0 || index as usize >= lights.len() {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "light index").await);
        }
        let mut transforms = Self::light_transform_list(jvm, &this).await?;
        transforms.resize_with(lights.len(), null_ref::<Transform>);
        lights[index as usize] = light;
        transforms[index as usize] = Self::transform_from_optional(jvm, transform).await?;
        Self::store_lights(jvm, &mut this, lights, transforms).await
    }

    async fn set_viewport(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "viewportX", "I", x).await?;
        jvm.put_field(&mut this, "viewportY", "I", y).await?;
        jvm.put_field(&mut this, "viewportW", "I", width).await?;
        jvm.put_field(&mut this, "viewportH", "I", height).await
    }

    async fn light_list(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<ClassInstanceRef<Light>>> {
        let lights: ClassInstanceRef<Array<ClassInstanceRef<Light>>> = jvm
            .get_field(this, "lights", "[Ljavax/microedition/m3g/Light;")
            .await
            .unwrap_or_else(|_| null_ref());
        if lights.is_null() {
            return Ok(Vec::new());
        }
        jvm.load_array(&lights, 0, jvm.array_length(&lights).await?).await
    }

    async fn light_transform_list(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<ClassInstanceRef<Transform>>> {
        let transforms: ClassInstanceRef<Array<ClassInstanceRef<Transform>>> = jvm
            .get_field(this, "lightTransforms", "[Ljavax/microedition/m3g/Transform;")
            .await
            .unwrap_or_else(|_| null_ref());
        if transforms.is_null() {
            return Ok(Vec::new());
        }
        jvm.load_array(&transforms, 0, jvm.array_length(&transforms).await?).await
    }

    async fn store_lights(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        lights: Vec<ClassInstanceRef<Light>>,
        mut transforms: Vec<ClassInstanceRef<Transform>>,
    ) -> Result<()> {
        transforms.resize_with(lights.len(), null_ref::<Transform>);
        let mut light_array = jvm.instantiate_array("Ljavax/microedition/m3g/Light;", lights.len()).await?;
        jvm.store_array(&mut light_array, 0, lights).await?;
        let mut transform_array = jvm.instantiate_array("Ljavax/microedition/m3g/Transform;", transforms.len()).await?;
        jvm.store_array(&mut transform_array, 0, transforms).await?;
        jvm.put_field(this, "lights", "[Ljavax/microedition/m3g/Light;", light_array).await?;
        jvm.put_field(this, "lightTransforms", "[Ljavax/microedition/m3g/Transform;", transform_array)
            .await
    }

    async fn transform_from_optional(jvm: &Jvm, transform: ClassInstanceRef<Transform>) -> Result<ClassInstanceRef<Transform>> {
        let matrix = if transform.is_null() {
            identity_matrix()
        } else {
            Transform::matrix(jvm, &transform).await?
        };
        let mut stored: ClassInstanceRef<Transform> = jvm.new_class("javax/microedition/m3g/Transform", "()V", ()).await?.into();
        Transform::put_matrix(jvm, &mut stored, matrix).await?;
        Ok(stored)
    }

    async fn put_property_i32(jvm: &Jvm, table: &ClassInstanceRef<Hashtable>, name: &str, value: i32) -> Result<()> {
        let key: ClassInstanceRef<Object> = JavaLangString::from_rust_string(jvm, name).await?.into();
        let value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into();
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(table, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        Ok(())
    }

    async fn put_property_string(jvm: &Jvm, table: &ClassInstanceRef<Hashtable>, name: &str, value: &str) -> Result<()> {
        let key: ClassInstanceRef<Object> = JavaLangString::from_rust_string(jvm, name).await?.into();
        let value: ClassInstanceRef<Object> = JavaLangString::from_rust_string(jvm, value).await?.into();
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(table, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        Ok(())
    }

    async fn camera_transform(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let matrix: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "cameraTransform", "[F").await?;
        let values = raw_f32_array(jvm, &matrix, 16).await?;
        Ok(matrix_to_array(&values))
    }

    async fn put_camera_transform(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, matrix: [f32; 16]) -> Result<()> {
        let mut array = jvm.instantiate_array("F", 16).await?;
        jvm.store_array(&mut array, 0, matrix).await?;
        jvm.put_field(this, "cameraTransform", "[F", array).await
    }

    async fn clear_framebuffers(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(this, "colorBuffer", "[I", null_ref::<Array<i32>>()).await?;
        jvm.put_field(this, "colorBufferValid", "Z", false).await?;
        jvm.put_field(this, "depthBuffer", "[F", null_ref::<Array<f32>>()).await
    }

    async fn load_color_buffer(jvm: &Jvm, this: &ClassInstanceRef<Self>, pixel_count: usize) -> Result<(Vec<i32>, bool)> {
        let valid = jvm.get_field::<bool>(this, "colorBufferValid", "Z").await.unwrap_or(false);
        if valid {
            let array: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "colorBuffer", "[I").await.unwrap_or_else(|_| null_ref());
            if !array.is_null() && jvm.array_length(&array).await.unwrap_or(0) == pixel_count {
                return Ok((raw_i32_array(jvm, &array, pixel_count).await?, true));
            }
        }
        Ok((vec![0; pixel_count], false))
    }

    async fn store_color_buffer(jvm: &Jvm, this: &ClassInstanceRef<Self>, pixels: &[i32]) -> Result<()> {
        let mut array: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "colorBuffer", "[I").await.unwrap_or_else(|_| null_ref());
        if array.is_null() || jvm.array_length(&array).await.unwrap_or(0) != pixels.len() {
            array = jvm.instantiate_array("I", pixels.len()).await?.into();
            let mut this = this.clone();
            jvm.put_field(&mut this, "colorBuffer", "[I", array.clone()).await?;
        }
        store_raw_i32_array(jvm, &mut array, pixels).await?;
        let mut this = this.clone();
        jvm.put_field(&mut this, "colorBufferValid", "Z", true).await
    }

    async fn load_depth_buffer(jvm: &Jvm, this: &ClassInstanceRef<Self>, pixel_count: usize, clear: bool) -> Result<Vec<f32>> {
        if !clear {
            let array: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "depthBuffer", "[F").await.unwrap_or_else(|_| null_ref());
            if !array.is_null() && jvm.array_length(&array).await.unwrap_or(0) == pixel_count {
                return raw_f32_array(jvm, &array, pixel_count).await;
            }
        }
        Ok(vec![f32::INFINITY; pixel_count])
    }

    async fn store_depth_buffer(jvm: &Jvm, this: &ClassInstanceRef<Self>, depth: &[f32]) -> Result<()> {
        let mut array: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "depthBuffer", "[F").await.unwrap_or_else(|_| null_ref());
        if array.is_null() || jvm.array_length(&array).await.unwrap_or(0) != depth.len() {
            array = jvm.instantiate_array("F", depth.len()).await?.into();
            let mut this = this.clone();
            jvm.put_field(&mut this, "depthBuffer", "[F", array.clone()).await?;
        }
        store_raw_f32_array(jvm, &mut array, depth).await?;
        Ok(())
    }

    async fn render_scene(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        root: ClassInstanceRef<Node>,
        root_matrix: [f32; 16],
        camera: ClassInstanceRef<Camera>,
        camera_transform: Option<[f32; 16]>,
        background: Option<M3gBackgroundFrame>,
    ) -> Result<()> {
        if root.is_null() {
            return Ok(());
        }

        let target: ClassInstanceRef<Object> = jvm.get_field(this, "target", "Ljava/lang/Object;").await?;
        if target.is_null() {
            return Ok(());
        }

        let viewport_x: i32 = jvm.get_field(this, "viewportX", "I").await?;
        let viewport_y: i32 = jvm.get_field(this, "viewportY", "I").await?;
        let viewport_w: i32 = jvm.get_field(this, "viewportW", "I").await?;
        let viewport_h: i32 = jvm.get_field(this, "viewportH", "I").await?;
        if viewport_w <= 0 || viewport_h <= 0 {
            return Ok(());
        }
        let render_start_ms = context.now();

        let full_frame_changed = background.as_ref().is_some_and(|background| background.color_clear);
        let pixel_count = (viewport_w * viewport_h) as usize;
        let (stored_pixels, stored_pixels_valid) = Self::load_color_buffer(jvm, this, pixel_count).await?;
        let (mut pixels, process_alpha, submit_when_empty, depth_clear) = match background {
            Some(background) if background.color_clear => (
                background.pixels,
                background.process_alpha,
                background.submit_when_empty,
                background.depth_clear,
            ),
            Some(background) => (stored_pixels, !stored_pixels_valid, background.submit_when_empty, background.depth_clear),
            None => (stored_pixels, !stored_pixels_valid, false, false),
        };
        if pixels.len() != pixel_count {
            pixels.resize(pixel_count, 0);
        }
        let mut depth = Self::load_depth_buffer(jvm, this, pixel_count, depth_clear).await?;
        let graphics: ClassInstanceRef<Graphics> = cast_ref(&target);
        let view = if let Some(transform) = camera_transform {
            invert_affine_matrix(transform).unwrap_or_else(identity_matrix)
        } else {
            Self::camera_view_matrix(jvm, &camera).await.unwrap_or_else(|_| identity_matrix())
        };
        let depth_enabled: bool = jvm.get_field(this, "depthEnabled", "Z").await.unwrap_or(true);
        let depth_range_near = jvm.get_field::<f32>(this, "depthRangeNear", "F").await.unwrap_or(0.0).clamp(0.0, 1.0);
        let depth_range_far = jvm.get_field::<f32>(this, "depthRangeFar", "F").await.unwrap_or(1.0).clamp(0.0, 1.0);
        let (fovy, aspect, near, far) = if camera.is_null() {
            (45.0, viewport_w as f32 / viewport_h as f32, 1.0, 1000.0)
        } else {
            let fovy = jvm.get_field::<f32>(&camera, "fovy", "F").await.unwrap_or(45.0);
            let aspect = jvm
                .get_field(&camera, "aspect", "F")
                .await
                .unwrap_or(viewport_w as f32 / viewport_h as f32)
                .max(0.001);
            let near = jvm.get_field::<f32>(&camera, "near", "F").await.unwrap_or(1.0).max(0.001);
            let far = jvm.get_field::<f32>(&camera, "far", "F").await.unwrap_or(1000.0).max(near + 0.001);
            (fovy, aspect, near, far)
        };
        let prepare_done_ms = context.now();
        let frame = M3G_RENDER_FRAME.fetch_add(1, Ordering::Relaxed);
        let root_class = root.class_definition().name().to_string();
        let root_user_id = jvm.get_field::<i32>(&cast_ref::<Node, Object3D>(&root), "userID", "I").await.unwrap_or(0);
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.render.begin frame={} root={} userID={} viewport={}x{}+{}+{} camera={} fovy={:.3} aspect={:.3} near={:.3} far={:.3} depth={} depthRange={:.3}..{:.3} alphaSubmit={}",
            frame,
            root_class,
            root_user_id,
            viewport_w,
            viewport_h,
            viewport_x,
            viewport_y,
            if camera.is_null() { "null" } else { "active" },
            fovy,
            aspect,
            near,
            far,
            depth_enabled,
            depth_range_near,
            depth_range_far,
            process_alpha
        );

        let lighting_start_ms = context.now();
        let scene_lighting = Self::collect_scene_lighting(jvm, root.clone(), root_matrix).await?;
        let lighting_done_ms = context.now();
        let mut stats = M3gRenderStats::default();
        let meshes_start_ms = lighting_done_ms;
        let mut stack = vec![(root, root_matrix, false, true)];
        while let Some((node, parent_matrix, apply_local_transform, parent_rendering_enabled)) = stack.pop() {
            if node.is_null() {
                continue;
            }
            stats.nodes += 1;

            let rendering_enabled = jvm.get_field::<bool>(&node, "renderingEnabled", "Z").await.unwrap_or(true);
            let subtree_rendering_enabled = parent_rendering_enabled && rendering_enabled;
            let world_matrix = if apply_local_transform {
                let local_matrix = Self::node_local_matrix(jvm, &node).await.unwrap_or_else(|_| identity_matrix());
                multiply_matrix(parent_matrix, local_matrix)
            } else {
                parent_matrix
            };
            let class_name = node.class_definition().name().to_string();

            if subtree_rendering_enabled && class_name == "javax/microedition/m3g/Mesh" {
                stats.meshes += 1;
                let mesh_stats = Self::render_mesh(
                    jvm,
                    context,
                    cast_ref::<Node, Mesh>(&node),
                    frame,
                    world_matrix,
                    view,
                    fovy,
                    aspect,
                    near,
                    far,
                    depth_enabled,
                    depth_range_near,
                    depth_range_far,
                    viewport_w,
                    viewport_h,
                    &mut pixels,
                    &mut depth,
                    scene_lighting,
                )
                .await?;
                stats.vertices += mesh_stats.vertices;
                stats.projected_vertices += mesh_stats.projected_vertices;
                stats.candidate_triangles += mesh_stats.candidate_triangles;
                stats.triangles += mesh_stats.rasterized_triangles;
                stats.pixels += mesh_stats.pixels;
                stats.dirty.include_dirty(mesh_stats.dirty);
                stats.mesh_vertex_ms = stats.mesh_vertex_ms.saturating_add(mesh_stats.vertex_ms);
                stats.mesh_appearance_ms = stats.mesh_appearance_ms.saturating_add(mesh_stats.appearance_ms);
                stats.mesh_raster_ms = stats.mesh_raster_ms.saturating_add(mesh_stats.raster_ms);
                if mesh_stats.rasterized_triangles > 0 {
                    stats.rendered_meshes += 1;
                }
            }

            if subtree_rendering_enabled && (class_name == "javax/microedition/m3g/Group" || class_name == "javax/microedition/m3g/World") {
                let group: ClassInstanceRef<Group> = cast_ref(&node);
                let child_count: i32 = jvm.get_field(&group, "childCount", "I").await.unwrap_or(0);
                if child_count > 0 {
                    let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> =
                        jvm.get_field(&group, "children", "[Ljavax/microedition/m3g/Node;").await?;
                    let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
                    for child in children.into_iter().rev() {
                        stack.push((child, world_matrix, true, subtree_rendering_enabled));
                    }
                }
            }
        }
        let meshes_done_ms = context.now();

        let submitted = stats.triangles > 0 || submit_when_empty;
        let draw_start_ms = meshes_done_ms;
        if submitted {
            if full_frame_changed || submit_when_empty || stats.dirty.is_empty() {
                Graphics::draw_pixels(
                    jvm,
                    context,
                    &graphics,
                    viewport_x,
                    viewport_y,
                    viewport_w,
                    viewport_h,
                    &pixels,
                    process_alpha,
                )
                .await?;
            } else if let Some((dirty_x, dirty_y, dirty_w, dirty_h)) = stats.dirty.bounds() {
                Graphics::draw_pixels_strided(
                    jvm,
                    context,
                    &graphics,
                    viewport_x + dirty_x,
                    viewport_y + dirty_y,
                    dirty_w,
                    dirty_h,
                    &pixels,
                    viewport_w,
                    dirty_x,
                    dirty_y,
                    process_alpha,
                )
                .await?;
            }
        }
        let draw_done_ms = context.now();
        let buffers_changed = stats.triangles > 0 || full_frame_changed || depth_clear;
        if buffers_changed {
            Self::store_color_buffer(jvm, this, &pixels).await?;
            Self::store_depth_buffer(jvm, this, &depth).await?;
        }
        let now_ms = context.now();
        let prepare_ms = prepare_done_ms.saturating_sub(render_start_ms);
        let lighting_ms = lighting_done_ms.saturating_sub(lighting_start_ms);
        let meshes_ms = meshes_done_ms.saturating_sub(meshes_start_ms);
        let draw_ms = draw_done_ms.saturating_sub(draw_start_ms);
        let store_ms = now_ms.saturating_sub(draw_done_ms);
        let total_ms = now_ms.saturating_sub(render_start_ms);
        publish_render_diagnostic(
            now_ms,
            total_ms,
            format!(
                "f={frame} total={total_ms}ms prep={prepare_ms} light={lighting_ms} mesh={meshes_ms} draw={draw_ms} store={store_ms} | meshParts v={} app={} rast={} | nodes={} meshes={}/{} verts={}/{} tri={}/{} px={} dirty={:?}",
                stats.mesh_vertex_ms,
                stats.mesh_appearance_ms,
                stats.mesh_raster_ms,
                stats.nodes,
                stats.rendered_meshes,
                stats.meshes,
                stats.projected_vertices,
                stats.vertices,
                stats.triangles,
                stats.candidate_triangles,
                stats.pixels,
                stats.dirty.bounds()
            ),
        );
        let previous_ms = M3G_RENDER_LAST_END_MS.swap(now_ms, Ordering::Relaxed);
        if previous_ms != 0 {
            let gap_ms = now_ms.saturating_sub(previous_ms);
            if gap_ms >= 50 {
                tracing::warn!(
                    target: "rustjava_m3g",
                    "m3g.render.slow_gap gapMs={} frame={} nodes={} meshes={} renderedMeshes={} triangles={} pixels={} submitted={} dirty={:?}",
                    gap_ms,
                    frame,
                    stats.nodes,
                    stats.meshes,
                    stats.rendered_meshes,
                    stats.triangles,
                    stats.pixels,
                    submitted,
                    stats.dirty.bounds()
                );
            }
        }
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.render.end frame={} nodes={} meshes={} renderedMeshes={} triangles={} pixels={} submitted={} buffersChanged={} dirty={:?}",
            frame,
            stats.nodes,
            stats.meshes,
            stats.rendered_meshes,
            stats.triangles,
            stats.pixels,
            submitted,
            buffers_changed,
            stats.dirty.bounds()
        );
        Ok(())
    }

    async fn collect_scene_lighting(jvm: &Jvm, root: ClassInstanceRef<Node>, root_matrix: [f32; 16]) -> Result<M3gSceneLighting> {
        let mut lighting = M3gSceneLighting::default();
        let mut stack = vec![(root, root_matrix, false, true)];
        while let Some((node, parent_matrix, apply_local_transform, parent_rendering_enabled)) = stack.pop() {
            if node.is_null() {
                continue;
            }

            let rendering_enabled = jvm.get_field::<bool>(&node, "renderingEnabled", "Z").await.unwrap_or(true);
            let subtree_rendering_enabled = parent_rendering_enabled && rendering_enabled;
            let world_matrix = if apply_local_transform {
                let local_matrix = Self::node_local_matrix(jvm, &node).await.unwrap_or_else(|_| identity_matrix());
                multiply_matrix(parent_matrix, local_matrix)
            } else {
                parent_matrix
            };
            let class_name = node.class_definition().name().to_string();

            if subtree_rendering_enabled && class_name == "javax/microedition/m3g/Light" {
                Self::add_light_to_scene(jvm, cast_ref(&node), &mut lighting).await?;
            }

            if subtree_rendering_enabled && (class_name == "javax/microedition/m3g/Group" || class_name == "javax/microedition/m3g/World") {
                let group: ClassInstanceRef<Group> = cast_ref(&node);
                let child_count: i32 = jvm.get_field(&group, "childCount", "I").await.unwrap_or(0);
                if child_count > 0 {
                    let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> =
                        jvm.get_field(&group, "children", "[Ljavax/microedition/m3g/Node;").await?;
                    let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
                    for child in children.into_iter().rev() {
                        stack.push((child, world_matrix, true, subtree_rendering_enabled));
                    }
                }
            }
        }
        Ok(lighting)
    }

    async fn add_light_to_scene(jvm: &Jvm, light: ClassInstanceRef<Light>, lighting: &mut M3gSceneLighting) -> Result<()> {
        let mode = jvm.get_field::<i32>(&light, "mode", "I").await.unwrap_or(Light::DIRECTIONAL);
        if mode != Light::AMBIENT {
            return Ok(());
        }

        let color = jvm.get_field::<i32>(&light, "color", "I").await.unwrap_or(0x00ff_ffff);
        let intensity = jvm.get_field::<f32>(&light, "intensity", "F").await.unwrap_or(1.0).max(0.0);
        lighting.ambient_r += color_channel_f32(color, 16) * intensity;
        lighting.ambient_g += color_channel_f32(color, 8) * intensity;
        lighting.ambient_b += color_channel_f32(color, 0) * intensity;
        lighting.ambient_lights += 1;
        Ok(())
    }

    async fn render_mesh(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mesh: ClassInstanceRef<Mesh>,
        frame: u64,
        world_matrix: [f32; 16],
        view_matrix: [f32; 16],
        fovy: f32,
        aspect: f32,
        near: f32,
        far: f32,
        depth_enabled: bool,
        depth_range_near: f32,
        depth_range_far: f32,
        viewport_w: i32,
        viewport_h: i32,
        pixels: &mut [i32],
        depth: &mut [f32],
        scene_lighting: M3gSceneLighting,
    ) -> Result<M3gMeshRenderStats> {
        let mut stats = M3gMeshRenderStats::default();
        let vertex_start_ms = context.now();
        let user_id = jvm.get_field::<i32>(&cast_ref::<Mesh, Object3D>(&mesh), "userID", "I").await.unwrap_or(0);
        let vertex_buffer: ClassInstanceRef<VertexBuffer> = jvm.get_field(&mesh, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;").await?;
        if vertex_buffer.is_null() {
            tracing::debug!(target: "rustjava_m3g", "m3g.render.mesh frame={} userID={} skipped=noVertexBuffer", frame, user_id);
            return Ok(stats);
        }

        let positions_ref: ClassInstanceRef<VertexArray> = jvm.get_field(&vertex_buffer, "positions", "Ljavax/microedition/m3g/VertexArray;").await?;
        let position_scale = jvm.get_field::<f32>(&vertex_buffer, "positionScale", "F").await.unwrap_or(1.0);
        let position_bias = Self::float_array_field(jvm, &vertex_buffer, "positionBias", 3, 0.0).await?;
        let positions = Self::vertex_array_vec3(jvm, &positions_ref, position_scale, &position_bias).await?;
        stats.vertices = positions.len();
        if positions.is_empty() {
            tracing::debug!(target: "rustjava_m3g", "m3g.render.mesh frame={} userID={} skipped=noPositions", frame, user_id);
            return Ok(stats);
        }

        let tex_coords_ref: ClassInstanceRef<VertexArray> = jvm
            .get_field(&vertex_buffer, "texCoords0", "Ljavax/microedition/m3g/VertexArray;")
            .await?;
        let tex_scale = jvm.get_field::<f32>(&vertex_buffer, "texScale", "F").await.unwrap_or(1.0);
        let tex_bias = Self::float_array_field(jvm, &vertex_buffer, "texBias", 3, 0.0).await?;
        let tex_coords = Self::vertex_array_vec2(jvm, &tex_coords_ref, tex_scale, &tex_bias).await?;

        let default_color = ensure_opaque(jvm.get_field(&vertex_buffer, "defaultColor", "I").await.unwrap_or(0x00ff_ffff));
        let colors_ref: ClassInstanceRef<VertexArray> = jvm.get_field(&vertex_buffer, "colors", "Ljavax/microedition/m3g/VertexArray;").await?;
        let colors = Self::vertex_array_colors(jvm, &colors_ref, default_color).await?;
        let has_vertex_colors = !colors.is_empty();

        let model_view = multiply_matrix(view_matrix, world_matrix);
        let mut camera_vertices = Vec::with_capacity(positions.len());
        for (index, position) in positions.iter().copied().enumerate() {
            let uv = tex_coords.get(index).copied().unwrap_or([0.0, 0.0]);
            let color = colors.get(index).copied().unwrap_or(default_color);
            camera_vertices.push(camera_vertex(position, uv, color, model_view));
        }
        stats.projected_vertices = camera_vertices
            .iter()
            .filter(|vertex| vertex.is_some_and(|vertex| vertex.depth > near))
            .count();
        let vertex_done_ms = context.now();
        stats.vertex_ms = vertex_done_ms.saturating_sub(vertex_start_ms);

        let submesh_count: i32 = jvm.get_field(&mesh, "submeshCount", "I").await.unwrap_or(0);
        if submesh_count <= 0 {
            tracing::debug!(
                target: "rustjava_m3g",
                "m3g.render.mesh frame={} userID={} vertices={} projected={} skipped=noSubmeshes",
                frame,
                user_id,
                stats.vertices,
                stats.projected_vertices
            );
            return Ok(stats);
        }
        stats.submeshes = submesh_count as usize;
        let index_buffers: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> =
            jvm.get_field(&mesh, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;").await?;
        let appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> =
            jvm.get_field(&mesh, "appearances", "[Ljavax/microedition/m3g/Appearance;").await?;
        let index_buffers: Vec<ClassInstanceRef<IndexBuffer>> = jvm.load_array(&index_buffers, 0, submesh_count as usize).await?;
        let appearances: Vec<ClassInstanceRef<Appearance>> = jvm.load_array(&appearances, 0, submesh_count as usize).await?;

        let mut draw_items = Vec::new();
        for (submesh, index_buffer) in index_buffers.into_iter().enumerate() {
            if index_buffer.is_null() {
                continue;
            }
            let appearance_state = if let Some(appearance) = appearances.get(submesh) {
                Self::appearance_state(jvm, appearance, default_color, scene_lighting).await?
            } else {
                M3gAppearanceState {
                    texture: None,
                    base_color: default_color,
                    layer: 0,
                    color_write: true,
                    alpha_write: true,
                    depth_test: true,
                    depth_write: true,
                    depth_offset_factor: 0.0,
                    depth_offset_units: 0.0,
                    alpha_threshold: 0.0,
                    blending: CompositingMode::REPLACE,
                    culling: PolygonMode::CULL_BACK,
                    winding: PolygonMode::WINDING_CCW,
                    perspective_correction: true,
                }
            };
            if appearance_state.texture.is_some() {
                stats.textured_submeshes += 1;
            }
            draw_items.push(M3gSubmeshDraw {
                sort_key: appearance_sort_key(&appearance_state),
                sequence: submesh,
                index_buffer,
                appearance: appearance_state,
            });
        }
        draw_items.sort_by_key(|item| (item.sort_key, item.sequence));
        let appearance_done_ms = context.now();
        stats.appearance_ms = appearance_done_ms.saturating_sub(vertex_done_ms);

        for draw_item in draw_items {
            let triangles = Self::triangle_indices(jvm, cast_ref::<IndexBuffer, TriangleStripArray>(&draw_item.index_buffer)).await?;
            stats.candidate_triangles += triangles.len();
            for [i0, i1, i2] in triangles.iter().copied() {
                let Some(mut v0) = camera_vertices.get(i0).and_then(|v| *v) else {
                    continue;
                };
                let Some(mut v1) = camera_vertices.get(i1).and_then(|v| *v) else {
                    continue;
                };
                let Some(mut v2) = camera_vertices.get(i2).and_then(|v| *v) else {
                    continue;
                };
                if !has_vertex_colors {
                    v0.color = draw_item.appearance.base_color;
                    v1.color = draw_item.appearance.base_color;
                    v2.color = draw_item.appearance.base_color;
                }
                let clipped = clip_triangle_near(v0, v1, v2, near);
                if clipped.len() < 3 {
                    continue;
                }

                for triangle_index in 1..clipped.len() - 1 {
                    let Some(v0) = project_camera_vertex(clipped[0], fovy, aspect, viewport_w, viewport_h) else {
                        continue;
                    };
                    let Some(v1) = project_camera_vertex(clipped[triangle_index], fovy, aspect, viewport_w, viewport_h) else {
                        continue;
                    };
                    let Some(v2) = project_camera_vertex(clipped[triangle_index + 1], fovy, aspect, viewport_w, viewport_h) else {
                        continue;
                    };
                    let drawn_pixels = rasterize_triangle(
                        pixels,
                        depth,
                        viewport_w,
                        viewport_h,
                        v0,
                        v1,
                        v2,
                        &draw_item.appearance,
                        depth_enabled,
                        near,
                        far,
                        depth_range_near,
                        depth_range_far,
                        &mut stats.dirty,
                    );
                    if drawn_pixels > 0 {
                        stats.rasterized_triangles += 1;
                        stats.pixels += drawn_pixels;
                    }
                }
            }
        }
        stats.raster_ms = context.now().saturating_sub(appearance_done_ms);
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.render.mesh frame={} userID={} vertices={} projected={} submeshes={} texturedSubmeshes={} candidateTriangles={} rasterizedTriangles={} pixels={}",
            frame,
            user_id,
            stats.vertices,
            stats.projected_vertices,
            stats.submeshes,
            stats.textured_submeshes,
            stats.candidate_triangles,
            stats.rasterized_triangles,
            stats.pixels
        );
        Ok(stats)
    }

    async fn node_local_matrix(jvm: &Jvm, node: &ClassInstanceRef<Node>) -> Result<[f32; 16]> {
        Transformable::local_matrix(jvm, &cast_ref::<Node, Transformable>(node)).await
    }

    async fn camera_view_matrix(jvm: &Jvm, camera: &ClassInstanceRef<Camera>) -> Result<[f32; 16]> {
        if camera.is_null() {
            return Ok(identity_matrix());
        }

        let mut chain = Vec::new();
        let mut current: ClassInstanceRef<Node> = cast_ref(camera);
        while !current.is_null() {
            if current.class_definition().name() == "javax/microedition/m3g/World" {
                break;
            }
            chain.push(current.clone());
            current = jvm
                .get_field(&current, "parent", "Ljavax/microedition/m3g/Node;")
                .await
                .unwrap_or_else(|_| null_ref());
        }

        let mut world = identity_matrix();
        for node in chain.into_iter().rev() {
            let local = Self::node_local_matrix(jvm, &node).await.unwrap_or_else(|_| identity_matrix());
            world = multiply_matrix(world, local);
        }
        Ok(invert_affine_matrix(world).unwrap_or_else(identity_matrix))
    }

    async fn float_array_field(jvm: &Jvm, vertex_buffer: &ClassInstanceRef<VertexBuffer>, name: &str, len: usize, default: f32) -> Result<Vec<f32>> {
        let array: ClassInstanceRef<Array<f32>> = jvm.get_field(vertex_buffer, name, "[F").await.unwrap_or_else(|_| null_ref());
        if array.is_null() {
            return Ok(vec![default; len]);
        }
        let mut values = raw_f32_array(jvm, &array, len).await?;
        values.resize(len, default);
        Ok(values)
    }

    async fn vertex_array_values(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>) -> Result<(usize, usize, Arc<Vec<f32>>)> {
        if array.is_null() {
            return Ok((0, 0, Arc::new(Vec::new())));
        }
        let key = instance_key(array);
        let version = jvm.get_field::<i32>(array, "version", "I").await.unwrap_or(0);
        if key != 0 {
            let cache = M3G_VERTEX_VALUES_CACHE.lock();
            if let Some(entry) = cache.iter().find(|entry| entry.key == key && entry.version == version) {
                return Ok((entry.component_count, entry.vertex_count, entry.values.clone()));
            }
        }
        let component_size: i32 = jvm.get_field(array, "componentSize", "I").await?;
        let component_count: i32 = jvm.get_field(array, "componentCount", "I").await?;
        let vertex_count: i32 = jvm.get_field(array, "vertexCount", "I").await?;
        if component_count <= 0 || vertex_count <= 0 {
            return Ok((0, 0, Arc::new(Vec::new())));
        }
        let component_count = component_count as usize;
        let vertex_count = vertex_count as usize;
        let value_count = component_count.saturating_mul(vertex_count);
        let values: Vec<f32> = if component_size == 1 {
            let data: ClassInstanceRef<Array<i8>> = jvm.get_field(array, "byteData", "[B").await?;
            if data.is_null() {
                return Ok((component_count, vertex_count, Arc::new(Vec::new())));
            }
            raw_u8_array(jvm, &data, value_count)
                .await?
                .into_iter()
                .map(|value| value as i8 as f32)
                .collect()
        } else {
            let data: ClassInstanceRef<Array<i16>> = jvm.get_field(array, "shortData", "[S").await?;
            if data.is_null() {
                return Ok((component_count, vertex_count, Arc::new(Vec::new())));
            }
            raw_i16_array(jvm, &data, value_count)
                .await?
                .into_iter()
                .map(|value| value as f32)
                .collect()
        };
        let values = Arc::new(values);
        if key != 0 {
            let mut cache = M3G_VERTEX_VALUES_CACHE.lock();
            push_cache_entry(
                &mut cache,
                VertexValuesCacheEntry {
                    key,
                    version,
                    component_count,
                    vertex_count,
                    values: values.clone(),
                },
            );
        }
        Ok((component_count, vertex_count, values))
    }

    async fn vertex_array_vec3(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>, scale: f32, bias: &[f32]) -> Result<Arc<Vec<[f32; 3]>>> {
        let key = instance_key(array);
        let version = if array.is_null() {
            0
        } else {
            jvm.get_field::<i32>(array, "version", "I").await.unwrap_or(0)
        };
        let bias_x = bias.first().copied().unwrap_or(0.0);
        let bias_y = bias.get(1).copied().unwrap_or(0.0);
        let bias_z = bias.get(2).copied().unwrap_or(0.0);
        let scale_bits = scale.to_bits();
        let bias_bits = [bias_x.to_bits(), bias_y.to_bits(), bias_z.to_bits()];
        if key != 0 {
            let cache = M3G_VERTEX_VEC3_CACHE.lock();
            if let Some(entry) = cache
                .iter()
                .find(|entry| entry.key == key && entry.version == version && entry.scale_bits == scale_bits && entry.bias_bits == bias_bits)
            {
                return Ok(entry.values.clone());
            }
        }
        let (component_count, vertex_count, values) = Self::vertex_array_values(jvm, array).await?;
        let mut result = Vec::with_capacity(vertex_count);
        for vertex in 0..vertex_count {
            let base = vertex * component_count;
            result.push([
                values.get(base).copied().unwrap_or(0.0) * scale + bias_x,
                values.get(base + 1).copied().unwrap_or(0.0) * scale + bias_y,
                values.get(base + 2).copied().unwrap_or(0.0) * scale + bias_z,
            ]);
        }
        let result = Arc::new(result);
        if key != 0 {
            let mut cache = M3G_VERTEX_VEC3_CACHE.lock();
            push_cache_entry(
                &mut cache,
                VertexVec3CacheEntry {
                    key,
                    version,
                    scale_bits,
                    bias_bits,
                    values: result.clone(),
                },
            );
        }
        Ok(result)
    }

    async fn vertex_array_vec2(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>, scale: f32, bias: &[f32]) -> Result<Arc<Vec<[f32; 2]>>> {
        let key = instance_key(array);
        let version = if array.is_null() {
            0
        } else {
            jvm.get_field::<i32>(array, "version", "I").await.unwrap_or(0)
        };
        let bias_x = bias.first().copied().unwrap_or(0.0);
        let bias_y = bias.get(1).copied().unwrap_or(0.0);
        let scale_bits = scale.to_bits();
        let bias_bits = [bias_x.to_bits(), bias_y.to_bits()];
        if key != 0 {
            let cache = M3G_VERTEX_VEC2_CACHE.lock();
            if let Some(entry) = cache
                .iter()
                .find(|entry| entry.key == key && entry.version == version && entry.scale_bits == scale_bits && entry.bias_bits == bias_bits)
            {
                return Ok(entry.values.clone());
            }
        }
        let (component_count, vertex_count, values) = Self::vertex_array_values(jvm, array).await?;
        let mut result = Vec::with_capacity(vertex_count);
        for vertex in 0..vertex_count {
            let base = vertex * component_count;
            result.push([
                values.get(base).copied().unwrap_or(0.0) * scale + bias_x,
                values.get(base + 1).copied().unwrap_or(0.0) * scale + bias_y,
            ]);
        }
        let result = Arc::new(result);
        if key != 0 {
            let mut cache = M3G_VERTEX_VEC2_CACHE.lock();
            push_cache_entry(
                &mut cache,
                VertexVec2CacheEntry {
                    key,
                    version,
                    scale_bits,
                    bias_bits,
                    values: result.clone(),
                },
            );
        }
        Ok(result)
    }

    async fn vertex_array_colors(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>, default_color: i32) -> Result<Arc<Vec<i32>>> {
        let key = instance_key(array);
        let version = if array.is_null() {
            0
        } else {
            jvm.get_field::<i32>(array, "version", "I").await.unwrap_or(0)
        };
        if key != 0 {
            let cache = M3G_VERTEX_COLOR_CACHE.lock();
            if let Some(entry) = cache
                .iter()
                .find(|entry| entry.key == key && entry.version == version && entry.default_color == default_color)
            {
                return Ok(entry.values.clone());
            }
        }
        let (component_count, vertex_count, values) = Self::vertex_array_values(jvm, array).await?;
        if component_count < 3 {
            let result = Arc::new(vec![default_color; vertex_count]);
            if key != 0 {
                let mut cache = M3G_VERTEX_COLOR_CACHE.lock();
                push_cache_entry(
                    &mut cache,
                    VertexColorCacheEntry {
                        key,
                        version,
                        default_color,
                        values: result.clone(),
                    },
                );
            }
            return Ok(result);
        }
        let mut result = Vec::with_capacity(vertex_count);
        for vertex in 0..vertex_count {
            let base = vertex * component_count;
            let r = clamp_color(values.get(base).copied().unwrap_or(0.0));
            let g = clamp_color(values.get(base + 1).copied().unwrap_or(0.0));
            let b = clamp_color(values.get(base + 2).copied().unwrap_or(0.0));
            let a = values.get(base + 3).map(|v| clamp_color(*v)).unwrap_or(255);
            result.push(((a << 24) | (r << 16) | (g << 8) | b) as i32);
        }
        let result = Arc::new(result);
        if key != 0 {
            let mut cache = M3G_VERTEX_COLOR_CACHE.lock();
            push_cache_entry(
                &mut cache,
                VertexColorCacheEntry {
                    key,
                    version,
                    default_color,
                    values: result.clone(),
                },
            );
        }
        Ok(result)
    }

    async fn vertex_array_components(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>) -> Result<Vec<Vec<f32>>> {
        let (component_count, vertex_count, values) = Self::vertex_array_values(jvm, array).await?;
        if component_count == 0 {
            return Ok(Vec::new());
        }
        let mut result = Vec::with_capacity(vertex_count);
        for chunk in values.chunks(component_count) {
            result.push(chunk.to_vec());
        }
        Ok(result)
    }

    async fn triangle_indices(jvm: &Jvm, array: ClassInstanceRef<TriangleStripArray>) -> Result<Arc<Vec<[usize; 3]>>> {
        if array.is_null() {
            return Ok(Arc::new(Vec::new()));
        }
        let key = instance_key(&array);
        if key != 0 {
            let cache = M3G_TRIANGLE_INDEX_CACHE.lock();
            if let Some(entry) = cache.iter().find(|entry| entry.key == key) {
                return Ok(entry.values.clone());
            }
        }
        let triangle_cache: ClassInstanceRef<Array<i32>> = jvm.get_field(&array, "triangleCache", "[I").await.unwrap_or_else(|_| null_ref());
        if !triangle_cache.is_null() {
            let values = raw_i32_array(jvm, &triangle_cache, jvm.array_length(&triangle_cache).await?).await?;
            let triangles: Arc<Vec<[usize; 3]>> = Arc::new(
                values
                    .chunks_exact(3)
                    .map(|chunk| [chunk[0].max(0) as usize, chunk[1].max(0) as usize, chunk[2].max(0) as usize])
                    .collect(),
            );
            if key != 0 {
                let mut cache = M3G_TRIANGLE_INDEX_CACHE.lock();
                push_cache_entry(
                    &mut cache,
                    TriangleIndexCacheEntry {
                        key,
                        values: triangles.clone(),
                    },
                );
            }
            return Ok(triangles);
        }

        let indices_array: ClassInstanceRef<Array<i32>> = jvm.get_field(&array, "indices", "[I").await?;
        let lengths_array: ClassInstanceRef<Array<i32>> = jvm.get_field(&array, "stripLengths", "[I").await?;
        if indices_array.is_null() || lengths_array.is_null() {
            return Ok(Arc::new(Vec::new()));
        }
        let indices = raw_i32_array(jvm, &indices_array, jvm.array_length(&indices_array).await?).await?;
        let lengths = raw_i32_array(jvm, &lengths_array, jvm.array_length(&lengths_array).await?).await?;
        let triangles = expand_triangle_strips(&indices, &lengths);
        let mut flat = Vec::with_capacity(triangles.len() * 3);
        for triangle in &triangles {
            flat.push(triangle[0] as i32);
            flat.push(triangle[1] as i32);
            flat.push(triangle[2] as i32);
        }
        let mut cache = jvm.instantiate_array("I", flat.len()).await?;
        store_raw_i32_array(jvm, &mut cache, &flat).await?;
        let mut array = array.clone();
        jvm.put_field(&mut array, "triangleCache", "[I", cache).await?;
        let triangles = Arc::new(triangles);
        if key != 0 {
            let mut cache = M3G_TRIANGLE_INDEX_CACHE.lock();
            push_cache_entry(
                &mut cache,
                TriangleIndexCacheEntry {
                    key,
                    values: triangles.clone(),
                },
            );
        }
        Ok(triangles)
    }

    async fn image_pixels_cached(jvm: &Jvm, image: &ClassInstanceRef<Image>) -> Result<(i32, i32, Arc<Vec<i32>>)> {
        let (width, height, pixels) = Image::pixels(jvm, image).await?;
        let key = instance_key(&pixels);
        if key != 0 {
            let cache = M3G_IMAGE_PIXEL_CACHE.lock();
            if let Some(entry) = cache
                .iter()
                .find(|entry| entry.key == key && entry.width == width && entry.height == height)
            {
                return Ok((width, height, entry.values.clone()));
            }
        }
        let values = Arc::new(raw_i32_array(jvm, &pixels, (width * height) as usize).await?);
        if key != 0 {
            let mut cache = M3G_IMAGE_PIXEL_CACHE.lock();
            push_cache_entry(
                &mut cache,
                ImagePixelCacheEntry {
                    key,
                    width,
                    height,
                    values: values.clone(),
                },
            );
        }
        Ok((width, height, values))
    }

    async fn appearance_state(
        jvm: &Jvm,
        appearance: &ClassInstanceRef<Appearance>,
        default_color: i32,
        scene_lighting: M3gSceneLighting,
    ) -> Result<M3gAppearanceState> {
        let mut state = M3gAppearanceState {
            texture: None,
            base_color: default_color,
            layer: 0,
            color_write: true,
            alpha_write: true,
            depth_test: true,
            depth_write: true,
            depth_offset_factor: 0.0,
            depth_offset_units: 0.0,
            alpha_threshold: 0.0,
            blending: CompositingMode::REPLACE,
            culling: PolygonMode::CULL_BACK,
            winding: PolygonMode::WINDING_CCW,
            perspective_correction: true,
        };
        if appearance.is_null() {
            return Ok(state);
        }
        state.layer = jvm.get_field(appearance, "layer", "I").await.unwrap_or(0);

        let material: ClassInstanceRef<Material> = jvm
            .get_field(appearance, "material", "Ljavax/microedition/m3g/Material;")
            .await
            .unwrap_or_else(|_| null_ref());
        let material_colors = if material.is_null() {
            None
        } else {
            let ambient = jvm.get_field(&material, "ambientColor", "I").await.unwrap_or(0x0033_3333);
            let diffuse = jvm.get_field(&material, "diffuseColor", "I").await.unwrap_or(default_color);
            let emissive = jvm.get_field(&material, "emissiveColor", "I").await.unwrap_or(0);
            Some((ambient, diffuse, emissive))
        };

        let compositing_mode: ClassInstanceRef<CompositingMode> = jvm
            .get_field(appearance, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;")
            .await
            .unwrap_or_else(|_| null_ref());
        if !compositing_mode.is_null() {
            state.color_write = jvm.get_field(&compositing_mode, "colorWrite", "Z").await.unwrap_or(true);
            state.alpha_write = jvm.get_field(&compositing_mode, "alphaWrite", "Z").await.unwrap_or(true);
            state.depth_test = jvm.get_field(&compositing_mode, "depthTest", "Z").await.unwrap_or(true);
            state.depth_write = jvm.get_field(&compositing_mode, "depthWrite", "Z").await.unwrap_or(true);
            state.depth_offset_factor = jvm.get_field::<f32>(&compositing_mode, "depthOffsetFactor", "F").await.unwrap_or(0.0);
            state.depth_offset_units = jvm.get_field::<f32>(&compositing_mode, "depthOffsetUnits", "F").await.unwrap_or(0.0);
            state.alpha_threshold = jvm.get_field::<f32>(&compositing_mode, "alphaThreshold", "F").await.unwrap_or(0.0);
            state.blending = jvm
                .get_field(&compositing_mode, "blending", "I")
                .await
                .unwrap_or(CompositingMode::REPLACE);
        }

        let polygon_mode: ClassInstanceRef<PolygonMode> = jvm
            .get_field(appearance, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;")
            .await
            .unwrap_or_else(|_| null_ref());
        if !polygon_mode.is_null() {
            state.culling = jvm.get_field(&polygon_mode, "culling", "I").await.unwrap_or(PolygonMode::CULL_BACK);
            state.winding = jvm.get_field(&polygon_mode, "winding", "I").await.unwrap_or(PolygonMode::WINDING_CCW);
            state.perspective_correction = jvm.get_field(&polygon_mode, "perspectiveCorrection", "Z").await.unwrap_or(true);
        }

        let texture: ClassInstanceRef<Texture2D> = jvm
            .get_field(appearance, "texture0", "Ljavax/microedition/m3g/Texture2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        if texture.is_null() {
            state.base_color = material_base_color(material_colors, default_color, scene_lighting);
            return Ok(state);
        }
        let image2d: ClassInstanceRef<Image2D> = jvm
            .get_field(&texture, "image", "Ljavax/microedition/m3g/Image2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image2d.is_null() {
            state.base_color = material_base_color(material_colors, default_color, scene_lighting);
            return Ok(state);
        }
        let format = jvm.get_field(&image2d, "format", "I").await.unwrap_or(Image2D::RGBA);
        let image: ClassInstanceRef<Image> = jvm
            .get_field(&image2d, "image", "Ljavax/microedition/lcdui/Image;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image.is_null() {
            state.base_color = material_base_color(material_colors, default_color, scene_lighting);
            return Ok(state);
        }
        let (width, height, pixels) = Self::image_pixels_cached(jvm, &image).await?;
        let blend_color = jvm.get_field(&texture, "blendColor", "I").await.unwrap_or(0);
        let blending = jvm.get_field(&texture, "blending", "I").await.unwrap_or(Texture2D::FUNC_MODULATE);
        let wrap_s = jvm.get_field(&texture, "wrappingS", "I").await.unwrap_or(Texture2D::WRAP_REPEAT);
        let wrap_t = jvm.get_field(&texture, "wrappingT", "I").await.unwrap_or(Texture2D::WRAP_REPEAT);
        let image_filter = jvm.get_field(&texture, "imageFilter", "I").await.unwrap_or(Texture2D::FILTER_NEAREST);
        let transform = Transformable::local_matrix(jvm, &cast_ref(&texture))
            .await
            .unwrap_or_else(|_| identity_matrix());
        let transform_identity = texture_transform_is_identity(transform);
        state.texture = Some(M3gTexture {
            width,
            height,
            pixels,
            format,
            blend_color,
            blending,
            wrap_s,
            wrap_t,
            image_filter,
            transform,
            transform_identity,
        });
        state.base_color = material_base_color(material_colors, default_color, scene_lighting);
        Ok(state)
    }

    async fn background_frame(jvm: &Jvm, this: &ClassInstanceRef<Self>, world: &ClassInstanceRef<World>) -> Result<M3gBackgroundFrame> {
        let viewport_w: i32 = jvm.get_field(this, "viewportW", "I").await?;
        let viewport_h: i32 = jvm.get_field(this, "viewportH", "I").await?;
        let pixel_count = (viewport_w.max(0) * viewport_h.max(0)) as usize;
        let background: ClassInstanceRef<Background> = jvm
            .get_field(world, "background", "Ljavax/microedition/m3g/Background;")
            .await
            .unwrap_or_else(|_| null_ref());
        if background.is_null() {
            return Ok(M3gBackgroundFrame {
                pixels: vec![0xff00_0000u32 as i32; pixel_count],
                color_clear: true,
                depth_clear: true,
                process_alpha: false,
                submit_when_empty: true,
            });
        }

        let color_clear: bool = jvm.get_field(&background, "colorClear", "Z").await.unwrap_or(true);
        let depth_clear: bool = jvm.get_field(&background, "depthClear", "Z").await.unwrap_or(true);
        if !color_clear {
            return Ok(M3gBackgroundFrame {
                pixels: vec![0; pixel_count],
                color_clear: false,
                depth_clear,
                process_alpha: true,
                submit_when_empty: false,
            });
        }

        let color = ensure_opaque(jvm.get_field(&background, "color", "I").await.unwrap_or(0));
        let mut pixels = vec![color; pixel_count];
        Self::paint_background_image(jvm, &background, viewport_w, viewport_h, color, &mut pixels).await?;
        Ok(M3gBackgroundFrame {
            pixels,
            color_clear: true,
            depth_clear,
            process_alpha: false,
            submit_when_empty: true,
        })
    }

    async fn paint_background_image(
        jvm: &Jvm,
        background: &ClassInstanceRef<Background>,
        viewport_w: i32,
        viewport_h: i32,
        border_color: i32,
        target: &mut [i32],
    ) -> Result<()> {
        if viewport_w <= 0 || viewport_h <= 0 {
            return Ok(());
        }
        let image2d: ClassInstanceRef<Image2D> = jvm
            .get_field(background, "image", "Ljavax/microedition/m3g/Image2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image2d.is_null() {
            return Ok(());
        }
        let image: ClassInstanceRef<Image> = jvm
            .get_field(&image2d, "image", "Ljavax/microedition/lcdui/Image;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image.is_null() {
            return Ok(());
        }

        let (image_w, image_h, image_pixels) = Self::image_pixels_cached(jvm, &image).await?;
        if image_w <= 0 || image_h <= 0 {
            return Ok(());
        }
        let crop_x: i32 = jvm.get_field(background, "cropX", "I").await.unwrap_or(0);
        let crop_y: i32 = jvm.get_field(background, "cropY", "I").await.unwrap_or(0);
        let crop_w: i32 = jvm.get_field(background, "cropW", "I").await.unwrap_or(image_w);
        let crop_h: i32 = jvm.get_field(background, "cropH", "I").await.unwrap_or(image_h);
        if crop_w <= 0 || crop_h <= 0 {
            return Ok(());
        }
        let mode_x: i32 = jvm.get_field(background, "imageModeX", "I").await.unwrap_or(Background::BORDER);
        let mode_y: i32 = jvm.get_field(background, "imageModeY", "I").await.unwrap_or(Background::BORDER);

        for y in 0..viewport_h {
            let src_y = crop_y + (y as i64 * crop_h as i64 / viewport_h as i64) as i32;
            let Some(sample_y) = background_coord(src_y, image_h, mode_y) else {
                continue;
            };
            for x in 0..viewport_w {
                let src_x = crop_x + (x as i64 * crop_w as i64 / viewport_w as i64) as i32;
                let Some(sample_x) = background_coord(src_x, image_w, mode_x) else {
                    continue;
                };
                let dst = (y * viewport_w + x) as usize;
                let src = image_pixels[(sample_y * image_w + sample_x) as usize];
                target[dst] = source_over(border_color, src);
            }
        }
        Ok(())
    }
}

struct M3gFileLoader<'a, 'b> {
    jvm: &'a Jvm,
    context: &'a mut RuntimeContext,
    resource_name: &'a str,
    history: &'b mut Vec<RustString>,
    refs: Vec<LoadedObject>,
}

struct LoadedObject {
    object: ClassInstanceRef<Object3D>,
    referenced: bool,
    external: bool,
}

impl<'a, 'b> M3gFileLoader<'a, 'b> {
    fn new(jvm: &'a Jvm, context: &'a mut RuntimeContext, resource_name: &'a str, history: &'b mut Vec<RustString>) -> Self {
        Self {
            jvm,
            context,
            resource_name,
            history,
            refs: Vec::new(),
        }
    }

    async fn parse(mut self, bytes: &[u8]) -> Result<Vec<ClassInstanceRef<Object3D>>> {
        if !bytes.starts_with(M3G_IDENTIFIER) {
            return Err(self.jvm.exception("java/io/IOException", "bad M3G identifier").await);
        }

        let mut offset = M3G_IDENTIFIER.len();
        let mut section_index = 0;
        while offset + 9 <= bytes.len() {
            let compression = bytes[offset];
            let total_len = read_u32_at(bytes, offset + 1) as usize;
            let inflated_len = read_u32_at(bytes, offset + 5) as usize;
            if total_len < 13 || offset + total_len > bytes.len() {
                return Err(self.jvm.exception("java/io/IOException", "bad M3G section length").await);
            }
            let data_start = offset + 9;
            let checksum_start = offset + total_len - 4;
            let section = match decode_m3g_section_data(compression, &bytes[data_start..checksum_start], inflated_len) {
                Ok(section) => section,
                Err(message) => return Err(self.jvm.exception("java/io/IOException", message).await),
            };
            self.parse_section(section_index, section.as_slice()).await?;
            offset += total_len;
            section_index += 1;
        }

        let roots: Vec<ClassInstanceRef<Object3D>> = self
            .refs
            .iter()
            .filter(|loaded| !loaded.external && !loaded.referenced)
            .map(|loaded| loaded.object.clone())
            .collect();
        let result = if roots.is_empty() {
            self.refs
                .iter()
                .filter(|loaded| !loaded.external)
                .map(|loaded| loaded.object.clone())
                .collect()
        } else {
            roots
        };

        tracing::info!(
            target: "rustjava_m3g",
            "m3g.loader.parsed resource={} sections={} objects={} roots={}",
            self.resource_name,
            section_index,
            self.refs.len(),
            result.len()
        );

        Ok(result)
    }

    async fn parse_section(&mut self, section_index: usize, section: &[u8]) -> Result<()> {
        let mut reader = M3gReader::new(section, self.reader_error("Malformed M3G section").await);
        while reader.remaining() >= 5 {
            let object_type = reader.read_u8()?;
            let length = reader.read_u32()? as usize;
            let payload = reader.read_slice(length)?;
            self.parse_object(section_index, object_type, payload).await?;
        }
        Ok(())
    }

    async fn parse_object(&mut self, section_index: usize, object_type: u8, payload: &[u8]) -> Result<()> {
        let mut reader = M3gReader::new(payload, self.reader_error("Malformed M3G object").await);
        let object = match object_type {
            0 => {
                self.parse_header(section_index, &mut reader)?;
                None
            }
            1 => Some(self.parse_animation_controller(&mut reader).await?),
            2 => Some(self.parse_animation_track(&mut reader).await?),
            3 => Some(self.parse_appearance(&mut reader).await?),
            4 => Some(self.parse_background(&mut reader).await?),
            5 => Some(self.parse_camera(&mut reader).await?),
            6 => Some(self.parse_compositing_mode(&mut reader).await?),
            7 => Some(self.parse_fog(&mut reader).await?),
            8 => Some(self.parse_polygon_mode(&mut reader).await?),
            9 => Some(self.parse_group(&mut reader).await?),
            10 => Some(self.parse_image2d(&mut reader).await?),
            11 => Some(self.parse_triangle_strip_array(&mut reader).await?),
            12 => Some(self.parse_light(&mut reader).await?),
            13 => Some(self.parse_material(&mut reader).await?),
            14 => Some(self.parse_mesh(&mut reader).await?),
            17 => Some(self.parse_texture2d(&mut reader).await?),
            18 => Some(self.parse_sprite3d(&mut reader).await?),
            19 => Some(self.parse_keyframe_sequence(&mut reader).await?),
            20 => Some(self.parse_vertex_array(&mut reader).await?),
            21 => Some(self.parse_vertex_buffer(&mut reader).await?),
            22 => Some(self.parse_world(&mut reader).await?),
            255 => {
                let name = reader.read_string()?;
                let objects = Loader::load_resource_objects(self.jvm, self.context, &name, self.history).await?;
                if let Some(object) = objects.into_iter().next() {
                    self.refs.push(LoadedObject {
                        object,
                        referenced: false,
                        external: true,
                    });
                }
                None
            }
            _ => {
                tracing::warn!(target: "rustjava_m3g", "unsupported M3G object type {} in {}", object_type, self.resource_name);
                None
            }
        };

        if let Some(object) = object {
            self.refs.push(LoadedObject {
                object,
                referenced: false,
                external: false,
            });
        }
        Ok(())
    }

    async fn reader_error(&self, message: &str) -> Box<dyn ClassInstance> {
        match self.jvm.exception("java/io/IOException", message).await {
            JavaError::JavaException(exception) => exception,
        }
    }

    fn parse_header(&self, section_index: usize, reader: &mut M3gReader<'_>) -> Result<()> {
        let major = reader.read_u8()?;
        let minor = reader.read_u8()?;
        let external_refs = reader.read_bool()?;
        let file_size = reader.read_u32()?;
        let content_size = reader.read_u32()?;
        let author = reader.read_string()?;
        tracing::info!(
            target: "rustjava_m3g",
            "m3g.header resource={} section={} version={}.{} external={} fileSize={} contentSize={} author={:?}",
            self.resource_name,
            section_index,
            major,
            minor,
            external_refs,
            file_size,
            content_size,
            author
        );
        Ok(())
    }

    async fn parse_object3d(&mut self, reader: &mut M3gReader<'_>, object: &mut ClassInstanceRef<Object3D>) -> Result<()> {
        let user_id = reader.read_i32()?;
        self.jvm.put_field(object, "userID", "I", user_id).await?;
        let animation_tracks = reader.read_u32()? as usize;
        let mut tracks = Vec::with_capacity(animation_tracks);
        for _ in 0..animation_tracks {
            tracks.push(cast_ref::<Object3D, AnimationTrack>(&self.get_ref(reader.read_i32()?)?));
        }
        let user_params = reader.read_u32()? as usize;
        for _ in 0..user_params {
            let _id = reader.read_u32()?;
            let len = reader.read_u32()? as usize;
            reader.skip(len)?;
        }
        let mut track_array = self
            .jvm
            .instantiate_array("Ljavax/microedition/m3g/AnimationTrack;", tracks.len())
            .await?;
        self.jvm.store_array(&mut track_array, 0, tracks).await?;
        self.jvm
            .put_field(object, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", track_array)
            .await?;
        Ok(())
    }

    async fn parse_animation_controller(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut controller: ClassInstanceRef<AnimationController> =
            self.jvm.new_class("javax/microedition/m3g/AnimationController", "()V", ()).await?.into();
        let mut object = cast_ref(&controller);
        self.parse_object3d(reader, &mut object).await?;
        let speed = reader.read_f32()?;
        let weight = reader.read_f32()?;
        let active_start = reader.read_i32()?;
        let active_end = reader.read_i32()?;
        let ref_sequence_time = reader.read_f32()?;
        let ref_world_time = reader.read_i32()?;
        self.jvm.put_field(&mut controller, "speed", "F", speed).await?;
        self.jvm.put_field(&mut controller, "weight", "F", weight).await?;
        self.jvm.put_field(&mut controller, "activeIntervalStart", "I", active_start).await?;
        self.jvm.put_field(&mut controller, "activeIntervalEnd", "I", active_end).await?;
        self.jvm.put_field(&mut controller, "refSequenceTime", "F", ref_sequence_time).await?;
        self.jvm.put_field(&mut controller, "refWorldTime", "I", ref_world_time).await?;
        Ok(cast_ref(&controller))
    }

    async fn parse_animation_track(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut track: ClassInstanceRef<AnimationTrack> = self.jvm.new_class("javax/microedition/m3g/AnimationTrack", "()V", ()).await?.into();
        let mut object = cast_ref(&track);
        self.parse_object3d(reader, &mut object).await?;
        let sequence = cast_ref::<Object3D, KeyframeSequence>(&self.get_ref(reader.read_i32()?)?);
        let controller = cast_ref::<Object3D, AnimationController>(&self.get_ref(reader.read_i32()?)?);
        let target_property = reader.read_i32()?;
        self.jvm
            .put_field(&mut track, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;", sequence)
            .await?;
        self.jvm
            .put_field(&mut track, "controller", "Ljavax/microedition/m3g/AnimationController;", controller)
            .await?;
        self.jvm.put_field(&mut track, "targetProperty", "I", target_property).await?;
        Ok(cast_ref(&track))
    }

    async fn parse_keyframe_sequence(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut sequence: ClassInstanceRef<KeyframeSequence> = self
            .jvm
            .new_class("javax/microedition/m3g/KeyframeSequence", "(III)V", (1, 1, KeyframeSequence::LINEAR))
            .await?
            .into();
        let mut object = cast_ref(&sequence);
        self.parse_object3d(reader, &mut object).await?;
        let interpolation = reader.read_u8()? as i32;
        let repeat_mode = reader.read_u8()? as i32;
        let encoding = reader.read_u8()?;
        let duration = reader.read_u32()? as i32;
        let valid_first = reader.read_u32()? as i32;
        let valid_last = reader.read_u32()? as i32;
        let component_count = reader.read_u32()? as i32;
        let keyframe_count = reader.read_u32()? as i32;
        let mut times = Vec::with_capacity(keyframe_count.max(0) as usize);
        let mut values = Vec::with_capacity((keyframe_count.max(0) * component_count.max(0)) as usize);

        match encoding {
            0 => {
                for _ in 0..keyframe_count.max(0) {
                    times.push(reader.read_u32()? as i32);
                    for _ in 0..component_count.max(0) {
                        values.push(reader.read_f32()?);
                    }
                }
            }
            1 => {
                let bias = Self::read_f32_vec(reader, component_count.max(0) as usize)?;
                let scale = Self::read_f32_vec(reader, component_count.max(0) as usize)?;
                for _ in 0..keyframe_count.max(0) {
                    times.push(reader.read_u32()? as i32);
                    for component in 0..component_count.max(0) as usize {
                        let raw = reader.read_u8()? as f32 / 255.0;
                        values.push(bias[component] + raw * scale[component]);
                    }
                }
            }
            2 => {
                let bias = Self::read_f32_vec(reader, component_count.max(0) as usize)?;
                let scale = Self::read_f32_vec(reader, component_count.max(0) as usize)?;
                for _ in 0..keyframe_count.max(0) {
                    times.push(reader.read_u32()? as i32);
                    for component in 0..component_count.max(0) as usize {
                        let raw = reader.read_u16()? as f32 / 65535.0;
                        values.push(bias[component] + raw * scale[component]);
                    }
                }
            }
            _ => return Err(self.jvm.exception("java/io/IOException", "unsupported KeyframeSequence encoding").await),
        }

        KeyframeSequence::put_data(
            self.jvm,
            &mut sequence,
            interpolation,
            repeat_mode,
            duration,
            valid_first,
            valid_last,
            component_count,
            times,
            values,
        )
        .await?;
        Ok(cast_ref(&sequence))
    }

    fn read_f32_vec(reader: &mut M3gReader<'_>, count: usize) -> Result<Vec<f32>> {
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(reader.read_f32()?);
        }
        Ok(values)
    }

    async fn parse_transformable(&mut self, reader: &mut M3gReader<'_>, object: &mut ClassInstanceRef<Transformable>) -> Result<()> {
        let mut object3d: ClassInstanceRef<Object3D> = cast_ref(object);
        self.parse_object3d(reader, &mut object3d).await?;
        if reader.read_bool()? {
            let tx = reader.read_f32()?;
            let ty = reader.read_f32()?;
            let tz = reader.read_f32()?;
            let sx = reader.read_f32()?;
            let sy = reader.read_f32()?;
            let sz = reader.read_f32()?;
            let angle = reader.read_f32()?;
            let ax = reader.read_f32()?;
            let ay = reader.read_f32()?;
            let az = reader.read_f32()?;
            Transformable::set_translation_fields(self.jvm, object, tx, ty, tz).await?;
            Transformable::set_scale_fields(self.jvm, object, sx, sy, sz).await?;
            Transformable::set_orientation_fields(self.jvm, object, angle, ax, ay, az).await?;
        }
        if reader.read_bool()? {
            let mut matrix = [0.0; 16];
            for item in &mut matrix {
                *item = reader.read_f32()?;
            }
            Transformable::put_generic_matrix(self.jvm, object, matrix).await?;
        }
        Ok(())
    }

    async fn parse_node(&mut self, reader: &mut M3gReader<'_>, object: &mut ClassInstanceRef<Node>) -> Result<()> {
        let mut transformable: ClassInstanceRef<Transformable> = cast_ref(object);
        self.parse_transformable(reader, &mut transformable).await?;
        let rendering_enabled = reader.read_bool()?;
        let picking_enabled = reader.read_bool()?;
        let alpha = reader.read_u8()? as f32 / 255.0;
        let scope = reader.read_i32()?;
        self.jvm.put_field(object, "renderingEnabled", "Z", rendering_enabled).await?;
        self.jvm.put_field(object, "pickingEnabled", "Z", picking_enabled).await?;
        self.jvm.put_field(object, "alphaFactor", "F", alpha).await?;
        self.jvm.put_field(object, "scope", "I", scope).await?;
        if reader.read_bool()? {
            let z_target = reader.read_u8()? as i32;
            let z_reference = cast_ref::<Object3D, Node>(&self.get_ref(reader.read_i32()?)?);
            let y_target = reader.read_u8()? as i32;
            let y_reference = cast_ref::<Object3D, Node>(&self.get_ref(reader.read_i32()?)?);
            self.jvm
                .put_field(object, "zReference", "Ljavax/microedition/m3g/Node;", z_reference)
                .await?;
            self.jvm
                .put_field(object, "yReference", "Ljavax/microedition/m3g/Node;", y_reference)
                .await?;
            self.jvm.put_field(object, "zTarget", "I", z_target).await?;
            self.jvm.put_field(object, "yTarget", "I", y_target).await?;
        }
        Ok(())
    }

    async fn parse_camera(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut camera: ClassInstanceRef<Camera> = self.jvm.new_class("javax/microedition/m3g/Camera", "()V", ()).await?.into();
        let mut node = cast_ref(&camera);
        self.parse_node(reader, &mut node).await?;
        match reader.read_u8()? {
            48 => {
                let mut matrix = [0.0; 16];
                for item in &mut matrix {
                    *item = reader.read_f32()?;
                }
                self.jvm.put_field(&mut camera, "projectionMode", "I", Camera::GENERIC).await?;
                Camera::put_generic_projection(self.jvm, &mut camera, matrix).await?;
            }
            49 => {
                let height = reader.read_f32()?;
                let aspect = reader.read_f32()?;
                let near = reader.read_f32()?;
                let far = reader.read_f32()?;
                Camera::set_parallel(self.jvm, self.context, camera.clone(), height, aspect, near, far).await?;
            }
            50 => {
                let f1 = reader.read_f32()?;
                let f2 = reader.read_f32()?;
                let f3 = reader.read_f32()?;
                let f4 = reader.read_f32()?;
                Camera::set_perspective(self.jvm, self.context, camera.clone(), f1, f2, f3, f4).await?;
            }
            _ => {}
        }
        Ok(cast_ref(&camera))
    }

    async fn parse_background(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut background: ClassInstanceRef<Background> = self.jvm.new_class("javax/microedition/m3g/Background", "()V", ()).await?.into();
        let mut object = cast_ref(&background);
        self.parse_object3d(reader, &mut object).await?;
        let color = reader.read_argb()?;
        let image = cast_ref::<Object3D, Image2D>(&self.get_ref(reader.read_i32()?)?);
        let image_mode_x = reader.read_u8()? as i32;
        let image_mode_y = reader.read_u8()? as i32;
        let crop_x = reader.read_i32()?;
        let crop_y = reader.read_i32()?;
        let crop_w = reader.read_i32()?;
        let crop_h = reader.read_i32()?;
        let color_clear = reader.read_bool()?;
        let depth_clear = reader.read_bool()?;
        self.jvm.put_field(&mut background, "color", "I", color).await?;
        self.jvm
            .put_field(&mut background, "image", "Ljavax/microedition/m3g/Image2D;", image)
            .await?;
        self.jvm.put_field(&mut background, "imageModeX", "I", image_mode_x).await?;
        self.jvm.put_field(&mut background, "imageModeY", "I", image_mode_y).await?;
        self.jvm.put_field(&mut background, "cropX", "I", crop_x).await?;
        self.jvm.put_field(&mut background, "cropY", "I", crop_y).await?;
        self.jvm.put_field(&mut background, "cropW", "I", crop_w).await?;
        self.jvm.put_field(&mut background, "cropH", "I", crop_h).await?;
        self.jvm.put_field(&mut background, "colorClear", "Z", color_clear).await?;
        self.jvm.put_field(&mut background, "depthClear", "Z", depth_clear).await?;
        Ok(cast_ref(&background))
    }

    async fn parse_compositing_mode(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut mode: ClassInstanceRef<CompositingMode> = self.jvm.new_class("javax/microedition/m3g/CompositingMode", "()V", ()).await?.into();
        let mut object = cast_ref(&mode);
        self.parse_object3d(reader, &mut object).await?;
        let depth_test = reader.read_bool()?;
        let depth_write = reader.read_bool()?;
        let color_write = reader.read_bool()?;
        let alpha_write = reader.read_bool()?;
        let blending = reader.read_u8()? as i32;
        let alpha_threshold = reader.read_u8()? as f32 / 255.0;
        let depth_factor = reader.read_f32()?;
        let depth_units = reader.read_f32()?;
        self.jvm.put_field(&mut mode, "depthTest", "Z", depth_test).await?;
        self.jvm.put_field(&mut mode, "depthWrite", "Z", depth_write).await?;
        self.jvm.put_field(&mut mode, "colorWrite", "Z", color_write).await?;
        self.jvm.put_field(&mut mode, "alphaWrite", "Z", alpha_write).await?;
        self.jvm.put_field(&mut mode, "blending", "I", blending).await?;
        self.jvm.put_field(&mut mode, "alphaThreshold", "F", alpha_threshold).await?;
        self.jvm.put_field(&mut mode, "depthOffsetFactor", "F", depth_factor).await?;
        self.jvm.put_field(&mut mode, "depthOffsetUnits", "F", depth_units).await?;
        Ok(cast_ref(&mode))
    }

    async fn parse_fog(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut fog: ClassInstanceRef<Fog> = self.jvm.new_class("javax/microedition/m3g/Fog", "()V", ()).await?.into();
        let mut object = cast_ref(&fog);
        self.parse_object3d(reader, &mut object).await?;
        let color = reader.read_rgb()?;
        let mode = reader.read_u8()? as i32;
        let (density, near, far) = if mode == Fog::EXPONENTIAL {
            (reader.read_f32()?, 0.0, 0.0)
        } else {
            (1.0, reader.read_f32()?, reader.read_f32()?)
        };
        self.jvm.put_field(&mut fog, "color", "I", color).await?;
        self.jvm.put_field(&mut fog, "density", "F", density).await?;
        self.jvm.put_field(&mut fog, "mode", "I", mode).await?;
        self.jvm.put_field(&mut fog, "near", "F", near).await?;
        self.jvm.put_field(&mut fog, "far", "F", far).await?;
        Ok(cast_ref(&fog))
    }

    async fn parse_polygon_mode(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut mode: ClassInstanceRef<PolygonMode> = self.jvm.new_class("javax/microedition/m3g/PolygonMode", "()V", ()).await?.into();
        let mut object = cast_ref(&mode);
        self.parse_object3d(reader, &mut object).await?;
        let culling = reader.read_u8()? as i32;
        let shading = reader.read_u8()? as i32;
        let winding = reader.read_u8()? as i32;
        let two_sided = reader.read_bool()?;
        let local_camera = reader.read_bool()?;
        let perspective = reader.read_bool()?;
        self.jvm.put_field(&mut mode, "culling", "I", culling).await?;
        self.jvm.put_field(&mut mode, "shading", "I", shading).await?;
        self.jvm.put_field(&mut mode, "winding", "I", winding).await?;
        self.jvm.put_field(&mut mode, "twoSidedLighting", "Z", two_sided).await?;
        self.jvm.put_field(&mut mode, "localCameraLighting", "Z", local_camera).await?;
        self.jvm.put_field(&mut mode, "perspectiveCorrection", "Z", perspective).await?;
        Ok(cast_ref(&mode))
    }

    async fn parse_appearance(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut appearance: ClassInstanceRef<Appearance> = self.jvm.new_class("javax/microedition/m3g/Appearance", "()V", ()).await?.into();
        let mut object = cast_ref(&appearance);
        self.parse_object3d(reader, &mut object).await?;
        let layer = reader.read_i8()? as i32;
        let compositing = cast_ref::<Object3D, CompositingMode>(&self.get_ref(reader.read_i32()?)?);
        let fog = cast_ref::<Object3D, Fog>(&self.get_ref(reader.read_i32()?)?);
        let polygon = cast_ref::<Object3D, PolygonMode>(&self.get_ref(reader.read_i32()?)?);
        let material = cast_ref::<Object3D, Material>(&self.get_ref(reader.read_i32()?)?);
        let texture_count = reader.read_u32()? as usize;
        let texture0 = if texture_count > 0 {
            cast_ref::<Object3D, Texture2D>(&self.get_ref(reader.read_i32()?)?)
        } else {
            null_ref()
        };
        if texture_count > 1 {
            reader.skip((texture_count - 1) * 4)?;
        }
        self.jvm
            .put_field(
                &mut appearance,
                "compositingMode",
                "Ljavax/microedition/m3g/CompositingMode;",
                compositing,
            )
            .await?;
        self.jvm.put_field(&mut appearance, "fog", "Ljavax/microedition/m3g/Fog;", fog).await?;
        self.jvm.put_field(&mut appearance, "layer", "I", layer).await?;
        self.jvm
            .put_field(&mut appearance, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;", polygon)
            .await?;
        self.jvm
            .put_field(&mut appearance, "material", "Ljavax/microedition/m3g/Material;", material)
            .await?;
        self.jvm
            .put_field(&mut appearance, "texture0", "Ljavax/microedition/m3g/Texture2D;", texture0)
            .await?;
        Ok(cast_ref(&appearance))
    }

    async fn parse_group(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut group: ClassInstanceRef<Group> = self.jvm.new_class("javax/microedition/m3g/Group", "()V", ()).await?.into();
        let mut node = cast_ref(&group);
        self.parse_node(reader, &mut node).await?;
        self.parse_group_children(reader, &mut group).await?;
        Ok(cast_ref(&group))
    }

    async fn parse_group_children(&mut self, reader: &mut M3gReader<'_>, group: &mut ClassInstanceRef<Group>) -> Result<()> {
        let child_count = reader.read_u32()? as usize;
        for _ in 0..child_count {
            let child = cast_ref::<Object3D, Node>(&self.get_ref(reader.read_i32()?)?);
            Group::push_child(self.jvm, group, child).await?;
        }
        Ok(())
    }

    async fn parse_world(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut world: ClassInstanceRef<World> = self.jvm.new_class("javax/microedition/m3g/World", "()V", ()).await?.into();
        let mut node = cast_ref(&world);
        self.parse_node(reader, &mut node).await?;
        let mut group = cast_ref(&world);
        self.parse_group_children(reader, &mut group).await?;
        let camera = cast_ref::<Object3D, Camera>(&self.get_ref(reader.read_i32()?)?);
        let background = cast_ref::<Object3D, Background>(&self.get_ref(reader.read_i32()?)?);
        self.jvm
            .put_field(&mut world, "activeCamera", "Ljavax/microedition/m3g/Camera;", camera)
            .await?;
        self.jvm
            .put_field(&mut world, "background", "Ljavax/microedition/m3g/Background;", background)
            .await?;
        Ok(cast_ref(&world))
    }

    async fn parse_image2d(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut image: ClassInstanceRef<Image2D> = self.jvm.new_class("javax/microedition/m3g/Image2D", "()V", ()).await?.into();
        let mut object = cast_ref(&image);
        self.parse_object3d(reader, &mut object).await?;
        let format = reader.read_u8()? as i32;
        let mutable = reader.read_bool()?;
        let width = (reader.read_u32()? as i32).max(1);
        let height = (reader.read_u32()? as i32).max(1);
        let argb = if mutable {
            vec![0; (width * height) as usize]
        } else {
            let palette = reader.read_byte_array()?;
            let pixels = reader.read_byte_array()?;
            decode_m3g_image_pixels(format, width, height, palette, pixels)
        };
        let lcdui_image = Image::from_argb(self.jvm, width, height, argb).await?;
        self.jvm.put_field(&mut image, "format", "I", format).await?;
        self.jvm.put_field(&mut image, "width", "I", width).await?;
        self.jvm.put_field(&mut image, "height", "I", height).await?;
        self.jvm
            .put_field(&mut image, "image", "Ljavax/microedition/lcdui/Image;", lcdui_image)
            .await?;
        self.jvm.put_field(&mut image, "mutable", "Z", mutable).await?;
        Ok(cast_ref(&image))
    }

    async fn parse_triangle_strip_array(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut tsa: ClassInstanceRef<TriangleStripArray> = self.jvm.new_class("javax/microedition/m3g/TriangleStripArray", "()V", ()).await?.into();
        let mut object = cast_ref(&tsa);
        self.parse_object3d(reader, &mut object).await?;
        let encoding = reader.read_u8()?;
        let indices = match encoding {
            0 => {
                let start = reader.read_i32()?;
                vec![start]
            }
            1 => vec![reader.read_u8()? as i32],
            2 => vec![reader.read_u16()? as i32],
            128 => {
                let count = reader.read_u32()? as usize;
                let mut values = Vec::with_capacity(count);
                for _ in 0..count {
                    values.push(reader.read_i32()?);
                }
                values
            }
            129 => {
                let count = reader.read_u32()? as usize;
                let mut values = Vec::with_capacity(count);
                for _ in 0..count {
                    values.push(reader.read_u8()? as i32);
                }
                values
            }
            130 => {
                let count = reader.read_u32()? as usize;
                let mut values = Vec::with_capacity(count);
                for _ in 0..count {
                    values.push(reader.read_u16()? as i32);
                }
                values
            }
            _ => Vec::new(),
        };
        let length_count = reader.read_u32()? as usize;
        let mut lengths = Vec::with_capacity(length_count);
        for _ in 0..length_count {
            lengths.push(reader.read_i32()?);
        }
        let mut indices_array = self.jvm.instantiate_array("I", indices.len()).await?;
        self.jvm.store_array(&mut indices_array, 0, indices).await?;
        let mut lengths_array = self.jvm.instantiate_array("I", lengths.len()).await?;
        self.jvm.store_array(&mut lengths_array, 0, lengths).await?;
        self.jvm.put_field(&mut tsa, "indices", "[I", indices_array).await?;
        self.jvm.put_field(&mut tsa, "stripLengths", "[I", lengths_array).await?;
        Ok(cast_ref(&tsa))
    }

    async fn parse_vertex_array(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut vertex_array: ClassInstanceRef<VertexArray> = self.jvm.new_class("javax/microedition/m3g/VertexArray", "()V", ()).await?.into();
        let mut object = cast_ref(&vertex_array);
        self.parse_object3d(reader, &mut object).await?;
        let component_size = reader.read_u8()? as i32;
        let component_count = reader.read_u8()? as i32;
        let encoding = reader.read_u8()?;
        let vertex_count = reader.read_u16()? as i32;
        self.jvm.put_field(&mut vertex_array, "componentSize", "I", component_size).await?;
        self.jvm.put_field(&mut vertex_array, "componentCount", "I", component_count).await?;
        self.jvm.put_field(&mut vertex_array, "vertexCount", "I", vertex_count).await?;
        self.jvm.put_field(&mut vertex_array, "version", "I", 1i32).await?;
        let value_count = (vertex_count * component_count).max(0) as usize;
        if component_size == 1 {
            let mut values = Vec::with_capacity(value_count);
            let mut previous = [0i8; 4];
            for index in 0..value_count {
                let component = index % component_count as usize;
                let value = reader.read_u8()? as i8;
                previous[component] = if encoding == 0 { value } else { previous[component].wrapping_add(value) };
                values.push(previous[component]);
            }
            let mut array = self.jvm.instantiate_array("B", values.len()).await?;
            self.jvm.store_array(&mut array, 0, values).await?;
            self.jvm.put_field(&mut vertex_array, "byteData", "[B", array).await?;
        } else {
            let mut values = Vec::with_capacity(value_count);
            let mut previous = [0i16; 4];
            for index in 0..value_count {
                let component = index % component_count as usize;
                let value = reader.read_i16()?;
                previous[component] = if encoding == 0 { value } else { previous[component].wrapping_add(value) };
                values.push(previous[component]);
            }
            let mut array = self.jvm.instantiate_array("S", values.len()).await?;
            self.jvm.store_array(&mut array, 0, values).await?;
            self.jvm.put_field(&mut vertex_array, "shortData", "[S", array).await?;
        }
        Ok(cast_ref(&vertex_array))
    }

    async fn parse_vertex_buffer(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut buffer: ClassInstanceRef<VertexBuffer> = self.jvm.new_class("javax/microedition/m3g/VertexBuffer", "()V", ()).await?.into();
        let mut object = cast_ref(&buffer);
        self.parse_object3d(reader, &mut object).await?;
        let default_color = reader.read_argb()?;
        let positions = cast_ref::<Object3D, VertexArray>(&self.get_ref(reader.read_i32()?)?);
        let bias = vec![reader.read_f32()?, reader.read_f32()?, reader.read_f32()?];
        let scale = reader.read_f32()?;
        let normals = cast_ref::<Object3D, VertexArray>(&self.get_ref(reader.read_i32()?)?);
        let colors = cast_ref::<Object3D, VertexArray>(&self.get_ref(reader.read_i32()?)?);
        let tex_count = reader.read_u32()? as usize;
        let mut tex_coords0 = null_ref();
        let mut tex_bias = vec![0.0, 0.0, 0.0];
        let mut tex_scale = 1.0;
        for index in 0..tex_count {
            let tex = cast_ref::<Object3D, VertexArray>(&self.get_ref(reader.read_i32()?)?);
            let bias_values = vec![reader.read_f32()?, reader.read_f32()?, reader.read_f32()?];
            let scale_value = reader.read_f32()?;
            if index == 0 {
                tex_coords0 = tex;
                tex_bias = bias_values;
                tex_scale = scale_value;
            }
        }
        let mut bias_array = self.jvm.instantiate_array("F", bias.len()).await?;
        self.jvm.store_array(&mut bias_array, 0, bias).await?;
        let mut tex_bias_array = self.jvm.instantiate_array("F", tex_bias.len()).await?;
        self.jvm.store_array(&mut tex_bias_array, 0, tex_bias).await?;
        self.jvm.put_field(&mut buffer, "defaultColor", "I", default_color).await?;
        self.jvm
            .put_field(&mut buffer, "positions", "Ljavax/microedition/m3g/VertexArray;", positions)
            .await?;
        self.jvm
            .put_field(&mut buffer, "normals", "Ljavax/microedition/m3g/VertexArray;", normals)
            .await?;
        self.jvm
            .put_field(&mut buffer, "colors", "Ljavax/microedition/m3g/VertexArray;", colors)
            .await?;
        self.jvm
            .put_field(&mut buffer, "texCoords0", "Ljavax/microedition/m3g/VertexArray;", tex_coords0)
            .await?;
        self.jvm.put_field(&mut buffer, "positionScale", "F", scale).await?;
        self.jvm.put_field(&mut buffer, "positionBias", "[F", bias_array).await?;
        self.jvm.put_field(&mut buffer, "texScale", "F", tex_scale).await?;
        self.jvm.put_field(&mut buffer, "texBias", "[F", tex_bias_array).await?;
        Ok(cast_ref(&buffer))
    }

    async fn parse_light(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut light: ClassInstanceRef<Light> = self.jvm.new_class("javax/microedition/m3g/Light", "()V", ()).await?.into();
        let mut node = cast_ref(&light);
        self.parse_node(reader, &mut node).await?;
        let constant_attenuation = reader.read_f32()?;
        let linear_attenuation = reader.read_f32()?;
        let quadratic_attenuation = reader.read_f32()?;
        let color = reader.read_rgb()?;
        let mode = reader.read_u8()? as i32;
        let intensity = reader.read_f32()?;
        let spot_angle = reader.read_f32()?;
        let spot_exponent = reader.read_f32()?;
        self.jvm.put_field(&mut light, "constantAttenuation", "F", constant_attenuation).await?;
        self.jvm.put_field(&mut light, "linearAttenuation", "F", linear_attenuation).await?;
        self.jvm.put_field(&mut light, "quadraticAttenuation", "F", quadratic_attenuation).await?;
        self.jvm.put_field(&mut light, "color", "I", color).await?;
        self.jvm.put_field(&mut light, "mode", "I", mode).await?;
        self.jvm.put_field(&mut light, "intensity", "F", intensity).await?;
        self.jvm.put_field(&mut light, "spotAngle", "F", spot_angle).await?;
        self.jvm.put_field(&mut light, "spotExponent", "F", spot_exponent).await?;
        Ok(cast_ref(&light))
    }

    async fn parse_material(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut material: ClassInstanceRef<Material> = self.jvm.new_class("javax/microedition/m3g/Material", "()V", ()).await?.into();
        let mut object = cast_ref(&material);
        self.parse_object3d(reader, &mut object).await?;
        let ambient = reader.read_rgb()?;
        let diffuse = reader.read_argb()?;
        let emissive = reader.read_rgb()?;
        let specular = reader.read_rgb()?;
        let shininess = reader.read_f32()?;
        let vertex_color_tracking = reader.read_bool()?;
        self.jvm.put_field(&mut material, "ambientColor", "I", ensure_opaque(ambient)).await?;
        self.jvm.put_field(&mut material, "diffuseColor", "I", diffuse).await?;
        self.jvm.put_field(&mut material, "emissiveColor", "I", ensure_opaque(emissive)).await?;
        self.jvm.put_field(&mut material, "specularColor", "I", ensure_opaque(specular)).await?;
        self.jvm.put_field(&mut material, "shininess", "F", shininess).await?;
        self.jvm
            .put_field(&mut material, "vertexColorTracking", "Z", vertex_color_tracking)
            .await?;
        Ok(cast_ref(&material))
    }

    async fn parse_mesh(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut mesh: ClassInstanceRef<Mesh> = self.jvm.new_class("javax/microedition/m3g/Mesh", "()V", ()).await?.into();
        let mut node = cast_ref(&mesh);
        self.parse_node(reader, &mut node).await?;
        let vertex_buffer = cast_ref::<Object3D, VertexBuffer>(&self.get_ref(reader.read_i32()?)?);
        let submesh_count = reader.read_u32()? as usize;
        let mut index_buffers = Vec::with_capacity(submesh_count);
        let mut appearances = Vec::with_capacity(submesh_count);
        for _ in 0..submesh_count {
            index_buffers.push(cast_ref::<Object3D, IndexBuffer>(&self.get_ref(reader.read_i32()?)?));
            appearances.push(cast_ref::<Object3D, Appearance>(&self.get_ref(reader.read_i32()?)?));
        }
        let mut index_array = self
            .jvm
            .instantiate_array("Ljavax/microedition/m3g/IndexBuffer;", index_buffers.len())
            .await?;
        self.jvm.store_array(&mut index_array, 0, index_buffers).await?;
        let mut appearance_array = self
            .jvm
            .instantiate_array("Ljavax/microedition/m3g/Appearance;", appearances.len())
            .await?;
        self.jvm.store_array(&mut appearance_array, 0, appearances).await?;
        self.jvm
            .put_field(&mut mesh, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", vertex_buffer)
            .await?;
        self.jvm.put_field(&mut mesh, "submeshCount", "I", submesh_count as i32).await?;
        self.jvm
            .put_field(&mut mesh, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", index_array)
            .await?;
        self.jvm
            .put_field(&mut mesh, "appearances", "[Ljavax/microedition/m3g/Appearance;", appearance_array)
            .await?;
        Ok(cast_ref(&mesh))
    }

    async fn parse_texture2d(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut texture: ClassInstanceRef<Texture2D> = self.jvm.new_class("javax/microedition/m3g/Texture2D", "()V", ()).await?.into();
        let mut transformable = cast_ref(&texture);
        self.parse_transformable(reader, &mut transformable).await?;
        let image = cast_ref::<Object3D, Image2D>(&self.get_ref(reader.read_i32()?)?);
        let blend_color = reader.read_rgb()?;
        let blending = reader.read_u8()? as i32;
        let wrapping_s = reader.read_u8()? as i32;
        let wrapping_t = reader.read_u8()? as i32;
        let level_filter = reader.read_u8()? as i32;
        let image_filter = reader.read_u8()? as i32;
        self.jvm
            .put_field(&mut texture, "image", "Ljavax/microedition/m3g/Image2D;", image)
            .await?;
        self.jvm.put_field(&mut texture, "blendColor", "I", blend_color).await?;
        self.jvm.put_field(&mut texture, "blending", "I", blending).await?;
        self.jvm.put_field(&mut texture, "wrappingS", "I", wrapping_s).await?;
        self.jvm.put_field(&mut texture, "wrappingT", "I", wrapping_t).await?;
        self.jvm.put_field(&mut texture, "levelFilter", "I", level_filter).await?;
        self.jvm.put_field(&mut texture, "imageFilter", "I", image_filter).await?;
        Ok(cast_ref(&texture))
    }

    async fn parse_sprite3d(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut sprite: ClassInstanceRef<Sprite3D> = self
            .jvm
            .new_class(
                "javax/microedition/m3g/Sprite3D",
                "(ZLjavax/microedition/m3g/Image2D;Ljavax/microedition/m3g/Appearance;)V",
                (false, null_ref::<Image2D>(), null_ref::<Appearance>()),
            )
            .await?
            .into();
        let mut node = cast_ref(&sprite);
        self.parse_node(reader, &mut node).await?;
        let image = cast_ref::<Object3D, Image2D>(&self.get_ref(reader.read_i32()?)?);
        let appearance = cast_ref::<Object3D, Appearance>(&self.get_ref(reader.read_i32()?)?);
        let scaled = reader.read_bool()?;
        let crop_x = reader.read_i32()?;
        let crop_y = reader.read_i32()?;
        let crop_w = reader.read_i32()?;
        let crop_h = reader.read_i32()?;
        self.jvm.put_field(&mut sprite, "scaled", "Z", scaled).await?;
        self.jvm
            .put_field(&mut sprite, "image", "Ljavax/microedition/m3g/Image2D;", image)
            .await?;
        self.jvm
            .put_field(&mut sprite, "appearance", "Ljavax/microedition/m3g/Appearance;", appearance)
            .await?;
        self.jvm.put_field(&mut sprite, "cropX", "I", crop_x).await?;
        self.jvm.put_field(&mut sprite, "cropY", "I", crop_y).await?;
        self.jvm.put_field(&mut sprite, "cropW", "I", crop_w).await?;
        self.jvm.put_field(&mut sprite, "cropH", "I", crop_h).await?;
        Ok(cast_ref(&sprite))
    }

    fn get_ref(&mut self, raw_index: i32) -> Result<ClassInstanceRef<Object3D>> {
        if raw_index == 0 {
            return Ok(null_ref());
        }
        let index = raw_index - 2;
        if index < 0 || index as usize >= self.refs.len() {
            return Ok(null_ref());
        }
        let loaded = &mut self.refs[index as usize];
        loaded.referenced = true;
        Ok(loaded.object.clone())
    }
}

struct M3gReader<'a> {
    data: &'a [u8],
    pos: usize,
    read_error: Box<dyn ClassInstance>,
}

impl<'a> M3gReader<'a> {
    fn new(data: &'a [u8], read_error: Box<dyn ClassInstance>) -> Self {
        Self { data, pos: 0, read_error }
    }

    fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    fn read_slice(&mut self, len: usize) -> Result<&'a [u8]> {
        if self.pos + len > self.data.len() {
            return Err(JavaError::JavaException(self.read_error.clone()));
        }
        let start = self.pos;
        self.pos += len;
        Ok(&self.data[start..start + len])
    }

    fn skip(&mut self, len: usize) -> Result<()> {
        self.read_slice(len).map(|_| ())
    }

    fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_slice(1)?[0])
    }

    fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    fn read_byte_array(&mut self) -> Result<&'a [u8]> {
        let len = self.read_u32()? as usize;
        self.read_slice(len)
    }

    fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    fn read_u16(&mut self) -> Result<u16> {
        let bytes = self.read_slice(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    fn read_u32(&mut self) -> Result<u32> {
        let bytes = self.read_slice(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    fn read_rgb(&mut self) -> Result<i32> {
        let r = self.read_u8()? as u32;
        let g = self.read_u8()? as u32;
        let b = self.read_u8()? as u32;
        Ok(((r << 16) | (g << 8) | b) as i32)
    }

    fn read_argb(&mut self) -> Result<i32> {
        let r = self.read_u8()? as u32;
        let g = self.read_u8()? as u32;
        let b = self.read_u8()? as u32;
        let a = self.read_u8()? as u32;
        Ok(((a << 24) | (r << 16) | (g << 8) | b) as i32)
    }

    fn read_string(&mut self) -> Result<RustString> {
        let start = self.pos;
        while self.pos < self.data.len() && self.data[self.pos] != 0 {
            self.pos += 1;
        }
        let bytes = &self.data[start..self.pos];
        if self.pos < self.data.len() {
            self.pos += 1;
        }
        Ok(core::str::from_utf8(bytes).unwrap_or("").to_string())
    }
}

fn read_u32_at(data: &[u8], offset: usize) -> u32 {
    if offset + 4 > data.len() {
        return 0;
    }
    u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

fn same_instance<T, U>(left: &ClassInstanceRef<T>, right: &ClassInstanceRef<U>) -> bool {
    match (&left.instance, &right.instance) {
        (Some(left), Some(right)) => left.equals(&**right).unwrap_or(false),
        (None, None) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use alloc::{sync::Arc, vec};

    use super::{
        Image2D, PolygonMode, Texture2D, decode_m3g_section_data,
        math::{identity_matrix, multiply_matrix, rotation_matrix, rotation_matrix_to_axis_angle},
        render::{M3gTexture, decode_m3g_image_pixels, sample_texture, should_cull_triangle},
    };

    #[test]
    fn uncompressed_m3g_section_uses_borrowed_data() {
        let data = b"m3g-section";
        let decoded = decode_m3g_section_data(0, data, data.len()).unwrap();

        assert_eq!(decoded.as_slice(), data);
    }

    #[test]
    fn zlib_m3g_section_inflates_to_declared_length() {
        let compressed_hello = [0x78, 0x9c, 0xcb, 0x48, 0xcd, 0xc9, 0xc9, 0x07, 0x00, 0x06, 0x2c, 0x02, 0x15];
        let decoded = decode_m3g_section_data(1, &compressed_hello, 5).unwrap();

        assert_eq!(decoded.as_slice(), b"hello");
    }

    #[test]
    fn zlib_m3g_section_rejects_length_mismatch() {
        let compressed_hello = [0x78, 0x9c, 0xcb, 0x48, 0xcd, 0xc9, 0xc9, 0x07, 0x00, 0x06, 0x2c, 0x02, 0x15];

        assert!(decode_m3g_section_data(1, &compressed_hello, 6).is_err());
    }

    #[test]
    fn image2d_embedded_rgba_pixels_are_decoded_as_argb() {
        let pixels = decode_m3g_image_pixels(Image2D::RGBA, 1, 1, &[], &[0x11, 0x22, 0x33, 0x44]);

        assert_eq!(pixels, vec![0x4411_2233]);
    }

    #[test]
    fn image2d_palette_indices_decode_palette_entries() {
        let palette = [0x10, 0x20, 0x30, 0x40, 0xaa, 0xbb, 0xcc, 0xdd];
        let pixels = decode_m3g_image_pixels(Image2D::RGBA, 2, 1, &palette, &[1, 0]);

        assert_eq!(pixels, vec![0xddaa_bbccu32 as i32, 0x4010_2030]);
    }

    #[test]
    fn linear_texture_filter_blends_neighboring_texels() {
        let texture = M3gTexture {
            width: 2,
            height: 2,
            pixels: Arc::new(vec![
                0xff00_0000u32 as i32,
                0xffff_0000u32 as i32,
                0xff00_ff00u32 as i32,
                0xff00_00ffu32 as i32,
            ]),
            format: Image2D::RGBA,
            blend_color: 0,
            blending: Texture2D::FUNC_REPLACE,
            wrap_s: Texture2D::WRAP_CLAMP,
            wrap_t: Texture2D::WRAP_CLAMP,
            image_filter: Texture2D::FILTER_LINEAR,
            transform: identity_matrix(),
            transform_identity: true,
        };

        let sampled = sample_texture(&texture, 0.5, 0.5) as u32;

        assert_eq!((sampled >> 24) & 0xff, 0xff);
        assert!(((sampled >> 16) & 0xff).abs_diff(63) <= 1);
        assert!(((sampled >> 8) & 0xff).abs_diff(63) <= 1);
        assert!((sampled & 0xff).abs_diff(63) <= 1);
    }

    #[test]
    fn polygon_mode_culls_by_projected_winding() {
        assert!(!should_cull_triangle(1.0, PolygonMode::CULL_BACK, PolygonMode::WINDING_CCW));
        assert!(should_cull_triangle(-1.0, PolygonMode::CULL_BACK, PolygonMode::WINDING_CCW));
        assert!(should_cull_triangle(1.0, PolygonMode::CULL_FRONT, PolygonMode::WINDING_CCW));
        assert!(!should_cull_triangle(-1.0, PolygonMode::CULL_FRONT, PolygonMode::WINDING_CCW));
        assert!(!should_cull_triangle(-1.0, PolygonMode::CULL_NONE, PolygonMode::WINDING_CCW));
        assert!(!should_cull_triangle(-1.0, PolygonMode::CULL_BACK, PolygonMode::WINDING_CW));
        assert!(should_cull_triangle(1.0, PolygonMode::CULL_BACK, PolygonMode::WINDING_CW));
    }

    #[test]
    fn composed_rotation_round_trips_through_axis_angle() {
        let composed = multiply_matrix(rotation_matrix(31.0, 0.0, 0.0, 1.0), rotation_matrix(79.0, 1.0, 0.0, 0.0));
        let (angle, ax, ay, az) = rotation_matrix_to_axis_angle(composed);
        let round_trip = rotation_matrix(angle, ax, ay, az);

        for (actual, expected) in round_trip.iter().zip(composed) {
            assert!((actual - expected).abs() < 0.0001, "{actual} != {expected}");
        }
    }
}
