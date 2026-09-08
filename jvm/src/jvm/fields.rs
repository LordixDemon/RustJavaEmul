use alloc::{boxed::Box, format};

use core::fmt::Debug;

use java_constants::FieldAccessFlags;

use crate::{
    Result,
    class_definition::ClassDefinition,
    class_instance::{ClassInstance, NullInstance},
    value::JavaValue,
};

use super::{Jvm, ResolvedField, ResolvedInstanceField, ResolvedStaticField};

impl Jvm {
    pub async fn get_static_field<T>(&self, class_name: &str, name: &str, descriptor: &str) -> Result<T>
    where
        T: From<JavaValue>,
    {
        tracing::trace!("Get static field {}.{}:{}", class_name, name, descriptor);

        let class = self.resolve_class(class_name).await?;

        let field = self.find_field(&*class.definition, name, descriptor, Some(true))?;
        if let Some(field) = field {
            self.ensure_initialized(&field.class).await?;

            Ok(field.class.definition.get_static_field(&*field.field)?.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub async fn resolve_static_field(&self, class_name: &str, name: &str, descriptor: &str) -> Result<ResolvedStaticField> {
        let class = self.resolve_class(class_name).await?;

        let field = self.find_field(&*class.definition, name, descriptor, Some(true))?;
        if let Some(field) = field {
            self.ensure_initialized(&field.class).await?;

            Ok(ResolvedStaticField {
                class: field.class,
                field: field.field,
            })
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub fn get_resolved_static_field(&self, resolved: &ResolvedStaticField) -> Result<JavaValue> {
        resolved.class.definition.get_static_field(&*resolved.field)
    }

    pub fn put_resolved_static_field(&self, resolved: &ResolvedStaticField, value: JavaValue) -> Result<()> {
        resolved.class.definition.put_static_field(&*resolved.field, value)
    }

    pub async fn resolve_field_for_instance(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str) -> Result<ResolvedInstanceField> {
        let class_definition = instance.class_definition();
        let field = self.find_field(&*class_definition, name, descriptor, None)?;

        if let Some(field) = field {
            let is_static = field.field.access_flags().contains(FieldAccessFlags::STATIC);
            if is_static {
                self.ensure_initialized(&field.class).await?;
            }
            Ok(ResolvedInstanceField {
                class: field.class,
                field: field.field,
                is_static,
            })
        } else {
            Err(self
                .exception(
                    "java/lang/NoSuchFieldError",
                    &format!("{}.{}:{}", class_definition.name(), name, descriptor),
                )
                .await)
        }
    }

    pub async fn resolve_instance_field(&self, class_name: &str, name: &str, descriptor: &str) -> Result<ResolvedInstanceField> {
        let class = self.resolve_class(class_name).await?;
        let field = self.find_field(&*class.definition, name, descriptor, None)?;

        if let Some(field) = field {
            let is_static = field.field.access_flags().contains(FieldAccessFlags::STATIC);
            if is_static {
                self.ensure_initialized(&field.class).await?;
            }
            Ok(ResolvedInstanceField {
                class: field.class,
                field: field.field,
                is_static,
            })
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub fn get_resolved_instance_field(&self, resolved: &ResolvedInstanceField, instance: &Box<dyn ClassInstance>) -> Result<JavaValue> {
        if resolved.is_static {
            resolved.class.definition.get_static_field(&*resolved.field)
        } else {
            instance.get_field(&*resolved.field)
        }
    }

    pub fn put_resolved_instance_field(
        &self,
        resolved: &ResolvedInstanceField,
        instance: &mut Box<dyn ClassInstance>,
        value: JavaValue,
    ) -> Result<()> {
        if resolved.is_static {
            resolved.class.definition.put_static_field(&*resolved.field, value)
        } else {
            instance.put_field(&*resolved.field, value)
        }
    }

    pub async fn put_static_field<T>(&self, class_name: &str, name: &str, descriptor: &str, value: T) -> Result<()>
    where
        T: Into<JavaValue> + Debug,
    {
        tracing::trace!("Put static field {}.{}:{} = {:?}", class_name, name, descriptor, value);

        let class = self.resolve_class(class_name).await?;

        let field = self.find_field(&*class.definition, name, descriptor, Some(true))?;

        if let Some(field) = field {
            self.ensure_initialized(&field.class).await?;

            let class = field.class;
            class.definition.put_static_field(&*field.field, value.into())
        } else {
            Err(self
                .exception("java/lang/NoSuchFieldError", &format!("{class_name}.{name}:{descriptor}"))
                .await)
        }
    }

    pub(crate) async fn ensure_not_null<'a>(&self, instance: &'a Box<dyn ClassInstance>) -> Result<&'a Box<dyn ClassInstance>> {
        if instance.as_any().downcast_ref::<NullInstance>().is_some() {
            Err(self.exception("java/lang/NullPointerException", "").await)
        } else {
            Ok(instance)
        }
    }

    pub(crate) async fn ensure_not_null_mut<'a>(&self, instance: &'a mut Box<dyn ClassInstance>) -> Result<&'a mut Box<dyn ClassInstance>> {
        if instance.as_any().downcast_ref::<NullInstance>().is_some() {
            Err(self.exception("java/lang/NullPointerException", "").await)
        } else {
            Ok(instance)
        }
    }

    pub async fn get_field<T>(&self, instance: &Box<dyn ClassInstance>, name: &str, descriptor: &str) -> Result<T>
    where
        T: From<JavaValue>,
    {
        let instance = self.ensure_not_null(instance).await?;
        tracing::trace!("Get field {}.{}:{}", instance.class_definition().name(), name, descriptor);

        let field = self.find_field(&*instance.class_definition(), name, descriptor, None)?;

        if let Some(field) = field {
            if field.field.access_flags().contains(FieldAccessFlags::STATIC) {
                self.ensure_initialized(&field.class).await?;
                Ok(field.class.definition.get_static_field(&*field.field)?.into())
            } else {
                Ok(instance.get_field(&*field.field)?.into())
            }
        } else {
            Err(self
                .exception(
                    "java/lang/NoSuchFieldError",
                    &format!("{}.{}:{}", instance.class_definition().name(), name, descriptor),
                )
                .await)
        }
    }

    pub async fn put_field<T>(&self, instance: &mut Box<dyn ClassInstance>, name: &str, descriptor: &str, value: T) -> Result<()>
    where
        T: Into<JavaValue> + Debug,
    {
        let instance = self.ensure_not_null_mut(instance).await?;
        tracing::trace!("Put field {}.{}:{} = {:?}", instance.class_definition().name(), name, descriptor, value);

        let field = self.find_field(&*instance.class_definition(), name, descriptor, None)?;

        if let Some(field) = field {
            let value = coerce_field_value(descriptor, value.into());
            if field.field.access_flags().contains(FieldAccessFlags::STATIC) {
                self.ensure_initialized(&field.class).await?;
                let class = field.class;
                class.definition.put_static_field(&*field.field, value)
            } else {
                instance.put_field(&*field.field, value)
            }
        } else {
            Err(self
                .exception(
                    "java/lang/NoSuchFieldError",
                    &format!("{}.{}:{}", instance.class_definition().name(), name, descriptor),
                )
                .await)
        }
    }
    fn find_field(&self, class: &dyn ClassDefinition, name: &str, descriptor: &str, static_filter: Option<bool>) -> Result<Option<ResolvedField>> {
        let class_name = class.name();
        let class = self.inner.classes.read().get(&class_name).cloned();
        let Some(class) = class else {
            return Ok(None);
        };

        let field = match static_filter {
            Some(is_static) => class.definition.field(name, descriptor, is_static),
            None => class
                .definition
                .field(name, descriptor, false)
                .or_else(|| class.definition.field(name, descriptor, true)),
        };
        if let Some(field) = field {
            return Ok(Some(ResolvedField { class, field }));
        }

        for interface in class.definition.interface_names() {
            let interface_class = self.inner.classes.read().get(&interface).map(|x| x.definition.clone());
            if let Some(interface_class) = interface_class
                && let Some(field) = self.find_field(&*interface_class, name, descriptor, static_filter)?
            {
                return Ok(Some(field));
            }
        }

        if let Some(super_class) = class.definition.super_class_name() {
            let super_class = self.inner.classes.read().get(&super_class).map(|x| x.definition.clone());
            if let Some(super_class) = super_class {
                return self.find_field(&*super_class, name, descriptor, static_filter);
            }
        }

        Ok(None)
    }
}

fn coerce_field_value(descriptor: &str, value: JavaValue) -> JavaValue {
    match (descriptor, value) {
        ("F", JavaValue::Double(value)) => JavaValue::Float(value as f32),
        (_, value) => value,
    }
}
