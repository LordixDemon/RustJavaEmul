use alloc::{string::String as RustString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::{
            lang::{Object, String},
            util::Vector,
        },
        javax::microedition::lcdui::{Command, Font, Graphics, Image},
    },
};

// class javax.microedition.lcdui.List
pub struct List;

impl List {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/List",
            parent_class: Some("javax/microedition/lcdui/Displayable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;I[Ljava/lang/String;[Ljavax/microedition/lcdui/Image;)V",
                    Self::init_items,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I",
                    Self::append,
                    Default::default(),
                ),
                JavaMethodProto::new("delete", "(I)V", Self::delete, Default::default()),
                JavaMethodProto::new("deleteAll", "()V", Self::delete_all, Default::default()),
                JavaMethodProto::new("getSelectedFlags", "([Z)I", Self::get_selected_flags, Default::default()),
                JavaMethodProto::new("getSelectedIndex", "()I", Self::get_selected_index, Default::default()),
                JavaMethodProto::new("getString", "(I)Ljava/lang/String;", Self::get_string, Default::default()),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/String;Ljavax/microedition/lcdui/Image;)V",
                    Self::insert,
                    Default::default(),
                ),
                JavaMethodProto::new("isSelected", "(I)Z", Self::is_selected, Default::default()),
                JavaMethodProto::new(
                    "set",
                    "(ILjava/lang/String;Ljavax/microedition/lcdui/Image;)V",
                    Self::set,
                    Default::default(),
                ),
                JavaMethodProto::new("setFitPolicy", "(I)V", Self::set_fit_policy, Default::default()),
                JavaMethodProto::new("setFont", "(ILjavax/microedition/lcdui/Font;)V", Self::set_font, Default::default()),
                JavaMethodProto::new("setSelectedFlags", "([Z)V", Self::set_selected_flags, Default::default()),
                JavaMethodProto::new("setSelectedIndex", "(IZ)V", Self::set_selected_index, Default::default()),
                JavaMethodProto::new("setTitle", "(Ljava/lang/String;)V", Self::set_title, Default::default()),
                JavaMethodProto::new(
                    "setSelectCommand",
                    "(Ljavax/microedition/lcdui/Command;)V",
                    Self::set_select_command,
                    Default::default(),
                ),
                JavaMethodProto::new("showNotify", "()V", Self::show_notify, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("title", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("listType", "I", Default::default()),
                JavaFieldProto::new("strings", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("images", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("selectedIndex", "I", Default::default()),
                JavaFieldProto::new("selectedFlags", "[Z", Default::default()),
                JavaFieldProto::new("fitPolicy", "I", Default::default()),
                JavaFieldProto::new("EXCLUSIVE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MULTIPLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("IMPLICIT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TEXT_WRAP_DEFAULT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TEXT_WRAP_ON", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TEXT_WRAP_OFF", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "SELECT_COMMAND",
                    "Ljavax/microedition/lcdui/Command;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/List";
        jvm.put_static_field(class, "EXCLUSIVE", "I", 1).await?;
        jvm.put_static_field(class, "MULTIPLE", "I", 2).await?;
        jvm.put_static_field(class, "IMPLICIT", "I", 3).await?;
        jvm.put_static_field(class, "TEXT_WRAP_DEFAULT", "I", 0).await?;
        jvm.put_static_field(class, "TEXT_WRAP_ON", "I", 1).await?;
        jvm.put_static_field(class, "TEXT_WRAP_OFF", "I", 2).await?;

        let label = JavaLangString::from_rust_string(jvm, "Select").await?;
        let select: ClassInstanceRef<Command> = jvm
            .new_class("javax/microedition/lcdui/Command", "(Ljava/lang/String;II)V", (label, 4, 0))
            .await?
            .into();
        jvm.put_static_field(class, "SELECT_COMMAND", "Ljavax/microedition/lcdui/Command;", select)
            .await
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        title: ClassInstanceRef<String>,
        list_type: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.List::<init>({this:?}, {title:?}, {list_type:?})");

        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Displayable", "<init>", "()V", ())
            .await?;
        let strings = jvm.new_class("java/util/Vector", "()V", ()).await?;
        let images = jvm.new_class("java/util/Vector", "()V", ()).await?;
        let flags = jvm.instantiate_array("Z", 0).await?;
        jvm.put_field(&mut this, "title", "Ljava/lang/String;", title).await?;
        jvm.put_field(&mut this, "listType", "I", list_type).await?;
        jvm.put_field(&mut this, "strings", "Ljava/util/Vector;", strings).await?;
        jvm.put_field(&mut this, "images", "Ljava/util/Vector;", images).await?;
        jvm.put_field(&mut this, "selectedIndex", "I", 0).await?;
        jvm.put_field(&mut this, "selectedFlags", "[Z", flags).await?;
        jvm.put_field(&mut this, "fitPolicy", "I", 0).await
    }

    async fn init_items(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        title: ClassInstanceRef<String>,
        list_type: i32,
        strings: ClassInstanceRef<Array<String>>,
        images: ClassInstanceRef<Array<Image>>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone(), title, list_type).await?;
        if strings.is_null() {
            return Ok(());
        }
        let count = jvm.array_length(&strings).await?;
        let image_count = if images.is_null() { 0 } else { jvm.array_length(&images).await? };
        for index in 0..count {
            let text: Vec<ClassInstanceRef<String>> = jvm.load_array(&strings, index, 1).await?;
            let image = if !images.is_null() && index < image_count {
                jvm.load_array(&images, index, 1)
                    .await?
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| ClassInstanceRef::new(None))
            } else {
                ClassInstanceRef::new(None)
            };
            let text = text.into_iter().next().unwrap_or_else(|| ClassInstanceRef::new(None));
            let _: i32 = Self::append(jvm, context, this.clone(), text, image).await?;
        }
        Ok(())
    }

    async fn set_select_command(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, command: ClassInstanceRef<Command>) -> Result<()> {
        if command.is_null() {
            return Ok(());
        }
        jvm.invoke_virtual(&this, "addCommand", "(Ljavax/microedition/lcdui/Command;)V", (command,))
            .await
    }

    async fn append(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        text: ClassInstanceRef<String>,
        image: ClassInstanceRef<Image>,
    ) -> Result<i32> {
        let size = Self::size(jvm, context, this.clone()).await?;
        Self::insert(jvm, context, this, size, text, image).await?;
        Ok(size)
    }

    async fn delete(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.List::delete({this:?}, {index:?})");

        let strings = Self::strings(jvm, &mut this).await?;
        let images = Self::images(jvm, &mut this).await?;
        let _: () = jvm.invoke_virtual(&strings, "removeElementAt", "(I)V", (index,)).await?;
        let _: () = jvm.invoke_virtual(&images, "removeElementAt", "(I)V", (index,)).await?;
        Self::resize_flags(jvm, &mut this).await?;
        Self::clamp_selected_index(jvm, &mut this).await
    }

    async fn delete_all(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.List::deleteAll({this:?})");

        let strings = Self::strings(jvm, &mut this).await?;
        let images = Self::images(jvm, &mut this).await?;
        let _: () = jvm.invoke_virtual(&strings, "removeAllElements", "()V", ()).await?;
        let _: () = jvm.invoke_virtual(&images, "removeAllElements", "()V", ()).await?;
        let flags = jvm.instantiate_array("Z", 0).await?;
        jvm.put_field(&mut this, "selectedFlags", "[Z", flags).await?;
        jvm.put_field(&mut this, "selectedIndex", "I", 0).await
    }

    async fn get_selected_flags(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut selected_array: ClassInstanceRef<Array<bool>>,
    ) -> Result<i32> {
        let flags: ClassInstanceRef<Array<bool>> = jvm.get_field(&this, "selectedFlags", "[Z").await?;
        let count = jvm.array_length(&flags).await?.min(jvm.array_length(&selected_array).await?);
        let values: Vec<bool> = jvm.load_array(&flags, 0, count).await?;
        let selected_count = values.iter().filter(|selected| **selected).count() as i32;
        jvm.store_array(&mut selected_array, 0, values).await?;
        Ok(selected_count)
    }

    async fn get_selected_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "selectedIndex", "I").await
    }

    async fn get_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<String>> {
        let strings = Self::strings(jvm, &mut this).await?;
        jvm.invoke_virtual(&strings, "elementAt", "(I)Ljava/lang/Object;", (index,)).await
    }

    async fn insert(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        index: i32,
        text: ClassInstanceRef<String>,
        image: ClassInstanceRef<Image>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.List::insert({this:?}, {index:?}, {text:?}, {image:?})");

        let strings = Self::strings(jvm, &mut this).await?;
        let images = Self::images(jvm, &mut this).await?;
        let text: ClassInstanceRef<Object> = ClassInstanceRef::new(text.instance);
        let image: ClassInstanceRef<Object> = ClassInstanceRef::new(image.instance);
        let _: () = jvm
            .invoke_virtual(&strings, "insertElementAt", "(Ljava/lang/Object;I)V", (text, index))
            .await?;
        let _: () = jvm
            .invoke_virtual(&images, "insertElementAt", "(Ljava/lang/Object;I)V", (image, index))
            .await?;
        Self::resize_flags(jvm, &mut this).await?;
        Self::clamp_selected_index(jvm, &mut this).await
    }

    async fn is_selected(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<bool> {
        let flags: ClassInstanceRef<Array<bool>> = jvm.get_field(&this, "selectedFlags", "[Z").await?;
        if index < 0 || index as usize >= jvm.array_length(&flags).await? {
            return Ok(false);
        }
        Ok(jvm.load_array(&flags, index as usize, 1).await?.into_iter().next().unwrap_or(false))
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        index: i32,
        text: ClassInstanceRef<String>,
        image: ClassInstanceRef<Image>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.List::set({this:?}, {index:?}, {text:?}, {image:?})");

        let strings = Self::strings(jvm, &mut this).await?;
        let images = Self::images(jvm, &mut this).await?;
        let text: ClassInstanceRef<Object> = ClassInstanceRef::new(text.instance);
        let image: ClassInstanceRef<Object> = ClassInstanceRef::new(image.instance);
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&strings, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", (index, text))
            .await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&images, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", (index, image))
            .await?;
        Ok(())
    }

    async fn set_font(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _index: i32, _font: ClassInstanceRef<Font>) -> Result<()> {
        Ok(())
    }

    async fn set_fit_policy(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, policy: i32) -> Result<()> {
        jvm.put_field(&mut this, "fitPolicy", "I", policy).await
    }

    async fn set_selected_flags(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        selected_array: ClassInstanceRef<Array<bool>>,
    ) -> Result<()> {
        let count = Self::item_count(jvm, &mut this).await?.max(0) as usize;
        let copy_count = count.min(jvm.array_length(&selected_array).await?);
        let mut flags = vec![false; count];
        if copy_count > 0 {
            let incoming: Vec<bool> = jvm.load_array(&selected_array, 0, copy_count).await?;
            flags[..copy_count].copy_from_slice(&incoming);
        }
        let selected_index = flags.iter().position(|selected| *selected).unwrap_or(0) as i32;
        let mut flags_array = jvm.instantiate_array("Z", count).await?;
        jvm.store_array(&mut flags_array, 0, flags).await?;
        jvm.put_field(&mut this, "selectedFlags", "[Z", flags_array).await?;
        jvm.put_field(&mut this, "selectedIndex", "I", selected_index).await
    }

    async fn set_selected_index(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32, selected: bool) -> Result<()> {
        let count = Self::item_count(jvm, &mut this).await?.max(0);
        if count == 0 {
            return jvm.put_field(&mut this, "selectedIndex", "I", 0).await;
        }
        let index = index.clamp(0, count - 1);
        jvm.put_field(&mut this, "selectedIndex", "I", index).await?;

        let mut flags = vec![false; count as usize];
        if selected {
            flags[index as usize] = true;
        }
        let mut flags_array = jvm.instantiate_array("Z", count as usize).await?;
        jvm.store_array(&mut flags_array, 0, flags).await?;
        jvm.put_field(&mut this, "selectedFlags", "[Z", flags_array).await
    }

    async fn set_title(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, title: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "title", "Ljava/lang/String;", title).await
    }

    async fn show_notify(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let graphics: ClassInstanceRef<Graphics> = jvm.new_class("javax/microedition/lcdui/Graphics", "()V", ()).await?.into();
        let width = context.screen_width();
        let height = context.screen_height();
        let _: () = jvm.invoke_virtual(&graphics, "setColor", "(I)V", (0x101820,)).await?;
        let _: () = jvm.invoke_virtual(&graphics, "fillRect", "(IIII)V", (0, 0, width, height)).await?;
        let _: () = jvm.invoke_virtual(&graphics, "setColor", "(I)V", (0xffffff,)).await?;

        let title: ClassInstanceRef<String> = jvm.get_field(&this, "title", "Ljava/lang/String;").await?;
        if !title.is_null() {
            let _: () = jvm
                .invoke_virtual(&graphics, "drawString", "(Ljava/lang/String;III)V", (title, width / 2, 8, 17))
                .await?;
        }

        let selected_index = Self::get_selected_index(jvm, context, this.clone()).await?;
        for (index, text) in Self::item_texts(jvm, this).await?.into_iter().take(10).enumerate() {
            let y = 34 + index as i32 * 22;
            if index as i32 == selected_index {
                let _: () = jvm.invoke_virtual(&graphics, "setColor", "(I)V", (0x2a806d,)).await?;
                let _: () = jvm.invoke_virtual(&graphics, "fillRect", "(IIII)V", (10, y - 3, width - 20, 18)).await?;
                let _: () = jvm.invoke_virtual(&graphics, "setColor", "(I)V", (0xffffff,)).await?;
            }
            let label = JavaLangString::from_rust_string(jvm, &text).await?;
            let _: () = jvm
                .invoke_virtual(&graphics, "drawString", "(Ljava/lang/String;III)V", (label, 16, y, 20))
                .await?;
        }

        context.screen_present();
        Ok(())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        Self::item_count(jvm, &mut this).await
    }

    async fn strings(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Vector>> {
        let strings: ClassInstanceRef<Vector> = jvm.get_field(this, "strings", "Ljava/util/Vector;").await?;
        if strings.is_null() {
            let strings = jvm.new_class("java/util/Vector", "()V", ()).await?;
            jvm.put_field(this, "strings", "Ljava/util/Vector;", strings.clone()).await?;
            return Ok(strings.into());
        }
        Ok(strings)
    }

    async fn images(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Vector>> {
        let images: ClassInstanceRef<Vector> = jvm.get_field(this, "images", "Ljava/util/Vector;").await?;
        if images.is_null() {
            let images = jvm.new_class("java/util/Vector", "()V", ()).await?;
            jvm.put_field(this, "images", "Ljava/util/Vector;", images.clone()).await?;
            return Ok(images.into());
        }
        Ok(images)
    }

    async fn item_count(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<i32> {
        let strings = Self::strings(jvm, this).await?;
        jvm.invoke_virtual(&strings, "size", "()I", ()).await
    }

    async fn resize_flags(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let count = Self::item_count(jvm, this).await?.max(0) as usize;
        let old_flags: ClassInstanceRef<Array<bool>> = jvm.get_field(this, "selectedFlags", "[Z").await?;
        let old_len = if old_flags.is_null() { 0 } else { jvm.array_length(&old_flags).await? };
        let mut flags = vec![false; count];
        if old_len > 0 && count > 0 {
            let copy_count = old_len.min(count);
            let existing: Vec<bool> = jvm.load_array(&old_flags, 0, copy_count).await?;
            flags[..copy_count].copy_from_slice(&existing);
        }
        let mut flags_array = jvm.instantiate_array("Z", count).await?;
        jvm.store_array(&mut flags_array, 0, flags).await?;
        jvm.put_field(this, "selectedFlags", "[Z", flags_array).await
    }

    async fn clamp_selected_index(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let count = Self::item_count(jvm, this).await?.max(0);
        let selected: i32 = jvm.get_field(this, "selectedIndex", "I").await.unwrap_or(0);
        let selected = if count == 0 { 0 } else { selected.clamp(0, count - 1) };
        jvm.put_field(this, "selectedIndex", "I", selected).await
    }

    async fn item_texts(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<Vec<RustString>> {
        let strings = Self::strings(jvm, &mut this).await?;
        let count: i32 = jvm.invoke_virtual(&strings, "size", "()I", ()).await?;
        let mut result = Vec::with_capacity(count.max(0) as usize);
        for index in 0..count {
            let text: ClassInstanceRef<String> = jvm.invoke_virtual(&strings, "elementAt", "(I)Ljava/lang/Object;", (index,)).await?;
            result.push(if text.is_null() {
                RustString::new()
            } else {
                JavaLangString::to_rust_string(jvm, &text).await?
            });
        }
        Ok(result)
    }
}
