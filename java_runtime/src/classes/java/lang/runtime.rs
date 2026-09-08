use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::System};

// class java.lang.Runtime
pub struct Runtime;

impl Runtime {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Runtime",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getRuntime", "()Ljava/lang/Runtime;", Self::get_runtime, MethodAccessFlags::STATIC),
                JavaMethodProto::new("totalMemory", "()J", Self::total_memory, Default::default()),
                JavaMethodProto::new("freeMemory", "()J", Self::free_memory, Default::default()),
                JavaMethodProto::new("gc", "()V", Self::gc, Default::default()),
                JavaMethodProto::new("exit", "(I)V", Self::exit, Default::default()),
                JavaMethodProto::new("halt", "(I)V", Self::halt, Default::default()),
                JavaMethodProto::new("availableProcessors", "()I", Self::available_processors, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<()> {
        tracing::warn!("stub java.lang.Runtime::<init>({:?})", &this);

        Ok(())
    }

    async fn get_runtime(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Runtime::getRuntime");

        let instance = jvm.new_class("java/lang/Runtime", "()V", []).await?;

        Ok(instance.into())
    }

    async fn total_memory(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<i64> {
        tracing::warn!("stub java.lang.Runtime::totalMemory({:?})", &this);

        Ok(0x100000) // TODO: hardcoded
    }

    async fn free_memory(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<i64> {
        tracing::warn!("stub java.lang.Runtime::freeMemory({:?})", &this);

        Ok(0x100000) // TODO: hardcoded
    }

    async fn gc(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<()> {
        tracing::debug!("java.lang.Runtime::gc({:?})", &this);

        System::explicit_gc(jvm, context.now())?;

        Ok(())
    }

    async fn exit(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>, status: i32) -> Result<()> {
        tracing::debug!("java.lang.Runtime::exit({this:?}, {status})");
        jvm.invoke_static("java/lang/System", "exit", "(I)V", (status,)).await
    }

    async fn halt(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Runtime>, status: i32) -> Result<()> {
        tracing::debug!("java.lang.Runtime::halt({this:?}, {status})");
        Self::exit(jvm, context, this, status).await
    }

    async fn available_processors(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Runtime>) -> Result<i32> {
        Ok(1)
    }
}
