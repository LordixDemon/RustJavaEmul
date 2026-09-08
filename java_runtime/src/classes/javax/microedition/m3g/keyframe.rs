#[allow(unused_imports)]
use super::common::*;
#[allow(unused_imports)]
use super::math::*;
#[allow(unused_imports)]
use super::prelude::*;
#[allow(unused_imports)]
use super::raw_arrays::*;
#[allow(unused_imports)]
use super::render::*;
#[allow(unused_imports)]
use super::types::*;

impl KeyframeSequence {
    pub(super) const LINEAR: i32 = 176;
    pub const SLERP: i32 = 177;
    pub const SPLINE: i32 = 178;
    pub const SQUAD: i32 = 179;
    pub const STEP: i32 = 180;
    pub(super) const CONSTANT: i32 = 192;
    pub const LOOP: i32 = 193;

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

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
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

    pub(super) async fn init_with_counts(
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

    pub(super) async fn get_component_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "componentCount", "I").await
    }

    pub(super) async fn get_duration(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "duration", "I").await
    }

    pub(super) async fn get_interpolation_type(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "interpolationType", "I").await
    }

    pub(super) async fn get_keyframe(
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

    pub(super) async fn get_keyframe_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "keyframeCount", "I").await
    }

    pub(super) async fn get_repeat_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "repeatMode", "I").await
    }

    pub(super) async fn get_valid_range_first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "validRangeFirst", "I").await
    }

    pub(super) async fn get_valid_range_last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "validRangeLast", "I").await
    }

    pub(super) async fn set_duration(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, duration: i32) -> Result<()> {
        if duration < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "KeyframeSequence duration").await);
        }
        jvm.put_field(&mut this, "duration", "I", duration).await
    }

    pub(super) async fn set_keyframe(
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

    pub(super) async fn set_repeat_mode(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, repeat_mode: i32) -> Result<()> {
        if repeat_mode != Self::CONSTANT && repeat_mode != Self::LOOP {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "KeyframeSequence repeat mode").await);
        }
        jvm.put_field(&mut this, "repeatMode", "I", repeat_mode).await
    }

    pub(super) async fn set_valid_range(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, first: i32, last: i32) -> Result<()> {
        let keyframe_count: i32 = jvm.get_field(&this, "keyframeCount", "I").await?;
        if first < 0 || last < first || last >= keyframe_count {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "KeyframeSequence valid range").await);
        }
        jvm.put_field(&mut this, "validRangeFirst", "I", first).await?;
        jvm.put_field(&mut this, "validRangeLast", "I", last).await
    }

    pub(super) async fn put_data(
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

    pub(super) async fn put_keyframes(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, times: Vec<i32>, values: Vec<f32>) -> Result<()> {
        let mut time_array = jvm.instantiate_array("I", times.len()).await?;
        store_raw_i32_array(jvm, &mut time_array, &times).await?;
        let mut value_array = jvm.instantiate_array("F", values.len()).await?;
        store_raw_f32_array(jvm, &mut value_array, &values).await?;
        jvm.put_field(this, "keyframeTimes", "[I", time_array).await?;
        jvm.put_field(this, "keyframeValues", "[F", value_array).await
    }

    pub(super) async fn times(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<i32>> {
        let keyframe_count: i32 = jvm.get_field(this, "keyframeCount", "I").await?;
        let times: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "keyframeTimes", "[I").await?;
        raw_i32_array(jvm, &times, keyframe_count.max(0) as usize).await
    }

    pub(super) async fn values(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<f32>> {
        let keyframe_count: i32 = jvm.get_field(this, "keyframeCount", "I").await?;
        let component_count: i32 = jvm.get_field(this, "componentCount", "I").await?;
        let values: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "keyframeValues", "[F").await?;
        raw_f32_array(jvm, &values, (keyframe_count.max(0) * component_count.max(0)) as usize).await
    }

    pub(super) async fn sample(jvm: &Jvm, this: &ClassInstanceRef<Self>, sequence_time: f32) -> Result<Option<Vec<f32>>> {
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
        if interpolation == Self::SLERP && component_count == 4 {
            let a = [
                values[lower_start],
                values[lower_start + 1],
                values[lower_start + 2],
                values[lower_start + 3],
            ];
            let b = [
                values[upper_start],
                values[upper_start + 1],
                values[upper_start + 2],
                values[upper_start + 3],
            ];
            return Ok(Some(slerp_quat(a, b, t).to_vec()));
        }
        if interpolation == Self::SQUAD && component_count == 4 {
            let q0 = quat_at(&values, lower.saturating_sub(1), component_count);
            let q1 = quat_at(&values, lower, component_count);
            let q2 = quat_at(&values, upper, component_count);
            let q3 = quat_at(&values, (upper + 1).min(times.len() - 1), component_count);
            return Ok(Some(squad_quat(q0, q1, q2, q3, t).to_vec()));
        }
        let mut result = Vec::with_capacity(component_count);
        if interpolation == Self::SPLINE && lower > 0 && upper + 1 < times.len() {
            let prev_start = (lower - 1) * component_count;
            let next_start = (upper + 1) * component_count;
            let t2 = t * t;
            let t3 = t2 * t;
            for component in 0..component_count {
                let p0 = values[prev_start + component];
                let p1 = values[lower_start + component];
                let p2 = values[upper_start + component];
                let p3 = values[next_start + component];
                result.push(0.5 * (2.0 * p1 + (-p0 + p2) * t + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2 + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3));
            }
            return Ok(Some(result));
        }
        for component in 0..component_count {
            let a = values[lower_start + component];
            let b = values[upper_start + component];
            result.push(a + (b - a) * t);
        }
        Ok(Some(result))
    }
}

pub(super) fn quat_at(values: &[f32], index: usize, component_count: usize) -> [f32; 4] {
    let start = index * component_count;
    [values[start], values[start + 1], values[start + 2], values[start + 3]]
}

pub(super) fn squad_quat(q0: [f32; 4], q1: [f32; 4], q2: [f32; 4], q3: [f32; 4], t: f32) -> [f32; 4] {
    let a = slerp_quat(q1, slerp_quat(q0, q2, 0.5), 1.0 / 3.0);
    let b = slerp_quat(q2, slerp_quat(q1, q3, 0.5), 1.0 / 3.0);
    slerp_quat(slerp_quat(q1, q2, t), slerp_quat(a, b, t), 2.0 * t * (1.0 - t))
}

pub(super) fn slerp_quat(a: [f32; 4], mut b: [f32; 4], t: f32) -> [f32; 4] {
    let mut dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
    if dot < 0.0 {
        for value in &mut b {
            *value = -*value;
        }
        dot = -dot;
    }
    if dot > 0.9995 {
        let mut out = [0.0; 4];
        let mut len = 0.0;
        for i in 0..4 {
            out[i] = a[i] + (b[i] - a[i]) * t;
            len += out[i] * out[i];
        }
        let inv = 1.0 / len.sqrt().max(1e-8);
        for value in &mut out {
            *value *= inv;
        }
        return out;
    }
    let theta = dot.clamp(-1.0, 1.0).acos();
    let sin_theta = theta.sin().max(1e-8);
    let w0 = ((1.0 - t) * theta).sin() / sin_theta;
    let w1 = (t * theta).sin() / sin_theta;
    [a[0] * w0 + b[0] * w1, a[1] * w0 + b[1] * w1, a[2] * w0 + b[2] * w1, a[3] * w0 + b[3] * w1]
}
