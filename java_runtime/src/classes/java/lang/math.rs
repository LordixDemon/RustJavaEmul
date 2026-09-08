use alloc::vec;
use core::sync::atomic::{AtomicU64, Ordering};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

static RANDOM_SEED: AtomicU64 = AtomicU64::new(0x5DEECE66D);

// class java.lang.Math
pub struct Math;

impl Math {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Math",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(I)I", Self::abs, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(J)J", Self::abs_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(F)F", Self::abs_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(D)D", Self::abs_double, MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(II)I", Self::max, MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(JJ)J", Self::max_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(FF)F", Self::max_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(DD)D", Self::max_double, MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(II)I", Self::min, MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(JJ)J", Self::min_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(FF)F", Self::min_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(DD)D", Self::min_double, MethodAccessFlags::STATIC),
                JavaMethodProto::new("sqrt", "(D)D", Self::sqrt, MethodAccessFlags::STATIC),
                JavaMethodProto::new("sin", "(D)D", Self::sin, MethodAccessFlags::STATIC),
                JavaMethodProto::new("cos", "(D)D", Self::cos, MethodAccessFlags::STATIC),
                JavaMethodProto::new("tan", "(D)D", Self::tan, MethodAccessFlags::STATIC),
                JavaMethodProto::new("asin", "(D)D", Self::asin, MethodAccessFlags::STATIC),
                JavaMethodProto::new("acos", "(D)D", Self::acos, MethodAccessFlags::STATIC),
                JavaMethodProto::new("atan", "(D)D", Self::atan, MethodAccessFlags::STATIC),
                JavaMethodProto::new("atan2", "(DD)D", Self::atan2, MethodAccessFlags::STATIC),
                JavaMethodProto::new("floor", "(D)D", Self::floor, MethodAccessFlags::STATIC),
                JavaMethodProto::new("ceil", "(D)D", Self::ceil, MethodAccessFlags::STATIC),
                JavaMethodProto::new("round", "(F)I", Self::round_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("round", "(D)J", Self::round_double, MethodAccessFlags::STATIC),
                JavaMethodProto::new("pow", "(DD)D", Self::pow, MethodAccessFlags::STATIC),
                JavaMethodProto::new("exp", "(D)D", Self::exp, MethodAccessFlags::STATIC),
                JavaMethodProto::new("log", "(D)D", Self::log, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toRadians", "(D)D", Self::to_radians, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toDegrees", "(D)D", Self::to_degrees, MethodAccessFlags::STATIC),
                JavaMethodProto::new("copySign", "(DD)D", Self::copy_sign_double, MethodAccessFlags::STATIC),
                JavaMethodProto::new("copySign", "(FF)F", Self::copy_sign_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("log10", "(D)D", Self::log10, MethodAccessFlags::STATIC),
                JavaMethodProto::new("random", "()D", Self::random, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("E", "D", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PI", "D", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Math", "E", "D", core::f64::consts::E).await?;
        jvm.put_static_field("java/lang/Math", "PI", "D", core::f64::consts::PI).await
    }

    async fn abs(_: &Jvm, _: &mut RuntimeContext, x: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::abs({x:?})");

        Ok(x.abs())
    }

    async fn abs_long(_: &Jvm, _: &mut RuntimeContext, x: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::abs({x:?})");

        Ok(x.abs())
    }

    async fn abs_float(_: &Jvm, _: &mut RuntimeContext, x: f32) -> Result<f32> {
        tracing::debug!("java.lang.Math::abs({x:?})");

        Ok(x.abs())
    }

    async fn abs_double(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::abs({x:?})");

        Ok(x.abs())
    }

    async fn max(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");

        Ok(x.max(y))
    }

    async fn max_long(_: &Jvm, _: &mut RuntimeContext, x: i64, y: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");

        Ok(x.max(y))
    }

    async fn max_float(_: &Jvm, _: &mut RuntimeContext, x: f32, y: f32) -> Result<f32> {
        Ok(x.max(y))
    }

    async fn max_double(_: &Jvm, _: &mut RuntimeContext, x: f64, y: f64) -> Result<f64> {
        Ok(x.max(y))
    }

    async fn min(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");

        Ok(x.min(y))
    }

    async fn min_long(_: &Jvm, _: &mut RuntimeContext, x: i64, y: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");

        Ok(x.min(y))
    }

    async fn min_float(_: &Jvm, _: &mut RuntimeContext, x: f32, y: f32) -> Result<f32> {
        Ok(x.min(y))
    }

    async fn min_double(_: &Jvm, _: &mut RuntimeContext, x: f64, y: f64) -> Result<f64> {
        Ok(x.min(y))
    }

    async fn sqrt(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::sqrt({x:?})");

        Ok(x.sqrt())
    }

    async fn sin(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::sin({x:?})");

        Ok(x.sin())
    }

    async fn cos(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::cos({x:?})");

        Ok(x.cos())
    }

    async fn tan(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::tan({x:?})");

        Ok(x.tan())
    }

    async fn asin(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::asin({x:?})");

        Ok(x.asin())
    }

    async fn acos(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::acos({x:?})");

        Ok(x.acos())
    }

    async fn atan(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::atan({x:?})");

        Ok(x.atan())
    }

    async fn atan2(_: &Jvm, _: &mut RuntimeContext, y: f64, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::atan2({y:?}, {x:?})");

        Ok(y.atan2(x))
    }

    async fn floor(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::floor({x:?})");

        Ok(x.floor())
    }

    async fn ceil(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::ceil({x:?})");

        Ok(x.ceil())
    }

    async fn pow(_: &Jvm, _: &mut RuntimeContext, x: f64, y: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::pow({x:?}, {y:?})");

        Ok(x.powf(y))
    }

    async fn exp(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::exp({x:?})");

        Ok(x.exp())
    }

    async fn log(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::trace!("java.lang.Math::log({x:?})");

        Ok(x.ln())
    }

    async fn round_float(_: &Jvm, _: &mut RuntimeContext, x: f32) -> Result<i32> {
        Ok(x.round() as i32)
    }

    async fn round_double(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<i64> {
        Ok(x.round() as i64)
    }

    async fn to_radians(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        Ok(x.to_radians())
    }

    async fn to_degrees(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        Ok(x.to_degrees())
    }

    async fn copy_sign_double(_: &Jvm, _: &mut RuntimeContext, magnitude: f64, sign: f64) -> Result<f64> {
        Ok(magnitude.copysign(sign))
    }

    async fn copy_sign_float(_: &Jvm, _: &mut RuntimeContext, magnitude: f32, sign: f32) -> Result<f32> {
        Ok(magnitude.copysign(sign))
    }

    async fn log10(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        Ok(x.log10())
    }

    async fn random(_: &Jvm, _: &mut RuntimeContext) -> Result<f64> {
        let high = next_random_bits(26) as i64;
        let low = next_random_bits(27) as i64;
        Ok(((high << 27) + low) as f64 / (1u64 << 53) as f64)
    }
}

fn next_random_bits(bits: u32) -> i32 {
    loop {
        let seed = RANDOM_SEED.load(Ordering::Relaxed);
        let next = seed.wrapping_mul(0x5DEECE66D).wrapping_add(0xB) & ((1u64 << 48) - 1);
        if RANDOM_SEED.compare_exchange(seed, next, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            return (next >> (48 - bits)) as i32;
        }
    }
}
