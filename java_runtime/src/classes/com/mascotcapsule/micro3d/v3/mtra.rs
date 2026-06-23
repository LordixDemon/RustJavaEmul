use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

use super::{
    binary::{ByteLoader, ParseResult},
    constants::{ACTION_TABLE_CLASS, BONE_STRIDE},
    math::{identity_matrix, mbac_bone_local_matrix, mul_matrix, roll_matrix, scale_matrix, set_rotation_from_vector},
    storage::{load_byte_array, load_resource_bytes_with_extensions, put_int_array_field},
};

// class com.mascotcapsule.micro3d.v3.ActionTable
pub struct ActionTable;

impl ActionTable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: ACTION_TABLE_CLASS,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_from_resource, Default::default()),
                JavaMethodProto::new("dispose", "()V", Self::dispose, Default::default()),
                JavaMethodProto::new("getNumAction", "()I", Self::get_num_actions, Default::default()),
                JavaMethodProto::new("getNumActions", "()I", Self::get_num_actions, Default::default()),
                JavaMethodProto::new("getNumFrame", "(I)I", Self::get_num_frames, Default::default()),
                JavaMethodProto::new("getNumFrames", "(I)I", Self::get_num_frames, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("data", "[B", Default::default()),
                JavaFieldProto::new("frameCounts", "[I", Default::default()),
                JavaFieldProto::new("valid", "Z", Default::default()),
                JavaFieldProto::new("disposed", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, data: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.ActionTable::<init>({this:?}, {data:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let bytes = load_byte_array(jvm, &data).await?;
        let (frame_counts, valid) = if bytes.is_empty() {
            (Vec::new(), false)
        } else {
            match parse_frame_counts(&bytes) {
                Ok(frame_counts) => (frame_counts, true),
                Err(err) => {
                    tracing::warn!("MTRA parse failed: {err}");
                    (Vec::new(), false)
                }
            }
        };
        put_int_array_field(jvm, &mut this, "frameCounts", frame_counts).await?;
        jvm.put_field(&mut this, "data", "[B", data).await?;
        jvm.put_field(&mut this, "valid", "Z", valid).await?;
        jvm.put_field(&mut this, "disposed", "Z", false).await
    }

    async fn init_from_resource(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        resource_name: ClassInstanceRef<String>,
    ) -> Result<()> {
        let bytes = load_resource_bytes_with_extensions(jvm, context, &resource_name, &[".mtra"])
            .await
            .unwrap_or_default();
        let mut data = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_array(&mut data, 0, bytes.into_iter().map(|value| value as i8)).await?;
        let data: ClassInstanceRef<Array<i8>> = data.into();
        Self::init(jvm, context, this, data).await
    }

    async fn dispose(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.ActionTable::dispose({this:?})");

        jvm.put_field(&mut this, "valid", "Z", false).await?;
        jvm.put_field(&mut this, "disposed", "Z", true).await
    }

    async fn get_num_actions(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let frame_counts: ClassInstanceRef<Array<i32>> = jvm.get_field(&this, "frameCounts", "[I").await?;
        if frame_counts.is_null() {
            Ok(1)
        } else {
            Ok(jvm.array_length(&frame_counts).await? as i32)
        }
    }

    async fn get_num_frames(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, action: i32) -> Result<i32> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.ActionTable::getNumFrames({this:?}, {action:?})");

        let frame_counts: ClassInstanceRef<Array<i32>> = jvm.get_field(&this, "frameCounts", "[I").await?;
        if frame_counts.is_null() {
            return Ok(1);
        }

        let length = jvm.array_length(&frame_counts).await? as i32;
        if action < 0 || action >= length {
            return Ok(1);
        }

        let values: Vec<i32> = jvm.load_array(&frame_counts, action as usize, 1).await?;
        Ok(values.first().copied().unwrap_or(1))
    }
}

fn parse_frame_counts(data: &[u8]) -> ParseResult<Vec<i32>> {
    let mut loader = ByteLoader::new(data);
    if loader.read_u8()? != b'M' || loader.read_u8()? != b'T' {
        return Err("not a MTRA file");
    }
    let version = loader.read_u8()?;
    if loader.read_u8()? != 0 || !(2..=5).contains(&version) {
        return Err("unsupported MTRA version");
    }

    let action_count = loader.read_u16()? as usize;
    let bone_count = loader.read_u16()? as usize;
    for _ in 0..8 {
        loader.read_u16()?;
    }
    loader.read_i32()?;

    let mut frame_counts = Vec::with_capacity(action_count);
    for _ in 0..action_count {
        let keyframes = loader.read_u16()? as i32;
        frame_counts.push((keyframes.max(1)) << 16);
        for _ in 0..bone_count {
            skip_bone_anim(&mut loader)?;
        }
        if version >= 5 {
            let dynamic_count = loader.read_u16()? as usize;
            loader.skip(dynamic_count * 6)?;
        }
    }

    Ok(frame_counts)
}

pub(super) fn compute_posture(data: &[u8], action_index: i32, frame: i32, bones: &[i32], previous_pattern: i32) -> ParseResult<(Vec<i32>, i32)> {
    let mut loader = ByteLoader::new(data);
    if loader.read_u8()? != b'M' || loader.read_u8()? != b'T' {
        return Err("not a MTRA file");
    }
    let version = loader.read_u8()?;
    if loader.read_u8()? != 0 || !(2..=5).contains(&version) {
        return Err("unsupported MTRA version");
    }

    let action_count = loader.read_u16()? as usize;
    let bone_count = loader.read_u16()? as usize;
    for _ in 0..8 {
        loader.read_u16()?;
    }
    loader.read_i32()?;

    if action_index < 0 || action_index as usize >= action_count {
        return Err("invalid MTRA action");
    }

    let figure_bone_count = bones.len() / BONE_STRIDE;
    let mut posture = vec![0; figure_bone_count * 12];
    for bone_index in 0..figure_bone_count {
        let matrix = mbac_bone_local_matrix(bones, bone_index);
        posture[bone_index * 12..bone_index * 12 + 12].copy_from_slice(&matrix);
    }

    let target_action = action_index as usize;
    let mut selected_pattern = previous_pattern;
    for action in 0..action_count {
        loader.read_u16()?;
        let is_target = action == target_action;
        for bone in 0..bone_count {
            if is_target {
                let local = if bone < figure_bone_count {
                    mbac_bone_local_matrix(bones, bone)
                } else {
                    identity_matrix()
                };
                let matrix = read_bone_posture(&mut loader, frame >> 4, local)?;
                if bone < figure_bone_count {
                    posture[bone * 12..bone * 12 + 12].copy_from_slice(&matrix);
                }
            } else {
                skip_bone_anim(&mut loader)?;
            }
        }

        if version >= 5 {
            let dynamic_count = loader.read_u16()? as usize;
            if is_target {
                let mut dynamic = Vec::with_capacity(dynamic_count);
                for _ in 0..dynamic_count {
                    dynamic.push((loader.read_u16()? as i32, loader.read_i32()?));
                }
                let frame_int = frame >> 16;
                for (dynamic_frame, pattern) in dynamic.into_iter().rev() {
                    if dynamic_frame <= frame_int {
                        selected_pattern = pattern;
                        break;
                    }
                }
            } else {
                loader.skip(dynamic_count * 6)?;
            }
        }
    }

    Ok((posture, selected_pattern))
}

fn skip_bone_anim(loader: &mut ByteLoader<'_>) -> ParseResult<()> {
    match loader.read_u8()? {
        0 => loader.skip(24),
        1 => Ok(()),
        2 => {
            skip_frames_3d(loader)?;
            skip_frames_3d(loader)?;
            skip_frames_3d(loader)?;
            skip_frames_1d(loader)
        }
        3 => {
            loader.skip(6)?;
            skip_frames_3d(loader)?;
            loader.skip(2)
        }
        4 => {
            skip_frames_3d(loader)?;
            skip_frames_1d(loader)
        }
        5 => skip_frames_3d(loader),
        6 => {
            skip_frames_3d(loader)?;
            skip_frames_3d(loader)?;
            skip_frames_1d(loader)
        }
        _ => Err("invalid MTRA bone animation"),
    }
}

fn skip_frames_3d(loader: &mut ByteLoader<'_>) -> ParseResult<()> {
    let count = loader.read_u16()? as usize;
    loader.skip(count * 8)
}

fn skip_frames_1d(loader: &mut ByteLoader<'_>) -> ParseResult<()> {
    let count = loader.read_u16()? as usize;
    loader.skip(count * 4)
}

fn read_bone_posture(loader: &mut ByteLoader<'_>, frame: i32, local: [i32; 12]) -> ParseResult<[i32; 12]> {
    match loader.read_u8()? {
        0 => {
            let transform = read_matrix(loader)?;
            Ok(mul_matrix(local, transform))
        }
        1 => Ok(local),
        2 => {
            let translate = read_frames_3d(loader)?;
            let scale = read_frames_3d(loader)?;
            let rotate = read_frames_3d(loader)?;
            let roll = read_frames_1d(loader)?;
            let mut matrix = identity_matrix();
            let translate = interp_3d(frame, &translate);
            matrix[3] = translate.0;
            matrix[7] = translate.1;
            matrix[11] = translate.2;
            set_rotation_from_vector(&mut matrix, interp_3d(frame, &rotate));
            roll_matrix(&mut matrix, interp_1d(frame, &roll));
            scale_matrix(&mut matrix, interp_3d(frame, &scale));
            Ok(mul_matrix(local, matrix))
        }
        3 => {
            let translate = read_const_3d(loader)?;
            let rotate = read_frames_3d(loader)?;
            let roll = loader.read_i16()? as i32;
            let mut matrix = identity_matrix();
            matrix[3] = translate.0;
            matrix[7] = translate.1;
            matrix[11] = translate.2;
            set_rotation_from_vector(&mut matrix, interp_3d(frame, &rotate));
            roll_matrix(&mut matrix, roll);
            Ok(mul_matrix(local, matrix))
        }
        4 => {
            let rotate = read_frames_3d(loader)?;
            let roll = read_frames_1d(loader)?;
            let mut matrix = identity_matrix();
            matrix[3] = 0;
            matrix[7] = 0;
            matrix[11] = 0;
            set_rotation_from_vector(&mut matrix, interp_3d(frame, &rotate));
            roll_matrix(&mut matrix, interp_1d(frame, &roll));
            Ok(mul_matrix(local, matrix))
        }
        5 => {
            let rotate = read_frames_3d(loader)?;
            let mut matrix = identity_matrix();
            matrix[3] = 0;
            matrix[7] = 0;
            matrix[11] = 0;
            set_rotation_from_vector(&mut matrix, interp_3d(frame, &rotate));
            Ok(mul_matrix(local, matrix))
        }
        6 => {
            let translate = read_frames_3d(loader)?;
            let rotate = read_frames_3d(loader)?;
            let roll = read_frames_1d(loader)?;
            let mut matrix = identity_matrix();
            let translate = interp_3d(frame, &translate);
            matrix[3] = translate.0;
            matrix[7] = translate.1;
            matrix[11] = translate.2;
            set_rotation_from_vector(&mut matrix, interp_3d(frame, &rotate));
            roll_matrix(&mut matrix, interp_1d(frame, &roll));
            Ok(mul_matrix(local, matrix))
        }
        _ => Err("invalid MTRA bone animation"),
    }
}

fn read_matrix(loader: &mut ByteLoader<'_>) -> ParseResult<[i32; 12]> {
    let mut matrix = [0; 12];
    for value in &mut matrix {
        *value = loader.read_i16()? as i32;
    }
    Ok(matrix)
}

fn read_frames_3d(loader: &mut ByteLoader<'_>) -> ParseResult<Vec<i16>> {
    let count = loader.read_u16()? as usize;
    let mut frames = Vec::with_capacity(count * 4);
    for _ in 0..count * 4 {
        frames.push(loader.read_i16()?);
    }
    Ok(frames)
}

fn read_const_3d(loader: &mut ByteLoader<'_>) -> ParseResult<(i32, i32, i32)> {
    Ok((loader.read_i16()? as i32, loader.read_i16()? as i32, loader.read_i16()? as i32))
}

fn read_frames_1d(loader: &mut ByteLoader<'_>) -> ParseResult<Vec<i16>> {
    let count = loader.read_u16()? as usize;
    let mut frames = Vec::with_capacity(count * 2);
    for _ in 0..count * 2 {
        frames.push(loader.read_i16()?);
    }
    Ok(frames)
}

fn interp_3d(keyframe: i32, frames: &[i16]) -> (i32, i32, i32) {
    let keys_count = frames.len() / 4;
    if keys_count == 0 {
        return (0, 0, 0);
    }

    let keyframe_int = keyframe >> 12;
    let max = keys_count - 1;
    let last = max * 4;
    if keyframe_int >= frames[last] as u16 as i32 {
        return (frames[last + 1] as i32, frames[last + 2] as i32, frames[last + 3] as i32);
    }

    for i in (0..max).rev() {
        let offset = i * 4;
        let prev_key = frames[offset] as u16 as i32;
        if prev_key > keyframe_int {
            continue;
        }
        if prev_key == keyframe_int {
            return (frames[offset + 1] as i32, frames[offset + 2] as i32, frames[offset + 3] as i32);
        }

        let next_key = frames[offset + 4] as u16 as i32;
        if next_key == prev_key {
            return (frames[offset + 1] as i32, frames[offset + 2] as i32, frames[offset + 3] as i32);
        }
        let delta = (keyframe - (prev_key << 12)) / (next_key - prev_key);
        return (
            interp_fixed12(frames[offset + 1] as i32, frames[offset + 5] as i32, delta),
            interp_fixed12(frames[offset + 2] as i32, frames[offset + 6] as i32, delta),
            interp_fixed12(frames[offset + 3] as i32, frames[offset + 7] as i32, delta),
        );
    }

    (0, 0, 0)
}

fn interp_1d(keyframe: i32, frames: &[i16]) -> i32 {
    let keys_count = frames.len() / 2;
    if keys_count == 0 {
        return 0;
    }

    let keyframe_int = keyframe >> 12;
    let max = keys_count - 1;
    let last = max * 2;
    if keyframe_int >= frames[last] as u16 as i32 {
        return frames[last + 1] as i32;
    }

    for i in (0..max).rev() {
        let offset = i * 2;
        let prev_key = frames[offset] as u16 as i32;
        if prev_key > keyframe_int {
            continue;
        }
        if prev_key == keyframe_int {
            return frames[offset + 1] as i32;
        }

        let next_key = frames[offset + 2] as u16 as i32;
        if next_key == prev_key {
            return frames[offset + 1] as i32;
        }
        let delta = (keyframe - (prev_key << 12)) / (next_key - prev_key);
        return interp_fixed12(frames[offset + 1] as i32, frames[offset + 3] as i32, delta);
    }

    0
}

fn interp_fixed12(prev: i32, next: i32, delta: i32) -> i32 {
    prev + (((next - prev) * delta) >> 12)
}
