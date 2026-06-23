use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};

use crate::RuntimeClassProto;

// interface javax.microedition.media.PlayerListener
pub struct PlayerListener;

impl PlayerListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/PlayerListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new_abstract(
                "playerUpdate",
                "(Ljavax/microedition/media/Player;Ljava/lang/String;Ljava/lang/Object;)V",
                MethodAccessFlags::ABSTRACT,
            )],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
