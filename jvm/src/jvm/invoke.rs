use alloc::{boxed::Box, format, sync::Arc, vec::Vec};
use core::iter;

use dyn_clone::clone_box;
use java_constants::MethodAccessFlags;

use crate::{
    Result, class_definition::ClassDefinition, class_instance::ClassInstance, class_loader::Class, invoke_arg::InvokeArg, method::Method,
    value::JavaValue,
};

use super::{Jvm, MissingStaticNoopMethod, ResolvedMethod};

impl Jvm {
    pub async fn resolve_static_method(&self, class_name: &str, name: &str, descriptor: &str) -> Result<ResolvedMethod> {
        let class = self.resolve_class(class_name).await?;

        let mut current_class = class.clone();
        let (declaring_class, method) = loop {
            if let Some(method) = current_class.definition.method(name, descriptor, true) {
                if !method.access_flags().contains(MethodAccessFlags::STATIC) {
                    return Err(self
                        .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                        .await);
                }
                break (current_class, method);
            }

            if let Some(super_name) = current_class.definition.super_class_name() {
                current_class = self.resolve_class(&super_name).await?;
            } else {
                if Self::allow_missing_static_void_noop(class_name, name, descriptor) {
                    tracing::warn!("Missing game static no-op method: {}.{}:{}", class_name, name, descriptor);
                    self.ensure_initialized(&class).await?;
                    let method: Box<dyn Method> = Box::new(MissingStaticNoopMethod::new(name, descriptor));
                    let frame_name = method.frame_name();
                    return Ok(ResolvedMethod { class, method, frame_name });
                }

                tracing::error!("No such method: {}.{}:{}", class_name, name, descriptor);

                return Err(self
                    .exception("java/lang/NoSuchMethodError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }
        };

        self.ensure_initialized(&declaring_class).await?;
        let frame_name = method.frame_name();

        Ok(ResolvedMethod {
            class: declaring_class,
            method,
            frame_name,
        })
    }

    pub async fn invoke_resolved_static(&self, resolved: &ResolvedMethod, args: Box<[JavaValue]>) -> Result<JavaValue> {
        self.execute_method_with_frame_name(&resolved.class, None, &resolved.method, resolved.frame_name.clone(), args)
            .await
    }

    pub async fn resolve_virtual_method_for_instance(
        &self,
        instance: &Box<dyn ClassInstance>,
        name: &str,
        descriptor: &str,
    ) -> Result<ResolvedMethod> {
        let class_definition = instance.class_definition();
        let method = self.find_virtual_method(&*class_definition, name, descriptor, false)?;
        if let Some(method) = method {
            let class = self.resolve_class(&class_definition.name()).await?;
            let frame_name = method.frame_name();
            Ok(ResolvedMethod { class, method, frame_name })
        } else {
            tracing::error!("No such method: {}.{}:{}", class_definition.name(), name, descriptor);

            Err(self
                .exception(
                    "java/lang/NoSuchMethodError",
                    &format!("{}.{}:{}", class_definition.name(), name, descriptor),
                )
                .await)
        }
    }

    pub async fn resolve_special_method(&self, class_name: &str, name: &str, descriptor: &str) -> Result<ResolvedMethod> {
        let class = self.resolve_class(class_name).await?;
        let method = if name == "<init>" {
            class.definition.method(name, descriptor, false)
        } else {
            self.find_virtual_method(&*class.definition, name, descriptor, false)?
        };

        if let Some(method) = method {
            if method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }
            let frame_name = method.frame_name();
            Ok(ResolvedMethod { class, method, frame_name })
        } else {
            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub async fn invoke_resolved_instance(
        &self,
        resolved: &ResolvedMethod,
        instance: &Box<dyn ClassInstance>,
        args: Box<[JavaValue]>,
    ) -> Result<JavaValue> {
        let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
            .chain(args.into_vec())
            .collect::<Vec<_>>();
        self.execute_method_with_frame_name(
            &resolved.class,
            Some(instance.clone()),
            &resolved.method,
            resolved.frame_name.clone(),
            args.into_boxed_slice(),
        )
        .await
    }

    pub async fn invoke_static<T, U>(&self, class_name: &str, name: &str, descriptor: &str, args: T) -> Result<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        let args = args.into_arg();

        tracing::trace!("Invoke static {}.{}:{}({:?})", class_name, name, descriptor, args);

        let class = self.resolve_class(class_name).await?;

        let method = class.definition.method(name, descriptor, true);

        if let Some(method) = method {
            if !method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            self.ensure_initialized(&class).await?;

            Ok(self.execute_method(&class, None, &method, args).await?.into())
        } else {
            if Self::allow_missing_static_void_noop(class_name, name, descriptor) {
                tracing::warn!("Missing game static no-op method: {}.{}:{}", class_name, name, descriptor);
                self.ensure_initialized(&class).await?;
                return Ok(JavaValue::Void.into());
            }

            tracing::error!("No such method: {}.{}:{}", class_name, name, descriptor);

            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    fn allow_missing_static_void_noop(class_name: &str, name: &str, descriptor: &str) -> bool {
        let runtime_class = class_name.starts_with("java/")
            || class_name.starts_with("javax/")
            || class_name.starts_with("com/mascotcapsule/")
            || class_name.starts_with("com/nokia/")
            || class_name.starts_with("org/rustjava/");

        descriptor == "()V" && !name.starts_with('<') && !runtime_class
    }

    pub async fn invoke_virtual<T, U>(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str, args: T) -> Result<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        let instance = self.ensure_not_null(instance).await?;
        let args = args.into_arg();
        tracing::trace!(
            "Invoke virtual {}.{}:{}({:?})",
            instance.class_definition().name(),
            name,
            descriptor,
            args
        );

        let class = instance.class_definition();
        let method = self.find_virtual_method(&*class, name, descriptor, false)?;
        if let Some(x) = method {
            let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
                .chain(args.into_vec())
                .collect::<Vec<_>>();

            let class = self.resolve_class(&class.name()).await?; // TODO we're resolving class twice
            Ok(self
                .execute_method(&class, Some(instance.clone()), &x, args.into_boxed_slice())
                .await?
                .into())
        } else {
            tracing::error!("No such method: {}.{}:{}", class.name(), name, descriptor);

            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{}.{}:{}", class.name(), name, descriptor))
                .await)
        }
    }

    // non-virtual
    #[async_recursion::async_recursion]
    pub async fn invoke_special<T, U>(&self, instance: &Box<dyn ClassInstance>, class_name: &str, name: &str, descriptor: &str, args: T) -> Result<U>
    where
        T: InvokeArg,
        U: From<JavaValue>,
    {
        let instance = self.ensure_not_null(instance).await?;
        let args = args.into_arg();
        tracing::trace!("Invoke special {}.{}:{}({:?})", class_name, name, descriptor, args);

        let class = self.resolve_class(class_name).await?;
        let method = if name == "<init>" {
            class.definition.method(name, descriptor, false)
        } else {
            self.find_virtual_method(&*class.definition, name, descriptor, false)?
        };

        if let Some(method) = method {
            let args = iter::once(JavaValue::Object(Some(clone_box(&**instance))))
                .chain(args.into_vec())
                .collect::<Vec<_>>();

            if method.access_flags().contains(MethodAccessFlags::STATIC) {
                return Err(self
                    .exception("java/lang/IncompatibleClassChangeError", &format!("{class_name}.{name}:{descriptor}"))
                    .await);
            }

            Ok(self
                .execute_method(&class, Some(instance.clone()), &method, args.into_boxed_slice())
                .await?
                .into())
        } else {
            Err(self
                .exception("java/lang/NoSuchMethodError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }
    fn find_virtual_method(&self, class: &dyn ClassDefinition, name: &str, descriptor: &str, is_static: bool) -> Result<Option<Box<dyn Method>>> {
        let method = class.method(name, descriptor, false);

        if let Some(x) = method {
            if x.access_flags().contains(MethodAccessFlags::STATIC) == is_static {
                return Ok(Some(x));
            }
        } else if let Some(x) = class.super_class_name() {
            if let Some(super_class) = self.inner.classes.read().get(&x).cloned() {
                return self.find_virtual_method(&*super_class.definition, name, descriptor, is_static);
            }
        }

        Ok(None)
    }

    pub(crate) async fn execute_method(
        &self,
        class: &Class,
        class_instance: Option<Box<dyn ClassInstance>>,
        method: &Box<dyn Method>,
        args: Box<[JavaValue]>,
    ) -> Result<JavaValue> {
        self.execute_method_with_frame_name(class, class_instance, method, method.frame_name(), args)
            .await
    }

    pub(crate) async fn execute_method_with_frame_name(
        &self,
        class: &Class,
        class_instance: Option<Box<dyn ClassInstance>>,
        method: &Box<dyn Method>,
        method_str: Arc<str>,
        args: Box<[JavaValue]>,
    ) -> Result<JavaValue> {
        let thread_id = (self.inner.get_current_thread_id)();
        self.inner
            .threads
            .write()
            .get_mut(&thread_id)
            .unwrap()
            .push_java_frame(class, class_instance, method_str);

        let result = method.run(self, args).await;

        tracing::trace!("Execute result: {:?}", result);

        self.inner.threads.write().get_mut(&thread_id).unwrap().pop_frame();

        result
    }
}
