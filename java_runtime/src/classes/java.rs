pub mod io;
pub mod lang;
pub mod net;
pub mod nio;
pub mod security;
pub mod util;

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    let mut factories = alloc::vec::Vec::new();
    factories.extend(io::class_protos());
    factories.extend(security::class_protos());
    factories.extend(lang::class_protos());
    factories.extend(net::class_protos());
    factories.extend(nio::class_protos());
    factories.extend(util::class_protos());
    factories
}
