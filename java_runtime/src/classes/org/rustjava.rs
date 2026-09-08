pub mod net;

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    net::class_protos().into_iter().collect()
}
