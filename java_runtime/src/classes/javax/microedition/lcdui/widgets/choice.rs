#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::lang::{Object, String},
        javax::microedition::lcdui::{Font, Image},
    },
};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl Choice {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Choice",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![
                JavaFieldProto::new("EXCLUSIVE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MULTIPLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("IMPLICIT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("POPUP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TEXT_WRAP_DEFAULT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TEXT_WRAP_ON", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TEXT_WRAP_OFF", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl ChoiceGroup {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/ChoiceGroup",
            parent_class: Some("javax/microedition/lcdui/Item"),
            interfaces: vec!["javax/microedition/lcdui/Choice"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;I[Ljava/lang/String;[Ljavax/microedition/lcdui/Image;)V",
                    Self::init_with_items,
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
                JavaMethodProto::new("getImage", "(I)Ljavax/microedition/lcdui/Image;", Self::get_image, Default::default()),
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
                JavaMethodProto::new("getFitPolicy", "()I", Self::get_fit_policy, Default::default()),
                JavaMethodProto::new("setSelectedFlags", "([Z)V", Self::set_selected_flags, Default::default()),
                JavaMethodProto::new("setSelectedIndex", "(IZ)V", Self::set_selected_index, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("choiceType", "I", Default::default()),
                JavaFieldProto::new("strings", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("images", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("selectedIndex", "I", Default::default()),
                JavaFieldProto::new("fitPolicy", "I", Default::default()),
                JavaFieldProto::new("EXCLUSIVE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MULTIPLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("IMPLICIT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("POPUP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/ChoiceGroup";
        jvm.put_static_field(class, "EXCLUSIVE", "I", 1).await?;
        jvm.put_static_field(class, "MULTIPLE", "I", 2).await?;
        jvm.put_static_field(class, "IMPLICIT", "I", 3).await?;
        jvm.put_static_field(class, "POPUP", "I", 4).await
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        choice_type: i32,
    ) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Item", "<init>", "(Ljava/lang/String;)V", (label,))
            .await?;
        let strings = jvm.new_class("java/util/Vector", "()V", ()).await?;
        let images = jvm.new_class("java/util/Vector", "()V", ()).await?;
        jvm.put_field(&mut this, "choiceType", "I", choice_type).await?;
        jvm.put_field(&mut this, "strings", "Ljava/util/Vector;", strings).await?;
        jvm.put_field(&mut this, "images", "Ljava/util/Vector;", images).await?;
        jvm.put_field(&mut this, "selectedIndex", "I", 0).await?;
        jvm.put_field(&mut this, "fitPolicy", "I", 0).await
    }

    pub(super) async fn init_with_items(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        choice_type: i32,
        string_elements: ClassInstanceRef<Array<ClassInstanceRef<String>>>,
        image_elements: ClassInstanceRef<Array<ClassInstanceRef<Image>>>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone(), label, choice_type).await?;
        if string_elements.is_null() {
            return Ok(());
        }
        let count = jvm.array_length(&string_elements).await?;
        for i in 0..count {
            let string: ClassInstanceRef<String> = jvm.load_array(&string_elements, i, 1).await?.into_iter().next().unwrap();
            let image: ClassInstanceRef<Image> = if image_elements.is_null() {
                ClassInstanceRef::new(None)
            } else {
                jvm.load_array(&image_elements, i.min(jvm.array_length(&image_elements).await?), 1)
                    .await?
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| ClassInstanceRef::new(None))
            };
            let _: i32 = jvm
                .invoke_virtual(&this, "append", "(Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I", (string, image))
                .await?;
        }
        Ok(())
    }

    pub(super) async fn append(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        image: ClassInstanceRef<Image>,
    ) -> Result<i32> {
        let strings: ClassInstanceRef<Object> = jvm.get_field(&this, "strings", "Ljava/util/Vector;").await?;
        let images: ClassInstanceRef<Object> = jvm.get_field(&this, "images", "Ljava/util/Vector;").await?;
        let _: () = jvm.invoke_virtual(&strings, "addElement", "(Ljava/lang/Object;)V", (string,)).await?;
        let _: () = jvm.invoke_virtual(&images, "addElement", "(Ljava/lang/Object;)V", (image,)).await?;
        let size: i32 = jvm.invoke_virtual(&strings, "size", "()I", ()).await?;
        Ok(size - 1)
    }

    pub(super) async fn delete(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        let strings: ClassInstanceRef<Object> = jvm.get_field(&this, "strings", "Ljava/util/Vector;").await?;
        let images: ClassInstanceRef<Object> = jvm.get_field(&this, "images", "Ljava/util/Vector;").await?;
        let _: () = jvm.invoke_virtual(&strings, "removeElementAt", "(I)V", (index,)).await?;
        let _: () = jvm.invoke_virtual(&images, "removeElementAt", "(I)V", (index,)).await?;
        Ok(())
    }

    pub(super) async fn delete_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let strings: ClassInstanceRef<Object> = jvm.get_field(&this, "strings", "Ljava/util/Vector;").await?;
        let images: ClassInstanceRef<Object> = jvm.get_field(&this, "images", "Ljava/util/Vector;").await?;
        let _: () = jvm.invoke_virtual(&strings, "removeAllElements", "()V", ()).await?;
        jvm.invoke_virtual(&images, "removeAllElements", "()V", ()).await
    }

    pub(super) async fn get_selected_flags(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        flags: ClassInstanceRef<Array<bool>>,
    ) -> Result<i32> {
        let selected: i32 = jvm.get_field(&this, "selectedIndex", "I").await?;
        if flags.is_null() {
            return Ok(0);
        }
        let len = jvm.array_length(&flags).await?;
        let mut count = 0;
        for i in 0..len {
            let on = i as i32 == selected;
            if on {
                count += 1;
            }
            jvm.store_array(&mut flags.clone().into(), i, vec![on]).await?;
        }
        Ok(count)
    }

    pub(super) async fn get_selected_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "selectedIndex", "I").await
    }

    pub(super) async fn get_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<String>> {
        let strings: ClassInstanceRef<Object> = jvm.get_field(&this, "strings", "Ljava/util/Vector;").await?;
        jvm.invoke_virtual(&strings, "elementAt", "(I)Ljava/lang/Object;", (index,)).await
    }

    pub(super) async fn get_image(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Image>> {
        let images: ClassInstanceRef<Object> = jvm.get_field(&this, "images", "Ljava/util/Vector;").await?;
        jvm.invoke_virtual(&images, "elementAt", "(I)Ljava/lang/Object;", (index,)).await
    }

    pub(super) async fn insert(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        string: ClassInstanceRef<String>,
        image: ClassInstanceRef<Image>,
    ) -> Result<()> {
        let strings: ClassInstanceRef<Object> = jvm.get_field(&this, "strings", "Ljava/util/Vector;").await?;
        let images: ClassInstanceRef<Object> = jvm.get_field(&this, "images", "Ljava/util/Vector;").await?;
        let _: () = jvm
            .invoke_virtual(&strings, "insertElementAt", "(Ljava/lang/Object;I)V", (string, index))
            .await?;
        jvm.invoke_virtual(&images, "insertElementAt", "(Ljava/lang/Object;I)V", (image, index))
            .await
    }

    pub(super) async fn is_selected(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<bool> {
        let selected: i32 = jvm.get_field(&this, "selectedIndex", "I").await?;
        Ok(selected == index)
    }

    pub(super) async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        string: ClassInstanceRef<String>,
        image: ClassInstanceRef<Image>,
    ) -> Result<()> {
        let strings: ClassInstanceRef<Object> = jvm.get_field(&this, "strings", "Ljava/util/Vector;").await?;
        let images: ClassInstanceRef<Object> = jvm.get_field(&this, "images", "Ljava/util/Vector;").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&strings, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", (index, string))
            .await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&images, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", (index, image))
            .await?;
        Ok(())
    }

    pub(super) async fn set_fit_policy(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, policy: i32) -> Result<()> {
        jvm.put_field(&mut this, "fitPolicy", "I", policy).await
    }

    pub(super) async fn set_font(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _index: i32,
        _font: ClassInstanceRef<Font>,
    ) -> Result<()> {
        Ok(())
    }

    pub(super) async fn get_fit_policy(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "fitPolicy", "I").await
    }

    pub(super) async fn set_selected_flags(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        flags: ClassInstanceRef<Array<bool>>,
    ) -> Result<()> {
        if flags.is_null() {
            return Ok(());
        }
        let len = jvm.array_length(&flags).await?;
        for i in 0..len {
            let on: bool = jvm.load_array(&flags, i, 1).await?.into_iter().next().unwrap_or(false);
            if on {
                return jvm.put_field(&mut this, "selectedIndex", "I", i as i32).await;
            }
        }
        Ok(())
    }

    pub(super) async fn set_selected_index(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        index: i32,
        selected: bool,
    ) -> Result<()> {
        if selected {
            jvm.put_field(&mut this, "selectedIndex", "I", index).await
        } else {
            Ok(())
        }
    }

    pub(super) async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let strings: ClassInstanceRef<Object> = jvm.get_field(&this, "strings", "Ljava/util/Vector;").await?;
        jvm.invoke_virtual(&strings, "size", "()I", ()).await
    }
}
