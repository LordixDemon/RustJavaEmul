use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::{lang::String, util::Vector},
        javax::microedition::lcdui::{Image, ImageItem, Item, ItemStateListener, StringItem},
    },
};

// class javax.microedition.lcdui.Form
pub struct Form;

impl Form {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Form",
            parent_class: Some("javax/microedition/lcdui/Displayable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new("append", "(Ljava/lang/String;)I", Self::append, Default::default()),
                JavaMethodProto::new("append", "(Ljavax/microedition/lcdui/Image;)I", Self::append_image, Default::default()),
                JavaMethodProto::new("append", "(Ljavax/microedition/lcdui/Item;)I", Self::append_item, Default::default()),
                JavaMethodProto::new("delete", "(I)V", Self::delete, Default::default()),
                JavaMethodProto::new("deleteAll", "()V", Self::delete_all, Default::default()),
                JavaMethodProto::new("get", "(I)Ljavax/microedition/lcdui/Item;", Self::get, Default::default()),
                JavaMethodProto::new("getTitle", "()Ljava/lang/String;", Self::get_title, Default::default()),
                JavaMethodProto::new("insert", "(ILjavax/microedition/lcdui/Item;)V", Self::insert, Default::default()),
                JavaMethodProto::new("set", "(ILjavax/microedition/lcdui/Item;)V", Self::set, Default::default()),
                JavaMethodProto::new(
                    "setItemStateListener",
                    "(Ljavax/microedition/lcdui/ItemStateListener;)V",
                    Self::set_item_state_listener,
                    Default::default(),
                ),
                JavaMethodProto::new("setTitle", "(Ljava/lang/String;)V", Self::set_title, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("title", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("itemCount", "I", Default::default()),
                JavaFieldProto::new("items", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("itemStateListener", "Ljavax/microedition/lcdui/ItemStateListener;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, title: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Form::<init>({this:?}, {title:?})");

        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Displayable", "<init>", "()V", ())
            .await?;
        let items = jvm.new_class("java/util/Vector", "()V", ()).await?;
        jvm.put_field(&mut this, "title", "Ljava/lang/String;", title).await?;
        jvm.put_field(&mut this, "itemCount", "I", 0).await?;
        jvm.put_field(&mut this, "items", "Ljava/util/Vector;", items).await?;
        jvm.put_field(
            &mut this,
            "itemStateListener",
            "Ljavax/microedition/lcdui/ItemStateListener;",
            ClassInstanceRef::<ItemStateListener>::new(None),
        )
        .await
    }

    async fn append(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, text: ClassInstanceRef<String>) -> Result<i32> {
        let text = if text.is_null() { ClassInstanceRef::<String>::new(None) } else { text };
        let text_for_log = if text.is_null() {
            alloc::string::String::new()
        } else {
            JavaLangString::to_rust_string(jvm, &text).await?
        };
        tracing::debug!("javax.microedition.lcdui.Form::append({this:?}, {text_for_log:?})");

        let item: ClassInstanceRef<StringItem> = jvm
            .new_class(
                "javax/microedition/lcdui/StringItem",
                "(Ljava/lang/String;Ljava/lang/String;)V",
                (ClassInstanceRef::<String>::new(None), text),
            )
            .await?
            .into();
        Self::append_item(jvm, context, this, ClassInstanceRef::<Item>::new(item.instance)).await
    }

    async fn append_image(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, image: ClassInstanceRef<Image>) -> Result<i32> {
        tracing::debug!("javax.microedition.lcdui.Form::append({this:?}, {image:?})");

        let item: ClassInstanceRef<ImageItem> = jvm
            .new_class(
                "javax/microedition/lcdui/ImageItem",
                "(Ljava/lang/String;Ljavax/microedition/lcdui/Image;ILjava/lang/String;)V",
                (ClassInstanceRef::<String>::new(None), image, 0, ClassInstanceRef::<String>::new(None)),
            )
            .await?
            .into();
        Self::append_item(jvm, context, this, ClassInstanceRef::<Item>::new(item.instance)).await
    }

    async fn append_item(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, item: ClassInstanceRef<Item>) -> Result<i32> {
        tracing::debug!("javax.microedition.lcdui.Form::append({this:?}, {item:?})");

        if item.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "item is null").await);
        }

        let items = Self::items(jvm, &mut this).await?;
        let index: i32 = jvm.invoke_virtual(&items, "size", "()I", ()).await?;
        let _: () = jvm.invoke_virtual(&items, "addElement", "(Ljava/lang/Object;)V", (item,)).await?;
        Self::sync_item_count(jvm, &mut this, &items).await?;
        Ok(index)
    }

    async fn delete(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Form::delete({this:?}, {index:?})");

        let items = Self::items(jvm, &mut this).await?;
        let _: () = jvm.invoke_virtual(&items, "removeElementAt", "(I)V", (index,)).await?;
        Self::sync_item_count(jvm, &mut this, &items).await
    }

    async fn delete_all(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Form::deleteAll({this:?})");

        let items = Self::items(jvm, &mut this).await?;
        let _: () = jvm.invoke_virtual(&items, "removeAllElements", "()V", ()).await?;
        jvm.put_field(&mut this, "itemCount", "I", 0).await
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Item>> {
        tracing::debug!("javax.microedition.lcdui.Form::get({this:?}, {index:?})");

        let items = Self::items(jvm, &mut this).await?;
        jvm.invoke_virtual(&items, "elementAt", "(I)Ljava/lang/Object;", (index,)).await
    }

    async fn get_title(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("javax.microedition.lcdui.Form::getTitle({this:?})");

        jvm.get_field(&this, "title", "Ljava/lang/String;").await
    }

    async fn set_title(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, title: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Form::setTitle({this:?}, {title:?})");

        jvm.put_field(&mut this, "title", "Ljava/lang/String;", title).await
    }

    async fn insert(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32, item: ClassInstanceRef<Item>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Form::insert({this:?}, {index:?}, {item:?})");

        if item.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "item is null").await);
        }

        let items = Self::items(jvm, &mut this).await?;
        let _: () = jvm
            .invoke_virtual(&items, "insertElementAt", "(Ljava/lang/Object;I)V", (item, index))
            .await?;
        Self::sync_item_count(jvm, &mut this, &items).await
    }

    async fn set(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32, item: ClassInstanceRef<Item>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Form::set({this:?}, {index:?}, {item:?})");

        if item.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "item is null").await);
        }

        let items = Self::items(jvm, &mut this).await?;
        let _: ClassInstanceRef<Item> = jvm
            .invoke_virtual(&items, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", (index, item))
            .await?;
        Self::sync_item_count(jvm, &mut this, &items).await
    }

    async fn set_item_state_listener(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<ItemStateListener>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Form::setItemStateListener({this:?}, {listener:?})");

        jvm.put_field(&mut this, "itemStateListener", "Ljavax/microedition/lcdui/ItemStateListener;", listener)
            .await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.lcdui.Form::size({this:?})");

        let items = Self::items(jvm, &mut this).await?;
        jvm.invoke_virtual(&items, "size", "()I", ()).await
    }

    async fn items(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Vector>> {
        let items: ClassInstanceRef<Vector> = jvm.get_field(this, "items", "Ljava/util/Vector;").await?;
        if items.is_null() {
            let items = jvm.new_class("java/util/Vector", "()V", ()).await?;
            jvm.put_field(this, "items", "Ljava/util/Vector;", items.clone()).await?;
            return Ok(items.into());
        }
        Ok(items)
    }

    async fn sync_item_count(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, items: &ClassInstanceRef<Vector>) -> Result<()> {
        let count: i32 = jvm.invoke_virtual(items, "size", "()I", ()).await?;
        jvm.put_field(this, "itemCount", "I", count).await
    }
}
