#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::nio::Buffer};
#[allow(unused_imports)]
use alloc::vec;
use alloc::vec::Vec;
use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

impl GL {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/khronos/opengles/GL",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl GL10 {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/khronos/opengles/GL10",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/khronos/opengles/GL"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("glAlphaFuncx", "(II)V", Self::gl_alpha_funcx, Default::default()),
                JavaMethodProto::new("glBindTexture", "(II)V", Self::gl_bind_texture, Default::default()),
                JavaMethodProto::new("glBlendFunc", "(II)V", Self::gl_blend_func, Default::default()),
                JavaMethodProto::new("glClear", "(I)V", Self::gl_clear, Default::default()),
                JavaMethodProto::new("glClearColorx", "(IIII)V", Self::gl_clear_colorx, Default::default()),
                JavaMethodProto::new("glColor4x", "(IIII)V", Self::gl_color4x, Default::default()),
                JavaMethodProto::new("glCullFace", "(I)V", Self::gl_cull_face, Default::default()),
                JavaMethodProto::new("glDepthFunc", "(I)V", Self::gl_depth_func, Default::default()),
                JavaMethodProto::new("glDepthMask", "(Z)V", Self::gl_depth_mask, Default::default()),
                JavaMethodProto::new("glDepthRangex", "(II)V", Self::gl_depth_rangex, Default::default()),
                JavaMethodProto::new("glDisable", "(I)V", Self::gl_disable, Default::default()),
                JavaMethodProto::new("glDisableClientState", "(I)V", Self::gl_disable_client_state, Default::default()),
                JavaMethodProto::new("glDrawArrays", "(III)V", Self::gl_draw_arrays, Default::default()),
                JavaMethodProto::new("glDrawElements", "(IIILjava/nio/Buffer;)V", Self::gl_draw_elements, Default::default()),
                JavaMethodProto::new("glEnable", "(I)V", Self::gl_enable, Default::default()),
                JavaMethodProto::new("glEnableClientState", "(I)V", Self::gl_enable_client_state, Default::default()),
                JavaMethodProto::new("glFlush", "()V", Self::gl_flush, Default::default()),
                JavaMethodProto::new("glFogf", "(IF)V", Self::gl_fogf, Default::default()),
                JavaMethodProto::new("glFogx", "(II)V", Self::gl_fogx, Default::default()),
                JavaMethodProto::new("glFogxv", "(I[II)V", Self::gl_fogxv, Default::default()),
                JavaMethodProto::new("glFrontFace", "(I)V", Self::gl_front_face, Default::default()),
                JavaMethodProto::new("glGenTextures", "(I[II)V", Self::gl_gen_textures, Default::default()),
                JavaMethodProto::new("glGetIntegerv", "(I[II)V", Self::gl_get_integerv, Default::default()),
                JavaMethodProto::new("glGetString", "(I)Ljava/lang/String;", Self::gl_get_string, Default::default()),
                JavaMethodProto::new("glHint", "(II)V", Self::gl_hint, Default::default()),
                JavaMethodProto::new("glLightModelx", "(II)V", Self::gl_light_modelx, Default::default()),
                JavaMethodProto::new("glLightModelxv", "(I[II)V", Self::gl_light_modelxv, Default::default()),
                JavaMethodProto::new("glLightxv", "(II[II)V", Self::gl_lightxv, Default::default()),
                JavaMethodProto::new("glLoadIdentity", "()V", Self::gl_load_identity, Default::default()),
                JavaMethodProto::new("glLoadMatrixx", "([II)V", Self::gl_load_matrixx, Default::default()),
                JavaMethodProto::new("glMaterialx", "(III)V", Self::gl_materialx, Default::default()),
                JavaMethodProto::new("glMaterialxv", "(II[II)V", Self::gl_materialxv, Default::default()),
                JavaMethodProto::new("glMatrixMode", "(I)V", Self::gl_matrix_mode, Default::default()),
                JavaMethodProto::new("glNormalPointer", "(IILjava/nio/Buffer;)V", Self::gl_normal_pointer, Default::default()),
                JavaMethodProto::new("glOrthox", "(IIIIII)V", Self::gl_orthox, Default::default()),
                JavaMethodProto::new("glPixelStorei", "(II)V", Self::gl_pixel_storei, Default::default()),
                JavaMethodProto::new("glPopMatrix", "()V", Self::gl_pop_matrix, Default::default()),
                JavaMethodProto::new("glPushMatrix", "()V", Self::gl_push_matrix, Default::default()),
                JavaMethodProto::new("glScalex", "(III)V", Self::gl_scalex, Default::default()),
                JavaMethodProto::new("glScissor", "(IIII)V", Self::gl_scissor, Default::default()),
                JavaMethodProto::new("glShadeModel", "(I)V", Self::gl_shade_model, Default::default()),
                JavaMethodProto::new(
                    "glTexCoordPointer",
                    "(IIILjava/nio/Buffer;)V",
                    Self::gl_tex_coord_pointer,
                    Default::default(),
                ),
                JavaMethodProto::new("glTexEnvx", "(III)V", Self::gl_tex_envx, Default::default()),
                JavaMethodProto::new("glTexImage2D", "(IIIIIIIILjava/nio/Buffer;)V", Self::gl_tex_image_2d, Default::default()),
                JavaMethodProto::new("glTexParameterf", "(IIF)V", Self::gl_tex_parameterf, Default::default()),
                JavaMethodProto::new("glTexParameterx", "(III)V", Self::gl_tex_parameterx, Default::default()),
                JavaMethodProto::new("glTranslatex", "(III)V", Self::gl_translatex, Default::default()),
                JavaMethodProto::new("glVertexPointer", "(IIILjava/nio/Buffer;)V", Self::gl_vertex_pointer, Default::default()),
                JavaMethodProto::new("glViewport", "(IIII)V", Self::gl_viewport, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    stub_void! {
        gl_alpha_funcx(_a: i32, _b: i32);
        gl_bind_texture(_a: i32, _b: i32);
        gl_blend_func(_a: i32, _b: i32);
        gl_clear(_a: i32);
        gl_clear_colorx(_a: i32, _b: i32, _c: i32, _d: i32);
        gl_color4x(_a: i32, _b: i32, _c: i32, _d: i32);
        gl_cull_face(_a: i32);
        gl_depth_func(_a: i32);
        gl_depth_mask(_a: bool);
        gl_depth_rangex(_a: i32, _b: i32);
        gl_disable(_a: i32);
        gl_disable_client_state(_a: i32);
        gl_draw_arrays(_a: i32, _b: i32, _c: i32);
        gl_draw_elements(_a: i32, _b: i32, _c: i32, _buf: ClassInstanceRef<Buffer>);
        gl_enable(_a: i32);
        gl_enable_client_state(_a: i32);
        gl_flush();
        gl_fogf(_a: i32, _b: f32);
        gl_fogx(_a: i32, _b: i32);
        gl_fogxv(_a: i32, _b: ClassInstanceRef<Array<i32>>, _c: i32);
        gl_front_face(_a: i32)
    }

    pub(super) async fn gl_gen_textures(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        n: i32,
        mut textures: ClassInstanceRef<Array<i32>>,
        offset: i32,
    ) -> Result<()> {
        if textures.is_null() || n <= 0 {
            return Ok(());
        }
        let ids: Vec<i32> = (1..=n).collect();
        jvm.store_array(&mut textures, offset.max(0) as usize, ids).await
    }
    pub(super) async fn gl_get_integerv(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _pname: i32,
        mut params: ClassInstanceRef<Array<i32>>,
        offset: i32,
    ) -> Result<()> {
        if params.is_null() {
            return Ok(());
        }
        let len = jvm.array_length(&params).await?;
        if (offset as usize) < len {
            jvm.store_array(&mut params, offset.max(0) as usize, vec![0]).await?;
        }
        Ok(())
    }
    pub(super) async fn gl_get_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: i32,
    ) -> Result<ClassInstanceRef<crate::classes::java::lang::String>> {
        JavaLangString::from_rust_string(jvm, "RustJava GL").await.map(Into::into)
    }

    stub_void! {
        gl_hint(_a: i32, _b: i32);
        gl_light_modelx(_a: i32, _b: i32);
        gl_light_modelxv(_a: i32, _b: ClassInstanceRef<Array<i32>>, _c: i32);
        gl_lightxv(_a: i32, _b: i32, _c: ClassInstanceRef<Array<i32>>, _d: i32);
        gl_load_identity();
        gl_load_matrixx(_a: ClassInstanceRef<Array<i32>>, _b: i32);
        gl_materialx(_a: i32, _b: i32, _c: i32);
        gl_materialxv(_a: i32, _b: i32, _c: ClassInstanceRef<Array<i32>>, _d: i32);
        gl_matrix_mode(_a: i32);
        gl_normal_pointer(_a: i32, _b: i32, _buf: ClassInstanceRef<Buffer>);
        gl_orthox(_a: i32, _b: i32, _c: i32, _d: i32, _e: i32, _f: i32);
        gl_pixel_storei(_a: i32, _b: i32);
        gl_pop_matrix();
        gl_push_matrix();
        gl_scalex(_a: i32, _b: i32, _c: i32);
        gl_scissor(_a: i32, _b: i32, _c: i32, _d: i32);
        gl_shade_model(_a: i32);
        gl_tex_coord_pointer(_a: i32, _b: i32, _c: i32, _buf: ClassInstanceRef<Buffer>);
        gl_tex_envx(_a: i32, _b: i32, _c: i32);
        gl_tex_image_2d(_a: i32, _b: i32, _c: i32, _d: i32, _e: i32, _f: i32, _g: i32, _h: i32, _buf: ClassInstanceRef<Buffer>);
        gl_tex_parameterf(_a: i32, _b: i32, _c: f32);
        gl_tex_parameterx(_a: i32, _b: i32, _c: i32);
        gl_translatex(_a: i32, _b: i32, _c: i32);
        gl_vertex_pointer(_a: i32, _b: i32, _c: i32, _buf: ClassInstanceRef<Buffer>);
        gl_viewport(_a: i32, _b: i32, _c: i32, _d: i32)
    }
}
