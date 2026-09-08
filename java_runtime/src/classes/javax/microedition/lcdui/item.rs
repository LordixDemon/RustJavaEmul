use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::{lang::String, util::Vector},
        javax::microedition::lcdui::{Command, ItemCommandListener},
    },
};

// class javax.microedition.lcdui.Item
pub struct Item;

impl Item {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Item",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init_empty, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "addCommand",
                    "(Ljavax/microedition/lcdui/Command;)V",
                    Self::add_command,
                    Default::default(),
                ),
                JavaMethodProto::new("getLabel", "()Ljava/lang/String;", Self::get_label, Default::default()),
                JavaMethodProto::new("getLayout", "()I", Self::get_layout, Default::default()),
                JavaMethodProto::new(
                    "removeCommand",
                    "(Ljavax/microedition/lcdui/Command;)V",
                    Self::remove_command,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setDefaultCommand",
                    "(Ljavax/microedition/lcdui/Command;)V",
                    Self::set_default_command,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setItemCommandListener",
                    "(Ljavax/microedition/lcdui/ItemCommandListener;)V",
                    Self::set_item_command_listener,
                    Default::default(),
                ),
                JavaMethodProto::new("setLabel", "(Ljava/lang/String;)V", Self::set_label, Default::default()),
                JavaMethodProto::new("setLayout", "(I)V", Self::set_layout, Default::default()),
                JavaMethodProto::new("setPreferredSize", "(II)V", Self::set_preferred_size, Default::default()),
                JavaMethodProto::new("notifyStateChanged", "()V", Self::notify_state_changed, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("label", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("layout", "I", Default::default()),
                JavaFieldProto::new("commands", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("defaultCommand", "Ljavax/microedition/lcdui/Command;", Default::default()),
                JavaFieldProto::new(
                    "itemCommandListener",
                    "Ljavax/microedition/lcdui/ItemCommandListener;",
                    Default::default(),
                ),
                JavaFieldProto::new("LAYOUT_DEFAULT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_LEFT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_RIGHT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_CENTER", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_TOP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_BOTTOM", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_VCENTER", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_NEWLINE_BEFORE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_NEWLINE_AFTER", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_SHRINK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_EXPAND", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_VSHRINK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_VEXPAND", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LAYOUT_2", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PLAIN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HYPERLINK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BUTTON", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/Item";
        jvm.put_static_field(class, "LAYOUT_DEFAULT", "I", 0).await?;
        jvm.put_static_field(class, "LAYOUT_LEFT", "I", 1).await?;
        jvm.put_static_field(class, "LAYOUT_RIGHT", "I", 2).await?;
        jvm.put_static_field(class, "LAYOUT_CENTER", "I", 3).await?;
        jvm.put_static_field(class, "LAYOUT_TOP", "I", 0x10).await?;
        jvm.put_static_field(class, "LAYOUT_BOTTOM", "I", 0x20).await?;
        jvm.put_static_field(class, "LAYOUT_VCENTER", "I", 0x30).await?;
        jvm.put_static_field(class, "LAYOUT_NEWLINE_BEFORE", "I", 0x100).await?;
        jvm.put_static_field(class, "LAYOUT_NEWLINE_AFTER", "I", 0x200).await?;
        jvm.put_static_field(class, "LAYOUT_SHRINK", "I", 0x400).await?;
        jvm.put_static_field(class, "LAYOUT_EXPAND", "I", 0x800).await?;
        jvm.put_static_field(class, "LAYOUT_VSHRINK", "I", 0x1000).await?;
        jvm.put_static_field(class, "LAYOUT_VEXPAND", "I", 0x2000).await?;
        jvm.put_static_field(class, "LAYOUT_2", "I", 0x4000).await?;
        jvm.put_static_field(class, "PLAIN", "I", 0).await?;
        jvm.put_static_field(class, "HYPERLINK", "I", 1).await?;
        jvm.put_static_field(class, "BUTTON", "I", 2).await
    }

    async fn init_empty(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        Self::init(jvm, context, this, ClassInstanceRef::<String>::new(None)).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, label: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Item::<init>({this:?}, {label:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let commands = jvm.new_class("java/util/Vector", "()V", ()).await?;
        jvm.put_field(&mut this, "label", "Ljava/lang/String;", label).await?;
        jvm.put_field(&mut this, "layout", "I", 0).await?;
        jvm.put_field(&mut this, "commands", "Ljava/util/Vector;", commands).await?;
        jvm.put_field(
            &mut this,
            "defaultCommand",
            "Ljavax/microedition/lcdui/Command;",
            ClassInstanceRef::<Command>::new(None),
        )
        .await?;
        jvm.put_field(
            &mut this,
            "itemCommandListener",
            "Ljavax/microedition/lcdui/ItemCommandListener;",
            ClassInstanceRef::<ItemCommandListener>::new(None),
        )
        .await
    }

    async fn add_command(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, command: ClassInstanceRef<Command>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Item::addCommand({this:?}, {command:?})");

        if command.is_null() {
            return Ok(());
        }

        let commands = Self::commands(jvm, &mut this).await?;
        let index: i32 = jvm
            .invoke_virtual(&commands, "indexOf", "(Ljava/lang/Object;)I", (command.clone(),))
            .await?;
        if index < 0 {
            let _: () = jvm.invoke_virtual(&commands, "addElement", "(Ljava/lang/Object;)V", (command,)).await?;
        }

        Ok(())
    }

    async fn get_label(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "label", "Ljava/lang/String;").await
    }

    async fn get_layout(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "layout", "I").await
    }

    async fn remove_command(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, command: ClassInstanceRef<Command>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Item::removeCommand({this:?}, {command:?})");

        if command.is_null() {
            return Ok(());
        }

        let commands = Self::commands(jvm, &mut this).await?;
        let _: bool = jvm
            .invoke_virtual(&commands, "removeElement", "(Ljava/lang/Object;)Z", (command,))
            .await?;

        Ok(())
    }

    async fn set_default_command(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        command: ClassInstanceRef<Command>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Item::setDefaultCommand({this:?}, {command:?})");

        jvm.put_field(&mut this, "defaultCommand", "Ljavax/microedition/lcdui/Command;", command)
            .await
    }

    async fn set_item_command_listener(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<ItemCommandListener>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Item::setItemCommandListener({this:?}, {listener:?})");

        jvm.put_field(
            &mut this,
            "itemCommandListener",
            "Ljavax/microedition/lcdui/ItemCommandListener;",
            listener,
        )
        .await
    }

    async fn set_label(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, label: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Item::setLabel({this:?}, {label:?})");

        jvm.put_field(&mut this, "label", "Ljava/lang/String;", label).await
    }

    async fn set_layout(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, layout: i32) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Item::setLayout({this:?}, {layout:?})");

        jvm.put_field(&mut this, "layout", "I", layout).await
    }

    async fn set_preferred_size(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _width: i32, _height: i32) -> Result<()> {
        Ok(())
    }

    async fn notify_state_changed(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Item::notifyStateChanged({this:?})");

        Ok(())
    }

    async fn commands(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Vector>> {
        let commands: ClassInstanceRef<Vector> = jvm.get_field(this, "commands", "Ljava/util/Vector;").await?;
        if commands.is_null() {
            let commands = jvm.new_class("java/util/Vector", "()V", ()).await?;
            jvm.put_field(this, "commands", "Ljava/util/Vector;", commands.clone()).await?;
            return Ok(commands.into());
        }
        Ok(commands)
    }
}
