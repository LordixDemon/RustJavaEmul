// class javax.microedition.lcdui.game.GameCanvas
pub struct GameCanvas;
pub struct Sprite;
pub struct Layer;
pub struct TiledLayer;
pub struct LayerManager;

pub(super) const BACK_BUFFER_FIELD: &str = "backBuffer";
pub(super) const BACK_BUFFER_DESC: &str = "Ljavax/microedition/lcdui/Image;";
pub(super) const BACK_GRAPHICS_FIELD: &str = "backGraphics";
pub(super) const BACK_GRAPHICS_DESC: &str = "Ljavax/microedition/lcdui/Graphics;";

mod game_canvas;
mod layer;
mod layer_manager;
mod sprite;
mod tiled_layer;
