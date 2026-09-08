#[allow(unused_imports)]
use super::super::super::common::*;
#[allow(unused_imports)]
use super::super::super::math::*;
#[allow(unused_imports)]
use super::super::super::prelude::*;
#[allow(unused_imports)]
use super::super::super::raw_arrays::*;
#[allow(unused_imports)]
use super::super::super::render::*;
#[allow(unused_imports)]
use super::super::super::types::*;

impl super::super::super::Graphics3D {
    pub(crate) async fn collect_scene_lighting(
        jvm: &Jvm,
        root: ClassInstanceRef<Node>,
        root_matrix: [f32; 16],
        view: [f32; 16],
    ) -> Result<M3gSceneLighting> {
        let mut lighting = M3gSceneLighting::default();
        let mut stack = vec![(root, root_matrix, false, true, 1.0f32)];
        while let Some((node, parent_matrix, apply_local_transform, parent_rendering_enabled, parent_alpha)) = stack.pop() {
            if node.is_null() {
                continue;
            }

            let rendering_enabled = jvm.get_field::<bool>(&node, "renderingEnabled", "Z").await.unwrap_or(true);
            let subtree_rendering_enabled = parent_rendering_enabled && rendering_enabled;
            let world_matrix = if apply_local_transform {
                let local_matrix = Self::node_local_matrix(jvm, &node).await.unwrap_or_else(|_| identity_matrix());
                multiply_matrix(parent_matrix, local_matrix)
            } else {
                parent_matrix
            };
            let class_name = node.class_definition().name().to_string();

            if subtree_rendering_enabled && class_name == "javax/microedition/m3g/Light" {
                Self::add_light_to_scene(jvm, cast_ref(&node), world_matrix, view, &mut lighting).await?;
            }

            if subtree_rendering_enabled && (class_name == "javax/microedition/m3g/Group" || class_name == "javax/microedition/m3g/World") {
                let group: ClassInstanceRef<Group> = cast_ref(&node);
                let child_count: i32 = jvm.get_field(&group, "childCount", "I").await.unwrap_or(0);
                if child_count > 0 {
                    let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> =
                        jvm.get_field(&group, "children", "[Ljavax/microedition/m3g/Node;").await?;
                    let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
                    for child in children.into_iter().rev() {
                        stack.push((child, world_matrix, true, subtree_rendering_enabled, parent_alpha));
                    }
                }
            }

            if subtree_rendering_enabled {
                push_skinned_skeleton(jvm, &node, &mut stack, world_matrix, subtree_rendering_enabled, parent_alpha).await?;
            }
        }
        Ok(lighting)
    }

    pub(crate) async fn collect_g3d_lights(jvm: &Jvm, this: &ClassInstanceRef<Self>, view: [f32; 16]) -> Result<M3gSceneLighting> {
        let mut lighting = M3gSceneLighting::default();
        let lights = Self::light_list(jvm, this).await?;
        let transforms = Self::light_transform_list(jvm, this).await?;
        for (index, light) in lights.into_iter().enumerate() {
            if light.is_null() {
                continue;
            }
            let transform = transforms.get(index).cloned().unwrap_or_else(null_ref);
            let light_matrix = if transform.is_null() {
                identity_matrix()
            } else {
                Transform::matrix(jvm, &transform).await.unwrap_or_else(|_| identity_matrix())
            };
            Self::add_light_to_scene(jvm, light, light_matrix, view, &mut lighting).await?;
        }
        Ok(lighting)
    }

    pub(crate) async fn add_light_to_scene(
        jvm: &Jvm,
        light: ClassInstanceRef<Light>,
        light_to_world: [f32; 16],
        view: [f32; 16],
        lighting: &mut M3gSceneLighting,
    ) -> Result<()> {
        let color = jvm.get_field::<i32>(&light, "color", "I").await.unwrap_or(0x00ff_ffff);
        let intensity = jvm.get_field::<f32>(&light, "intensity", "F").await.unwrap_or(1.0).max(0.0);
        let mode = jvm.get_field::<i32>(&light, "mode", "I").await.unwrap_or(Light::DIRECTIONAL);
        let rgb = [
            color_channel_f32(color, 16) * intensity,
            color_channel_f32(color, 8) * intensity,
            color_channel_f32(color, 0) * intensity,
        ];
        if mode == Light::AMBIENT {
            lighting.ambient_r += rgb[0];
            lighting.ambient_g += rgb[1];
            lighting.ambient_b += rgb[2];
            lighting.ambient_lights += 1;
            return Ok(());
        }

        let light_to_camera = multiply_matrix(view, light_to_world);
        let position = transform_point(light_to_camera, [0.0, 0.0, 0.0]);
        let direction = normalize3(transform_direction(light_to_camera, [0.0, 0.0, -1.0])).unwrap_or([0.0, 0.0, -1.0]);
        let scope = jvm.get_field::<i32>(&cast_ref::<Light, Node>(&light), "scope", "I").await.unwrap_or(-1);
        let spot_angle = jvm.get_field::<f32>(&light, "spotAngle", "F").await.unwrap_or(45.0).clamp(0.0, 90.0);
        lighting.lights.push(M3gLightState {
            mode,
            color: rgb,
            position,
            direction,
            constant: jvm.get_field::<f32>(&light, "constantAttenuation", "F").await.unwrap_or(1.0).max(0.0),
            linear: jvm.get_field::<f32>(&light, "linearAttenuation", "F").await.unwrap_or(0.0).max(0.0),
            quadratic: jvm.get_field::<f32>(&light, "quadraticAttenuation", "F").await.unwrap_or(0.0).max(0.0),
            spot_cos: (spot_angle * core::f32::consts::PI / 180.0).cos(),
            spot_exponent: jvm.get_field::<f32>(&light, "spotExponent", "F").await.unwrap_or(0.0).max(0.0),
            scope,
        });
        lighting.ambient_lights += 1;
        Ok(())
    }
}
