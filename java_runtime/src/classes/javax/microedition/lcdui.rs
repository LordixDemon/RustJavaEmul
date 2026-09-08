mod alert;
mod canvas;
mod command;
mod command_listener;
mod display;
mod displayable;
mod font;
mod form;
mod game;
mod gpu_image;
mod graphics;
mod image;
mod image_item;
mod item;
mod item_command_listener;
mod item_state_listener;
mod list;
mod screen;
mod string_item;
mod text_box;
mod text_field;
mod widgets;

pub use self::{
    alert::Alert,
    canvas::Canvas,
    command::Command,
    command_listener::CommandListener,
    display::Display,
    displayable::Displayable,
    font::Font,
    form::Form,
    game::{GameCanvas, Layer, LayerManager, Sprite, TiledLayer},
    graphics::Graphics,
    graphics::benchmark::{LcdUiBenchmark, LcdUiBenchmarkConfig, LcdUiBenchmarkRun},
    image::Image,
    image_item::ImageItem,
    item::Item,
    item_command_listener::ItemCommandListener,
    item_state_listener::ItemStateListener,
    list::List,
    screen::Screen,
    string_item::StringItem,
    text_box::TextBox,
    text_field::TextField,
    widgets::{AlertType, Choice, ChoiceGroup, CustomItem, DateField, Gauge, Spacer, Ticker},
};

pub(crate) use canvas::pace_game_frame;
pub(crate) use gpu_image::{invalidate_gpu_image, publish_gpu_image_to_screen, register_gpu_image_hooks};

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        Alert,
        AlertType,
        Canvas,
        Choice,
        ChoiceGroup,
        Command,
        CommandListener,
        CustomItem,
        DateField,
        Display,
        Displayable,
        Font,
        Form,
        GameCanvas,
        Gauge,
        Graphics,
        Image,
        ImageItem,
        Item,
        ItemCommandListener,
        ItemStateListener,
        Layer,
        LayerManager,
        List,
        Screen,
        Spacer,
        Sprite,
        StringItem,
        TextBox,
        TextField,
        Ticker,
        TiledLayer,
    ]
}
