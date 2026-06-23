use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::javax::microedition::lcdui::Graphics};

use super::{
    diagnostics::{V3FrameStats, publish_frame_diagnostic},
    effect::Effect3D,
    figure::Figure,
    layout::FigureLayout,
    queue::{graphics_3d_key, push_graphics_3d_queue, take_graphics_3d_queue},
    raster::{PreparedSource, SceneTri},
    render::{draw_scene_triangles, prepare_native_figure, prepare_primitives, prepare_primitives_with_state, primitive_render_state},
    storage::storage_cache_stats,
    texture::Texture,
};

// class com.mascotcapsule.micro3d.v3.Graphics3D
pub struct Graphics3D;

fn static_int_field(name: &str) -> JavaFieldProto {
    JavaFieldProto::new(name, "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL)
}

impl Graphics3D {
    const CLASS_NAME: &'static str = "com/mascotcapsule/micro3d/v3/Graphics3D";

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

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
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

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
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

    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("com/mascotcapsule/micro3d/v3/Graphics3D", "()V", ()).await?.into())
    }

    async fn bind(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, graphics: ClassInstanceRef<Graphics>) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Graphics3D::bind({this:?}, {graphics:?})");

        jvm.put_field(&mut this, "boundGraphics", "Ljavax/microedition/lcdui/Graphics;", graphics)
            .await
    }

    async fn release(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, graphics: ClassInstanceRef<Graphics>) -> Result<()> {
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

    async fn flush(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
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

    async fn dispose(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _ = take_graphics_3d_queue(graphics_3d_key(&this));
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn draw_figure(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        figure: ClassInstanceRef<Figure>,
        x: i32,
        y: i32,
        layout: ClassInstanceRef<FigureLayout>,
        effect: ClassInstanceRef<Effect3D>,
    ) -> Result<()> {
        Self::render_figure(jvm, context, this.clone(), figure, x, y, layout, effect).await?;
        Self::flush(jvm, context, this).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn render_figure(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        figure: ClassInstanceRef<Figure>,
        x: i32,
        y: i32,
        layout: ClassInstanceRef<FigureLayout>,
        effect: ClassInstanceRef<Effect3D>,
    ) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Graphics3D::renderFigure({this:?}, {figure:?}, {x:?}, {y:?}, {layout:?}, {effect:?})");

        let mut count: i32 = jvm.get_field(&this, "renderCount", "I").await?;
        count = count.wrapping_add(1);
        jvm.put_field(&mut this, "renderCount", "I", count).await?;

        let prepare_start_ms = context.now();
        let mut prepared = prepare_native_figure(jvm, context, &figure, x, y, &layout, &effect).await?;
        prepared.prepare_ms = context.now().saturating_sub(prepare_start_ms);
        push_graphics_3d_queue(graphics_3d_key(&this), prepared);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn render_primitives(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        texture: ClassInstanceRef<Texture>,
        x: i32,
        y: i32,
        layout: ClassInstanceRef<FigureLayout>,
        effect: ClassInstanceRef<Effect3D>,
        command: i32,
        num_primitives: i32,
        vertices: ClassInstanceRef<Array<i32>>,
        _normals: ClassInstanceRef<Array<i32>>,
        texture_coords: ClassInstanceRef<Array<i32>>,
        colors: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Graphics3D::renderPrimitives({this:?})");
        let count: i32 = jvm.get_field(&this, "renderCount", "I").await?;
        jvm.put_field(&mut this, "renderCount", "I", count.wrapping_add(1)).await?;

        let prepare_start_ms = context.now();
        let vertex_values = load_i32_array(jvm, &vertices).await?;
        let texture_values = load_i32_array(jvm, &texture_coords).await?;
        let color_values = load_i32_array(jvm, &colors).await?;
        let mut prepared = prepare_primitives(
            jvm,
            context,
            &texture,
            x,
            y,
            &layout,
            &effect,
            command,
            num_primitives,
            &vertex_values,
            0,
            &texture_values,
            0,
            &color_values,
            0,
        )
        .await?;
        prepared.prepare_ms = context.now().saturating_sub(prepare_start_ms);
        push_graphics_3d_queue(graphics_3d_key(&this), prepared);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn draw_command_list_single(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        texture: ClassInstanceRef<Texture>,
        x: i32,
        y: i32,
        layout: ClassInstanceRef<FigureLayout>,
        effect: ClassInstanceRef<Effect3D>,
        command_list: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        Self::draw_command_list_impl(jvm, context, this, vec![texture], x, y, layout, effect, command_list).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn draw_command_list_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        textures: ClassInstanceRef<Array<Texture>>,
        x: i32,
        y: i32,
        layout: ClassInstanceRef<FigureLayout>,
        effect: ClassInstanceRef<Effect3D>,
        command_list: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        let textures = if textures.is_null() {
            Vec::new()
        } else {
            let length = jvm.array_length(&textures).await?;
            jvm.load_array(&textures, 0, length).await?
        };
        Self::draw_command_list_impl(jvm, context, this, textures, x, y, layout, effect, command_list).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn draw_command_list_impl(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        textures: Vec<ClassInstanceRef<Texture>>,
        x: i32,
        y: i32,
        layout: ClassInstanceRef<FigureLayout>,
        effect: ClassInstanceRef<Effect3D>,
        command_list: ClassInstanceRef<Array<i32>>,
    ) -> Result<()> {
        const COMMAND_LIST_VERSION_1_0: i32 = 0xfe00_0001u32 as i32;
        const COMMAND_END: i32 = 0x8000_0000u32 as i32;
        const COMMAND_NOP: i32 = 0x8100_0000u32 as i32;
        const COMMAND_FLUSH: i32 = 0x8200_0000u32 as i32;
        const COMMAND_ATTRIBUTE: i32 = 0x8300_0000u32 as i32;
        const COMMAND_CLIP: i32 = 0x8400_0000u32 as i32;
        const COMMAND_CENTER: i32 = 0x8500_0000u32 as i32;
        const COMMAND_TEXTURE_INDEX: i32 = 0x8600_0000u32 as i32;
        const COMMAND_AFFINE_INDEX: i32 = 0x8700_0000u32 as i32;
        const COMMAND_PARALLEL_SCALE: i32 = 0x9000_0000u32 as i32;
        const COMMAND_PARALLEL_SIZE: i32 = 0x9100_0000u32 as i32;
        const COMMAND_PERSPECTIVE_FOV: i32 = 0x9200_0000u32 as i32;
        const COMMAND_PERSPECTIVE_WH: i32 = 0x9300_0000u32 as i32;
        const COMMAND_AMBIENT_LIGHT: i32 = 0xa000_0000u32 as i32;
        const COMMAND_DIRECTION_LIGHT: i32 = 0xa100_0000u32 as i32;
        const COMMAND_THRESHOLD: i32 = 0xaf00_0000u32 as i32;
        const PRIMITIVE_POINTS: i32 = 0x0100_0000;
        const PRIMITIVE_LINES: i32 = 0x0200_0000;
        const PRIMITIVE_TRIANGLES: i32 = 0x0300_0000;
        const PRIMITIVE_QUADS: i32 = 0x0400_0000;
        const PRIMITIVE_POINT_SPRITES: i32 = 0x0500_0000;
        const ENV_ATTR_SEMI_TRANSPARENT: i32 = 8;
        const PDATA_NORMAL_MASK: i32 = 0x0300;
        const PDATA_NORMAL_PER_FACE: i32 = 0x0200;
        const PDATA_NORMAL_PER_VERTEX: i32 = 0x0300;
        const PDATA_COLOR_MASK: i32 = 0x0c00;
        const PDATA_COLOR_PER_COMMAND: i32 = 0x0400;
        const PDATA_COLOR_PER_FACE: i32 = 0x0800;
        const PDATA_TEXTURE_COORD: i32 = 0x3000;
        const PDATA_SPRITE_PARAMS_MASK: i32 = 0x3000;
        const PDATA_POINT_SPRITE_PARAMS_PER_CMD: i32 = 0x1000;

        let command_values = load_i32_array(jvm, &command_list).await?;
        if command_values.first().copied() != Some(COMMAND_LIST_VERSION_1_0) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Unsupported V3 command list").await);
        }

        let active_layout = clone_layout(jvm, context, &layout).await?;
        let mut texture = textures.first().cloned().unwrap_or_else(|| ClassInstanceRef::<Texture>::new(None));
        let mut effect_transparency_override = None;
        let mut active_clip = None;
        let mut primitive_state = None;
        let mut index = 1usize;

        while index < command_values.len() {
            let command = command_values[index];
            index += 1;
            let command_high = command & 0xff00_0000u32 as i32;

            match command_high {
                COMMAND_END => break,
                COMMAND_NOP => index = index.saturating_add((command & 0x00ff_ffff) as usize).min(command_values.len()),
                COMMAND_FLUSH => Self::flush(jvm, context, this.clone()).await?,
                COMMAND_CENTER => {
                    if index + 1 < command_values.len() {
                        let center_x = command_values.get(index).copied().unwrap_or(0);
                        let center_y = command_values.get(index + 1).copied().unwrap_or(0);
                        index += 2;
                        let _: () = jvm.invoke_virtual(&active_layout, "setCenter", "(II)V", (center_x, center_y)).await?;
                        primitive_state = None;
                    }
                }
                COMMAND_TEXTURE_INDEX => {
                    let texture_index = (command & 0x00ff_ffff) as usize;
                    texture = textures
                        .get(texture_index)
                        .cloned()
                        .unwrap_or_else(|| ClassInstanceRef::<Texture>::new(None));
                }
                COMMAND_AFFINE_INDEX => {
                    let affine_index = (command & 0x00ff_ffff) as i32;
                    let _: core::result::Result<(), _> = jvm.invoke_virtual(&active_layout, "selectAffineTrans", "(I)V", (affine_index,)).await;
                    primitive_state = None;
                }
                COMMAND_PARALLEL_SCALE => {
                    if index + 1 < command_values.len() {
                        let sx = command_values.get(index).copied().unwrap_or(4096);
                        let sy = command_values.get(index + 1).copied().unwrap_or(4096);
                        index += 2;
                        let _: () = jvm.invoke_virtual(&active_layout, "setScale", "(II)V", (sx, sy)).await?;
                        primitive_state = None;
                    }
                }
                COMMAND_PARALLEL_SIZE => {
                    if index + 1 < command_values.len() {
                        let width = command_values.get(index).copied().unwrap_or(4096);
                        let height = command_values.get(index + 1).copied().unwrap_or(4096);
                        index += 2;
                        let _: () = jvm.invoke_virtual(&active_layout, "setParallelSize", "(II)V", (width, height)).await?;
                        primitive_state = None;
                    }
                }
                COMMAND_PERSPECTIVE_FOV => {
                    if index + 2 < command_values.len() {
                        let near = command_values.get(index).copied().unwrap_or(1);
                        let far = command_values.get(index + 1).copied().unwrap_or(32767);
                        let angle = command_values.get(index + 2).copied().unwrap_or(512);
                        index += 3;
                        let _: () = jvm.invoke_virtual(&active_layout, "setPerspective", "(III)V", (near, far, angle)).await?;
                        primitive_state = None;
                    }
                }
                COMMAND_PERSPECTIVE_WH => {
                    if index + 3 < command_values.len() {
                        let near = command_values.get(index).copied().unwrap_or(1);
                        let far = command_values.get(index + 1).copied().unwrap_or(32767);
                        let width = command_values.get(index + 2).copied().unwrap_or(4096);
                        let height = command_values.get(index + 3).copied().unwrap_or(4096);
                        index += 4;
                        let _: () = jvm
                            .invoke_virtual(&active_layout, "setPerspective", "(IIII)V", (near, far, width, height))
                            .await?;
                        primitive_state = None;
                    }
                }
                COMMAND_ATTRIBUTE => effect_transparency_override = Some((command & ENV_ATTR_SEMI_TRANSPARENT) != 0),
                COMMAND_CLIP => {
                    if index + 3 < command_values.len() {
                        active_clip = Some((
                            command_values[index],
                            command_values[index + 1],
                            command_values[index + 2],
                            command_values[index + 3],
                        ));
                    }
                    index = index.saturating_add(4).min(command_values.len());
                }
                COMMAND_AMBIENT_LIGHT => index = index.saturating_add(1).min(command_values.len()),
                COMMAND_DIRECTION_LIGHT => index = index.saturating_add(4).min(command_values.len()),
                COMMAND_THRESHOLD => index = index.saturating_add(3).min(command_values.len()),
                PRIMITIVE_POINTS | PRIMITIVE_LINES | PRIMITIVE_TRIANGLES | PRIMITIVE_QUADS | PRIMITIVE_POINT_SPRITES => {
                    let num_primitives = ((command >> 16) & 0xff).max(0) as usize;
                    let vertices_per_primitive = match command_high {
                        PRIMITIVE_POINTS | PRIMITIVE_POINT_SPRITES => 1,
                        PRIMITIVE_LINES => 2,
                        PRIMITIVE_TRIANGLES => 3,
                        PRIMITIVE_QUADS => 4,
                        _ => 0,
                    };
                    if index >= command_values.len() {
                        break;
                    }
                    let offset_word_end = index + 1;
                    let vertex_count = num_primitives.saturating_mul(vertices_per_primitive);
                    let vertex_offset = command_values[index].max(0) as usize;

                    let normal_offset = vertex_offset.saturating_add(vertex_count.saturating_mul(3));
                    let normal_size = match command & PDATA_NORMAL_MASK {
                        PDATA_NORMAL_PER_FACE => num_primitives.saturating_mul(3),
                        PDATA_NORMAL_PER_VERTEX => vertex_count.saturating_mul(3),
                        _ => 0,
                    };

                    let texture_offset = normal_offset.saturating_add(normal_size);
                    let texture_size = if command_high == PRIMITIVE_POINT_SPRITES {
                        match command & PDATA_SPRITE_PARAMS_MASK {
                            PDATA_POINT_SPRITE_PARAMS_PER_CMD => 8,
                            0 => 0,
                            _ => num_primitives.saturating_mul(8),
                        }
                    } else if (command & PDATA_TEXTURE_COORD) != 0 {
                        vertex_count.saturating_mul(2)
                    } else {
                        0
                    };

                    let color_offset = texture_offset.saturating_add(texture_size);
                    let color_size = match command & PDATA_COLOR_MASK {
                        PDATA_COLOR_PER_COMMAND => 1,
                        PDATA_COLOR_PER_FACE => num_primitives,
                        _ => 0,
                    };
                    index = color_offset.saturating_add(color_size).max(offset_word_end).min(command_values.len());

                    let _ = normal_offset;
                    if primitive_state.is_none() {
                        primitive_state = Some(primitive_render_state(jvm, context, &active_layout, &effect, x, y).await?);
                    }
                    let state = primitive_state.as_ref().expect("primitive state initialized");
                    let prepare_start_ms = context.now();
                    let mut prepared = prepare_primitives_with_state(
                        jvm,
                        &texture,
                        state,
                        command,
                        num_primitives as i32,
                        &command_values,
                        vertex_offset,
                        &command_values,
                        texture_offset,
                        &command_values,
                        color_offset,
                    )
                    .await?;
                    prepared.prepare_ms = context.now().saturating_sub(prepare_start_ms);
                    if let Some(transparency) = effect_transparency_override {
                        prepared.effect_transparency = transparency;
                    }
                    prepared.clip = active_clip;
                    push_graphics_3d_queue(graphics_3d_key(&this), prepared);
                }
                _ => return Err(jvm.exception("java/lang/IllegalArgumentException", "Unsupported V3 command").await),
            }
        }

        Ok(())
    }
}

async fn clone_layout(jvm: &Jvm, _: &mut RuntimeContext, layout: &ClassInstanceRef<FigureLayout>) -> Result<ClassInstanceRef<FigureLayout>> {
    let mut cloned: ClassInstanceRef<FigureLayout> = jvm.new_class("com/mascotcapsule/micro3d/v3/FigureLayout", "()V", ()).await?.into();
    if layout.is_null() {
        return Ok(cloned);
    }

    let affine: ClassInstanceRef<super::AffineTrans> = jvm.get_field(layout, "affineTrans", "Lcom/mascotcapsule/micro3d/v3/AffineTrans;").await?;
    let _: () = jvm
        .invoke_virtual(&cloned, "setAffineTrans", "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V", (affine,))
        .await?;

    let affine_array: ClassInstanceRef<Array<super::AffineTrans>> = jvm
        .get_field(layout, "affineArray", "[Lcom/mascotcapsule/micro3d/v3/AffineTrans;")
        .await?;
    if !affine_array.is_null() {
        jvm.put_field(&mut cloned, "affineArray", "[Lcom/mascotcapsule/micro3d/v3/AffineTrans;", affine_array)
            .await?;
    }

    for field in [
        "centerX",
        "centerY",
        "near",
        "far",
        "perspective",
        "projection",
        "projectionMode",
        "parallelWidth",
        "parallelHeight",
        "scaleX",
        "scaleY",
    ] {
        let value: i32 = jvm.get_field(layout, field, "I").await?;
        jvm.put_field(&mut cloned, field, "I", value).await?;
    }

    Ok(cloned)
}

async fn load_i32_array(jvm: &Jvm, values: &ClassInstanceRef<Array<i32>>) -> Result<Vec<i32>> {
    if values.is_null() {
        return Ok(Vec::new());
    }
    let length = jvm.array_length(values).await?;
    jvm.load_array(values, 0, length).await
}
