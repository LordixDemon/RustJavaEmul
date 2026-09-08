#![no_std]
extern crate alloc;

#[macro_use]
mod macros;

pub mod classes;
mod coverage_stubs;
mod loader;
mod paths;
pub mod profile;
mod runtime;
pub mod zip;

pub use self::{
    loader::{all_real_runtime_class_protos, all_runtime_class_protos, get_bootstrap_class_loader, get_runtime_class_proto},
    paths::{filesystem_path_candidates, resource_path_candidates},
    profile::{DeviceProfile, KeyLayout, detect_device_profile},
    runtime::{
        DecodedImage, File, FileDescriptorId, FileSize, FileStat, FileType, IOError, IOResult, Runtime, RuntimeClassDefine, RuntimeClock, RuntimeFs,
        RuntimeNet, RuntimeScreen, RuntimeStore, SpawnCallback,
    },
};

pub type RuntimeContext = dyn runtime::Runtime;
pub type RuntimeClassProto = java_class_proto::JavaClassProto<dyn runtime::Runtime>;
pub type RuntimeClassProtoFactory = fn() -> RuntimeClassProto;

pub static RT_RUSTJAR: &str = "rt.rustjar";
