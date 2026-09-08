pub mod sound;
pub mod ui;

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    let mut factories = alloc::vec::Vec::new();
    factories.extend(sound::class_protos());
    factories.extend(ui::class_protos());
    factories
}
