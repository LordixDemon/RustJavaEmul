use alloc::string::String;
use core::fmt::Debug;

use java_constants::FieldAccessFlags;

use crate::as_any::AsAny;

use dyn_clone::DynClone;

pub trait Field: Sync + Send + AsAny + Debug + DynClone {
    fn name(&self) -> String;
    fn descriptor(&self) -> String;
    fn access_flags(&self) -> FieldAccessFlags;
}

dyn_clone::clone_trait_object!(Field);
