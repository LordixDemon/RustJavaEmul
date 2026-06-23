use alloc::vec::Vec;
use std::{
    collections::BTreeSet,
    io::Write,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::Context;
use jvm::Jvm;
use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
    Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, CurrentSurfaceTexture, Device, DeviceDescriptor, Extent3d, Features, FilterMode,
    FragmentState, FrontFace, Instance, Limits, LoadOp, MemoryHints, MipmapFilterMode, MultisampleState, Operations, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, SamplerBindingType, SamplerDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp, Surface, SurfaceConfiguration, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture,
    TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureViewDimension, Trace, VertexState,
};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use super::{
    RuntimeImpl,
    controls::{CONTROL_PANEL_HEIGHT, KEYBOARD_MAPPINGS, control_at, draw_control_panel, normalize_mouse_pos},
    window_target_fps,
};
use crate::profile;

const SHADER: &str = r#"
struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOut {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
    );
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0),
    );

    var out: VertexOut;
    out.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    out.uv = uvs[vertex_index];
    return out;
}

@group(0) @binding(0) var frame_texture: texture_2d<f32>;
@group(0) @binding(1) var frame_sampler: sampler;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let color = textureSample(frame_texture, frame_sampler, in.uv);
    return vec4<f32>(color.rgb, 1.0);
}
"#;

pub(super) fn run<T>(runtime: RuntimeImpl<T>, jvm: Jvm) -> anyhow::Result<()>
where
    T: Sync + Send + Write + 'static,
{
    let event_loop = EventLoop::new()?;
    let tokio = tokio::runtime::Handle::current();
    let mut app = GpuWindowApp::new(runtime, jvm, tokio);
    event_loop.run_app(&mut app)?;

    if let Some(error) = app.error { Err(error) } else { Ok(()) }
}

struct GpuWindowApp<T>
where
    T: Sync + Send + Write + 'static,
{
    runtime: RuntimeImpl<T>,
    jvm: Jvm,
    tokio: tokio::runtime::Handle,
    window_state: Option<WindowState>,
    window_width: usize,
    window_height: usize,
    game_width: usize,
    game_height: usize,
    pixels: Vec<u32>,
    texture_dirty: bool,
    last_generation: u64,
    pressed_physical_keys: BTreeSet<KeyCode>,
    pressed_game_keys: BTreeSet<i32>,
    pressed_control: Option<i32>,
    rendered_control: Option<i32>,
    cursor_pos: Option<(usize, usize)>,
    mouse_left_down: bool,
    target_frame_interval: Duration,
    next_frame_at: Instant,
    diagnostics: bool,
    profiling: bool,
    frames: u64,
    last_frame_log: Instant,
    last_profile_log: Instant,
    error: Option<anyhow::Error>,
}

impl<T> GpuWindowApp<T>
where
    T: Sync + Send + Write + 'static,
{
    fn new(runtime: RuntimeImpl<T>, jvm: Jvm, tokio: tokio::runtime::Handle) -> Self {
        let (game_width, game_height) = {
            let screen = runtime.screen.lock().unwrap();
            (screen.width, screen.height)
        };
        let window_width = game_width.max(240);
        let window_height = game_height + CONTROL_PANEL_HEIGHT;
        let mut pixels = vec![0; window_width * window_height];
        draw_control_panel(&mut pixels, window_width, game_height, CONTROL_PANEL_HEIGHT, None);

        let profiling = profile::enabled();
        if profiling {
            profile::reset();
            jvm_rust::reset_profile();
        }

        Self {
            runtime,
            jvm,
            tokio,
            window_state: None,
            window_width,
            window_height,
            game_width,
            game_height,
            pixels,
            texture_dirty: true,
            last_generation: 0,
            pressed_physical_keys: BTreeSet::new(),
            pressed_game_keys: BTreeSet::new(),
            pressed_control: None,
            rendered_control: None,
            cursor_pos: None,
            mouse_left_down: false,
            target_frame_interval: target_frame_interval(),
            next_frame_at: Instant::now(),
            diagnostics: std::env::var_os("RUSTJAVA_DIAG").is_some(),
            profiling,
            frames: 0,
            last_frame_log: Instant::now(),
            last_profile_log: Instant::now(),
            error: None,
        }
    }

    fn create_window(&mut self, event_loop: &ActiveEventLoop) -> anyhow::Result<()> {
        let window = Arc::new(
            event_loop.create_window(
                WindowAttributes::default()
                    .with_title("RustJava - J2ME")
                    .with_inner_size(LogicalSize::new(self.window_width as f64, self.window_height as f64))
                    .with_resizable(false),
            )?,
        );
        let presenter = pollster::block_on(GpuPresenter::new(window.clone(), self.window_width as u32, self.window_height as u32))?;
        self.window_state = Some(WindowState { window, presenter });
        Ok(())
    }

    fn redraw(&mut self, event_loop: &ActiveEventLoop) {
        if self.error.is_some() {
            event_loop.exit();
            return;
        }

        let _loop_timer = profile::timer(&profile::WINDOW_LOOP);
        let changed = self.refresh_pixels_from_screen() | self.refresh_control_panel();
        if changed {
            self.texture_dirty = true;
        }

        let Some(_) = self.window_state.as_ref() else {
            return;
        };

        let upload = self.texture_dirty.then_some(self.pixels.as_slice());
        let (window, result) = {
            let state = self.window_state.as_mut().unwrap();
            let window = state.window.clone();
            let result = {
                let _update_timer = profile::timer(&profile::WINDOW_UPDATE);
                state.presenter.render(upload)
            };
            (window, result)
        };
        match result {
            Ok(()) => {
                self.texture_dirty = false;
                self.log_frame_if_needed(&window);
                self.log_profile_if_needed();
            }
            Err(error) => {
                self.error = Some(error);
                event_loop.exit();
            }
        }
    }

    fn refresh_pixels_from_screen(&mut self) -> bool {
        let _copy_timer = profile::timer(&profile::WINDOW_FRAME_COPY);
        let Ok(mut screen) = self.runtime.screen.try_lock() else {
            return false;
        };
        screen.publish_pending_present();
        if screen.generation == self.last_generation {
            return false;
        }

        for row in 0..self.game_height {
            let screen_row_start = row * self.game_width;
            let window_row_start = row * self.window_width;
            self.pixels[window_row_start..window_row_start + self.game_width]
                .copy_from_slice(&screen.front_pixels[screen_row_start..screen_row_start + self.game_width]);
            self.pixels[window_row_start + self.game_width..window_row_start + self.window_width].fill(0);
        }
        self.last_generation = screen.generation;
        true
    }

    fn refresh_control_panel(&mut self) -> bool {
        if self.rendered_control == self.pressed_control {
            return false;
        }

        draw_control_panel(
            &mut self.pixels,
            self.window_width,
            self.game_height,
            CONTROL_PANEL_HEIGHT,
            self.pressed_control,
        );
        self.rendered_control = self.pressed_control;
        true
    }

    fn log_frame_if_needed(&mut self, window: &Window) {
        if !self.diagnostics {
            return;
        }

        self.frames += 1;
        if self.last_frame_log.elapsed() >= Duration::from_secs(1) {
            let stacks = self.jvm.stack_trace_all_threads().join(" | ");
            window.set_title(&format!(
                "RustJava - J2ME GPU gen {} fps {} {}",
                self.last_generation, self.frames, stacks
            ));
            self.frames = 0;
            self.last_frame_log = Instant::now();
        }
    }

    fn log_profile_if_needed(&mut self) {
        if !self.profiling || self.last_profile_log.elapsed() < Duration::from_secs(2) {
            return;
        }

        let elapsed = self.last_profile_log.elapsed();
        eprintln!("{}", profile::report(jvm_rust::profile_snapshot(), elapsed));
        eprintln!("  stacks {}", self.jvm.stack_trace_all_threads().join(" | "));
        profile::reset();
        jvm_rust::reset_profile();
        self.last_profile_log = Instant::now();
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, event: &KeyEvent) {
        let PhysicalKey::Code(code) = event.physical_key else {
            return;
        };

        if code == KeyCode::Escape && event.state == ElementState::Pressed {
            if let Err(error) = self.release_pressed_inputs() {
                self.error = Some(error);
            }
            event_loop.exit();
            return;
        }

        let _input_timer = profile::timer(&profile::WINDOW_INPUT);
        match event.state {
            ElementState::Pressed => {
                self.pressed_physical_keys.insert(code);
            }
            ElementState::Released => {
                self.pressed_physical_keys.remove(&code);
            }
        }

        if let Err(error) = self.sync_keyboard_game_keys() {
            self.error = Some(error);
            event_loop.exit();
        }
    }

    fn sync_keyboard_game_keys(&mut self) -> anyhow::Result<()> {
        let next_keys = KEYBOARD_MAPPINGS
            .iter()
            .filter_map(|(key, key_code)| self.pressed_physical_keys.contains(key).then_some(*key_code))
            .collect::<BTreeSet<_>>();
        let released = self.pressed_game_keys.difference(&next_keys).copied().collect::<Vec<_>>();
        let pressed = next_keys.difference(&self.pressed_game_keys).copied().collect::<Vec<_>>();

        for key_code in released {
            self.dispatch_key_sync(key_code, false)?;
        }
        for key_code in pressed {
            self.dispatch_key_sync(key_code, true)?;
        }

        self.pressed_game_keys = next_keys;
        Ok(())
    }

    fn update_cursor_pos(&mut self, position: PhysicalPosition<f64>) {
        let Some(state) = self.window_state.as_ref() else {
            return;
        };
        let logical = position.to_logical::<f64>(state.window.scale_factor());
        self.cursor_pos = Some(normalize_mouse_pos(
            logical.x as f32,
            logical.y as f32,
            self.window_width,
            self.window_height,
        ));
    }

    fn set_mouse_left_down(&mut self, pressed: bool) -> anyhow::Result<()> {
        let _input_timer = profile::timer(&profile::WINDOW_INPUT);
        self.mouse_left_down = pressed;
        self.sync_pointer_control()
    }

    fn sync_pointer_control(&mut self) -> anyhow::Result<()> {
        let next_control = if self.mouse_left_down {
            self.cursor_pos.and_then(|pos| control_at(pos, self.game_height))
        } else {
            None
        };

        if self.pressed_control == next_control {
            return Ok(());
        }

        if let Some(key_code) = self.pressed_control.take() {
            self.dispatch_key_sync(key_code, false)?;
        }
        if let Some(key_code) = next_control {
            self.dispatch_key_sync(key_code, true)?;
            self.pressed_control = Some(key_code);
        }
        Ok(())
    }

    fn release_pressed_inputs(&mut self) -> anyhow::Result<()> {
        self.pressed_physical_keys.clear();
        for key_code in core::mem::take(&mut self.pressed_game_keys) {
            self.dispatch_key_sync(key_code, false)?;
        }
        if let Some(key_code) = self.pressed_control.take() {
            self.dispatch_key_sync(key_code, false)?;
        }
        self.mouse_left_down = false;
        Ok(())
    }

    fn dispatch_key_sync(&self, key_code: i32, pressed: bool) -> anyhow::Result<()> {
        let method = if pressed { "keyPressed" } else { "keyReleased" };
        tokio::task::block_in_place(|| self.tokio.block_on(self.runtime.dispatch_canvas_key(&self.jvm, method, key_code)))
    }
}

impl<T> ApplicationHandler for GpuWindowApp<T>
where
    T: Sync + Send + Write + 'static,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_state.is_none() {
            if let Err(error) = self.create_window(event_loop) {
                self.error = Some(error);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let Some(state) = self.window_state.as_mut() else {
            return;
        };
        if state.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                if let Err(error) = self.release_pressed_inputs() {
                    self.error = Some(error);
                }
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                state.presenter.resize(size);
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                state.presenter.resize(state.window.inner_size());
            }
            WindowEvent::RedrawRequested => {
                self.redraw(event_loop);
            }
            WindowEvent::KeyboardInput {
                event, is_synthetic: false, ..
            } => {
                self.handle_key(event_loop, &event);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.update_cursor_pos(position);
                if self.mouse_left_down {
                    if let Err(error) = self.sync_pointer_control() {
                        self.error = Some(error);
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::CursorLeft { .. } => {
                self.cursor_pos = None;
                if self.mouse_left_down {
                    if let Err(error) = self.sync_pointer_control() {
                        self.error = Some(error);
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                if let Err(error) = self.set_mouse_left_down(state == ElementState::Pressed) {
                    self.error = Some(error);
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.error.is_some() {
            event_loop.exit();
            return;
        }

        let Some(state) = self.window_state.as_ref() else {
            return;
        };

        let now = Instant::now();
        if now >= self.next_frame_at {
            state.window.request_redraw();
            self.next_frame_at = now + self.target_frame_interval;
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame_at));
    }
}

struct WindowState {
    window: Arc<Window>,
    presenter: GpuPresenter,
}

struct GpuPresenter {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    frame_texture: Texture,
    bind_group: BindGroup,
    pipeline: RenderPipeline,
    texture_width: u32,
    texture_height: u32,
}

impl GpuPresenter {
    async fn new(window: Arc<Window>, texture_width: u32, texture_height: u32) -> anyhow::Result<Self> {
        let instance = Instance::default();
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .context("no compatible GPU adapter found")?;

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("RustJava GPU device"),
                required_features: Features::empty(),
                required_limits: Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .await?;

        let size = nonzero_size(window.inner_size());
        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .context("GPU surface is not supported by the selected adapter")?;
        config.present_mode = PresentMode::AutoVsync;
        config.desired_maximum_frame_latency = 1;
        surface.configure(&device, &config);

        let (frame_texture, bind_group, pipeline) = create_frame_pipeline(&device, config.format, texture_width, texture_height);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            frame_texture,
            bind_group,
            pipeline,
            texture_width,
            texture_height,
        })
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        let size = nonzero_size(size);
        if self.config.width == size.width && self.config.height == size.height {
            return;
        }

        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn render(&mut self, pixels: Option<&[u32]>) -> anyhow::Result<()> {
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
        {
            let color_attachment = Some(RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
            });
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("RustJava frame pass"),
                color_attachments: &[color_attachment],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.draw(0..6, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        output.present();
        Ok(())
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
) -> (Texture, BindGroup, RenderPipeline) {
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
        source: ShaderSource::Wgsl(SHADER.into()),
    });
    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("RustJava frame pipeline"),
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
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    });

    (frame_texture, bind_group, pipeline)
}

fn nonzero_size(size: PhysicalSize<u32>) -> PhysicalSize<u32> {
    PhysicalSize::new(size.width.max(1), size.height.max(1))
}

fn target_frame_interval() -> Duration {
    let fps = window_target_fps().max(1) as u64;
    Duration::from_nanos((1_000_000_000 / fps).max(1))
}
