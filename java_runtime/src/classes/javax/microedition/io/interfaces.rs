#[allow(unused_imports)]
use super::*;
use crate::RuntimeClassProto;
#[allow(unused_imports)]
use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};
use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;

impl Connection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/Connection",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new_abstract("close", "()V", Default::default())],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl InputConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/InputConnection",
            parent_class: None,
            interfaces: vec!["javax/microedition/io/Connection"],
            methods: vec![
                JavaMethodProto::new_abstract("openInputStream", "()Ljava/io/InputStream;", Default::default()),
                JavaMethodProto::new_abstract("openDataInputStream", "()Ljava/io/DataInputStream;", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl OutputConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/OutputConnection",
            parent_class: None,
            interfaces: vec!["javax/microedition/io/Connection"],
            methods: vec![
                JavaMethodProto::new_abstract("openOutputStream", "()Ljava/io/OutputStream;", Default::default()),
                JavaMethodProto::new_abstract("openDataOutputStream", "()Ljava/io/DataOutputStream;", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl StreamConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/StreamConnection",
            parent_class: None,
            interfaces: vec!["javax/microedition/io/InputConnection", "javax/microedition/io/OutputConnection"],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl ContentConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/ContentConnection",
            parent_class: None,
            interfaces: vec!["javax/microedition/io/StreamConnection"],
            methods: vec![
                JavaMethodProto::new_abstract("getType", "()Ljava/lang/String;", Default::default()),
                JavaMethodProto::new_abstract("getEncoding", "()Ljava/lang/String;", Default::default()),
                JavaMethodProto::new_abstract("getLength", "()J", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl StreamConnectionNotifier {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/StreamConnectionNotifier",
            parent_class: None,
            interfaces: vec!["javax/microedition/io/Connection"],
            methods: vec![JavaMethodProto::new_abstract(
                "acceptAndOpen",
                "()Ljavax/microedition/io/StreamConnection;",
                Default::default(),
            )],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl DatagramConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/DatagramConnection",
            parent_class: None,
            interfaces: vec!["javax/microedition/io/Connection"],
            methods: vec![
                JavaMethodProto::new_abstract("getMaximumLength", "()I", Default::default()),
                JavaMethodProto::new_abstract("getNominalLength", "()I", Default::default()),
                JavaMethodProto::new_abstract("send", "(Ljavax/microedition/io/Datagram;)V", Default::default()),
                JavaMethodProto::new_abstract("receive", "(Ljavax/microedition/io/Datagram;)V", Default::default()),
                JavaMethodProto::new_abstract("newDatagram", "(I)Ljavax/microedition/io/Datagram;", Default::default()),
                JavaMethodProto::new_abstract("newDatagram", "(ILjava/lang/String;)Ljavax/microedition/io/Datagram;", Default::default()),
                JavaMethodProto::new_abstract("newDatagram", "([BI)Ljavax/microedition/io/Datagram;", Default::default()),
                JavaMethodProto::new_abstract(
                    "newDatagram",
                    "([BILjava/lang/String;)Ljavax/microedition/io/Datagram;",
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
