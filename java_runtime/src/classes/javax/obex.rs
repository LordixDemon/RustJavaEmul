use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{InputStream, OutputStream},
        lang::String,
    },
};

pub struct ClientSession;
pub struct HeaderSet;
pub struct Operation;

impl HeaderSet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/obex/HeaderSet",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getHeader", "(I)Ljava/lang/Object;", Self::get_header, Default::default()),
                JavaMethodProto::new("getHeaderList", "()[I", Self::get_header_list, Default::default()),
                JavaMethodProto::new("getResponseCode", "()I", Self::get_response_code, Default::default()),
                JavaMethodProto::new("setHeader", "(ILjava/lang/Object;)V", Self::set_header, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("NAME", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LENGTH", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TIME_ISO_8601", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TIME_4_BYTE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DESCRIPTION", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TARGET", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("WHO", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("OBJECT_CLASS", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("APPLICATION_PARAMETER", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/obex/HeaderSet";
        jvm.put_static_field(class, "NAME", "I", 0x01).await?;
        jvm.put_static_field(class, "TYPE", "I", 0x42).await?;
        jvm.put_static_field(class, "LENGTH", "I", 0xc3).await?;
        jvm.put_static_field(class, "TIME_ISO_8601", "I", 0x44).await?;
        jvm.put_static_field(class, "TIME_4_BYTE", "I", 0xc4).await?;
        jvm.put_static_field(class, "DESCRIPTION", "I", 0x05).await?;
        jvm.put_static_field(class, "TARGET", "I", 0x46).await?;
        jvm.put_static_field(class, "HTTP", "I", 0x47).await?;
        jvm.put_static_field(class, "WHO", "I", 0x4a).await?;
        jvm.put_static_field(class, "OBJECT_CLASS", "I", 0x4f).await?;
        jvm.put_static_field(class, "APPLICATION_PARAMETER", "I", 0x4c).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn get_header(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _id: i32,
    ) -> Result<ClassInstanceRef<crate::classes::java::lang::Object>> {
        Ok(None.into())
    }

    async fn get_header_list(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<jvm::Array<i32>>> {
        Ok(jvm.instantiate_array("I", 0).await?.into())
    }

    async fn get_response_code(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0xa0)
    }

    stub_void! {
        set_header(_id: i32, _value: ClassInstanceRef<crate::classes::java::lang::Object>);
    }
}

impl Operation {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/obex/Operation",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/ContentConnection"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("abort", "()V", Self::abort, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new(
                    "getReceivedHeaders",
                    "()Ljavax/obex/HeaderSet;",
                    Self::get_received_headers,
                    Default::default(),
                ),
                JavaMethodProto::new("getResponseCode", "()I", Self::get_response_code, Default::default()),
                JavaMethodProto::new("getType", "()Ljava/lang/String;", Self::get_type, Default::default()),
                JavaMethodProto::new("getEncoding", "()Ljava/lang/String;", Self::get_encoding, Default::default()),
                JavaMethodProto::new("getLength", "()J", Self::get_length, Default::default()),
                JavaMethodProto::new("openInputStream", "()Ljava/io/InputStream;", Self::open_input_stream, Default::default()),
                JavaMethodProto::new(
                    "openOutputStream",
                    "()Ljava/io/OutputStream;",
                    Self::open_output_stream,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "openDataOutputStream",
                    "()Ljava/io/DataOutputStream;",
                    Self::open_data_output_stream,
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

    stub_void! {
        abort();
        close();
    }

    async fn get_received_headers(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<HeaderSet>> {
        Ok(jvm.new_class("javax/obex/HeaderSet", "()V", ()).await?.into())
    }

    async fn get_response_code(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0xa0)
    }

    async fn get_type(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        Ok(None.into())
    }

    async fn get_encoding(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        Ok(None.into())
    }

    async fn get_length(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(-1)
    }

    async fn open_input_stream(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        let empty = jvm.instantiate_array("B", 0).await?;
        Ok(jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (empty,)).await?.into())
    }

    async fn open_output_stream(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<OutputStream>> {
        Ok(jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into())
    }

    async fn open_data_output_stream(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::io::DataOutputStream>> {
        let stream = Self::open_output_stream(jvm, context, this).await?;
        Ok(jvm
            .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream,))
            .await?
            .into())
    }
}

impl ClientSession {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/obex/ClientSession",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/Connection"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init_url, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new(
                    "connect",
                    "(Ljavax/obex/HeaderSet;)Ljavax/obex/HeaderSet;",
                    Self::connect,
                    Default::default(),
                ),
                JavaMethodProto::new("createHeaderSet", "()Ljavax/obex/HeaderSet;", Self::create_header_set, Default::default()),
                JavaMethodProto::new(
                    "disconnect",
                    "(Ljavax/obex/HeaderSet;)Ljavax/obex/HeaderSet;",
                    Self::disconnect,
                    Default::default(),
                ),
                JavaMethodProto::new("put", "(Ljavax/obex/HeaderSet;)Ljavax/obex/Operation;", Self::put, Default::default()),
                JavaMethodProto::new(
                    "setAuthenticator",
                    "(Ljavax/obex/Authenticator;)V",
                    Self::set_authenticator,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setPath",
                    "(Ljavax/obex/HeaderSet;ZZ)Ljavax/obex/HeaderSet;",
                    Self::set_path,
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

    async fn init_url(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _url: ClassInstanceRef<String>,
        _mode: i32,
    ) -> Result<()> {
        Self::init(jvm, context, this).await
    }

    stub_void! {
        close();
        set_authenticator(_auth: ClassInstanceRef<crate::classes::java::lang::Object>);
    }

    async fn create_header_set(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<HeaderSet>> {
        Ok(jvm.new_class("javax/obex/HeaderSet", "()V", ()).await?.into())
    }

    async fn connect(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _headers: ClassInstanceRef<HeaderSet>,
    ) -> Result<ClassInstanceRef<HeaderSet>> {
        Self::create_header_set(jvm, context, this).await
    }

    async fn disconnect(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _headers: ClassInstanceRef<HeaderSet>,
    ) -> Result<ClassInstanceRef<HeaderSet>> {
        Self::create_header_set(jvm, context, this).await
    }

    async fn put(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _headers: ClassInstanceRef<HeaderSet>,
    ) -> Result<ClassInstanceRef<Operation>> {
        Ok(jvm.new_class("javax/obex/Operation", "()V", ()).await?.into())
    }

    async fn set_path(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _headers: ClassInstanceRef<HeaderSet>,
        _backup: bool,
        _create: bool,
    ) -> Result<ClassInstanceRef<HeaderSet>> {
        Self::create_header_set(jvm, context, this).await
    }
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![ClientSession, HeaderSet, Operation]
}
