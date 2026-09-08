use alloc::{boxed::Box, string::String, vec, vec::Vec};
use std::path::PathBuf;

use jvm::{ClassInstance, Jvm};
use wasm_bindgen::{JsValue, prelude::*};
use web_sys::HtmlCanvasElement;

use crate::{
    StartType, create_jvm_with_screen, invoke_entrypoint, java_error_to_anyhow,
    runtime::{RuntimeImpl, browser_timer_diagnostics, jvm_profile_diagnostics, reset_jvm_profile_diagnostics},
};
use java_runtime::classes::com::mascotcapsule::micro3d::v3::{latest_render_diagnostics, set_gpu_scene_enabled};

mod canvas;
mod gpu;

use self::{canvas::BrowserCanvasPresenter, gpu::BrowserGpuPresenter};

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
        browser_log(format!("web.create canvas={width}x{height}"));
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

        if self.presenter.render_overlays_if_changed().map_err(js_error)? {
            changed = true;
        }

        Ok(changed)
    }

    #[wasm_bindgen(js_name = deviceProfile)]
    pub fn device_profile(&self) -> String {
        self.runtime
            .as_ref()
            .map(|runtime| runtime.device_profile().as_str().to_string())
            .unwrap_or_else(|| "Generic".to_string())
    }

    #[wasm_bindgen(js_name = sessionLabel)]
    pub fn session_label(&self) -> String {
        self.runtime
            .as_ref()
            .map(|runtime| runtime.session_label())
            .unwrap_or_else(|| "RustJava".to_string())
    }

    #[wasm_bindgen(js_name = keyLayout)]
    pub fn key_layout(&self) -> js_sys::Object {
        let keys = self
            .runtime
            .as_ref()
            .map(|runtime| runtime.device_profile().key_layout())
            .unwrap_or(java_runtime::KeyLayout::NOKIA);
        let object = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&object, &"up".into(), &keys.up.into());
        let _ = js_sys::Reflect::set(&object, &"down".into(), &keys.down.into());
        let _ = js_sys::Reflect::set(&object, &"left".into(), &keys.left.into());
        let _ = js_sys::Reflect::set(&object, &"right".into(), &keys.right.into());
        let _ = js_sys::Reflect::set(&object, &"fire".into(), &keys.fire.into());
        let _ = js_sys::Reflect::set(&object, &"softLeft".into(), &keys.soft_left.into());
        let _ = js_sys::Reflect::set(&object, &"softRight".into(), &keys.soft_right.into());
        object
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

#[allow(clippy::large_enum_variant)]
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

    fn render_overlays_if_changed(&mut self) -> anyhow::Result<bool> {
        match self {
            Self::Gpu(presenter) => presenter.render_overlays_if_changed(),
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
pub(super) fn js_error(error: impl core::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}

pub(super) fn js_value_to_error(value: JsValue) -> anyhow::Error {
    anyhow::anyhow!("{}", js_value_to_string(&value))
}

pub(super) fn js_value_to_string(value: &JsValue) -> String {
    if let Some(string) = value.as_string() {
        return string;
    }
    js_sys::JSON::stringify(value)
        .ok()
        .and_then(|string| string.as_string())
        .unwrap_or_else(|| format!("{value:?}"))
}

pub(super) fn browser_is_secure_context() -> bool {
    web_sys::window().is_some_and(|window| window.is_secure_context())
}

pub(super) fn browser_navigator_has_property(name: &str) -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(navigator) = js_sys::Reflect::get(&window, &JsValue::from_str("navigator")) else {
        return false;
    };
    js_sys::Reflect::has(&navigator, &JsValue::from_str(name)).unwrap_or(false)
}

pub(super) fn probe_canvas_context_available(context_id: &str) -> bool {
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

pub(crate) fn browser_log(message: impl core::fmt::Display) {
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
