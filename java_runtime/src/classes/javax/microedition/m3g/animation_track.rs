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

impl AnimationTrack {
    pub const ALPHA: i32 = 256;
    pub const AMBIENT_COLOR: i32 = 257;
    pub const COLOR: i32 = 258;
    pub const CROP: i32 = 259;
    pub const DENSITY: i32 = 260;
    pub const DIFFUSE_COLOR: i32 = 261;
    pub const EMISSIVE_COLOR: i32 = 262;
    pub const FAR_DISTANCE: i32 = 263;
    pub const FIELD_OF_VIEW: i32 = 264;
    pub const INTENSITY: i32 = 265;
    pub const MORPH_WEIGHTS: i32 = 266;
    pub const NEAR_DISTANCE: i32 = 267;
    pub const ORIENTATION: i32 = 268;
    pub const PICKABILITY: i32 = 269;
    pub const SCALE: i32 = 270;
    pub const SHININESS: i32 = 271;
    pub const SPECULAR_COLOR: i32 = 272;
    pub const SPOT_ANGLE: i32 = 273;
    pub const SPOT_EXPONENT: i32 = 274;
    pub const TRANSLATION: i32 = 275;
    pub const VISIBILITY: i32 = 276;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/AnimationTrack",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/m3g/KeyframeSequence;I)V",
                    Self::init_with_sequence,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getController",
                    "()Ljavax/microedition/m3g/AnimationController;",
                    Self::get_controller,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getKeyframeSequence",
                    "()Ljavax/microedition/m3g/KeyframeSequence;",
                    Self::get_keyframe_sequence,
                    Default::default(),
                ),
                JavaMethodProto::new("getTargetProperty", "()I", Self::get_target_property, Default::default()),
                JavaMethodProto::new(
                    "setController",
                    "(Ljavax/microedition/m3g/AnimationController;)V",
                    Self::set_controller,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("ALPHA", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("AMBIENT_COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("CROP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DENSITY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DIFFUSE_COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("EMISSIVE_COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FAR_DISTANCE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FIELD_OF_VIEW", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INTENSITY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MORPH_WEIGHTS", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("NEAR_DISTANCE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ORIENTATION", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PICKABILITY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SCALE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SHININESS", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SPECULAR_COLOR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SPOT_ANGLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SPOT_EXPONENT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRANSLATION", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("VISIBILITY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("sequence", "Ljavax/microedition/m3g/KeyframeSequence;", Default::default()),
                JavaFieldProto::new("controller", "Ljavax/microedition/m3g/AnimationController;", Default::default()),
                JavaFieldProto::new("targetProperty", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/m3g/AnimationTrack";
        for (name, value) in [
            ("ALPHA", Self::ALPHA),
            ("AMBIENT_COLOR", Self::AMBIENT_COLOR),
            ("COLOR", Self::COLOR),
            ("CROP", Self::CROP),
            ("DENSITY", Self::DENSITY),
            ("DIFFUSE_COLOR", Self::DIFFUSE_COLOR),
            ("EMISSIVE_COLOR", Self::EMISSIVE_COLOR),
            ("FAR_DISTANCE", Self::FAR_DISTANCE),
            ("FIELD_OF_VIEW", Self::FIELD_OF_VIEW),
            ("INTENSITY", Self::INTENSITY),
            ("MORPH_WEIGHTS", Self::MORPH_WEIGHTS),
            ("NEAR_DISTANCE", Self::NEAR_DISTANCE),
            ("ORIENTATION", Self::ORIENTATION),
            ("PICKABILITY", Self::PICKABILITY),
            ("SCALE", Self::SCALE),
            ("SHININESS", Self::SHININESS),
            ("SPECULAR_COLOR", Self::SPECULAR_COLOR),
            ("SPOT_ANGLE", Self::SPOT_ANGLE),
            ("SPOT_EXPONENT", Self::SPOT_EXPONENT),
            ("TRANSLATION", Self::TRANSLATION),
            ("VISIBILITY", Self::VISIBILITY),
        ] {
            jvm.put_static_field(class, name, "I", value).await?;
        }
        Ok(())
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "sequence",
            "Ljavax/microedition/m3g/KeyframeSequence;",
            null_ref::<KeyframeSequence>(),
        )
        .await?;
        jvm.put_field(
            &mut this,
            "controller",
            "Ljavax/microedition/m3g/AnimationController;",
            null_ref::<AnimationController>(),
        )
        .await?;
        jvm.put_field(&mut this, "targetProperty", "I", 0).await
    }

    pub(super) async fn init_with_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<KeyframeSequence>,
        property: i32,
    ) -> Result<()> {
        if sequence.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "AnimationTrack sequence").await);
        }
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;", sequence)
            .await?;
        jvm.put_field(
            &mut this,
            "controller",
            "Ljavax/microedition/m3g/AnimationController;",
            null_ref::<AnimationController>(),
        )
        .await?;
        jvm.put_field(&mut this, "targetProperty", "I", property).await
    }

    pub(super) async fn get_controller(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<AnimationController>> {
        jvm.get_field(&this, "controller", "Ljavax/microedition/m3g/AnimationController;").await
    }

    pub(super) async fn get_keyframe_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<KeyframeSequence>> {
        jvm.get_field(&this, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;").await
    }

    pub(super) async fn get_target_property(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "targetProperty", "I").await
    }

    pub(super) async fn set_controller(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        controller: ClassInstanceRef<AnimationController>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "controller", "Ljavax/microedition/m3g/AnimationController;", controller)
            .await
    }
}
