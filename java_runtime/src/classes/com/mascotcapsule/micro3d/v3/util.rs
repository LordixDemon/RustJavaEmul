use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;

use crate::{RuntimeClassProto, RuntimeContext};
use jvm::{Jvm, Result};

use super::{
    constants::UTIL_3D_CLASS,
    math::{isqrt, sin_mc},
};

// class com.mascotcapsule.micro3d.v3.Util3D
pub struct Util3D;

impl Util3D {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: UTIL_3D_CLASS,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("sqrt", "(I)I", Self::sqrt, MethodAccessFlags::STATIC),
                JavaMethodProto::new("sin", "(I)I", Self::sin, MethodAccessFlags::STATIC),
                JavaMethodProto::new("cos", "(I)I", Self::cos, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn sqrt(_: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<i32> {
        if value == 0 {
            return Ok(0);
        }

        let high_unsigned_limit = 0xfffd_0002u32 as i32;
        if value < 0 && value > high_unsigned_limit {
            return Ok(0xffff);
        }

        let value = value as u32 as u64;
        let root = isqrt(value);
        let next = root + 1;
        let rounded = if next.saturating_mul(next).saturating_sub(value) < value.saturating_sub(root.saturating_mul(root)) {
            next
        } else {
            root
        };
        Ok(rounded.min(i32::MAX as u64) as i32)
    }

    async fn sin(_: &Jvm, _: &mut RuntimeContext, angle: i32) -> Result<i32> {
        Ok(sin_mc(angle))
    }

    async fn cos(_: &Jvm, _: &mut RuntimeContext, angle: i32) -> Result<i32> {
        Ok(sin_mc(angle + 1024))
    }
}
