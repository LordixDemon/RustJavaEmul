#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{InputStream, OutputStream},
        lang::String,
    },
};
#[allow(unused_imports)]
use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::FieldAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

macro_rules! stream_conn {
    ($ty:ident, $name:literal) => {
        impl $ty {
            pub fn as_proto() -> RuntimeClassProto {
                RuntimeClassProto {
                    name: $name,
                    parent_class: Some("java/lang/Object"),
                    interfaces: vec!["javax/microedition/io/StreamConnection"],
                    methods: vec![
                        JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, Default::default()),
                        JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                        JavaMethodProto::new("openInputStream", "()Ljava/io/InputStream;", Self::open_input, Default::default()),
                        JavaMethodProto::new(
                            "openOutputStream",
                            "()Ljava/io/OutputStream;",
                            Self::open_output,
                            Default::default(),
                        ),
                        JavaMethodProto::new(
                            "openDataInputStream",
                            "()Ljava/io/DataInputStream;",
                            Self::open_data_in,
                            Default::default(),
                        ),
                        JavaMethodProto::new(
                            "openDataOutputStream",
                            "()Ljava/io/DataOutputStream;",
                            Self::open_data_out,
                            Default::default(),
                        ),
                        JavaMethodProto::new("getLocalAddress", "()Ljava/lang/String;", Self::local_address, Default::default()),
                        JavaMethodProto::new("getLocalPort", "()I", Self::local_port, Default::default()),
                        JavaMethodProto::new("getAddress", "()Ljava/lang/String;", Self::address, Default::default()),
                        JavaMethodProto::new("getPort", "()I", Self::port, Default::default()),
                        JavaMethodProto::new("setSocketOption", "(BI)V", Self::set_option, Default::default()),
                        JavaMethodProto::new("getSocketOption", "(B)I", Self::get_option, Default::default()),
                    ],
                    fields: vec![
                        JavaFieldProto::new("url", "Ljava/lang/String;", Default::default()),
                        JavaFieldProto::new("DELAY", "B", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                        JavaFieldProto::new("LINGER", "B", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                        JavaFieldProto::new("KEEPALIVE", "B", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                        JavaFieldProto::new("RCVBUF", "B", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                        JavaFieldProto::new("SNDBUF", "B", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                    ],
                    access_flags: Default::default(),
                }
            }

            async fn init(
                jvm: &Jvm,
                _: &mut RuntimeContext,
                mut this: ClassInstanceRef<Self>,
                url: ClassInstanceRef<String>,
                _mode: i32,
            ) -> Result<()> {
                let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
                jvm.put_field(&mut this, "url", "Ljava/lang/String;", url).await
            }
            async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
                Ok(())
            }
            async fn open_input(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
                let empty = jvm.instantiate_array("B", 0).await?;
                Ok(jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (empty,)).await?.into())
            }
            async fn open_output(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<OutputStream>> {
                Ok(jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into())
            }
            async fn open_data_in(
                jvm: &Jvm,
                context: &mut RuntimeContext,
                this: ClassInstanceRef<Self>,
            ) -> Result<ClassInstanceRef<crate::classes::java::io::DataInputStream>> {
                let stream = Self::open_input(jvm, context, this).await?;
                Ok(jvm
                    .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (stream,))
                    .await?
                    .into())
            }
            async fn open_data_out(
                jvm: &Jvm,
                context: &mut RuntimeContext,
                this: ClassInstanceRef<Self>,
            ) -> Result<ClassInstanceRef<crate::classes::java::io::DataOutputStream>> {
                let stream = Self::open_output(jvm, context, this).await?;
                Ok(jvm
                    .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream,))
                    .await?
                    .into())
            }
            async fn local_address(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
                Ok(JavaLangString::from_rust_string(jvm, "127.0.0.1").await?.into())
            }
            async fn local_port(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
                Ok(0)
            }
            async fn address(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
                jvm.get_field(&this, "url", "Ljava/lang/String;").await
            }
            async fn port(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
                Ok(0)
            }
            async fn set_option(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _opt: i8, _value: i32) -> Result<()> {
                Ok(())
            }
            async fn get_option(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _opt: i8) -> Result<i32> {
                Ok(-1)
            }
        }
    };
}

stream_conn!(SocketConnection, "javax/microedition/io/SocketConnection");
stream_conn!(SecureConnection, "javax/microedition/io/SecureConnection");

impl HttpsConnection {
    pub fn as_proto() -> RuntimeClassProto {
        let mut proto = HttpConnection::as_proto();
        proto.name = "javax/microedition/io/HttpsConnection";
        proto.interfaces = vec!["javax/microedition/io/HttpConnection"];
        proto
            .methods
            .push(JavaMethodProto::new("getPort", "()I", HttpConnection::get_port, Default::default()));
        proto.methods.push(JavaMethodProto::new(
            "getSecurityInfo",
            "()Ljavax/microedition/io/SecurityInfo;",
            Self::get_security_info,
            Default::default(),
        ));
        proto
    }

    pub(super) async fn get_security_info(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Connection>> {
        Ok(ClassInstanceRef::new(None))
    }
}

impl ServerSocketConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/ServerSocketConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/StreamConnectionNotifier"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new(
                    "acceptAndOpen",
                    "()Ljavax/microedition/io/StreamConnection;",
                    Self::accept,
                    Default::default(),
                ),
                JavaMethodProto::new("getLocalAddress", "()Ljava/lang/String;", Self::local_address, Default::default()),
                JavaMethodProto::new("getLocalPort", "()I", Self::local_port, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("url", "Ljava/lang/String;", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<String>,
        _mode: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "url", "Ljava/lang/String;", url).await
    }
    pub(super) async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
    pub(super) async fn accept(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<SocketConnection>> {
        let url: ClassInstanceRef<String> = jvm.get_field(&this, "url", "Ljava/lang/String;").await?;
        Ok(jvm
            .new_class("javax/microedition/io/SocketConnection", "(Ljava/lang/String;I)V", (url, 3))
            .await?
            .into())
    }
    pub(super) async fn local_address(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, "0.0.0.0").await?.into())
    }
    pub(super) async fn local_port(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }
}

impl UDPDatagramConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/UDPDatagramConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/DatagramConnection"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("getMaximumLength", "()I", Self::max_len, Default::default()),
                JavaMethodProto::new("getNominalLength", "()I", Self::nom_len, Default::default()),
                JavaMethodProto::new("send", "(Ljavax/microedition/io/Datagram;)V", Self::send, Default::default()),
                JavaMethodProto::new("receive", "(Ljavax/microedition/io/Datagram;)V", Self::receive, Default::default()),
                JavaMethodProto::new(
                    "newDatagram",
                    "(I)Ljavax/microedition/io/Datagram;",
                    Self::new_datagram,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "newDatagram",
                    "(ILjava/lang/String;)Ljavax/microedition/io/Datagram;",
                    Self::new_datagram_addr,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "newDatagram",
                    "([BI)Ljavax/microedition/io/Datagram;",
                    Self::new_datagram_buf,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "newDatagram",
                    "([BILjava/lang/String;)Ljavax/microedition/io/Datagram;",
                    Self::new_datagram_buf_addr,
                    Default::default(),
                ),
                JavaMethodProto::new("getLocalAddress", "()Ljava/lang/String;", Self::local_address, Default::default()),
                JavaMethodProto::new("getLocalPort", "()I", Self::local_port, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("url", "Ljava/lang/String;", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<String>,
        _mode: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "url", "Ljava/lang/String;", url).await
    }
    pub(super) async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
    pub(super) async fn max_len(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1500)
    }
    pub(super) async fn nom_len(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1500)
    }
    pub(super) async fn send(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _d: ClassInstanceRef<Datagram>) -> Result<()> {
        Ok(())
    }
    pub(super) async fn receive(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _d: ClassInstanceRef<Datagram>) -> Result<()> {
        Ok(())
    }
    pub(super) async fn new_datagram(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        size: i32,
    ) -> Result<ClassInstanceRef<Datagram>> {
        let buf = jvm.instantiate_array("B", size.max(0) as usize).await?;
        Ok(jvm.new_class("javax/microedition/io/Datagram", "([BI)V", (buf, size)).await?.into())
    }
    pub(super) async fn new_datagram_addr(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        size: i32,
        _addr: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Datagram>> {
        Self::new_datagram(jvm, context, this, size).await
    }
    pub(super) async fn new_datagram_buf(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<i8>>,
        size: i32,
    ) -> Result<ClassInstanceRef<Datagram>> {
        Ok(jvm.new_class("javax/microedition/io/Datagram", "([BI)V", (buf, size)).await?.into())
    }
    pub(super) async fn new_datagram_buf_addr(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<i8>>,
        size: i32,
        _addr: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Datagram>> {
        Self::new_datagram_buf(jvm, context, this, buf, size).await
    }
    pub(super) async fn local_address(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, "0.0.0.0").await?.into())
    }
    pub(super) async fn local_port(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }
}

impl CommConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/CommConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/StreamConnection"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("getBaudRate", "()I", Self::get_baud, Default::default()),
                JavaMethodProto::new("setBaudRate", "(I)I", Self::set_baud, Default::default()),
                JavaMethodProto::new("openInputStream", "()Ljava/io/InputStream;", Self::open_input, Default::default()),
                JavaMethodProto::new("openOutputStream", "()Ljava/io/OutputStream;", Self::open_output, Default::default()),
                JavaMethodProto::new(
                    "openDataInputStream",
                    "()Ljava/io/DataInputStream;",
                    Self::open_data_in,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "openDataOutputStream",
                    "()Ljava/io/DataOutputStream;",
                    Self::open_data_out,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("url", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("baud", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<String>,
        _mode: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "url", "Ljava/lang/String;", url).await?;
        jvm.put_field(&mut this, "baud", "I", 9600).await
    }
    pub(super) async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
    pub(super) async fn get_baud(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "baud", "I").await
    }
    pub(super) async fn set_baud(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, baud: i32) -> Result<i32> {
        jvm.put_field(&mut this, "baud", "I", baud).await?;
        Ok(baud)
    }
    pub(super) async fn open_input(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        let empty = jvm.instantiate_array("B", 0).await?;
        Ok(jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (empty,)).await?.into())
    }
    pub(super) async fn open_output(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<OutputStream>> {
        Ok(jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into())
    }
    pub(super) async fn open_data_in(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::io::DataInputStream>> {
        let stream = Self::open_input(jvm, context, this).await?;
        Ok(jvm
            .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (stream,))
            .await?
            .into())
    }
    pub(super) async fn open_data_out(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::io::DataOutputStream>> {
        let stream = Self::open_output(jvm, context, this).await?;
        Ok(jvm
            .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream,))
            .await?
            .into())
    }
}
