use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// abstract class java.util.TimeZone
pub struct TimeZone;

impl TimeZone {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TimeZone",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getTimeZone",
                    "(Ljava/lang/String;)Ljava/util/TimeZone;",
                    Self::get_time_zone,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getDefault", "()Ljava/util/TimeZone;", Self::get_default, MethodAccessFlags::STATIC),
                JavaMethodProto::new("getRawOffset", "()I", Self::get_raw_offset, Default::default()),
                JavaMethodProto::new("useDaylightTime", "()Z", Self::use_daylight_time, Default::default()),
                JavaMethodProto::new("getID", "()Ljava/lang/String;", Self::get_id, Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimeZone::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_time_zone(jvm: &Jvm, _: &mut RuntimeContext, id: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.TimeZone::getTimeZone({id:?})");

        let result = jvm.new_class("java/util/SimpleTimeZone", "(Ljava/lang/String;)V", (id,)).await?;

        Ok(result.into())
    }

    async fn get_default(jvm: &Jvm, context: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let id = JavaLangString::from_rust_string(jvm, "GMT").await?;
        Self::get_time_zone(jvm, context, id.into()).await
    }

    async fn get_raw_offset(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }

    async fn use_daylight_time(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }

    async fn get_id(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, "GMT").await?.into())
    }
}
