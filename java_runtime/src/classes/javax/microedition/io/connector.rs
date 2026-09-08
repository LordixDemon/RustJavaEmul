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
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

simple_exception!(
    ConnectionNotFoundException,
    "javax/microedition/io/ConnectionNotFoundException",
    "java/io/IOException",
    "javax.microedition.io.ConnectionNotFoundException"
);

impl Connector {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/Connector",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "open",
                    "(Ljava/lang/String;)Ljavax/microedition/io/Connection;",
                    Self::open,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "open",
                    "(Ljava/lang/String;I)Ljavax/microedition/io/Connection;",
                    Self::open_mode,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "open",
                    "(Ljava/lang/String;IZ)Ljavax/microedition/io/Connection;",
                    Self::open_mode_timeouts,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "openInputStream",
                    "(Ljava/lang/String;)Ljava/io/InputStream;",
                    Self::open_input_stream,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "openOutputStream",
                    "(Ljava/lang/String;)Ljava/io/OutputStream;",
                    Self::open_output_stream,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "openDataInputStream",
                    "(Ljava/lang/String;)Ljava/io/DataInputStream;",
                    Self::open_data_input_stream,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "openDataOutputStream",
                    "(Ljava/lang/String;)Ljava/io/DataOutputStream;",
                    Self::open_data_output_stream,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("READ", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("WRITE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("READ_WRITE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("javax/microedition/io/Connector", "READ", "I", 1).await?;
        jvm.put_static_field("javax/microedition/io/Connector", "WRITE", "I", 2).await?;
        jvm.put_static_field("javax/microedition/io/Connector", "READ_WRITE", "I", 3).await
    }

    pub(super) async fn open(jvm: &Jvm, context: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Connection>> {
        Self::open_mode_timeouts(jvm, context, name, 3, false).await
    }

    pub(super) async fn open_mode(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
        mode: i32,
    ) -> Result<ClassInstanceRef<Connection>> {
        Self::open_mode_timeouts(jvm, context, name, mode, false).await
    }

    pub(super) async fn open_mode_timeouts(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
        mode: i32,
        _timeouts: bool,
    ) -> Result<ClassInstanceRef<Connection>> {
        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }
        let url = JavaLangString::to_rust_string(jvm, &name).await?;
        tracing::debug!("javax.microedition.io.Connector::open({url:?}, {mode})");
        let class = if url.starts_with("https:") {
            "javax/microedition/io/HttpsConnection"
        } else if url.starts_with("http:") {
            "javax/microedition/io/HttpConnection"
        } else if url.starts_with("socket:") {
            "javax/microedition/io/SocketConnection"
        } else if url.starts_with("serversocket:") {
            "javax/microedition/io/ServerSocketConnection"
        } else if url.starts_with("datagram:") {
            "javax/microedition/io/UDPDatagramConnection"
        } else if url.starts_with("comm:") {
            "javax/microedition/io/CommConnection"
        } else if url.starts_with("ssl:") {
            "javax/microedition/io/SecureConnection"
        } else if url.starts_with("file:") {
            "javax/microedition/io/file/FileConnection"
        } else if url.starts_with("sms:") || url.starts_with("mms:") {
            "javax/wireless/messaging/MessageConnection"
        } else if url.starts_with("btl2cap:") && url.contains(";server=") {
            "javax/bluetooth/L2CAPConnectionNotifier"
        } else if url.starts_with("btl2cap:") {
            "javax/bluetooth/L2CAPConnection"
        } else if url.starts_with("btgoep:") {
            "javax/obex/ClientSession"
        } else if url.starts_with("btspp:") {
            "javax/microedition/io/SocketConnection"
        } else {
            return Err(jvm.exception("javax/microedition/io/ConnectionNotFoundException", &url).await);
        };
        Ok(jvm.new_class(class, "(Ljava/lang/String;I)V", (name, mode)).await?.into())
    }

    pub(super) async fn open_input_stream(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        let conn = Self::open(jvm, context, name).await?;
        jvm.invoke_virtual(&conn, "openInputStream", "()Ljava/io/InputStream;", ()).await
    }

    pub(super) async fn open_output_stream(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<OutputStream>> {
        let conn = Self::open(jvm, context, name).await?;
        jvm.invoke_virtual(&conn, "openOutputStream", "()Ljava/io/OutputStream;", ()).await
    }

    pub(super) async fn open_data_input_stream(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<crate::classes::java::io::DataInputStream>> {
        let stream = Self::open_input_stream(jvm, context, name).await?;
        Ok(jvm
            .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (stream,))
            .await?
            .into())
    }

    pub(super) async fn open_data_output_stream(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<crate::classes::java::io::DataOutputStream>> {
        let stream = Self::open_output_stream(jvm, context, name).await?;
        Ok(jvm
            .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream,))
            .await?
            .into())
    }
}

pub(super) fn connection_methods() -> Vec<JavaMethodProto<dyn crate::runtime::Runtime>> {
    vec![
        JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", HttpConnection::init, Default::default()),
        JavaMethodProto::new("close", "()V", HttpConnection::close, Default::default()),
        JavaMethodProto::new(
            "openInputStream",
            "()Ljava/io/InputStream;",
            HttpConnection::open_input_stream,
            Default::default(),
        ),
        JavaMethodProto::new(
            "openDataInputStream",
            "()Ljava/io/DataInputStream;",
            HttpConnection::open_data_input_stream,
            Default::default(),
        ),
        JavaMethodProto::new(
            "openOutputStream",
            "()Ljava/io/OutputStream;",
            HttpConnection::open_output_stream,
            Default::default(),
        ),
        JavaMethodProto::new(
            "openDataOutputStream",
            "()Ljava/io/DataOutputStream;",
            HttpConnection::open_data_output_stream,
            Default::default(),
        ),
        JavaMethodProto::new("getURL", "()Ljava/lang/String;", HttpConnection::get_url, Default::default()),
        JavaMethodProto::new("getProtocol", "()Ljava/lang/String;", HttpConnection::get_protocol, Default::default()),
        JavaMethodProto::new("getHost", "()Ljava/lang/String;", HttpConnection::get_host, Default::default()),
        JavaMethodProto::new("getFile", "()Ljava/lang/String;", HttpConnection::get_file, Default::default()),
        JavaMethodProto::new("getRef", "()Ljava/lang/String;", HttpConnection::get_ref, Default::default()),
        JavaMethodProto::new("getQuery", "()Ljava/lang/String;", HttpConnection::get_query, Default::default()),
        JavaMethodProto::new("getPort", "()I", HttpConnection::get_port, Default::default()),
        JavaMethodProto::new(
            "getRequestMethod",
            "()Ljava/lang/String;",
            HttpConnection::get_request_method,
            Default::default(),
        ),
        JavaMethodProto::new(
            "setRequestMethod",
            "(Ljava/lang/String;)V",
            HttpConnection::set_request_method,
            Default::default(),
        ),
        JavaMethodProto::new(
            "getRequestProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            HttpConnection::get_request_property,
            Default::default(),
        ),
        JavaMethodProto::new(
            "setRequestProperty",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            HttpConnection::set_request_property,
            Default::default(),
        ),
        JavaMethodProto::new("getResponseCode", "()I", HttpConnection::get_response_code, Default::default()),
        JavaMethodProto::new(
            "getResponseMessage",
            "()Ljava/lang/String;",
            HttpConnection::get_response_message,
            Default::default(),
        ),
        JavaMethodProto::new("getExpiration", "()J", HttpConnection::get_expiration, Default::default()),
        JavaMethodProto::new("getDate", "()J", HttpConnection::get_date, Default::default()),
        JavaMethodProto::new("getLastModified", "()J", HttpConnection::get_last_modified, Default::default()),
        JavaMethodProto::new(
            "getHeaderField",
            "(Ljava/lang/String;)Ljava/lang/String;",
            HttpConnection::get_header_field,
            Default::default(),
        ),
        JavaMethodProto::new(
            "getHeaderField",
            "(I)Ljava/lang/String;",
            HttpConnection::get_header_field_index,
            Default::default(),
        ),
        JavaMethodProto::new(
            "getHeaderFieldKey",
            "(I)Ljava/lang/String;",
            HttpConnection::get_header_field_key,
            Default::default(),
        ),
        JavaMethodProto::new(
            "getHeaderFieldInt",
            "(Ljava/lang/String;I)I",
            HttpConnection::get_header_field_int,
            Default::default(),
        ),
        JavaMethodProto::new(
            "getHeaderFieldDate",
            "(Ljava/lang/String;J)J",
            HttpConnection::get_header_field_date,
            Default::default(),
        ),
        JavaMethodProto::new("getType", "()Ljava/lang/String;", HttpConnection::get_type, Default::default()),
        JavaMethodProto::new("getEncoding", "()Ljava/lang/String;", HttpConnection::get_encoding, Default::default()),
        JavaMethodProto::new("getLength", "()J", HttpConnection::get_length, Default::default()),
    ]
}
