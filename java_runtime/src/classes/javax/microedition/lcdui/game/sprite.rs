#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::javax::microedition::lcdui::{Graphics, Image},
};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

// class javax.microedition.lcdui.game.Sprite
impl Sprite {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/game/Sprite",
            parent_class: Some("javax/microedition/lcdui/game/Layer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljavax/microedition/lcdui/Image;)V", Self::init_from_image, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/lcdui/Image;II)V",
                    Self::init_from_frames,
                    Default::default(),
                ),
                JavaMethodProto::new("defineReferencePixel", "(II)V", Self::define_reference_pixel, Default::default()),
                JavaMethodProto::new("getFrame", "()I", Self::get_frame, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new("getRawFrameCount", "()I", Self::get_raw_frame_count, Default::default()),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, Default::default()),
                JavaMethodProto::new("nextFrame", "()V", Self::next_frame, Default::default()),
                JavaMethodProto::new("paint", "(Ljavax/microedition/lcdui/Graphics;)V", Self::paint, Default::default()),
                JavaMethodProto::new("setFrame", "(I)V", Self::set_frame, Default::default()),
                JavaMethodProto::new("setPosition", "(II)V", Self::set_position, Default::default()),
                JavaMethodProto::new("setRefPixelPosition", "(II)V", Self::set_ref_pixel_position, Default::default()),
                JavaMethodProto::new("setTransform", "(I)V", Self::set_transform, Default::default()),
                JavaMethodProto::new("setFrameSequence", "([I)V", Self::set_frame_sequence, Default::default()),
                JavaMethodProto::new("getFrameSequenceLength", "()I", Self::get_frame_sequence_length, Default::default()),
                JavaMethodProto::new("prevFrame", "()V", Self::prev_frame, Default::default()),
                JavaMethodProto::new(
                    "collidesWith",
                    "(Ljavax/microedition/lcdui/game/Sprite;Z)Z",
                    Self::collides_with_sprite,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "collidesWith",
                    "(Ljavax/microedition/lcdui/game/TiledLayer;Z)Z",
                    Self::collides_with_tiled,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "collidesWith",
                    "(Ljavax/microedition/lcdui/Image;IIZ)Z",
                    Self::collides_with_image,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "defineCollisionRectangle",
                    "(IIII)V",
                    Self::define_collision_rectangle,
                    Default::default(),
                ),
                JavaMethodProto::new("<init>", "(Ljavax/microedition/lcdui/game/Sprite;)V", Self::init_copy, Default::default()),
                JavaMethodProto::new("getRefPixelX", "()I", Self::get_ref_pixel_x, Default::default()),
                JavaMethodProto::new("getRefPixelY", "()I", Self::get_ref_pixel_y, Default::default()),
                JavaMethodProto::new("setImage", "(Ljavax/microedition/lcdui/Image;II)V", Self::set_image, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("image", "Ljavax/microedition/lcdui/Image;", Default::default()),
                JavaFieldProto::new("frameWidth", "I", Default::default()),
                JavaFieldProto::new("frameHeight", "I", Default::default()),
                JavaFieldProto::new("frame", "I", Default::default()),
                JavaFieldProto::new("x", "I", Default::default()),
                JavaFieldProto::new("y", "I", Default::default()),
                JavaFieldProto::new("refX", "I", Default::default()),
                JavaFieldProto::new("refY", "I", Default::default()),
                JavaFieldProto::new("transform", "I", Default::default()),
                JavaFieldProto::new("sequence", "[I", Default::default()),
                JavaFieldProto::new("collisionX", "I", Default::default()),
                JavaFieldProto::new("collisionY", "I", Default::default()),
                JavaFieldProto::new("collisionW", "I", Default::default()),
                JavaFieldProto::new("collisionH", "I", Default::default()),
                JavaFieldProto::new("TRANS_NONE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRANS_ROT90", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRANS_ROT180", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRANS_ROT270", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRANS_MIRROR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRANS_MIRROR_ROT90", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRANS_MIRROR_ROT180", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRANS_MIRROR_ROT270", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init_from_image(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
    ) -> Result<()> {
        let (width, height, _) = if image.is_null() {
            (0, 0, ClassInstanceRef::new(None))
        } else {
            Image::pixels(jvm, &image).await?
        };
        Self::init_from_frames(jvm, context, this, image, width, height).await
    }

    pub(super) async fn init_from_frames(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        frame_width: i32,
        frame_height: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.game.Sprite::<init>({this:?}, {image:?}, {frame_width:?}, {frame_height:?})");

        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/game/Layer", "<init>", "()V", ())
            .await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "frameWidth", "I", frame_width.max(0)).await?;
        jvm.put_field(&mut this, "frameHeight", "I", frame_height.max(0)).await?;
        jvm.put_field(&mut this, "frame", "I", 0).await?;
        jvm.put_field(&mut this, "x", "I", 0).await?;
        jvm.put_field(&mut this, "y", "I", 0).await?;
        jvm.put_field(&mut this, "refX", "I", 0).await?;
        jvm.put_field(&mut this, "refY", "I", 0).await?;
        jvm.put_field(&mut this, "transform", "I", 0).await?;
        jvm.put_field(&mut this, "collisionX", "I", 0).await?;
        jvm.put_field(&mut this, "collisionY", "I", 0).await?;
        jvm.put_field(&mut this, "collisionW", "I", frame_width.max(0)).await?;
        jvm.put_field(&mut this, "collisionH", "I", frame_height.max(0)).await?;
        jvm.put_field(&mut this, "visible", "Z", true).await
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/game/Sprite";
        jvm.put_static_field(class, "TRANS_NONE", "I", 0).await?;
        jvm.put_static_field(class, "TRANS_ROT90", "I", 5).await?;
        jvm.put_static_field(class, "TRANS_ROT180", "I", 3).await?;
        jvm.put_static_field(class, "TRANS_ROT270", "I", 6).await?;
        jvm.put_static_field(class, "TRANS_MIRROR", "I", 2).await?;
        jvm.put_static_field(class, "TRANS_MIRROR_ROT90", "I", 7).await?;
        jvm.put_static_field(class, "TRANS_MIRROR_ROT180", "I", 1).await?;
        jvm.put_static_field(class, "TRANS_MIRROR_ROT270", "I", 4).await
    }

    pub(super) async fn define_reference_pixel(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        jvm.put_field(&mut this, "refX", "I", x).await?;
        jvm.put_field(&mut this, "refY", "I", y).await
    }

    pub(super) async fn get_frame(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "frame", "I").await
    }

    pub(super) async fn get_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let transform: i32 = jvm.get_field(&this, "transform", "I").await?;
        let width: i32 = jvm.get_field(&this, "frameWidth", "I").await?;
        let height: i32 = jvm.get_field(&this, "frameHeight", "I").await?;
        Ok(if (4..=7).contains(&transform) { width } else { height })
    }

    pub(super) async fn get_raw_frame_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Self::frame_count(jvm, &this).await
    }

    pub(super) async fn get_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let transform: i32 = jvm.get_field(&this, "transform", "I").await?;
        let width: i32 = jvm.get_field(&this, "frameWidth", "I").await?;
        let height: i32 = jvm.get_field(&this, "frameHeight", "I").await?;
        Ok(if (4..=7).contains(&transform) { height } else { width })
    }

    pub(super) async fn next_frame(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let frame: i32 = jvm.get_field(&this, "frame", "I").await?;
        let count = Self::frame_count(jvm, &this).await?.max(1);
        jvm.put_field(&mut this, "frame", "I", (frame + 1).rem_euclid(count)).await
    }

    pub(super) async fn paint(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        graphics: ClassInstanceRef<Graphics>,
    ) -> Result<()> {
        if graphics.is_null() {
            return Ok(());
        }

        let image: ClassInstanceRef<Image> = jvm.get_field(&this, "image", "Ljavax/microedition/lcdui/Image;").await?;
        if image.is_null() {
            return Ok(());
        }

        let (image_width, _, _) = Image::pixels(jvm, &image).await?;
        let frame_width: i32 = jvm.get_field(&this, "frameWidth", "I").await?;
        let frame_height: i32 = jvm.get_field(&this, "frameHeight", "I").await?;
        if frame_width <= 0 || frame_height <= 0 {
            return Ok(());
        }

        let columns = (image_width / frame_width).max(1);
        let frame: i32 = jvm.get_field(&this, "frame", "I").await?;
        let src_x = frame.rem_euclid(columns) * frame_width;
        let src_y = (frame / columns) * frame_height;
        let x: i32 = jvm.get_field(&this, "x", "I").await?;
        let y: i32 = jvm.get_field(&this, "y", "I").await?;
        let transform: i32 = jvm.get_field(&this, "transform", "I").await?;

        Graphics::draw_region(
            jvm,
            context,
            graphics,
            image,
            src_x,
            src_y,
            frame_width,
            frame_height,
            transform,
            x,
            y,
            4 | 16,
        )
        .await
    }

    pub(super) async fn set_frame(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, frame: i32) -> Result<()> {
        let count = Self::frame_count(jvm, &this).await?.max(1);
        jvm.put_field(&mut this, "frame", "I", frame.clamp(0, count - 1)).await
    }

    pub(super) async fn set_position(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        jvm.put_field(&mut this, "x", "I", x).await?;
        jvm.put_field(&mut this, "y", "I", y).await
    }

    pub(super) async fn set_ref_pixel_position(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        let ref_x: i32 = jvm.get_field(&this, "refX", "I").await?;
        let ref_y: i32 = jvm.get_field(&this, "refY", "I").await?;
        jvm.put_field(&mut this, "x", "I", x - ref_x).await?;
        jvm.put_field(&mut this, "y", "I", y - ref_y).await
    }

    pub(super) async fn set_transform(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, transform: i32) -> Result<()> {
        jvm.put_field(&mut this, "transform", "I", transform).await
    }

    pub(super) async fn frame_count(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<i32> {
        let image: ClassInstanceRef<Image> = jvm.get_field(this, "image", "Ljavax/microedition/lcdui/Image;").await?;
        if image.is_null() {
            return Ok(0);
        }

        let (image_width, image_height, _) = Image::pixels(jvm, &image).await?;
        let frame_width: i32 = jvm.get_field(this, "frameWidth", "I").await?;
        let frame_height: i32 = jvm.get_field(this, "frameHeight", "I").await?;
        if frame_width <= 0 || frame_height <= 0 {
            return Ok(0);
        }

        Ok(((image_width / frame_width).max(1)) * ((image_height / frame_height).max(1)))
    }

    pub(super) async fn set_frame_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "sequence", "[I", sequence).await
    }

    pub(super) async fn get_frame_sequence_length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let sequence: ClassInstanceRef<Array<i32>> = jvm.get_field(&this, "sequence", "[I").await?;
        if sequence.is_null() {
            return Self::frame_count(jvm, &this).await;
        }
        Ok(jvm.array_length(&sequence).await? as i32)
    }

    pub(super) async fn prev_frame(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let frame: i32 = jvm.get_field(&this, "frame", "I").await?;
        let count = Self::frame_count(jvm, &this).await?.max(1);
        jvm.put_field(&mut this, "frame", "I", (frame - 1).rem_euclid(count)).await
    }

    pub(super) async fn define_collision_rectangle(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "collisionX", "I", x).await?;
        jvm.put_field(&mut this, "collisionY", "I", y).await?;
        jvm.put_field(&mut this, "collisionW", "I", width).await?;
        jvm.put_field(&mut this, "collisionH", "I", height).await
    }

    pub(super) async fn init_copy(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<()> {
        let image: ClassInstanceRef<Image> = jvm.get_field(&other, "image", "Ljavax/microedition/lcdui/Image;").await?;
        let frame_width: i32 = jvm.get_field(&other, "frameWidth", "I").await?;
        let frame_height: i32 = jvm.get_field(&other, "frameHeight", "I").await?;
        Self::init_from_frames(jvm, context, this.clone(), image, frame_width, frame_height).await?;
        let mut this = this;
        let x: i32 = jvm.get_field(&other, "x", "I").await?;
        let y: i32 = jvm.get_field(&other, "y", "I").await?;
        let frame: i32 = jvm.get_field(&other, "frame", "I").await?;
        let transform: i32 = jvm.get_field(&other, "transform", "I").await?;
        jvm.put_field(&mut this, "x", "I", x).await?;
        jvm.put_field(&mut this, "y", "I", y).await?;
        jvm.put_field(&mut this, "frame", "I", frame).await?;
        jvm.put_field(&mut this, "transform", "I", transform).await
    }

    pub(super) async fn get_ref_pixel_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let x: i32 = jvm.get_field(&this, "x", "I").await?;
        let ref_x: i32 = jvm.get_field(&this, "refX", "I").await?;
        Ok(x + ref_x)
    }

    pub(super) async fn get_ref_pixel_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let y: i32 = jvm.get_field(&this, "y", "I").await?;
        let ref_y: i32 = jvm.get_field(&this, "refY", "I").await?;
        Ok(y + ref_y)
    }

    pub(super) async fn set_image(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        frame_width: i32,
        frame_height: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "frameWidth", "I", frame_width.max(0)).await?;
        jvm.put_field(&mut this, "frameHeight", "I", frame_height.max(0)).await?;
        jvm.put_field(&mut this, "frame", "I", 0).await
    }

    pub(super) async fn bounds(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<(i32, i32, i32, i32)> {
        let x: i32 = jvm.get_field(this, "x", "I").await?;
        let y: i32 = jvm.get_field(this, "y", "I").await?;
        let cx: i32 = jvm.get_field(this, "collisionX", "I").await?;
        let cy: i32 = jvm.get_field(this, "collisionY", "I").await?;
        let cw: i32 = jvm.get_field(this, "collisionW", "I").await?;
        let ch: i32 = jvm.get_field(this, "collisionH", "I").await?;
        Ok((x + cx, y + cy, cw.max(0), ch.max(0)))
    }

    pub(super) fn aabb_overlap(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> bool {
        a.0 < b.0 + b.2 && a.0 + a.2 > b.0 && a.1 < b.1 + b.3 && a.1 + a.3 > b.1
    }

    pub(super) async fn collides_with_sprite(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
        _pixel_level: bool,
    ) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }
        let a = Self::bounds(jvm, &this).await?;
        let b = Self::bounds(jvm, &other).await?;
        Ok(Self::aabb_overlap(a, b))
    }

    pub(super) async fn collides_with_tiled(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<TiledLayer>,
        _pixel_level: bool,
    ) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }
        let a = Self::bounds(jvm, &this).await?;
        let x: i32 = jvm.get_field(&other, "x", "I").await?;
        let y: i32 = jvm.get_field(&other, "y", "I").await?;
        let w: i32 = jvm.invoke_virtual(&other, "getWidth", "()I", ()).await?;
        let h: i32 = jvm.invoke_virtual(&other, "getHeight", "()I", ()).await?;
        Ok(Self::aabb_overlap(a, (x, y, w, h)))
    }

    pub(super) async fn collides_with_image(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        x: i32,
        y: i32,
        _pixel_level: bool,
    ) -> Result<bool> {
        if image.is_null() {
            return Ok(false);
        }
        let a = Self::bounds(jvm, &this).await?;
        let (w, h, _) = Image::pixels(jvm, &image).await?;
        Ok(Self::aabb_overlap(a, (x, y, w, h)))
    }
}
