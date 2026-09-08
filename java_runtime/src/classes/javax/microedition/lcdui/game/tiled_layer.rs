#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::javax::microedition::lcdui::{Graphics, Image},
};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

impl TiledLayer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/game/TiledLayer",
            parent_class: Some("javax/microedition/lcdui/game/Layer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(IILjavax/microedition/lcdui/Image;II)V", Self::init, Default::default()),
                JavaMethodProto::new("createAnimatedTile", "(I)I", Self::create_animated_tile, Default::default()),
                JavaMethodProto::new("fillCells", "(IIIII)V", Self::fill_cells, Default::default()),
                JavaMethodProto::new("getAnimatedTile", "(I)I", Self::get_animated_tile, Default::default()),
                JavaMethodProto::new("getCell", "(II)I", Self::get_cell, Default::default()),
                JavaMethodProto::new("getCellHeight", "()I", Self::get_cell_height, Default::default()),
                JavaMethodProto::new("getCellWidth", "()I", Self::get_cell_width, Default::default()),
                JavaMethodProto::new("getColumns", "()I", Self::get_columns, Default::default()),
                JavaMethodProto::new("getRows", "()I", Self::get_rows, Default::default()),
                JavaMethodProto::new("paint", "(Ljavax/microedition/lcdui/Graphics;)V", Self::paint, Default::default()),
                JavaMethodProto::new("setAnimatedTile", "(II)V", Self::set_animated_tile, Default::default()),
                JavaMethodProto::new("setCell", "(III)V", Self::set_cell, Default::default()),
                JavaMethodProto::new(
                    "setStaticTileSet",
                    "(Ljavax/microedition/lcdui/Image;II)V",
                    Self::set_static_tile_set,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("image", "Ljavax/microedition/lcdui/Image;", Default::default()),
                JavaFieldProto::new("columns", "I", Default::default()),
                JavaFieldProto::new("rows", "I", Default::default()),
                JavaFieldProto::new("cellWidth", "I", Default::default()),
                JavaFieldProto::new("cellHeight", "I", Default::default()),
                JavaFieldProto::new("cells", "[I", Default::default()),
                JavaFieldProto::new("animated", "[I", Default::default()),
                JavaFieldProto::new("animatedCount", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        columns: i32,
        rows: i32,
        image: ClassInstanceRef<Image>,
        tile_width: i32,
        tile_height: i32,
    ) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/game/Layer", "<init>", "()V", ())
            .await?;
        let columns = columns.max(1);
        let rows = rows.max(1);
        let cells = jvm.instantiate_array("I", (columns * rows) as usize).await?;
        let animated = jvm.instantiate_array("I", 16).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "columns", "I", columns).await?;
        jvm.put_field(&mut this, "rows", "I", rows).await?;
        jvm.put_field(&mut this, "cellWidth", "I", tile_width.max(1)).await?;
        jvm.put_field(&mut this, "cellHeight", "I", tile_height.max(1)).await?;
        jvm.put_field(&mut this, "cells", "[I", cells).await?;
        jvm.put_field(&mut this, "animated", "[I", animated).await?;
        jvm.put_field(&mut this, "animatedCount", "I", 0).await?;
        jvm.put_field(&mut this, "width", "I", columns * tile_width.max(1)).await?;
        jvm.put_field(&mut this, "height", "I", rows * tile_height.max(1)).await
    }

    pub(super) async fn create_animated_tile(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        static_tile_index: i32,
    ) -> Result<i32> {
        let mut count: i32 = jvm.get_field(&this, "animatedCount", "I").await?;
        let mut animated = jvm.get_field(&this, "animated", "[I").await?;
        let len = jvm.array_length(&animated).await? as i32;
        if count >= len {
            let mut bigger = jvm.instantiate_array("I", (len * 2) as usize).await?;
            let values: alloc::vec::Vec<i32> = jvm.load_array(&animated, 0, len as usize).await?;
            jvm.store_array(&mut bigger, 0, values).await?;
            animated = bigger;
            jvm.put_field(&mut this, "animated", "[I", animated.clone()).await?;
        }
        jvm.store_array(&mut animated, count as usize, alloc::vec![static_tile_index]).await?;
        count += 1;
        jvm.put_field(&mut this, "animatedCount", "I", count).await?;
        Ok(-count)
    }

    pub(super) async fn fill_cells(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        col: i32,
        row: i32,
        num_cols: i32,
        num_rows: i32,
        tile: i32,
    ) -> Result<()> {
        let columns: i32 = jvm.get_field(&this, "columns", "I").await?;
        let rows: i32 = jvm.get_field(&this, "rows", "I").await?;
        let mut cells = jvm.get_field(&this, "cells", "[I").await?;
        for r in row..(row + num_rows).min(rows) {
            for c in col..(col + num_cols).min(columns) {
                if r >= 0 && c >= 0 {
                    jvm.store_array(&mut cells, (r * columns + c) as usize, alloc::vec![tile]).await?;
                }
            }
        }
        Ok(())
    }

    pub(super) async fn get_animated_tile(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, animated_tile_index: i32) -> Result<i32> {
        let index = (-animated_tile_index - 1).max(0) as usize;
        let animated = jvm.get_field(&this, "animated", "[I").await?;
        Ok(jvm.load_array(&animated, index, 1).await?.into_iter().next().unwrap_or(0))
    }

    pub(super) async fn get_cell(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, col: i32, row: i32) -> Result<i32> {
        let columns: i32 = jvm.get_field(&this, "columns", "I").await?;
        let cells = jvm.get_field(&this, "cells", "[I").await?;
        Ok(jvm
            .load_array(&cells, (row * columns + col) as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or(0))
    }

    pub(super) async fn get_cell_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cellHeight", "I").await
    }
    pub(super) async fn get_cell_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cellWidth", "I").await
    }
    pub(super) async fn get_columns(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "columns", "I").await
    }
    pub(super) async fn get_rows(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "rows", "I").await
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
        let visible: bool = jvm.get_field(&this, "visible", "Z").await?;
        if !visible {
            return Ok(());
        }
        let image: ClassInstanceRef<Image> = jvm.get_field(&this, "image", "Ljavax/microedition/lcdui/Image;").await?;
        if image.is_null() {
            return Ok(());
        }
        let columns: i32 = jvm.get_field(&this, "columns", "I").await?;
        let rows: i32 = jvm.get_field(&this, "rows", "I").await?;
        let cell_w: i32 = jvm.get_field(&this, "cellWidth", "I").await?;
        let cell_h: i32 = jvm.get_field(&this, "cellHeight", "I").await?;
        let origin_x: i32 = jvm.get_field(&this, "x", "I").await?;
        let origin_y: i32 = jvm.get_field(&this, "y", "I").await?;
        let (image_width, _, _) = Image::pixels(jvm, &image).await?;
        let tiles_per_row = (image_width / cell_w).max(1);
        for row in 0..rows {
            for col in 0..columns {
                let mut tile = Self::get_cell(jvm, context, this.clone(), col, row).await?;
                if tile < 0 {
                    tile = Self::get_animated_tile(jvm, context, this.clone(), tile).await?;
                }
                if tile <= 0 {
                    continue;
                }
                let index = tile - 1;
                let src_x = (index % tiles_per_row) * cell_w;
                let src_y = (index / tiles_per_row) * cell_h;
                Graphics::draw_region(
                    jvm,
                    context,
                    graphics.clone(),
                    image.clone(),
                    src_x,
                    src_y,
                    cell_w,
                    cell_h,
                    0,
                    origin_x + col * cell_w,
                    origin_y + row * cell_h,
                    4 | 16,
                )
                .await?;
            }
        }
        Ok(())
    }

    pub(super) async fn set_animated_tile(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        animated_tile_index: i32,
        static_tile_index: i32,
    ) -> Result<()> {
        let index = (-animated_tile_index - 1).max(0) as usize;
        let mut animated = jvm.get_field(&this, "animated", "[I").await?;
        jvm.store_array(&mut animated, index, alloc::vec![static_tile_index]).await
    }

    pub(super) async fn set_cell(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, col: i32, row: i32, tile: i32) -> Result<()> {
        let columns: i32 = jvm.get_field(&this, "columns", "I").await?;
        let mut cells = jvm.get_field(&this, "cells", "[I").await?;
        jvm.store_array(&mut cells, (row * columns + col) as usize, alloc::vec![tile]).await
    }

    pub(super) async fn set_static_tile_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        tile_width: i32,
        tile_height: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "cellWidth", "I", tile_width.max(1)).await?;
        jvm.put_field(&mut this, "cellHeight", "I", tile_height.max(1)).await
    }
}
