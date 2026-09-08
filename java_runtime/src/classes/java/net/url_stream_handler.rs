use alloc::{borrow::ToOwned, string::String as RustString, string::ToString, vec, vec::Vec};

use java_constants::ClassAccessFlags;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{lang::String, net::URL},
};

struct ParsedUrl<'a> {
    scheme: &'a str,
    host: Option<&'a str>,
    port: Option<i32>,
    path: &'a str,
    query: Option<&'a str>,
    r#ref: Option<&'a str>,
}

fn parse_url_spec(input: &str) -> core::result::Result<ParsedUrl<'_>, &'static str> {
    let (scheme, rest) = input.split_once(':').ok_or("no scheme")?;
    if scheme.is_empty() {
        return Err("empty scheme");
    }

    let (before_ref, r#ref) = match rest.split_once('#') {
        Some((r, f)) => (r, Some(f)),
        None => (rest, None),
    };

    let (host, port, path_and_query) = if let Some(after_slashes) = before_ref.strip_prefix("//") {
        let (authority, remainder) = match after_slashes.find('/') {
            Some(idx) => (&after_slashes[..idx], &after_slashes[idx..]),
            None => (after_slashes, ""),
        };
        let (host_str, port_val) = if let Some((h, p)) = authority.rsplit_once(':') {
            if let Ok(p_num) = p.parse::<i32>() {
                (h, Some(p_num))
            } else {
                (authority, None)
            }
        } else {
            (authority, None)
        };
        (Some(host_str), port_val, remainder)
    } else {
        (None, None, before_ref)
    };

    let (path, query) = match path_and_query.split_once('?') {
        Some((p, q)) => (p, Some(q)),
        None => (path_and_query, None),
    };

    Ok(ParsedUrl {
        scheme,
        host,
        port,
        path,
        query,
        r#ref,
    })
}

// abstract class java.net.URLStreamHandler
pub struct URLStreamHandler;

impl URLStreamHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/net/URLStreamHandler",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("openConnection", "(Ljava/net/URL;)Ljava/net/URLConnection;", Default::default()),
                JavaMethodProto::new("parseURL", "(Ljava/net/URL;Ljava/lang/String;II)V", Self::parse_url, Default::default()),
                JavaMethodProto::new(
                    "setURL",
                    "(Ljava/net/URL;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
                    Self::set_url,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.net.URLStreamHandler::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn set_url(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
        protocol: ClassInstanceRef<String>,
        host: ClassInstanceRef<String>,
        port: i32,
        file: ClassInstanceRef<String>,
        r#ref: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!(
            "java.net.URLStreamHandler::setURL({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            &this,
            &url,
            &protocol,
            &host,
            &port,
            &file,
            &r#ref,
        );

        let _: () = jvm
            .invoke_virtual(
                &url,
                "set",
                "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
                (protocol, host, port, file, r#ref),
            )
            .await?;

        Ok(())
    }

    async fn parse_url(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
        spec: ClassInstanceRef<String>,
        start: i32,
        limit: i32,
    ) -> Result<()> {
        tracing::debug!(
            "java.net.URLStreamHandler::parseURL({:?}, {:?}, {:?}, {:?}, {:?})",
            &this,
            &url,
            &spec,
            &start,
            &limit
        );

        let spec_str = JavaLangString::to_rust_string(jvm, &spec).await?;

        let parsed = match parse_url_spec(&spec_str) {
            Ok(p) => p,
            Err(e) => return Err(jvm.exception("java/net/MalformedURLException", e).await),
        };

        let protocol = parsed.scheme;
        let path = parsed.path.to_owned() + &parsed.query.map(|x| "?".to_owned() + x).unwrap_or_default();
        let file = if protocol == "file" { normalize_file_url_path(&path) } else { path };

        let protocol = JavaLangString::from_rust_string(jvm, parsed.scheme).await?;
        let host = JavaLangString::from_rust_string(jvm, parsed.host.unwrap_or("")).await?;
        let port = parsed.port.unwrap_or(-1);
        let file = JavaLangString::from_rust_string(jvm, &file).await?;
        let r#ref = match parsed.r#ref {
            Some(r) => Some(JavaLangString::from_rust_string(jvm, r).await?),
            None => None,
        };

        let _: () = jvm
            .invoke_virtual(
                &this,
                "setURL",
                "(Ljava/net/URL;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
                (url, protocol, host, port, file, r#ref),
            )
            .await?;

        Ok(())
    }
}

fn normalize_file_url_path(path: &str) -> RustString {
    percent_decode(path).replace('\\', "/").trim_start_matches('/').to_string()
}

fn percent_decode(value: &str) -> RustString {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(value.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let (Some(high), Some(low)) = (hex_value(bytes[index + 1]), hex_value(bytes[index + 2])) {
                out.push(high << 4 | low);
                index += 3;
                continue;
            }
        }
        out.push(bytes[index]);
        index += 1;
    }
    RustString::from_utf8_lossy(&out).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
