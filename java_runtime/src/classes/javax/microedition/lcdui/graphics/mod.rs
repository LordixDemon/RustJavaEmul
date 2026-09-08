pub struct Graphics;

pub(super) const TARGET_IMAGE_FIELD: &str = "targetImage";
pub(super) const TARGET_IMAGE_DESC: &str = "Ljavax/microedition/lcdui/Image;";
pub(super) use draw_pixels::OffscreenTarget;

#[derive(Clone, Copy)]
pub(super) struct TextMetrics {
    pub(super) advance: i32,
    pub(super) height: i32,
    pub(super) baseline: i32,
    pub(super) scale: i32,
}

pub mod benchmark;
mod draw_pixels;
mod draw_shapes;
mod methods;
mod raster;
