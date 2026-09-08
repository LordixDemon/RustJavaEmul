#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Object, String},
        util::Date,
    },
};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

impl DateField {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/DateField",
            parent_class: Some("javax/microedition/lcdui/Item"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;ILjava/util/TimeZone;)V", Self::init_tz, Default::default()),
                JavaMethodProto::new("getDate", "()Ljava/util/Date;", Self::get_date, Default::default()),
                JavaMethodProto::new("getInputMode", "()I", Self::get_input_mode, Default::default()),
                JavaMethodProto::new("setDate", "(Ljava/util/Date;)V", Self::set_date, Default::default()),
                JavaMethodProto::new("setInputMode", "(I)V", Self::set_input_mode, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("date", "Ljava/util/Date;", Default::default()),
                JavaFieldProto::new("inputMode", "I", Default::default()),
                JavaFieldProto::new("DATE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TIME", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DATE_TIME", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/DateField";
        jvm.put_static_field(class, "DATE", "I", 1).await?;
        jvm.put_static_field(class, "TIME", "I", 2).await?;
        jvm.put_static_field(class, "DATE_TIME", "I", 3).await
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        mode: i32,
    ) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Item", "<init>", "(Ljava/lang/String;)V", (label,))
            .await?;
        jvm.put_field(&mut this, "inputMode", "I", mode).await?;
        jvm.put_field(&mut this, "date", "Ljava/util/Date;", ClassInstanceRef::<Date>::new(None))
            .await
    }

    pub(super) async fn init_tz(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        mode: i32,
        _tz: ClassInstanceRef<Object>,
    ) -> Result<()> {
        Self::init(jvm, context, this, label, mode).await
    }

    pub(super) async fn get_date(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Date>> {
        jvm.get_field(&this, "date", "Ljava/util/Date;").await
    }

    pub(super) async fn get_input_mode(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "inputMode", "I").await
    }

    pub(super) async fn set_date(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, date: ClassInstanceRef<Date>) -> Result<()> {
        jvm.put_field(&mut this, "date", "Ljava/util/Date;", date).await
    }

    pub(super) async fn set_input_mode(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, mode: i32) -> Result<()> {
        jvm.put_field(&mut this, "inputMode", "I", mode).await
    }
}
