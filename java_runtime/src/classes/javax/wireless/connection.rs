#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

impl MessageConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/wireless/messaging/MessageConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/Connection"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init_url, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new(
                    "newMessage",
                    "(Ljava/lang/String;)Ljavax/wireless/messaging/Message;",
                    Self::new_message,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "newMessage",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljavax/wireless/messaging/Message;",
                    Self::new_message_addr,
                    Default::default(),
                ),
                JavaMethodProto::new("receive", "()Ljavax/wireless/messaging/Message;", Self::receive, Default::default()),
                JavaMethodProto::new("send", "(Ljavax/wireless/messaging/Message;)V", Self::send, Default::default()),
                JavaMethodProto::new(
                    "setMessageListener",
                    "(Ljavax/wireless/messaging/MessageListener;)V",
                    Self::set_message_listener,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "numberOfSegments",
                    "(Ljavax/wireless/messaging/Message;)I",
                    Self::number_of_segments,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("url", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("listener", "Ljavax/wireless/messaging/MessageListener;", Default::default()),
                JavaFieldProto::new("TEXT_MESSAGE", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BINARY_MESSAGE", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "MULTIPART_MESSAGE",
                    "Ljava/lang/String;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/wireless/messaging/MessageConnection";
        let text = JavaLangString::from_rust_string(jvm, "text").await?;
        let binary = JavaLangString::from_rust_string(jvm, "binary").await?;
        let multipart = JavaLangString::from_rust_string(jvm, "multipart").await?;
        jvm.put_static_field(class, "TEXT_MESSAGE", "Ljava/lang/String;", text).await?;
        jvm.put_static_field(class, "BINARY_MESSAGE", "Ljava/lang/String;", binary).await?;
        jvm.put_static_field(class, "MULTIPART_MESSAGE", "Ljava/lang/String;", multipart).await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn init_url(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<String>,
        _mode: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "url", "Ljava/lang/String;", url).await
    }

    pub(super) async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    pub(super) async fn new_message(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        kind: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Message>> {
        let binary = if kind.is_null() {
            false
        } else {
            JavaLangString::to_rust_string(jvm, &kind).await?.eq_ignore_ascii_case("binary")
        };
        let class = if binary {
            "javax/wireless/messaging/BinaryMessage"
        } else {
            "javax/wireless/messaging/TextMessage"
        };
        Ok(jvm.new_class(class, "()V", ()).await?.into())
    }

    pub(super) async fn new_message_addr(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        kind: ClassInstanceRef<String>,
        address: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Message>> {
        let message = Self::new_message(jvm, context, this, kind).await?;
        let _: () = jvm.invoke_virtual(&message, "setAddress", "(Ljava/lang/String;)V", (address,)).await?;
        Ok(message)
    }

    pub(super) async fn receive(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Message>> {
        Ok(None.into())
    }

    pub(super) async fn send(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _message: ClassInstanceRef<Message>) -> Result<()> {
        Ok(())
    }

    pub(super) async fn set_message_listener(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<MessageListener>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "listener", "Ljavax/wireless/messaging/MessageListener;", listener)
            .await
    }

    pub(super) async fn number_of_segments(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _message: ClassInstanceRef<Message>,
    ) -> Result<i32> {
        Ok(1)
    }
}
