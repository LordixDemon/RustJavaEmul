#[cfg(not(target_arch = "wasm32"))]
use std::{
    env, io,
    path::{Path, PathBuf},
    time::Duration,
};

#[cfg(not(target_arch = "wasm32"))]
use anyhow::{Context, bail};

#[cfg(not(target_arch = "wasm32"))]
use rust_java::{HostConfig, StartType, boot_jar_headless, init_config, parse_screen_size, run, run_compat_lab};

#[cfg(not(target_arch = "wasm32"))]
const USAGE: &str = "\
RustJava — Java ME / MIDP emulator

USAGE:
    rust_java -jar <game.jar> [args...]
    rust_java <main-class-path> [args...]
    rust_java --compat-lab [apk-root]
    rust_java --compat-boot <game.jar>

The main-class argument is a filesystem path (stem becomes the class name), not a Java name.

OPTIONS:
    -h, --help              Show this help
    -V, --version           Show version
    -jar, --jar <file>      Run a JAR (may appear anywhere)
    --screen <WxH>          Override virtual LCD size (80-1000)
    --device <name>         Device profile (Nokia, SonyEricsson, …)
    --timing                Enable CPU timing profiler

ENVIRONMENT:
    RUST_LOG                tracing-subscriber filter (default warn)
    RUSTJAVA_DEVICE         Device profile (preferred over RUSTJAVA_PROFILE)
    RUSTJAVA_TIMING         CPU profiler (1/true/on)
    RUSTJAVA_PROFILE        Legacy: vendor name, or 1/true to enable timing
    RUSTJAVA_SCREEN         WIDTHxHEIGHT
    RUSTJAVA_GPU            wgpu backends (vulkan, dx12, metal, gl, webgpu)
    WGPU_BACKEND            wgpu backend (wins over RUSTJAVA_GPU)
    RUSTJAVA_CONTROL        0/false/off/no disables the control HTTP port
    RUSTJAVA_CONTROL_PORT   default 17420
    RUSTJAVA_SCREENSHOT_DIR default target/screenshots
    RUSTJAVA_SCREENSHOT_EVERY  dump LCD every N seconds
    RUSTJAVA_WINDOW_FPS     presenter cap (default 60)
    RUSTJAVA_GAME_FPS       MIDlet frame rate (default 30)
    RUSTJAVA_YIELD_INTERVAL yield every N ticks (default 64)
    RUSTJAVA_RMS_PRESET     name=hex,hex;...
    RUSTJAVA_DIAG           window/GPU diagnostics
    RUSTJAVA_INPUT_DIAG     key dispatch logs
    RUSTJAVA_INPUT_DEBUG    Treasure Towers field dump
";

#[cfg(not(target_arch = "wasm32"))]
struct Opts {
    jar: Option<PathBuf>,
    main_class: Option<PathBuf>,
    args: Vec<String>,
    compat_root: Option<PathBuf>,
    compat_boot: Option<PathBuf>,
    screen: Option<String>,
    device: Option<String>,
    timing: bool,
}

#[cfg(not(target_arch = "wasm32"))]
enum Command {
    Help,
    Version,
    Run(Box<Opts>),
}

#[cfg(not(target_arch = "wasm32"))]
fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).with_writer(std::io::stderr).try_init();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> anyhow::Result<()> {
    init_tracing();

    match parse_args()? {
        Command::Help => {
            print!("{USAGE}");
            Ok(())
        }
        Command::Version => {
            println!("rust_java {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Command::Run(opts) => {
            let opts = *opts;
            let mut config = HostConfig::from_env();
            if let Some(screen) = &opts.screen {
                config.screen = Some(parse_screen_size(screen).with_context(|| format!("parse --screen {screen}"))?);
                config.screen_parse_error = None;
            }
            if let Some(device) = &opts.device {
                config.device_profile = Some(device.clone());
            }
            if opts.timing {
                config.timing_enabled = true;
            }
            init_config(config);

            std::thread::Builder::new()
                .name("rustjava-main".to_string())
                .stack_size(16 * 1024 * 1024)
                .spawn(move || {
                    let runtime = tokio::runtime::Runtime::new().context("failed to start async runtime")?;
                    runtime.block_on(async_main(opts))
                })?
                .join()
                .unwrap_or_else(|e| std::panic::resume_unwind(e))
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn main() {}

#[cfg(not(target_arch = "wasm32"))]
async fn async_main(opts: Opts) -> anyhow::Result<()> {
    if let Some(root) = opts.compat_root {
        run_compat_lab(&root).await?;
        return Ok(());
    }
    if let Some(jar) = opts.compat_boot {
        ensure_jar_file(&jar)?;
        let (_, status) = boot_jar_headless(&jar, Duration::from_secs(8)).await;
        println!("{status}");
        if status == "ok" || status == "running" {
            return Ok(());
        }
        bail!("{status}");
    }

    let start_type = if let Some(main_class) = &opts.main_class {
        StartType::Class(main_class)
    } else {
        let jar = opts.jar.as_ref().context("internal error: missing -jar path")?;
        ensure_jar_file(jar)?;
        StartType::Jar(jar)
    };

    run(io::stdout(), start_type, &opts.args, &[Path::new(".")]).await
}

#[cfg(not(target_arch = "wasm32"))]
fn ensure_jar_file(path: &Path) -> anyhow::Result<()> {
    if path.is_file() {
        Ok(())
    } else {
        bail!("JAR not found: {}", path.display());
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_args() -> anyhow::Result<Command> {
    let mut args = env::args().skip(1).peekable();
    if args.peek().is_none() {
        bail!("No class or -jar specified\n\n{USAGE}");
    }

    let mut jar = None;
    let mut main_class = None;
    let mut rest_args = Vec::new();
    let mut compat_root = None;
    let mut compat_boot = None;
    let mut screen = None;
    let mut device = None;
    let mut timing = false;
    let mut positional = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(Command::Help),
            "-V" | "--version" => return Ok(Command::Version),
            "--compat-lab" => {
                let root = args.next().filter(|value| !value.starts_with('-')).unwrap_or_else(|| "apk".to_string());
                compat_root = Some(root.into());
            }
            "--compat-boot" => {
                let jar_path = args.next().ok_or_else(|| anyhow::anyhow!("Missing jar file after --compat-boot"))?;
                compat_boot = Some(jar_path.into());
            }
            "-jar" | "--jar" => {
                let Some(jar_path) = args.next() else {
                    bail!("Missing jar file after -jar");
                };
                jar = Some(jar_path.into());
            }
            "--screen" => {
                let Some(value) = args.next() else {
                    bail!("Missing value after --screen");
                };
                screen = Some(value);
            }
            "--device" => {
                let Some(value) = args.next() else {
                    bail!("Missing value after --device");
                };
                device = Some(value);
            }
            "--timing" => timing = true,
            "--" => {
                rest_args.extend(args);
                break;
            }
            _ if arg.starts_with('-') => bail!("Unknown option: {arg}\n\n{USAGE}"),
            _ => positional.push(arg),
        }
    }

    for item in positional {
        if jar.is_none() && compat_boot.is_none() && item.ends_with(".jar") {
            jar = Some(item.into());
        } else if jar.is_none() && main_class.is_none() && compat_root.is_none() && compat_boot.is_none() {
            main_class = Some(item.into());
        } else {
            rest_args.push(item);
        }
    }

    if jar.is_none() && main_class.is_none() && compat_root.is_none() && compat_boot.is_none() {
        bail!("No class or -jar specified\n\n{USAGE}");
    }

    Ok(Command::Run(Box::new(Opts {
        jar,
        main_class,
        args: rest_args,
        compat_root,
        compat_boot,
        screen,
        device,
        timing,
    })))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    #[test]
    fn usage_mentions_device_and_timing() {
        assert!(super::USAGE.contains("RUSTJAVA_DEVICE"));
        assert!(super::USAGE.contains("RUSTJAVA_TIMING"));
        assert!(super::USAGE.contains("--screen"));
    }
}
