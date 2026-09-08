#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

impl FullCanvas {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/nokia/mid/ui/FullCanvas",
            parent_class: Some("javax/microedition/lcdui/Canvas"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("KEY_UP_ARROW", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_DOWN_ARROW", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_LEFT_ARROW", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_RIGHT_ARROW", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_SOFTKEY1", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_SOFTKEY2", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_SOFTKEY3", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "com/nokia/mid/ui/FullCanvas";
        jvm.put_static_field(class, "KEY_UP_ARROW", "I", -1).await?;
        jvm.put_static_field(class, "KEY_DOWN_ARROW", "I", -2).await?;
        jvm.put_static_field(class, "KEY_LEFT_ARROW", "I", -3).await?;
        jvm.put_static_field(class, "KEY_RIGHT_ARROW", "I", -4).await?;
        jvm.put_static_field(class, "KEY_SOFTKEY1", "I", -6).await?;
        jvm.put_static_field(class, "KEY_SOFTKEY2", "I", -7).await?;
        jvm.put_static_field(class, "KEY_SOFTKEY3", "I", -5).await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/lcdui/Canvas", "<init>", "()V", ()).await?;
        jvm.invoke_virtual(&this, "setFullScreenMode", "(Z)V", (true,)).await
    }
}
