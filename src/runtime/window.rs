use core::sync::atomic::Ordering;
use std::io::Write;

use jvm::Jvm;

use super::RuntimeImpl;
#[cfg(feature = "desktop-window")]
use super::gpu_window;
use crate::profile;

impl<T> RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    pub async fn dispatch_key(&self, jvm: &Jvm, key_code: i32, pressed: bool) -> anyhow::Result<()> {
        let method = if pressed { "keyPressed" } else { "keyReleased" };
        self.dispatch_canvas_key(jvm, method, key_code).await
    }

    pub fn set_key_state(&self, key_code: i32, pressed: bool) {
        let method = if pressed { "keyPressed" } else { "keyReleased" };
        self.update_game_key_state(method, key_code);
    }

    #[cfg(feature = "desktop-window")]
    pub async fn run_window(&self, jvm: &Jvm) -> anyhow::Result<()> {
        gpu_window::run(self.clone(), jvm.clone())
    }

    #[cfg(not(feature = "desktop-window"))]
    pub async fn run_window(&self, _jvm: &Jvm) -> anyhow::Result<()> {
        anyhow::bail!("desktop window support is not compiled; enable the desktop-window feature")
    }

    pub async fn dispatch_canvas_key(&self, jvm: &Jvm, method: &str, key_code: i32) -> anyhow::Result<()> {
        let _timer = profile::timer(&profile::DISPATCH_KEY);
        let diagnostics = input_diag_enabled();
        let previous_states = self.game_key_states.load(Ordering::Relaxed);
        self.update_game_key_state(method, key_code);
        let next_states = self.game_key_states.load(Ordering::Relaxed);

        let displayable = self.current_displayable.lock().clone();
        let Some(displayable) = displayable else {
            if diagnostics {
                eprintln!("[input] {method} key={key_code} displayable=<none> states={previous_states:#06x}->{next_states:#06x}");
            }
            return Ok(());
        };

        let class_name = displayable.class_definition().name();
        let has_method = displayable.class_definition().method(method, "(I)V", false).is_some();
        if diagnostics {
            eprintln!(
                "[input] {method} key={key_code} displayable={class_name} states={previous_states:#06x}->{next_states:#06x} has_method={has_method}"
            );
        }

        if !has_method {
            return Ok(());
        }

        let result: jvm::Result<()> = jvm.invoke_virtual(&displayable, method, "(I)V", (key_code,)).await;
        if diagnostics {
            match &result {
                Ok(()) => self.log_input_state(jvm, &class_name, method, key_code).await,
                Err(error) => eprintln!("[input] {method} key={key_code} displayable={class_name} error={error:?}"),
            }
        }
        result?;

        Ok(())
    }

    async fn log_input_state(&self, jvm: &Jvm, class_name: &str, method: &str, key_code: i32) {
        if !crate::config::get().input_debug {
            return;
        }

        if class_name != "b" {
            return;
        }

        let h = jvm.get_static_field::<i32>("b", "h", "I").await.ok();
        let g = jvm.get_static_field::<i32>("b", "G", "I").await.ok();
        let h_counter = jvm.get_static_field::<i32>("b", "H", "I").await.ok();
        let pressed_any = jvm.get_static_field::<bool>("b", "k", "Z").await.ok();
        let mapped_key = jvm.get_static_field::<i32>("b", "aE", "I").await.ok();
        let intro_mode = jvm.get_static_field::<bool>("TreasureTowers", "b", "Z").await.ok();
        eprintln!(
            "[input] after {method} key={key_code}: b.h={h:?} b.G={g:?} b.H={h_counter:?} b.k={pressed_any:?} b.aE={mapped_key:?} TreasureTowers.b={intro_mode:?}"
        );
    }

    fn update_game_key_state(&self, method: &str, key_code: i32) {
        let Some(mask) = self.device_profile().key_layout().game_canvas_mask(key_code) else {
            return;
        };

        match method {
            "keyPressed" => {
                self.game_key_states.fetch_or(mask, Ordering::Relaxed);
            }
            "keyReleased" => {
                self.game_key_states.fetch_and(!mask, Ordering::Relaxed);
            }
            _ => {}
        }
    }
}

#[cfg(feature = "desktop-window")]
pub(crate) fn window_target_fps() -> usize {
    crate::config::get().window_fps as usize
}

pub(crate) fn input_diag_enabled() -> bool {
    crate::config::get().input_diag
}
