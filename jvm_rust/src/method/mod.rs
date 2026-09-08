mod fixed_point;
mod general;
mod gfx;
mod int_compiler;
mod loop_compiler;

use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::{
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

use classfile::{AttributeInfo, AttributeInfoCode, MethodInfo};
use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{JavaError, JavaType, JavaValue, Jvm, JvmCallback, Method, Result};

use crate::{
    interpreter::{Interpreter, MethodBytecodeCache},
    profile,
};

pub enum MethodBody {
    ByteCode(AttributeInfoCode, MethodBytecodeCache),
    Rust(Box<dyn JvmCallback>),
}

impl MethodBody {
    pub fn from_rust(callback: Box<dyn JvmCallback>) -> Self {
        Self::Rust(callback)
    }
}

impl Debug for MethodBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MethodBody::ByteCode(..) => write!(f, "ByteCode"),
            MethodBody::Rust(_) => write!(f, "Rust"),
        }
    }
}

#[derive(Debug)]
struct MethodInner {
    name: String,
    descriptor: String,
    frame_name: Arc<str>,
    profile_name: Arc<str>,
    return_type: JavaType,
    body: Option<MethodBody>,
    access_flags: MethodAccessFlags,
}

#[derive(Clone, Debug)]
pub struct MethodImpl {
    inner: Arc<MethodInner>,
}

impl MethodImpl {
    pub fn new(name: &str, descriptor: &str, body: MethodBody, access_flags: MethodAccessFlags) -> Self {
        Self::new_with_profile_name(name, descriptor, body, access_flags, alloc::format!("{name}{descriptor}").into())
    }

    fn new_with_profile_name(name: &str, descriptor: &str, body: MethodBody, access_flags: MethodAccessFlags, profile_name: Arc<str>) -> Self {
        let return_type = JavaType::parse(descriptor).as_method().1.clone();
        let frame_name = alloc::format!("{name}{descriptor}").into();
        Self {
            inner: Arc::new(MethodInner {
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                frame_name,
                profile_name,
                return_type,
                body: Some(body),
                access_flags,
            }),
        }
    }

    pub fn from_method_proto<C, Context>(proto: JavaMethodProto<C>, context: Context) -> Self
    where
        C: ?Sized + 'static + Send,
        Context: Sync + Send + DerefMut + Deref<Target = C> + Clone + 'static,
    {
        Self::from_method_proto_with_owner("", proto, context)
    }

    pub fn from_method_proto_with_owner<C, Context>(owner: &str, proto: JavaMethodProto<C>, context: Context) -> Self
    where
        C: ?Sized + 'static + Send,
        Context: Sync + Send + DerefMut + Deref<Target = C> + Clone + 'static,
    {
        struct MethodProxy<C, Context>
        where
            C: ?Sized,
            Context: Sync + Send + DerefMut + Deref<Target = C> + Clone,
        {
            body: Box<dyn java_class_proto::MethodBody<JavaError, C>>,
            context: Context,
        }

        #[async_trait::async_trait]
        impl<C, Context> JvmCallback for MethodProxy<C, Context>
        where
            C: ?Sized + Send,
            Context: Sync + Send + DerefMut + Deref<Target = C> + Clone,
        {
            async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
                let mut context = self.context.clone();

                self.body.call(jvm, &mut context, args).await
            }
        }

        let profile_name = method_profile_name(owner, &proto.name, &proto.descriptor);

        Self::new_with_profile_name(
            &proto.name,
            &proto.descriptor,
            MethodBody::Rust(Box::new(MethodProxy { body: proto.body, context })),
            proto.access_flags,
            profile_name,
        )
    }

    pub fn from_method_info(method_info: MethodInfo) -> Self {
        Self::from_method_info_with_owner("", method_info)
    }

    pub fn from_method_info_with_owner(owner: &str, method_info: MethodInfo) -> Self {
        let return_type = JavaType::parse(&method_info.descriptor).as_method().1.clone();
        let frame_name = alloc::format!("{}{}", method_info.name, method_info.descriptor).into();
        let profile_name = method_profile_name(owner, &method_info.name, &method_info.descriptor);
        let body = Self::extract_body(method_info.attributes).map(|code| {
            if let Some(intrinsic) = fixed_point::maybe_intrinsic(method_info.descriptor.as_str(), &code) {
                profile::record_bytecode_intrinsic_install();
                MethodBody::Rust(intrinsic)
            } else if let Some(compiled) = int_compiler::maybe_compile(method_info.descriptor.as_str(), &code) {
                profile::record_int_compiler_install();
                MethodBody::Rust(compiled)
            } else {
                let cache = MethodBytecodeCache::from_code(&code);
                MethodBody::ByteCode(code, cache)
            }
        });
        Self {
            inner: Arc::new(MethodInner {
                name: method_info.name.to_string(),
                descriptor: method_info.descriptor.to_string(),
                frame_name,
                profile_name,
                return_type,
                body,
                access_flags: method_info.access_flags,
            }),
        }
    }

    fn extract_body(attributes: Vec<AttributeInfo>) -> Option<AttributeInfoCode> {
        for attribute in attributes {
            if let AttributeInfo::Code(x) = attribute {
                return Some(x);
            }
        }

        None
    }
}

#[async_trait::async_trait]
impl Method for MethodImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn descriptor(&self) -> String {
        self.inner.descriptor.clone()
    }

    fn frame_name(&self) -> Arc<str> {
        self.inner.frame_name.clone()
    }

    fn access_flags(&self) -> MethodAccessFlags {
        self.inner.access_flags
    }

    fn run_sync(&self, jvm: &Jvm, args: &[JavaValue]) -> Option<Result<JavaValue>> {
        match &self.inner.body {
            Some(MethodBody::Rust(body)) => body.call_sync(jvm, args),
            _ => None,
        }
    }

    async fn run(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        match &self.inner.body {
            Some(MethodBody::ByteCode(x, cache)) => {
                Interpreter::run(jvm, x, cache, args, &self.inner.return_type, self.inner.profile_name.clone()).await
            }
            Some(MethodBody::Rust(x)) => {
                tracing::trace!(
                    target: "rustjava_native",
                    "native.enter {}{} args={:?}",
                    self.inner.name,
                    self.inner.descriptor,
                    args
                );
                if let Some(result) = x.call_sync(jvm, &args) {
                    return result;
                }
                x.call(jvm, args).await
            }
            None => {
                let message = alloc::format!("{}{}", self.inner.name, self.inner.descriptor);
                if self.inner.access_flags.contains(MethodAccessFlags::NATIVE) {
                    Err(jvm.exception("java/lang/UnsatisfiedLinkError", &message).await)
                } else {
                    Err(jvm.exception("java/lang/AbstractMethodError", &message).await)
                }
            }
        }
    }
}

fn method_profile_name(owner: &str, name: &str, descriptor: &str) -> Arc<str> {
    if owner.is_empty() {
        alloc::format!("{name}{descriptor}").into()
    } else {
        alloc::format!("{owner}.{name}{descriptor}").into()
    }
}
