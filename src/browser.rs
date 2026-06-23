use alloc::{boxed::Box, string::String, vec, vec::Vec};
use std::path::PathBuf;

use anyhow::Context;
use bytemuck::cast_slice;
use jvm::{ClassInstance, Jvm};
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::HtmlCanvasElement;
use wgpu::util::DeviceExt;
use wgpu::{
    AddressMode, Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, BlendComponent, BlendFactor, BlendOperation, BlendState, BufferUsages, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, CurrentSurfaceTexture, Device, DeviceDescriptor, Extent3d, Features, FilterMode, FragmentState, FrontFace,
    InstanceDescriptor, Limits, LoadOp, MemoryHints, MipmapFilterMode, MultisampleState, Operations, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, Sampler, SamplerBindingType, SamplerDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp, Surface, SurfaceConfiguration, SurfaceTarget, SurfaceTexture, TexelCopyBufferLayout,
    TexelCopyTextureInfo, Texture, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages,
    TextureViewDimension, Trace, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use crate::{
    StartType, create_jvm_with_screen, invoke_entrypoint, java_error_to_anyhow,
    runtime::{RuntimeImpl, browser_timer_diagnostics, jvm_profile_diagnostics, reset_jvm_profile_diagnostics},
};
use java_runtime::classes::com::mascotcapsule::micro3d::v3::{V3GpuFrame, latest_gpu_frame_after, latest_render_diagnostics, set_gpu_scene_enabled};

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

const V3_SHADER: &str = r#"
struct VertexIn {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.uv = in.uv;
    out.color = in.color;
    return out;
}

@group(0) @binding(0) var tri_texture: texture_2d<f32>;
@group(0) @binding(1) var tri_sampler: sampler;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let color = textureSample(tri_texture, tri_sampler, in.uv) * in.color;
    if (color.a <= 0.003) {
        discard;
    }
    return color;
}
"#;

#[wasm_bindgen(inline_js = r#"
export function rustjava_canvas2d_create(canvas, width, height) {
    const ctx = canvas.getContext("2d", { alpha: false, desynchronized: true });
    if (!ctx) {
        throw new Error("2d canvas context is not available");
    }
    ctx.imageSmoothingEnabled = false;
    return {
        canvas,
        ctx,
        width,
        height,
        imageData: ctx.createImageData(width, height),
    };
}

export function rustjava_canvas2d_resize(state, width, height) {
    state.width = width;
    state.height = height;
    state.imageData = state.ctx.createImageData(width, height);
}

export function rustjava_canvas2d_present(state, data) {
    if (state.imageData.data.length !== data.length) {
        throw new Error(`canvas2d buffer mismatch image=${state.imageData.data.length} data=${data.length}`);
    }
    state.imageData.data.set(data);
    state.ctx.putImageData(state.imageData, 0, 0);

    const canvasWidth = state.canvas.width;
    const canvasHeight = state.canvas.height;
    if (canvasWidth > state.width) {
        state.ctx.clearRect(state.width, 0, canvasWidth - state.width, canvasHeight);
    }
    if (canvasHeight > state.height) {
        state.ctx.clearRect(0, state.height, canvasWidth, canvasHeight - state.height);
    }
}
"#)]
extern "C" {
    #[wasm_bindgen(catch)]
    fn rustjava_canvas2d_create(canvas: &HtmlCanvasElement, width: u32, height: u32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    fn rustjava_canvas2d_resize(state: &JsValue, width: u32, height: u32) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    fn rustjava_canvas2d_present(state: &JsValue, data: &js_sys::Uint8ClampedArray) -> Result<(), JsValue>;
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    browser_log("wasm: panic hook installed");
}

#[wasm_bindgen]
pub struct RustJavaWeb {
    presenter: BrowserPresenter,
    runtime: Option<RuntimeImpl<Vec<u8>>>,
    jvm: Option<Jvm>,
    midlet: Option<Box<dyn ClassInstance>>,
    frame_pixels: Vec<u32>,
    last_generation: u64,
    game_width: usize,
    game_height: usize,
}

#[wasm_bindgen]
impl RustJavaWeb {
    #[wasm_bindgen(js_name = create)]
    pub async fn create(canvas: HtmlCanvasElement, width: u32, height: u32) -> Result<RustJavaWeb, JsValue> {
        let width = width.max(1);
        let height = height.max(1);
        browser_log(format!("web.create canvas={}x{}", width, height));
        canvas.set_width(width);
        canvas.set_height(height);

        Ok(Self {
            presenter: BrowserPresenter::new(canvas, width, height).await.map_err(js_error)?,
            runtime: None,
            jvm: None,
            midlet: None,
            frame_pixels: vec![0; width as usize * height as usize],
            last_generation: 0,
            game_width: width as usize,
            game_height: height as usize,
        })
    }

    #[wasm_bindgen(js_name = loadJar)]
    pub async fn load_jar(&mut self, name: String, bytes: Vec<u8>) -> Result<(), JsValue> {
        browser_log(format!("loadJar begin name={name} bytes={}", bytes.len()));
        self.shutdown().await?;

        let path = PathBuf::from(if name.is_empty() { "game.jar" } else { name.as_str() });
        let start_type = StartType::JarBytes {
            name: path.as_path(),
            bytes: bytes.as_slice(),
        };
        let (jvm, runtime) = create_jvm_with_screen(Vec::<u8>::new(), &start_type, &[], Some((self.game_width, self.game_height)))
            .await
            .map_err(|error| {
                browser_log(format!("loadJar create_jvm error: {error:#}"));
                js_error(error)
            })?;
        browser_log("loadJar jvm ready");
        reset_jvm_profile_diagnostics();

        let args: [&str; 0] = [];
        let midlet = match invoke_entrypoint(&jvm, &start_type, &args).await {
            Ok(midlet) => {
                browser_log(if midlet.is_some() {
                    "loadJar entrypoint started: MIDlet"
                } else {
                    "loadJar entrypoint started: main"
                });
                midlet
            }
            Err(error) => {
                runtime.abort_spawned();
                let error = java_error_to_anyhow(&jvm, error).await;
                browser_log(format!("loadJar entrypoint error: {error:#}"));
                return Err(js_error(error));
            }
        };

        self.last_generation = 0;
        self.runtime = Some(runtime);
        self.jvm = Some(jvm);
        self.midlet = midlet;
        browser_log("loadJar complete");
        Ok(())
    }

    #[wasm_bindgen(js_name = present)]
    pub fn present(&mut self) -> Result<bool, JsValue> {
        let mut changed = false;
        if let Some(runtime) = &self.runtime {
            if let Some((width, height, generation)) = runtime.copy_presented_frame(self.last_generation, &mut self.frame_pixels) {
                self.last_generation = generation;
                self.presenter.render(width as u32, height as u32, &self.frame_pixels).map_err(js_error)?;
                changed = true;
            }
        }

        if self.presenter.render_v3_if_changed().map_err(js_error)? {
            changed = true;
        }

        Ok(changed)
    }

    #[wasm_bindgen(js_name = key)]
    pub async fn key(&self, key_code: i32, pressed: bool) -> Result<(), JsValue> {
        let (Some(runtime), Some(jvm)) = (&self.runtime, &self.jvm) else {
            return Ok(());
        };
        runtime.dispatch_key(jvm, key_code, pressed).await.map_err(js_error)
    }

    #[wasm_bindgen(js_name = setKeyState)]
    pub fn set_key_state(&self, key_code: i32, pressed: bool) {
        if let Some(runtime) = &self.runtime {
            runtime.set_key_state(key_code, pressed);
        }
    }

    #[wasm_bindgen(js_name = diagnostics)]
    pub fn diagnostics(&self) -> String {
        format!(
            "{}\n{}\n{}",
            latest_render_diagnostics(),
            browser_timer_diagnostics(),
            jvm_profile_diagnostics()
        )
    }

    #[wasm_bindgen(js_name = rendererInfo)]
    pub fn renderer_info(&self) -> String {
        self.presenter.info()
    }

    #[wasm_bindgen(js_name = shutdown)]
    pub async fn shutdown(&mut self) -> Result<(), JsValue> {
        if let (Some(runtime), Some(jvm), Some(midlet)) = (&self.runtime, &self.jvm, &self.midlet) {
            browser_log("shutdown begin");
            let result = jvm.invoke_virtual::<_, ()>(midlet, "destroyApp", "(Z)V", (true,)).await;
            runtime.abort_spawned();
            if let Err(error) = result {
                let error = java_error_to_anyhow(jvm, error).await;
                browser_log(format!("shutdown error: {error:#}"));
                return Err(js_error(error));
            }
        }

        self.runtime = None;
        self.jvm = None;
        self.midlet = None;
        self.last_generation = 0;
        browser_log("shutdown complete");
        Ok(())
    }
}

enum BrowserPresenter {
    Gpu(BrowserGpuPresenter),
    Canvas2d(BrowserCanvasPresenter),
}

impl BrowserPresenter {
    async fn new(canvas: HtmlCanvasElement, texture_width: u32, texture_height: u32) -> anyhow::Result<Self> {
        browser_log(format!(
            "renderer env secure={} webgpu={} webgl2={}",
            browser_is_secure_context(),
            browser_navigator_has_property("gpu"),
            probe_canvas_context_available("webgl2"),
        ));

        if !browser_is_secure_context() {
            browser_log("renderer fallback: insecure LAN context, using canvas2d");
            set_gpu_scene_enabled(false);
            return Ok(Self::Canvas2d(BrowserCanvasPresenter::new(canvas, texture_width, texture_height)?));
        }

        match BrowserGpuPresenter::new(canvas.clone(), texture_width, texture_height).await {
            Ok(gpu) => {
                set_gpu_scene_enabled(true);
                Ok(Self::Gpu(gpu))
            }
            Err(error) => {
                browser_log(format!("renderer fallback: gpu unavailable, using canvas2d: {error:#}"));
                set_gpu_scene_enabled(false);
                Ok(Self::Canvas2d(BrowserCanvasPresenter::new(canvas, texture_width, texture_height)?))
            }
        }
    }

    fn render(&mut self, texture_width: u32, texture_height: u32, pixels: &[u32]) -> anyhow::Result<()> {
        match self {
            Self::Gpu(presenter) => presenter.render(texture_width, texture_height, pixels),
            Self::Canvas2d(presenter) => presenter.render(texture_width, texture_height, pixels),
        }
    }

    fn render_v3_if_changed(&mut self) -> anyhow::Result<bool> {
        match self {
            Self::Gpu(presenter) => presenter.render_v3_if_changed(),
            Self::Canvas2d(_) => Ok(false),
        }
    }

    fn info(&self) -> String {
        match self {
            Self::Gpu(presenter) => presenter.info.clone(),
            Self::Canvas2d(_) => "renderer=canvas2d".to_string(),
        }
    }
}

struct BrowserCanvasPresenter {
    state: JsValue,
    rgba_words: Vec<u32>,
    texture_width: u32,
    texture_height: u32,
}

impl BrowserCanvasPresenter {
    fn new(canvas: HtmlCanvasElement, texture_width: u32, texture_height: u32) -> anyhow::Result<Self> {
        let state = rustjava_canvas2d_create(&canvas, texture_width, texture_height).map_err(js_value_to_error)?;
        browser_log("renderer canvas2d ready");

        Ok(Self {
            state,
            rgba_words: vec![0; texture_width as usize * texture_height as usize],
            texture_width,
            texture_height,
        })
    }

    fn render(&mut self, texture_width: u32, texture_height: u32, pixels: &[u32]) -> anyhow::Result<()> {
        self.resize_frame_texture(texture_width, texture_height)?;
        self.upload_frame(pixels);

        let rgba_bytes: &[u8] = cast_slice(&self.rgba_words);
        let data = unsafe { js_sys::Uint8ClampedArray::view(rgba_bytes) };
        rustjava_canvas2d_present(&self.state, &data).map_err(js_value_to_error)?;
        Ok(())
    }

    fn resize_frame_texture(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        if self.texture_width == width && self.texture_height == height {
            return Ok(());
        }

        self.texture_width = width;
        self.texture_height = height;
        self.rgba_words.resize(width as usize * height as usize, 0);
        rustjava_canvas2d_resize(&self.state, width, height).map_err(js_value_to_error)?;
        browser_log(format!("renderer canvas2d frame resize={}x{}", width, height));
        Ok(())
    }

    fn upload_frame(&mut self, pixels: &[u32]) {
        let required_len = self.texture_width as usize * self.texture_height as usize;
        if pixels.len() < required_len {
            return;
        }
        self.rgba_words.resize(required_len, 0);
        for (&src, dst) in pixels.iter().take(required_len).zip(&mut self.rgba_words) {
            *dst = 0xff00_0000 | ((src & 0x0000_00ff) << 16) | (src & 0x0000_ff00) | ((src & 0x00ff_0000) >> 16);
        }
    }
}

struct BrowserGpuPresenter {
    canvas: HtmlCanvasElement,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    frame_texture: Texture,
    bind_group: BindGroup,
    pipeline: RenderPipeline,
    v3: BrowserV3GpuPresenter,
    last_v3_generation: u64,
    texture_width: u32,
    texture_height: u32,
    rgba_words: Vec<u32>,
    info: String,
}

impl BrowserGpuPresenter {
    async fn new(canvas: HtmlCanvasElement, texture_width: u32, texture_height: u32) -> anyhow::Result<Self> {
        let mut instance_descriptor = InstanceDescriptor::new_without_display_handle();
        instance_descriptor.backends = Backends::BROWSER_WEBGPU | Backends::GL;
        browser_log("gpu instance: detecting WebGPU with WebGL fallback");
        let instance = wgpu::util::new_instance_with_webgpu_detection(instance_descriptor).await;
        let surface = instance
            .create_surface(SurfaceTarget::Canvas(canvas.clone()))
            .context("create browser GPU surface")?;
        browser_log("gpu surface created");
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .context("no browser GPU adapter found")?;
        let adapter_info = adapter.get_info();
        browser_log(format!("gpu adapter backend={:?} name={}", adapter_info.backend, adapter_info.name));
        let presenter_info = format!(
            "renderer=wgpu backend={:?} adapter={} deviceType={:?}",
            adapter_info.backend, adapter_info.name, adapter_info.device_type
        );

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("RustJava browser GPU device"),
                required_features: Features::empty(),
                required_limits: Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .await?;

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

        let (frame_texture, bind_group, pipeline) = create_frame_pipeline(&device, config.format, texture_width, texture_height);
        let v3 = BrowserV3GpuPresenter::new(&device, &queue, config.format);

        Ok(Self {
            canvas,
            surface,
            device,
            queue,
            config,
            frame_texture,
            bind_group,
            pipeline,
            v3,
            last_v3_generation: 0,
            texture_width,
            texture_height,
            rgba_words: vec![0; texture_width as usize * texture_height as usize],
            info: presenter_info,
        })
    }

    fn render(&mut self, texture_width: u32, texture_height: u32, pixels: &[u32]) -> anyhow::Result<()> {
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
                label: Some("RustJava browser frame pass"),
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

    fn render_v3_if_changed(&mut self) -> anyhow::Result<bool> {
        let Some(frame) = latest_gpu_frame_after(self.last_v3_generation) else {
            return Ok(false);
        };

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

        self.v3.render(&self.device, &self.queue, output, &frame)?;
        self.last_v3_generation = frame.generation;
        Ok(true)
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

        let (frame_texture, bind_group, pipeline) = create_frame_pipeline(&self.device, self.config.format, width, height);
        self.frame_texture = frame_texture;
        self.bind_group = bind_group;
        self.pipeline = pipeline;
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
            *dst = 0xff00_0000 | ((src & 0x0000_00ff) << 16) | (src & 0x0000_ff00) | ((src & 0x00ff_0000) >> 16);
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

struct BrowserV3GpuPresenter {
    normal_pipeline: RenderPipeline,
    additive_pipeline: RenderPipeline,
    subtractive_pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    sampler: Sampler,
    _white_texture: Texture,
    white_bind_group: BindGroup,
}

struct BrowserV3Batch {
    texture: Option<usize>,
    blend_mode: u8,
    start: u32,
    count: u32,
}

impl BrowserV3GpuPresenter {
    fn new(device: &Device, queue: &Queue, surface_format: TextureFormat) -> Self {
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

    fn render(&self, device: &Device, queue: &Queue, output: SurfaceTexture, frame: &V3GpuFrame) -> anyhow::Result<()> {
        if frame.width <= 0 || frame.height <= 0 || frame.triangles.is_empty() {
            output.present();
            return Ok(());
        }

        let (vertices, batches) = self.build_vertices(frame);
        if vertices.is_empty() {
            output.present();
            return Ok(());
        }
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("RustJava V3 vertex buffer"),
            contents: cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        let texture_bind_groups = self.create_texture_bind_groups(device, queue, frame);

        let view = output.texture.create_view(&Default::default());
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("RustJava V3 frame encoder"),
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
                label: Some("RustJava V3 frame pass"),
                color_attachments: &[color_attachment],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
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

        queue.submit([encoder.finish()]);
        output.present();
        Ok(())
    }

    fn pipeline_for_blend_mode(&self, blend_mode: u8) -> &RenderPipeline {
        match blend_mode {
            2 => &self.additive_pipeline,
            3 => &self.subtractive_pipeline,
            _ => &self.normal_pipeline,
        }
    }

    fn build_vertices(&self, frame: &V3GpuFrame) -> (Vec<f32>, Vec<BrowserV3Batch>) {
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
                batches.push(BrowserV3Batch {
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
            for vertex in tri.vertices {
                let x = (vertex.x as f32 / frame.width as f32) * 2.0 - 1.0;
                let y = 1.0 - (vertex.y as f32 / frame.height as f32) * 2.0;
                vertices.extend_from_slice(&[x, y, vertex.u as f32 / 255.0, vertex.v as f32 / 255.0, r, g, b, a]);
            }
            current_count += 3;
        }

        if current_count > 0 {
            batches.push(BrowserV3Batch {
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

fn create_frame_pipeline(
    device: &Device,
    surface_format: TextureFormat,
    texture_width: u32,
    texture_height: u32,
) -> (Texture, BindGroup, RenderPipeline) {
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
        source: ShaderSource::Wgsl(SHADER.into()),
    });
    let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("RustJava browser frame pipeline"),
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

fn js_error(error: impl core::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

fn js_value_to_error(value: JsValue) -> anyhow::Error {
    anyhow::anyhow!("{}", js_value_to_string(&value))
}

fn js_value_to_string(value: &JsValue) -> String {
    if let Some(string) = value.as_string() {
        return string;
    }
    js_sys::JSON::stringify(value)
        .ok()
        .and_then(|string| string.as_string())
        .unwrap_or_else(|| format!("{value:?}"))
}

fn browser_is_secure_context() -> bool {
    web_sys::window().is_some_and(|window| window.is_secure_context())
}

fn browser_navigator_has_property(name: &str) -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(navigator) = js_sys::Reflect::get(&window, &JsValue::from_str("navigator")) else {
        return false;
    };
    js_sys::Reflect::has(&navigator, &JsValue::from_str(name)).unwrap_or(false)
}

fn probe_canvas_context_available(context_id: &str) -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Some(document) = window.document() else {
        return false;
    };
    let Ok(element) = document.create_element("canvas") else {
        return false;
    };
    let Ok(canvas) = element.dyn_into::<HtmlCanvasElement>() else {
        return false;
    };
    canvas.get_context(context_id).ok().flatten().is_some()
}

fn browser_log(message: impl core::fmt::Display) {
    let message = message.to_string();
    web_sys::console::log_1(&JsValue::from_str(&message));

    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(value) = js_sys::Reflect::get(&window, &JsValue::from_str("__rustjavaLog")) else {
        return;
    };
    if value.is_function() {
        let function: js_sys::Function = value.unchecked_into();
        let _ = function.call1(&JsValue::NULL, &JsValue::from_str(&message));
    }
}
