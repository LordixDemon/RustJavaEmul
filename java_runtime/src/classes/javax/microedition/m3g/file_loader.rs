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

use super::m3g_reader::M3gReader;
pub(super) struct M3gFileLoader<'a, 'b> {
    pub(super) jvm: &'a Jvm,
    pub(super) context: &'a mut RuntimeContext,
    pub(super) resource_name: &'a str,
    pub(super) history: &'b mut Vec<RustString>,
    pub(super) refs: Vec<LoadedObject>,
}

pub(super) struct LoadedObject {
    pub(super) object: ClassInstanceRef<Object3D>,
    pub(super) referenced: bool,
    pub(super) external: bool,
}

impl<'a, 'b> M3gFileLoader<'a, 'b> {
    pub(super) fn new(jvm: &'a Jvm, context: &'a mut RuntimeContext, resource_name: &'a str, history: &'b mut Vec<RustString>) -> Self {
        Self {
            jvm,
            context,
            resource_name,
            history,
            refs: Vec::new(),
        }
    }

    pub(super) async fn parse(mut self, bytes: &[u8]) -> Result<Vec<ClassInstanceRef<Object3D>>> {
        if !bytes.starts_with(M3G_IDENTIFIER) {
            return Err(self.jvm.exception("java/io/IOException", "bad M3G identifier").await);
        }

        let mut offset = M3G_IDENTIFIER.len();
        let mut section_index = 0;
        while offset + 9 <= bytes.len() {
            let compression = bytes[offset];
            let total_len = read_u32_at(bytes, offset + 1) as usize;
            let inflated_len = read_u32_at(bytes, offset + 5) as usize;
            if total_len < 13 || offset + total_len > bytes.len() {
                return Err(self.jvm.exception("java/io/IOException", "bad M3G section length").await);
            }
            let data_start = offset + 9;
            let checksum_start = offset + total_len - 4;
            let section = match decode_m3g_section_data(compression, &bytes[data_start..checksum_start], inflated_len) {
                Ok(section) => section,
                Err(message) => return Err(self.jvm.exception("java/io/IOException", message).await),
            };
            self.parse_section(section_index, section.as_slice()).await?;
            offset += total_len;
            section_index += 1;
        }

        let roots: Vec<ClassInstanceRef<Object3D>> = self
            .refs
            .iter()
            .filter(|loaded| !loaded.external && !loaded.referenced)
            .map(|loaded| loaded.object.clone())
            .collect();
        let result = if roots.is_empty() {
            self.refs
                .iter()
                .filter(|loaded| !loaded.external)
                .map(|loaded| loaded.object.clone())
                .collect()
        } else {
            roots
        };

        tracing::info!(
            target: "rustjava_m3g",
            "m3g.loader.parsed resource={} sections={} objects={} roots={}",
            self.resource_name,
            section_index,
            self.refs.len(),
            result.len()
        );

        Ok(result)
    }

    pub(super) async fn parse_section(&mut self, section_index: usize, section: &[u8]) -> Result<()> {
        let mut reader = M3gReader::new(section, self.reader_error("Malformed M3G section").await);
        while reader.remaining() >= 5 {
            let object_type = reader.read_u8()?;
            let length = reader.read_u32()? as usize;
            let payload = reader.read_slice(length)?;
            self.parse_object(section_index, object_type, payload).await?;
        }
        Ok(())
    }

    pub(super) async fn parse_object(&mut self, section_index: usize, object_type: u8, payload: &[u8]) -> Result<()> {
        let mut reader = M3gReader::new(payload, self.reader_error("Malformed M3G object").await);
        let object = match object_type {
            0 => {
                self.parse_header(section_index, &mut reader)?;
                None
            }
            1 => Some(self.parse_animation_controller(&mut reader).await?),
            2 => Some(self.parse_animation_track(&mut reader).await?),
            3 => Some(self.parse_appearance(&mut reader).await?),
            4 => Some(self.parse_background(&mut reader).await?),
            5 => Some(self.parse_camera(&mut reader).await?),
            6 => Some(self.parse_compositing_mode(&mut reader).await?),
            7 => Some(self.parse_fog(&mut reader).await?),
            8 => Some(self.parse_polygon_mode(&mut reader).await?),
            9 => Some(self.parse_group(&mut reader).await?),
            10 => Some(self.parse_image2d(&mut reader).await?),
            11 => Some(self.parse_triangle_strip_array(&mut reader).await?),
            12 => Some(self.parse_light(&mut reader).await?),
            13 => Some(self.parse_material(&mut reader).await?),
            14 => Some(self.parse_mesh(&mut reader).await?),
            15 => Some(self.parse_morphing_mesh(&mut reader).await?),
            16 => Some(self.parse_skinned_mesh(&mut reader).await?),
            17 => Some(self.parse_texture2d(&mut reader).await?),
            18 => Some(self.parse_sprite3d(&mut reader).await?),
            19 => Some(self.parse_keyframe_sequence(&mut reader).await?),
            20 => Some(self.parse_vertex_array(&mut reader).await?),
            21 => Some(self.parse_vertex_buffer(&mut reader).await?),
            22 => Some(self.parse_world(&mut reader).await?),
            255 => {
                let name = reader.read_string()?;
                let resolved_name = if !name.starts_with('/') {
                    let normalized_res = self.resource_name.replace('\\', "/");
                    if let Some(pos) = normalized_res.rfind('/') {
                        let dir = &normalized_res[..pos];
                        format!("{}/{}", dir.trim_start_matches('/'), name.trim_start_matches('/'))
                    } else {
                        name.clone()
                    }
                } else {
                    name.clone()
                };
                let objects = match Loader::load_resource_objects(self.jvm, self.context, &resolved_name, self.history).await {
                    Ok(objects) => objects,
                    Err(e) => {
                        if resolved_name != name {
                            match Loader::load_resource_objects(self.jvm, self.context, &name, self.history).await {
                                Ok(objects) => objects,
                                Err(_) => return Err(e),
                            }
                        } else {
                            return Err(e);
                        }
                    }
                };
                let object = if let Some(object) = objects.into_iter().next() {
                    object
                } else {
                    self.placeholder_object().await?
                };
                self.refs.push(LoadedObject {
                    object,
                    referenced: false,
                    external: true,
                });
                None
            }
            _ => {
                tracing::warn!(target: "rustjava_m3g", "unsupported M3G object type {} in {}", object_type, self.resource_name);
                Some(self.placeholder_object().await?)
            }
        };

        if let Some(object) = object {
            self.refs.push(LoadedObject {
                object,
                referenced: false,
                external: false,
            });
        }
        Ok(())
    }

    pub(super) async fn placeholder_object(&self) -> Result<ClassInstanceRef<Object3D>> {
        Ok(self.jvm.new_class("javax/microedition/m3g/Group", "()V", ()).await?.into())
    }

    pub(super) async fn reader_error(&self, message: &str) -> Box<dyn ClassInstance> {
        match self.jvm.exception("java/io/IOException", message).await {
            JavaError::JavaException(exception) => exception,
        }
    }

    pub(super) fn parse_header(&self, section_index: usize, reader: &mut M3gReader<'_>) -> Result<()> {
        let major = reader.read_u8()?;
        let minor = reader.read_u8()?;
        let external_refs = reader.read_bool()?;
        let file_size = reader.read_u32()?;
        let content_size = reader.read_u32()?;
        let author = reader.read_string()?;
        tracing::info!(
            target: "rustjava_m3g",
            "m3g.header resource={} section={} version={}.{} external={} fileSize={} contentSize={} author={:?}",
            self.resource_name,
            section_index,
            major,
            minor,
            external_refs,
            file_size,
            content_size,
            author
        );
        Ok(())
    }

    pub(super) async fn parse_object3d(&mut self, reader: &mut M3gReader<'_>, object: &mut ClassInstanceRef<Object3D>) -> Result<()> {
        let user_id = reader.read_i32()?;
        self.jvm.put_field(object, "userID", "I", user_id).await?;
        let animation_tracks = reader.read_u32()? as usize;
        let mut tracks = Vec::with_capacity(animation_tracks);
        for _ in 0..animation_tracks {
            tracks.push(cast_ref::<Object3D, AnimationTrack>(&self.get_ref(reader.read_i32()?)?));
        }
        let user_params = reader.read_u32()? as usize;
        for _ in 0..user_params {
            let _id = reader.read_u32()?;
            let len = reader.read_u32()? as usize;
            reader.skip(len)?;
        }
        let mut track_array = self
            .jvm
            .instantiate_array("Ljavax/microedition/m3g/AnimationTrack;", tracks.len())
            .await?;
        self.jvm.store_array(&mut track_array, 0, tracks).await?;
        self.jvm
            .put_field(object, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", track_array)
            .await?;
        Ok(())
    }

    pub(super) async fn parse_animation_controller(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut controller: ClassInstanceRef<AnimationController> =
            self.jvm.new_class("javax/microedition/m3g/AnimationController", "()V", ()).await?.into();
        let mut object = cast_ref(&controller);
        self.parse_object3d(reader, &mut object).await?;
        let speed = reader.read_f32()?;
        let weight = reader.read_f32()?;
        let active_start = reader.read_i32()?;
        let active_end = reader.read_i32()?;
        let ref_sequence_time = reader.read_f32()?;
        let ref_world_time = reader.read_i32()?;
        self.jvm.put_field(&mut controller, "speed", "F", speed).await?;
        self.jvm.put_field(&mut controller, "weight", "F", weight).await?;
        self.jvm.put_field(&mut controller, "activeIntervalStart", "I", active_start).await?;
        self.jvm.put_field(&mut controller, "activeIntervalEnd", "I", active_end).await?;
        self.jvm.put_field(&mut controller, "refSequenceTime", "F", ref_sequence_time).await?;
        self.jvm.put_field(&mut controller, "refWorldTime", "I", ref_world_time).await?;
        Ok(cast_ref(&controller))
    }

    pub(super) async fn parse_animation_track(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut track: ClassInstanceRef<AnimationTrack> = self.jvm.new_class("javax/microedition/m3g/AnimationTrack", "()V", ()).await?.into();
        let mut object = cast_ref(&track);
        self.parse_object3d(reader, &mut object).await?;
        let sequence = cast_ref::<Object3D, KeyframeSequence>(&self.get_ref(reader.read_i32()?)?);
        let controller = cast_ref::<Object3D, AnimationController>(&self.get_ref(reader.read_i32()?)?);
        let target_property = reader.read_i32()?;
        self.jvm
            .put_field(&mut track, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;", sequence)
            .await?;
        self.jvm
            .put_field(&mut track, "controller", "Ljavax/microedition/m3g/AnimationController;", controller)
            .await?;
        self.jvm.put_field(&mut track, "targetProperty", "I", target_property).await?;
        Ok(cast_ref(&track))
    }

    pub(super) async fn parse_keyframe_sequence(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut sequence: ClassInstanceRef<KeyframeSequence> = self
            .jvm
            .new_class("javax/microedition/m3g/KeyframeSequence", "(III)V", (1, 1, KeyframeSequence::LINEAR))
            .await?
            .into();
        let mut object = cast_ref(&sequence);
        self.parse_object3d(reader, &mut object).await?;
        let interpolation = reader.read_u8()? as i32;
        let repeat_mode = reader.read_u8()? as i32;
        let encoding = reader.read_u8()?;
        let duration = reader.read_u32()? as i32;
        let valid_first = reader.read_u32()? as i32;
        let valid_last = reader.read_u32()? as i32;
        let component_count = reader.read_u32()? as i32;
        let keyframe_count = reader.read_u32()? as i32;
        let mut times = Vec::with_capacity(keyframe_count.max(0) as usize);
        let mut values = Vec::with_capacity((keyframe_count.max(0) * component_count.max(0)) as usize);

        match encoding {
            0 => {
                for _ in 0..keyframe_count.max(0) {
                    times.push(reader.read_u32()? as i32);
                    for _ in 0..component_count.max(0) {
                        values.push(reader.read_f32()?);
                    }
                }
            }
            1 => {
                let bias = Self::read_f32_vec(reader, component_count.max(0) as usize)?;
                let scale = Self::read_f32_vec(reader, component_count.max(0) as usize)?;
                for _ in 0..keyframe_count.max(0) {
                    times.push(reader.read_u32()? as i32);
                    for component in 0..component_count.max(0) as usize {
                        let raw = reader.read_u8()? as f32 / 255.0;
                        values.push(bias[component] + raw * scale[component]);
                    }
                }
            }
            2 => {
                let bias = Self::read_f32_vec(reader, component_count.max(0) as usize)?;
                let scale = Self::read_f32_vec(reader, component_count.max(0) as usize)?;
                for _ in 0..keyframe_count.max(0) {
                    times.push(reader.read_u32()? as i32);
                    for component in 0..component_count.max(0) as usize {
                        let raw = reader.read_u16()? as f32 / 65535.0;
                        values.push(bias[component] + raw * scale[component]);
                    }
                }
            }
            _ => return Err(self.jvm.exception("java/io/IOException", "unsupported KeyframeSequence encoding").await),
        }

        KeyframeSequence::put_data(
            self.jvm,
            &mut sequence,
            interpolation,
            repeat_mode,
            duration,
            valid_first,
            valid_last,
            component_count,
            times,
            values,
        )
        .await?;
        Ok(cast_ref(&sequence))
    }

    pub(super) fn read_f32_vec(reader: &mut M3gReader<'_>, count: usize) -> Result<Vec<f32>> {
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(reader.read_f32()?);
        }
        Ok(values)
    }

    pub(super) async fn parse_transformable(&mut self, reader: &mut M3gReader<'_>, object: &mut ClassInstanceRef<Transformable>) -> Result<()> {
        let mut object3d: ClassInstanceRef<Object3D> = cast_ref(object);
        self.parse_object3d(reader, &mut object3d).await?;
        if reader.read_bool()? {
            let tx = reader.read_f32()?;
            let ty = reader.read_f32()?;
            let tz = reader.read_f32()?;
            let sx = reader.read_f32()?;
            let sy = reader.read_f32()?;
            let sz = reader.read_f32()?;
            let angle = reader.read_f32()?;
            let ax = reader.read_f32()?;
            let ay = reader.read_f32()?;
            let az = reader.read_f32()?;
            Transformable::set_translation_fields(self.jvm, object, tx, ty, tz).await?;
            Transformable::set_scale_fields(self.jvm, object, sx, sy, sz).await?;
            Transformable::set_orientation_fields(self.jvm, object, angle, ax, ay, az).await?;
        }
        if reader.read_bool()? {
            let mut matrix = [0.0; 16];
            for item in &mut matrix {
                *item = reader.read_f32()?;
            }
            Transformable::put_generic_matrix(self.jvm, object, matrix).await?;
        }
        Ok(())
    }

    pub(super) async fn parse_node(&mut self, reader: &mut M3gReader<'_>, object: &mut ClassInstanceRef<Node>) -> Result<()> {
        let mut transformable: ClassInstanceRef<Transformable> = cast_ref(object);
        self.parse_transformable(reader, &mut transformable).await?;
        let rendering_enabled = reader.read_bool()?;
        let picking_enabled = reader.read_bool()?;
        let alpha = reader.read_u8()? as f32 / 255.0;
        let scope = reader.read_i32()?;
        self.jvm.put_field(object, "renderingEnabled", "Z", rendering_enabled).await?;
        self.jvm.put_field(object, "pickingEnabled", "Z", picking_enabled).await?;
        self.jvm.put_field(object, "alphaFactor", "F", alpha).await?;
        self.jvm.put_field(object, "scope", "I", scope).await?;
        if reader.read_bool()? {
            let z_target = reader.read_u8()? as i32;
            let z_reference = cast_ref::<Object3D, Node>(&self.get_ref(reader.read_i32()?)?);
            let y_target = reader.read_u8()? as i32;
            let y_reference = cast_ref::<Object3D, Node>(&self.get_ref(reader.read_i32()?)?);
            self.jvm
                .put_field(object, "zReference", "Ljavax/microedition/m3g/Node;", z_reference)
                .await?;
            self.jvm
                .put_field(object, "yReference", "Ljavax/microedition/m3g/Node;", y_reference)
                .await?;
            self.jvm.put_field(object, "zTarget", "I", z_target).await?;
            self.jvm.put_field(object, "yTarget", "I", y_target).await?;
        }
        Ok(())
    }

    pub(super) async fn parse_camera(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut camera: ClassInstanceRef<Camera> = self.jvm.new_class("javax/microedition/m3g/Camera", "()V", ()).await?.into();
        let mut node = cast_ref(&camera);
        self.parse_node(reader, &mut node).await?;
        match reader.read_u8()? {
            48 => {
                let mut matrix = [0.0; 16];
                for item in &mut matrix {
                    *item = reader.read_f32()?;
                }
                self.jvm.put_field(&mut camera, "projectionMode", "I", Camera::GENERIC).await?;
                Camera::put_generic_projection(self.jvm, &mut camera, matrix).await?;
            }
            49 => {
                let height = reader.read_f32()?;
                let aspect = reader.read_f32()?;
                let near = reader.read_f32()?;
                let far = reader.read_f32()?;
                Camera::set_parallel(self.jvm, self.context, camera.clone(), height, aspect, near, far).await?;
            }
            50 => {
                let f1 = reader.read_f32()?;
                let f2 = reader.read_f32()?;
                let f3 = reader.read_f32()?;
                let f4 = reader.read_f32()?;
                Camera::set_perspective(self.jvm, self.context, camera.clone(), f1, f2, f3, f4).await?;
            }
            _ => {}
        }
        Ok(cast_ref(&camera))
    }

    pub(super) async fn parse_background(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut background: ClassInstanceRef<Background> = self.jvm.new_class("javax/microedition/m3g/Background", "()V", ()).await?.into();
        let mut object = cast_ref(&background);
        self.parse_object3d(reader, &mut object).await?;
        let color = reader.read_argb()?;
        let image = cast_ref::<Object3D, Image2D>(&self.get_ref(reader.read_i32()?)?);
        let image_mode_x = reader.read_u8()? as i32;
        let image_mode_y = reader.read_u8()? as i32;
        let crop_x = reader.read_i32()?;
        let crop_y = reader.read_i32()?;
        let crop_w = reader.read_i32()?;
        let crop_h = reader.read_i32()?;
        let color_clear = reader.read_bool()?;
        let depth_clear = reader.read_bool()?;
        self.jvm.put_field(&mut background, "color", "I", color).await?;
        self.jvm
            .put_field(&mut background, "image", "Ljavax/microedition/m3g/Image2D;", image)
            .await?;
        self.jvm.put_field(&mut background, "imageModeX", "I", image_mode_x).await?;
        self.jvm.put_field(&mut background, "imageModeY", "I", image_mode_y).await?;
        self.jvm.put_field(&mut background, "cropX", "I", crop_x).await?;
        self.jvm.put_field(&mut background, "cropY", "I", crop_y).await?;
        self.jvm.put_field(&mut background, "cropW", "I", crop_w).await?;
        self.jvm.put_field(&mut background, "cropH", "I", crop_h).await?;
        self.jvm.put_field(&mut background, "colorClear", "Z", color_clear).await?;
        self.jvm.put_field(&mut background, "depthClear", "Z", depth_clear).await?;
        Ok(cast_ref(&background))
    }

    pub(super) async fn parse_compositing_mode(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut mode: ClassInstanceRef<CompositingMode> = self.jvm.new_class("javax/microedition/m3g/CompositingMode", "()V", ()).await?.into();
        let mut object = cast_ref(&mode);
        self.parse_object3d(reader, &mut object).await?;
        let depth_test = reader.read_bool()?;
        let depth_write = reader.read_bool()?;
        let color_write = reader.read_bool()?;
        let alpha_write = reader.read_bool()?;
        let blending = reader.read_u8()? as i32;
        let alpha_threshold = reader.read_u8()? as f32 / 255.0;
        let depth_factor = reader.read_f32()?;
        let depth_units = reader.read_f32()?;
        self.jvm.put_field(&mut mode, "depthTest", "Z", depth_test).await?;
        self.jvm.put_field(&mut mode, "depthWrite", "Z", depth_write).await?;
        self.jvm.put_field(&mut mode, "colorWrite", "Z", color_write).await?;
        self.jvm.put_field(&mut mode, "alphaWrite", "Z", alpha_write).await?;
        self.jvm.put_field(&mut mode, "blending", "I", blending).await?;
        self.jvm.put_field(&mut mode, "alphaThreshold", "F", alpha_threshold).await?;
        self.jvm.put_field(&mut mode, "depthOffsetFactor", "F", depth_factor).await?;
        self.jvm.put_field(&mut mode, "depthOffsetUnits", "F", depth_units).await?;
        Ok(cast_ref(&mode))
    }

    pub(super) async fn parse_fog(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut fog: ClassInstanceRef<Fog> = self.jvm.new_class("javax/microedition/m3g/Fog", "()V", ()).await?.into();
        let mut object = cast_ref(&fog);
        self.parse_object3d(reader, &mut object).await?;
        let color = reader.read_rgb()?;
        let mode = reader.read_u8()? as i32;
        let (density, near, far) = if mode == Fog::EXPONENTIAL {
            (reader.read_f32()?, 0.0, 0.0)
        } else {
            (1.0, reader.read_f32()?, reader.read_f32()?)
        };
        self.jvm.put_field(&mut fog, "color", "I", color).await?;
        self.jvm.put_field(&mut fog, "density", "F", density).await?;
        self.jvm.put_field(&mut fog, "mode", "I", mode).await?;
        self.jvm.put_field(&mut fog, "near", "F", near).await?;
        self.jvm.put_field(&mut fog, "far", "F", far).await?;
        Ok(cast_ref(&fog))
    }

    pub(super) async fn parse_polygon_mode(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut mode: ClassInstanceRef<PolygonMode> = self.jvm.new_class("javax/microedition/m3g/PolygonMode", "()V", ()).await?.into();
        let mut object = cast_ref(&mode);
        self.parse_object3d(reader, &mut object).await?;
        let culling = reader.read_u8()? as i32;
        let shading = reader.read_u8()? as i32;
        let winding = reader.read_u8()? as i32;
        let two_sided = reader.read_bool()?;
        let local_camera = reader.read_bool()?;
        let perspective = reader.read_bool()?;
        self.jvm.put_field(&mut mode, "culling", "I", culling).await?;
        self.jvm.put_field(&mut mode, "shading", "I", shading).await?;
        self.jvm.put_field(&mut mode, "winding", "I", winding).await?;
        self.jvm.put_field(&mut mode, "twoSidedLighting", "Z", two_sided).await?;
        self.jvm.put_field(&mut mode, "localCameraLighting", "Z", local_camera).await?;
        self.jvm.put_field(&mut mode, "perspectiveCorrection", "Z", perspective).await?;
        Ok(cast_ref(&mode))
    }

    pub(super) async fn parse_appearance(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut appearance: ClassInstanceRef<Appearance> = self.jvm.new_class("javax/microedition/m3g/Appearance", "()V", ()).await?.into();
        let mut object = cast_ref(&appearance);
        self.parse_object3d(reader, &mut object).await?;
        let layer = reader.read_i8()? as i32;
        let compositing = cast_ref::<Object3D, CompositingMode>(&self.get_ref(reader.read_i32()?)?);
        let fog = cast_ref::<Object3D, Fog>(&self.get_ref(reader.read_i32()?)?);
        let polygon = cast_ref::<Object3D, PolygonMode>(&self.get_ref(reader.read_i32()?)?);
        let material = cast_ref::<Object3D, Material>(&self.get_ref(reader.read_i32()?)?);
        let texture_count = reader.read_u32()? as usize;
        let mut texture0 = null_ref();
        let mut texture1 = null_ref();
        for index in 0..texture_count {
            let texture = cast_ref::<Object3D, Texture2D>(&self.get_ref(reader.read_i32()?)?);
            match index {
                0 => texture0 = texture,
                1 => texture1 = texture,
                _ => {}
            }
        }
        self.jvm
            .put_field(
                &mut appearance,
                "compositingMode",
                "Ljavax/microedition/m3g/CompositingMode;",
                compositing,
            )
            .await?;
        self.jvm.put_field(&mut appearance, "fog", "Ljavax/microedition/m3g/Fog;", fog).await?;
        self.jvm.put_field(&mut appearance, "layer", "I", layer).await?;
        self.jvm
            .put_field(&mut appearance, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;", polygon)
            .await?;
        self.jvm
            .put_field(&mut appearance, "material", "Ljavax/microedition/m3g/Material;", material)
            .await?;
        self.jvm
            .put_field(&mut appearance, "texture0", "Ljavax/microedition/m3g/Texture2D;", texture0)
            .await?;
        self.jvm
            .put_field(&mut appearance, "texture1", "Ljavax/microedition/m3g/Texture2D;", texture1)
            .await?;
        Ok(cast_ref(&appearance))
    }

    pub(super) async fn parse_group(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut group: ClassInstanceRef<Group> = self.jvm.new_class("javax/microedition/m3g/Group", "()V", ()).await?.into();
        let mut node = cast_ref(&group);
        self.parse_node(reader, &mut node).await?;
        self.parse_group_children(reader, &mut group).await?;
        Ok(cast_ref(&group))
    }

    pub(super) async fn parse_group_children(&mut self, reader: &mut M3gReader<'_>, group: &mut ClassInstanceRef<Group>) -> Result<()> {
        let child_count = reader.read_u32()? as usize;
        for _ in 0..child_count {
            let child = cast_ref::<Object3D, Node>(&self.get_ref(reader.read_i32()?)?);
            Group::push_child(self.jvm, group, child).await?;
        }
        Ok(())
    }

    pub(super) async fn parse_world(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut world: ClassInstanceRef<World> = self.jvm.new_class("javax/microedition/m3g/World", "()V", ()).await?.into();
        let mut node = cast_ref(&world);
        self.parse_node(reader, &mut node).await?;
        let mut group = cast_ref(&world);
        self.parse_group_children(reader, &mut group).await?;
        let camera = cast_ref::<Object3D, Camera>(&self.get_ref(reader.read_i32()?)?);
        let background = cast_ref::<Object3D, Background>(&self.get_ref(reader.read_i32()?)?);
        self.jvm
            .put_field(&mut world, "activeCamera", "Ljavax/microedition/m3g/Camera;", camera)
            .await?;
        self.jvm
            .put_field(&mut world, "background", "Ljavax/microedition/m3g/Background;", background)
            .await?;
        Ok(cast_ref(&world))
    }
}
