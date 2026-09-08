use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::ClassAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.util.TimerTask
pub struct TimerTask;

impl TimerTask {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TimerTask",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("run", "()V", Default::default()),
                JavaMethodProto::new("cancel", "()Z", Self::cancel, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("nextExecutionTime", "J", Default::default()),
                JavaFieldProto::new("period", "J", Default::default()),
                JavaFieldProto::new("cancelled", "Z", Default::default()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimerTask::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn cancel(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<bool> {
        let cancelled: bool = jvm.get_field(&this, "cancelled", "Z").await.unwrap_or(false);
        jvm.put_field(&mut this, "cancelled", "Z", true).await?;
        jvm.put_field(&mut this, "nextExecutionTime", "J", -1i64).await?;
        Ok(!cancelled)
    }
}
