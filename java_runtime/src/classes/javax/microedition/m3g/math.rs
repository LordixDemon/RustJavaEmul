use alloc::vec::Vec;

pub(super) struct M3gTriangleHit {
    pub(super) distance: f32,
    pub(super) u: f32,
    pub(super) v: f32,
    pub(super) normal: [f32; 3],
}

pub(super) fn identity_matrix() -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ]
}

pub(super) fn matrix_to_array(values: &[f32]) -> [f32; 16] {
    let mut matrix = identity_matrix();
    for (dst, src) in matrix.iter_mut().zip(values.iter().copied()) {
        *dst = src;
    }
    matrix
}

pub(super) fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    let mut matrix = identity_matrix();
    matrix[3] = tx;
    matrix[7] = ty;
    matrix[11] = tz;
    matrix
}

pub(super) fn scale_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
    let mut matrix = identity_matrix();
    matrix[0] = sx;
    matrix[5] = sy;
    matrix[10] = sz;
    matrix
}

pub(super) fn rotation_matrix(angle_degrees: f32, ax: f32, ay: f32, az: f32) -> [f32; 16] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len <= f32::EPSILON || angle_degrees == 0.0 {
        return identity_matrix();
    }
    let x = ax / len;
    let y = ay / len;
    let z = az / len;
    let angle = angle_degrees * core::f32::consts::PI / 180.0;
    let c = angle.cos();
    let s = angle.sin();
    let t = 1.0 - c;
    [
        t * x * x + c,
        t * x * y - s * z,
        t * x * z + s * y,
        0.0,
        t * x * y + s * z,
        t * y * y + c,
        t * y * z - s * x,
        0.0,
        t * x * z - s * y,
        t * y * z + s * x,
        t * z * z + c,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

pub(super) fn rotation_matrix_to_axis_angle(matrix: [f32; 16]) -> (f32, f32, f32, f32) {
    let trace = matrix[0] + matrix[5] + matrix[10];
    let cos_angle = ((trace - 1.0) * 0.5).clamp(-1.0, 1.0);
    let angle = cos_angle.acos();
    if angle.abs() <= 0.0001 {
        return (0.0, 0.0, 0.0, 1.0);
    }

    let sin_angle = angle.sin();
    let (x, y, z) = if sin_angle.abs() > 0.0001 {
        (
            (matrix[9] - matrix[6]) / (2.0 * sin_angle),
            (matrix[2] - matrix[8]) / (2.0 * sin_angle),
            (matrix[4] - matrix[1]) / (2.0 * sin_angle),
        )
    } else {
        let xx = ((matrix[0] + 1.0) * 0.5).max(0.0).sqrt();
        let yy = ((matrix[5] + 1.0) * 0.5).max(0.0).sqrt();
        let zz = ((matrix[10] + 1.0) * 0.5).max(0.0).sqrt();
        if xx >= yy && xx >= zz {
            (xx, copy_sign(yy, matrix[4] + matrix[1]), copy_sign(zz, matrix[8] + matrix[2]))
        } else if yy >= zz {
            (copy_sign(xx, matrix[4] + matrix[1]), yy, copy_sign(zz, matrix[9] + matrix[6]))
        } else {
            (copy_sign(xx, matrix[8] + matrix[2]), copy_sign(yy, matrix[9] + matrix[6]), zz)
        }
    };

    let len = (x * x + y * y + z * z).sqrt();
    if len <= f32::EPSILON {
        return (0.0, 0.0, 0.0, 1.0);
    }
    (angle * 180.0 / core::f32::consts::PI, x / len, y / len, z / len)
}

fn copy_sign(value: f32, sign: f32) -> f32 {
    if sign < 0.0 { -value.abs() } else { value.abs() }
}

pub(super) fn multiply_matrix(left: [f32; 16], right: [f32; 16]) -> [f32; 16] {
    let mut result = [0.0; 16];
    for row in 0..4 {
        for col in 0..4 {
            result[row * 4 + col] = left[row * 4] * right[col]
                + left[row * 4 + 1] * right[4 + col]
                + left[row * 4 + 2] * right[8 + col]
                + left[row * 4 + 3] * right[12 + col];
        }
    }
    result
}

pub(super) fn transpose_matrix(matrix: [f32; 16]) -> [f32; 16] {
    [
        matrix[0], matrix[4], matrix[8], matrix[12], //
        matrix[1], matrix[5], matrix[9], matrix[13], //
        matrix[2], matrix[6], matrix[10], matrix[14], //
        matrix[3], matrix[7], matrix[11], matrix[15],
    ]
}

pub(super) fn invert_matrix(matrix: [f32; 16]) -> Option<[f32; 16]> {
    let mut rows = [[0.0; 8]; 4];
    for row in 0..4 {
        for col in 0..4 {
            rows[row][col] = matrix[row * 4 + col];
        }
        rows[row][4 + row] = 1.0;
    }

    for col in 0..4 {
        let mut pivot = col;
        let mut pivot_abs = rows[pivot][col].abs();
        for (row, values) in rows.iter().enumerate().skip(col + 1) {
            let candidate = values[col].abs();
            if candidate > pivot_abs {
                pivot = row;
                pivot_abs = candidate;
            }
        }
        if pivot_abs <= f32::EPSILON {
            return None;
        }
        if pivot != col {
            rows.swap(pivot, col);
        }

        let divisor = rows[col][col];
        for value in &mut rows[col] {
            *value /= divisor;
        }

        let pivot_row = rows[col];
        for (row_index, row) in rows.iter_mut().enumerate() {
            if row_index == col {
                continue;
            }
            let factor = row[col];
            if factor == 0.0 {
                continue;
            }
            for col_index in 0..8 {
                row[col_index] -= factor * pivot_row[col_index];
            }
        }
    }

    let mut result = [0.0; 16];
    for row in 0..4 {
        for col in 0..4 {
            result[row * 4 + col] = rows[row][4 + col];
        }
    }
    Some(result)
}

pub(super) fn quaternion_to_axis_angle(qx: f32, qy: f32, qz: f32, qw: f32) -> (f32, f32, f32, f32) {
    let len = (qx * qx + qy * qy + qz * qz + qw * qw).sqrt();
    if len <= f32::EPSILON {
        return (0.0, 0.0, 0.0, 1.0);
    }
    let mut w = (qw / len).clamp(-1.0, 1.0);
    let mut x = qx / len;
    let mut y = qy / len;
    let mut z = qz / len;
    if w < 0.0 {
        w = -w;
        x = -x;
        y = -y;
        z = -z;
    }
    let half_angle = w.acos();
    let sin_half = half_angle.sin();
    if sin_half.abs() <= 1e-4 {
        return (0.0, 0.0, 0.0, 1.0);
    }
    let angle_deg = half_angle * 2.0 * 180.0 / core::f32::consts::PI;
    (angle_deg, x / sin_half, y / sin_half, z / sin_half)
}

pub(super) fn axis_angle_to_quaternion(angle: f32, ax: f32, ay: f32, az: f32) -> [f32; 4] {
    let axis = normalize3([ax, ay, az]).unwrap_or([0.0, 0.0, 1.0]);
    let half = (angle * core::f32::consts::PI / 180.0) * 0.5;
    let sin = half.sin();
    [axis[0] * sin, axis[1] * sin, axis[2] * sin, half.cos()]
}

pub(super) fn nlerp_quaternion(left: [f32; 4], right: [f32; 4], t: f32) -> [f32; 4] {
    let dot = left[0] * right[0] + left[1] * right[1] + left[2] * right[2] + left[3] * right[3];
    let sign = if dot < 0.0 { -1.0 } else { 1.0 };
    let mixed = [
        left[0] + (right[0] * sign - left[0]) * t,
        left[1] + (right[1] * sign - left[1]) * t,
        left[2] + (right[2] * sign - left[2]) * t,
        left[3] + (right[3] * sign - left[3]) * t,
    ];
    let len = (mixed[0] * mixed[0] + mixed[1] * mixed[1] + mixed[2] * mixed[2] + mixed[3] * mixed[3]).sqrt();
    if len <= f32::EPSILON {
        left
    } else {
        [mixed[0] / len, mixed[1] / len, mixed[2] / len, mixed[3] / len]
    }
}

pub(super) fn transform_direction(matrix: [f32; 16], direction: [f32; 3]) -> [f32; 3] {
    [
        matrix[0] * direction[0] + matrix[1] * direction[1] + matrix[2] * direction[2],
        matrix[4] * direction[0] + matrix[5] * direction[1] + matrix[6] * direction[2],
        matrix[8] * direction[0] + matrix[9] * direction[1] + matrix[10] * direction[2],
    ]
}

pub(super) fn quaternion_matrix(qx: f32, qy: f32, qz: f32, qw: f32) -> Option<[f32; 16]> {
    let len = (qx * qx + qy * qy + qz * qz + qw * qw).sqrt();
    if len <= f32::EPSILON {
        return None;
    }
    let x = qx / len;
    let y = qy / len;
    let z = qz / len;
    let w = qw / len;
    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;
    Some([
        1.0 - 2.0 * (yy + zz),
        2.0 * (xy - wz),
        2.0 * (xz + wy),
        0.0,
        2.0 * (xy + wz),
        1.0 - 2.0 * (xx + zz),
        2.0 * (yz - wx),
        0.0,
        2.0 * (xz - wy),
        2.0 * (yz + wx),
        1.0 - 2.0 * (xx + yy),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

pub(super) fn transform_vec4(matrix: [f32; 16], vector: [f32; 4]) -> [f32; 4] {
    [
        matrix[0] * vector[0] + matrix[1] * vector[1] + matrix[2] * vector[2] + matrix[3] * vector[3],
        matrix[4] * vector[0] + matrix[5] * vector[1] + matrix[6] * vector[2] + matrix[7] * vector[3],
        matrix[8] * vector[0] + matrix[9] * vector[1] + matrix[10] * vector[2] + matrix[11] * vector[3],
        matrix[12] * vector[0] + matrix[13] * vector[1] + matrix[14] * vector[2] + matrix[15] * vector[3],
    ]
}

pub(super) fn perspective_projection_matrix(fovy: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let fovy = fovy.clamp(0.001, 179.999) * core::f32::consts::PI / 180.0;
    let aspect = aspect.max(0.001);
    let near = near.max(0.001);
    let far = far.max(near + 0.001);
    let f = 1.0 / (fovy * 0.5).tan();
    [
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        far / (far - near),
        -(near * far) / (far - near),
        0.0,
        0.0,
        1.0,
        0.0,
    ]
}

pub(super) fn parallel_projection_matrix(height: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let height = height.max(0.001);
    let aspect = aspect.max(0.001);
    let range = if (far - near).abs() < 0.001 {
        if far >= near { 0.001 } else { -0.001 }
    } else {
        far - near
    };
    let width = height * aspect;
    [
        2.0 / width,
        0.0,
        0.0,
        0.0,
        0.0,
        2.0 / height,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0 / range,
        -near / range,
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

pub(super) fn transform_point(matrix: [f32; 16], point: [f32; 3]) -> [f32; 3] {
    [
        matrix[0] * point[0] + matrix[1] * point[1] + matrix[2] * point[2] + matrix[3],
        matrix[4] * point[0] + matrix[5] * point[1] + matrix[6] * point[2] + matrix[7],
        matrix[8] * point[0] + matrix[9] * point[1] + matrix[10] * point[2] + matrix[11],
    ]
}

pub(super) fn normalize3(value: [f32; 3]) -> Option<[f32; 3]> {
    let len = dot3(value, value).sqrt();
    if len <= f32::EPSILON {
        return None;
    }
    Some([value[0] / len, value[1] / len, value[2] / len])
}

fn sub3(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

pub(super) fn dot3(left: [f32; 3], right: [f32; 3]) -> f32 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

pub(super) fn cross3(left: [f32; 3], right: [f32; 3]) -> [f32; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

pub(super) fn ray_triangle_intersection(origin: [f32; 3], direction: [f32; 3], p0: [f32; 3], p1: [f32; 3], p2: [f32; 3]) -> Option<M3gTriangleHit> {
    let edge1 = sub3(p1, p0);
    let edge2 = sub3(p2, p0);
    let h = cross3(direction, edge2);
    let det = dot3(edge1, h);
    if det.abs() <= 0.000001 {
        return None;
    }
    let inv_det = 1.0 / det;
    let s = sub3(origin, p0);
    let u = inv_det * dot3(s, h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = cross3(s, edge1);
    let v = inv_det * dot3(direction, q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = inv_det * dot3(edge2, q);
    if distance < 0.0 {
        return None;
    }
    let normal = normalize3(cross3(edge1, edge2)).unwrap_or([0.0, 0.0, 1.0]);
    Some(M3gTriangleHit { distance, u, v, normal })
}

pub(super) fn invert_affine_matrix(matrix: [f32; 16]) -> Option<[f32; 16]> {
    let a00 = matrix[0];
    let a01 = matrix[1];
    let a02 = matrix[2];
    let a10 = matrix[4];
    let a11 = matrix[5];
    let a12 = matrix[6];
    let a20 = matrix[8];
    let a21 = matrix[9];
    let a22 = matrix[10];

    let c00 = a11 * a22 - a12 * a21;
    let c01 = a02 * a21 - a01 * a22;
    let c02 = a01 * a12 - a02 * a11;
    let c10 = a12 * a20 - a10 * a22;
    let c11 = a00 * a22 - a02 * a20;
    let c12 = a02 * a10 - a00 * a12;
    let c20 = a10 * a21 - a11 * a20;
    let c21 = a01 * a20 - a00 * a21;
    let c22 = a00 * a11 - a01 * a10;
    let det = a00 * c00 + a01 * c10 + a02 * c20;
    if det.abs() <= f32::EPSILON {
        return None;
    }
    let inv_det = 1.0 / det;
    let r00 = c00 * inv_det;
    let r01 = c01 * inv_det;
    let r02 = c02 * inv_det;
    let r10 = c10 * inv_det;
    let r11 = c11 * inv_det;
    let r12 = c12 * inv_det;
    let r20 = c20 * inv_det;
    let r21 = c21 * inv_det;
    let r22 = c22 * inv_det;
    let tx = matrix[3];
    let ty = matrix[7];
    let tz = matrix[11];

    Some([
        r00,
        r01,
        r02,
        -(r00 * tx + r01 * ty + r02 * tz),
        r10,
        r11,
        r12,
        -(r10 * tx + r11 * ty + r12 * tz),
        r20,
        r21,
        r22,
        -(r20 * tx + r21 * ty + r22 * tz),
        0.0,
        0.0,
        0.0,
        1.0,
    ])
}

pub(super) fn expand_triangle_strips(indices: &[i32], lengths: &[i32]) -> Vec<[usize; 3]> {
    let mut triangles = Vec::new();
    let mut explicit_offset = 0usize;
    let mut implicit_start = indices.first().copied().unwrap_or(0).max(0) as usize;
    let explicit = indices.len() > 1;

    for &length in lengths {
        let length = length.max(0) as usize;
        if length < 3 {
            if explicit {
                explicit_offset = explicit_offset.saturating_add(length);
            } else {
                implicit_start = implicit_start.saturating_add(length);
            }
            continue;
        }

        let mut strip = Vec::with_capacity(length);
        if explicit {
            for index in indices.iter().skip(explicit_offset).take(length) {
                strip.push((*index).max(0) as usize);
            }
            explicit_offset = explicit_offset.saturating_add(length);
        } else {
            strip.extend(implicit_start..implicit_start + length);
            implicit_start = implicit_start.saturating_add(length);
        }

        for i in 2..strip.len() {
            if i % 2 == 0 {
                triangles.push([strip[i - 2], strip[i - 1], strip[i]]);
            } else {
                triangles.push([strip[i - 1], strip[i - 2], strip[i]]);
            }
        }
    }

    triangles
}
