mod m3g;
mod shaders;
mod v3;

use alloc::string::String;

use anyhow::Context;
use wgpu::{
    Adapter, Backend, Backends, Device, DeviceDescriptor, DeviceType, ExperimentalFeatures, Features, Instance, InstanceDescriptor, Limits,
    MemoryHints, PowerPreference, Queue, RequestAdapterOptions, Surface, Trace,
};

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(crate) use m3g::M3gGpuDrawGuard;
pub(crate) use m3g::{M3gGpuPresenter, scaled_m3g_viewport};
pub(crate) use shaders::{FRAME_SHADER, M3G_SHADER, V3_SHADER};
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(crate) use v3::V3GpuDrawGuard;
pub(crate) use v3::V3GpuPresenter;

pub(crate) fn preferred_backends() -> Backends {
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(value) = crate::config::get().gpu_backends.as_deref() {
        let parsed = Backends::from_comma_list(value);
        if !parsed.is_empty() {
            return parsed;
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        Backends::BROWSER_WEBGPU | Backends::GL
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Backends::VULKAN | Backends::DX12 | Backends::METAL | Backends::GL
    }
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(crate) fn instance_descriptor_without_display() -> InstanceDescriptor {
    let mut descriptor = InstanceDescriptor::new_without_display_handle();
    descriptor.backends = preferred_backends();
    descriptor.with_env()
}

#[cfg(feature = "desktop-window")]
pub(crate) fn instance_descriptor_with_display(display: winit::event_loop::OwnedDisplayHandle) -> InstanceDescriptor {
    let mut descriptor = InstanceDescriptor::new_with_display_handle(alloc::boxed::Box::new(display));
    descriptor.backends = preferred_backends();
    descriptor.with_env()
}

pub(crate) async fn create_instance(descriptor: InstanceDescriptor) -> Instance {
    #[cfg(target_arch = "wasm32")]
    {
        wgpu::util::new_instance_with_webgpu_detection(descriptor).await
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Instance::new(descriptor)
    }
}

pub(crate) fn device_limits_for_adapter(adapter: &Adapter) -> Limits {
    let adapter_limits = adapter.limits();
    match adapter.get_info().backend {
        Backend::Gl => Limits::downlevel_webgl2_defaults().using_resolution(adapter_limits),
        _ => Limits::default().using_resolution(adapter_limits),
    }
}

pub(crate) fn adapter_info_string(adapter: &Adapter) -> String {
    let info = adapter.get_info();
    format!(
        "renderer=wgpu backend={:?} adapter={} deviceType={:?}",
        info.backend, info.name, info.device_type
    )
}

pub(crate) async fn request_preferred_adapter(instance: &Instance, surface: &Surface<'_>) -> anyhow::Result<Adapter> {
    let mut adapters = instance.enumerate_adapters(Backends::all()).await;
    adapters.retain(|adapter| adapter.is_surface_supported(surface));

    if adapters.iter().any(|adapter| adapter.get_info().device_type != DeviceType::Cpu) {
        adapters.retain(|adapter| adapter.get_info().device_type != DeviceType::Cpu);
    }

    adapters.sort_by_key(|adapter| {
        let info = adapter.get_info();
        (backend_priority(info.backend), device_type_priority(info.device_type))
    });

    if let Some(adapter) = adapters.into_iter().next() {
        return Ok(adapter);
    }

    instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await
        .context("no compatible GPU adapter found")
}

pub(crate) async fn request_device(adapter: &Adapter, label: &'static str) -> anyhow::Result<(Device, Queue)> {
    adapter
        .request_device(&DeviceDescriptor {
            label: Some(label),
            required_features: Features::empty(),
            required_limits: device_limits_for_adapter(adapter),
            experimental_features: ExperimentalFeatures::disabled(),
            memory_hints: MemoryHints::Performance,
            trace: Trace::Off,
        })
        .await
        .context("failed to create GPU device")
}

fn backend_priority(backend: Backend) -> u8 {
    match backend {
        Backend::Vulkan => 0,
        Backend::Metal => 1,
        Backend::Dx12 => 2,
        Backend::BrowserWebGpu => 3,
        Backend::Gl => 4,
        Backend::Noop => 9,
    }
}

fn device_type_priority(device_type: DeviceType) -> u8 {
    match device_type {
        DeviceType::DiscreteGpu => 0,
        DeviceType::IntegratedGpu => 1,
        DeviceType::VirtualGpu => 2,
        DeviceType::Other => 3,
        DeviceType::Cpu => 4,
    }
}
