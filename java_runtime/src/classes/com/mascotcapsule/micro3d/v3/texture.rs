use alloc::{sync::Arc, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{DecodedImage, RuntimeClassProto, RuntimeContext, classes::java::lang::String};

use super::{
    constants::TEXTURE_CLASS,
    storage::{invalidate_native_texture_cache, load_byte_array, load_resource_bytes_with_extensions, put_byte_array_field, put_int_array_field},
};

// class com.mascotcapsule.micro3d.v3.Texture
pub struct Texture;

impl Texture {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: TEXTURE_CLASS,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;Z)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "([BZ)V", Self::init_from_bytes, Default::default()),
                JavaMethodProto::new("dispose", "()V", Self::dispose, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("resourceName", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("mipMap", "Z", Default::default()),
                JavaFieldProto::new("isForModel", "Z", Default::default()),
                JavaFieldProto::new("isSphere", "Z", Default::default()),
                JavaFieldProto::new("width", "I", Default::default()),
                JavaFieldProto::new("height", "I", Default::default()),
                JavaFieldProto::new("pixels", "[I", Default::default()),
                JavaFieldProto::new("indices", "[B", Default::default()),
                JavaFieldProto::new("palette", "[I", Default::default()),
                JavaFieldProto::new("colorKey", "I", Default::default()),
                JavaFieldProto::new("disposed", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        resource_name: ClassInstanceRef<String>,
        mip_map: bool,
    ) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Texture::<init>({this:?}, {resource_name:?}, {mip_map:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let bytes = load_resource_bytes_with_extensions(jvm, context, &resource_name, &[".bmp", ".png"])
            .await
            .unwrap_or_default();
        Self::put_decoded_texture(jvm, context, &mut this, &bytes).await?;
        jvm.put_field(&mut this, "resourceName", "Ljava/lang/String;", resource_name).await?;
        jvm.put_field(&mut this, "mipMap", "Z", mip_map).await?;
        jvm.put_field(&mut this, "isForModel", "Z", mip_map).await?;
        jvm.put_field(&mut this, "isSphere", "Z", !mip_map).await?;
        jvm.put_field(&mut this, "disposed", "Z", false).await
    }

    async fn init_from_bytes(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
        mip_map: bool,
    ) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Texture::<init>({this:?}, {data:?}, {mip_map:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let bytes = load_byte_array(jvm, &data).await?;
        Self::put_decoded_texture(jvm, context, &mut this, &bytes).await?;
        jvm.put_field(&mut this, "resourceName", "Ljava/lang/String;", ClassInstanceRef::<String>::new(None))
            .await?;
        jvm.put_field(&mut this, "mipMap", "Z", mip_map).await?;
        jvm.put_field(&mut this, "isForModel", "Z", mip_map).await?;
        jvm.put_field(&mut this, "isSphere", "Z", !mip_map).await?;
        jvm.put_field(&mut this, "disposed", "Z", false).await
    }

    async fn put_decoded_texture(jvm: &Jvm, context: &mut RuntimeContext, this: &mut ClassInstanceRef<Self>, data: &[u8]) -> Result<()> {
        if let Some(indexed) = decode_indexed_bmp(data) {
            let pixels = indexed
                .indices
                .iter()
                .map(|index| indexed.palette.get(*index as usize).copied().unwrap_or(0xffff_00ffu32 as i32))
                .collect();
            let color_key = indexed.palette.first().copied().unwrap_or(0xff00_0000u32 as i32);
            jvm.put_field(this, "width", "I", indexed.width).await?;
            jvm.put_field(this, "height", "I", indexed.height).await?;
            jvm.put_field(this, "colorKey", "I", color_key).await?;
            put_byte_array_field(jvm, this, "indices", indexed.indices.into_iter().map(|value| value as i8).collect()).await?;
            put_int_array_field(jvm, this, "palette", indexed.palette).await?;
            return put_int_array_field(jvm, this, "pixels", pixels).await;
        }

        let decoded = context.decode_image(data).unwrap_or_else(fallback_texture);
        jvm.put_field(this, "width", "I", decoded.width).await?;
        jvm.put_field(this, "height", "I", decoded.height).await?;
        jvm.put_field(this, "colorKey", "I", 0xff00_0000u32 as i32).await?;
        put_int_array_field(jvm, this, "pixels", decoded.argb).await
    }

    async fn dispose(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Texture::dispose({this:?})");

        invalidate_native_texture_cache(&this);
        jvm.put_field(&mut this, "disposed", "Z", true).await
    }
}

#[derive(Clone)]
pub(super) struct NativeTexture {
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) pixels: Arc<Vec<i32>>,
    pub(super) indices: Arc<Vec<u8>>,
    pub(super) palette: Arc<Vec<i32>>,
    pub(super) color_key: i32,
}

pub(super) struct IndexedTextureData {
    pub(super) width: i32,
    pub(super) height: i32,
    pub(super) indices: Vec<u8>,
    pub(super) palette: Vec<i32>,
}

pub(super) fn decode_indexed_bmp(data: &[u8]) -> Option<IndexedTextureData> {
    if data.len() < 32 || data.get(0..2)? != b"BM" {
        return None;
    }

    let pixel_offset = read_u32_le_at(data, 10)? as usize;
    let dib_size = read_u32_le_at(data, 14)? as usize;
    let (width, raw_height, bits_per_pixel, compression, colors_used, palette_offset, palette_entry_size) = if dib_size == 12 {
        (
            read_u16_le_at(data, 18)? as i32,
            read_u16_le_at(data, 20)? as i32,
            read_u16_le_at(data, 24)?,
            0,
            0,
            14usize.checked_add(dib_size)?,
            3usize,
        )
    } else {
        (
            read_i32_le_at(data, 18)?,
            read_i32_le_at(data, 22)?,
            read_u16_le_at(data, 28)?,
            read_u32_le_at(data, 30)?,
            if dib_size >= 40 {
                read_u32_le_at(data, 46).unwrap_or(0) as usize
            } else {
                0
            },
            14usize.checked_add(dib_size)?,
            4usize,
        )
    };

    if width <= 0 || raw_height == 0 || compression != 0 || bits_per_pixel > 8 || bits_per_pixel == 0 {
        return None;
    }

    let width = width as usize;
    let height = raw_height.unsigned_abs() as usize;
    let padded_width = width.checked_next_power_of_two()?;
    let padded_height = height.checked_next_power_of_two()?;
    let top_down = raw_height < 0;
    let color_count = if colors_used == 0 { 1usize << bits_per_pixel } else { colors_used };
    let palette_bytes = color_count.checked_mul(palette_entry_size)?;
    if palette_offset.checked_add(palette_bytes)? > data.len() {
        return None;
    }

    let mut palette = vec![0xff00_0000u32 as i32; 256];
    for (index, color) in palette.iter_mut().enumerate().take(color_count.min(256)) {
        let offset = palette_offset + index * palette_entry_size;
        let b = data[offset] as u32;
        let g = data[offset + 1] as u32;
        let r = data[offset + 2] as u32;
        *color = (0xff00_0000 | (r << 16) | (g << 8) | b) as i32;
    }

    let row_stride = ((width * bits_per_pixel as usize).div_ceil(32)) * 4;
    if pixel_offset.checked_add(row_stride.checked_mul(height)?)? > data.len() {
        return None;
    }

    let mut indices = vec![0; padded_width.checked_mul(padded_height)?];
    for y in 0..height {
        let src_y = if top_down { y } else { height - 1 - y };
        let row_offset = pixel_offset + src_y * row_stride;
        for x in 0..width {
            let index = match bits_per_pixel {
                8 => data[row_offset + x],
                4 => {
                    let packed = data[row_offset + x / 2];
                    if x & 1 == 0 { packed >> 4 } else { packed & 0x0f }
                }
                1 => {
                    let packed = data[row_offset + x / 8];
                    (packed >> (7 - (x & 7))) & 1
                }
                _ => return None,
            };
            indices[y * padded_width + x] = index;
        }
    }

    Some(IndexedTextureData {
        width: padded_width as i32,
        height: padded_height as i32,
        indices,
        palette,
    })
}

fn read_u16_le_at(data: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_le_bytes(data.get(offset..offset + 2)?.try_into().ok()?))
}

fn read_u32_le_at(data: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(offset..offset + 4)?.try_into().ok()?))
}

fn read_i32_le_at(data: &[u8], offset: usize) -> Option<i32> {
    Some(i32::from_le_bytes(data.get(offset..offset + 4)?.try_into().ok()?))
}

pub(super) fn fallback_texture() -> DecodedImage {
    DecodedImage {
        width: 2,
        height: 2,
        argb: vec![0xffff_00ffu32 as i32, 0xff20_2020u32 as i32, 0xff20_2020u32 as i32, 0xffff_00ffu32 as i32],
    }
}
