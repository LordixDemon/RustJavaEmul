#![no_std]
extern crate alloc;

mod array_class_definition;
mod array_class_instance;
mod class_definition;
mod class_instance;
mod field;
mod interpreter;
mod method;
mod profile;
mod stack_frame;

pub use self::{
    array_class_definition::ArrayClassDefinitionImpl,
    class_definition::ClassDefinitionImpl,
    field::FieldImpl,
    method::{MethodBody, MethodImpl},
    profile::{
        ARRAY_TYPE_COUNT, ProfileSnapshot, intrinsic_report_and_reset, method_opcode_report_and_reset, reset as reset_profile,
        set_enabled as set_profile_enabled, snapshot as profile_snapshot,
    },
};
