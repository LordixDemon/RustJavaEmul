pub struct Light;
pub struct GraphicObject;
pub struct Sprite;
pub struct Melody;
pub struct Sound;
pub struct Vibrator;
pub struct Image;

mod extras;
#[path = "game.rs"]
mod game_impl;
#[path = "ui.rs"]
mod ui_impl;

pub use extras::*;

pub mod game {
    pub use super::{GraphicObject, Light, Melody, Sound, Sprite, Vibrator};
}

pub mod ui {
    pub use super::Image;
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        Light,
        Vibrator,
        Sound,
        Melody,
        GraphicObject,
        Sprite,
        Image,
        FileConnection,
        SiemensCommand,
        SiemensImage,
        SiemensManager,
        SiemensMediaException,
        SiemensPlayer,
        SiemensResource,
        SiemensMessageConnection,
        MessagePart,
        MultipartMessage,
    ]
}
