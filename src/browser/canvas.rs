use alloc::{vec, vec::Vec};
use bytemuck::cast_slice;
use wasm_bindgen::{JsValue, prelude::*};
use web_sys::HtmlCanvasElement;

use super::{browser_log, js_value_to_error};

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
pub(super) struct BrowserCanvasPresenter {
    state: JsValue,
    rgba_words: Vec<u32>,
    texture_width: u32,
    texture_height: u32,
}

impl BrowserCanvasPresenter {
    pub(super) fn new(canvas: HtmlCanvasElement, texture_width: u32, texture_height: u32) -> anyhow::Result<Self> {
        let state = rustjava_canvas2d_create(&canvas, texture_width, texture_height).map_err(js_value_to_error)?;
        browser_log("renderer canvas2d ready");

        Ok(Self {
            state,
            rgba_words: vec![0; texture_width as usize * texture_height as usize],
            texture_width,
            texture_height,
        })
    }

    pub(super) fn render(&mut self, texture_width: u32, texture_height: u32, pixels: &[u32]) -> anyhow::Result<()> {
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
        browser_log(format!("renderer canvas2d frame resize={width}x{height}"));
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
