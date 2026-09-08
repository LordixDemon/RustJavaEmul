#[allow(unused_imports)]
use super::common::*;
#[allow(unused_imports)]
use super::math::*;
#[allow(unused_imports)]
use super::prelude::*;
#[allow(unused_imports)]
use super::raw_arrays::*;
#[allow(unused_imports)]
use super::render::*;
#[allow(unused_imports)]
use super::types::*;

impl Node {
    pub const NONE: i32 = 144;
    pub const ORIGIN: i32 = 145;
    pub const X_AXIS: i32 = 146;
    pub const Y_AXIS: i32 = 147;
    pub const Z_AXIS: i32 = 148;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Node",
            parent_class: Some("javax/microedition/m3g/Transformable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("align", "(Ljavax/microedition/m3g/Node;)V", Self::align, Default::default()),
                JavaMethodProto::new(
                    "getAlignmentReference",
                    "(I)Ljavax/microedition/m3g/Node;",
                    Self::get_alignment_reference,
                    Default::default(),
                ),
                JavaMethodProto::new("getAlignmentTarget", "(I)I", Self::get_alignment_target, Default::default()),
                JavaMethodProto::new("getAlphaFactor", "()F", Self::get_alpha_factor, Default::default()),
                JavaMethodProto::new("getParent", "()Ljavax/microedition/m3g/Node;", Self::get_parent, Default::default()),
                JavaMethodProto::new("getScope", "()I", Self::get_scope, Default::default()),
                JavaMethodProto::new(
                    "getTransformTo",
                    "(Ljavax/microedition/m3g/Node;Ljavax/microedition/m3g/Transform;)Z",
                    Self::get_transform_to,
                    Default::default(),
                ),
                JavaMethodProto::new("isPickingEnabled", "()Z", Self::is_picking_enabled, Default::default()),
                JavaMethodProto::new("isRenderingEnabled", "()Z", Self::is_rendering_enabled, Default::default()),
                JavaMethodProto::new(
                    "setAlignment",
                    "(Ljavax/microedition/m3g/Node;ILjavax/microedition/m3g/Node;I)V",
                    Self::set_alignment,
                    Default::default(),
                ),
                JavaMethodProto::new("setAlphaFactor", "(F)V", Self::set_alpha_factor, Default::default()),
                JavaMethodProto::new("setPickingEnable", "(Z)V", Self::set_picking_enable, Default::default()),
                JavaMethodProto::new("setRenderingEnable", "(Z)V", Self::set_rendering_enable, Default::default()),
                JavaMethodProto::new("setScope", "(I)V", Self::set_scope, Default::default()),
            ],
            fields: vec![
                static_int_field("NONE"),
                static_int_field("ORIGIN"),
                static_int_field("X_AXIS"),
                static_int_field("Y_AXIS"),
                static_int_field("Z_AXIS"),
                JavaFieldProto::new("parent", "Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("renderingEnabled", "Z", Default::default()),
                JavaFieldProto::new("pickingEnabled", "Z", Default::default()),
                JavaFieldProto::new("alphaFactor", "F", Default::default()),
                JavaFieldProto::new("scope", "I", Default::default()),
                JavaFieldProto::new("zReference", "Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("yReference", "Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("zTarget", "I", Default::default()),
                JavaFieldProto::new("yTarget", "I", Default::default()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        put_static_ints(
            jvm,
            "javax/microedition/m3g/Node",
            &[
                ("NONE", Self::NONE),
                ("ORIGIN", Self::ORIGIN),
                ("X_AXIS", Self::X_AXIS),
                ("Y_AXIS", Self::Y_AXIS),
                ("Z_AXIS", Self::Z_AXIS),
            ],
        )
        .await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "javax/microedition/m3g/Transformable", "<init>", "()V", ())
            .await?;
        jvm.put_field(&mut this, "parent", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(&mut this, "renderingEnabled", "Z", true).await?;
        jvm.put_field(&mut this, "pickingEnabled", "Z", true).await?;
        jvm.put_field(&mut this, "alphaFactor", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "scope", "I", -1).await?;
        jvm.put_field(&mut this, "zReference", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(&mut this, "yReference", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(&mut this, "zTarget", "I", Self::NONE).await?;
        jvm.put_field(&mut this, "yTarget", "I", Self::NONE).await
    }

    pub(super) async fn align(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, reference: ClassInstanceRef<Self>) -> Result<()> {
        let mut stack = vec![this.clone()];
        let mut visited = Vec::new();
        while let Some(node) = stack.pop() {
            if node.is_null() || visited.iter().any(|seen| same_instance(seen, &node)) {
                continue;
            }
            visited.push(node.clone());
            Self::align_one(jvm, &node, &reference).await?;
            let class_name = node.class_definition().name().to_string();
            if class_name == "javax/microedition/m3g/Group" || class_name == "javax/microedition/m3g/World" {
                let child_count: i32 = jvm.get_field(&node, "childCount", "I").await.unwrap_or(0);
                if child_count > 0 {
                    let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> =
                        jvm.get_field(&node, "children", "[Ljavax/microedition/m3g/Node;").await?;
                    let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
                    stack.extend(children);
                }
            }
            if class_name == "javax/microedition/m3g/SkinnedMesh" {
                let skeleton: ClassInstanceRef<Group> = jvm
                    .get_field(&node, "skeleton", "Ljavax/microedition/m3g/Group;")
                    .await
                    .unwrap_or_else(|_| null_ref());
                if !skeleton.is_null() {
                    stack.push(cast_ref(&skeleton));
                }
            }
        }
        Ok(())
    }

    pub(super) async fn align_one(jvm: &Jvm, node: &ClassInstanceRef<Self>, default_reference: &ClassInstanceRef<Self>) -> Result<()> {
        let z_target = jvm.get_field::<i32>(node, "zTarget", "I").await.unwrap_or(Self::NONE);
        let y_target = jvm.get_field::<i32>(node, "yTarget", "I").await.unwrap_or(Self::NONE);
        if z_target == Self::NONE && y_target == Self::NONE {
            return Ok(());
        }

        let z_reference: ClassInstanceRef<Self> = jvm
            .get_field(node, "zReference", "Ljavax/microedition/m3g/Node;")
            .await
            .unwrap_or_else(|_| null_ref());
        let y_reference: ClassInstanceRef<Self> = jvm
            .get_field(node, "yReference", "Ljavax/microedition/m3g/Node;")
            .await
            .unwrap_or_else(|_| null_ref());
        let z_ref = if z_reference.is_null() { default_reference.clone() } else { z_reference };
        let y_ref = if y_reference.is_null() { default_reference.clone() } else { y_reference };

        let (_, node_world) = Self::root_and_world_matrix(jvm, node).await?;
        let z_axis = Self::alignment_axis(jvm, node, &node_world, &z_ref, z_target, 2).await?;
        let is_camera = node.class_definition().name() == "javax/microedition/m3g/Camera";
        let z_axis = if is_camera {
            z_axis.map(|axis| [-axis[0], -axis[1], -axis[2]])
        } else {
            z_axis
        };
        let y_axis = Self::alignment_axis(jvm, node, &node_world, &y_ref, y_target, 1).await?;
        let Some(z_axis) = z_axis.or_else(|| normalize3([node_world[2], node_world[6], node_world[10]])) else {
            return Ok(());
        };
        let y_guess = y_axis.unwrap_or([node_world[1], node_world[5], node_world[9]]);
        let Some(x_axis) = normalize3(cross3(y_guess, z_axis)) else {
            return Ok(());
        };
        let Some(y_axis) = normalize3(cross3(z_axis, x_axis)) else {
            return Ok(());
        };

        let aligned = [
            x_axis[0],
            y_axis[0],
            z_axis[0],
            node_world[3],
            x_axis[1],
            y_axis[1],
            z_axis[1],
            node_world[7],
            x_axis[2],
            y_axis[2],
            z_axis[2],
            node_world[11],
            0.0,
            0.0,
            0.0,
            1.0,
        ];
        let parent: ClassInstanceRef<Self> = jvm
            .get_field(node, "parent", "Ljavax/microedition/m3g/Node;")
            .await
            .unwrap_or_else(|_| null_ref());
        let local = if parent.is_null() {
            aligned
        } else {
            let (_, parent_world) = Self::root_and_world_matrix(jvm, &parent).await?;
            let Some(parent_inverse) = invert_matrix(parent_world) else {
                return Ok(());
            };
            multiply_matrix(parent_inverse, aligned)
        };
        let (angle, ax, ay, az) = rotation_matrix_to_axis_angle(local);
        let mut transformable = cast_ref::<Node, Transformable>(node);
        Transformable::set_orientation_fields(jvm, &mut transformable, angle, ax, ay, az).await
    }

    pub(super) async fn alignment_axis(
        jvm: &Jvm,
        _node: &ClassInstanceRef<Self>,
        node_world: &[f32; 16],
        reference: &ClassInstanceRef<Self>,
        target: i32,
        fallback_column: usize,
    ) -> Result<Option<[f32; 3]>> {
        if target == Self::NONE || reference.is_null() {
            return Ok(None);
        }
        let (_, ref_world) = Self::root_and_world_matrix(jvm, reference).await?;
        let axis = match target {
            Self::ORIGIN => [ref_world[3] - node_world[3], ref_world[7] - node_world[7], ref_world[11] - node_world[11]],
            Self::X_AXIS => [ref_world[0], ref_world[4], ref_world[8]],
            Self::Y_AXIS => [ref_world[1], ref_world[5], ref_world[9]],
            Self::Z_AXIS => [ref_world[2], ref_world[6], ref_world[10]],
            _ => {
                let col = fallback_column;
                [node_world[col], node_world[col + 4], node_world[col + 8]]
            }
        };
        Ok(normalize3(axis))
    }

    pub(super) async fn get_alignment_reference(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        axis: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        match axis {
            Self::Y_AXIS => jvm.get_field(&this, "yReference", "Ljavax/microedition/m3g/Node;").await,
            Self::Z_AXIS => jvm.get_field(&this, "zReference", "Ljavax/microedition/m3g/Node;").await,
            _ => Err(jvm.exception("java/lang/IllegalArgumentException", "alignment axis").await),
        }
    }

    pub(super) async fn get_alignment_target(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, axis: i32) -> Result<i32> {
        match axis {
            Self::Y_AXIS => jvm.get_field(&this, "yTarget", "I").await,
            Self::Z_AXIS => jvm.get_field(&this, "zTarget", "I").await,
            _ => Err(jvm.exception("java/lang/IllegalArgumentException", "alignment axis").await),
        }
    }

    pub(super) async fn get_alpha_factor(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "alphaFactor", "F").await
    }

    pub(super) async fn get_parent(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        jvm.get_field(&this, "parent", "Ljavax/microedition/m3g/Node;").await
    }

    pub(super) async fn get_scope(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "scope", "I").await
    }

    pub(super) async fn get_transform_to(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Self>,
        mut transform: ClassInstanceRef<Transform>,
    ) -> Result<bool> {
        if target.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Node.getTransformTo target").await);
        }
        let (source_root, source_world) = Self::root_and_world_matrix(jvm, &this).await?;
        let (target_root, target_world) = Self::root_and_world_matrix(jvm, &target).await?;
        if !same_instance(&source_root, &target_root) {
            return Ok(false);
        }
        let Some(target_inverse) = invert_matrix(target_world) else {
            return Ok(false);
        };
        if !transform.is_null() {
            Transform::put_matrix(jvm, &mut transform, multiply_matrix(target_inverse, source_world)).await?;
        }
        Ok(true)
    }

    pub(super) async fn is_picking_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "pickingEnabled", "Z").await
    }

    pub(super) async fn is_rendering_enabled(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "renderingEnabled", "Z").await
    }

    pub(super) async fn set_alignment(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        z_reference: ClassInstanceRef<Self>,
        z_target: i32,
        y_reference: ClassInstanceRef<Self>,
        y_target: i32,
    ) -> Result<()> {
        Self::validate_alignment_target(jvm, z_target).await?;
        Self::validate_alignment_target(jvm, y_target).await?;
        jvm.put_field(&mut this, "zReference", "Ljavax/microedition/m3g/Node;", z_reference)
            .await?;
        jvm.put_field(&mut this, "yReference", "Ljavax/microedition/m3g/Node;", y_reference)
            .await?;
        jvm.put_field(&mut this, "zTarget", "I", z_target).await?;
        jvm.put_field(&mut this, "yTarget", "I", y_target).await
    }

    pub(super) async fn set_alpha_factor(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, alpha_factor: f32) -> Result<()> {
        jvm.put_field(&mut this, "alphaFactor", "F", alpha_factor.clamp(0.0, 1.0)).await
    }

    pub(super) async fn set_picking_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, enabled: bool) -> Result<()> {
        jvm.put_field(&mut this, "pickingEnabled", "Z", enabled).await
    }

    pub(super) async fn set_rendering_enable(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, enabled: bool) -> Result<()> {
        let user_id = jvm.get_field::<i32>(&cast_ref::<Node, Object3D>(&this), "userID", "I").await.unwrap_or(0);
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.Node.setRenderingEnable class={} userID={} enabled={}",
            this.class_definition().name(),
            user_id,
            enabled
        );
        jvm.put_field(&mut this, "renderingEnabled", "Z", enabled).await
    }

    pub(super) async fn set_scope(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, scope: i32) -> Result<()> {
        jvm.put_field(&mut this, "scope", "I", scope).await
    }

    pub(super) async fn validate_alignment_target(jvm: &Jvm, target: i32) -> Result<()> {
        match target {
            Self::NONE | Self::ORIGIN | Self::X_AXIS | Self::Y_AXIS | Self::Z_AXIS => Ok(()),
            _ => Err(jvm.exception("java/lang/IllegalArgumentException", "alignment target").await),
        }
    }

    pub(super) async fn root_and_world_matrix(jvm: &Jvm, node: &ClassInstanceRef<Self>) -> Result<(ClassInstanceRef<Self>, [f32; 16])> {
        let mut chain = Vec::new();
        let mut current = node.clone();
        while !current.is_null() {
            chain.push(current.clone());
            current = jvm
                .get_field(&current, "parent", "Ljavax/microedition/m3g/Node;")
                .await
                .unwrap_or_else(|_| null_ref());
        }
        let root = chain.last().cloned().unwrap_or_else(null_ref);
        let mut world = identity_matrix();
        for node in chain.into_iter().rev() {
            let local = Transformable::local_matrix(jvm, &cast_ref::<Node, Transformable>(&node))
                .await
                .unwrap_or_else(|_| identity_matrix());
            world = multiply_matrix(world, local);
        }
        Ok((root, world))
    }
}
