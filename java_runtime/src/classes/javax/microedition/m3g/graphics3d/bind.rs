#[allow(unused_imports)]
use super::super::common::*;
#[allow(unused_imports)]
use super::super::math::*;
#[allow(unused_imports)]
use super::super::prelude::*;
#[allow(unused_imports)]
use super::super::raw_arrays::*;
#[allow(unused_imports)]
use super::super::render::*;
#[allow(unused_imports)]
use super::super::types::*;

impl super::super::Graphics3D {
    pub(crate) async fn bind_target(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Object>,
    ) -> Result<()> {
        tracing::debug!(target: "rustjava_m3g", "m3g.Graphics3D.bindTarget target={target:?}");
        let target = Self::resolve_bind_target(jvm, target).await?;
        jvm.put_field(&mut this, "target", "Ljava/lang/Object;", target.clone()).await?;
        if !target.is_null() {
            if let Some(instance) = target.instance.as_deref() {
                if jvm.is_instance(instance, "javax/microedition/lcdui/Graphics") {
                    let graphics: ClassInstanceRef<Graphics> = cast_ref(&target);
                    let clip_x: i32 = jvm.invoke_virtual(&graphics, "getClipX", "()I", ()).await.unwrap_or(0);
                    let clip_y: i32 = jvm.invoke_virtual(&graphics, "getClipY", "()I", ()).await.unwrap_or(0);
                    let clip_w: i32 = jvm.invoke_virtual(&graphics, "getClipWidth", "()I", ()).await.unwrap_or(0);
                    let clip_h: i32 = jvm.invoke_virtual(&graphics, "getClipHeight", "()I", ()).await.unwrap_or(0);
                    if clip_w > 0 && clip_h > 0 {
                        jvm.put_field(&mut this, "viewportX", "I", clip_x).await?;
                        jvm.put_field(&mut this, "viewportY", "I", clip_y).await?;
                        jvm.put_field(&mut this, "viewportW", "I", clip_w).await?;
                        jvm.put_field(&mut this, "viewportH", "I", clip_h).await?;
                    }
                }
            }
        }
        Self::clear_framebuffers(jvm, &mut this).await
    }
    pub(crate) async fn resolve_bind_target(jvm: &Jvm, target: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        if target.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Graphics3D.bindTarget").await);
        }
        let Some(instance) = target.instance.as_deref() else {
            return Ok(target);
        };
        if jvm.is_instance(instance, "javax/microedition/lcdui/Image") {
            let image: ClassInstanceRef<Image> = cast_ref(&target);
            if !jvm.get_field::<bool>(&image, "mutable", "Z").await.unwrap_or(false) {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "Graphics3D target Image is immutable")
                    .await);
            }
            let graphics: ClassInstanceRef<Graphics> = jvm
                .invoke_virtual(&image, "getGraphics", "()Ljavax/microedition/lcdui/Graphics;", ())
                .await?;
            return Ok(cast_ref(&graphics));
        }
        if jvm.is_instance(instance, "javax/microedition/lcdui/Graphics") {
            return Ok(target);
        }
        Err(jvm
            .exception("java/lang/IllegalArgumentException", "Graphics3D target must be Graphics or Image")
            .await)
    }
    pub(crate) async fn bind_target_with_options(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Object>,
        depth_buffer: bool,
        hints: i32,
    ) -> Result<()> {
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.Graphics3D.bindTarget target={target:?} depth={depth_buffer:?} hints={hints:?}"
        );
        jvm.put_field(&mut this, "depthEnabled", "Z", depth_buffer).await?;
        jvm.put_field(&mut this, "hints", "I", hints).await?;
        Self::bind_target(jvm, context, this, target).await
    }
    pub(crate) async fn clear(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        background: ClassInstanceRef<Background>,
    ) -> Result<()> {
        let target: ClassInstanceRef<Object> = jvm.get_field(&this, "target", "Ljava/lang/Object;").await?;
        if target.is_null() {
            return Ok(());
        }
        let viewport_x: i32 = jvm.get_field(&this, "viewportX", "I").await?;
        let viewport_y: i32 = jvm.get_field(&this, "viewportY", "I").await?;
        let viewport_w: i32 = jvm.get_field(&this, "viewportW", "I").await?;
        let viewport_h: i32 = jvm.get_field(&this, "viewportH", "I").await?;
        if viewport_w <= 0 || viewport_h <= 0 {
            return Ok(());
        }
        let pixel_count = (viewport_w * viewport_h) as usize;
        let color_clear = if !background.is_null() {
            jvm.get_field(&background, "colorClear", "Z").await.unwrap_or(true)
        } else {
            true
        };
        let depth_clear = if !background.is_null() {
            jvm.get_field(&background, "depthClear", "Z").await.unwrap_or(true)
        } else {
            true
        };

        if color_clear {
            let mut pixels = vec![0i32; pixel_count];
            let process_alpha = if !background.is_null() {
                let color = ensure_opaque(jvm.get_field(&background, "color", "I").await.unwrap_or(0));
                pixels.fill(color);
                Self::paint_background_image(jvm, &background, viewport_w, viewport_h, color, &mut pixels).await?;
                false
            } else {
                true
            };

            let graphics: ClassInstanceRef<Graphics> = cast_ref(&target);
            Graphics::draw_pixels(
                jvm,
                context,
                &graphics,
                viewport_x,
                viewport_y,
                viewport_w,
                viewport_h,
                &pixels,
                process_alpha,
            )
            .await?;
            Self::store_color_buffer(jvm, &this, &pixels).await?;
        }

        if depth_clear {
            let depth = vec![f32::INFINITY; pixel_count];
            Self::store_depth_buffer(jvm, &this, &depth).await?;
        }
        Ok(())
    }
    pub(crate) async fn get_target(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "target", "Ljava/lang/Object;").await
    }
    pub(crate) async fn get_viewport_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportH", "I").await
    }
    pub(crate) async fn get_viewport_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportW", "I").await
    }
    pub(crate) async fn get_viewport_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportX", "I").await
    }
    pub(crate) async fn get_viewport_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportY", "I").await
    }
    pub(crate) async fn is_depth_buffer_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "depthEnabled", "Z").await
    }
    pub(crate) async fn release_target(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!(target: "rustjava_m3g", "m3g.Graphics3D.releaseTarget");
        jvm.put_field(&mut this, "target", "Ljava/lang/Object;", null_ref::<Object>()).await?;
        Self::clear_framebuffers(jvm, &mut this).await
    }
    pub(crate) async fn set_viewport(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "viewportX", "I", x).await?;
        jvm.put_field(&mut this, "viewportY", "I", y).await?;
        jvm.put_field(&mut this, "viewportW", "I", width).await?;
        jvm.put_field(&mut this, "viewportH", "I", height).await
    }
}
