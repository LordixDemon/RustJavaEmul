use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

pub struct ScalableImage;
pub struct SVGImage;
pub struct SVGAnimator;
pub struct SVGEventListener;
pub struct ExternalResourceHandler;
pub struct ScalableGraphics;

impl ScalableImage {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m2g/ScalableImage",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljava/io/InputStream;Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/ScalableImage;",
                    Self::create_image_stream,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljava/lang/String;Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/ScalableImage;",
                    Self::create_image_string,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("setViewportWidth", "(I)V", Self::set_viewport_width, Default::default()),
                JavaMethodProto::new("setViewportHeight", "(I)V", Self::set_viewport_height, Default::default()),
                JavaMethodProto::new("getViewportWidth", "()I", Self::get_viewport_width, Default::default()),
                JavaMethodProto::new("getViewportHeight", "()I", Self::get_viewport_height, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("viewportWidth", "I", Default::default()),
                JavaFieldProto::new("viewportHeight", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "viewportWidth", "I", 240i32).await?;
        jvm.put_field(&mut this, "viewportHeight", "I", 320i32).await?;
        Ok(())
    }

    async fn create_image_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _stream: ClassInstanceRef<()>,
        _handler: ClassInstanceRef<()>,
    ) -> Result<ClassInstanceRef<ScalableImage>> {
        let img = jvm.new_class("javax/microedition/m2g/SVGImage", "()V", ()).await?;
        Ok(img.into())
    }

    async fn create_image_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _path: ClassInstanceRef<()>,
        _handler: ClassInstanceRef<()>,
    ) -> Result<ClassInstanceRef<ScalableImage>> {
        let img = jvm.new_class("javax/microedition/m2g/SVGImage", "()V", ()).await?;
        Ok(img.into())
    }

    async fn set_viewport_width(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, width: i32) -> Result<()> {
        jvm.put_field(&mut this, "viewportWidth", "I", width).await
    }

    async fn set_viewport_height(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, height: i32) -> Result<()> {
        jvm.put_field(&mut this, "viewportHeight", "I", height).await
    }

    async fn get_viewport_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportWidth", "I").await
    }

    async fn get_viewport_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportHeight", "I").await
    }
}

impl SVGImage {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m2g/SVGImage",
            parent_class: Some("javax/microedition/m2g/ScalableImage"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljava/io/InputStream;Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/ScalableImage;",
                    Self::create_image_stream,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljava/lang/String;Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/ScalableImage;",
                    Self::create_image_string,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getDocument", "()Lorg/w3c/dom/Document;", Self::get_document, Default::default()),
                JavaMethodProto::new("focusOn", "(Lorg/w3c/dom/svg/SVGElement;)V", Self::focus_on, Default::default()),
                JavaMethodProto::new("setViewportWidth", "(I)V", Self::set_viewport_width, Default::default()),
                JavaMethodProto::new("setViewportHeight", "(I)V", Self::set_viewport_height, Default::default()),
                JavaMethodProto::new("getViewportWidth", "()I", Self::get_viewport_width, Default::default()),
                JavaMethodProto::new("getViewportHeight", "()I", Self::get_viewport_height, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("document", "Lorg/w3c/dom/Document;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/m2g/ScalableImage", "<init>", "()V", ())
            .await?;
        let doc = jvm.new_class("org/w3c/dom/svg/SVGSVGElement", "()V", ()).await?;
        jvm.put_field(&mut this, "document", "Lorg/w3c/dom/Document;", doc).await?;
        Ok(())
    }

    async fn create_image_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _stream: ClassInstanceRef<()>,
        _handler: ClassInstanceRef<()>,
    ) -> Result<ClassInstanceRef<ScalableImage>> {
        let img = jvm.new_class("javax/microedition/m2g/SVGImage", "()V", ()).await?;
        Ok(img.into())
    }

    async fn create_image_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _path: ClassInstanceRef<()>,
        _handler: ClassInstanceRef<()>,
    ) -> Result<ClassInstanceRef<ScalableImage>> {
        let img = jvm.new_class("javax/microedition/m2g/SVGImage", "()V", ()).await?;
        Ok(img.into())
    }

    async fn get_document(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<()>> {
        let doc: ClassInstanceRef<()> = jvm.get_field(&this, "document", "Lorg/w3c/dom/Document;").await?;
        if doc.is_null() {
            let new_doc = jvm.new_class("org/w3c/dom/svg/SVGSVGElement", "()V", ()).await?;
            jvm.put_field(&mut this, "document", "Lorg/w3c/dom/Document;", new_doc.clone()).await?;
            Ok(new_doc.into())
        } else {
            Ok(doc)
        }
    }

    async fn focus_on(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _elem: ClassInstanceRef<()>) -> Result<()> {
        Ok(())
    }

    async fn set_viewport_width(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, width: i32) -> Result<()> {
        jvm.put_field(&mut this, "viewportWidth", "I", width).await
    }

    async fn set_viewport_height(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, height: i32) -> Result<()> {
        jvm.put_field(&mut this, "viewportHeight", "I", height).await
    }

    async fn get_viewport_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportWidth", "I").await
    }

    async fn get_viewport_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "viewportHeight", "I").await
    }
}

impl SVGAnimator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m2g/SVGAnimator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "createAnimator",
                    "(Ljavax/microedition/m2g/SVGImage;)Ljavax/microedition/m2g/SVGAnimator;",
                    Self::create_animator,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createAnimator",
                    "(Ljavax/microedition/m2g/SVGImage;Ljava/lang/String;)Ljavax/microedition/m2g/SVGAnimator;",
                    Self::create_animator_str,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "setSVGEventListener",
                    "(Ljavax/microedition/m2g/SVGEventListener;)V",
                    Self::set_svg_event_listener,
                    Default::default(),
                ),
                JavaMethodProto::new("setTimeIncrement", "(F)V", Self::set_time_increment, Default::default()),
                JavaMethodProto::new("getTimeIncrement", "()F", Self::get_time_increment, Default::default()),
                JavaMethodProto::new("play", "()V", Self::play, Default::default()),
                JavaMethodProto::new("pause", "()V", Self::pause, Default::default()),
                JavaMethodProto::new("stop", "()V", Self::stop, Default::default()),
                JavaMethodProto::new(
                    "getTargetComponent",
                    "()Ljava/lang/Object;",
                    Self::get_target_component,
                    Default::default(),
                ),
                JavaMethodProto::new("invokeAndWait", "(Ljava/lang/Runnable;)V", Self::invoke_and_wait, Default::default()),
                JavaMethodProto::new("invokeLater", "(Ljava/lang/Runnable;)V", Self::invoke_later, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("svgImage", "Ljavax/microedition/m2g/SVGImage;", Default::default()),
                JavaFieldProto::new("eventListener", "Ljavax/microedition/m2g/SVGEventListener;", Default::default()),
                JavaFieldProto::new("timeIncrement", "F", Default::default()),
                JavaFieldProto::new("targetComponent", "Ljava/lang/Object;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "timeIncrement", "F", 0.05f32).await?;
        Ok(())
    }

    async fn create_animator(jvm: &Jvm, _: &mut RuntimeContext, svg_image: ClassInstanceRef<SVGImage>) -> Result<ClassInstanceRef<SVGAnimator>> {
        let mut animator: ClassInstanceRef<Self> = jvm.new_class("javax/microedition/m2g/SVGAnimator", "()V", ()).await?.into();
        jvm.put_field(&mut animator, "svgImage", "Ljavax/microedition/m2g/SVGImage;", svg_image)
            .await?;
        Ok(animator)
    }

    async fn create_animator_str(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        svg_image: ClassInstanceRef<SVGImage>,
        _comp: ClassInstanceRef<()>,
    ) -> Result<ClassInstanceRef<SVGAnimator>> {
        let mut animator: ClassInstanceRef<Self> = jvm.new_class("javax/microedition/m2g/SVGAnimator", "()V", ()).await?.into();
        jvm.put_field(&mut animator, "svgImage", "Ljavax/microedition/m2g/SVGImage;", svg_image)
            .await?;
        Ok(animator)
    }

    async fn set_svg_event_listener(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<()>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "eventListener", "Ljavax/microedition/m2g/SVGEventListener;", listener)
            .await
    }

    async fn set_time_increment(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, inc: f32) -> Result<()> {
        jvm.put_field(&mut this, "timeIncrement", "F", inc).await
    }

    async fn get_time_increment(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "timeIncrement", "F").await
    }

    async fn play(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn pause(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn stop(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn get_target_component(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<()>> {
        let comp: ClassInstanceRef<()> = jvm.get_field(&this, "targetComponent", "Ljava/lang/Object;").await?;
        if comp.is_null() {
            let canvas = jvm.new_class("javax/microedition/lcdui/Canvas", "()V", ()).await?;
            jvm.put_field(&mut this, "targetComponent", "Ljava/lang/Object;", canvas.clone()).await?;
            Ok(canvas.into())
        } else {
            Ok(comp)
        }
    }

    async fn invoke_and_wait(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, runnable: ClassInstanceRef<()>) -> Result<()> {
        if !runnable.is_null() {
            let _: () = jvm.invoke_virtual(&runnable, "run", "()V", ()).await?;
        }
        Ok(())
    }

    async fn invoke_later(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, runnable: ClassInstanceRef<()>) -> Result<()> {
        if !runnable.is_null() {
            let _: () = jvm.invoke_virtual(&runnable, "run", "()V", ()).await?;
        }
        Ok(())
    }
}

impl SVGEventListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m2g/SVGEventListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("sizeChanged", "(II)V", Default::default()),
                JavaMethodProto::new_abstract("keyPressed", "(I)V", Default::default()),
                JavaMethodProto::new_abstract("keyReleased", "(I)V", Default::default()),
                JavaMethodProto::new_abstract("pointerPressed", "(II)V", Default::default()),
                JavaMethodProto::new_abstract("pointerReleased", "(II)V", Default::default()),
                JavaMethodProto::new_abstract("showNotify", "()V", Default::default()),
                JavaMethodProto::new_abstract("hideNotify", "()V", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
        }
    }
}

impl ExternalResourceHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m2g/ExternalResourceHandler",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new_abstract(
                "requestResource",
                "(Ljavax/microedition/m2g/SVGImage;Ljava/lang/String;)V",
                Default::default(),
            )],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
        }
    }
}

impl ScalableGraphics {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m2g/ScalableGraphics",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("bindTarget", "(Ljava/lang/Object;)V", Self::bind_target, Default::default()),
                JavaMethodProto::new("releaseTarget", "()V", Self::release_target, Default::default()),
                JavaMethodProto::new("render", "(IILjavax/microedition/m2g/ScalableImage;)V", Self::render, Default::default()),
                JavaMethodProto::new("setTransparency", "(F)V", Self::set_transparency, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Ok(())
    }

    async fn bind_target(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _target: ClassInstanceRef<()>) -> Result<()> {
        Ok(())
    }

    async fn release_target(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn render(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _x: i32,
        _y: i32,
        _img: ClassInstanceRef<ScalableImage>,
    ) -> Result<()> {
        Ok(())
    }

    async fn set_transparency(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _alpha: f32) -> Result<()> {
        Ok(())
    }
}

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    vec![
        ScalableImage::as_proto,
        SVGImage::as_proto,
        SVGAnimator::as_proto,
        SVGEventListener::as_proto,
        ExternalResourceHandler::as_proto,
        ScalableGraphics::as_proto,
    ]
}
