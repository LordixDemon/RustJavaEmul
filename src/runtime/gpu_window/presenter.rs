use std::sync::Arc;

use anyhow::Context;
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    BlendState, Color, ColorTargetState, ColorWrites, CommandEncoder, CommandEncoderDescriptor, CurrentSurfaceTexture, Device, Extent3d, FilterMode,
    FragmentState, FrontFace, LoadOp, MipmapFilterMode, MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PresentMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    SamplerBindingType, SamplerDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp, Surface, SurfaceConfiguration,
    TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType,
    TextureUsages, TextureView, TextureViewDimension, VertexState,
};
use winit::{dpi::PhysicalSize, event_loop::OwnedDisplayHandle, window::Window};

use java_runtime::classes::com::mascotcapsule::micro3d::v3::latest_gpu_frame_after;
use java_runtime::classes::javax::microedition::m3g::latest_m3g_gpu_frame_after;

use crate::gpu::{self, FRAME_SHADER, M3gGpuPresenter, V3GpuPresenter};
use crate::profile;

pub(super) struct WindowState {
    pub(super) window: Arc<Window>,
    pub(super) presenter: GpuPresenter,
}

pub(super) struct GpuPresenter {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    frame_texture: Texture,
    bind_group: BindGroup,
    pipeline: RenderPipeline,
    alpha_pipeline: RenderPipeline,
    v3: V3GpuPresenter,
    m3g: M3gGpuPresenter,
    texture_width: u32,
    texture_height: u32,
    pub(super) info: String,
}

impl GpuPresenter {
    pub(super) async fn new(window: Arc<Window>, display: OwnedDisplayHandle, texture_width: u32, texture_height: u32) -> anyhow::Result<Self> {
        let instance = gpu::create_instance(gpu::instance_descriptor_with_display(display)).await;
        let surface = instance.create_surface(window.clone())?;
        let adapter = gpu::request_preferred_adapter(&instance, &surface).await?;
        let info = gpu::adapter_info_string(&adapter);
        tracing::info!("{info}");

        let (device, queue) = gpu::request_device(&adapter, "RustJava GPU device").await?;

        let size = nonzero_size(window.inner_size());
        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .context("GPU surface is not supported by the selected adapter")?;
        config.present_mode = PresentMode::AutoVsync;
        config.desired_maximum_frame_latency = 1;
        surface.configure(&device, &config);

        let (frame_texture, bind_group, pipeline, alpha_pipeline) = create_frame_pipeline(&device, config.format, texture_width, texture_height);
        let v3 = V3GpuPresenter::new(&device, &queue, config.format);
        let m3g = M3gGpuPresenter::new(&device, &queue, config.format);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            frame_texture,
            bind_group,
            pipeline,
            alpha_pipeline,
            v3,
            m3g,
            texture_width,
            texture_height,
            info,
        })
    }

    pub(super) fn resize(&mut self, size: PhysicalSize<u32>) {
        let size = nonzero_size(size);
        if self.config.width == size.width && self.config.height == size.height {
            return;
        }

        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub(super) fn render(&mut self, pixels: Option<&[u32]>, game_width: u32, game_height: u32) -> anyhow::Result<()> {
        if let Some(pixels) = pixels {
            self.upload_frame(pixels);
        }

        let output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(output) | CurrentSurfaceTexture::Suboptimal(output) => output,
            CurrentSurfaceTexture::Lost | CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return Ok(()),
            CurrentSurfaceTexture::Validation => anyhow::bail!("GPU surface validation failed"),
        };

        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("RustJava frame encoder"),
        });
        let scale_x = self.config.width as f32 / self.texture_width.max(1) as f32;
        let scale_y = self.config.height as f32 / self.texture_height.max(1) as f32;
        let m3g_frame = latest_m3g_gpu_frame_after(0);
        let hud = m3g_frame.as_ref().is_some_and(|frame| frame.color_clear);
        let m3g_guard = if hud {
            m3g_frame.map(|frame| {
                self.m3g.encode(
                    &self.device,
                    &self.queue,
                    &mut encoder,
                    &view,
                    &frame,
                    self.config.width,
                    self.config.height,
                    Some(gpu::scaled_m3g_viewport(&frame, scale_x, scale_y)),
                    LoadOp::Clear(Color::BLACK),
                )
            })
        } else {
            self.blit_frame(&mut encoder, &view, LoadOp::Clear(Color::BLACK), false);
            m3g_frame.map(|frame| {
                self.m3g.encode(
                    &self.device,
                    &self.queue,
                    &mut encoder,
                    &view,
                    &frame,
                    self.config.width,
                    self.config.height,
                    Some(gpu::scaled_m3g_viewport(&frame, scale_x, scale_y)),
                    LoadOp::Load,
                )
            })
        };
        if hud {
            self.blit_frame(&mut encoder, &view, LoadOp::Load, true);
        }
        let v3_guard = latest_gpu_frame_after(0).map(|frame| {
            self.v3.encode(
                &self.device,
                &self.queue,
                &mut encoder,
                &view,
                &frame,
                Some((0.0, 0.0, game_width as f32 * scale_x, game_height as f32 * scale_y)),
                LoadOp::Load,
            )
        });

        self.queue.submit([encoder.finish()]);
        drop((m3g_guard, v3_guard));
        output.present();
        Ok(())
    }

    fn blit_frame(&self, encoder: &mut CommandEncoder, view: &TextureView, load: LoadOp<Color>, alpha: bool) {
        let pipeline = if alpha { &self.alpha_pipeline } else { &self.pipeline };
        let color_attachment = Some(RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: None,
            ops: Operations { load, store: StoreOp::Store },
        });
        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("RustJava frame pass"),
            color_attachments: &[color_attachment],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..6, 0..1);
    }

    fn upload_frame(&self, pixels: &[u32]) {
        let _upload_timer = profile::timer(&profile::WINDOW_FRAME_UPLOAD);
        self.queue.write_texture(
            TexelCopyTextureInfo {
                texture: &self.frame_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            bytemuck::cast_slice(pixels),
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(self.texture_width * 4),
                rows_per_image: Some(self.texture_height),
            },
            Extent3d {
                width: self.texture_width,
                height: self.texture_height,
                depth_or_array_layers: 1,
            },
        );
    }
}

fn create_frame_pipeline(
    device: &Device,
    surface_format: TextureFormat,
    texture_width: u32,
    texture_height: u32,
) -> (Texture, BindGroup, RenderPipeline, RenderPipeline) {
    let frame_texture = device.create_texture(&TextureDescriptor {
        label: Some("RustJava frame texture"),
        size: Extent3d {
            width: texture_width,
            height: texture_height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Bgra8UnormSrgb,
        usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let frame_view = frame_texture.create_view(&Default::default());
    let sampler = device.create_sampler(&SamplerDescriptor {
        label: Some("RustJava frame sampler"),
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        mipmap_filter: MipmapFilterMode::Nearest,
        ..Default::default()
    });

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("RustJava frame bind group layout"),
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
    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("RustJava frame bind group"),
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&frame_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(&sampler),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("RustJava frame pipeline layout"),
        bind_group_layouts: &[Some(&bind_group_layout)],
        immediate_size: 0,
    });
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("RustJava frame shader"),
        source: ShaderSource::Wgsl(FRAME_SHADER.into()),
    });
    let make_pipeline = |label, blend| {
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
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
                module: &shader,
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
    };
    let pipeline = make_pipeline("RustJava frame pipeline", None);
    let alpha_pipeline = make_pipeline("RustJava frame alpha pipeline", Some(BlendState::ALPHA_BLENDING));

    (frame_texture, bind_group, pipeline, alpha_pipeline)
}

fn nonzero_size(size: PhysicalSize<u32>) -> PhysicalSize<u32> {
    PhysicalSize::new(size.width.max(1), size.height.max(1))
}
