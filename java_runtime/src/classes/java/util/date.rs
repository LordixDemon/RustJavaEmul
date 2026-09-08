use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.util.Date
pub struct Date;

impl Date {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Date",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable", "java/lang/Cloneable", "java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(J)V", Self::init_with_time, Default::default()),
                JavaMethodProto::new("getTime", "()J", Self::get_time, Default::default()),
                JavaMethodProto::new("setTime", "(J)V", Self::set_time, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("value", "J", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Date::<init>({:?})", &this);

        let now: u64 = context.now();

        let _: () = jvm.invoke_special(&this, "java/util/Date", "<init>", "(J)V", (now as i64,)).await?;

        Ok(())
    }

    async fn init_with_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time: i64) -> Result<()> {
        tracing::debug!("java.util.Date::<init>({:?}, {:?})", &this, time);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "value", "J", time).await?;

        Ok(())
    }

    async fn get_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.Date::getTime({:?})", &this);

        let time = jvm.get_field(&this, "value", "J").await?;

        Ok(time)
    }

    async fn set_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time: i64) -> Result<()> {
        tracing::debug!("java.util.Date::setTime({:?}, {:?})", &this, time);

        jvm.put_field(&mut this, "value", "J", time).await?;

        Ok(())
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let time: i64 = jvm.get_field(&this, "value", "J").await?;
        JavaLangString::from_rust_string(jvm, &format!("{time}")).await.map(Into::into)
    }
}
