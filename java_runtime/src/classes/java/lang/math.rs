use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.lang.Math
pub struct Math;

impl Math {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Math",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("abs", "(I)I", Self::abs, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(J)J", Self::abs_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(F)F", Self::abs_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(D)D", Self::abs_double, MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(II)I", Self::max, MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(JJ)J", Self::max_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(II)I", Self::min, MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(JJ)J", Self::min_long, MethodAccessFlags::STATIC),
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
                JavaMethodProto::new("pow", "(DD)D", Self::pow, MethodAccessFlags::STATIC),
                JavaMethodProto::new("exp", "(D)D", Self::exp, MethodAccessFlags::STATIC),
                JavaMethodProto::new("log", "(D)D", Self::log, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
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

    async fn min(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");

        Ok(x.min(y))
    }

    async fn min_long(_: &Jvm, _: &mut RuntimeContext, x: i64, y: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");

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
}
