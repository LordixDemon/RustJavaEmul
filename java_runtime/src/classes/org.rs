pub mod netbeans;
pub mod rustjava;
pub mod w3c;
pub mod xml;

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    let mut factories = rustjava::class_protos();
    factories.extend(netbeans::class_protos());
    factories.extend(w3c::class_protos());
    factories.extend(xml::class_protos());
    factories
}
