use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::{lang::String, util::Vector},
        javax::microedition::lcdui::{Command, CommandListener},
    },
};

// class javax.microedition.lcdui.Displayable
pub struct Displayable;

impl Displayable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Displayable",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "addCommand",
                    "(Ljavax/microedition/lcdui/Command;)V",
                    Self::add_command,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "removeCommand",
                    "(Ljavax/microedition/lcdui/Command;)V",
                    Self::remove_command,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setCommandListener",
                    "(Ljavax/microedition/lcdui/CommandListener;)V",
                    Self::set_command_listener,
                    Default::default(),
                ),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new("isShown", "()Z", Self::is_shown, Default::default()),
                JavaMethodProto::new("showNotify", "()V", Self::show_notify, Default::default()),
                JavaMethodProto::new("hideNotify", "()V", Self::hide_notify, Default::default()),
                JavaMethodProto::new("setTitle", "(Ljava/lang/String;)V", Self::set_title, Default::default()),
                JavaMethodProto::new("getTitle", "()Ljava/lang/String;", Self::get_title, Default::default()),
                JavaMethodProto::new("setTicker", "(Ljavax/microedition/lcdui/Ticker;)V", Self::set_ticker, Default::default()),
                JavaMethodProto::new("getTicker", "()Ljavax/microedition/lcdui/Ticker;", Self::get_ticker, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("commands", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("commandListener", "Ljavax/microedition/lcdui/CommandListener;", Default::default()),
                JavaFieldProto::new("title", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("ticker", "Ljavax/microedition/lcdui/Ticker;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Displayable::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let commands = jvm.new_class("java/util/Vector", "()V", ()).await?;
        jvm.put_field(&mut this, "commands", "Ljava/util/Vector;", commands).await?;

        Ok(())
    }

    async fn add_command(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, command: ClassInstanceRef<Command>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Displayable::addCommand({this:?}, {command:?})");

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

    async fn remove_command(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, command: ClassInstanceRef<Command>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Displayable::removeCommand({this:?}, {command:?})");

        if command.is_null() {
            return Ok(());
        }

        let commands = Self::commands(jvm, &mut this).await?;
        let _: bool = jvm
            .invoke_virtual(&commands, "removeElement", "(Ljava/lang/Object;)Z", (command,))
            .await?;

        Ok(())
    }

    async fn set_command_listener(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<CommandListener>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Displayable::setCommandListener({this:?}, {listener:?})");

        jvm.put_field(&mut this, "commandListener", "Ljavax/microedition/lcdui/CommandListener;", listener)
            .await?;

        Ok(())
    }

    async fn get_width(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Displayable::getWidth({this:?})");

        Ok(context.screen_width())
    }

    async fn get_height(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Displayable::getHeight({this:?})");

        Ok(context.screen_height())
    }

    async fn is_shown(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::trace!("javax.microedition.lcdui.Displayable::isShown({this:?})");
        let Some(instance) = this.instance.as_deref() else {
            return Ok(false);
        };
        Ok(context.is_current_displayable(instance))
    }

    async fn show_notify(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Displayable::showNotify({this:?})");

        Ok(())
    }

    async fn hide_notify(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Displayable::hideNotify({this:?})");

        Ok(())
    }

    async fn set_title(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, title: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "title", "Ljava/lang/String;", title).await
    }

    async fn get_title(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "title", "Ljava/lang/String;").await
    }

    async fn set_ticker(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, ticker: ClassInstanceRef<()>) -> Result<()> {
        jvm.put_field(&mut this, "ticker", "Ljavax/microedition/lcdui/Ticker;", ticker).await
    }

    async fn get_ticker(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<()>> {
        jvm.get_field(&this, "ticker", "Ljavax/microedition/lcdui/Ticker;").await
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
