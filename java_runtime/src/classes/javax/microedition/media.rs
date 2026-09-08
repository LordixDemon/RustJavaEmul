mod control;
mod controllable;
mod exception;
mod manager;
mod player;
mod player_listener;

pub use self::{
    control::*, controllable::Controllable, exception::MediaException, manager::Manager, player::Player, player_listener::PlayerListener,
};

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        Control,
        Controllable,
        Manager,
        MediaException,
        MIDIControl,
        MetaDataControl,
        PitchControl,
        Player,
        PlayerListener,
        StopTimeControl,
        TempoControl,
        ToneControl,
        VideoControl,
        VolumeControl,
    ]
}
