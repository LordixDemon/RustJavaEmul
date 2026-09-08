#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::{boxed::Box, format, vec};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
#[allow(unused_imports)]
use java_constants::{FieldAccessFlags, MethodAccessFlags};
#[allow(unused_imports)]
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

impl DataElement {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/DataElement",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_type, Default::default()),
                JavaMethodProto::new("<init>", "(IJ)V", Self::init_long, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/Object;)V", Self::init_value, Default::default()),
                JavaMethodProto::new("<init>", "(ILjava/lang/Object;)V", Self::init_type_value, Default::default()),
                JavaMethodProto::new("<init>", "(Z)V", Self::init_bool, Default::default()),
                JavaMethodProto::new("addElement", "(Ljavax/bluetooth/DataElement;)V", Self::add_element, Default::default()),
                JavaMethodProto::new("getDataType", "()I", Self::get_data_type, Default::default()),
                JavaMethodProto::new("getLong", "()J", Self::get_long, Default::default()),
                JavaMethodProto::new("getValue", "()Ljava/lang/Object;", Self::get_value, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("type", "I", Default::default()),
                JavaFieldProto::new("longValue", "J", Default::default()),
                JavaFieldProto::new("value", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("NULL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("U_INT_1", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("U_INT_2", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("U_INT_4", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INT_1", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INT_2", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INT_4", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INT_8", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("URL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("UUID", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BOOL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STRING", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DATSEQ", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DATALT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/bluetooth/DataElement";
        jvm.put_static_field(class, "NULL", "I", 0).await?;
        jvm.put_static_field(class, "U_INT_1", "I", 0x08).await?;
        jvm.put_static_field(class, "U_INT_2", "I", 0x09).await?;
        jvm.put_static_field(class, "U_INT_4", "I", 0x0a).await?;
        jvm.put_static_field(class, "INT_1", "I", 0x10).await?;
        jvm.put_static_field(class, "INT_2", "I", 0x11).await?;
        jvm.put_static_field(class, "INT_4", "I", 0x12).await?;
        jvm.put_static_field(class, "INT_8", "I", 0x13).await?;
        jvm.put_static_field(class, "URL", "I", 0x40).await?;
        jvm.put_static_field(class, "UUID", "I", 0x18).await?;
        jvm.put_static_field(class, "BOOL", "I", 0x28).await?;
        jvm.put_static_field(class, "STRING", "I", 0x20).await?;
        jvm.put_static_field(class, "DATSEQ", "I", 0x30).await?;
        jvm.put_static_field(class, "DATALT", "I", 0x38).await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn init_type(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, data_type: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "type", "I", data_type).await
    }

    pub(super) async fn init_long(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, data_type: i32, value: i64) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "type", "I", data_type).await?;
        jvm.put_field(&mut this, "longValue", "J", value).await
    }

    pub(super) async fn init_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<crate::classes::java::lang::Object>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "type", "I", 0x20).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await
    }

    pub(super) async fn init_type_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data_type: i32,
        value: ClassInstanceRef<crate::classes::java::lang::Object>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "type", "I", data_type).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await
    }

    pub(super) async fn init_bool(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "type", "I", 0x28).await?;
        jvm.put_field(&mut this, "longValue", "J", if value { 1i64 } else { 0 }).await
    }

    stub_void! {
        add_element(_element: ClassInstanceRef<DataElement>);
    }

    pub(super) async fn get_data_type(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "type", "I").await
    }

    pub(super) async fn get_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        jvm.get_field(&this, "longValue", "J").await
    }

    pub(super) async fn get_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::lang::Object>> {
        jvm.get_field(&this, "value", "Ljava/lang/Object;").await
    }
}
