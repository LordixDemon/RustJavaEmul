use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class javax.microedition.lcdui.Command
pub struct Command;

impl Command {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Command",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;II)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;II)V",
                    Self::init_with_long_label,
                    Default::default(),
                ),
                JavaMethodProto::new("getLabel", "()Ljava/lang/String;", Self::get_label, Default::default()),
                JavaMethodProto::new("getLongLabel", "()Ljava/lang/String;", Self::get_long_label, Default::default()),
                JavaMethodProto::new("getCommandType", "()I", Self::get_command_type, Default::default()),
                JavaMethodProto::new("getPriority", "()I", Self::get_priority, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("label", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("longLabel", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("commandType", "I", Default::default()),
                JavaFieldProto::new("priority", "I", Default::default()),
                JavaFieldProto::new("SCREEN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BACK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("CANCEL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("OK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HELP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STOP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("EXIT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ITEM", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/Command";
        jvm.put_static_field(class, "SCREEN", "I", 1).await?;
        jvm.put_static_field(class, "BACK", "I", 2).await?;
        jvm.put_static_field(class, "CANCEL", "I", 3).await?;
        jvm.put_static_field(class, "OK", "I", 4).await?;
        jvm.put_static_field(class, "HELP", "I", 5).await?;
        jvm.put_static_field(class, "STOP", "I", 6).await?;
        jvm.put_static_field(class, "EXIT", "I", 7).await?;
        jvm.put_static_field(class, "ITEM", "I", 8).await
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        command_type: i32,
        priority: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Command::<init>({this:?}, {label:?}, {command_type:?}, {priority:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "label", "Ljava/lang/String;", label).await?;
        jvm.put_field(&mut this, "longLabel", "Ljava/lang/String;", ClassInstanceRef::<String>::new(None))
            .await?;
        jvm.put_field(&mut this, "commandType", "I", command_type).await?;
        jvm.put_field(&mut this, "priority", "I", priority).await?;

        Ok(())
    }

    async fn init_with_long_label(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        long_label: ClassInstanceRef<String>,
        command_type: i32,
        priority: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Command::<init>({this:?}, {label:?}, {long_label:?}, {command_type:?}, {priority:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "label", "Ljava/lang/String;", label).await?;
        jvm.put_field(&mut this, "longLabel", "Ljava/lang/String;", long_label).await?;
        jvm.put_field(&mut this, "commandType", "I", command_type).await?;
        jvm.put_field(&mut this, "priority", "I", priority).await
    }

    async fn get_label(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "label", "Ljava/lang/String;").await
    }

    async fn get_long_label(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let long_label: ClassInstanceRef<String> = jvm.get_field(&this, "longLabel", "Ljava/lang/String;").await?;
        if long_label.is_null() {
            return jvm.get_field(&this, "label", "Ljava/lang/String;").await;
        }
        Ok(long_label)
    }

    async fn get_command_type(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "commandType", "I").await
    }

    async fn get_priority(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "priority", "I").await
    }
}
