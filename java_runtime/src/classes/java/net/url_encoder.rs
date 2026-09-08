use alloc::{format, string::String as RustString, vec};

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

pub struct URLEncoder;

impl URLEncoder {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/net/URLEncoder",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "encode",
                "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                Self::encode,
                MethodAccessFlags::STATIC,
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn encode(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        value: ClassInstanceRef<String>,
        _encoding: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "s").await);
        }
        let rust = JavaLangString::to_rust_string(jvm, &value).await?;
        JavaLangString::from_rust_string(jvm, &percent_encode(&rust)).await.map(Into::into)
    }
}

fn percent_encode(input: &str) -> RustString {
    let mut out = RustString::new();
    for byte in input.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'*' => out.push(*byte as char),
            b' ' => out.push('+'),
            b => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
