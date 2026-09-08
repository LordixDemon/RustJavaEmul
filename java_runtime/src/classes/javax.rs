pub mod bluetooth;
pub mod microedition;
pub mod obex;
pub mod wireless;
pub mod xml;

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    let mut factories = alloc::vec::Vec::new();
    factories.extend(microedition::class_protos());
    factories.extend(bluetooth::class_protos());
    factories.extend(obex::class_protos());
    factories.extend(wireless::class_protos());
    factories.extend(xml::class_protos());
    factories
}
