#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl Datagram {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/Datagram",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([BI)V", Self::init, Default::default()),
                JavaMethodProto::new("getAddress", "()Ljava/lang/String;", Self::get_address, Default::default()),
                JavaMethodProto::new("getData", "()[B", Self::get_data, Default::default()),
                JavaMethodProto::new("getLength", "()I", Self::get_length, Default::default()),
                JavaMethodProto::new("getOffset", "()I", Self::get_offset, Default::default()),
                JavaMethodProto::new("reset", "()V", Self::reset, Default::default()),
                JavaMethodProto::new("setAddress", "(Ljava/lang/String;)V", Self::set_address, Default::default()),
                JavaMethodProto::new(
                    "setAddress",
                    "(Ljavax/microedition/io/Datagram;)V",
                    Self::set_address_dgram,
                    Default::default(),
                ),
                JavaMethodProto::new("setData", "([BII)V", Self::set_data, Default::default()),
                JavaMethodProto::new("setLength", "(I)V", Self::set_length, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("data", "[B", Default::default()),
                JavaFieldProto::new("length", "I", Default::default()),
                JavaFieldProto::new("offset", "I", Default::default()),
                JavaFieldProto::new("address", "Ljava/lang/String;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
        length: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "data", "[B", data).await?;
        jvm.put_field(&mut this, "length", "I", length).await?;
        jvm.put_field(&mut this, "offset", "I", 0).await
    }
    pub(super) async fn get_address(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "address", "Ljava/lang/String;").await
    }
    pub(super) async fn get_data(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        jvm.get_field(&this, "data", "[B").await
    }
    pub(super) async fn get_length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "length", "I").await
    }
    pub(super) async fn get_offset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "offset", "I").await
    }
    pub(super) async fn reset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "offset", "I", 0).await?;
        jvm.put_field(&mut this, "length", "I", 0).await
    }
    pub(super) async fn set_address(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        addr: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "address", "Ljava/lang/String;", addr).await
    }
    pub(super) async fn set_address_dgram(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<()> {
        let addr: ClassInstanceRef<String> = jvm.get_field(&other, "address", "Ljava/lang/String;").await?;
        Self::set_address(jvm, context, this, addr).await
    }
    pub(super) async fn set_data(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "data", "[B", data).await?;
        jvm.put_field(&mut this, "offset", "I", offset).await?;
        jvm.put_field(&mut this, "length", "I", length).await
    }
    pub(super) async fn set_length(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, length: i32) -> Result<()> {
        jvm.put_field(&mut this, "length", "I", length).await
    }
}
