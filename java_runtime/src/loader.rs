use alloc::{boxed::Box, vec::Vec};

use hashbrown::HashMap;
use jvm::{BootstrapClassLoader, ClassDefinition, Jvm, Result};
use parking_lot::Mutex;

use crate::{RT_RUSTJAR, Runtime, RuntimeClassProto, RuntimeClassProtoFactory};

fn all_proto_factories() -> Vec<RuntimeClassProtoFactory> {
    let mut factories = Vec::new();
    factories.extend(crate::classes::com::class_protos());
    factories.extend(crate::classes::java::class_protos());
    factories.extend(crate::classes::javax::class_protos());
    factories.extend(crate::classes::org::class_protos());
    factories.extend(crate::classes::root::class_protos());
    factories
}

pub fn all_runtime_class_protos() -> Vec<RuntimeClassProto> {
    let mut protos = all_proto_factories().into_iter().map(|factory| factory()).collect();
    crate::coverage_stubs::merge_into(&mut protos);
    protos
}

pub fn all_real_runtime_class_protos() -> Vec<RuntimeClassProto> {
    all_proto_factories().into_iter().map(|factory| factory()).collect()
}

fn proto_index() -> HashMap<&'static str, RuntimeClassProtoFactory> {
    let mut map = HashMap::new();
    for factory in all_proto_factories() {
        let proto = factory();
        map.insert(proto.name, factory);
    }
    map
}

pub fn get_runtime_class_proto(name: &str) -> Option<RuntimeClassProto> {
    static INDEX: Mutex<Option<HashMap<&'static str, RuntimeClassProtoFactory>>> = Mutex::new(None);

    let factory = {
        let mut index = INDEX.lock();
        if index.is_none() {
            *index = Some(proto_index());
        }
        index.as_ref().and_then(|map| map.get(name).copied())
    };

    let mut proto = if let Some(factory) = factory {
        factory()
    } else {
        crate::coverage_stubs::class_proto(name)?
    };
    crate::coverage_stubs::apply_to(&mut proto);
    Some(proto)
}

struct JavaRuntimeClassLoader {
    runtime: Box<dyn Runtime>,
}

#[async_trait::async_trait]
impl BootstrapClassLoader for JavaRuntimeClassLoader {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Box<dyn ClassDefinition>>> {
        if let Some(element_type_name) = name.strip_prefix('[') {
            return Ok(Some(self.runtime.define_array_class(jvm, element_type_name).await?));
        }

        self.runtime.find_rustjar_class(jvm, RT_RUSTJAR, name).await
    }
}

pub fn get_bootstrap_class_loader(runtime: Box<dyn Runtime>) -> impl BootstrapClassLoader {
    JavaRuntimeClassLoader { runtime }
}
