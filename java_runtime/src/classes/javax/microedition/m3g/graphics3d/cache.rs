#[allow(unused_imports)]
use super::super::common::*;
#[allow(unused_imports)]
use super::super::math::*;
#[allow(unused_imports)]
use super::super::prelude::*;
#[allow(unused_imports)]
use super::super::raw_arrays::*;
#[allow(unused_imports)]
use super::super::render::*;
#[allow(unused_imports)]
use super::super::types::*;

impl super::super::Graphics3D {
    pub(crate) async fn node_local_matrix(jvm: &Jvm, node: &ClassInstanceRef<Node>) -> Result<[f32; 16]> {
        Transformable::local_matrix(jvm, &cast_ref::<Node, Transformable>(node)).await
    }

    pub(crate) async fn camera_view_matrix(jvm: &Jvm, camera: &ClassInstanceRef<Camera>) -> Result<[f32; 16]> {
        if camera.is_null() {
            return Ok(identity_matrix());
        }

        let mut chain = Vec::new();
        let mut current: ClassInstanceRef<Node> = cast_ref(camera);
        while !current.is_null() {
            if current.class_definition().name() == "javax/microedition/m3g/World" {
                break;
            }
            chain.push(current.clone());
            current = jvm
                .get_field(&current, "parent", "Ljavax/microedition/m3g/Node;")
                .await
                .unwrap_or_else(|_| null_ref());
        }

        let mut world = identity_matrix();
        for node in chain.into_iter().rev() {
            let local = Self::node_local_matrix(jvm, &node).await.unwrap_or_else(|_| identity_matrix());
            world = multiply_matrix(world, local);
        }
        Ok(invert_affine_matrix(world).unwrap_or_else(identity_matrix))
    }

    pub(crate) async fn float_array_field(
        jvm: &Jvm,
        vertex_buffer: &ClassInstanceRef<VertexBuffer>,
        name: &str,
        len: usize,
        default: f32,
    ) -> Result<Vec<f32>> {
        let array: ClassInstanceRef<Array<f32>> = jvm.get_field(vertex_buffer, name, "[F").await.unwrap_or_else(|_| null_ref());
        if array.is_null() {
            return Ok(vec![default; len]);
        }
        let mut values = raw_f32_array(jvm, &array, len).await?;
        values.resize(len, default);
        Ok(values)
    }

    pub(crate) async fn vertex_array_values(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>) -> Result<(usize, usize, Arc<Vec<f32>>)> {
        if array.is_null() {
            return Ok((0, 0, Arc::new(Vec::new())));
        }
        let key = instance_key(array);
        let version = jvm.get_field::<i32>(array, "version", "I").await.unwrap_or(0);
        if key != 0 {
            let cache = M3G_VERTEX_VALUES_CACHE.lock();
            if let Some(entry) = cache.iter().find(|entry| entry.key == key && entry.version == version) {
                return Ok((entry.component_count, entry.vertex_count, entry.values.clone()));
            }
        }
        let component_size: i32 = jvm.get_field(array, "componentSize", "I").await?;
        let component_count: i32 = jvm.get_field(array, "componentCount", "I").await?;
        let vertex_count: i32 = jvm.get_field(array, "vertexCount", "I").await?;
        if component_count <= 0 || vertex_count <= 0 {
            return Ok((0, 0, Arc::new(Vec::new())));
        }
        let component_count = component_count as usize;
        let vertex_count = vertex_count as usize;
        let value_count = component_count.saturating_mul(vertex_count);
        let values: Vec<f32> = if component_size == 1 {
            let data: ClassInstanceRef<Array<i8>> = jvm.get_field(array, "byteData", "[B").await?;
            if data.is_null() {
                return Ok((component_count, vertex_count, Arc::new(Vec::new())));
            }
            raw_u8_array(jvm, &data, value_count)
                .await?
                .into_iter()
                .map(|value| value as i8 as f32)
                .collect()
        } else {
            let data: ClassInstanceRef<Array<i16>> = jvm.get_field(array, "shortData", "[S").await?;
            if data.is_null() {
                return Ok((component_count, vertex_count, Arc::new(Vec::new())));
            }
            raw_i16_array(jvm, &data, value_count)
                .await?
                .into_iter()
                .map(|value| value as f32)
                .collect()
        };
        let values = Arc::new(values);
        if key != 0 {
            let mut cache = M3G_VERTEX_VALUES_CACHE.lock();
            push_cache_entry(
                &mut cache,
                VertexValuesCacheEntry {
                    key,
                    version,
                    component_count,
                    vertex_count,
                    values: values.clone(),
                },
            );
        }
        Ok((component_count, vertex_count, values))
    }

    pub(crate) async fn vertex_array_vec3(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>, scale: f32, bias: &[f32]) -> Result<Arc<Vec<[f32; 3]>>> {
        let key = instance_key(array);
        let version = if array.is_null() {
            0
        } else {
            jvm.get_field::<i32>(array, "version", "I").await.unwrap_or(0)
        };
        let bias_x = bias.first().copied().unwrap_or(0.0);
        let bias_y = bias.get(1).copied().unwrap_or(0.0);
        let bias_z = bias.get(2).copied().unwrap_or(0.0);
        let scale_bits = scale.to_bits();
        let bias_bits = [bias_x.to_bits(), bias_y.to_bits(), bias_z.to_bits()];
        if key != 0 {
            let cache = M3G_VERTEX_VEC3_CACHE.lock();
            if let Some(entry) = cache
                .iter()
                .find(|entry| entry.key == key && entry.version == version && entry.scale_bits == scale_bits && entry.bias_bits == bias_bits)
            {
                return Ok(entry.values.clone());
            }
        }
        let (component_count, vertex_count, values) = Self::vertex_array_values(jvm, array).await?;
        let mut result = Vec::with_capacity(vertex_count);
        for vertex in 0..vertex_count {
            let base = vertex * component_count;
            result.push([
                values.get(base).copied().unwrap_or(0.0) * scale + bias_x,
                values.get(base + 1).copied().unwrap_or(0.0) * scale + bias_y,
                values.get(base + 2).copied().unwrap_or(0.0) * scale + bias_z,
            ]);
        }
        let result = Arc::new(result);
        if key != 0 {
            let mut cache = M3G_VERTEX_VEC3_CACHE.lock();
            push_cache_entry(
                &mut cache,
                VertexVec3CacheEntry {
                    key,
                    version,
                    scale_bits,
                    bias_bits,
                    values: result.clone(),
                },
            );
        }
        Ok(result)
    }

    pub(crate) async fn vertex_array_vec2(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>, scale: f32, bias: &[f32]) -> Result<Arc<Vec<[f32; 2]>>> {
        let key = instance_key(array);
        let version = if array.is_null() {
            0
        } else {
            jvm.get_field::<i32>(array, "version", "I").await.unwrap_or(0)
        };
        let bias_x = bias.first().copied().unwrap_or(0.0);
        let bias_y = bias.get(1).copied().unwrap_or(0.0);
        let scale_bits = scale.to_bits();
        let bias_bits = [bias_x.to_bits(), bias_y.to_bits()];
        if key != 0 {
            let cache = M3G_VERTEX_VEC2_CACHE.lock();
            if let Some(entry) = cache
                .iter()
                .find(|entry| entry.key == key && entry.version == version && entry.scale_bits == scale_bits && entry.bias_bits == bias_bits)
            {
                return Ok(entry.values.clone());
            }
        }
        let (component_count, vertex_count, values) = Self::vertex_array_values(jvm, array).await?;
        let mut result = Vec::with_capacity(vertex_count);
        for vertex in 0..vertex_count {
            let base = vertex * component_count;
            result.push([
                values.get(base).copied().unwrap_or(0.0) * scale + bias_x,
                values.get(base + 1).copied().unwrap_or(0.0) * scale + bias_y,
            ]);
        }
        let result = Arc::new(result);
        if key != 0 {
            let mut cache = M3G_VERTEX_VEC2_CACHE.lock();
            push_cache_entry(
                &mut cache,
                VertexVec2CacheEntry {
                    key,
                    version,
                    scale_bits,
                    bias_bits,
                    values: result.clone(),
                },
            );
        }
        Ok(result)
    }

    pub(crate) async fn vertex_array_colors(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>, default_color: i32) -> Result<Arc<Vec<i32>>> {
        let key = instance_key(array);
        let version = if array.is_null() {
            0
        } else {
            jvm.get_field::<i32>(array, "version", "I").await.unwrap_or(0)
        };
        if key != 0 {
            let cache = M3G_VERTEX_COLOR_CACHE.lock();
            if let Some(entry) = cache
                .iter()
                .find(|entry| entry.key == key && entry.version == version && entry.default_color == default_color)
            {
                return Ok(entry.values.clone());
            }
        }
        let (component_count, vertex_count, values) = Self::vertex_array_values(jvm, array).await?;
        if component_count < 3 {
            let result = Arc::new(vec![default_color; vertex_count]);
            if key != 0 {
                let mut cache = M3G_VERTEX_COLOR_CACHE.lock();
                push_cache_entry(
                    &mut cache,
                    VertexColorCacheEntry {
                        key,
                        version,
                        default_color,
                        values: result.clone(),
                    },
                );
            }
            return Ok(result);
        }
        let mut result = Vec::with_capacity(vertex_count);
        for vertex in 0..vertex_count {
            let base = vertex * component_count;
            let r = clamp_color(values.get(base).copied().unwrap_or(0.0));
            let g = clamp_color(values.get(base + 1).copied().unwrap_or(0.0));
            let b = clamp_color(values.get(base + 2).copied().unwrap_or(0.0));
            let a = values.get(base + 3).map(|v| clamp_color(*v)).unwrap_or(255);
            result.push(((a << 24) | (r << 16) | (g << 8) | b) as i32);
        }
        let result = Arc::new(result);
        if key != 0 {
            let mut cache = M3G_VERTEX_COLOR_CACHE.lock();
            push_cache_entry(
                &mut cache,
                VertexColorCacheEntry {
                    key,
                    version,
                    default_color,
                    values: result.clone(),
                },
            );
        }
        Ok(result)
    }

    pub(crate) async fn vertex_array_components(jvm: &Jvm, array: &ClassInstanceRef<VertexArray>) -> Result<Vec<Vec<f32>>> {
        let (component_count, vertex_count, values) = Self::vertex_array_values(jvm, array).await?;
        if component_count == 0 {
            return Ok(Vec::new());
        }
        let mut result = Vec::with_capacity(vertex_count);
        for chunk in values.chunks(component_count) {
            result.push(chunk.to_vec());
        }
        Ok(result)
    }

    pub(crate) async fn triangle_indices(jvm: &Jvm, array: ClassInstanceRef<TriangleStripArray>) -> Result<Arc<Vec<[usize; 3]>>> {
        if array.is_null() {
            return Ok(Arc::new(Vec::new()));
        }
        let key = instance_key(&array);
        if key != 0 {
            let cache = M3G_TRIANGLE_INDEX_CACHE.lock();
            if let Some(entry) = cache.iter().find(|entry| entry.key == key) {
                return Ok(entry.values.clone());
            }
        }
        let triangle_cache: ClassInstanceRef<Array<i32>> = jvm.get_field(&array, "triangleCache", "[I").await.unwrap_or_else(|_| null_ref());
        if !triangle_cache.is_null() {
            let values = raw_i32_array(jvm, &triangle_cache, jvm.array_length(&triangle_cache).await?).await?;
            let triangles: Arc<Vec<[usize; 3]>> = Arc::new(
                values
                    .chunks_exact(3)
                    .map(|chunk| [chunk[0].max(0) as usize, chunk[1].max(0) as usize, chunk[2].max(0) as usize])
                    .collect(),
            );
            if key != 0 {
                let mut cache = M3G_TRIANGLE_INDEX_CACHE.lock();
                push_cache_entry(
                    &mut cache,
                    TriangleIndexCacheEntry {
                        key,
                        values: triangles.clone(),
                    },
                );
            }
            return Ok(triangles);
        }

        let indices_array: ClassInstanceRef<Array<i32>> = jvm.get_field(&array, "indices", "[I").await?;
        let lengths_array: ClassInstanceRef<Array<i32>> = jvm.get_field(&array, "stripLengths", "[I").await?;
        if indices_array.is_null() || lengths_array.is_null() {
            return Ok(Arc::new(Vec::new()));
        }
        let indices = raw_i32_array(jvm, &indices_array, jvm.array_length(&indices_array).await?).await?;
        let lengths = raw_i32_array(jvm, &lengths_array, jvm.array_length(&lengths_array).await?).await?;
        let triangles = expand_triangle_strips(&indices, &lengths);
        let mut flat = Vec::with_capacity(triangles.len() * 3);
        for triangle in &triangles {
            flat.push(triangle[0] as i32);
            flat.push(triangle[1] as i32);
            flat.push(triangle[2] as i32);
        }
        let mut cache = jvm.instantiate_array("I", flat.len()).await?;
        store_raw_i32_array(jvm, &mut cache, &flat).await?;
        let mut array = array.clone();
        jvm.put_field(&mut array, "triangleCache", "[I", cache).await?;
        let triangles = Arc::new(triangles);
        if key != 0 {
            let mut cache = M3G_TRIANGLE_INDEX_CACHE.lock();
            push_cache_entry(
                &mut cache,
                TriangleIndexCacheEntry {
                    key,
                    values: triangles.clone(),
                },
            );
        }
        Ok(triangles)
    }

    pub(crate) async fn image_pixels_cached(jvm: &Jvm, image: &ClassInstanceRef<Image>) -> Result<(i32, i32, Arc<Vec<i32>>)> {
        let (width, height, pixels) = Image::pixels(jvm, image).await?;
        let key = instance_key(&pixels);
        if key != 0 {
            let cache = M3G_IMAGE_PIXEL_CACHE.lock();
            if let Some(entry) = cache
                .iter()
                .find(|entry| entry.key == key && entry.width == width && entry.height == height)
            {
                return Ok((width, height, entry.values.clone()));
            }
        }
        let values = Arc::new(raw_i32_array(jvm, &pixels, (width * height) as usize).await?);
        if key != 0 {
            let mut cache = M3G_IMAGE_PIXEL_CACHE.lock();
            push_cache_entry(
                &mut cache,
                ImagePixelCacheEntry {
                    key,
                    width,
                    height,
                    values: values.clone(),
                },
            );
        }
        Ok((width, height, values))
    }

    pub(crate) async fn appearance_state(
        jvm: &Jvm,
        appearance: &ClassInstanceRef<Appearance>,
        default_color: i32,
        scene_lighting: &M3gSceneLighting,
    ) -> Result<M3gAppearanceState> {
        let mut state = default_appearance_state(default_color);
        if appearance.is_null() {
            return Ok(state);
        }
        state.layer = jvm.get_field(appearance, "layer", "I").await.unwrap_or(0);

        let material: ClassInstanceRef<Material> = jvm
            .get_field(appearance, "material", "Ljavax/microedition/m3g/Material;")
            .await
            .unwrap_or_else(|_| null_ref());
        let material_colors = if material.is_null() {
            None
        } else {
            let ambient = jvm.get_field(&material, "ambientColor", "I").await.unwrap_or(0x0033_3333);
            let diffuse = jvm.get_field(&material, "diffuseColor", "I").await.unwrap_or(default_color);
            let emissive = jvm.get_field(&material, "emissiveColor", "I").await.unwrap_or(0);
            let specular = jvm.get_field(&material, "specularColor", "I").await.unwrap_or(0);
            let shininess = jvm.get_field::<f32>(&material, "shininess", "F").await.unwrap_or(0.0);
            let vertex_color_tracking = jvm.get_field::<bool>(&material, "vertexColorTracking", "Z").await.unwrap_or(false);
            state.material = Some(M3gMaterialState {
                ambient,
                diffuse,
                emissive,
                specular,
                shininess,
                vertex_color_tracking,
            });
            Some((ambient, diffuse, emissive))
        };

        let compositing_mode: ClassInstanceRef<CompositingMode> = jvm
            .get_field(appearance, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;")
            .await
            .unwrap_or_else(|_| null_ref());
        if !compositing_mode.is_null() {
            state.color_write = jvm.get_field(&compositing_mode, "colorWrite", "Z").await.unwrap_or(true);
            state.alpha_write = jvm.get_field(&compositing_mode, "alphaWrite", "Z").await.unwrap_or(true);
            state.depth_test = jvm.get_field(&compositing_mode, "depthTest", "Z").await.unwrap_or(true);
            state.depth_write = jvm.get_field(&compositing_mode, "depthWrite", "Z").await.unwrap_or(true);
            state.depth_offset_factor = jvm.get_field::<f32>(&compositing_mode, "depthOffsetFactor", "F").await.unwrap_or(0.0);
            state.depth_offset_units = jvm.get_field::<f32>(&compositing_mode, "depthOffsetUnits", "F").await.unwrap_or(0.0);
            state.alpha_threshold = jvm.get_field::<f32>(&compositing_mode, "alphaThreshold", "F").await.unwrap_or(0.0);
            state.blending = jvm
                .get_field(&compositing_mode, "blending", "I")
                .await
                .unwrap_or(CompositingMode::REPLACE);
        }

        let polygon_mode: ClassInstanceRef<PolygonMode> = jvm
            .get_field(appearance, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;")
            .await
            .unwrap_or_else(|_| null_ref());
        if !polygon_mode.is_null() {
            state.culling = jvm.get_field(&polygon_mode, "culling", "I").await.unwrap_or(PolygonMode::CULL_BACK);
            state.winding = jvm.get_field(&polygon_mode, "winding", "I").await.unwrap_or(PolygonMode::WINDING_CCW);
            state.perspective_correction = jvm.get_field(&polygon_mode, "perspectiveCorrection", "Z").await.unwrap_or(true);
            state.two_sided_lighting = jvm.get_field(&polygon_mode, "twoSidedLighting", "Z").await.unwrap_or(false);
            state.local_camera_lighting = jvm.get_field(&polygon_mode, "localCameraLighting", "Z").await.unwrap_or(false);
        }

        state.texture = Self::appearance_texture(jvm, appearance, "texture0").await?;
        state.texture1 = Self::appearance_texture(jvm, appearance, "texture1").await?;
        state.base_color = material_base_color(material_colors, default_color, scene_lighting);
        Self::bind_fog(jvm, appearance, state).await
    }

    pub(crate) async fn appearance_texture(jvm: &Jvm, appearance: &ClassInstanceRef<Appearance>, field: &str) -> Result<Option<M3gTexture>> {
        let texture: ClassInstanceRef<Texture2D> = jvm
            .get_field(appearance, field, "Ljavax/microedition/m3g/Texture2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        if texture.is_null() {
            return Ok(None);
        }
        let image2d: ClassInstanceRef<Image2D> = jvm
            .get_field(&texture, "image", "Ljavax/microedition/m3g/Image2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image2d.is_null() {
            return Ok(None);
        }
        let image: ClassInstanceRef<Image> = jvm
            .get_field(&image2d, "image", "Ljavax/microedition/lcdui/Image;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image.is_null() {
            return Ok(None);
        }
        let format = jvm.get_field(&image2d, "format", "I").await.unwrap_or(Image2D::RGBA);
        let (width, height, pixels) = Self::image_pixels_cached(jvm, &image).await?;
        let blend_color = jvm.get_field(&texture, "blendColor", "I").await.unwrap_or(0);
        let blending = jvm.get_field(&texture, "blending", "I").await.unwrap_or(Texture2D::FUNC_MODULATE);
        let wrap_s = jvm.get_field(&texture, "wrappingS", "I").await.unwrap_or(Texture2D::WRAP_REPEAT);
        let wrap_t = jvm.get_field(&texture, "wrappingT", "I").await.unwrap_or(Texture2D::WRAP_REPEAT);
        let image_filter = jvm.get_field(&texture, "imageFilter", "I").await.unwrap_or(Texture2D::FILTER_NEAREST);
        let transform = Transformable::local_matrix(jvm, &cast_ref(&texture))
            .await
            .unwrap_or_else(|_| identity_matrix());
        Ok(Some(M3gTexture {
            width,
            height,
            pixels,
            format,
            blend_color,
            blending,
            wrap_s,
            wrap_t,
            image_filter,
            transform,
            transform_identity: texture_transform_is_identity(transform),
        }))
    }

    pub(crate) async fn bind_fog(jvm: &Jvm, appearance: &ClassInstanceRef<Appearance>, mut state: M3gAppearanceState) -> Result<M3gAppearanceState> {
        let fog: ClassInstanceRef<Fog> = jvm
            .get_field(appearance, "fog", "Ljavax/microedition/m3g/Fog;")
            .await
            .unwrap_or_else(|_| null_ref());
        if fog.is_null() {
            return Ok(state);
        }
        state.fog = Some(M3gFogState {
            mode: jvm.get_field::<i32>(&fog, "mode", "I").await.unwrap_or(Fog::LINEAR),
            color: jvm.get_field::<i32>(&fog, "color", "I").await.unwrap_or(0),
            density: jvm.get_field::<f32>(&fog, "density", "F").await.unwrap_or(1.0).max(0.0),
            near: jvm.get_field::<f32>(&fog, "near", "F").await.unwrap_or(0.0),
            far: jvm.get_field::<f32>(&fog, "far", "F").await.unwrap_or(1.0),
        });
        Ok(state)
    }

    pub(crate) async fn background_frame(jvm: &Jvm, this: &ClassInstanceRef<Self>, world: &ClassInstanceRef<World>) -> Result<M3gBackgroundFrame> {
        let viewport_w: i32 = jvm.get_field(this, "viewportW", "I").await?;
        let viewport_h: i32 = jvm.get_field(this, "viewportH", "I").await?;
        let pixel_count = (viewport_w.max(0) * viewport_h.max(0)) as usize;
        let background: ClassInstanceRef<Background> = jvm
            .get_field(world, "background", "Ljavax/microedition/m3g/Background;")
            .await
            .unwrap_or_else(|_| null_ref());
        if background.is_null() {
            return Ok(M3gBackgroundFrame {
                pixels: vec![0; pixel_count],
                color_clear: false,
                depth_clear: false,
                process_alpha: true,
                submit_when_empty: false,
            });
        }

        let color_clear: bool = jvm.get_field(&background, "colorClear", "Z").await.unwrap_or(true);
        let depth_clear: bool = jvm.get_field(&background, "depthClear", "Z").await.unwrap_or(true);
        if !color_clear {
            return Ok(M3gBackgroundFrame {
                pixels: vec![0; pixel_count],
                color_clear: false,
                depth_clear,
                process_alpha: true,
                submit_when_empty: false,
            });
        }

        let color = ensure_opaque(jvm.get_field(&background, "color", "I").await.unwrap_or(0));
        let mut pixels = vec![color; pixel_count];
        Self::paint_background_image(jvm, &background, viewport_w, viewport_h, color, &mut pixels).await?;
        Ok(M3gBackgroundFrame {
            pixels,
            color_clear: true,
            depth_clear,
            process_alpha: false,
            submit_when_empty: true,
        })
    }

    pub(crate) async fn paint_background_image(
        jvm: &Jvm,
        background: &ClassInstanceRef<Background>,
        viewport_w: i32,
        viewport_h: i32,
        border_color: i32,
        target: &mut [i32],
    ) -> Result<()> {
        if viewport_w <= 0 || viewport_h <= 0 {
            return Ok(());
        }
        let image2d: ClassInstanceRef<Image2D> = jvm
            .get_field(background, "image", "Ljavax/microedition/m3g/Image2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image2d.is_null() {
            return Ok(());
        }
        let image: ClassInstanceRef<Image> = jvm
            .get_field(&image2d, "image", "Ljavax/microedition/lcdui/Image;")
            .await
            .unwrap_or_else(|_| null_ref());
        if image.is_null() {
            return Ok(());
        }

        let (image_w, image_h, image_pixels) = Self::image_pixels_cached(jvm, &image).await?;
        if image_w <= 0 || image_h <= 0 {
            return Ok(());
        }
        let crop_x: i32 = jvm.get_field(background, "cropX", "I").await.unwrap_or(0);
        let crop_y: i32 = jvm.get_field(background, "cropY", "I").await.unwrap_or(0);
        let crop_w: i32 = jvm.get_field(background, "cropW", "I").await.unwrap_or(image_w);
        let crop_h: i32 = jvm.get_field(background, "cropH", "I").await.unwrap_or(image_h);
        if crop_w <= 0 || crop_h <= 0 {
            return Ok(());
        }
        let mode_x: i32 = jvm.get_field(background, "imageModeX", "I").await.unwrap_or(Background::BORDER);
        let mode_y: i32 = jvm.get_field(background, "imageModeY", "I").await.unwrap_or(Background::BORDER);

        for y in 0..viewport_h {
            let src_y = crop_y + (y as i64 * crop_h as i64 / viewport_h as i64) as i32;
            let Some(sample_y) = background_coord(src_y, image_h, mode_y) else {
                continue;
            };
            for x in 0..viewport_w {
                let src_x = crop_x + (x as i64 * crop_w as i64 / viewport_w as i64) as i32;
                let Some(sample_x) = background_coord(src_x, image_w, mode_x) else {
                    continue;
                };
                let dst = (y * viewport_w + x) as usize;
                let src = image_pixels[(sample_y * image_w + sample_x) as usize];
                target[dst] = source_over(border_color, src);
            }
        }
        Ok(())
    }
}
