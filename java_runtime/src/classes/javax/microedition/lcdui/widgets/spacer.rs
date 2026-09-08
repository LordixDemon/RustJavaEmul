#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

impl Spacer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Spacer",
            parent_class: Some("javax/microedition/lcdui/Item"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(II)V", Self::init, Default::default()),
                JavaMethodProto::new("setMinimumSize", "(II)V", Self::set_minimum_size, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("minWidth", "I", Default::default()),
                JavaFieldProto::new("minHeight", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, min_width: i32, min_height: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/lcdui/Item", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "minWidth", "I", min_width).await?;
        jvm.put_field(&mut this, "minHeight", "I", min_height).await
    }

    pub(super) async fn set_minimum_size(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        min_width: i32,
        min_height: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "minWidth", "I", min_width).await?;
        jvm.put_field(&mut this, "minHeight", "I", min_height).await
    }
}
