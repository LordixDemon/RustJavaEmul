use super::constants::BONE_STRIDE;

pub(super) fn identity_matrix() -> [i32; 12] {
    [4096, 0, 0, 0, 0, 4096, 0, 0, 0, 0, 4096, 0]
}

pub(super) fn mbac_bone_local_matrix(bones: &[i32], bone_index: usize) -> [i32; 12] {
    let offset = bone_index * BONE_STRIDE + 3;
    if offset + 12 > bones.len() {
        return identity_matrix();
    }

    let mut matrix = [0; 12];
    matrix.copy_from_slice(&bones[offset..offset + 12]);
    matrix
}

pub(super) fn mul_matrix(lhs: [i32; 12], rhs: [i32; 12]) -> [i32; 12] {
    [
        fixed_mul3(lhs[0], rhs[0], lhs[1], rhs[4], lhs[2], rhs[8]),
        fixed_mul3(lhs[0], rhs[1], lhs[1], rhs[5], lhs[2], rhs[9]),
        fixed_mul3(lhs[0], rhs[2], lhs[1], rhs[6], lhs[2], rhs[10]),
        fixed_mul3(lhs[0], rhs[3], lhs[1], rhs[7], lhs[2], rhs[11]) + lhs[3],
        fixed_mul3(lhs[4], rhs[0], lhs[5], rhs[4], lhs[6], rhs[8]),
        fixed_mul3(lhs[4], rhs[1], lhs[5], rhs[5], lhs[6], rhs[9]),
        fixed_mul3(lhs[4], rhs[2], lhs[5], rhs[6], lhs[6], rhs[10]),
        fixed_mul3(lhs[4], rhs[3], lhs[5], rhs[7], lhs[6], rhs[11]) + lhs[7],
        fixed_mul3(lhs[8], rhs[0], lhs[9], rhs[4], lhs[10], rhs[8]),
        fixed_mul3(lhs[8], rhs[1], lhs[9], rhs[5], lhs[10], rhs[9]),
        fixed_mul3(lhs[8], rhs[2], lhs[9], rhs[6], lhs[10], rhs[10]),
        fixed_mul3(lhs[8], rhs[3], lhs[9], rhs[7], lhs[10], rhs[11]) + lhs[11],
    ]
}

pub(super) fn fixed_mul3(a0: i32, v0: i32, a1: i32, v1: i32, a2: i32, v2: i32) -> i32 {
    ((a0 as i64 * v0 as i64 + a1 as i64 * v1 as i64 + a2 as i64 * v2 as i64 + 2048) >> 12) as i32
}

pub(super) fn set_rotation_from_vector(matrix: &mut [i32; 12], vector: (i32, i32, i32)) {
    let (x, y, z) = normalize3(vector);
    let xx = fixed_mul(x, x);
    let yy = fixed_mul(y, y);

    if xx > 0 || yy > 0 {
        let a = (((4096 - z) as i64) << 12) / (yy + xx) as i64;
        let xy = fixed_mul(x, y);
        let b = ((a * -(xy as i64)) >> 12) as i32;
        matrix[0] = z + (((yy as i64 * a + 2048) >> 12) as i32);
        matrix[1] = b;
        matrix[2] = x;
        matrix[4] = b;
        matrix[5] = z + (((xx as i64 * a + 2048) >> 12) as i32);
        matrix[6] = y;
        matrix[8] = -x;
        matrix[9] = -y;
    } else {
        matrix[0] = 4096;
        matrix[1] = 0;
        matrix[2] = 0;
        matrix[4] = 0;
        matrix[5] = z;
        matrix[6] = 0;
        matrix[8] = 0;
        matrix[9] = 0;
    }

    matrix[10] = z;
}

pub(super) fn roll_matrix(matrix: &mut [i32; 12], angle: i32) {
    let (sin, cos) = sin_cos_mc(angle);
    let m00 = matrix[0];
    let m01 = matrix[1];
    let m10 = matrix[4];
    let m11 = matrix[5];
    let m20 = matrix[8];
    let m21 = matrix[9];

    matrix[0] = fixed_mul2_add(m00, cos, m01, sin);
    matrix[1] = fixed_mul2_add(m01, cos, -m00, sin);
    matrix[4] = fixed_mul2_add(m10, cos, m11, sin);
    matrix[5] = fixed_mul2_add(m11, cos, -m10, sin);
    matrix[8] = fixed_mul2_add(m20, cos, m21, sin);
    matrix[9] = fixed_mul2_add(m21, cos, -m20, sin);
}

pub(super) fn scale_matrix(matrix: &mut [i32; 12], scale: (i32, i32, i32)) {
    matrix[0] = fixed_mul(matrix[0], scale.0);
    matrix[1] = fixed_mul(matrix[1], scale.1);
    matrix[2] = fixed_mul(matrix[2], scale.2);
    matrix[4] = fixed_mul(matrix[4], scale.0);
    matrix[5] = fixed_mul(matrix[5], scale.1);
    matrix[6] = fixed_mul(matrix[6], scale.2);
    matrix[8] = fixed_mul(matrix[8], scale.0);
    matrix[9] = fixed_mul(matrix[9], scale.1);
    matrix[10] = fixed_mul(matrix[10], scale.2);
}

pub(super) fn cross3(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    (
        clamp_i64_to_i32(a.1 as i64 * b.2 as i64 - a.2 as i64 * b.1 as i64),
        clamp_i64_to_i32(a.2 as i64 * b.0 as i64 - a.0 as i64 * b.2 as i64),
        clamp_i64_to_i32(a.0 as i64 * b.1 as i64 - a.1 as i64 * b.0 as i64),
    )
}

pub(super) fn dot3(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    (((a.0 as i64 * b.0 as i64 + a.1 as i64 * b.1 as i64 + a.2 as i64 * b.2 as i64) + 2048) >> 12) as i32
}

pub(super) fn normalize3(v: (i32, i32, i32)) -> (i32, i32, i32) {
    let len_sq = v.0 as i64 * v.0 as i64 + v.1 as i64 * v.1 as i64 + v.2 as i64 * v.2 as i64;
    if len_sq <= 0 {
        return (0, 0, 4096);
    }
    let len = isqrt(len_sq as u64).max(1) as i64;
    (
        ((v.0 as i64 * 4096) / len) as i32,
        ((v.1 as i64 * 4096) / len) as i32,
        ((v.2 as i64 * 4096) / len) as i32,
    )
}

pub(super) fn sin_cos_mc(angle: i32) -> (i32, i32) {
    (sin_mc(angle), sin_mc(angle + 1024))
}

pub(super) fn fixed_mul(value: i32, scale: i32) -> i32 {
    (((value as i64 * scale as i64) + 2048) >> 12) as i32
}

fn fixed_mul2_add(a: i32, b: i32, c: i32, d: i32) -> i32 {
    (((a as i64 * b as i64 + c as i64 * d as i64) + 2048) >> 12) as i32
}

fn clamp_i64_to_i32(value: i64) -> i32 {
    value.max(i32::MIN as i64).min(i32::MAX as i64) as i32
}

pub(super) fn isqrt(value: u64) -> u64 {
    if value == 0 {
        return 0;
    }
    let mut x = value;
    let mut y = x.div_ceil(2);
    while y < x {
        x = y;
        y = (x + value / x) / 2;
    }
    x
}

pub(super) fn sin_mc(angle: i32) -> i32 {
    let mut angle = angle.rem_euclid(4096);
    let sign = if angle >= 2048 {
        angle -= 2048;
        -1
    } else {
        1
    };
    if angle == 0 {
        return 0;
    }

    let x = angle as i64;
    let span = 2048i64;
    let curve = x * (span - x);
    let denominator = 5 * span * span - 4 * curve;
    let value = if denominator == 0 { 0 } else { (16 * curve * 4096) / denominator };
    sign * value as i32
}
