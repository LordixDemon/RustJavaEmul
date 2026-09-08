#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::{boxed::Box, format, vec};
use java_class_proto::JavaMethodProto;
#[allow(unused_imports)]
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl L2CAPConnectionNotifier {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/L2CAPConnectionNotifier",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/Connection"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init_url, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new(
                    "acceptAndOpen",
                    "()Ljavax/bluetooth/L2CAPConnection;",
                    Self::accept_and_open,
                    Default::default(),
                ),
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

    pub(super) async fn accept_and_open(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<L2CAPConnection>> {
        Ok(jvm.new_class("javax/bluetooth/L2CAPConnection", "()V", ()).await?.into())
    }
}
