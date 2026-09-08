#![no_std]
extern crate alloc;

mod array_class_definition;
mod array_class_instance;
mod as_any;
mod class_definition;
mod class_instance;
mod class_loader;
mod error;
mod field;
mod garbage_collector;
mod invoke_arg;
mod jvm;
mod method;
mod thread;
mod r#type;
mod value;

pub mod runtime;

use alloc::boxed::Box;
use core::result;

pub type Result<T> = result::Result<T, error::JavaError>;

#[async_trait::async_trait]
pub trait JvmCallback: Sync + Send {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue>;
    fn call_sync(&self, _jvm: &Jvm, _args: &[JavaValue]) -> Option<Result<JavaValue>> {
        None
    }
}

pub use self::{
    array_class_definition::ArrayClassDefinition,
    array_class_instance::{ArrayClassInstance, ArrayRawBuffer, ArrayRawBufferMut},
    class_definition::ClassDefinition,
    class_instance::{Array, ClassInstance, ClassInstanceRef},
    class_loader::BootstrapClassLoader,
    error::JavaError,
    field::Field,
    jvm::{Jvm, ResolvedInstanceField, ResolvedMethod, ResolvedStaticField},
    method::Method,
    r#type::JavaType,
    value::{JavaChar, JavaValue},
};
