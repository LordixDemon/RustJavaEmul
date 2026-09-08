use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

pub struct CancellableTask;

impl CancellableTask {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/netbeans/microedition/util/CancellableTask",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("hasFailed", "()Z", Self::has_failed, Default::default()),
                JavaMethodProto::new("isCancelled", "()Z", Self::is_cancelled, Default::default()),
                JavaMethodProto::new("cancel", "()V", Self::cancel, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn has_failed(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }

    async fn is_cancelled(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }

    async fn cancel(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn run(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
}

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    vec![CancellableTask::as_proto]
}
