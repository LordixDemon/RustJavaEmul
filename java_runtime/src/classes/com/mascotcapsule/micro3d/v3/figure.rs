use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

use super::{
    constants::FIGURE_CLASS,
    mbac::{NativeFigure, parse as parse_mbac},
    mtra::{ActionTable, compute_posture as compute_mtra_posture},
    storage::{
        invalidate_runtime_figure_cache, load_byte_array, load_int_field, load_resource_bytes_with_extensions, put_int_array_field, put_native_figure,
    },
    texture::Texture,
};

// class com.mascotcapsule.micro3d.v3.Figure
pub struct Figure;

impl Figure {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: FIGURE_CLASS,
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_from_resource, Default::default()),
                JavaMethodProto::new("dispose", "()V", Self::dispose, Default::default()),
                JavaMethodProto::new(
                    "setPosture",
                    "(Lcom/mascotcapsule/micro3d/v3/ActionTable;II)V",
                    Self::set_posture,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setTexture",
                    "(Lcom/mascotcapsule/micro3d/v3/Texture;)V",
                    Self::set_texture,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getTexture",
                    "()Lcom/mascotcapsule/micro3d/v3/Texture;",
                    Self::get_texture,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setTexture",
                    "([Lcom/mascotcapsule/micro3d/v3/Texture;)V",
                    Self::set_textures,
                    Default::default(),
                ),
                JavaMethodProto::new("getNumTextures", "()I", Self::get_num_textures, Default::default()),
                JavaMethodProto::new("selectTexture", "(I)V", Self::select_texture, Default::default()),
                JavaMethodProto::new("getNumPattern", "()I", Self::get_num_pattern, Default::default()),
                JavaMethodProto::new("setPattern", "(I)V", Self::set_pattern, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("data", "[B", Default::default()),
                JavaFieldProto::new("vertexCount", "I", Default::default()),
                JavaFieldProto::new("vertices", "[S", Default::default()),
                JavaFieldProto::new("polyC3", "[S", Default::default()),
                JavaFieldProto::new("polyC4", "[S", Default::default()),
                JavaFieldProto::new("polyT3", "[S", Default::default()),
                JavaFieldProto::new("polyT4", "[S", Default::default()),
                JavaFieldProto::new("colors", "[I", Default::default()),
                JavaFieldProto::new("patterns", "[I", Default::default()),
                JavaFieldProto::new("patternCount", "I", Default::default()),
                JavaFieldProto::new("patternSlots", "I", Default::default()),
                JavaFieldProto::new("bones", "[I", Default::default()),
                JavaFieldProto::new("postureBones", "[I", Default::default()),
                JavaFieldProto::new("numPolyC3", "I", Default::default()),
                JavaFieldProto::new("numPolyC4", "I", Default::default()),
                JavaFieldProto::new("numPolyT3", "I", Default::default()),
                JavaFieldProto::new("numPolyT4", "I", Default::default()),
                JavaFieldProto::new("allMatsOr", "I", Default::default()),
                JavaFieldProto::new("allMatsAnd", "I", Default::default()),
                JavaFieldProto::new("selectedPattern", "I", Default::default()),
                JavaFieldProto::new("textureIndex", "I", Default::default()),
                JavaFieldProto::new("texture", "Lcom/mascotcapsule/micro3d/v3/Texture;", Default::default()),
                JavaFieldProto::new("textures", "[Lcom/mascotcapsule/micro3d/v3/Texture;", Default::default()),
                JavaFieldProto::new("postureAction", "Lcom/mascotcapsule/micro3d/v3/ActionTable;", Default::default()),
                JavaFieldProto::new("postureActionIndex", "I", Default::default()),
                JavaFieldProto::new("postureFrame", "I", Default::default()),
                JavaFieldProto::new("disposed", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, data: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("com.mascotcapsule.micro3d.v3.Figure::<init>({this:?}, {data:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let bytes = load_byte_array(jvm, &data).await?;
        let model = parse_mbac(&bytes).unwrap_or_else(|err| {
            tracing::warn!("MBAC parse failed: {err}");
            NativeFigure::empty()
        });
        put_native_figure(jvm, &mut this, model).await?;
        jvm.put_field(&mut this, "data", "[B", data).await?;
        jvm.put_field(
            &mut this,
            "texture",
            "Lcom/mascotcapsule/micro3d/v3/Texture;",
            ClassInstanceRef::<Texture>::new(None),
        )
        .await?;
        jvm.put_field(
            &mut this,
            "textures",
            "[Lcom/mascotcapsule/micro3d/v3/Texture;",
            ClassInstanceRef::<Array<Texture>>::new(None),
        )
        .await?;
        jvm.put_field(
            &mut this,
            "postureAction",
            "Lcom/mascotcapsule/micro3d/v3/ActionTable;",
            ClassInstanceRef::<ActionTable>::new(None),
        )
        .await?;
        jvm.put_field(&mut this, "postureActionIndex", "I", 0).await?;
        jvm.put_field(&mut this, "postureFrame", "I", 0).await?;
        jvm.put_field(&mut this, "selectedPattern", "I", 0).await?;
        jvm.put_field(&mut this, "textureIndex", "I", -1).await?;
        put_int_array_field(jvm, &mut this, "postureBones", Vec::new()).await?;
        jvm.put_field(&mut this, "disposed", "Z", false).await
    }

    async fn init_from_resource(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        resource_name: ClassInstanceRef<String>,
    ) -> Result<()> {
        let bytes = load_resource_bytes_with_extensions(jvm, context, &resource_name, &[".mbac"])
            .await
            .unwrap_or_default();
        let mut data = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_array(&mut data, 0, bytes.into_iter().map(|value| value as i8)).await?;
        let data: ClassInstanceRef<Array<i8>> = data.into();
        Self::init(jvm, context, this, data).await
    }

    async fn dispose(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Figure::dispose({this:?})");

        invalidate_runtime_figure_cache(&this);
        jvm.put_field(&mut this, "disposed", "Z", true).await
    }

    async fn set_posture(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        action_table: ClassInstanceRef<ActionTable>,
        action: i32,
        frame: i32,
    ) -> Result<()> {
        tracing::trace!("com.mascotcapsule.micro3d.v3.Figure::setPosture({this:?}, {action_table:?}, {action:?}, {frame:?})");

        let frame = frame.max(0);
        let current_action_table: ClassInstanceRef<ActionTable> = jvm
            .get_field(&this, "postureAction", "Lcom/mascotcapsule/micro3d/v3/ActionTable;")
            .await?;
        let current_action: i32 = jvm.get_field(&this, "postureActionIndex", "I").await?;
        let current_frame: i32 = jvm.get_field(&this, "postureFrame", "I").await?;
        if same_instance(&current_action_table, &action_table) && current_action == action && current_frame == frame {
            return Ok(());
        }

        let previous_pattern: i32 = jvm.get_field(&this, "selectedPattern", "I").await?;
        let (posture_bones, selected_pattern) = if action_table.is_null() {
            (Vec::new(), previous_pattern)
        } else {
            let valid: bool = jvm.get_field(&action_table, "valid", "Z").await?;
            if !valid {
                return Self::put_posture_state(jvm, &mut this, action_table, action, frame, previous_pattern, Vec::new()).await;
            }

            let data: ClassInstanceRef<Array<i8>> = jvm.get_field(&action_table, "data", "[B").await?;
            let bytes = load_byte_array(jvm, &data).await?;
            let bones = load_int_field(jvm, &this, "bones").await?;
            compute_mtra_posture(&bytes, action, frame, &bones, previous_pattern).unwrap_or_else(|err| {
                tracing::warn!("MTRA posture failed: {err}");
                (Vec::new(), previous_pattern)
            })
        };

        Self::put_posture_state(jvm, &mut this, action_table, action, frame, selected_pattern, posture_bones).await
    }

    async fn put_posture_state(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        action_table: ClassInstanceRef<ActionTable>,
        action: i32,
        frame: i32,
        selected_pattern: i32,
        posture_bones: Vec<i32>,
    ) -> Result<()> {
        jvm.put_field(this, "postureAction", "Lcom/mascotcapsule/micro3d/v3/ActionTable;", action_table)
            .await?;
        jvm.put_field(this, "postureActionIndex", "I", action).await?;
        jvm.put_field(this, "postureFrame", "I", frame).await?;
        jvm.put_field(this, "selectedPattern", "I", selected_pattern).await?;
        put_int_array_field(jvm, this, "postureBones", posture_bones).await
    }

    async fn set_texture(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, texture: ClassInstanceRef<Texture>) -> Result<()> {
        jvm.put_field(&mut this, "texture", "Lcom/mascotcapsule/micro3d/v3/Texture;", texture)
            .await?;
        jvm.put_field(&mut this, "textureIndex", "I", 0).await
    }

    async fn get_texture(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Texture>> {
        let texture_index: i32 = jvm.get_field(&this, "textureIndex", "I").await?;
        if texture_index < 0 {
            return Ok(ClassInstanceRef::<Texture>::new(None));
        }

        let textures: ClassInstanceRef<Array<Texture>> = jvm.get_field(&this, "textures", "[Lcom/mascotcapsule/micro3d/v3/Texture;").await?;
        if !textures.is_null() {
            let length = jvm.array_length(&textures).await?;
            if (texture_index as usize) < length {
                let values: Vec<ClassInstanceRef<Texture>> = jvm.load_array(&textures, texture_index as usize, 1).await?;
                return Ok(values.into_iter().next().unwrap_or_else(|| ClassInstanceRef::<Texture>::new(None)));
            }
        }

        jvm.get_field(&this, "texture", "Lcom/mascotcapsule/micro3d/v3/Texture;").await
    }

    async fn set_textures(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        textures: ClassInstanceRef<Array<Texture>>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "textures", "[Lcom/mascotcapsule/micro3d/v3/Texture;", textures)
            .await?;
        jvm.put_field(&mut this, "textureIndex", "I", -1).await
    }

    async fn get_num_textures(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let texture_index: i32 = jvm.get_field(&this, "textureIndex", "I").await?;
        if texture_index >= 0 {
            let texture: ClassInstanceRef<Texture> = jvm.get_field(&this, "texture", "Lcom/mascotcapsule/micro3d/v3/Texture;").await?;
            return Ok((!texture.is_null()) as i32);
        }

        let textures: ClassInstanceRef<Array<Texture>> = jvm.get_field(&this, "textures", "[Lcom/mascotcapsule/micro3d/v3/Texture;").await?;
        if textures.is_null() {
            Ok(0)
        } else {
            Ok(jvm.array_length(&textures).await? as i32)
        }
    }

    async fn select_texture(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, texture_index: i32) -> Result<()> {
        if texture_index < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Figure texture index").await);
        }
        jvm.put_field(&mut this, "textureIndex", "I", texture_index).await
    }

    async fn get_num_pattern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "patternCount", "I").await
    }

    async fn set_pattern(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pattern: i32) -> Result<()> {
        jvm.put_field(&mut this, "selectedPattern", "I", pattern).await
    }
}

fn same_instance<T, U>(left: &ClassInstanceRef<T>, right: &ClassInstanceRef<U>) -> bool {
    match (&left.instance, &right.instance) {
        (Some(left), Some(right)) => left.equals(&**right).unwrap_or(false),
        (None, None) => true,
        _ => false,
    }
}
