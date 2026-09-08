use alloc::{boxed::Box, vec};
use core::time::Duration;

use dyn_clone::clone_box;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    Runtime, RuntimeClassProto, RuntimeContext, SpawnCallback,
    classes::{
        java::lang::Runnable,
        javax::microedition::{
            lcdui::{Alert, Displayable, Item},
            midlet::MIDlet,
        },
    },
};

// class javax.microedition.lcdui.Display
pub struct Display;

impl Display {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Display",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getDisplay",
                    "(Ljavax/microedition/midlet/MIDlet;)Ljavax/microedition/lcdui/Display;",
                    Self::get_display,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "setCurrent",
                    "(Ljavax/microedition/lcdui/Displayable;)V",
                    Self::set_current,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setCurrent",
                    "(Ljavax/microedition/lcdui/Alert;Ljavax/microedition/lcdui/Displayable;)V",
                    Self::set_current_alert,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getCurrent",
                    "()Ljavax/microedition/lcdui/Displayable;",
                    Self::get_current,
                    Default::default(),
                ),
                JavaMethodProto::new("callSerially", "(Ljava/lang/Runnable;)V", Self::call_serially, Default::default()),
                JavaMethodProto::new("flashBacklight", "(I)Z", Self::flash_backlight, Default::default()),
                JavaMethodProto::new("vibrate", "(I)Z", Self::vibrate, Default::default()),
                JavaMethodProto::new("isColor", "()Z", Self::is_color, Default::default()),
                JavaMethodProto::new("numColors", "()I", Self::num_colors, Default::default()),
                JavaMethodProto::new("numAlphaLevels", "()I", Self::num_alpha_levels, Default::default()),
                JavaMethodProto::new("getBestImageWidth", "(I)I", Self::get_best_image_width, Default::default()),
                JavaMethodProto::new("getBestImageHeight", "(I)I", Self::get_best_image_height, Default::default()),
                JavaMethodProto::new("getColor", "(I)I", Self::get_ui_color, Default::default()),
                JavaMethodProto::new(
                    "setCurrentItem",
                    "(Ljavax/microedition/lcdui/Item;)V",
                    Self::set_current_item,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("instance", "Ljavax/microedition/lcdui/Display;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("current", "Ljavax/microedition/lcdui/Displayable;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Display::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_display(jvm: &Jvm, _: &mut RuntimeContext, midlet: ClassInstanceRef<MIDlet>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.lcdui.Display::getDisplay({midlet:?})");

        let display: ClassInstanceRef<Self> = jvm
            .get_static_field("javax/microedition/lcdui/Display", "instance", "Ljavax/microedition/lcdui/Display;")
            .await?;
        if !display.is_null() {
            return Ok(display);
        }

        let display: ClassInstanceRef<Self> = jvm.new_class("javax/microedition/lcdui/Display", "()V", ()).await?.into();
        jvm.put_static_field(
            "javax/microedition/lcdui/Display",
            "instance",
            "Ljavax/microedition/lcdui/Display;",
            display.clone(),
        )
        .await?;

        Ok(display)
    }

    async fn set_current(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        current: ClassInstanceRef<Displayable>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Display::setCurrent({this:?}, {current:?})");

        let previous: ClassInstanceRef<Displayable> = jvm.get_field(&this, "current", "Ljavax/microedition/lcdui/Displayable;").await?;
        let previous_name = if previous.is_null() {
            "<none>".into()
        } else {
            previous.class_definition().name()
        };
        let current_name = if current.is_null() {
            "<none>".into()
        } else {
            current.class_definition().name()
        };
        tracing::info!(target: "rustjava_render", "display.setCurrent previous={} current={}", previous_name, current_name);

        if !previous.is_null() && !current.is_null() && previous.equals(&**current).unwrap_or(false) {
            return Ok(());
        }

        if !previous.is_null() && previous.class_definition().method("hideNotify", "()V", false).is_some() {
            let _: () = jvm.invoke_virtual(&previous, "hideNotify", "()V", ()).await?;
        }

        context.set_current_displayable(current.instance.clone());
        jvm.put_field(&mut this, "current", "Ljavax/microedition/lcdui/Displayable;", current.clone())
            .await?;

        if !current.is_null() && current.class_definition().method("showNotify", "()V", false).is_some() {
            struct ShowNotifyProxy {
                jvm: Jvm,
                current: ClassInstanceRef<Displayable>,
                context: Box<dyn Runtime>,
            }

            #[async_trait::async_trait]
            impl SpawnCallback for ShowNotifyProxy {
                async fn call(&self) -> Result<()> {
                    self.context.sleep(Duration::from_millis(50)).await;
                    self.jvm.attach_thread()?;

                    let result: Result<()> = self.jvm.invoke_virtual(&self.current, "showNotify", "()V", ()).await;
                    if let Err(jvm::JavaError::JavaException(exception)) = result {
                        tracing::warn!("Uncaught exception in Displayable.showNotify: {exception:?}");
                    } else if let Err(err) = result {
                        tracing::warn!("Error in Displayable.showNotify: {err:?}");
                    }

                    self.jvm.detach_thread()?;
                    Ok(())
                }
            }

            context.spawn(
                jvm,
                Box::new(ShowNotifyProxy {
                    jvm: jvm.clone(),
                    current: current.clone(),
                    context: clone_box(context),
                }),
            );
        }

        Ok(())
    }

    async fn set_current_alert(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        alert: ClassInstanceRef<Alert>,
        next: ClassInstanceRef<Displayable>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Display::setCurrent({this:?}, {alert:?}, {next:?})");

        if !alert.is_null() {
            let alert_displayable: ClassInstanceRef<Displayable> = ClassInstanceRef::new(alert.instance);
            Self::set_current(jvm, context, this.clone(), alert_displayable).await?;
        }

        if !next.is_null() {
            Self::set_current(jvm, context, this, next).await?;
        }

        Ok(())
    }

    async fn get_current(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Displayable>> {
        tracing::trace!("javax.microedition.lcdui.Display::getCurrent({this:?})");

        jvm.get_field(&this, "current", "Ljavax/microedition/lcdui/Displayable;").await
    }

    async fn call_serially(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        runnable: ClassInstanceRef<Runnable>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Display::callSerially({this:?}, {runnable:?})");

        if runnable.is_null() {
            return Ok(());
        }

        // MIDP runs the callback later on the event thread. Invoking it immediately
        // re-enters games that schedule the next frame with callSerially(this).
        struct CallSeriallyProxy {
            jvm: Jvm,
            runnable: ClassInstanceRef<Runnable>,
            context: Box<dyn Runtime>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for CallSeriallyProxy {
            async fn call(&self) -> Result<()> {
                self.context.sleep(Duration::from_millis(0)).await;
                self.jvm.attach_thread()?;

                let result: Result<()> = self.jvm.invoke_virtual(&self.runnable, "run", "()V", []).await;
                if let Err(jvm::JavaError::JavaException(exception)) = result {
                    tracing::error!("Uncaught exception in Display.callSerially: {exception:?}");
                } else {
                    result?;
                }

                self.jvm.detach_thread()?;
                Ok(())
            }
        }

        context.spawn(
            jvm,
            Box::new(CallSeriallyProxy {
                jvm: jvm.clone(),
                runnable,
                context: clone_box(context),
            }),
        );

        Ok(())
    }

    async fn flash_backlight(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, duration: i32) -> Result<bool> {
        tracing::debug!("javax.microedition.lcdui.Display::flashBacklight({this:?}, {duration:?})");

        Ok(duration > 0)
    }

    async fn vibrate(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, duration: i32) -> Result<bool> {
        tracing::debug!("javax.microedition.lcdui.Display::vibrate({this:?}, {duration:?})");

        Ok(duration > 0)
    }

    async fn is_color(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::trace!("javax.microedition.lcdui.Display::isColor({this:?})");

        Ok(true)
    }

    async fn num_colors(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Display::numColors({this:?})");

        Ok(0x0100_0000)
    }

    async fn num_alpha_levels(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Display::numAlphaLevels({this:?})");

        Ok(256)
    }

    async fn get_best_image_width(_: &Jvm, context: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _image_type: i32) -> Result<i32> {
        Ok(context.screen_width())
    }

    async fn get_best_image_height(_: &Jvm, context: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _image_type: i32) -> Result<i32> {
        Ok(context.screen_height() / 4)
    }

    async fn get_ui_color(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, color_specifier: i32) -> Result<i32> {
        Ok(match color_specifier {
            0 | 2 => 0x00ff_ffff,
            3 => 0x00ff_ffff,
            4 | 5 => 0x0000_0000,
            _ => 0x0000_0000,
        })
    }

    async fn set_current_item(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _item: ClassInstanceRef<Item>) -> Result<()> {
        Ok(())
    }
}
