use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.util.Dictionary
pub struct Dictionary;

impl Dictionary {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Dictionary",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("get", "(Ljava/lang/Object;)Ljava/lang/Object;", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract(
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new_abstract("remove", "(Ljava/lang/Object;)Ljava/lang/Object;", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("size", "()I", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("isEmpty", "()Z", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("keys", "()Ljava/util/Enumeration;", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("elements", "()Ljava/util/Enumeration;", MethodAccessFlags::ABSTRACT),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Dictionary::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }
}
