use std::{env, path::PathBuf, sync::OnceLock};

use anyhow::{Context, bail};

static CONFIG: OnceLock<HostConfig> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct HostConfig {
    pub device_profile: Option<String>,
    pub timing_enabled: bool,
    pub screen: Option<(usize, usize)>,
    pub screen_parse_error: Option<String>,
    pub gpu_backends: Option<String>,
    pub yield_interval: u64,
    pub window_fps: u32,
    pub game_fps: u32,
    pub control_enabled: bool,
    pub control_port: u16,
    pub screenshot_dir: PathBuf,
    pub screenshot_every_secs: Option<u64>,
    pub diag: bool,
    pub input_diag: bool,
    pub input_debug: bool,
    pub rms_preset: Option<String>,
}

impl Default for HostConfig {
    fn default() -> Self {
        Self {
            device_profile: None,
            timing_enabled: false,
            screen: None,
            screen_parse_error: None,
            gpu_backends: None,
            yield_interval: 64,
            window_fps: 60,
            game_fps: 30,
            control_enabled: true,
            control_port: 17420,
            screenshot_dir: PathBuf::from("target/screenshots"),
            screenshot_every_secs: None,
            diag: false,
            input_diag: false,
            input_debug: false,
            rms_preset: None,
        }
    }
}

impl HostConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Some(device) = env_nonempty("RUSTJAVA_DEVICE") {
            config.device_profile = Some(device);
        } else if let Some(legacy) = env_nonempty("RUSTJAVA_PROFILE") {
            if is_legacy_timing_token(&legacy) {
                config.timing_enabled = true;
            } else {
                config.device_profile = Some(legacy);
            }
        }

        if let Some(timing) = env::var("RUSTJAVA_TIMING").ok().filter(|value| !value.is_empty()) {
            config.timing_enabled = parse_boolish_enabled(&timing);
        } else if env::var_os("RUSTJAVA_TIMING").is_some() {
            config.timing_enabled = true;
        }

        if let Some(value) = env::var_os("RUSTJAVA_SCREEN") {
            let value = value.to_string_lossy();
            match parse_screen_size(&value) {
                Ok(size) => config.screen = Some(size),
                Err(error) => config.screen_parse_error = Some(format!("parse RUSTJAVA_SCREEN={value}: {error:#}")),
            }
        }

        if let Some(gpu) = env_nonempty("RUSTJAVA_GPU") {
            if !gpu.eq_ignore_ascii_case("auto") {
                config.gpu_backends = Some(gpu);
            }
        }

        if let Some(interval) = parse_env_u64("RUSTJAVA_YIELD_INTERVAL").filter(|interval| *interval > 0) {
            config.yield_interval = interval;
        }
        if let Some(fps) = parse_env_u32("RUSTJAVA_WINDOW_FPS").filter(|fps| *fps > 0) {
            config.window_fps = fps;
        }
        if let Some(fps) = parse_env_u32("RUSTJAVA_GAME_FPS").filter(|fps| *fps > 0) {
            config.game_fps = fps;
        }

        if let Ok(control) = env::var("RUSTJAVA_CONTROL") {
            config.control_enabled = parse_boolish_enabled(&control);
        }
        if let Some(port) = env::var("RUSTJAVA_CONTROL_PORT").ok().and_then(|value| value.parse::<u16>().ok()) {
            config.control_port = port;
        }

        if let Some(dir) = env_nonempty("RUSTJAVA_SCREENSHOT_DIR") {
            config.screenshot_dir = PathBuf::from(dir);
        }
        config.screenshot_every_secs = parse_env_u64("RUSTJAVA_SCREENSHOT_EVERY").filter(|seconds| *seconds > 0);

        config.diag = env::var_os("RUSTJAVA_DIAG").is_some();
        config.input_diag = env::var_os("RUSTJAVA_INPUT_DIAG").is_some() || config.diag;
        config.input_debug = env::var_os("RUSTJAVA_INPUT_DEBUG").is_some();
        config.rms_preset = env_nonempty("RUSTJAVA_RMS_PRESET");

        config
    }
}

pub fn init(config: HostConfig) {
    let _ = CONFIG.set(config);
}

pub fn get() -> &'static HostConfig {
    CONFIG.get_or_init(HostConfig::from_env)
}

pub fn parse_screen_size(value: &str) -> anyhow::Result<(usize, usize)> {
    let Some((width, height)) = value.split_once(['x', 'X']) else {
        bail!("expected WIDTHxHEIGHT");
    };

    let width = width.trim().parse::<usize>().context("parse screen width")?;
    let height = height.trim().parse::<usize>().context("parse screen height")?;
    if width < 80 || height < 80 || width > 1000 || height > 1000 {
        bail!("screen size is outside the supported range: {width}x{height}");
    }

    Ok((width, height))
}

fn env_nonempty(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn parse_env_u64(name: &str) -> Option<u64> {
    env::var(name).ok().and_then(|value| value.parse().ok())
}

fn parse_env_u32(name: &str) -> Option<u32> {
    env::var(name).ok().and_then(|value| value.parse().ok())
}

pub(crate) fn parse_boolish_enabled(value: &str) -> bool {
    !matches!(value.trim().to_ascii_lowercase().as_str(), "0" | "false" | "off" | "no")
}

pub(crate) fn is_legacy_timing_token(value: &str) -> bool {
    matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_screen_size_accepts_wxh() {
        assert_eq!(parse_screen_size("240x320").unwrap(), (240, 320));
        assert_eq!(parse_screen_size("176X220").unwrap(), (176, 220));
    }

    #[test]
    fn parse_screen_size_rejects_bad_values() {
        assert!(parse_screen_size("240").is_err());
        assert!(parse_screen_size("10x10").is_err());
    }

    #[test]
    fn legacy_timing_tokens_are_not_device_names() {
        assert!(is_legacy_timing_token("1"));
        assert!(is_legacy_timing_token("true"));
        assert!(!is_legacy_timing_token("Nokia"));
        assert!(!is_legacy_timing_token("SonyEricsson"));
    }

    #[test]
    fn boolish_off_values() {
        assert!(!parse_boolish_enabled("0"));
        assert!(!parse_boolish_enabled("false"));
        assert!(parse_boolish_enabled("1"));
        assert!(parse_boolish_enabled("Nokia"));
    }
}
