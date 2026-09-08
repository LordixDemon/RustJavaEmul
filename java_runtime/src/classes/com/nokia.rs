pub mod mid;

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    mid::class_protos()
}
