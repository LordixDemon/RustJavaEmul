#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};
use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl PushRegistry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/PushRegistry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "registerConnection",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                    Self::register,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "unregisterConnection",
                    "(Ljava/lang/String;)Z",
                    Self::unregister,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("listConnections", "(Z)[Ljava/lang/String;", Self::list, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "getMIDlet",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_midlet,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getFilter",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_filter,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("registerAlarm", "(Ljava/lang/String;J)J", Self::register_alarm, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn register(
        _: &Jvm,
        _: &mut RuntimeContext,
        _c: ClassInstanceRef<String>,
        _m: ClassInstanceRef<String>,
        _f: ClassInstanceRef<String>,
    ) -> Result<()> {
        Ok(())
    }
    pub(super) async fn unregister(_: &Jvm, _: &mut RuntimeContext, _c: ClassInstanceRef<String>) -> Result<bool> {
        Ok(true)
    }
    pub(super) async fn list(jvm: &Jvm, _: &mut RuntimeContext, _available: bool) -> Result<ClassInstanceRef<Array<ClassInstanceRef<String>>>> {
        Ok(jvm.instantiate_array("Ljava/lang/String;", 0).await?.into())
    }
    pub(super) async fn get_midlet(_: &Jvm, _: &mut RuntimeContext, _c: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        Ok(ClassInstanceRef::new(None))
    }
    pub(super) async fn get_filter(_: &Jvm, _: &mut RuntimeContext, _c: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        Ok(ClassInstanceRef::new(None))
    }
    pub(super) async fn register_alarm(_: &Jvm, _: &mut RuntimeContext, _m: ClassInstanceRef<String>, _t: i64) -> Result<i64> {
        Ok(0)
    }
}

impl SecurityInfo {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/SecurityInfo",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("getCipherSuite", "()Ljava/lang/String;", Default::default()),
                JavaMethodProto::new_abstract("getProtocolName", "()Ljava/lang/String;", Default::default()),
                JavaMethodProto::new_abstract("getProtocolVersion", "()Ljava/lang/String;", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
