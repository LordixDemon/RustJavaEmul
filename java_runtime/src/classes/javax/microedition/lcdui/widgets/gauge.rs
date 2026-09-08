#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

impl Gauge {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Gauge",
            parent_class: Some("javax/microedition/lcdui/Item"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;ZII)V", Self::init, Default::default()),
                JavaMethodProto::new("getMaxValue", "()I", Self::get_max_value, Default::default()),
                JavaMethodProto::new("getValue", "()I", Self::get_value, Default::default()),
                JavaMethodProto::new("isInteractive", "()Z", Self::is_interactive, Default::default()),
                JavaMethodProto::new("setMaxValue", "(I)V", Self::set_max_value, Default::default()),
                JavaMethodProto::new("setValue", "(I)V", Self::set_value, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("interactive", "Z", Default::default()),
                JavaFieldProto::new("maxValue", "I", Default::default()),
                JavaFieldProto::new("value", "I", Default::default()),
                JavaFieldProto::new("INDEFINITE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("CONTINUOUS_IDLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INCREMENTAL_IDLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("CONTINUOUS_RUNNING", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INCREMENTAL_UPDATING", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/Gauge";
        jvm.put_static_field(class, "INDEFINITE", "I", -1).await?;
        jvm.put_static_field(class, "CONTINUOUS_IDLE", "I", 0).await?;
        jvm.put_static_field(class, "INCREMENTAL_IDLE", "I", 1).await?;
        jvm.put_static_field(class, "CONTINUOUS_RUNNING", "I", 2).await?;
        jvm.put_static_field(class, "INCREMENTAL_UPDATING", "I", 3).await
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        interactive: bool,
        max_value: i32,
        initial_value: i32,
    ) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Item", "<init>", "(Ljava/lang/String;)V", (label,))
            .await?;
        jvm.put_field(&mut this, "interactive", "Z", interactive).await?;
        jvm.put_field(&mut this, "maxValue", "I", max_value).await?;
        jvm.put_field(&mut this, "value", "I", initial_value).await
    }

    pub(super) async fn get_max_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "maxValue", "I").await
    }

    pub(super) async fn get_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "value", "I").await
    }

    pub(super) async fn is_interactive(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "interactive", "Z").await
    }

    pub(super) async fn set_max_value(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, max_value: i32) -> Result<()> {
        jvm.put_field(&mut this, "maxValue", "I", max_value).await
    }

    pub(super) async fn set_value(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "value", "I", value).await
    }
}
