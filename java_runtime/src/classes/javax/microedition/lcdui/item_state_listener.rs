use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::javax::microedition::lcdui::Item};

// interface javax.microedition.lcdui.ItemStateListener
pub struct ItemStateListener;

impl ItemStateListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/ItemStateListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "itemStateChanged",
                "(Ljavax/microedition/lcdui/Item;)V",
                Self::item_state_changed,
                Default::default(),
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn item_state_changed(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, item: ClassInstanceRef<Item>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.ItemStateListener::itemStateChanged({this:?}, {item:?})");

        Ok(())
    }
}
