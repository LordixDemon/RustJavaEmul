use super::super::{
    diagnostics::{V3FrameStats, publish_frame_diagnostic},
    queue::{graphics_3d_key, take_graphics_3d_queue},
    raster::{PreparedSource, SceneTri},
    render::draw_scene_triangles,
    storage::storage_cache_stats,
};
#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::javax::microedition::lcdui::Graphics};
#[allow(unused_imports)]
use alloc::{vec, vec::Vec};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

impl Graphics3D {
    pub const CLASS_NAME: &'static str = "com/mascotcapsule/micro3d/v3/Graphics3D";

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: Self::CLASS_NAME,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getInstance",
                    "()Lcom/mascotcapsule/micro3d/v3/Graphics3D;",
                    Self::get_instance,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("bind", "(Ljavax/microedition/lcdui/Graphics;)V", Self::bind, Default::default()),
                JavaMethodProto::new("release", "(Ljavax/microedition/lcdui/Graphics;)V", Self::release, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
                JavaMethodProto::new("dispose", "()V", Self::dispose, Default::default()),
                JavaMethodProto::new(
                    "renderFigure",
                    "(Lcom/mascotcapsule/micro3d/v3/Figure;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;)V",
                    Self::render_figure,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "drawFigure",
                    "(Lcom/mascotcapsule/micro3d/v3/Figure;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;)V",
                    Self::draw_figure,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "renderPrimitives",
                    "(Lcom/mascotcapsule/micro3d/v3/Texture;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;II[I[I[I[I)V",
                    Self::render_primitives,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "drawCommandList",
                    "(Lcom/mascotcapsule/micro3d/v3/Texture;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;[I)V",
                    Self::draw_command_list_single,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "drawCommandList",
                    "([Lcom/mascotcapsule/micro3d/v3/Texture;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;[I)V",
                    Self::draw_command_list_array,
                    Default::default(),
                ),
            ],
            fields: vec![
                static_int_field("COMMAND_LIST_VERSION_1_0"),
                static_int_field("COMMAND_END"),
                static_int_field("COMMAND_NOP"),
                static_int_field("COMMAND_FLUSH"),
                static_int_field("COMMAND_ATTRIBUTE"),
                static_int_field("COMMAND_CLIP"),
                static_int_field("COMMAND_CENTER"),
                static_int_field("COMMAND_TEXTURE_INDEX"),
                static_int_field("COMMAND_AFFINE_INDEX"),
                static_int_field("COMMAND_PARALLEL_SCALE"),
                static_int_field("COMMAND_PARALLEL_SIZE"),
                static_int_field("COMMAND_PERSPECTIVE_FOV"),
                static_int_field("COMMAND_PERSPECTIVE_WH"),
                static_int_field("COMMAND_AMBIENT_LIGHT"),
                static_int_field("COMMAND_DIRECTION_LIGHT"),
                static_int_field("COMMAND_THRESHOLD"),
                static_int_field("ENV_ATTR_LIGHTING"),
                static_int_field("ENV_ATTR_SPHERE_MAP"),
                static_int_field("ENV_ATTR_TOON_SHADING"),
                static_int_field("ENV_ATTR_SEMI_TRANSPARENT"),
                static_int_field("PATTR_LIGHTING"),
                static_int_field("PATTR_SPHERE_MAP"),
                static_int_field("PATTR_COLORKEY"),
                static_int_field("PATTR_BLEND_NORMAL"),
                static_int_field("PATTR_BLEND_HALF"),
                static_int_field("PATTR_BLEND_ADD"),
                static_int_field("PATTR_BLEND_SUB"),
                static_int_field("PDATA_NORMAL_NONE"),
                static_int_field("PDATA_NORMAL_PER_FACE"),
                static_int_field("PDATA_NORMAL_PER_VERTEX"),
                static_int_field("PDATA_COLOR_NONE"),
                static_int_field("PDATA_COLOR_PER_COMMAND"),
                static_int_field("PDATA_COLOR_PER_FACE"),
                static_int_field("PDATA_TEXURE_COORD_NONE"),
                static_int_field("PDATA_TEXURE_COORD"),
                static_int_field("PDATA_POINT_SPRITE_PARAMS_PER_CMD"),
                static_int_field("PDATA_POINT_SPRITE_PARAMS_PER_FACE"),
                static_int_field("PDATA_POINT_SPRITE_PARAMS_PER_VERTEX"),
                static_int_field("POINT_SPRITE_LOCAL_SIZE"),
                static_int_field("POINT_SPRITE_PIXEL_SIZE"),
                static_int_field("POINT_SPRITE_PERSPECTIVE"),
                static_int_field("POINT_SPRITE_NO_PERS"),
                static_int_field("PRIMITVE_POINTS"),
                static_int_field("PRIMITVE_LINES"),
                static_int_field("PRIMITVE_TRIANGLES"),
                static_int_field("PRIMITVE_QUADS"),
                static_int_field("PRIMITVE_POINT_SPRITES"),
                JavaFieldProto::new("boundGraphics", "Ljavax/microedition/lcdui/Graphics;", Default::default()),
                JavaFieldProto::new("renderCount", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        for (name, value) in [
            ("COMMAND_LIST_VERSION_1_0", 0xfe00_0001u32 as i32),
            ("COMMAND_END", 0x8000_0000u32 as i32),
            ("COMMAND_NOP", 0x8100_0000u32 as i32),
            ("COMMAND_FLUSH", 0x8200_0000u32 as i32),
            ("COMMAND_ATTRIBUTE", 0x8300_0000u32 as i32),
            ("COMMAND_CLIP", 0x8400_0000u32 as i32),
            ("COMMAND_CENTER", 0x8500_0000u32 as i32),
            ("COMMAND_TEXTURE_INDEX", 0x8600_0000u32 as i32),
            ("COMMAND_AFFINE_INDEX", 0x8700_0000u32 as i32),
            ("COMMAND_PARALLEL_SCALE", 0x9000_0000u32 as i32),
            ("COMMAND_PARALLEL_SIZE", 0x9100_0000u32 as i32),
            ("COMMAND_PERSPECTIVE_FOV", 0x9200_0000u32 as i32),
            ("COMMAND_PERSPECTIVE_WH", 0x9300_0000u32 as i32),
            ("COMMAND_AMBIENT_LIGHT", 0xa000_0000u32 as i32),
            ("COMMAND_DIRECTION_LIGHT", 0xa100_0000u32 as i32),
            ("COMMAND_THRESHOLD", 0xaf00_0000u32 as i32),
            ("ENV_ATTR_LIGHTING", 0x01),
            ("ENV_ATTR_SPHERE_MAP", 0x02),
            ("ENV_ATTR_TOON_SHADING", 0x04),
            ("ENV_ATTR_SEMI_TRANSPARENT", 0x08),
            ("PATTR_LIGHTING", 0x01),
            ("PATTR_SPHERE_MAP", 0x02),
            ("PATTR_COLORKEY", 0x10),
            ("PATTR_BLEND_NORMAL", 0x00),
            ("PATTR_BLEND_HALF", 0x20),
            ("PATTR_BLEND_ADD", 0x40),
            ("PATTR_BLEND_SUB", 0x60),
            ("PDATA_NORMAL_NONE", 0x0000),
            ("PDATA_NORMAL_PER_FACE", 0x0200),
            ("PDATA_NORMAL_PER_VERTEX", 0x0300),
            ("PDATA_COLOR_NONE", 0x0000),
            ("PDATA_COLOR_PER_COMMAND", 0x0400),
            ("PDATA_COLOR_PER_FACE", 0x0800),
            ("PDATA_TEXURE_COORD_NONE", 0x0000),
            ("PDATA_TEXURE_COORD", 0x3000),
            ("PDATA_POINT_SPRITE_PARAMS_PER_CMD", 0x1000),
            ("PDATA_POINT_SPRITE_PARAMS_PER_FACE", 0x2000),
            ("PDATA_POINT_SPRITE_PARAMS_PER_VERTEX", 0x3000),
            ("POINT_SPRITE_LOCAL_SIZE", 0),
            ("POINT_SPRITE_PIXEL_SIZE", 1),
            ("POINT_SPRITE_PERSPECTIVE", 0),
            ("POINT_SPRITE_NO_PERS", 2),
            ("PRIMITVE_POINTS", 0x0100_0000),
            ("PRIMITVE_LINES", 0x0200_0000),
            ("PRIMITVE_TRIANGLES", 0x0300_0000),
            ("PRIMITVE_QUADS", 0x0400_0000),
            ("PRIMITVE_POINT_SPRITES", 0x0500_0000),
        ] {
            jvm.put_static_field(Self::CLASS_NAME, name, "I", value).await?;
        }
        Ok(())
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Graphics3D::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "boundGraphics",
            "Ljavax/microedition/lcdui/Graphics;",
            ClassInstanceRef::<Graphics>::new(None),
        )
        .await?;
        jvm.put_field(&mut this, "renderCount", "I", 0).await
    }

    pub(super) async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("com/mascotcapsule/micro3d/v3/Graphics3D", "()V", ()).await?.into())
    }

    pub(super) async fn bind(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        graphics: ClassInstanceRef<Graphics>,
    ) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Graphics3D::bind({this:?}, {graphics:?})");

        jvm.put_field(&mut this, "boundGraphics", "Ljavax/microedition/lcdui/Graphics;", graphics)
            .await
    }

    pub(super) async fn release(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        graphics: ClassInstanceRef<Graphics>,
    ) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Graphics3D::release({this:?}, {graphics:?})");

        Self::flush(jvm, context, this.clone()).await?;
        jvm.put_field(
            &mut this,
            "boundGraphics",
            "Ljavax/microedition/lcdui/Graphics;",
            ClassInstanceRef::<Graphics>::new(None),
        )
        .await
    }

    pub(super) async fn flush(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Graphics3D::flush({this:?})");

        let frame_start_ms = context.now();
        let queue_start_ms = context.now();
        let prepared_figures = take_graphics_3d_queue(graphics_3d_key(&this));
        let queue_ms = context.now().saturating_sub(queue_start_ms);
        if prepared_figures.is_empty() {
            return Ok(());
        }
        let queued = prepared_figures.len();

        let graphics: ClassInstanceRef<Graphics> = jvm.get_field(&this, "boundGraphics", "Ljavax/microedition/lcdui/Graphics;").await?;
        if graphics.is_null() {
            return Ok(());
        }

        let merge_start_ms = context.now();
        let mut scene_textures = Vec::new();
        let mut scene_triangles = Vec::new();
        let mut figure_count = 0usize;
        let mut primitive_count = 0usize;
        let mut prepare_ms = 0u64;
        let mut figure_prepare_ms = 0u64;
        let mut primitive_prepare_ms = 0u64;
        for prepared in prepared_figures {
            match prepared.source {
                PreparedSource::Figure => {
                    figure_count += 1;
                    figure_prepare_ms += prepared.prepare_ms;
                }
                PreparedSource::Primitive => {
                    primitive_count += 1;
                    primitive_prepare_ms += prepared.prepare_ms;
                }
            }
            prepare_ms += prepared.prepare_ms;
            let texture_base = scene_textures.len();
            scene_textures.extend(prepared.textures);
            for mut tri in prepared.triangles {
                if let Some(texture) = tri.texture {
                    tri.texture = Some(texture_base + texture);
                }
                scene_triangles.push(SceneTri {
                    tri,
                    effect_transparency: prepared.effect_transparency,
                    clip: prepared.clip,
                });
            }
        }
        let merge_ms = context.now().saturating_sub(merge_start_ms);

        let sort_start_ms = context.now();
        scene_triangles.sort_by(|a, b| b.tri.z.cmp(&a.tri.z));
        let sort_ms = context.now().saturating_sub(sort_start_ms);

        let draw_start_ms = context.now();
        let draw = draw_scene_triangles(jvm, context, &graphics, &scene_textures, &scene_triangles).await?;
        let draw_ms = context.now().saturating_sub(draw_start_ms);
        let now_ms = context.now();
        let cache_stats = storage_cache_stats();
        publish_frame_diagnostic(
            now_ms,
            V3FrameStats {
                total_ms: now_ms.saturating_sub(frame_start_ms).saturating_add(prepare_ms),
                queue_ms,
                merge_ms,
                sort_ms,
                draw_ms,
                prepare_ms,
                figure_prepare_ms,
                primitive_prepare_ms,
                queued,
                figure_count,
                primitive_count,
                textures: scene_textures.len(),
                triangles: scene_triangles.len(),
                figure_cache_hits: cache_stats.figure_hits,
                figure_cache_misses: cache_stats.figure_misses,
                texture_cache_hits: cache_stats.texture_hits,
                texture_cache_misses: cache_stats.texture_misses,
                draw,
            },
        );
        Ok(())
    }

    pub(super) async fn dispose(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _ = take_graphics_3d_queue(graphics_3d_key(&this));
        Ok(())
    }
}
