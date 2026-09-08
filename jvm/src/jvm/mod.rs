#![allow(clippy::borrowed_box)] // We have get parameter by Box<T> to make ergonomic interface

mod arrays;
mod fields;
mod invoke;
mod monitors;

use alloc::{borrow::ToOwned, boxed::Box, collections::BTreeMap, format, string::String, sync::Arc, vec::Vec};
use core::{
    fmt::Debug,
    sync::atomic::{AtomicBool, Ordering},
};

use dyn_clone::clone_box;
use event_listener::Event;
use hashbrown::{DefaultHashBuilder, HashSet};
use parking_lot::{Mutex, RwLock};

use java_constants::{ClassAccessFlags, MethodAccessFlags};

use crate::{
    Result,
    class_definition::ClassDefinition,
    class_instance::ClassInstance,
    class_loader::{BootstrapClassLoader, BootstrapClassLoaderWrapper, Class, ClassLoaderWrapper, InitState, JavaClassLoaderWrapper},
    error::JavaError,
    field::Field,
    garbage_collector::determine_reachable_objects,
    invoke_arg::InvokeArg,
    method::Method,
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
    thread::JvmThread,
    value::JavaValue,
};

struct JvmInner {
    classes: RwLock<BTreeMap<String, Class>>,
    threads: RwLock<BTreeMap<u64, JvmThread>>,
    all_objects: RwLock<HashSet<Box<dyn ClassInstance>>>,
    interned_strings: RwLock<BTreeMap<String, Box<dyn ClassInstance>>>,
    monitors: RwLock<BTreeMap<u64, Arc<ObjectMonitor>>>,
    monitor_hasher: DefaultHashBuilder,
    get_current_thread_id: Box<dyn Fn() -> u64 + Sync + Send>,
    bootstrap_class_loader: Box<dyn BootstrapClassLoader>,
    bootstrapping: AtomicBool,
    now_millis: RwLock<Option<Arc<dyn Fn() -> i64 + Sync + Send>>>,
}

#[derive(Clone)]
pub struct Jvm {
    inner: Arc<JvmInner>,
}

#[derive(Clone)]
pub struct ResolvedMethod {
    class: Class,
    method: Box<dyn Method>,
    frame_name: Arc<str>,
}

impl ResolvedMethod {
    pub fn run_sync(&self, jvm: &Jvm, args: &[JavaValue]) -> Option<crate::Result<JavaValue>> {
        self.method.run_sync(jvm, args)
    }
}

#[derive(Clone)]
pub struct ResolvedStaticField {
    class: Class,
    field: Box<dyn Field>,
}

#[derive(Clone)]
pub struct ResolvedInstanceField {
    class: Class,
    field: Box<dyn Field>,
    is_static: bool,
}

struct ResolvedField {
    class: Class,
    field: Box<dyn Field>,
}

#[derive(Clone, Debug)]
struct MissingStaticNoopMethod {
    name: String,
    descriptor: String,
    frame_name: Arc<str>,
}

impl MissingStaticNoopMethod {
    fn new(name: &str, descriptor: &str) -> Self {
        Self {
            name: name.to_owned(),
            descriptor: descriptor.to_owned(),
            frame_name: format!("{name}{descriptor}").into(),
        }
    }
}

#[async_trait::async_trait]
impl Method for MissingStaticNoopMethod {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn descriptor(&self) -> String {
        self.descriptor.clone()
    }

    fn frame_name(&self) -> Arc<str> {
        self.frame_name.clone()
    }

    fn access_flags(&self) -> MethodAccessFlags {
        MethodAccessFlags::STATIC
    }

    async fn run(&self, _: &Jvm, _: Box<[JavaValue]>) -> Result<JavaValue> {
        Ok(JavaValue::Void)
    }
}
struct ObjectMonitor {
    inner: Mutex<MonitorState>,
    event: Event,
}

struct MonitorState {
    owner: Option<u64>,
    count: u32,
}

impl ObjectMonitor {
    fn new() -> Self {
        Self {
            inner: Mutex::new(MonitorState { owner: None, count: 0 }),
            event: Event::new(),
        }
    }
}

impl Jvm {
    pub fn set_now_millis<F>(&self, now_millis: F)
    where
        F: Fn() -> i64 + Sync + Send + 'static,
    {
        *self.inner.now_millis.write() = Some(Arc::new(now_millis));
    }

    pub fn now_millis(&self) -> Option<i64> {
        self.inner.now_millis.read().as_ref().map(|now| now())
    }

    pub async fn new<C, F>(bootstrap_class_loader: C, get_current_thread_id: F, properties: BTreeMap<&str, &str>) -> Result<Self>
    where
        C: BootstrapClassLoader + 'static,
        F: Fn() -> u64 + 'static + Sync + Send,
    {
        let jvm = Self {
            inner: Arc::new(JvmInner {
                classes: RwLock::new(BTreeMap::new()),
                threads: RwLock::new(BTreeMap::new()),
                all_objects: RwLock::new(HashSet::new()),
                interned_strings: RwLock::new(BTreeMap::new()),
                monitors: RwLock::new(BTreeMap::new()),
                monitor_hasher: DefaultHashBuilder::default(),
                get_current_thread_id: Box::new(get_current_thread_id),
                bootstrap_class_loader: Box::new(bootstrap_class_loader),
                bootstrapping: AtomicBool::new(true),
                now_millis: RwLock::new(None),
            }),
        };

        // load bootstrap classes
        let bootstrap_classes = ["java/lang/Object", "java/lang/Thread", "[B", "java/lang/Class"];
        for class_name in bootstrap_classes.iter() {
            let class_definition = jvm.inner.bootstrap_class_loader.load_class(&jvm, class_name).await?.unwrap();
            let class = Class::new(class_definition, None);

            jvm.register_class_internal(class, None).await?;
        }

        // init startup thread
        jvm.attach_thread()?;

        // set java class for bootstrap classes
        let classes = jvm.inner.classes.read().values().cloned().collect::<Vec<_>>();
        for class in classes {
            let java_class = JavaLangClass::from_rust_class(&jvm, class.definition.clone(), None).await?;
            class.set_java_class(java_class);
        }

        // init properties
        for (key, value) in properties {
            let key = JavaLangString::from_rust_string(&jvm, key).await?;
            let value = JavaLangString::from_rust_string(&jvm, value).await?;

            let _: Option<Box<dyn ClassInstance>> = jvm
                .invoke_static(
                    "java/lang/System",
                    "setProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                    (key, value),
                )
                .await?;
        }

        // load system class loader
        JavaLangClassLoader::get_system_class_loader(&jvm).await?;

        jvm.inner.bootstrapping.store(false, Ordering::Relaxed);

        Ok(jvm)
    }

    #[async_recursion::async_recursion]
    pub async fn instantiate_class(&self, class_name: &str) -> Result<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate {}", class_name);

        let class = self.resolve_class(class_name).await?;

        let access_flags = class.definition.access_flags();
        if access_flags.contains(ClassAccessFlags::INTERFACE) || access_flags.contains(ClassAccessFlags::ABSTRACT) {
            return Err(self
                .exception(
                    "java/lang/InstantiationError",
                    &format!("Cannot instantiate abstract class or interface: {class_name}"),
                )
                .await);
        }

        self.ensure_initialized(&class).await?;

        let instance = class.definition.instantiate(self).await?;

        let thread_id = (self.inner.get_current_thread_id)();
        let mut threads = self.inner.threads.write();
        let thread = threads.get_mut(&thread_id).unwrap();

        thread.top_frame_mut().local_variables_mut().push(instance.clone());
        self.inner.all_objects.write().insert(instance.clone());

        Ok(instance)
    }

    pub async fn new_class<T>(&self, class_name: &str, init_descriptor: &str, init_args: T) -> Result<Box<dyn ClassInstance>>
    where
        T: InvokeArg,
    {
        let instance = self.instantiate_class(class_name).await?;

        let _: () = self.invoke_special(&instance, class_name, "<init>", init_descriptor, init_args).await?;

        Ok(instance)
    }

    pub fn destroy(&self, instance: Box<dyn ClassInstance>) -> Result<()> {
        tracing::debug!("Destroy {}", instance.class_definition().name());

        self.inner.all_objects.write().remove(&instance);
        instance.destroy();

        Ok(())
    }

    pub fn has_class(&self, class_name: &str) -> bool {
        self.inner.classes.read().contains_key(class_name)
    }

    pub fn get_class(&self, class_name: &str) -> Option<Class> {
        self.inner.classes.read().get(class_name).cloned()
    }

    pub fn set_current_frame_extra_roots(&self, roots: Vec<Box<dyn ClassInstance>>) {
        let thread_id = (self.inner.get_current_thread_id)();
        if let Some(thread) = self.inner.threads.write().get_mut(&thread_id) {
            thread.top_frame_mut().set_extra_roots(roots);
        }
    }

    #[async_recursion::async_recursion]
    pub async fn resolve_class(&self, class_name: &str) -> Result<Class> {
        self.resolve_class_internal(class_name, None).await
    }

    #[async_recursion::async_recursion]
    async fn resolve_class_internal(&self, class_name: &str, class_loader_wrapper: Option<&dyn ClassLoaderWrapper>) -> Result<Class> {
        tracing::trace!("Resolving class {}", class_name);
        let class = self.inner.classes.read().get(class_name).cloned();

        if let Some(x) = class {
            return Ok(x);
        }

        if class_name.starts_with('[') {
            let stripped_name = class_name.trim_start_matches('[');
            if stripped_name.starts_with('L') {
                self.resolve_class(&stripped_name[1..stripped_name.len() - 1]).await?;
                // ensure element type is loaded
            }
        }

        let class_loader_wrapper: &dyn ClassLoaderWrapper = if let Some(x) = class_loader_wrapper {
            x
        } else if self.inner.bootstrapping.load(Ordering::Relaxed) {
            &BootstrapClassLoaderWrapper::new(&*self.inner.bootstrap_class_loader)
        } else {
            &JavaClassLoaderWrapper::new(self.current_class_loader().await?)
        };

        let class = self.load_class(class_name, class_loader_wrapper).await?;

        // loader wrappers may build a fresh Class around an already-registered definition,
        // so return the registry copy to keep init state shared
        if let Some(registered) = self.get_class(class_name) {
            return Ok(registered);
        }

        Ok(class)
    }

    async fn load_class(&self, class_name: &str, class_loader_wrapper: &dyn ClassLoaderWrapper) -> Result<Class> {
        tracing::debug!("Loading class {}", class_name);

        let class = class_loader_wrapper.load_class(self, class_name).await?;

        if class.is_none() {
            tracing::error!("No such class: {}", class_name);

            return Err(self.exception("java/lang/NoClassDefFoundError", class_name).await);
        }

        tracing::debug!("Loaded class {}", class_name);

        Ok(class.unwrap())
    }

    #[allow(clippy::type_complexity)]
    fn find_calling_class(&self) -> Result<Option<(Class, Option<Box<dyn ClassInstance>>)>> {
        let thread_id = (self.inner.get_current_thread_id)();

        let threads = self.inner.threads.read();
        let thread = threads.get(&thread_id).unwrap();

        Ok(thread.top_java_frame().map(|x| (x.class.clone(), x.class_instance.clone())))
    }

    pub async fn register_class(
        &self,
        class: Box<dyn ClassDefinition>,
        class_loader: Option<Box<dyn ClassInstance>>,
    ) -> Result<Option<Box<dyn ClassInstance>>> {
        let class_name = class.name();
        if let Some(existing) = self.inner.classes.read().get(&class_name).cloned() {
            return Ok(Some(existing.java_class()));
        }

        tracing::debug!("Register class {}", class_name);

        let java_class = Some(JavaLangClass::from_rust_class(self, class.clone(), class_loader.clone()).await?);

        let class = Class::new(class, java_class.clone());

        if let Some(x) = class_loader {
            self.register_class_internal(class, Some(&JavaClassLoaderWrapper::new(x))).await?;
        } else {
            self.register_class_internal(class, None).await?;
        };

        if let Some(existing) = self.inner.classes.read().get(&class_name) {
            return Ok(Some(existing.java_class()));
        }

        Ok(java_class)
    }

    pub fn is_instance(&self, instance: &dyn ClassInstance, class_name: &str) -> bool {
        let class = instance.class_definition();

        self.is_inherited_from(&*class, class_name)
    }

    pub fn is_inherited_from(&self, class: &dyn ClassDefinition, class_name: &str) -> bool {
        if class.name() == class_name {
            return true;
        }

        let this_name = class.name();
        if this_name.starts_with('[') {
            if class_name == "java/lang/Object" || class_name == "java/lang/Cloneable" || class_name == "java/io/Serializable" {
                return true;
            }
            if class_name.starts_with('[') {
                return self.array_is_subtype(&this_name, class_name);
            }
            return false;
        }

        for interface in class.interface_names() {
            if interface == class_name {
                return true;
            }

            if let Some(interface_class) = self.inner.classes.read().get(&interface).cloned() {
                if self.is_inherited_from(&*interface_class.definition, class_name) {
                    return true;
                }
            }
        }

        if let Some(super_class) = class.super_class_name() {
            if let Some(super_class) = self.inner.classes.read().get(&super_class).cloned() {
                self.is_inherited_from(&*super_class.definition, class_name)
            } else {
                false
            }
        } else {
            false
        }
    }
    fn array_is_subtype(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }
        if !from.starts_with('[') || !to.starts_with('[') {
            return false;
        }
        let from_el = &from[1..];
        let to_el = &to[1..];
        if Self::is_primitive_array_component(from_el) || Self::is_primitive_array_component(to_el) {
            return false;
        }
        self.reference_descriptor_is_subtype(from_el, to_el)
    }

    fn is_primitive_array_component(descriptor: &str) -> bool {
        matches!(descriptor, "B" | "C" | "D" | "F" | "I" | "J" | "S" | "Z")
    }

    fn reference_descriptor_is_subtype(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }
        if matches!(
            to,
            "Ljava/lang/Object;"
                | "Ljava/lang/Cloneable;"
                | "Ljava/io/Serializable;"
                | "java/lang/Object"
                | "java/lang/Cloneable"
                | "java/io/Serializable"
        ) {
            return true;
        }
        if from.starts_with('[') && to.starts_with('[') {
            return self.array_is_subtype(from, to);
        }
        if from.starts_with('L') && to.starts_with('L') {
            let from_cn = from.trim_start_matches('L').trim_end_matches(';');
            let to_cn = to.trim_start_matches('L').trim_end_matches(';');
            return self.class_name_is_subtype(from_cn, to_cn);
        }
        if from.starts_with('[') && to.starts_with('L') {
            let to_cn = to.trim_start_matches('L').trim_end_matches(';');
            return matches!(to_cn, "java/lang/Object" | "java/lang/Cloneable" | "java/io/Serializable");
        }
        false
    }

    fn class_name_is_subtype(&self, from_cn: &str, to_cn: &str) -> bool {
        if from_cn == to_cn {
            return true;
        }
        if let Some(from_class) = self.get_class(from_cn) {
            self.is_inherited_from(&*from_class.definition, to_cn)
        } else {
            false
        }
    }
    pub async fn exception(&self, r#type: &str, message: &str) -> JavaError {
        tracing::info!("throwing java exception: {} {}", r#type, message);
        tracing::debug!("java stack trace:\n{}", self.stack_trace().join("\n"));

        if let Ok(error) = self.try_exception(r#type, message).await {
            return error;
        }
        if r#type != "java/lang/InternalError" {
            if let Ok(error) = self.try_exception("java/lang/InternalError", message).await {
                return error;
            }
        }
        if let Ok(error) = self.try_exception_no_message("java/lang/Error").await {
            return error;
        }
        match self.try_exception_no_message("java/lang/Throwable").await {
            Ok(error) | Err(error) => error,
        }
    }

    async fn try_exception(&self, r#type: &str, message: &str) -> Result<JavaError> {
        let message_str = JavaLangString::from_rust_string(self, message).await?;
        let instance = self.new_class(r#type, "(Ljava/lang/String;)V", (message_str,)).await?;
        Ok(JavaError::JavaException(instance))
    }

    async fn try_exception_no_message(&self, r#type: &str) -> Result<JavaError> {
        let instance = self.new_class(r#type, "()V", ()).await?;
        Ok(JavaError::JavaException(instance))
    }

    pub fn stack_trace(&self) -> Vec<String> {
        // TODO we should return in another format

        let thread_id = (self.inner.get_current_thread_id)();
        let threads = self.inner.threads.read();
        let Some(thread) = threads.get(&thread_id) else {
            return Vec::new();
        };

        thread
            .iter_java_frame()
            .rev()
            .filter_map(|x| {
                // skip exception classes
                if self.is_inherited_from(&*x.class.definition, "java/lang/Throwable") {
                    None
                } else {
                    Some(format!("{}.{}", x.class.definition.name(), x.method.as_ref()))
                }
            })
            .collect()
    }

    pub fn stack_trace_all_threads(&self) -> Vec<String> {
        let threads = self.inner.threads.read();
        threads
            .iter()
            .map(|(thread_id, thread)| {
                let frames = thread
                    .iter_java_frame()
                    .rev()
                    .filter_map(|x| {
                        if self.is_inherited_from(&*x.class.definition, "java/lang/Throwable") {
                            None
                        } else {
                            Some(format!("{}.{}", x.class.definition.name(), x.method.as_ref()))
                        }
                    })
                    .take(4)
                    .collect::<Vec<_>>()
                    .join("<");
                format!("{thread_id}:{frames}")
            })
            .collect()
    }

    pub fn interned_string(&self, key: &str) -> Option<Box<dyn ClassInstance>> {
        self.inner.interned_strings.read().get(key).cloned()
    }

    pub fn intern_string_instance(&self, key: String, instance: Box<dyn ClassInstance>) -> Box<dyn ClassInstance> {
        let mut interned = self.inner.interned_strings.write();
        if let Some(existing) = interned.get(&key) {
            return existing.clone();
        }
        interned.insert(key, instance.clone());
        instance
    }

    pub fn collect_garbage(&self) -> Result<usize> {
        tracing::trace!("Collecting garbage");

        let reachable_objects = {
            let threads = self.inner.threads.read();
            let classes = self.inner.classes.read();
            let interned_strings = self.inner.interned_strings.read();

            determine_reachable_objects(self, &threads, &classes, &interned_strings)
        };

        let mut all_objects = self.inner.all_objects.write();
        let before_count = all_objects.len();
        all_objects.retain(|object| reachable_objects.contains(object));
        let garbage_count = before_count - all_objects.len();

        tracing::trace!("Garbage count: {}", garbage_count);

        Ok(garbage_count)
    }

    async fn register_class_internal(&self, class: Class, class_loader_wrapper: Option<&dyn ClassLoaderWrapper>) -> Result<()> {
        if self.has_class(&class.definition.name()) {
            return Ok(());
        }

        if !class.definition.name().starts_with('[') {
            // ensure superclass and superinterfaces are loaded
            if let Some(super_class) = class.definition.super_class_name()
                && !self.has_class(&super_class)
            {
                self.resolve_class_internal(&super_class, class_loader_wrapper).await?;
            }

            for interface in class.definition.interface_names() {
                if !self.has_class(&interface) {
                    self.resolve_class_internal(&interface, class_loader_wrapper).await?;
                }
            }
        }

        let mut classes = self.inner.classes.write();
        if !classes.contains_key(&class.definition.name()) {
            classes.insert(class.definition.name().to_owned(), class);
        }

        Ok(())
    }

    #[async_recursion::async_recursion]
    async fn ensure_initialized(&self, class: &Class) -> Result<()> {
        if class.definition.name().starts_with('[') {
            return Ok(());
        }

        let current_thread_id = (self.inner.get_current_thread_id)();

        loop {
            enum Action {
                AlreadyInitialized,
                Erroneous,
                Wait(event_listener::EventListener),
                StartInit,
            }

            let action = {
                let mut state = class.init_state.write();
                match *state {
                    InitState::Initialized => Action::AlreadyInitialized,
                    InitState::Erroneous => Action::Erroneous,
                    InitState::InProgress(thread_id) => {
                        if thread_id == current_thread_id {
                            Action::AlreadyInitialized
                        } else {
                            Action::Wait(class.init_event.listen())
                        }
                    }
                    InitState::NotInitialized => {
                        *state = InitState::InProgress(current_thread_id);
                        Action::StartInit
                    }
                }
            };

            match action {
                Action::AlreadyInitialized => return Ok(()),
                Action::Erroneous => {
                    return Err(self
                        .exception(
                            "java/lang/NoClassDefFoundError",
                            &format!("Could not initialize class {}", class.definition.name()),
                        )
                        .await);
                }
                Action::Wait(listener) => listener.await,
                Action::StartInit => break,
            }
        }

        if let Some(super_name) = class.definition.super_class_name() {
            // resolution failure is not an initialization failure, so initialization may be retried
            let super_class = match self.resolve_class(&super_name).await {
                Ok(x) => x,
                Err(err) => {
                    class.set_init_state(InitState::NotInitialized);
                    class.init_event.notify(usize::MAX);
                    return Err(err);
                }
            };

            if let Err(err) = self.ensure_initialized(&super_class).await {
                class.set_init_state(InitState::Erroneous);
                class.init_event.notify(usize::MAX);
                return Err(err);
            }
        }

        if let Err(err) = class.definition.prepare(self).await {
            class.set_init_state(InitState::Erroneous);
            class.init_event.notify(usize::MAX);
            return Err(err);
        }

        if let Some(clinit) = class.definition.method("<clinit>", "()V", true) {
            tracing::debug!("Calling <clinit> for {}", class.definition.name());

            if let Err(err) = self.execute_method(class, None, &clinit, Box::new([])).await {
                class.set_init_state(InitState::Erroneous);
                class.init_event.notify(usize::MAX);

                let JavaError::JavaException(exception) = &err;
                if self.is_instance(&**exception, "java/lang/Error") {
                    return Err(err);
                }

                let cause = clone_box(&**exception);
                let wrapped = self
                    .new_class("java/lang/ExceptionInInitializerError", "(Ljava/lang/Throwable;)V", (cause,))
                    .await?;

                return Err(JavaError::JavaException(wrapped));
            }
        }

        class.set_init_state(InitState::Initialized);
        class.init_event.notify(usize::MAX);

        Ok(())
    }

    pub fn attach_thread(&self) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().insert(thread_id, JvmThread::new());
        self.push_native_frame();

        Ok(())
    }

    pub fn detach_thread(&self) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().remove(&thread_id);

        Ok(())
    }

    // TODO we need safe, ergonomic api..
    pub fn push_native_frame(&self) {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().get_mut(&thread_id).unwrap().push_native_frame();
    }

    pub fn pop_frame(&self) {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner.threads.write().get_mut(&thread_id).unwrap().pop_frame();
    }

    pub async fn current_class_loader(&self) -> Result<Box<dyn ClassInstance>> {
        let calling_class = self.find_calling_class()?;

        if let Some((class, class_instance)) = calling_class {
            // called in java

            if self.is_inherited_from(&*class.definition, "java/lang/ClassLoader") {
                return Ok(class_instance.unwrap());
            }

            let calling_class_class_loader = JavaLangClass::class_loader(self, &class.java_class()).await?;
            if let Some(x) = calling_class_class_loader {
                Ok(x)
            } else {
                let system_class_loader = JavaLangClassLoader::get_system_class_loader(self).await?;
                Ok(system_class_loader)
            }
        } else {
            // called outside of java

            let system_class_loader = JavaLangClassLoader::get_system_class_loader(self).await?;
            Ok(system_class_loader)
        }
    }

    pub fn dump_threads(&self) -> Vec<(u64, Vec<String>)> {
        let threads = self.inner.threads.read();
        threads
            .iter()
            .map(|(&tid, thread)| {
                let frames = thread
                    .iter_java_frame()
                    .map(|f| format!("{}.{}", f.class.definition.name(), f.method))
                    .collect();
                (tid, frames)
            })
            .collect()
    }
}
