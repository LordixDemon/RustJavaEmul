use alloc::vec::Vec;
use anyhow::Context;
use bytemuck::cast_slice;
use web_sys::HtmlCanvasElement;
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    BlendState, Color, ColorTargetState, ColorWrites, CommandEncoder, CommandEncoderDescriptor, CurrentSurfaceTexture, Device, Extent3d, FilterMode,
    FragmentState, FrontFace, LoadOp, MipmapFilterMode, MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PresentMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    SamplerBindingType, SamplerDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp, Surface, SurfaceConfiguration, SurfaceTarget,
    TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType,
    TextureUsages, TextureView, TextureViewDimension, VertexState,
};

use java_runtime::classes::com::mascotcapsule::micro3d::v3::latest_gpu_frame_after;
use java_runtime::classes::javax::microedition::m3g::latest_m3g_gpu_frame_after;

use super::browser_log;
use crate::gpu::{self, FRAME_SHADER, M3gGpuDrawGuard, M3gGpuPresenter, V3GpuDrawGuard, V3GpuPresenter};

pub(super) struct BrowserGpuPresenter {
    canvas: HtmlCanvasElement,
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
    last_v3_generation: u64,
    last_m3g_generation: u64,
    texture_width: u32,
    texture_height: u32,
    rgba_words: Vec<u32>,
    pub(super) info: String,
}

struct OverlayGuard {
    _m3g: Option<M3gGpuDrawGuard>,
    _v3: Option<V3GpuDrawGuard>,
}

impl BrowserGpuPresenter {
    pub(super) async fn new(canvas: HtmlCanvasElement, texture_width: u32, texture_height: u32) -> anyhow::Result<Self> {
        browser_log("gpu instance: detecting WebGPU with WebGL fallback");
        let instance = gpu::create_instance(gpu::instance_descriptor_without_display()).await;
        let surface = instance
            .create_surface(SurfaceTarget::Canvas(canvas.clone()))
            .context("create browser GPU surface")?;
        browser_log("gpu surface created");
        let adapter = gpu::request_preferred_adapter(&instance, &surface).await?;
        let adapter_info = adapter.get_info();
        browser_log(format!("gpu adapter backend={:?} name={}", adapter_info.backend, adapter_info.name));
        let presenter_info = gpu::adapter_info_string(&adapter);

        let (device, queue) = gpu::request_device(&adapter, "RustJava browser GPU device").await?;

        let width = canvas.width().max(1);
        let height = canvas.height().max(1);
        let mut config = surface
            .get_default_config(&adapter, width, height)
            .context("browser GPU surface is not supported by selected adapter")?;
        config.present_mode = PresentMode::AutoVsync;
        config.desired_maximum_frame_latency = 1;
        surface.configure(&device, &config);
        browser_log(format!(
            "gpu configured surface={}x{} format={:?}",
            config.width, config.height, config.format
        ));

        let (frame_texture, bind_group, pipeline, alpha_pipeline) = create_frame_pipeline(&device, config.format, texture_width, texture_height);
        let v3 = V3GpuPresenter::new(&device, &queue, config.format);
        let m3g = M3gGpuPresenter::new(&device, &queue, config.format);

        Ok(Self {
            canvas,
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
            last_v3_generation: 0,
            last_m3g_generation: 0,
            texture_width,
            texture_height,
            rgba_words: vec![0; texture_width as usize * texture_height as usize],
            info: presenter_info,
        })
    }

    pub(super) fn render(&mut self, texture_width: u32, texture_height: u32, pixels: &[u32]) -> anyhow::Result<()> {
        self.resize_surface();
        self.resize_frame_texture(texture_width, texture_height);
        self.upload_frame(pixels);

        let output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(output) | CurrentSurfaceTexture::Suboptimal(output) => output,
            CurrentSurfaceTexture::Lost | CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return Ok(()),
            CurrentSurfaceTexture::Validation => anyhow::bail!("browser GPU surface validation failed"),
        };

        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("RustJava browser frame encoder"),
        });
        let overlay_guard = self.present_layers(&mut encoder, &view);

        self.queue.submit([encoder.finish()]);
        drop(overlay_guard);
        output.present();
        Ok(())
    }

    pub(super) fn render_overlays_if_changed(&mut self) -> anyhow::Result<bool> {
        let has_v3 = latest_gpu_frame_after(self.last_v3_generation).is_some();
        let has_m3g = latest_m3g_gpu_frame_after(self.last_m3g_generation).is_some();
        if !has_v3 && !has_m3g {
            return Ok(false);
        }

        self.resize_surface();
        let output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(output) | CurrentSurfaceTexture::Suboptimal(output) => output,
            CurrentSurfaceTexture::Lost | CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(false);
            }
            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return Ok(false),
            CurrentSurfaceTexture::Validation => anyhow::bail!("browser GPU surface validation failed"),
        };

        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("RustJava browser overlay encoder"),
        });
        let overlay_guard = self.present_layers(&mut encoder, &view);
        self.queue.submit([encoder.finish()]);
        drop(overlay_guard);
        output.present();
        Ok(true)
    }

    fn present_layers(&mut self, encoder: &mut CommandEncoder, view: &TextureView) -> OverlayGuard {
        let scale_x = self.config.width as f32 / self.texture_width.max(1) as f32;
        let scale_y = self.config.height as f32 / self.texture_height.max(1) as f32;
        let m3g_frame = latest_m3g_gpu_frame_after(self.last_m3g_generation);
        let hud = m3g_frame.as_ref().is_some_and(|frame| frame.color_clear);
        let m3g_guard = if hud {
            m3g_frame.map(|frame| {
                self.last_m3g_generation = frame.generation;
                self.m3g.encode(
                    &self.device,
                    &self.queue,
                    encoder,
                    view,
                    &frame,
                    self.config.width,
                    self.config.height,
                    Some(gpu::scaled_m3g_viewport(&frame, scale_x, scale_y)),
                    LoadOp::Clear(Color::BLACK),
                )
            })
        } else {
            self.blit_frame(encoder, view, LoadOp::Clear(Color::BLACK), false);
            m3g_frame.map(|frame| {
                self.last_m3g_generation = frame.generation;
                self.m3g.encode(
                    &self.device,
                    &self.queue,
                    encoder,
                    view,
                    &frame,
                    self.config.width,
                    self.config.height,
                    Some(gpu::scaled_m3g_viewport(&frame, scale_x, scale_y)),
                    LoadOp::Load,
                )
            })
        };
        if hud {
            self.blit_frame(encoder, view, LoadOp::Load, true);
        }
        let v3_guard = latest_gpu_frame_after(self.last_v3_generation).map(|frame| {
            self.last_v3_generation = frame.generation;
            self.v3.encode(&self.device, &self.queue, encoder, view, &frame, None, LoadOp::Load)
        });
        OverlayGuard {
            _m3g: m3g_guard,
            _v3: v3_guard,
        }
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
            label: Some("RustJava browser frame pass"),
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

    fn resize_surface(&mut self) {
        let width = self.canvas.width().max(1);
        let height = self.canvas.height().max(1);
        if self.config.width == width && self.config.height == height {
            return;
        }

        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    fn resize_frame_texture(&mut self, width: u32, height: u32) {
        if self.texture_width == width && self.texture_height == height {
            return;
        }

        let (frame_texture, bind_group, pipeline, alpha_pipeline) = create_frame_pipeline(&self.device, self.config.format, width, height);
        self.frame_texture = frame_texture;
        self.bind_group = bind_group;
        self.pipeline = pipeline;
        self.alpha_pipeline = alpha_pipeline;
        self.texture_width = width;
        self.texture_height = height;
        self.rgba_words.resize(width as usize * height as usize, 0);
    }

    fn upload_frame(&mut self, pixels: &[u32]) {
        let required_len = self.texture_width as usize * self.texture_height as usize;
        if pixels.len() < required_len {
            return;
        }
        self.rgba_words.resize(required_len, 0);
        for (&src, dst) in pixels.iter().take(required_len).zip(&mut self.rgba_words) {
            *dst = (src & 0xff00_0000) | ((src & 0x0000_00ff) << 16) | (src & 0x0000_ff00) | ((src & 0x00ff_0000) >> 16);
        }

        let rgba_bytes: &[u8] = cast_slice(&self.rgba_words);
        self.queue.write_texture(
            TexelCopyTextureInfo {
                texture: &self.frame_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            rgba_bytes,
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
        label: Some("RustJava browser frame texture"),
        size: Extent3d {
            width: texture_width,
            height: texture_height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8Unorm,
        usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    let frame_view = frame_texture.create_view(&Default::default());
    let sampler = device.create_sampler(&SamplerDescriptor {
        label: Some("RustJava browser frame sampler"),
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        mipmap_filter: MipmapFilterMode::Nearest,
        ..Default::default()
    });

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("RustJava browser frame bind group layout"),
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
        label: Some("RustJava browser frame bind group"),
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
        label: Some("RustJava browser frame pipeline layout"),
        bind_group_layouts: &[Some(&bind_group_layout)],
        immediate_size: 0,
    });
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: Some("RustJava browser frame shader"),
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
    let pipeline = make_pipeline("RustJava browser frame pipeline", None);
    let alpha_pipeline = make_pipeline("RustJava browser frame alpha pipeline", Some(BlendState::ALPHA_BLENDING));

    (frame_texture, bind_group, pipeline, alpha_pipeline)
}
