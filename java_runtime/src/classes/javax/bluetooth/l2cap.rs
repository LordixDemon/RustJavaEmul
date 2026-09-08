#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::{boxed::Box, format, vec};
use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl L2CAPConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/L2CAPConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/Connection"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init_url, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("ready", "()Z", Self::ready, Default::default()),
                JavaMethodProto::new("receive", "([B)I", Self::receive, Default::default()),
                JavaMethodProto::new("send", "([B)V", Self::send, Default::default()),
                JavaMethodProto::new("getTransmitMTU", "()I", Self::get_transmit_mtu, Default::default()),
                JavaMethodProto::new("getReceiveMTU", "()I", Self::get_receive_mtu, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn init_url(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _url: ClassInstanceRef<String>,
        _mode: i32,
    ) -> Result<()> {
        Self::init(jvm, context, this).await
    }

    pub(super) async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    pub(super) async fn ready(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }

    pub(super) async fn receive(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _buf: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        Ok(-1)
    }

    pub(super) async fn send(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _buf: ClassInstanceRef<Array<i8>>) -> Result<()> {
        Ok(())
    }

    pub(super) async fn get_transmit_mtu(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(672)
    }

    pub(super) async fn get_receive_mtu(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(672)
    }
}
