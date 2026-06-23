use alloc::{boxed::Box, string::String, sync::Arc};
use core::fmt::Debug;

use java_constants::MethodAccessFlags;

use crate::{JavaValue, Jvm, Result, as_any::AsAny};

#[async_trait::async_trait]
pub trait Method: Sync + Send + AsAny + Debug {
    fn name(&self) -> String;
    fn descriptor(&self) -> String;
    fn frame_name(&self) -> Arc<str>;
    fn access_flags(&self) -> MethodAccessFlags;

    async fn run(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue>;
}
