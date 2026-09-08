use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{lang::String, net::URL},
};

pub struct HttpURLConnection;

impl HttpURLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/net/HttpURLConnection",
            parent_class: Some("java/net/URLConnection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, Default::default()),
                JavaMethodProto::new("setRequestMethod", "(Ljava/lang/String;)V", Self::set_request_method, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("method", "Ljava/lang/String;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "java/net/URLConnection", "<init>", "(Ljava/net/URL;)V", (url,))
            .await?;
        jvm.put_field(&mut this, "method", "Ljava/lang/String;", ClassInstanceRef::<String>::new(None))
            .await
    }

    async fn set_request_method(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, method: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "method", "Ljava/lang/String;", method).await
    }
}
