#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl EGL {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/khronos/egl/EGL",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

simple_object!(EGLConfig, "javax/microedition/khronos/egl/EGLConfig");
simple_object!(EGLDisplay, "javax/microedition/khronos/egl/EGLDisplay");
simple_object!(EGLSurface, "javax/microedition/khronos/egl/EGLSurface");

impl EGLContext {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/khronos/egl/EGLContext",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getEGL",
                    "()Ljavax/microedition/khronos/egl/EGL;",
                    Self::get_egl,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getGL", "()Ljavax/microedition/khronos/opengles/GL;", Self::get_gl, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn get_egl(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<EGL>> {
        Ok(jvm.new_class("javax/microedition/khronos/egl/EGL10", "()V", ()).await?.into())
    }

    pub(super) async fn get_gl(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<GL>> {
        Ok(jvm.new_class("javax/microedition/khronos/opengles/GL10", "()V", ()).await?.into())
    }
}

impl EGL10 {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/khronos/egl/EGL10",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/khronos/egl/EGL"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "eglChooseConfig",
                    "(Ljavax/microedition/khronos/egl/EGLDisplay;[I[Ljavax/microedition/khronos/egl/EGLConfig;I[I)Z",
                    Self::egl_choose_config,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "eglCreateContext",
                    "(Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLConfig;Ljavax/microedition/khronos/egl/EGLContext;[I)Ljavax/microedition/khronos/egl/EGLContext;",
                    Self::egl_create_context,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "eglCreateWindowSurface",
                    "(Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLConfig;Ljava/lang/Object;[I)Ljavax/microedition/khronos/egl/EGLSurface;",
                    Self::egl_create_window_surface,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "eglDestroyContext",
                    "(Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLContext;)Z",
                    Self::egl_destroy_context,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "eglDestroySurface",
                    "(Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLSurface;)Z",
                    Self::egl_destroy_surface,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "eglGetCurrentContext",
                    "()Ljavax/microedition/khronos/egl/EGLContext;",
                    Self::egl_get_current_context,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "eglGetDisplay",
                    "(Ljava/lang/Object;)Ljavax/microedition/khronos/egl/EGLDisplay;",
                    Self::egl_get_display,
                    Default::default(),
                ),
                JavaMethodProto::new("eglGetError", "()I", Self::egl_get_error, Default::default()),
                JavaMethodProto::new(
                    "eglInitialize",
                    "(Ljavax/microedition/khronos/egl/EGLDisplay;[I)Z",
                    Self::egl_initialize,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "eglMakeCurrent",
                    "(Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLSurface;Ljavax/microedition/khronos/egl/EGLSurface;Ljavax/microedition/khronos/egl/EGLContext;)Z",
                    Self::egl_make_current,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "eglSwapBuffers",
                    "(Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLSurface;)Z",
                    Self::egl_swap_buffers,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "eglTerminate",
                    "(Ljavax/microedition/khronos/egl/EGLDisplay;)Z",
                    Self::egl_terminate,
                    Default::default(),
                ),
                JavaMethodProto::new("eglWaitGL", "()Z", Self::egl_wait_gl, Default::default()),
                JavaMethodProto::new("eglWaitNative", "(ILjava/lang/Object;)Z", Self::egl_wait_native, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "EGL_DEFAULT_DISPLAY",
                    "Ljava/lang/Object;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "EGL_NO_CONTEXT",
                    "Ljavax/microedition/khronos/egl/EGLContext;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "EGL_NO_SURFACE",
                    "Ljavax/microedition/khronos/egl/EGLSurface;",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/khronos/egl/EGL10";
        let display = jvm.new_class("java/lang/Object", "()V", ()).await?;
        let no_context = jvm.new_class("javax/microedition/khronos/egl/EGLContext", "()V", ()).await?;
        let no_surface = jvm.new_class("javax/microedition/khronos/egl/EGLSurface", "()V", ()).await?;
        jvm.put_static_field(class, "EGL_DEFAULT_DISPLAY", "Ljava/lang/Object;", display).await?;
        jvm.put_static_field(class, "EGL_NO_CONTEXT", "Ljavax/microedition/khronos/egl/EGLContext;", no_context)
            .await?;
        jvm.put_static_field(class, "EGL_NO_SURFACE", "Ljavax/microedition/khronos/egl/EGLSurface;", no_surface)
            .await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn egl_choose_config(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<EGLDisplay>,
        _attrib: ClassInstanceRef<Array<i32>>,
        mut configs: ClassInstanceRef<Array<EGLConfig>>,
        config_size: i32,
        mut num_config: ClassInstanceRef<Array<i32>>,
    ) -> Result<bool> {
        if !num_config.is_null() && jvm.array_length(&num_config).await? > 0 {
            jvm.store_array(&mut num_config, 0, vec![1]).await?;
        }
        if !configs.is_null() && config_size > 0 && jvm.array_length(&configs).await? > 0 {
            let config = jvm.new_class("javax/microedition/khronos/egl/EGLConfig", "()V", ()).await?;
            jvm.store_array(&mut configs, 0, vec![config]).await?;
        }
        Ok(true)
    }

    pub(super) async fn egl_create_context(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<EGLDisplay>,
        _config: ClassInstanceRef<EGLConfig>,
        _share: ClassInstanceRef<EGLContext>,
        _attrib: ClassInstanceRef<Array<i32>>,
    ) -> Result<ClassInstanceRef<EGLContext>> {
        Ok(jvm.new_class("javax/microedition/khronos/egl/EGLContext", "()V", ()).await?.into())
    }

    pub(super) async fn egl_create_window_surface(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<EGLDisplay>,
        _config: ClassInstanceRef<EGLConfig>,
        _native: ClassInstanceRef<Object>,
        _attrib: ClassInstanceRef<Array<i32>>,
    ) -> Result<ClassInstanceRef<EGLSurface>> {
        Ok(jvm.new_class("javax/microedition/khronos/egl/EGLSurface", "()V", ()).await?.into())
    }

    pub(super) async fn egl_destroy_context(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<EGLDisplay>,
        _ctx: ClassInstanceRef<EGLContext>,
    ) -> Result<bool> {
        Ok(true)
    }

    pub(super) async fn egl_destroy_surface(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<EGLDisplay>,
        _surface: ClassInstanceRef<EGLSurface>,
    ) -> Result<bool> {
        Ok(true)
    }

    pub(super) async fn egl_get_current_context(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<EGLContext>> {
        jvm.get_static_field(
            "javax/microedition/khronos/egl/EGL10",
            "EGL_NO_CONTEXT",
            "Ljavax/microedition/khronos/egl/EGLContext;",
        )
        .await
    }

    pub(super) async fn egl_get_display(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _native: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<EGLDisplay>> {
        Ok(jvm.new_class("javax/microedition/khronos/egl/EGLDisplay", "()V", ()).await?.into())
    }

    pub(super) async fn egl_get_error(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0x3000)
    }

    pub(super) async fn egl_initialize(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<EGLDisplay>,
        mut version: ClassInstanceRef<Array<i32>>,
    ) -> Result<bool> {
        if !version.is_null() && jvm.array_length(&version).await? >= 2 {
            jvm.store_array(&mut version, 0, vec![1, 0]).await?;
        }
        Ok(true)
    }

    pub(super) async fn egl_make_current(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<EGLDisplay>,
        _draw: ClassInstanceRef<EGLSurface>,
        _read: ClassInstanceRef<EGLSurface>,
        _ctx: ClassInstanceRef<EGLContext>,
    ) -> Result<bool> {
        Ok(true)
    }

    pub(super) async fn egl_swap_buffers(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<EGLDisplay>,
        _surface: ClassInstanceRef<EGLSurface>,
    ) -> Result<bool> {
        Ok(true)
    }

    pub(super) async fn egl_terminate(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _display: ClassInstanceRef<EGLDisplay>,
    ) -> Result<bool> {
        Ok(true)
    }

    pub(super) async fn egl_wait_gl(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(true)
    }

    pub(super) async fn egl_wait_native(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _engine: i32,
        _bind: ClassInstanceRef<Object>,
    ) -> Result<bool> {
        Ok(true)
    }
}
