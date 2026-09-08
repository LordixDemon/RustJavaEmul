use alloc::boxed::Box;
use std::io::Write;

use java_runtime::{RT_RUSTJAR, RuntimeClassDefine, get_runtime_class_proto};
use jvm::{ClassDefinition, Jvm};
use jvm_rust::{ArrayClassDefinitionImpl, ClassDefinitionImpl};

use super::RuntimeImpl;

#[async_trait::async_trait]
impl<T> RuntimeClassDefine for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    async fn find_rustjar_class(&self, _jvm: &Jvm, classpath: &str, class: &str) -> jvm::Result<Option<Box<dyn ClassDefinition>>> {
        if classpath == RT_RUSTJAR {
            if !self.allows_runtime_class(class) {
                return Ok(None);
            }
            let proto = get_runtime_class_proto(class);
            if let Some(proto) = proto {
                return Ok(Some(Box::new(ClassDefinitionImpl::from_class_proto(
                    proto,
                    Box::new(self.clone()) as Box<_>,
                ))));
            }
        }

        Ok(None)
    }

    async fn define_class(&self, jvm: &Jvm, data: &[u8]) -> jvm::Result<Box<dyn ClassDefinition>> {
        // One modded Treasure Towers classfile encodes screen height 320 as sipush 220's neighbour.
        // Keep the patch: removing it breaks that JAR.
        const CORRUPTED_MOD_PATTERN: &[u8] = &[
            0x59, 0x11, 0x00, 0xdb, 0x11, 0x0a, 0xa8, 0x4f, 0x59, 0x11, 0x01, 0x40, 0x11, 0x0a, 0xb4, 0x4f, 0x59, 0x11, 0x00, 0xdd,
        ];
        if let Some(pos) = data.windows(CORRUPTED_MOD_PATTERN.len()).position(|w| w == CORRUPTED_MOD_PATTERN) {
            tracing::warn!("Patching modder-corrupted sipush 320 -> 220 in class bytecode");
            let mut patched = data.to_vec();
            patched[pos + 10] = 0x00;
            patched[pos + 11] = 0xdc;
            match ClassDefinitionImpl::from_classfile(&patched) {
                Ok(x) => Ok(Box::new(x) as Box<_>),
                Err(message) => Err(jvm.exception("java/lang/ClassFormatError", message).await),
            }
        } else {
            match ClassDefinitionImpl::from_classfile(data) {
                Ok(x) => Ok(Box::new(x) as Box<_>),
                Err(message) => Err(jvm.exception("java/lang/ClassFormatError", message).await),
            }
        }
    }

    async fn define_array_class(&self, _jvm: &Jvm, element_type_name: &str) -> jvm::Result<Box<dyn ClassDefinition>> {
        Ok(Box::new(ArrayClassDefinitionImpl::new(element_type_name)))
    }

    fn allows_runtime_class(&self, name: &str) -> bool {
        self.device_profile().allows_class(name)
    }
}
