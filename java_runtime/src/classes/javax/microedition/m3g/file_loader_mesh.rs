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

use super::file_loader::M3gFileLoader;
use super::m3g_reader::M3gReader;
impl<'a, 'b> M3gFileLoader<'a, 'b> {
    pub(super) async fn parse_image2d(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut image: ClassInstanceRef<Image2D> = self.jvm.new_class("javax/microedition/m3g/Image2D", "()V", ()).await?.into();
        let mut object = cast_ref(&image);
        self.parse_object3d(reader, &mut object).await?;
        let format = reader.read_u8()? as i32;
        let mutable = reader.read_bool()?;
        let width = (reader.read_u32()? as i32).max(1);
        let height = (reader.read_u32()? as i32).max(1);
        let argb = if mutable {
            vec![0; (width * height) as usize]
        } else {
            let palette = reader.read_byte_array()?;
            let pixels = reader.read_byte_array()?;
            decode_m3g_image_pixels(format, width, height, palette, pixels)
        };
        let lcdui_image = Image::from_argb(self.jvm, width, height, argb).await?;
        self.jvm.put_field(&mut image, "format", "I", format).await?;
        self.jvm.put_field(&mut image, "width", "I", width).await?;
        self.jvm.put_field(&mut image, "height", "I", height).await?;
        self.jvm
            .put_field(&mut image, "image", "Ljavax/microedition/lcdui/Image;", lcdui_image)
            .await?;
        self.jvm.put_field(&mut image, "mutable", "Z", mutable).await?;
        Ok(cast_ref(&image))
    }

    pub(super) async fn parse_triangle_strip_array(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut tsa: ClassInstanceRef<TriangleStripArray> = self.jvm.new_class("javax/microedition/m3g/TriangleStripArray", "()V", ()).await?.into();
        let mut object = cast_ref(&tsa);
        self.parse_object3d(reader, &mut object).await?;
        let encoding = reader.read_u8()?;
        let indices = match encoding {
            0 => {
                let start = reader.read_i32()?;
                vec![start]
            }
            1 => vec![reader.read_u8()? as i32],
            2 => vec![reader.read_u16()? as i32],
            128 => {
                let count = reader.read_u32()? as usize;
                let mut values = Vec::with_capacity(count);
                for _ in 0..count {
                    values.push(reader.read_i32()?);
                }
                values
            }
            129 => {
                let count = reader.read_u32()? as usize;
                let mut values = Vec::with_capacity(count);
                for _ in 0..count {
                    values.push(reader.read_u8()? as i32);
                }
                values
            }
            130 => {
                let count = reader.read_u32()? as usize;
                let mut values = Vec::with_capacity(count);
                for _ in 0..count {
                    values.push(reader.read_u16()? as i32);
                }
                values
            }
            _ => Vec::new(),
        };
        let length_count = reader.read_u32()? as usize;
        let mut lengths = Vec::with_capacity(length_count);
        for _ in 0..length_count {
            lengths.push(reader.read_i32()?);
        }
        let mut indices_array = self.jvm.instantiate_array("I", indices.len()).await?;
        self.jvm.store_array(&mut indices_array, 0, indices).await?;
        let mut lengths_array = self.jvm.instantiate_array("I", lengths.len()).await?;
        self.jvm.store_array(&mut lengths_array, 0, lengths).await?;
        self.jvm.put_field(&mut tsa, "indices", "[I", indices_array).await?;
        self.jvm.put_field(&mut tsa, "stripLengths", "[I", lengths_array).await?;
        Ok(cast_ref(&tsa))
    }

    pub(super) async fn parse_vertex_array(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut vertex_array: ClassInstanceRef<VertexArray> = self.jvm.new_class("javax/microedition/m3g/VertexArray", "()V", ()).await?.into();
        let mut object = cast_ref(&vertex_array);
        self.parse_object3d(reader, &mut object).await?;
        let component_size = reader.read_u8()? as i32;
        let component_count = reader.read_u8()? as i32;
        let encoding = reader.read_u8()?;
        let vertex_count = reader.read_u16()? as i32;
        self.jvm.put_field(&mut vertex_array, "componentSize", "I", component_size).await?;
        self.jvm.put_field(&mut vertex_array, "componentCount", "I", component_count).await?;
        self.jvm.put_field(&mut vertex_array, "vertexCount", "I", vertex_count).await?;
        self.jvm.put_field(&mut vertex_array, "version", "I", 1i32).await?;
        let value_count = (vertex_count * component_count).max(0) as usize;
        if component_size == 1 {
            let mut values = Vec::with_capacity(value_count);
            let mut previous = [0i8; 4];
            for index in 0..value_count {
                let component = index % component_count as usize;
                let value = reader.read_u8()? as i8;
                previous[component] = if encoding == 0 { value } else { previous[component].wrapping_add(value) };
                values.push(previous[component]);
            }
            let mut array = self.jvm.instantiate_array("B", values.len()).await?;
            self.jvm.store_array(&mut array, 0, values).await?;
            self.jvm.put_field(&mut vertex_array, "byteData", "[B", array).await?;
        } else {
            let mut values = Vec::with_capacity(value_count);
            let mut previous = [0i16; 4];
            for index in 0..value_count {
                let component = index % component_count as usize;
                let value = reader.read_i16()?;
                previous[component] = if encoding == 0 { value } else { previous[component].wrapping_add(value) };
                values.push(previous[component]);
            }
            let mut array = self.jvm.instantiate_array("S", values.len()).await?;
            self.jvm.store_array(&mut array, 0, values).await?;
            self.jvm.put_field(&mut vertex_array, "shortData", "[S", array).await?;
        }
        Ok(cast_ref(&vertex_array))
    }

    pub(super) async fn parse_vertex_buffer(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut buffer: ClassInstanceRef<VertexBuffer> = self.jvm.new_class("javax/microedition/m3g/VertexBuffer", "()V", ()).await?.into();
        let mut object = cast_ref(&buffer);
        self.parse_object3d(reader, &mut object).await?;
        let default_color = reader.read_argb()?;
        let positions = cast_ref::<Object3D, VertexArray>(&self.get_ref(reader.read_i32()?)?);
        let bias = vec![reader.read_f32()?, reader.read_f32()?, reader.read_f32()?];
        let scale = reader.read_f32()?;
        let normals = cast_ref::<Object3D, VertexArray>(&self.get_ref(reader.read_i32()?)?);
        let colors = cast_ref::<Object3D, VertexArray>(&self.get_ref(reader.read_i32()?)?);
        let tex_count = reader.read_u32()? as usize;
        let mut tex_coords0 = null_ref();
        let mut tex_bias = vec![0.0, 0.0, 0.0];
        let mut tex_scale = 1.0;
        let mut tex_coords1 = null_ref();
        let mut tex_bias1 = vec![0.0, 0.0, 0.0];
        let mut tex_scale1 = 1.0;
        for index in 0..tex_count {
            let tex = cast_ref::<Object3D, VertexArray>(&self.get_ref(reader.read_i32()?)?);
            let bias_values = vec![reader.read_f32()?, reader.read_f32()?, reader.read_f32()?];
            let scale_value = reader.read_f32()?;
            if index == 0 {
                tex_coords0 = tex;
                tex_bias = bias_values;
                tex_scale = scale_value;
            } else if index == 1 {
                tex_coords1 = tex;
                tex_bias1 = bias_values;
                tex_scale1 = scale_value;
            }
        }
        let mut bias_array = self.jvm.instantiate_array("F", bias.len()).await?;
        self.jvm.store_array(&mut bias_array, 0, bias).await?;
        let mut tex_bias_array = self.jvm.instantiate_array("F", tex_bias.len()).await?;
        self.jvm.store_array(&mut tex_bias_array, 0, tex_bias).await?;
        let mut tex_bias1_array = self.jvm.instantiate_array("F", tex_bias1.len()).await?;
        self.jvm.store_array(&mut tex_bias1_array, 0, tex_bias1).await?;
        self.jvm.put_field(&mut buffer, "defaultColor", "I", default_color).await?;
        self.jvm
            .put_field(&mut buffer, "positions", "Ljavax/microedition/m3g/VertexArray;", positions)
            .await?;
        self.jvm
            .put_field(&mut buffer, "normals", "Ljavax/microedition/m3g/VertexArray;", normals)
            .await?;
        self.jvm
            .put_field(&mut buffer, "colors", "Ljavax/microedition/m3g/VertexArray;", colors)
            .await?;
        self.jvm
            .put_field(&mut buffer, "texCoords0", "Ljavax/microedition/m3g/VertexArray;", tex_coords0)
            .await?;
        self.jvm
            .put_field(&mut buffer, "texCoords1", "Ljavax/microedition/m3g/VertexArray;", tex_coords1)
            .await?;
        self.jvm.put_field(&mut buffer, "positionScale", "F", scale).await?;
        self.jvm.put_field(&mut buffer, "positionBias", "[F", bias_array).await?;
        self.jvm.put_field(&mut buffer, "texScale", "F", tex_scale).await?;
        self.jvm.put_field(&mut buffer, "texBias", "[F", tex_bias_array).await?;
        self.jvm.put_field(&mut buffer, "texScale1", "F", tex_scale1).await?;
        self.jvm.put_field(&mut buffer, "texBias1", "[F", tex_bias1_array).await?;
        Ok(cast_ref(&buffer))
    }

    pub(super) async fn parse_light(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut light: ClassInstanceRef<Light> = self.jvm.new_class("javax/microedition/m3g/Light", "()V", ()).await?.into();
        let mut node = cast_ref(&light);
        self.parse_node(reader, &mut node).await?;
        let constant_attenuation = reader.read_f32()?;
        let linear_attenuation = reader.read_f32()?;
        let quadratic_attenuation = reader.read_f32()?;
        let color = reader.read_rgb()?;
        let mode = reader.read_u8()? as i32;
        let intensity = reader.read_f32()?;
        let spot_angle = reader.read_f32()?;
        let spot_exponent = reader.read_f32()?;
        self.jvm.put_field(&mut light, "constantAttenuation", "F", constant_attenuation).await?;
        self.jvm.put_field(&mut light, "linearAttenuation", "F", linear_attenuation).await?;
        self.jvm.put_field(&mut light, "quadraticAttenuation", "F", quadratic_attenuation).await?;
        self.jvm.put_field(&mut light, "color", "I", color).await?;
        self.jvm.put_field(&mut light, "mode", "I", mode).await?;
        self.jvm.put_field(&mut light, "intensity", "F", intensity).await?;
        self.jvm.put_field(&mut light, "spotAngle", "F", spot_angle).await?;
        self.jvm.put_field(&mut light, "spotExponent", "F", spot_exponent).await?;
        Ok(cast_ref(&light))
    }

    pub(super) async fn parse_material(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut material: ClassInstanceRef<Material> = self.jvm.new_class("javax/microedition/m3g/Material", "()V", ()).await?.into();
        let mut object = cast_ref(&material);
        self.parse_object3d(reader, &mut object).await?;
        let ambient = reader.read_rgb()?;
        let diffuse = reader.read_argb()?;
        let emissive = reader.read_rgb()?;
        let specular = reader.read_rgb()?;
        let shininess = reader.read_f32()?;
        let vertex_color_tracking = reader.read_bool()?;
        self.jvm.put_field(&mut material, "ambientColor", "I", ensure_opaque(ambient)).await?;
        self.jvm.put_field(&mut material, "diffuseColor", "I", diffuse).await?;
        self.jvm.put_field(&mut material, "emissiveColor", "I", ensure_opaque(emissive)).await?;
        self.jvm.put_field(&mut material, "specularColor", "I", ensure_opaque(specular)).await?;
        self.jvm.put_field(&mut material, "shininess", "F", shininess).await?;
        self.jvm
            .put_field(&mut material, "vertexColorTracking", "Z", vertex_color_tracking)
            .await?;
        Ok(cast_ref(&material))
    }

    pub(super) async fn parse_mesh(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut mesh: ClassInstanceRef<Mesh> = self.jvm.new_class("javax/microedition/m3g/Mesh", "()V", ()).await?.into();
        let mut node = cast_ref(&mesh);
        self.parse_node(reader, &mut node).await?;
        let vertex_buffer = cast_ref::<Object3D, VertexBuffer>(&self.get_ref(reader.read_i32()?)?);
        let submesh_count = reader.read_u32()? as usize;
        let mut index_buffers = Vec::with_capacity(submesh_count);
        let mut appearances = Vec::with_capacity(submesh_count);
        for _ in 0..submesh_count {
            index_buffers.push(cast_ref::<Object3D, IndexBuffer>(&self.get_ref(reader.read_i32()?)?));
            appearances.push(cast_ref::<Object3D, Appearance>(&self.get_ref(reader.read_i32()?)?));
        }
        let mut index_array = self
            .jvm
            .instantiate_array("Ljavax/microedition/m3g/IndexBuffer;", index_buffers.len())
            .await?;
        self.jvm.store_array(&mut index_array, 0, index_buffers).await?;
        let mut appearance_array = self
            .jvm
            .instantiate_array("Ljavax/microedition/m3g/Appearance;", appearances.len())
            .await?;
        self.jvm.store_array(&mut appearance_array, 0, appearances).await?;
        self.jvm
            .put_field(&mut mesh, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", vertex_buffer)
            .await?;
        self.jvm.put_field(&mut mesh, "submeshCount", "I", submesh_count as i32).await?;
        self.jvm
            .put_field(&mut mesh, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", index_array)
            .await?;
        self.jvm
            .put_field(&mut mesh, "appearances", "[Ljavax/microedition/m3g/Appearance;", appearance_array)
            .await?;
        Ok(cast_ref(&mesh))
    }

    pub(super) async fn parse_morphing_mesh(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut mesh: ClassInstanceRef<MorphingMesh> = self.jvm.new_class("javax/microedition/m3g/MorphingMesh", "()V", ()).await?.into();
        let mut mesh_ref = cast_ref(&mesh);
        self.fill_mesh_fields(reader, &mut mesh_ref).await?;
        let target_count = reader.read_u32().unwrap_or(0) as usize;
        let mut targets = Vec::with_capacity(target_count);
        for _ in 0..target_count {
            targets.push(cast_ref::<Object3D, VertexBuffer>(&self.get_ref(reader.read_i32().unwrap_or(0))?));
        }
        let mut target_array = self.jvm.instantiate_array("Ljavax/microedition/m3g/VertexBuffer;", targets.len()).await?;
        self.jvm.store_array(&mut target_array, 0, targets).await?;
        let mut weights = Vec::with_capacity(target_count);
        for _ in 0..target_count {
            weights.push(reader.read_f32().unwrap_or(0.0));
        }
        let mut weight_array = self.jvm.instantiate_array("F", weights.len()).await?;
        self.jvm.store_array(&mut weight_array, 0, weights).await?;
        self.jvm
            .put_field(&mut mesh, "targets", "[Ljavax/microedition/m3g/VertexBuffer;", target_array)
            .await?;
        self.jvm.put_field(&mut mesh, "weights", "[F", weight_array).await?;
        Ok(cast_ref(&mesh))
    }

    pub(super) async fn parse_skinned_mesh(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut mesh: ClassInstanceRef<SkinnedMesh> = self.jvm.new_class("javax/microedition/m3g/SkinnedMesh", "()V", ()).await?.into();
        let mut mesh_ref = cast_ref(&mesh);
        self.fill_mesh_fields(reader, &mut mesh_ref).await?;
        let skeleton = cast_ref::<Object3D, Group>(&self.get_ref(reader.read_i32().unwrap_or(0))?);
        if !skeleton.is_null() {
            let mut skeleton_node: ClassInstanceRef<Node> = cast_ref(&skeleton);
            self.jvm
                .put_field(
                    &mut skeleton_node,
                    "parent",
                    "Ljavax/microedition/m3g/Node;",
                    cast_ref::<SkinnedMesh, Node>(&mesh),
                )
                .await?;
        }
        self.jvm
            .put_field(&mut mesh, "skeleton", "Ljavax/microedition/m3g/Group;", skeleton)
            .await?;
        let transform_count = reader.read_u32().unwrap_or(0);
        for _ in 0..transform_count {
            let bone = cast_ref::<Object3D, Node>(&self.get_ref(reader.read_i32().unwrap_or(0))?);
            let first = reader.read_u32().unwrap_or(0) as i32;
            let count = reader.read_u32().unwrap_or(0) as i32;
            let weight = reader.read_i32().unwrap_or(0);
            if !bone.is_null() && weight > 0 && first >= 0 && count > 0 {
                let _: () = self
                    .jvm
                    .invoke_virtual(&mesh, "addTransform", "(Ljavax/microedition/m3g/Node;III)V", (bone, weight, first, count))
                    .await?;
            }
        }
        Ok(cast_ref(&mesh))
    }

    pub(super) async fn fill_mesh_fields(&mut self, reader: &mut M3gReader<'_>, mesh: &mut ClassInstanceRef<Mesh>) -> Result<()> {
        let mut node = cast_ref(mesh);
        self.parse_node(reader, &mut node).await?;
        let vertex_buffer = cast_ref::<Object3D, VertexBuffer>(&self.get_ref(reader.read_i32()?)?);
        let submesh_count = reader.read_u32()? as usize;
        let mut index_buffers = Vec::with_capacity(submesh_count);
        let mut appearances = Vec::with_capacity(submesh_count);
        for _ in 0..submesh_count {
            index_buffers.push(cast_ref::<Object3D, IndexBuffer>(&self.get_ref(reader.read_i32()?)?));
            appearances.push(cast_ref::<Object3D, Appearance>(&self.get_ref(reader.read_i32()?)?));
        }
        let mut index_array = self
            .jvm
            .instantiate_array("Ljavax/microedition/m3g/IndexBuffer;", index_buffers.len())
            .await?;
        self.jvm.store_array(&mut index_array, 0, index_buffers).await?;
        let mut appearance_array = self
            .jvm
            .instantiate_array("Ljavax/microedition/m3g/Appearance;", appearances.len())
            .await?;
        self.jvm.store_array(&mut appearance_array, 0, appearances).await?;
        self.jvm
            .put_field(mesh, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", vertex_buffer)
            .await?;
        self.jvm.put_field(mesh, "submeshCount", "I", submesh_count as i32).await?;
        self.jvm
            .put_field(mesh, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", index_array)
            .await?;
        self.jvm
            .put_field(mesh, "appearances", "[Ljavax/microedition/m3g/Appearance;", appearance_array)
            .await
    }

    pub(super) async fn parse_texture2d(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut texture: ClassInstanceRef<Texture2D> = self.jvm.new_class("javax/microedition/m3g/Texture2D", "()V", ()).await?.into();
        let mut transformable = cast_ref(&texture);
        self.parse_transformable(reader, &mut transformable).await?;
        let image = cast_ref::<Object3D, Image2D>(&self.get_ref(reader.read_i32()?)?);
        let blend_color = reader.read_rgb()?;
        let blending = reader.read_u8()? as i32;
        let wrapping_s = reader.read_u8()? as i32;
        let wrapping_t = reader.read_u8()? as i32;
        let level_filter = reader.read_u8()? as i32;
        let image_filter = reader.read_u8()? as i32;
        self.jvm
            .put_field(&mut texture, "image", "Ljavax/microedition/m3g/Image2D;", image)
            .await?;
        self.jvm.put_field(&mut texture, "blendColor", "I", blend_color).await?;
        self.jvm.put_field(&mut texture, "blending", "I", blending).await?;
        self.jvm.put_field(&mut texture, "wrappingS", "I", wrapping_s).await?;
        self.jvm.put_field(&mut texture, "wrappingT", "I", wrapping_t).await?;
        self.jvm.put_field(&mut texture, "levelFilter", "I", level_filter).await?;
        self.jvm.put_field(&mut texture, "imageFilter", "I", image_filter).await?;
        Ok(cast_ref(&texture))
    }

    pub(super) async fn parse_sprite3d(&mut self, reader: &mut M3gReader<'_>) -> Result<ClassInstanceRef<Object3D>> {
        let mut sprite: ClassInstanceRef<Sprite3D> = self
            .jvm
            .new_class(
                "javax/microedition/m3g/Sprite3D",
                "(ZLjavax/microedition/m3g/Image2D;Ljavax/microedition/m3g/Appearance;)V",
                (false, null_ref::<Image2D>(), null_ref::<Appearance>()),
            )
            .await?
            .into();
        let mut node = cast_ref(&sprite);
        self.parse_node(reader, &mut node).await?;
        let image = cast_ref::<Object3D, Image2D>(&self.get_ref(reader.read_i32()?)?);
        let appearance = cast_ref::<Object3D, Appearance>(&self.get_ref(reader.read_i32()?)?);
        let scaled = reader.read_bool()?;
        let crop_x = reader.read_i32()?;
        let crop_y = reader.read_i32()?;
        let crop_w = reader.read_i32()?;
        let crop_h = reader.read_i32()?;
        self.jvm.put_field(&mut sprite, "scaled", "Z", scaled).await?;
        self.jvm
            .put_field(&mut sprite, "image", "Ljavax/microedition/m3g/Image2D;", image)
            .await?;
        self.jvm
            .put_field(&mut sprite, "appearance", "Ljavax/microedition/m3g/Appearance;", appearance)
            .await?;
        self.jvm.put_field(&mut sprite, "cropX", "I", crop_x).await?;
        self.jvm.put_field(&mut sprite, "cropY", "I", crop_y).await?;
        self.jvm.put_field(&mut sprite, "cropW", "I", crop_w).await?;
        self.jvm.put_field(&mut sprite, "cropH", "I", crop_h).await?;
        Ok(cast_ref(&sprite))
    }

    pub(super) fn get_ref(&mut self, raw_index: i32) -> Result<ClassInstanceRef<Object3D>> {
        if raw_index == 0 {
            return Ok(null_ref());
        }
        let index = raw_index - 2;
        if index < 0 || index as usize >= self.refs.len() {
            return Ok(null_ref());
        }
        let loaded = &mut self.refs[index as usize];
        loaded.referenced = true;
        Ok(loaded.object.clone())
    }
}
