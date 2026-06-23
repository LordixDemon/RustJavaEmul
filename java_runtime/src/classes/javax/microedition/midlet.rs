use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// abstract class javax.microedition.midlet.MIDlet
pub struct MIDlet;

impl MIDlet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/midlet/MIDlet",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("startApp", "()V", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("pauseApp", "()V", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("destroyApp", "(Z)V", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new("notifyDestroyed", "()V", Self::notify_destroyed, Default::default()),
                JavaMethodProto::new("notifyPaused", "()V", Self::notify_paused, Default::default()),
                JavaMethodProto::new(
                    "getAppProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_app_property,
                    Default::default(),
                ),
                JavaMethodProto::new("platformRequest", "(Ljava/lang/String;)Z", Self::platform_request, Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.midlet.MIDlet::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn notify_destroyed(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.midlet.MIDlet::notifyDestroyed({this:?})");

        Ok(())
    }

    async fn notify_paused(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.midlet.MIDlet::notifyPaused({this:?})");

        Ok(())
    }

    async fn get_app_property(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("javax.microedition.midlet.MIDlet::getAppProperty({this:?}, {key:?})");

        if key.is_null() {
            return Ok(None.into());
        }

        let value = jvm
            .invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;

        Ok(value)
    }

    async fn platform_request(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<String>) -> Result<bool> {
        let url = if url.is_null() {
            "null".into()
        } else {
            JavaLangString::to_rust_string(jvm, &url).await?
        };
        tracing::warn!("stub javax.microedition.midlet.MIDlet::platformRequest({this:?}, {url:?})");

        Ok(false)
    }
}
