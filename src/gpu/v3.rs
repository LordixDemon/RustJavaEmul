use alloc::vec::Vec;
use bytemuck::cast_slice;
use wgpu::util::DeviceExt;
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource,
    BindingType, BlendComponent, BlendFactor, BlendOperation, BlendState, Buffer, BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoder,
    Device, Extent3d, FilterMode, FragmentState, FrontFace, LoadOp, MipmapFilterMode, MultisampleState, Operations, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp,
    TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType,
    TextureUsages, TextureView, TextureViewDimension, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use java_runtime::classes::com::mascotcapsule::micro3d::v3::V3GpuFrame;

use super::V3_SHADER;

pub(crate) struct V3GpuPresenter {
    normal_pipeline: RenderPipeline,
    additive_pipeline: RenderPipeline,
    subtractive_pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    sampler: Sampler,
    _white_texture: Texture,
    white_bind_group: BindGroup,
}

pub(crate) struct V3GpuDrawGuard {
    _vertex_buffer: Option<Buffer>,
    _textures: Vec<Option<(Texture, BindGroup)>>,
}

struct V3Batch {
    texture: Option<usize>,
    blend_mode: u8,
    start: u32,
    count: u32,
}

impl V3GpuPresenter {
    pub(crate) fn new(device: &Device, queue: &Queue, surface_format: TextureFormat) -> Self {
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("RustJava V3 sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::Nearest,
            ..Default::default()
        });
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("RustJava V3 bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let white_texture = device.create_texture(&TextureDescriptor {
            label: Some("RustJava V3 white texture"),
            size: Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &white_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &[0xff, 0xff, 0xff, 0xff],
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        let white_view = white_texture.create_view(&Default::default());
        let white_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("RustJava V3 white bind group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&white_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("RustJava V3 pipeline layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("RustJava V3 shader"),
            source: ShaderSource::Wgsl(V3_SHADER.into()),
        });
        let normal_pipeline = Self::create_pipeline(
            device,
            surface_format,
            &pipeline_layout,
            &shader,
            "RustJava V3 normal pipeline",
            Some(BlendState::ALPHA_BLENDING),
        );
        let additive_pipeline = Self::create_pipeline(
            device,
            surface_format,
            &pipeline_layout,
            &shader,
            "RustJava V3 additive pipeline",
            Some(BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent::REPLACE,
            }),
        );
        let subtractive_pipeline = Self::create_pipeline(
            device,
            surface_format,
            &pipeline_layout,
            &shader,
            "RustJava V3 subtractive pipeline",
            Some(BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::ReverseSubtract,
                },
                alpha: BlendComponent::REPLACE,
            }),
        );

        Self {
            normal_pipeline,
            additive_pipeline,
            subtractive_pipeline,
            bind_group_layout,
            sampler,
            _white_texture: white_texture,
            white_bind_group,
        }
    }

    fn create_pipeline(
        device: &Device,
        surface_format: TextureFormat,
        pipeline_layout: &wgpu::PipelineLayout,
        shader: &wgpu::ShaderModule,
        label: &'static str,
        blend: Option<BlendState>,
    ) -> RenderPipeline {
        let vertex_attributes = [
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 8,
                shader_location: 1,
            },
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 16,
                shader_location: 2,
            },
        ];
        let vertex_layout = VertexBufferLayout {
            array_stride: 32,
            step_mode: VertexStepMode::Vertex,
            attributes: &vertex_attributes,
        };
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(pipeline_layout),
            vertex: VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[vertex_layout],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: surface_format,
                    blend,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        })
    }

    pub(crate) fn encode(
        &self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        frame: &V3GpuFrame,
        viewport: Option<(f32, f32, f32, f32)>,
        load: LoadOp<Color>,
    ) -> V3GpuDrawGuard {
        if frame.width <= 0 || frame.height <= 0 || frame.triangles.is_empty() {
            return V3GpuDrawGuard {
                _vertex_buffer: None,
                _textures: Vec::new(),
            };
        }

        let (vertices, batches) = self.build_vertices(frame);
        if vertices.is_empty() {
            return V3GpuDrawGuard {
                _vertex_buffer: None,
                _textures: Vec::new(),
            };
        }
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("RustJava V3 vertex buffer"),
            contents: cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        let texture_bind_groups = self.create_texture_bind_groups(device, queue, frame);

        {
            let color_attachment = Some(RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations { load, store: StoreOp::Store },
            });
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("RustJava V3 frame pass"),
                color_attachments: &[color_attachment],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            if let Some((x, y, width, height)) = viewport {
                pass.set_viewport(x, y, width.max(1.0), height.max(1.0), 0.0, 1.0);
            }
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            for batch in batches {
                pass.set_pipeline(self.pipeline_for_blend_mode(batch.blend_mode));
                let bind_group = batch
                    .texture
                    .and_then(|index| texture_bind_groups.get(index))
                    .and_then(|entry| entry.as_ref())
                    .map(|(_, bind_group)| bind_group)
                    .unwrap_or(&self.white_bind_group);
                pass.set_bind_group(0, bind_group, &[]);
                pass.draw(batch.start..batch.start + batch.count, 0..1);
            }
        }

        V3GpuDrawGuard {
            _vertex_buffer: Some(vertex_buffer),
            _textures: texture_bind_groups,
        }
    }

    fn pipeline_for_blend_mode(&self, blend_mode: u8) -> &RenderPipeline {
        match blend_mode {
            2 => &self.additive_pipeline,
            3 => &self.subtractive_pipeline,
            _ => &self.normal_pipeline,
        }
    }

    fn build_vertices(&self, frame: &V3GpuFrame) -> (Vec<f32>, Vec<V3Batch>) {
        let mut vertices = Vec::with_capacity(frame.triangles.len() * 3 * 8);
        let mut batches = Vec::new();
        let mut current_texture = None;
        let mut current_blend_mode = 0u8;
        let mut current_start = 0u32;
        let mut current_count = 0u32;

        for tri in &frame.triangles {
            let texture = tri
                .texture
                .filter(|index| frame.textures.get(*index).is_some_and(|texture| texture.is_some()));
            let blend_mode = tri.blend_mode.min(3);
            if current_count == 0 {
                current_texture = texture;
                current_blend_mode = blend_mode;
                current_start = (vertices.len() / 8) as u32;
            } else if current_texture != texture || current_blend_mode != blend_mode {
                batches.push(V3Batch {
                    texture: current_texture,
                    blend_mode: current_blend_mode,
                    start: current_start,
                    count: current_count,
                });
                current_texture = texture;
                current_blend_mode = blend_mode;
                current_start = (vertices.len() / 8) as u32;
                current_count = 0;
            }

            let textured = texture.is_some();
            let color = if textured { 0x00ff_ffff } else { tri.color };
            let [r, g, b, mut a] = argb_to_rgba_f32(color);
            if blend_mode == 1 {
                a = 128.0 / 255.0;
            }
            let (tex_w, tex_h) = texture
                .and_then(|idx| frame.textures.get(idx).and_then(|t| t.as_ref()))
                .map(|t| (t.width.max(1) as f32, t.height.max(1) as f32))
                .unwrap_or((256.0, 256.0));
            for vertex in tri.vertices {
                let x = (vertex.x as f32 / frame.width as f32) * 2.0 - 1.0;
                let y = 1.0 - (vertex.y as f32 / frame.height as f32) * 2.0;
                vertices.extend_from_slice(&[x, y, vertex.u as f32 / tex_w, vertex.v as f32 / tex_h, r, g, b, a]);
            }
            current_count += 3;
        }

        if current_count > 0 {
            batches.push(V3Batch {
                texture: current_texture,
                blend_mode: current_blend_mode,
                start: current_start,
                count: current_count,
            });
        }

        (vertices, batches)
    }

    fn create_texture_bind_groups(&self, device: &Device, queue: &Queue, frame: &V3GpuFrame) -> Vec<Option<(Texture, BindGroup)>> {
        frame
            .textures
            .iter()
            .map(|texture| self.create_texture_bind_group(device, queue, texture))
            .collect()
    }

    fn create_texture_bind_group(
        &self,
        device: &Device,
        queue: &Queue,
        texture: &Option<java_runtime::classes::com::mascotcapsule::micro3d::v3::V3GpuTexture>,
    ) -> Option<(Texture, BindGroup)> {
        let texture = texture.as_ref()?;
        if texture.width <= 0 || texture.height <= 0 || texture.rgba.is_empty() {
            return None;
        }

        let gpu_texture = device.create_texture(&TextureDescriptor {
            label: Some("RustJava V3 texture"),
            size: Extent3d {
                width: texture.width as u32,
                height: texture.height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &gpu_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &texture.rgba,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(texture.width as u32 * 4),
                rows_per_image: Some(texture.height as u32),
            },
            Extent3d {
                width: texture.width as u32,
                height: texture.height as u32,
                depth_or_array_layers: 1,
            },
        );
        let view = gpu_texture.create_view(&Default::default());
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("RustJava V3 texture bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        Some((gpu_texture, bind_group))
    }
}

fn argb_to_rgba_f32(color: i32) -> [f32; 4] {
    let color = color as u32;
    let alpha = ((color >> 24) & 0xff).max(0xff);
    [
        ((color >> 16) & 0xff) as f32 / 255.0,
        ((color >> 8) & 0xff) as f32 / 255.0,
        (color & 0xff) as f32 / 255.0,
        alpha as f32 / 255.0,
    ]
}
