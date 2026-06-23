use alloc::vec;

use java_class_proto::JavaMethodProto;

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::javax::microedition::lcdui::{Command, Item},
};
use jvm::{ClassInstanceRef, Jvm, Result};

// interface javax.microedition.lcdui.ItemCommandListener
pub struct ItemCommandListener;

impl ItemCommandListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/ItemCommandListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "commandAction",
                "(Ljavax/microedition/lcdui/Command;Ljavax/microedition/lcdui/Item;)V",
                Self::command_action,
                Default::default(),
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn command_action(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        command: ClassInstanceRef<Command>,
        item: ClassInstanceRef<Item>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.ItemCommandListener::commandAction({this:?}, {command:?}, {item:?})");

        Ok(())
    }
}
