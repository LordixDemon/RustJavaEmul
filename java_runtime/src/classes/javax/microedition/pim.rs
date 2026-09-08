use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{lang::String, util::Enumeration},
};

pub struct PIM;
pub struct PIMList;
pub struct Contact;
pub struct ContactList;

simple_exception!(
    PIMException,
    "javax/microedition/pim/PIMException",
    "java/lang/Exception",
    "javax.microedition.pim.PIMException"
);
simple_exception!(
    UnsupportedFieldException,
    "javax/microedition/pim/UnsupportedFieldException",
    "java/lang/RuntimeException",
    "javax.microedition.pim.UnsupportedFieldException"
);

impl PIM {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/pim/PIM",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getInstance",
                    "()Ljavax/microedition/pim/PIM;",
                    Self::get_instance,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "openPIMList",
                    "(II)Ljavax/microedition/pim/PIMList;",
                    Self::open_pim_list,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("javax/microedition/pim/PIM", "()V", ()).await?.into())
    }
    async fn open_pim_list(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _type: i32,
        _mode: i32,
    ) -> Result<ClassInstanceRef<PIMList>> {
        Ok(jvm.new_class("javax/microedition/pim/ContactList", "()V", ()).await?.into())
    }
}

impl PIMList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/pim/PIMList",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("items", "()Ljava/util/Enumeration;", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("close", "()V", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("getName", "()Ljava/lang/String;", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("isSupportedField", "(I)Z", MethodAccessFlags::ABSTRACT),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

pub struct PIMItem;

impl PIMItem {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/pim/PIMItem",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("getString", "(II)Ljava/lang/String;", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("getStringArray", "(II)[Ljava/lang/String;", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("countValues", "(I)I", MethodAccessFlags::ABSTRACT),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl Contact {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/pim/Contact",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/pim/PIMItem"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("countValues", "(I)I", Self::count_values, Default::default()),
                JavaMethodProto::new("getString", "(II)Ljava/lang/String;", Self::get_string, Default::default()),
                JavaMethodProto::new("getStringArray", "(II)[Ljava/lang/String;", Self::get_string_array, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn count_values(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _field: i32) -> Result<i32> {
        Ok(0)
    }
    async fn get_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _field: i32,
        _index: i32,
    ) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "").await.map(Into::into)
    }
    async fn get_string_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _field: i32,
        _index: i32,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<String>>>> {
        Ok(jvm.instantiate_array("Ljava/lang/String;", 0).await?.into())
    }
}

impl ContactList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/pim/ContactList",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/pim/PIMList"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("isSupportedField", "(I)Z", Self::is_supported_field, Default::default()),
                JavaMethodProto::new("items", "()Ljava/util/Enumeration;", Self::items, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    stub_void! {
        close();
    }
    async fn is_supported_field(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _field: i32) -> Result<bool> {
        Ok(false)
    }
    async fn items(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Enumeration>> {
        let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
        jvm.invoke_virtual(&vector, "elements", "()Ljava/util/Enumeration;", ()).await
    }
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![Contact, ContactList, PIM, PIMException, PIMItem, PIMList, UnsupportedFieldException]
}
