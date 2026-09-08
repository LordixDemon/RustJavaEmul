#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::javax::microedition::lcdui::Image as LcdImage};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl Image {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/ui/Image",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "createImageFromBitmap",
                "([BII)Ljavax/microedition/lcdui/Image;",
                Self::from_bitmap,
                MethodAccessFlags::STATIC,
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn from_bitmap(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _data: ClassInstanceRef<Array<i8>>,
        width: i32,
        height: i32,
    ) -> Result<ClassInstanceRef<LcdImage>> {
        jvm.invoke_static(
            "javax/microedition/lcdui/Image",
            "createImage",
            "(II)Ljavax/microedition/lcdui/Image;",
            (width.max(1), height.max(1)),
        )
        .await
    }
}
