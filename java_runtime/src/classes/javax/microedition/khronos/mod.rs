pub struct EGL;
pub struct EGL10;
pub struct EGLConfig;
pub struct EGLContext;
pub struct EGLDisplay;
pub struct EGLSurface;
pub struct GL;
pub struct GL10;

#[path = "egl.rs"]
mod egl_impl;
#[path = "gl.rs"]
mod gl_impl;

pub mod egl {
    pub use super::{EGL, EGL10, EGLConfig, EGLContext, EGLDisplay, EGLSurface};
}

pub mod opengles {
    pub use super::{GL, GL10};
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![EGL, EGL10, EGLConfig, EGLContext, EGLDisplay, EGLSurface, GL, GL10]
}
