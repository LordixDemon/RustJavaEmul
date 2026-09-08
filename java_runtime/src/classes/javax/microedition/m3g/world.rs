#[allow(unused_imports)]
use super::common::*;
#[allow(unused_imports)]
use super::math::*;
#[allow(unused_imports)]
use super::prelude::*;
#[allow(unused_imports)]
use super::raw_arrays::*;
#[allow(unused_imports)]
use super::render::*;
#[allow(unused_imports)]
use super::types::*;

impl World {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/World",
            parent_class: Some("javax/microedition/m3g/Group"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getActiveCamera",
                    "()Ljavax/microedition/m3g/Camera;",
                    Self::get_active_camera,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getBackground",
                    "()Ljavax/microedition/m3g/Background;",
                    Self::get_background,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setActiveCamera",
                    "(Ljavax/microedition/m3g/Camera;)V",
                    Self::set_active_camera,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setBackground",
                    "(Ljavax/microedition/m3g/Background;)V",
                    Self::set_background,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("activeCamera", "Ljavax/microedition/m3g/Camera;", Default::default()),
                JavaFieldProto::new("background", "Ljavax/microedition/m3g/Background;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Group", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "activeCamera", "Ljavax/microedition/m3g/Camera;", null_ref::<Camera>())
            .await?;
        jvm.put_field(&mut this, "background", "Ljavax/microedition/m3g/Background;", null_ref::<Background>())
            .await
    }

    pub(super) async fn get_active_camera(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Camera>> {
        jvm.get_field(&this, "activeCamera", "Ljavax/microedition/m3g/Camera;").await
    }

    pub(super) async fn get_background(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Background>> {
        jvm.get_field(&this, "background", "Ljavax/microedition/m3g/Background;").await
    }

    pub(super) async fn set_active_camera(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        camera: ClassInstanceRef<Camera>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "activeCamera", "Ljavax/microedition/m3g/Camera;", camera).await
    }

    pub(super) async fn set_background(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        background: ClassInstanceRef<Background>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "background", "Ljavax/microedition/m3g/Background;", background)
            .await
    }
}
