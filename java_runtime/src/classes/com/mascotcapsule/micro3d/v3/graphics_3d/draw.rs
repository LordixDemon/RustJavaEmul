use super::super::{
    AffineTrans,
    effect::{Effect3D, Light},
    figure::Figure,
    layout::FigureLayout,
    queue::{graphics_3d_key, push_graphics_3d_queue},
    render::{prepare_native_figure, prepare_primitives, prepare_primitives_with_state, primitive_render_state},
    texture::Texture,
};
#[allow(unused_imports)]
use super::*;
use crate::RuntimeContext;
#[allow(unused_imports)]
use alloc::{vec, vec::Vec};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl Graphics3D {
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn draw_figure(
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
    pub(super) async fn render_figure(
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
    pub(super) async fn render_primitives(
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
    pub(super) async fn draw_command_list_single(
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
    pub(super) async fn draw_command_list_array(
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
    pub(super) async fn draw_command_list_impl(
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
        let mut active_effect = effect.clone();
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
                COMMAND_AMBIENT_LIGHT => {
                    let intensity = command_values.get(index).copied().unwrap_or(0);
                    index = index.saturating_add(1).min(command_values.len());
                    let light: ClassInstanceRef<Light> = jvm
                        .invoke_virtual(&active_effect, "getLight", "()Lcom/mascotcapsule/micro3d/v3/Light;", ())
                        .await
                        .unwrap_or_else(|_| ClassInstanceRef::new(None));
                    if !light.is_null() {
                        let _: () = jvm.invoke_virtual(&light, "setAmbIntensity", "(I)V", (intensity,)).await.unwrap_or(());
                    }
                }
                COMMAND_DIRECTION_LIGHT => {
                    let dir_x = command_values.get(index).copied().unwrap_or(0);
                    let dir_y = command_values.get(index + 1).copied().unwrap_or(0);
                    let dir_z = command_values.get(index + 2).copied().unwrap_or(4096);
                    let intensity = command_values.get(index + 3).copied().unwrap_or(4096);
                    index = index.saturating_add(4).min(command_values.len());
                    let direction = jvm
                        .new_class("com/mascotcapsule/micro3d/v3/Vector3D", "(III)V", (dir_x, dir_y, dir_z))
                        .await
                        .ok();
                    let light: ClassInstanceRef<Light> = jvm
                        .invoke_virtual(&active_effect, "getLight", "()Lcom/mascotcapsule/micro3d/v3/Light;", ())
                        .await
                        .unwrap_or_else(|_| ClassInstanceRef::new(None));
                    if !light.is_null() {
                        if let Some(direction) = direction {
                            let _: () = jvm
                                .invoke_virtual(&light, "setDirection", "(Lcom/mascotcapsule/micro3d/v3/Vector3D;)V", (direction,))
                                .await
                                .unwrap_or(());
                        }
                        let _: () = jvm.invoke_virtual(&light, "setDirIntensity", "(I)V", (intensity,)).await.unwrap_or(());
                    }
                    let _ = &mut active_effect;
                }
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

    let affine: ClassInstanceRef<AffineTrans> = jvm.get_field(layout, "affineTrans", "Lcom/mascotcapsule/micro3d/v3/AffineTrans;").await?;
    let _: () = jvm
        .invoke_virtual(&cloned, "setAffineTrans", "(Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V", (affine,))
        .await?;

    let affine_array: ClassInstanceRef<Array<AffineTrans>> = jvm
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
