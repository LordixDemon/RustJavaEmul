#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::ClassAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl Message {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/wireless/messaging/Message",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getAddress", "()Ljava/lang/String;", Self::get_address, Default::default()),
                JavaMethodProto::new("setAddress", "(Ljava/lang/String;)V", Self::set_address, Default::default()),
                JavaMethodProto::new("getTimestamp", "()Ljava/util/Date;", Self::get_timestamp, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("address", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("timestamp", "Ljava/util/Date;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn get_address(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "address", "Ljava/lang/String;").await
    }

    pub(super) async fn set_address(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        address: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "address", "Ljava/lang/String;", address).await
    }

    pub(super) async fn get_timestamp(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::util::Date>> {
        jvm.get_field(&this, "timestamp", "Ljava/util/Date;").await
    }
}

impl TextMessage {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/wireless/messaging/TextMessage",
            parent_class: Some("javax/wireless/messaging/Message"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getPayloadText", "()Ljava/lang/String;", Self::get_payload_text, Default::default()),
                JavaMethodProto::new("setPayloadText", "(Ljava/lang/String;)V", Self::set_payload_text, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("payload", "Ljava/lang/String;", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/wireless/messaging/Message", "<init>", "()V", ()).await
    }

    pub(super) async fn get_payload_text(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "payload", "Ljava/lang/String;").await
    }

    pub(super) async fn set_payload_text(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        text: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "payload", "Ljava/lang/String;", text).await
    }
}

impl BinaryMessage {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/wireless/messaging/BinaryMessage",
            parent_class: Some("javax/wireless/messaging/Message"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getPayloadData", "()[B", Self::get_payload_data, Default::default()),
                JavaMethodProto::new("setPayloadData", "([B)V", Self::set_payload_data, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("payload", "[B", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/wireless/messaging/Message", "<init>", "()V", ()).await
    }

    pub(super) async fn get_payload_data(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        jvm.get_field(&this, "payload", "[B").await
    }

    pub(super) async fn set_payload_data(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "payload", "[B", data).await
    }
}

impl MessageListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/wireless/messaging/MessageListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new_abstract(
                "notifyIncomingMessage",
                "(Ljavax/wireless/messaging/MessageConnection;)V",
                Default::default(),
            )],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
