use alloc::{vec, vec::Vec};

use super::{
    binary::{ByteLoader, ParseResult},
    constants::{
        BONE_STRIDE, MAT_COLORKEY, MAT_DOUBLE_FACE, MAT_FLAT_NORMAL, MAT_LIGHTING, MAT_MASK, MAT_SPECULAR, MAT_ZSORT_FAR,
        MAT_ZSORT_NEAR, PATTERN_STRIDE, QUAD_C_STRIDE, QUAD_T_STRIDE, TRI_C_STRIDE, TRI_T_STRIDE,
    },
};

pub(super) struct NativeFigure {
    pub(super) vertex_count: i32,
    pub(super) vertices: Vec<i16>,
    pub(super) poly_c3: Vec<i16>,
    pub(super) poly_c4: Vec<i16>,
    pub(super) poly_t3: Vec<i16>,
    pub(super) poly_t4: Vec<i16>,
    pub(super) colors: Vec<i32>,
    pub(super) patterns: Vec<i32>,
    pub(super) pattern_count: i32,
    pub(super) pattern_slots: i32,
    pub(super) bones: Vec<i32>,
    pub(super) num_poly_c3: i32,
    pub(super) num_poly_c4: i32,
    pub(super) num_poly_t3: i32,
    pub(super) num_poly_t4: i32,
    pub(super) all_mats_or: i32,
    pub(super) all_mats_and: i32,
}

impl NativeFigure {
    pub(super) fn empty() -> Self {
        Self {
            vertex_count: 0,
            vertices: Vec::new(),
            poly_c3: Vec::new(),
            poly_c4: Vec::new(),
            poly_t3: Vec::new(),
            poly_t4: Vec::new(),
            colors: Vec::new(),
            patterns: vec![0, 0, 0, 0],
            pattern_count: 1,
            pattern_slots: 1,
            bones: Vec::new(),
            num_poly_c3: 0,
            num_poly_c4: 0,
            num_poly_t3: 0,
            num_poly_t4: 0,
            all_mats_or: 0,
            all_mats_and: MAT_MASK,
        }
    }
}

pub(super) fn parse(data: &[u8]) -> ParseResult<NativeFigure> {
    let mut loader = ByteLoader::new(data);
    if loader.read_u8()? != b'M' || loader.read_u8()? != b'B' {
        return Err("not a MBAC file");
    }
    let version = loader.read_u8()?;
    if loader.read_u8()? != 0 || !(2..=5).contains(&version) {
        return Err("unsupported MBAC version");
    }

    let mut vertex_format = 1;
    let mut normal_format = 0;
    let mut polygon_format = 1;
    let mut bone_format = 1;
    if version > 3 {
        vertex_format = loader.read_u8()? as i32;
        normal_format = loader.read_u8()? as i32;
        polygon_format = loader.read_u8()? as i32;
        bone_format = loader.read_u8()? as i32;
    }
    if bone_format != 1 {
        return Err("unsupported MBAC bone format");
    }

    let num_vertices = loader.read_u16()? as usize;
    let num_poly_t3 = loader.read_u16()? as usize;
    let num_poly_t4 = loader.read_u16()? as usize;
    let num_bones = loader.read_u16()? as usize;

    let mut num_poly_c3 = 0usize;
    let mut num_poly_c4 = 0usize;
    let mut num_textures = 1usize;
    let mut num_patterns = 1usize;
    let mut num_colors = 0usize;
    if polygon_format >= 3 {
        num_poly_c3 = loader.read_u16()? as usize;
        num_poly_c4 = loader.read_u16()? as usize;
        num_textures = loader.read_u16()? as usize;
        num_patterns = loader.read_u16()? as usize;
        num_colors = loader.read_u16()? as usize;
    }

    if num_vertices > 65535 || num_textures > 64 || num_patterns > 64 || num_colors > 1024 {
        return Err("MBAC limits exceeded");
    }

    let pattern_slots = num_textures + 1;
    let mut patterns = vec![0; num_patterns * pattern_slots * PATTERN_STRIDE];
    if version == 5 {
        let mut c3_offset = 0i32;
        let mut c4_offset = 0i32;
        let mut t3_offset = 0i32;
        let mut t4_offset = 0i32;
        for p in 0..num_patterns {
            let base = p * pattern_slots * PATTERN_STRIDE;
            let c3_count = loader.read_u16()? as i32;
            let c4_count = loader.read_u16()? as i32;
            patterns[base] = c3_offset;
            patterns[base + 1] = c4_offset;
            patterns[base + 2] = c3_count;
            patterns[base + 3] = c4_count;
            c3_offset += c3_count;
            c4_offset += c4_count;

            for t in 1..=num_textures {
                let base = (p * pattern_slots + t) * PATTERN_STRIDE;
                let t3_count = loader.read_u16()? as i32;
                let t4_count = loader.read_u16()? as i32;
                patterns[base] = t3_offset;
                patterns[base + 1] = t4_offset;
                patterns[base + 2] = t3_count;
                patterns[base + 3] = t4_count;
                t3_offset += t3_count;
                t4_offset += t4_count;
            }
        }
    } else {
        patterns[2] = num_poly_c3 as i32;
        patterns[3] = num_poly_c4 as i32;
        let tex_base = PATTERN_STRIDE;
        patterns[tex_base + 2] = num_poly_t3 as i32;
        patterns[tex_base + 3] = num_poly_t4 as i32;
    }

    let vertices = read_vertices(&mut loader, num_vertices, vertex_format)?;
    loader.clear_cache();
    skip_normals(&mut loader, num_vertices, normal_format)?;
    loader.clear_cache();

    let mut all_mats_or = 0i32;
    let mut all_mats_and = MAT_MASK;
    let (colors, poly_c3, poly_c4) = if num_poly_c3 + num_poly_c4 > 0 {
        read_colored_polys(&mut loader, num_poly_c3, num_poly_c4, num_colors, &mut all_mats_or, &mut all_mats_and)?
    } else {
        (Vec::new(), Vec::new(), Vec::new())
    };

    let (poly_t3, poly_t4) = if num_poly_t3 + num_poly_t4 > 0 {
        read_textured_polys(&mut loader, num_poly_t3, num_poly_t4, polygon_format, &mut all_mats_or, &mut all_mats_and)?
    } else {
        (Vec::new(), Vec::new())
    };
    loader.clear_cache();

    let bones = read_bones(&mut loader, num_bones)?;

    Ok(NativeFigure {
        vertex_count: num_vertices as i32,
        vertices,
        poly_c3,
        poly_c4,
        poly_t3,
        poly_t4,
        colors,
        patterns,
        pattern_count: num_patterns as i32,
        pattern_slots: pattern_slots as i32,
        bones,
        num_poly_c3: num_poly_c3 as i32,
        num_poly_c4: num_poly_c4 as i32,
        num_poly_t3: num_poly_t3 as i32,
        num_poly_t4: num_poly_t4 as i32,
        all_mats_or,
        all_mats_and,
    })
}

pub(super) fn unpack_uv(value: i16) -> (i32, i32) {
    let value = value as u16;
    (((value >> 8) & 0xff) as i32, (value & 0xff) as i32)
}

fn read_vertices(loader: &mut ByteLoader<'_>, count: usize, format: i32) -> ParseResult<Vec<i16>> {
    let mut vertices = Vec::with_capacity(count * 3);
    if format == 1 {
        for _ in 0..count * 3 {
            vertices.push(loader.read_i16()?);
        }
        return Ok(vertices);
    }
    if format != 2 {
        return Err("unsupported MBAC vertex format");
    }

    const SIZES: [u32; 4] = [8, 10, 13, 16];
    while vertices.len() < count * 3 {
        let chunk = loader.read_ubits(8)? as usize;
        let size = SIZES[(chunk >> 6) & 3];
        let run = (chunk & 0x3f) + 1;
        for _ in 0..run {
            vertices.push(loader.read_bits(size)? as i16);
            vertices.push(loader.read_bits(size)? as i16);
            vertices.push(loader.read_bits(size)? as i16);
            if vertices.len() >= count * 3 {
                break;
            }
        }
    }
    Ok(vertices)
}

fn skip_normals(loader: &mut ByteLoader<'_>, count: usize, format: i32) -> ParseResult<()> {
    match format {
        0 => Ok(()),
        1 => loader.skip(count * 6),
        2 => {
            for _ in 0..count {
                let x = loader.read_ubits(7)?;
                if x == 64 {
                    loader.read_ubits(3)?;
                } else {
                    loader.read_ubits(7)?;
                    loader.read_ubits(1)?;
                }
            }
            Ok(())
        }
        _ => Err("unsupported MBAC normal format"),
    }
}

fn read_colored_polys(
    loader: &mut ByteLoader<'_>,
    num_c3: usize,
    num_c4: usize,
    num_colors: usize,
    all_mats_or: &mut i32,
    all_mats_and: &mut i32,
) -> ParseResult<(Vec<i32>, Vec<i16>, Vec<i16>)> {
    let mat_bits = loader.read_u8()? as u32;
    let vertex_bits = loader.read_u8()? as u32;
    let color_bits = loader.read_u8()? as u32;
    let color_idx_bits = loader.read_u8()? as u32;
    loader.read_u8()?;

    let max_color = ((1i32 << color_bits.min(24)) - 1).max(1);
    let mut colors = Vec::with_capacity(num_colors);
    for _ in 0..num_colors {
        let r = 255 * loader.read_ubits(color_bits)? / max_color;
        let g = 255 * loader.read_ubits(color_bits)? / max_color;
        let b = 255 * loader.read_ubits(color_bits)? / max_color;
        colors.push(0xff00_0000u32 as i32 | (r << 16) | (g << 8) | b);
    }

    let mut c3 = Vec::with_capacity(num_c3 * TRI_C_STRIDE);
    for _ in 0..num_c3 {
        let mat = (loader.read_ubits(mat_bits)? << 1) & MAT_MASK;
        let a = loader.read_ubits(vertex_bits)? as i16;
        let b = loader.read_ubits(vertex_bits)? as i16;
        let c = loader.read_ubits(vertex_bits)? as i16;
        let color = loader.read_ubits(color_idx_bits)? as i16;
        push_mat(mat, all_mats_or, all_mats_and);
        c3.extend([mat as i16, color, a, b, c]);
    }

    let mut c4 = Vec::with_capacity(num_c4 * QUAD_C_STRIDE);
    for _ in 0..num_c4 {
        let mat = (loader.read_ubits(mat_bits)? << 1) & MAT_MASK;
        let a = loader.read_ubits(vertex_bits)? as i16;
        let b = loader.read_ubits(vertex_bits)? as i16;
        let c = loader.read_ubits(vertex_bits)? as i16;
        let d = loader.read_ubits(vertex_bits)? as i16;
        let color = loader.read_ubits(color_idx_bits)? as i16;
        push_mat(mat, all_mats_or, all_mats_and);
        c4.extend([mat as i16, color, a, b, c, d]);
    }

    Ok((colors, c3, c4))
}

#[inline]
pub(super) fn map_textured_poly_mat(raw: i32) -> i32 {
    let mut mat = 0;
    if (raw & 0x03) != 0 {
        mat |= MAT_COLORKEY;
    }
    if (raw & 0x14) != 0 {
        mat |= MAT_DOUBLE_FACE;
    }
    if (raw & 0x20) != 0 {
        mat |= MAT_LIGHTING;
    }
    if (raw & 0x40) != 0 {
        mat |= MAT_SPECULAR;
    }
    if (raw & 0x80) != 0 {
        mat |= MAT_FLAT_NORMAL;
    }
    if (raw & 0x100) != 0 {
        mat |= MAT_ZSORT_NEAR;
    }
    if (raw & 0x200) != 0 {
        mat |= MAT_ZSORT_FAR;
    }
    mat
}

fn read_textured_polys(
    loader: &mut ByteLoader<'_>,
    num_t3: usize,
    num_t4: usize,
    format: i32,
    all_mats_or: &mut i32,
    all_mats_and: &mut i32,
) -> ParseResult<(Vec<i16>, Vec<i16>)> {
    let mut t3 = Vec::with_capacity(num_t3 * TRI_T_STRIDE);
    let mut t4 = Vec::with_capacity(num_t4 * QUAD_T_STRIDE);

    if format == 1 {
        for _ in 0..num_t3 {
            let raw = loader.read_u16()? as i32;
            let mat = map_textured_poly_mat(raw);
            let a = loader.read_u16()? as i16;
            let b = loader.read_u16()? as i16;
            let c = loader.read_u16()? as i16;
            let au = loader.read_i8()? as i32 & 0xff;
            let av = loader.read_i8()? as i32 & 0xff;
            let bu = loader.read_i8()? as i32 & 0xff;
            let bv = loader.read_i8()? as i32 & 0xff;
            let cu = loader.read_i8()? as i32 & 0xff;
            let cv = loader.read_i8()? as i32 & 0xff;
            push_tex_tri(&mut t3, mat, a, b, c, au, av, bu, bv, cu, cv, all_mats_or, all_mats_and);
        }
        for _ in 0..num_t4 {
            let raw = loader.read_u16()? as i32;
            let mat = map_textured_poly_mat(raw);
            let a = loader.read_u16()? as i16;
            let b = loader.read_u16()? as i16;
            let c = loader.read_u16()? as i16;
            let d = loader.read_u16()? as i16;
            let au = loader.read_i8()? as i32 & 0xff;
            let av = loader.read_i8()? as i32 & 0xff;
            let bu = loader.read_i8()? as i32 & 0xff;
            let bv = loader.read_i8()? as i32 & 0xff;
            let cu = loader.read_i8()? as i32 & 0xff;
            let cv = loader.read_i8()? as i32 & 0xff;
            let du = loader.read_i8()? as i32 & 0xff;
            let dv = loader.read_i8()? as i32 & 0xff;
            push_tex_quad(&mut t4, mat, a, b, c, d, au, av, bu, bv, cu, cv, du, dv, all_mats_or, all_mats_and);
        }
        return Ok((t3, t4));
    }

    let (mat_bits, vertex_bits, uv_bits) = if format == 2 {
        (loader.read_u8()? as u32, loader.read_u8()? as u32, 7)
    } else {
        let mat = loader.read_ubits(8)? as u32;
        let vertex = loader.read_ubits(8)? as u32;
        let uv = loader.read_ubits(8)? as u32;
        loader.read_ubits(8)?;
        (mat, vertex, uv)
    };

    for _ in 0..num_t3 {
        let raw = loader.read_ubits(mat_bits)?;
        let mat = map_textured_poly_mat(raw);
        let a = loader.read_ubits(vertex_bits)? as i16;
        let b = loader.read_ubits(vertex_bits)? as i16;
        let c = loader.read_ubits(vertex_bits)? as i16;
        let au = loader.read_ubits(uv_bits)?;
        let av = loader.read_ubits(uv_bits)?;
        let bu = loader.read_ubits(uv_bits)?;
        let bv = loader.read_ubits(uv_bits)?;
        let cu = loader.read_ubits(uv_bits)?;
        let cv = loader.read_ubits(uv_bits)?;
        push_tex_tri(&mut t3, mat, a, b, c, au, av, bu, bv, cu, cv, all_mats_or, all_mats_and);
    }

    for _ in 0..num_t4 {
        let raw = loader.read_ubits(mat_bits)?;
        let mat = map_textured_poly_mat(raw);
        let a = loader.read_ubits(vertex_bits)? as i16;
        let b = loader.read_ubits(vertex_bits)? as i16;
        let c = loader.read_ubits(vertex_bits)? as i16;
        let d = loader.read_ubits(vertex_bits)? as i16;
        let au = loader.read_ubits(uv_bits)?;
        let av = loader.read_ubits(uv_bits)?;
        let bu = loader.read_ubits(uv_bits)?;
        let bv = loader.read_ubits(uv_bits)?;
        let cu = loader.read_ubits(uv_bits)?;
        let cv = loader.read_ubits(uv_bits)?;
        let du = loader.read_ubits(uv_bits)?;
        let dv = loader.read_ubits(uv_bits)?;
        push_tex_quad(&mut t4, mat, a, b, c, d, au, av, bu, bv, cu, cv, du, dv, all_mats_or, all_mats_and);
    }

    Ok((t3, t4))
}

#[allow(clippy::too_many_arguments)]
fn push_tex_tri(
    out: &mut Vec<i16>,
    mat: i32,
    a: i16,
    b: i16,
    c: i16,
    au: i32,
    av: i32,
    bu: i32,
    bv: i32,
    cu: i32,
    cv: i32,
    all_mats_or: &mut i32,
    all_mats_and: &mut i32,
) {
    let mat = mat & MAT_MASK;
    push_mat(mat, all_mats_or, all_mats_and);
    out.extend([mat as i16, a, b, c, pack_uv(au, av), pack_uv(bu, bv), pack_uv(cu, cv)]);
}

#[allow(clippy::too_many_arguments)]
fn push_tex_quad(
    out: &mut Vec<i16>,
    mat: i32,
    a: i16,
    b: i16,
    c: i16,
    d: i16,
    au: i32,
    av: i32,
    bu: i32,
    bv: i32,
    cu: i32,
    cv: i32,
    du: i32,
    dv: i32,
    all_mats_or: &mut i32,
    all_mats_and: &mut i32,
) {
    let mat = mat & MAT_MASK;
    push_mat(mat, all_mats_or, all_mats_and);
    out.extend([mat as i16, a, b, c, d, pack_uv(au, av), pack_uv(bu, bv), pack_uv(cu, cv), pack_uv(du, dv)]);
}

fn push_mat(mat: i32, all_mats_or: &mut i32, all_mats_and: &mut i32) {
    *all_mats_or |= mat;
    *all_mats_and &= mat;
}

fn pack_uv(u: i32, v: i32) -> i16 {
    (((u & 0xff) << 8) | (v & 0xff)) as i16
}

fn read_bones(loader: &mut ByteLoader<'_>, count: usize) -> ParseResult<Vec<i32>> {
    let mut bones = Vec::with_capacity(count * BONE_STRIDE);
    let mut vertex_start = 0i32;
    for _ in 0..count {
        let vertex_count = loader.read_u16()? as i32;
        let parent = loader.read_i16()? as i32;
        bones.push(vertex_start);
        bones.push(vertex_count);
        bones.push(parent);
        for _ in 0..12 {
            bones.push(loader.read_i16()? as i32);
        }
        vertex_start += vertex_count;
    }
    Ok(bones)
}

