use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;
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
                JavaMethodProto::new("startApp", "()V", Self::start_app, Default::default()),
                JavaMethodProto::new("pauseApp", "()V", Self::pause_app, Default::default()),
                JavaMethodProto::new("destroyApp", "(Z)V", Self::destroy_app, Default::default()),
                JavaMethodProto::new("notifyDestroyed", "()V", Self::notify_destroyed, Default::default()),
                JavaMethodProto::new("notifyPaused", "()V", Self::notify_paused, Default::default()),
                JavaMethodProto::new(
                    "getAppProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_app_property,
                    Default::default(),
                ),
                JavaMethodProto::new("platformRequest", "(Ljava/lang/String;)Z", Self::platform_request, Default::default()),
                JavaMethodProto::new("checkPermission", "(Ljava/lang/String;)I", Self::check_permission, Default::default()),
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

    stub_void! {
        start_app();
        pause_app();
        destroy_app(_unconditional: bool);
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

    async fn platform_request(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<String>) -> Result<bool> {
        let url = if url.is_null() {
            "null".into()
        } else {
            JavaLangString::to_rust_string(jvm, &url).await?
        };
        tracing::debug!("javax.microedition.midlet.MIDlet::platformRequest({this:?}, {url:?})");
        Ok(context.platform_request(&url))
    }

    async fn check_permission(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _permission: ClassInstanceRef<String>) -> Result<i32> {
        Ok(1)
    }
}

simple_exception!(
    MIDletStateChangeException,
    "javax/microedition/midlet/MIDletStateChangeException",
    "java/lang/Exception",
    "javax.microedition.midlet.MIDletStateChangeException"
);

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![MIDlet, MIDletStateChangeException]
}
