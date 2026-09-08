use alloc::vec::Vec;
use bytemuck::cast_slice;
use wgpu::util::DeviceExt;
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource,
    BindingType, BlendComponent, BlendFactor, BlendOperation, BlendState, Buffer, BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoder,
    CompareFunction, DepthBiasState, DepthStencilState, Device, Extent3d, FilterMode, FragmentState, FrontFace, LoadOp, MipmapFilterMode,
    MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, Queue,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, StencilState, StoreOp, TexelCopyBufferLayout,
    TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView,
    TextureViewDimension, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use java_runtime::classes::javax::microedition::m3g::{CompositingMode, M3gGpuFrame, M3gGpuTexture, Texture2D};

use super::M3G_SHADER;

const PIPELINE_COUNT: usize = 16;
const FLOATS_PER_VERTEX: usize = 36;

pub(crate) struct M3gGpuPresenter {
    pipelines: [RenderPipeline; PIPELINE_COUNT],
    bind_group_layout: BindGroupLayout,
    nearest_clamp: Sampler,
    linear_clamp: Sampler,
    nearest_repeat: Sampler,
    linear_repeat: Sampler,
    _white_texture: Texture,
    white_view: TextureView,
    white_bind_group: BindGroup,
    depth: Texture,
    depth_view: TextureView,
    depth_width: u32,
    depth_height: u32,
}

pub(crate) struct M3gGpuDrawGuard {
    _vertex_buffer: Option<Buffer>,
    _textures: Vec<Option<UploadedTexture>>,
    _bind_groups: Vec<BindGroup>,
}

struct UploadedTexture {
    _texture: Texture,
    view: TextureView,
    wrap_s: i32,
    wrap_t: i32,
    filter: i32,
}

struct M3gBatch {
    tex0: Option<usize>,
    tex1: Option<usize>,
    pipeline: usize,
    start: u32,
    count: u32,
}

impl M3gGpuPresenter {
    pub(crate) fn new(device: &Device, queue: &Queue, surface_format: TextureFormat) -> Self {
        let nearest_clamp = sampler(device, AddressMode::ClampToEdge, FilterMode::Nearest);
        let linear_clamp = sampler(device, AddressMode::ClampToEdge, FilterMode::Linear);
        let nearest_repeat = sampler(device, AddressMode::Repeat, FilterMode::Nearest);
        let linear_repeat = sampler(device, AddressMode::Repeat, FilterMode::Linear);
        let bind_group_layout = dual_texture_layout(device);
        let white_texture = device.create_texture(&TextureDescriptor {
            label: Some("RustJava M3G white texture"),
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
        let white_bind_group = bind_group(device, &bind_group_layout, &white_view, &nearest_clamp, &white_view, &nearest_clamp);
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("RustJava M3G pipeline layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("RustJava M3G shader"),
            source: ShaderSource::Wgsl(M3G_SHADER.into()),
        });
        let pipelines = core::array::from_fn(|index| {
            let blend = index / 4;
            let depth_test = (index / 2) % 2 == 1;
            let depth_write = index % 2 == 1;
            create_pipeline(device, surface_format, &pipeline_layout, &shader, blend, depth_test, depth_write)
        });
        let (depth, depth_view) = create_depth_texture(device, 1, 1);

        Self {
            pipelines,
            bind_group_layout,
            nearest_clamp,
            linear_clamp,
            nearest_repeat,
            linear_repeat,
            _white_texture: white_texture,
            white_view,
            white_bind_group,
            depth,
            depth_view,
            depth_width: 1,
            depth_height: 1,
        }
    }

    pub(crate) fn encode(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        view: &TextureView,
        frame: &M3gGpuFrame,
        surface_width: u32,
        surface_height: u32,
        viewport: Option<(f32, f32, f32, f32)>,
        load: LoadOp<Color>,
    ) -> M3gGpuDrawGuard {
        if frame.width <= 0 || frame.height <= 0 || frame.triangles.is_empty() {
            return M3gGpuDrawGuard {
                _vertex_buffer: None,
                _textures: Vec::new(),
                _bind_groups: Vec::new(),
            };
        }
        self.ensure_depth(device, surface_width.max(1), surface_height.max(1));

        let (vertices, batches) = self.build_vertices(frame);
        if vertices.is_empty() {
            return M3gGpuDrawGuard {
                _vertex_buffer: None,
                _textures: Vec::new(),
                _bind_groups: Vec::new(),
            };
        }
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("RustJava M3G vertex buffer"),
            contents: cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        let textures = self.upload_textures(device, queue, frame);
        let mut bind_groups = Vec::with_capacity(batches.len());

        {
            let color_attachment = Some(RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations { load, store: StoreOp::Store },
            });
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("RustJava M3G frame pass"),
                color_attachments: &[color_attachment],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(Operations {
                        load: if frame.clear_depth { LoadOp::Clear(1.0) } else { LoadOp::Load },
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            if let Some((x, y, width, height)) = viewport {
                pass.set_viewport(x, y, width.max(1.0), height.max(1.0), 0.0, 1.0);
            }
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            for batch in &batches {
                pass.set_pipeline(&self.pipelines[batch.pipeline]);
                if batch.tex0.is_none() && batch.tex1.is_none() {
                    pass.set_bind_group(0, &self.white_bind_group, &[]);
                } else {
                    let bind_group = self.bind_group_for(device, &textures, batch.tex0, batch.tex1);
                    pass.set_bind_group(0, &bind_group, &[]);
                    bind_groups.push(bind_group);
                }
                pass.draw(batch.start..batch.start + batch.count, 0..1);
            }
        }

        M3gGpuDrawGuard {
            _vertex_buffer: Some(vertex_buffer),
            _textures: textures,
            _bind_groups: bind_groups,
        }
    }

    fn ensure_depth(&mut self, device: &Device, width: u32, height: u32) {
        if self.depth_width == width && self.depth_height == height {
            return;
        }
        let (depth, depth_view) = create_depth_texture(device, width, height);
        self.depth = depth;
        self.depth_view = depth_view;
        self.depth_width = width;
        self.depth_height = height;
    }

    fn build_vertices(&self, frame: &M3gGpuFrame) -> (Vec<f32>, Vec<M3gBatch>) {
        let mut vertices = Vec::with_capacity(frame.triangles.len() * 3 * FLOATS_PER_VERTEX);
        let mut batches = Vec::new();
        let mut current_tex0 = None;
        let mut current_tex1 = None;
        let mut current_pipeline = 0usize;
        let mut current_start = 0u32;
        let mut current_count = 0u32;
        let width = frame.width.max(1) as f32;
        let height = frame.height.max(1) as f32;

        for tri in &frame.triangles {
            let tex0 = tri.tex0.index.filter(|index| frame.textures.get(*index).is_some());
            let tex1 = tri.tex1.index.filter(|index| frame.textures.get(*index).is_some());
            let pipeline = pipeline_index(tri.blending, tri.depth_test, tri.depth_write);
            if current_count == 0 {
                current_tex0 = tex0;
                current_tex1 = tex1;
                current_pipeline = pipeline;
                current_start = (vertices.len() / FLOATS_PER_VERTEX) as u32;
            } else if current_tex0 != tex0 || current_tex1 != tex1 || current_pipeline != pipeline {
                batches.push(M3gBatch {
                    tex0: current_tex0,
                    tex1: current_tex1,
                    pipeline: current_pipeline,
                    start: current_start,
                    count: current_count,
                });
                current_tex0 = tex0;
                current_tex1 = tex1;
                current_pipeline = pipeline;
                current_start = (vertices.len() / FLOATS_PER_VERTEX) as u32;
                current_count = 0;
            }

            let fog_color = argb_to_rgba_f32(tri.fog.color);
            let blend0 = argb_to_rgba_f32(tri.tex0.blend_color);
            let blend1 = argb_to_rgba_f32(tri.tex1.blend_color);
            for vertex in tri.vertices {
                let [r, g, b, a] = argb_to_rgba_f32(vertex.color);
                let x = (vertex.x / width) * 2.0 - 1.0;
                let y = 1.0 - (vertex.y / height) * 2.0;
                vertices.extend_from_slice(&[
                    x,
                    y,
                    vertex.depth.clamp(0.0, 1.0),
                    1.0,
                    vertex.u,
                    vertex.v,
                    vertex.u1,
                    vertex.v1,
                    r,
                    g,
                    b,
                    a,
                    tri.alpha_threshold,
                    tri.tex0.blending as f32,
                    tri.tex0.format as f32,
                    vertex.camera_z,
                    tri.tex1.blending as f32,
                    tri.tex1.format as f32,
                    tri.fog.mode as f32,
                    tri.fog.density,
                    tri.fog.near,
                    tri.fog.far,
                    0.0,
                    0.0,
                    fog_color[0],
                    fog_color[1],
                    fog_color[2],
                    fog_color[3],
                    blend0[0],
                    blend0[1],
                    blend0[2],
                    blend0[3],
                    blend1[0],
                    blend1[1],
                    blend1[2],
                    blend1[3],
                ]);
            }
            current_count += 3;
        }

        if current_count > 0 {
            batches.push(M3gBatch {
                tex0: current_tex0,
                tex1: current_tex1,
                pipeline: current_pipeline,
                start: current_start,
                count: current_count,
            });
        }
        (vertices, batches)
    }

    fn upload_textures(&self, device: &Device, queue: &Queue, frame: &M3gGpuFrame) -> Vec<Option<UploadedTexture>> {
        frame.textures.iter().map(|texture| self.upload_texture(device, queue, texture)).collect()
    }

    fn upload_texture(&self, device: &Device, queue: &Queue, texture: &M3gGpuTexture) -> Option<UploadedTexture> {
        if texture.width <= 0 || texture.height <= 0 || texture.rgba.is_empty() {
            return None;
        }
        let gpu_texture = device.create_texture(&TextureDescriptor {
            label: Some("RustJava M3G texture"),
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
        Some(UploadedTexture {
            _texture: gpu_texture,
            view,
            wrap_s: texture.wrap_s,
            wrap_t: texture.wrap_t,
            filter: texture.filter,
        })
    }

    fn bind_group_for(&self, device: &Device, textures: &[Option<UploadedTexture>], tex0: Option<usize>, tex1: Option<usize>) -> BindGroup {
        let (view0, sampler0) = self.texture_binding(tex0.and_then(|index| textures.get(index).and_then(|entry| entry.as_ref())));
        let (view1, sampler1) = self.texture_binding(tex1.and_then(|index| textures.get(index).and_then(|entry| entry.as_ref())));
        bind_group(device, &self.bind_group_layout, view0, sampler0, view1, sampler1)
    }

    fn texture_binding<'a>(&'a self, texture: Option<&'a UploadedTexture>) -> (&'a TextureView, &'a Sampler) {
        match texture {
            Some(texture) => (&texture.view, self.sampler_for(texture.wrap_s, texture.wrap_t, texture.filter)),
            None => (&self.white_view, &self.nearest_clamp),
        }
    }

    fn sampler_for(&self, wrap_s: i32, wrap_t: i32, filter: i32) -> &Sampler {
        let repeat = wrap_s == Texture2D::WRAP_REPEAT || wrap_t == Texture2D::WRAP_REPEAT;
        let linear = filter == Texture2D::FILTER_LINEAR;
        match (repeat, linear) {
            (false, false) => &self.nearest_clamp,
            (false, true) => &self.linear_clamp,
            (true, false) => &self.nearest_repeat,
            (true, true) => &self.linear_repeat,
        }
    }
}

fn pipeline_index(blending: i32, depth_test: bool, depth_write: bool) -> usize {
    let blend = match blending {
        CompositingMode::ALPHA => 1,
        CompositingMode::ALPHA_ADD => 2,
        CompositingMode::MODULATE | CompositingMode::MODULATE_X2 => 3,
        _ => 0,
    };
    blend * 4 + usize::from(depth_test) * 2 + usize::from(depth_write)
}

fn create_pipeline(
    device: &Device,
    surface_format: TextureFormat,
    pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    blend: usize,
    depth_test: bool,
    depth_write: bool,
) -> RenderPipeline {
    let vertex_attributes = [
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 0,
            shader_location: 0,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 16,
            shader_location: 1,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 32,
            shader_location: 2,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 48,
            shader_location: 3,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 64,
            shader_location: 4,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 80,
            shader_location: 5,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 96,
            shader_location: 6,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 112,
            shader_location: 7,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 128,
            shader_location: 8,
        },
    ];
    let vertex_layout = VertexBufferLayout {
        array_stride: (FLOATS_PER_VERTEX * 4) as u64,
        step_mode: VertexStepMode::Vertex,
        attributes: &vertex_attributes,
    };
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("RustJava M3G pipeline"),
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
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth24Plus,
            depth_write_enabled: Some(depth_write),
            depth_compare: Some(if depth_test {
                CompareFunction::LessEqual
            } else {
                CompareFunction::Always
            }),
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState::default(),
        fragment: Some(FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            compilation_options: PipelineCompilationOptions::default(),
            targets: &[Some(ColorTargetState {
                format: surface_format,
                blend: blend_state(blend),
                write_mask: ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    })
}

fn blend_state(blend: usize) -> Option<BlendState> {
    match blend {
        1 => Some(BlendState::ALPHA_BLENDING),
        2 => Some(BlendState {
            color: BlendComponent {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            alpha: BlendComponent::REPLACE,
        }),
        3 => Some(BlendState {
            color: BlendComponent {
                src_factor: BlendFactor::Zero,
                dst_factor: BlendFactor::Src,
                operation: BlendOperation::Add,
            },
            alpha: BlendComponent::REPLACE,
        }),
        _ => None,
    }
}

fn dual_texture_layout(device: &Device) -> BindGroupLayout {
    let texture_entry = |binding| BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::FRAGMENT,
        ty: BindingType::Texture {
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    };
    let sampler_entry = |binding| BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::FRAGMENT,
        ty: BindingType::Sampler(SamplerBindingType::Filtering),
        count: None,
    };
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("RustJava M3G bind group layout"),
        entries: &[texture_entry(0), sampler_entry(1), texture_entry(2), sampler_entry(3)],
    })
}

fn sampler(device: &Device, address: AddressMode, filter: FilterMode) -> Sampler {
    device.create_sampler(&SamplerDescriptor {
        label: Some("RustJava M3G sampler"),
        address_mode_u: address,
        address_mode_v: address,
        address_mode_w: address,
        mag_filter: filter,
        min_filter: filter,
        mipmap_filter: MipmapFilterMode::Nearest,
        ..Default::default()
    })
}

fn bind_group(
    device: &Device,
    layout: &BindGroupLayout,
    view0: &TextureView,
    sampler0: &Sampler,
    view1: &TextureView,
    sampler1: &Sampler,
) -> BindGroup {
    device.create_bind_group(&BindGroupDescriptor {
        label: Some("RustJava M3G bind group"),
        layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(view0),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(sampler0),
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::TextureView(view1),
            },
            BindGroupEntry {
                binding: 3,
                resource: BindingResource::Sampler(sampler1),
            },
        ],
    })
}

fn create_depth_texture(device: &Device, width: u32, height: u32) -> (Texture, TextureView) {
    let depth = device.create_texture(&TextureDescriptor {
        label: Some("RustJava M3G depth"),
        size: Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Depth24Plus,
        usage: TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = depth.create_view(&Default::default());
    (depth, view)
}

fn argb_to_rgba_f32(color: i32) -> [f32; 4] {
    let color = color as u32;
    [
        ((color >> 16) & 0xff) as f32 / 255.0,
        ((color >> 8) & 0xff) as f32 / 255.0,
        (color & 0xff) as f32 / 255.0,
        ((color >> 24) & 0xff) as f32 / 255.0,
    ]
}

pub(crate) fn scaled_m3g_viewport(frame: &M3gGpuFrame, scale_x: f32, scale_y: f32) -> (f32, f32, f32, f32) {
    (
        frame.viewport_x as f32 * scale_x,
        frame.viewport_y as f32 * scale_y,
        frame.width.max(1) as f32 * scale_x,
        frame.height.max(1) as f32 * scale_y,
    )
}
