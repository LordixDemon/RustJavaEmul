use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::ClassAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

pub struct Node;
pub struct Element;
pub struct Document;
pub struct SVGElement;
pub struct SVGSVGElement;
pub struct SVGLocatableElement;
pub struct SVGAnimationElement;
pub struct SVGMatrix;
pub struct SVGRect;
pub struct SVGRGBColor;

impl Node {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/Node",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("getNodeName", "()Ljava/lang/String;", Default::default()),
                JavaMethodProto::new_abstract("getNodeValue", "()Ljava/lang/String;", Default::default()),
                JavaMethodProto::new_abstract("getParentNode", "()Lorg/w3c/dom/Node;", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
        }
    }
}

impl Element {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/Element",
            parent_class: None,
            interfaces: vec!["org/w3c/dom/Node"],
            methods: vec![
                JavaMethodProto::new_abstract("getTagName", "()Ljava/lang/String;", Default::default()),
                JavaMethodProto::new_abstract("getAttribute", "(Ljava/lang/String;)Ljava/lang/String;", Default::default()),
                JavaMethodProto::new_abstract("setAttribute", "(Ljava/lang/String;Ljava/lang/String;)V", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
        }
    }
}

impl Document {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/Document",
            parent_class: None,
            interfaces: vec!["org/w3c/dom/Node"],
            methods: vec![
                JavaMethodProto::new_abstract("getDocumentElement", "()Lorg/w3c/dom/Element;", Default::default()),
                JavaMethodProto::new_abstract("getElementById", "(Ljava/lang/String;)Lorg/w3c/dom/Element;", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
        }
    }
}

impl SVGElement {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/svg/SVGElement",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["org/w3c/dom/Element", "org/w3c/dom/Node"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getId", "()Ljava/lang/String;", Self::get_id, Default::default()),
                JavaMethodProto::new("setId", "(Ljava/lang/String;)V", Self::set_id, Default::default()),
                JavaMethodProto::new("getTrait", "(Ljava/lang/String;)Ljava/lang/String;", Self::get_trait, Default::default()),
                JavaMethodProto::new("setTrait", "(Ljava/lang/String;Ljava/lang/String;)V", Self::set_trait, Default::default()),
                JavaMethodProto::new("getFloatTrait", "(Ljava/lang/String;)F", Self::get_float_trait, Default::default()),
                JavaMethodProto::new("setFloatTrait", "(Ljava/lang/String;F)V", Self::set_float_trait, Default::default()),
                JavaMethodProto::new(
                    "getMatrixTrait",
                    "(Ljava/lang/String;)Lorg/w3c/dom/svg/SVGMatrix;",
                    Self::get_matrix_trait,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setMatrixTrait",
                    "(Ljava/lang/String;Lorg/w3c/dom/svg/SVGMatrix;)V",
                    Self::set_matrix_trait,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getRGBColorTrait",
                    "(Ljava/lang/String;)Lorg/w3c/dom/svg/SVGRGBColor;",
                    Self::get_rgb_color_trait,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setRGBColorTrait",
                    "(Ljava/lang/String;Lorg/w3c/dom/svg/SVGRGBColor;)V",
                    Self::set_rgb_color_trait,
                    Default::default(),
                ),
                // Element/Node basic implementations
                JavaMethodProto::new("getTagName", "()Ljava/lang/String;", Self::get_tag_name, Default::default()),
                JavaMethodProto::new(
                    "getAttribute",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_trait,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setAttribute",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::set_trait,
                    Default::default(),
                ),
                JavaMethodProto::new("getNodeName", "()Ljava/lang/String;", Self::get_tag_name, Default::default()),
                JavaMethodProto::new("getNodeValue", "()Ljava/lang/String;", Self::get_id, Default::default()),
                JavaMethodProto::new("getParentNode", "()Lorg/w3c/dom/Node;", Self::get_parent_node, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("id", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("tagName", "Ljava/lang/String;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let empty_id = JavaLangString::from_rust_string(jvm, "").await?;
        jvm.put_field(&mut this, "id", "Ljava/lang/String;", empty_id).await?;
        let tag = JavaLangString::from_rust_string(jvm, "g").await?;
        jvm.put_field(&mut this, "tagName", "Ljava/lang/String;", tag).await?;
        Ok(())
    }

    async fn get_id(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "id", "Ljava/lang/String;").await
    }

    async fn set_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, id: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "id", "Ljava/lang/String;", id).await
    }

    async fn get_tag_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "tagName", "Ljava/lang/String;").await
    }

    async fn get_parent_node(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<()>> {
        Ok(ClassInstanceRef::new(None))
    }

    async fn get_trait(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "").await.map(Into::into)
    }

    async fn set_trait(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
        _val: ClassInstanceRef<String>,
    ) -> Result<()> {
        Ok(())
    }

    async fn get_float_trait(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _name: ClassInstanceRef<String>) -> Result<f32> {
        Ok(0.0f32)
    }

    async fn set_float_trait(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
        _val: f32,
    ) -> Result<()> {
        Ok(())
    }

    async fn get_matrix_trait(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<SVGMatrix>> {
        let m = jvm.new_class("org/w3c/dom/svg/SVGMatrix", "()V", ()).await?;
        Ok(m.into())
    }

    async fn set_matrix_trait(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
        _m: ClassInstanceRef<SVGMatrix>,
    ) -> Result<()> {
        Ok(())
    }

    async fn get_rgb_color_trait(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<SVGRGBColor>> {
        let c = jvm.new_class("org/w3c/dom/svg/SVGRGBColor", "()V", ()).await?;
        Ok(c.into())
    }

    async fn set_rgb_color_trait(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
        _c: ClassInstanceRef<SVGRGBColor>,
    ) -> Result<()> {
        Ok(())
    }
}

impl SVGLocatableElement {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/svg/SVGLocatableElement",
            parent_class: Some("org/w3c/dom/svg/SVGElement"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getBBox", "()Lorg/w3c/dom/svg/SVGRect;", Self::get_b_box, Default::default()),
                JavaMethodProto::new("getScreenBBox", "()Lorg/w3c/dom/svg/SVGRect;", Self::get_screen_b_box, Default::default()),
                JavaMethodProto::new("getScreenCTM", "()Lorg/w3c/dom/svg/SVGMatrix;", Self::get_screen_ctm, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "org/w3c/dom/svg/SVGElement", "<init>", "()V", ()).await?;
        Ok(())
    }

    async fn get_b_box(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<SVGRect>> {
        let r = jvm.new_class("org/w3c/dom/svg/SVGRect", "()V", ()).await?;
        Ok(r.into())
    }

    async fn get_screen_b_box(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<SVGRect>> {
        let r = jvm.new_class("org/w3c/dom/svg/SVGRect", "()V", ()).await?;
        Ok(r.into())
    }

    async fn get_screen_ctm(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<SVGMatrix>> {
        let m = jvm.new_class("org/w3c/dom/svg/SVGMatrix", "()V", ()).await?;
        Ok(m.into())
    }
}

impl SVGSVGElement {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/svg/SVGSVGElement",
            parent_class: Some("org/w3c/dom/svg/SVGLocatableElement"),
            interfaces: vec!["org/w3c/dom/Document"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "createSVGMatrixComponents",
                    "(FFFFFF)Lorg/w3c/dom/svg/SVGMatrix;",
                    Self::create_svg_matrix_components,
                    Default::default(),
                ),
                JavaMethodProto::new("createSVGRect", "()Lorg/w3c/dom/svg/SVGRect;", Self::create_svg_rect, Default::default()),
                JavaMethodProto::new(
                    "createSVGRGBColor",
                    "(III)Lorg/w3c/dom/svg/SVGRGBColor;",
                    Self::create_svg_rgb_color,
                    Default::default(),
                ),
                JavaMethodProto::new("setCurrentTime", "(F)V", Self::set_current_time, Default::default()),
                JavaMethodProto::new("getCurrentTime", "()F", Self::get_current_time, Default::default()),
                // Document methods
                JavaMethodProto::new(
                    "getDocumentElement",
                    "()Lorg/w3c/dom/Element;",
                    Self::get_document_element,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getElementById",
                    "(Ljava/lang/String;)Lorg/w3c/dom/Element;",
                    Self::get_element_by_id,
                    Default::default(),
                ),
            ],
            fields: vec![JavaFieldProto::new("currentTime", "F", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "org/w3c/dom/svg/SVGLocatableElement", "<init>", "()V", ())
            .await?;
        let tag = JavaLangString::from_rust_string(jvm, "svg").await?;
        jvm.put_field(&mut this, "tagName", "Ljava/lang/String;", tag).await?;
        Ok(())
    }

    async fn create_svg_matrix_components(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
    ) -> Result<ClassInstanceRef<SVGMatrix>> {
        let m = jvm.new_class("org/w3c/dom/svg/SVGMatrix", "(FFFFFF)V", (a, b, c, d, e, f)).await?;
        Ok(m.into())
    }

    async fn create_svg_rect(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<SVGRect>> {
        let r = jvm.new_class("org/w3c/dom/svg/SVGRect", "()V", ()).await?;
        Ok(r.into())
    }

    async fn create_svg_rgb_color(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        red: i32,
        green: i32,
        blue: i32,
    ) -> Result<ClassInstanceRef<SVGRGBColor>> {
        let col = jvm.new_class("org/w3c/dom/svg/SVGRGBColor", "(III)V", (red, green, blue)).await?;
        Ok(col.into())
    }

    async fn set_current_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time: f32) -> Result<()> {
        jvm.put_field(&mut this, "currentTime", "F", time).await
    }

    async fn get_current_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "currentTime", "F").await
    }

    async fn get_document_element(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<()>> {
        Ok(ClassInstanceRef::new(this.instance.clone()))
    }

    async fn get_element_by_id(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        id: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<()>> {
        let mut elem: ClassInstanceRef<SVGElement> = jvm.new_class("org/w3c/dom/svg/SVGLocatableElement", "()V", ()).await?.into();
        jvm.put_field(&mut elem, "id", "Ljava/lang/String;", id).await?;
        Ok(ClassInstanceRef::new(elem.instance.clone()))
    }
}

impl SVGAnimationElement {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/svg/SVGAnimationElement",
            parent_class: Some("org/w3c/dom/svg/SVGElement"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("beginElementAt", "(F)V", Self::begin_element_at, Default::default()),
                JavaMethodProto::new("endElementAt", "(F)V", Self::end_element_at, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "org/w3c/dom/svg/SVGElement", "<init>", "()V", ()).await?;
        Ok(())
    }

    async fn begin_element_at(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _offset: f32) -> Result<()> {
        Ok(())
    }

    async fn end_element_at(_jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _offset: f32) -> Result<()> {
        Ok(())
    }
}

impl SVGMatrix {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/svg/SVGMatrix",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(FFFFFF)V", Self::init_components, Default::default()),
                JavaMethodProto::new("getComponent", "(I)F", Self::get_component, Default::default()),
                JavaMethodProto::new(
                    "mMultiply",
                    "(Lorg/w3c/dom/svg/SVGMatrix;)Lorg/w3c/dom/svg/SVGMatrix;",
                    Self::m_multiply,
                    Default::default(),
                ),
                JavaMethodProto::new("inverse", "()Lorg/w3c/dom/svg/SVGMatrix;", Self::inverse, Default::default()),
                JavaMethodProto::new("mTranslate", "(FF)Lorg/w3c/dom/svg/SVGMatrix;", Self::m_translate, Default::default()),
                JavaMethodProto::new("mScale", "(FF)Lorg/w3c/dom/svg/SVGMatrix;", Self::m_scale, Default::default()),
                JavaMethodProto::new("mRotate", "(F)Lorg/w3c/dom/svg/SVGMatrix;", Self::m_rotate, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("a", "F", Default::default()),
                JavaFieldProto::new("b", "F", Default::default()),
                JavaFieldProto::new("c", "F", Default::default()),
                JavaFieldProto::new("d", "F", Default::default()),
                JavaFieldProto::new("e", "F", Default::default()),
                JavaFieldProto::new("f", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        // Identity matrix
        jvm.put_field(&mut this, "a", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "b", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "c", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "d", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "e", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "f", "F", 0.0f32).await?;
        Ok(())
    }

    async fn init_components(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "a", "F", a).await?;
        jvm.put_field(&mut this, "b", "F", b).await?;
        jvm.put_field(&mut this, "c", "F", c).await?;
        jvm.put_field(&mut this, "d", "F", d).await?;
        jvm.put_field(&mut this, "e", "F", e).await?;
        jvm.put_field(&mut this, "f", "F", f).await?;
        Ok(())
    }

    async fn get_component(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, idx: i32) -> Result<f32> {
        match idx {
            0 => jvm.get_field(&this, "a", "F").await,
            1 => jvm.get_field(&this, "b", "F").await,
            2 => jvm.get_field(&this, "c", "F").await,
            3 => jvm.get_field(&this, "d", "F").await,
            4 => jvm.get_field(&this, "e", "F").await,
            5 => jvm.get_field(&this, "f", "F").await,
            _ => Ok(0.0f32),
        }
    }

    async fn m_multiply(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        let a: f32 = jvm.get_field(&this, "a", "F").await?;
        let b: f32 = jvm.get_field(&this, "b", "F").await?;
        let c: f32 = jvm.get_field(&this, "c", "F").await?;
        let d: f32 = jvm.get_field(&this, "d", "F").await?;
        let e: f32 = jvm.get_field(&this, "e", "F").await?;
        let f: f32 = jvm.get_field(&this, "f", "F").await?;

        let oa: f32 = jvm.get_field(&other, "a", "F").await?;
        let ob: f32 = jvm.get_field(&other, "b", "F").await?;
        let oc: f32 = jvm.get_field(&other, "c", "F").await?;
        let od: f32 = jvm.get_field(&other, "d", "F").await?;
        let oe: f32 = jvm.get_field(&other, "e", "F").await?;
        let of: f32 = jvm.get_field(&other, "f", "F").await?;

        let na = a * oa + c * ob;
        let nb = b * oa + d * ob;
        let nc = a * oc + c * od;
        let nd = b * oc + d * od;
        let ne = a * oe + c * of + e;
        let nf = b * oe + d * of + f;

        jvm.put_field(&mut this, "a", "F", na).await?;
        jvm.put_field(&mut this, "b", "F", nb).await?;
        jvm.put_field(&mut this, "c", "F", nc).await?;
        jvm.put_field(&mut this, "d", "F", nd).await?;
        jvm.put_field(&mut this, "e", "F", ne).await?;
        jvm.put_field(&mut this, "f", "F", nf).await?;

        Ok(this)
    }

    async fn inverse(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        let a: f32 = jvm.get_field(&this, "a", "F").await?;
        let b: f32 = jvm.get_field(&this, "b", "F").await?;
        let c: f32 = jvm.get_field(&this, "c", "F").await?;
        let d: f32 = jvm.get_field(&this, "d", "F").await?;
        let e: f32 = jvm.get_field(&this, "e", "F").await?;
        let f: f32 = jvm.get_field(&this, "f", "F").await?;

        let det = a * d - b * c;
        if det.abs() > 1e-6 {
            let inv_det = 1.0f32 / det;
            let na = d * inv_det;
            let nb = -b * inv_det;
            let nc = -c * inv_det;
            let nd = a * inv_det;
            let ne = (c * f - d * e) * inv_det;
            let nf = (b * e - a * f) * inv_det;

            let m = jvm.new_class("org/w3c/dom/svg/SVGMatrix", "(FFFFFF)V", (na, nb, nc, nd, ne, nf)).await?;
            Ok(m.into())
        } else {
            Ok(this)
        }
    }

    async fn m_translate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, tx: f32, ty: f32) -> Result<ClassInstanceRef<Self>> {
        let a: f32 = jvm.get_field(&this, "a", "F").await?;
        let b: f32 = jvm.get_field(&this, "b", "F").await?;
        let c: f32 = jvm.get_field(&this, "c", "F").await?;
        let d: f32 = jvm.get_field(&this, "d", "F").await?;
        let e: f32 = jvm.get_field(&this, "e", "F").await?;
        let f: f32 = jvm.get_field(&this, "f", "F").await?;

        let ne = a * tx + c * ty + e;
        let nf = b * tx + d * ty + f;

        jvm.put_field(&mut this, "e", "F", ne).await?;
        jvm.put_field(&mut this, "f", "F", nf).await?;
        Ok(this)
    }

    async fn m_scale(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sx: f32, sy: f32) -> Result<ClassInstanceRef<Self>> {
        let a: f32 = jvm.get_field(&this, "a", "F").await?;
        let b: f32 = jvm.get_field(&this, "b", "F").await?;
        let c: f32 = jvm.get_field(&this, "c", "F").await?;
        let d: f32 = jvm.get_field(&this, "d", "F").await?;

        jvm.put_field(&mut this, "a", "F", a * sx).await?;
        jvm.put_field(&mut this, "b", "F", b * sx).await?;
        jvm.put_field(&mut this, "c", "F", c * sy).await?;
        jvm.put_field(&mut this, "d", "F", d * sy).await?;
        Ok(this)
    }

    async fn m_rotate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, angle: f32) -> Result<ClassInstanceRef<Self>> {
        let rad = angle as f64 * (core::f64::consts::PI / 180.0);
        let cos_a = rad.cos() as f32;
        let sin_a = rad.sin() as f32;

        let a: f32 = jvm.get_field(&this, "a", "F").await?;
        let b: f32 = jvm.get_field(&this, "b", "F").await?;
        let c: f32 = jvm.get_field(&this, "c", "F").await?;
        let d: f32 = jvm.get_field(&this, "d", "F").await?;

        let na = a * cos_a + c * sin_a;
        let nb = b * cos_a + d * sin_a;
        let nc = -a * sin_a + c * cos_a;
        let nd = -b * sin_a + d * cos_a;

        jvm.put_field(&mut this, "a", "F", na).await?;
        jvm.put_field(&mut this, "b", "F", nb).await?;
        jvm.put_field(&mut this, "c", "F", nc).await?;
        jvm.put_field(&mut this, "d", "F", nd).await?;
        Ok(this)
    }
}

impl SVGRect {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/svg/SVGRect",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(FFFF)V", Self::init_bounds, Default::default()),
                JavaMethodProto::new("getX", "()F", Self::get_x, Default::default()),
                JavaMethodProto::new("getY", "()F", Self::get_y, Default::default()),
                JavaMethodProto::new("getWidth", "()F", Self::get_width, Default::default()),
                JavaMethodProto::new("getHeight", "()F", Self::get_height, Default::default()),
                JavaMethodProto::new("setX", "(F)V", Self::set_x, Default::default()),
                JavaMethodProto::new("setY", "(F)V", Self::set_y, Default::default()),
                JavaMethodProto::new("setWidth", "(F)V", Self::set_width, Default::default()),
                JavaMethodProto::new("setHeight", "(F)V", Self::set_height, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("x", "F", Default::default()),
                JavaFieldProto::new("y", "F", Default::default()),
                JavaFieldProto::new("width", "F", Default::default()),
                JavaFieldProto::new("height", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "x", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "y", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "width", "F", 240.0f32).await?;
        jvm.put_field(&mut this, "height", "F", 320.0f32).await?;
        Ok(())
    }

    async fn init_bounds(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: f32, y: f32, w: f32, h: f32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "x", "F", x).await?;
        jvm.put_field(&mut this, "y", "F", y).await?;
        jvm.put_field(&mut this, "width", "F", w).await?;
        jvm.put_field(&mut this, "height", "F", h).await?;
        Ok(())
    }

    async fn get_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "x", "F").await
    }

    async fn get_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "y", "F").await
    }

    async fn get_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "width", "F").await
    }

    async fn get_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "height", "F").await
    }

    async fn set_x(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, val: f32) -> Result<()> {
        jvm.put_field(&mut this, "x", "F", val).await
    }

    async fn set_y(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, val: f32) -> Result<()> {
        jvm.put_field(&mut this, "y", "F", val).await
    }

    async fn set_width(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, val: f32) -> Result<()> {
        jvm.put_field(&mut this, "width", "F", val).await
    }

    async fn set_height(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, val: f32) -> Result<()> {
        jvm.put_field(&mut this, "height", "F", val).await
    }
}

impl SVGRGBColor {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/w3c/dom/svg/SVGRGBColor",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(III)V", Self::init_rgb, Default::default()),
                JavaMethodProto::new("getRed", "()I", Self::get_red, Default::default()),
                JavaMethodProto::new("getGreen", "()I", Self::get_green, Default::default()),
                JavaMethodProto::new("getBlue", "()I", Self::get_blue, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("r", "I", Default::default()),
                JavaFieldProto::new("g", "I", Default::default()),
                JavaFieldProto::new("b", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Ok(())
    }

    async fn init_rgb(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, r: i32, g: i32, b: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "r", "I", r).await?;
        jvm.put_field(&mut this, "g", "I", g).await?;
        jvm.put_field(&mut this, "b", "I", b).await?;
        Ok(())
    }

    async fn get_red(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "r", "I").await
    }

    async fn get_green(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "g", "I").await
    }

    async fn get_blue(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "b", "I").await
    }
}

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    vec![
        Node::as_proto,
        Element::as_proto,
        Document::as_proto,
        SVGElement::as_proto,
        SVGLocatableElement::as_proto,
        SVGSVGElement::as_proto,
        SVGAnimationElement::as_proto,
        SVGMatrix::as_proto,
        SVGRect::as_proto,
        SVGRGBColor::as_proto,
    ]
}
