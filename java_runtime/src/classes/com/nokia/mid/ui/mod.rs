pub struct DirectGraphics;
pub struct FullCanvas;
pub struct DirectUtils;
pub struct DeviceControl;

mod device_control;
mod direct_graphics;
mod direct_utils;
mod full_canvas;

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![DeviceControl, DirectGraphics, DirectUtils, FullCanvas]
}
