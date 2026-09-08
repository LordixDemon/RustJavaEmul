pub mod lg;
pub mod mascotcapsule;
pub mod motorola;
pub mod nokia;
pub mod samsung;
pub mod siemens;
pub mod sonyericsson;
pub mod sprintpcs;
pub mod vodafone;

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    let mut factories = alloc::vec::Vec::new();
    factories.extend(mascotcapsule::micro3d::v3::class_protos());
    factories.extend(nokia::class_protos());
    factories.extend(siemens::class_protos());
    factories.extend(samsung::class_protos());
    factories.extend(motorola::class_protos());
    factories.extend(lg::class_protos());
    factories.extend(sonyericsson::class_protos());
    factories.extend(vodafone::class_protos());
    factories.extend(sprintpcs::class_protos());
    factories
}
