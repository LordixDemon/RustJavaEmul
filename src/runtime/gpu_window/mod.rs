use alloc::vec::Vec;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    sync::Arc,
    time::{Duration, Instant},
};

use jvm::Jvm;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition},
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use super::{
    RuntimeImpl,
    control_server::{self, ControlCommand, ControlHandle},
    controls::{CONTROL_PANEL_HEIGHT, control_at, draw_control_panel, keyboard_mappings, normalize_mouse_pos},
    screenshot::ScreenshotSink,
    window::window_target_fps,
};
use crate::profile;
use java_runtime::classes::com::mascotcapsule::micro3d::v3::{latest_gpu_frame_after, set_gpu_scene_enabled};
use java_runtime::classes::javax::microedition::m3g::{latest_m3g_gpu_frame_after, latest_render_diagnostics};

mod presenter;

use presenter::{GpuPresenter, WindowState};

pub(super) fn run<T>(runtime: RuntimeImpl<T>, jvm: Jvm) -> anyhow::Result<()>
where
    T: Sync + Send + Write + 'static,
{
    #[cfg(target_os = "windows")]
    let event_loop = {
        use winit::platform::windows::EventLoopBuilderExtWindows;
        EventLoop::builder().with_any_thread(true).build()?
    };
    #[cfg(not(target_os = "windows"))]
    let event_loop = EventLoop::new()?;
    let tokio = tokio::runtime::Handle::current();
    let mut app = GpuWindowApp::new(runtime, jvm, tokio);
    control_server::spawn(app.control.clone());
    event_loop.run_app(&mut app)?;

    set_gpu_scene_enabled(false);
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
    screenshots: ScreenshotSink,
    control: ControlHandle,
    remote_keys: BTreeSet<i32>,
    remote_hold: BTreeMap<i32, u32>,
    error: Option<anyhow::Error>,
}

impl<T> GpuWindowApp<T>
where
    T: Sync + Send + Write + 'static,
{
    fn new(runtime: RuntimeImpl<T>, jvm: Jvm, tokio: tokio::runtime::Handle) -> Self {
        let (game_width, game_height) = {
            let screen = runtime.screen.lock();
            (screen.width, screen.height)
        };
        let window_width = game_width.max(240);
        let window_height = game_height + CONTROL_PANEL_HEIGHT;
        let mut pixels = vec![0; window_width * window_height];
        draw_control_panel(
            &mut pixels,
            window_width,
            game_height,
            CONTROL_PANEL_HEIGHT,
            None,
            runtime.device_profile(),
        );

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
            diagnostics: {
                let diag = crate::config::get().diag;
                java_runtime::classes::java::lang::Thread::set_sleep_trace_enabled(diag || profiling);
                diag
            },
            profiling,
            frames: 0,
            last_frame_log: Instant::now(),
            last_profile_log: Instant::now(),
            screenshots: ScreenshotSink::from_env(),
            control: ControlHandle::new(),
            remote_keys: BTreeSet::new(),
            remote_hold: BTreeMap::new(),
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
        let presenter = pollster::block_on(GpuPresenter::new(
            window.clone(),
            event_loop.owned_display_handle(),
            self.window_width as u32,
            self.window_height as u32,
        ))?;
        set_gpu_scene_enabled(true);
        window.set_title(&format!("RustJava — {} — {}", self.runtime.session_label(), presenter.info));
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
        if let Err(error) = self.drain_control() {
            self.error = Some(error);
            event_loop.exit();
            return;
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
                state.presenter.render(upload, self.game_width as u32, self.game_height as u32)
            };
            (window, result)
        };
        match result {
            Ok(()) => {
                self.texture_dirty = false;
                self.log_frame_if_needed(&window);
                self.log_profile_if_needed();
                self.screenshots
                    .save_if_due(self.game_width, self.game_height, self.window_width, &self.pixels);
            }
            Err(error) => {
                self.error = Some(error);
                event_loop.exit();
            }
        }
    }

    fn refresh_pixels_from_screen(&mut self) -> bool {
        let _copy_timer = profile::timer(&profile::WINDOW_FRAME_COPY);
        let Some(mut screen) = self.runtime.screen.try_lock() else {
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
            self.runtime.device_profile(),
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
        eprintln!("  {}", latest_render_diagnostics().replace('\n', " | "));
        match latest_m3g_gpu_frame_after(0) {
            Some(frame) => eprintln!(
                "  m3g.gpu gen={} {}x{}+{}+{} tex={} tri={} color_clear={} depth_clear={}",
                frame.generation,
                frame.width,
                frame.height,
                frame.viewport_x,
                frame.viewport_y,
                frame.textures.len(),
                frame.triangles.len(),
                frame.color_clear,
                frame.clear_depth
            ),
            None => eprintln!("  m3g.gpu none"),
        }
        match latest_gpu_frame_after(0) {
            Some(frame) => eprintln!("  v3.gpu gen={} tri={}", frame.generation, frame.triangles.len()),
            None => eprintln!("  v3.gpu none"),
        }
        eprintln!("  stacks {}", self.jvm.stack_trace_all_threads().join(" | "));
        profile::reset();
        jvm_rust::reset_profile();
        self.last_profile_log = Instant::now();
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, event: &KeyEvent) {
        let PhysicalKey::Code(code) = event.physical_key else {
            return;
        };

        if code == KeyCode::F12 && event.state == ElementState::Pressed && !event.repeat {
            self.screenshots
                .save_lcd("hotkey", self.game_width, self.game_height, self.window_width, &self.pixels);
            return;
        }

        if code == KeyCode::Escape && event.state == ElementState::Pressed {
            set_gpu_scene_enabled(false);
            if let Err(error) = self.release_pressed_inputs() {
                self.error = Some(error);
            }
            event_loop.exit();
            return;
        }

        let _input_timer = profile::timer(&profile::WINDOW_INPUT);
        match event.state {
            ElementState::Pressed => {
                let is_new = self.pressed_physical_keys.insert(code);
                if !is_new && event.repeat {
                    for (key, key_code) in keyboard_mappings(self.runtime.device_profile()) {
                        if key == code {
                            if let Err(error) = self.dispatch_key_sync(key_code, true) {
                                self.error = Some(error);
                                event_loop.exit();
                                return;
                            }
                        }
                    }
                }
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
        let next_keys = keyboard_mappings(self.runtime.device_profile())
            .into_iter()
            .filter_map(|(key, key_code)| self.pressed_physical_keys.contains(&key).then_some(key_code))
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
            self.cursor_pos
                .and_then(|pos| control_at(pos, self.game_height, self.runtime.device_profile()))
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
        for key_code in core::mem::take(&mut self.remote_keys) {
            self.dispatch_key_sync(key_code, false)?;
        }
        self.remote_hold.clear();
        if let Some(key_code) = self.pressed_control.take() {
            self.dispatch_key_sync(key_code, false)?;
        }
        self.mouse_left_down = false;
        Ok(())
    }

    fn drain_control(&mut self) -> anyhow::Result<()> {
        let expired: Vec<i32> = self
            .remote_hold
            .iter_mut()
            .filter_map(|(code, frames)| {
                *frames = frames.saturating_sub(1);
                (*frames == 0).then_some(*code)
            })
            .collect();
        for code in expired {
            self.remote_hold.remove(&code);
            self.remote_up(code)?;
        }

        for command in self.control.take() {
            match command {
                ControlCommand::Down(code) => self.remote_down(code)?,
                ControlCommand::Up(code) => self.remote_up(code)?,
                ControlCommand::Tap { code, hold_frames } => {
                    self.remote_down(code)?;
                    self.remote_hold.insert(code, hold_frames.max(1));
                }
                ControlCommand::Screenshot { reply } => {
                    let result = control_server::encode_game_png(self.game_width, self.game_height, self.window_width, &self.pixels);
                    let _ = reply.send(result);
                }
                ControlCommand::Status { reply } => {
                    let _ = reply.send(self.control_status());
                }
            }
        }
        Ok(())
    }

    fn remote_down(&mut self, code: i32) -> anyhow::Result<()> {
        if self.remote_keys.insert(code) {
            self.dispatch_key_sync(code, true)?;
        }
        Ok(())
    }

    fn remote_up(&mut self, code: i32) -> anyhow::Result<()> {
        if self.remote_keys.remove(&code) {
            self.dispatch_key_sync(code, false)?;
        }
        self.remote_hold.remove(&code);
        Ok(())
    }

    fn control_status(&self) -> String {
        let displayable = self
            .runtime
            .current_displayable
            .lock()
            .as_ref()
            .map(|displayable| displayable.class_definition().name())
            .unwrap_or_else(|| "<none>".into());
        let mut lines = vec![
            "ok".into(),
            format!("displayable={displayable}"),
            format!("screen={}x{} gen={}", self.game_width, self.game_height, self.last_generation),
            format!("keys={:#06x}", self.runtime.game_key_states.load(core::sync::atomic::Ordering::Relaxed)),
            format!(
                "remote={}",
                self.remote_keys
                    .iter()
                    .copied()
                    .map(|code| code.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ];
        lines.push(latest_render_diagnostics().replace('\n', " | "));
        match latest_m3g_gpu_frame_after(0) {
            Some(frame) => lines.push(format!(
                "m3g.gpu gen={} {}x{} tex={} tri={} color_clear={}",
                frame.generation,
                frame.width,
                frame.height,
                frame.textures.len(),
                frame.triangles.len(),
                frame.color_clear
            )),
            None => lines.push("m3g.gpu none".into()),
        }
        match latest_gpu_frame_after(0) {
            Some(frame) => lines.push(format!("v3.gpu gen={} tri={}", frame.generation, frame.triangles.len())),
            None => lines.push("v3.gpu none".into()),
        }
        for (tid, frames) in self.jvm.dump_threads() {
            lines.push(format!("thread #{tid} frames={}", frames.len()));
            for frame in frames.iter().rev().take(10) {
                lines.push(format!("  at {frame}"));
            }
        }
        lines.join("\n") + "\n"
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
                set_gpu_scene_enabled(false);
                if let Err(error) = self.release_pressed_inputs() {
                    self.error = Some(error);
                }
                self.runtime.abort_spawned();
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

fn target_frame_interval() -> Duration {
    let fps = window_target_fps().max(1) as u64;
    Duration::from_nanos((1_000_000_000 / fps).max(1))
}
