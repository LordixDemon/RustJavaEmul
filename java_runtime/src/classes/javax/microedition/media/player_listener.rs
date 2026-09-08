use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext};

// interface javax.microedition.media.PlayerListener
pub struct PlayerListener;

impl PlayerListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/PlayerListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new_abstract(
                    "playerUpdate",
                    "(Ljavax/microedition/media/Player;Ljava/lang/String;Ljava/lang/Object;)V",
                    MethodAccessFlags::ABSTRACT,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("STARTED", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("END_OF_MEDIA", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STOPPED", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "STOPPED_AT_TIME",
                    "Ljava/lang/String;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("CLOSED", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ERROR", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "DEVICE_AVAILABLE",
                    "Ljava/lang/String;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "DEVICE_UNAVAILABLE",
                    "Ljava/lang/String;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("VOLUME_CHANGED", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "DURATION_UPDATED",
                    "Ljava/lang/String;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/media/PlayerListener";
        for (name, value) in [
            ("STARTED", "started"),
            ("END_OF_MEDIA", "endOfMedia"),
            ("STOPPED", "stopped"),
            ("STOPPED_AT_TIME", "stoppedAtTime"),
            ("CLOSED", "closed"),
            ("ERROR", "error"),
            ("DEVICE_AVAILABLE", "deviceAvailable"),
            ("DEVICE_UNAVAILABLE", "deviceUnavailable"),
            ("VOLUME_CHANGED", "volumeChanged"),
            ("DURATION_UPDATED", "durationUpdated"),
        ] {
            let interned = JavaLangString::intern_rust_string(jvm, value).await?;
            jvm.put_static_field(class, name, "Ljava/lang/String;", interned).await?;
        }
        Ok(())
    }
}
