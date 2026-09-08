use super::connector::connection_methods;
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
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

impl HttpConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/HttpConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/ContentConnection"],
            methods: {
                let mut methods = connection_methods();
                methods.insert(0, JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC));
                methods
            },
            fields: vec![
                JavaFieldProto::new("url", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("mode", "I", Default::default()),
                JavaFieldProto::new("method", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("responseCode", "I", Default::default()),
                JavaFieldProto::new("responseBody", "[B", Default::default()),
                JavaFieldProto::new("GET", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("POST", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HEAD", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_OK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_CREATED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_ACCEPTED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_NOT_AUTHORITATIVE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_NO_CONTENT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_RESET", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_PARTIAL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_MULT_CHOICE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_MOVED_PERM", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_MOVED_TEMP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_SEE_OTHER", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_NOT_MODIFIED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_USE_PROXY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_TEMP_REDIRECT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_BAD_REQUEST", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_UNAUTHORIZED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_PAYMENT_REQUIRED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_FORBIDDEN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_NOT_FOUND", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_BAD_METHOD", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_NOT_ACCEPTABLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_PROXY_AUTH", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_CLIENT_TIMEOUT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_CONFLICT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_GONE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_LENGTH_REQUIRED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_PRECON_FAILED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_ENTITY_TOO_LARGE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_REQ_TOO_LONG", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_UNSUPPORTED_TYPE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_INTERNAL_ERROR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_NOT_IMPLEMENTED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_BAD_GATEWAY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_UNAVAILABLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_GATEWAY_TIMEOUT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("HTTP_VERSION", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/io/HttpConnection";
        for (name, value) in [("GET", "GET"), ("POST", "POST"), ("HEAD", "HEAD")] {
            let s = JavaLangString::from_rust_string(jvm, value).await?;
            jvm.put_static_field(class, name, "Ljava/lang/String;", s).await?;
        }
        let codes = [
            ("HTTP_OK", 200),
            ("HTTP_CREATED", 201),
            ("HTTP_ACCEPTED", 202),
            ("HTTP_NOT_AUTHORITATIVE", 203),
            ("HTTP_NO_CONTENT", 204),
            ("HTTP_RESET", 205),
            ("HTTP_PARTIAL", 206),
            ("HTTP_MULT_CHOICE", 300),
            ("HTTP_MOVED_PERM", 301),
            ("HTTP_MOVED_TEMP", 302),
            ("HTTP_SEE_OTHER", 303),
            ("HTTP_NOT_MODIFIED", 304),
            ("HTTP_USE_PROXY", 305),
            ("HTTP_TEMP_REDIRECT", 307),
            ("HTTP_BAD_REQUEST", 400),
            ("HTTP_UNAUTHORIZED", 401),
            ("HTTP_PAYMENT_REQUIRED", 402),
            ("HTTP_FORBIDDEN", 403),
            ("HTTP_NOT_FOUND", 404),
            ("HTTP_BAD_METHOD", 405),
            ("HTTP_NOT_ACCEPTABLE", 406),
            ("HTTP_PROXY_AUTH", 407),
            ("HTTP_CLIENT_TIMEOUT", 408),
            ("HTTP_CONFLICT", 409),
            ("HTTP_GONE", 410),
            ("HTTP_LENGTH_REQUIRED", 411),
            ("HTTP_PRECON_FAILED", 412),
            ("HTTP_ENTITY_TOO_LARGE", 413),
            ("HTTP_REQ_TOO_LONG", 414),
            ("HTTP_UNSUPPORTED_TYPE", 415),
            ("HTTP_INTERNAL_ERROR", 500),
            ("HTTP_NOT_IMPLEMENTED", 501),
            ("HTTP_BAD_GATEWAY", 502),
            ("HTTP_UNAVAILABLE", 503),
            ("HTTP_GATEWAY_TIMEOUT", 504),
            ("HTTP_VERSION", 505),
        ];
        for (name, value) in codes {
            jvm.put_static_field(class, name, "I", value).await?;
        }
        Ok(())
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<String>,
        mode: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "url", "Ljava/lang/String;", url).await?;
        jvm.put_field(&mut this, "mode", "I", mode).await?;
        let method = JavaLangString::from_rust_string(jvm, "GET").await?;
        jvm.put_field(&mut this, "method", "Ljava/lang/String;", method).await?;
        jvm.put_field(&mut this, "responseCode", "I", 0).await
    }

    pub(super) async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    pub(super) async fn ensure_connected(jvm: &Jvm, context: &mut RuntimeContext, this: &ClassInstanceRef<Self>) -> Result<()> {
        let code: i32 = jvm.get_field(this, "responseCode", "I").await?;
        if code != 0 {
            return Ok(());
        }
        let url: ClassInstanceRef<String> = jvm.get_field(this, "url", "Ljava/lang/String;").await?;
        let url = JavaLangString::to_rust_string(jvm, &url).await?;
        let method: ClassInstanceRef<String> = jvm.get_field(this, "method", "Ljava/lang/String;").await?;
        let method = JavaLangString::to_rust_string(jvm, &method).await?;
        match context.http_request(&method, &url, &[], &[]).await {
            Ok((status, _headers, body)) => {
                let mut array = jvm.instantiate_array("B", body.len()).await?;
                let bytes: Vec<i8> = body.into_iter().map(|b| b as i8).collect();
                jvm.store_array(&mut array, 0, bytes).await?;
                jvm.put_field(&mut this.clone(), "responseBody", "[B", array).await?;
                jvm.put_field(&mut this.clone(), "responseCode", "I", status).await
            }
            Err(_) => {
                jvm.put_field(&mut this.clone(), "responseCode", "I", 503).await?;
                let array = jvm.instantiate_array("B", 0).await?;
                jvm.put_field(&mut this.clone(), "responseBody", "[B", array).await
            }
        }
    }

    pub(super) async fn open_input_stream(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        Self::ensure_connected(jvm, context, &this).await?;
        let body: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "responseBody", "[B").await?;
        Ok(jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (body,)).await?.into())
    }

    pub(super) async fn open_data_input_stream(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::io::DataInputStream>> {
        let stream = Self::open_input_stream(jvm, context, this).await?;
        Ok(jvm
            .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (stream,))
            .await?
            .into())
    }

    pub(super) async fn open_output_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<OutputStream>> {
        Ok(jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into())
    }

    pub(super) async fn open_data_output_stream(
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

    pub(super) async fn get_url(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "url", "Ljava/lang/String;").await
    }

    pub(super) async fn url_part(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<RustString> {
        let url: ClassInstanceRef<String> = jvm.get_field(this, "url", "Ljava/lang/String;").await?;
        JavaLangString::to_rust_string(jvm, &url).await
    }

    pub(super) async fn get_protocol(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let url = Self::url_part(jvm, &this).await?;
        let proto = url.split(':').next().unwrap_or("http");
        Ok(JavaLangString::from_rust_string(jvm, proto).await?.into())
    }

    pub(super) async fn get_host(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let url = Self::url_part(jvm, &this).await?;
        let host = url
            .split("://")
            .nth(1)
            .unwrap_or("")
            .split(['/', '?', '#'])
            .next()
            .unwrap_or("")
            .split(':')
            .next()
            .unwrap_or("");
        Ok(JavaLangString::from_rust_string(jvm, host).await?.into())
    }

    pub(super) async fn get_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let url = Self::url_part(jvm, &this).await?;
        let path = url.split("://").nth(1).and_then(|rest| rest.find('/').map(|i| &rest[i..])).unwrap_or("/");
        let path = path.split(['?', '#']).next().unwrap_or("/");
        Ok(JavaLangString::from_rust_string(jvm, path).await?.into())
    }

    pub(super) async fn get_ref(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let url = Self::url_part(jvm, &this).await?;
        let value = url.split('#').nth(1).unwrap_or("");
        Ok(JavaLangString::from_rust_string(jvm, value).await?.into())
    }

    pub(super) async fn get_query(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let url = Self::url_part(jvm, &this).await?;
        let value = url.split('?').nth(1).unwrap_or("").split('#').next().unwrap_or("");
        Ok(JavaLangString::from_rust_string(jvm, value).await?.into())
    }

    pub(super) async fn get_port(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let url = Self::url_part(jvm, &this).await?;
        let hostport = url.split("://").nth(1).unwrap_or("").split(['/', '?', '#']).next().unwrap_or("");
        Ok(hostport
            .split(':')
            .nth(1)
            .and_then(|p| p.parse().ok())
            .unwrap_or(if url.starts_with("https:") { 443 } else { 80 }))
    }

    pub(super) async fn get_request_method(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "method", "Ljava/lang/String;").await
    }

    pub(super) async fn set_request_method(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        method: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "method", "Ljava/lang/String;", method).await
    }

    pub(super) async fn get_request_property(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _key: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        Ok(ClassInstanceRef::new(None))
    }

    pub(super) async fn set_request_property(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _key: ClassInstanceRef<String>,
        _value: ClassInstanceRef<String>,
    ) -> Result<()> {
        Ok(())
    }

    pub(super) async fn get_response_code(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Self::ensure_connected(jvm, context, &this).await?;
        jvm.get_field(&this, "responseCode", "I").await
    }

    pub(super) async fn get_response_message(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<String>> {
        let code = Self::get_response_code(jvm, context, this).await?;
        Ok(JavaLangString::from_rust_string(jvm, &format!("{code}")).await?.into())
    }

    pub(super) async fn get_expiration(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(0)
    }
    pub(super) async fn get_date(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(0)
    }
    pub(super) async fn get_last_modified(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(0)
    }
    pub(super) async fn get_header_field(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        Ok(ClassInstanceRef::new(None))
    }
    pub(super) async fn get_header_field_index(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _n: i32,
    ) -> Result<ClassInstanceRef<String>> {
        Ok(ClassInstanceRef::new(None))
    }
    pub(super) async fn get_header_field_key(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _n: i32,
    ) -> Result<ClassInstanceRef<String>> {
        Ok(ClassInstanceRef::new(None))
    }
    pub(super) async fn get_header_field_int(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
        def: i32,
    ) -> Result<i32> {
        Ok(def)
    }
    pub(super) async fn get_header_field_date(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
        def: i64,
    ) -> Result<i64> {
        Ok(def)
    }
    pub(super) async fn get_type(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, "text/plain").await?.into())
    }
    pub(super) async fn get_encoding(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        Ok(ClassInstanceRef::new(None))
    }
    pub(super) async fn get_length(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        Self::ensure_connected(jvm, context, &this).await?;
        let body: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "responseBody", "[B").await?;
        if body.is_null() {
            return Ok(-1);
        }
        Ok(jvm.array_length(&body).await? as i64)
    }
}
