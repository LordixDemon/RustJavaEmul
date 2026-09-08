use std::{
    collections::VecDeque,
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    sync::{
        Arc,
        mpsc::{self, RecvTimeoutError, SyncSender},
    },
    thread,
    time::Duration,
};

use parking_lot::Mutex;

use super::screenshot::encode_lcd_png;

const URL_FILE: &str = "target/emu_control.url";

#[derive(Clone)]
pub(super) struct ControlHandle {
    commands: Arc<Mutex<VecDeque<ControlCommand>>>,
}

pub(super) enum ControlCommand {
    Down(i32),
    Up(i32),
    Tap { code: i32, hold_frames: u32 },
    Screenshot { reply: SyncSender<Result<Vec<u8>, String>> },
    Status { reply: SyncSender<String> },
}

impl ControlHandle {
    pub(super) fn new() -> Self {
        Self {
            commands: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub(super) fn take(&self) -> Vec<ControlCommand> {
        self.commands.lock().drain(..).collect()
    }

    fn push(&self, command: ControlCommand) {
        self.commands.lock().push_back(command);
    }
}

pub(super) fn spawn(handle: ControlHandle) -> Option<SocketAddr> {
    let config = crate::config::get();
    if !config.control_enabled {
        eprintln!("control disabled (RUSTJAVA_CONTROL=0)");
        return None;
    }

    let preferred = config.control_port;

    let mut last_error = None;
    for port in preferred..preferred.saturating_add(16) {
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(listener) => {
                let addr = listener.local_addr().ok()?;
                listener.set_nonblocking(false).ok()?;
                write_url_file(addr);
                eprintln!("control {}", format_url(addr));
                thread::Builder::new()
                    .name("rustjava-control".into())
                    .spawn(move || accept_loop(listener, handle))
                    .ok()?;
                return Some(addr);
            }
            Err(error) => last_error = Some(error),
        }
    }

    if let Some(error) = last_error {
        eprintln!("control bind failed: {error}");
    }
    None
}

fn format_url(addr: SocketAddr) -> String {
    format!("http://{addr}")
}

fn write_url_file(addr: SocketAddr) {
    let path = PathBuf::from(URL_FILE);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, format!("{}\n", format_url(addr)));
}

fn accept_loop(listener: TcpListener, handle: ControlHandle) {
    loop {
        let Ok((stream, _)) = listener.accept() else {
            continue;
        };
        let handle = handle.clone();
        thread::spawn(move || {
            if let Err(error) = handle_client(stream, &handle) {
                tracing::debug!("control request failed: {error:#}");
            }
        });
    }
}

fn handle_client(stream: TcpStream, handle: &ControlHandle) -> anyhow::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;
    let mut stream = stream;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let mut content_length = 0usize;
    loop {
        let mut header = String::new();
        reader.read_line(&mut header)?;
        if header.trim().is_empty() {
            break;
        }
        if let Some(value) = header.strip_prefix("Content-Length:") {
            content_length = value.trim().parse().unwrap_or(0);
        }
    }
    if content_length > 0 {
        let mut body = vec![0; content_length.min(4096)];
        reader.read_exact(&mut body)?;
    }

    let mut parts = request_line.split_whitespace();
    let _method = parts.next().unwrap_or("GET");
    let target = parts.next().unwrap_or("/");
    let (path, query) = split_target(target);
    let response = dispatch(path, query, handle);
    stream.write_all(&response)?;
    Ok(())
}

fn split_target(target: &str) -> (&str, &str) {
    match target.split_once('?') {
        Some((path, query)) => (path, query),
        None => (target, ""),
    }
}

fn dispatch(path: &str, query: &str, handle: &ControlHandle) -> Vec<u8> {
    match path {
        "/" => text_response(200, "text/html; charset=utf-8", WEB_UI_HTML),
        "/help" => text_response(200, "text/plain; charset=utf-8", HELP),
        "/health" => text_response(200, "text/plain; charset=utf-8", "ok\n"),
        "/status" => match request_status(handle) {
            Ok(body) => text_response(200, "text/plain; charset=utf-8", &body),
            Err(error) => text_response(503, "text/plain; charset=utf-8", &format!("{error}\n")),
        },
        "/screenshot" => match request_screenshot(handle) {
            Ok(png) => binary_response(200, "image/png", &png),
            Err(error) => text_response(503, "text/plain; charset=utf-8", &format!("{error}\n")),
        },
        "/down" | "/press" => key_command(query, handle, KeyAction::Down),
        "/up" | "/release" => key_command(query, handle, KeyAction::Up),
        "/tap" | "/click" => key_command(query, handle, KeyAction::Tap),
        "/keys" => text_response(200, "text/plain; charset=utf-8", KEY_HELP),
        _ => text_response(404, "text/plain; charset=utf-8", "not found\n"),
    }
}

enum KeyAction {
    Down,
    Up,
    Tap,
}

fn key_command(query: &str, handle: &ControlHandle, action: KeyAction) -> Vec<u8> {
    let params = Query::parse(query);
    let Some(code) = params.key_code() else {
        return text_response(400, "text/plain; charset=utf-8", "missing key= or code=\n");
    };
    match action {
        KeyAction::Down => handle.push(ControlCommand::Down(code)),
        KeyAction::Up => handle.push(ControlCommand::Up(code)),
        KeyAction::Tap => handle.push(ControlCommand::Tap {
            code,
            hold_frames: params.hold_frames(),
        }),
    }
    text_response(200, "text/plain; charset=utf-8", &format!("ok key={code}\n"))
}

fn request_screenshot(handle: &ControlHandle) -> Result<Vec<u8>, String> {
    let (reply, wait) = mpsc::sync_channel(1);
    handle.push(ControlCommand::Screenshot { reply });
    wait_reply(wait)?
}

fn request_status(handle: &ControlHandle) -> Result<String, String> {
    let (reply, wait) = mpsc::sync_channel(1);
    handle.push(ControlCommand::Status { reply });
    wait_reply(wait)
}

fn wait_reply<T>(wait: mpsc::Receiver<T>) -> Result<T, String> {
    match wait.recv_timeout(Duration::from_secs(3)) {
        Ok(value) => Ok(value),
        Err(RecvTimeoutError::Timeout) => Err("emulator did not answer in time".into()),
        Err(RecvTimeoutError::Disconnected) => Err("emulator control queue closed".into()),
    }
}

pub(super) fn encode_game_png(width: usize, height: usize, stride: usize, pixels: &[u32]) -> Result<Vec<u8>, String> {
    encode_lcd_png(width, height, stride, pixels).map_err(|error| error.to_string())
}

struct Query<'a> {
    raw: &'a str,
}

impl<'a> Query<'a> {
    fn parse(raw: &'a str) -> Self {
        Self { raw }
    }

    fn get(&self, name: &str) -> Option<&'a str> {
        self.raw.split('&').find_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            (key == name).then_some(value)
        })
    }

    fn key_code(&self) -> Option<i32> {
        if let Some(code) = self.get("code").and_then(|value| value.parse().ok()) {
            return Some(code);
        }
        parse_key(self.get("key").or_else(|| self.get("name")).unwrap_or(""))
    }

    fn hold_frames(&self) -> u32 {
        self.get("hold").and_then(|value| value.parse().ok()).unwrap_or(3).clamp(1, 30)
    }
}

pub(super) fn parse_key(name: &str) -> Option<i32> {
    let name = name.trim().to_ascii_lowercase();
    match name.as_str() {
        "ok" | "fire" | "select" | "enter" | "center" | "softcenter" => Some(-5),
        "up" => Some(-1),
        "down" => Some(-2),
        "left" => Some(-3),
        "right" => Some(-4),
        "l" | "lsk" | "lsoft" | "leftsoft" | "soft1" => Some(-6),
        "r" | "rsk" | "rsoft" | "rightsoft" | "soft2" => Some(-7),
        "star" | "*" => Some(b'*' as i32),
        "pound" | "hash" | "#" => Some(b'#' as i32),
        digit if digit.len() == 1 && digit.as_bytes()[0].is_ascii_digit() => Some(digit.as_bytes()[0] as i32),
        _ => None,
    }
}

fn text_response(status: u16, content_type: &str, body: &str) -> Vec<u8> {
    binary_response(status, content_type, body.as_bytes())
}

fn binary_response(status: u16, content_type: &str, body: &[u8]) -> Vec<u8> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        503 => "Service Unavailable",
        _ => "OK",
    };
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let mut out = header.into_bytes();
    out.extend_from_slice(body);
    out
}

const HELP: &str = "\
RustJava emulator control (127.0.0.1 only)

GET  /health
GET  /status
GET  /screenshot
GET  /keys
GET  /tap?key=ok
GET  /down?key=up
GET  /up?key=up

keys: ok/fire, up, down, left, right, lsoft, rsoft, 0-9
";

const KEY_HELP: &str = "\
ok,fire,select,enter,center = -5
up = -1
down = -2
left = -3
right = -4
lsoft,lsk,l = -6
rsoft,rsk,r = -7
0-9 = keypad
";

const WEB_UI_HTML: &str = r##"<!DOCTYPE html>
<html lang="ru">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>RustJava Emulator Remote</title>
<style>
  :root {
    --bg-main: #0b0f19;
    --bg-card: #151b2b;
    --bg-btn: #1f273d;
    --bg-btn-hover: #2d3856;
    --bg-btn-active: #3b82f6;
    --accent: #38bdf8;
    --text: #f1f5f9;
    --text-dim: #94a3b8;
    --border: #2e3852;
  }
  * { box-sizing: border-box; margin: 0; padding: 0; user-select: none; }
  body {
    background: var(--bg-main);
    color: var(--text);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    display: flex;
    flex-direction: column;
    align-items: center;
    min-height: 100vh;
    padding: 16px;
  }
  header {
    margin-bottom: 12px;
    text-align: center;
  }
  header h1 {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--accent);
    letter-spacing: 0.5px;
    display: flex;
    align-items: center;
    gap: 8px;
    justify-content: center;
  }
  .status-badge {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 0.75rem;
    background: rgba(56, 189, 248, 0.1);
    color: var(--accent);
    padding: 4px 10px;
    border-radius: 9999px;
    margin-top: 6px;
    border: 1px solid rgba(56, 189, 248, 0.2);
  }
  .status-dot {
    width: 8px;
    height: 8px;
    background: #22c55e;
    border-radius: 50%;
    animation: pulse 2s infinite;
  }
  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.4; transform: scale(0.85); }
  }
  .phone-container {
    background: var(--bg-card);
    border: 2px solid var(--border);
    border-radius: 28px;
    padding: 16px;
    box-shadow: 0 20px 40px rgba(0,0,0,0.6);
    display: flex;
    flex-direction: column;
    align-items: center;
    max-width: 360px;
    width: 100%;
  }
  .screen-bezel {
    background: #000;
    border: 3px solid #111;
    border-radius: 12px;
    overflow: hidden;
    display: flex;
    justify-content: center;
    align-items: center;
    box-shadow: inset 0 0 10px rgba(0,0,0,0.8);
    position: relative;
    width: 240px;
    height: 320px;
  }
  #screen {
    width: 100%;
    height: 100%;
    object-fit: contain;
    image-rendering: pixelated;
    image-rendering: crisp-edges;
    display: block;
  }
  .controls {
    width: 100%;
    margin-top: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .softkeys {
    display: flex;
    justify-content: space-between;
    gap: 12px;
  }
  .btn {
    background: var(--bg-btn);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 10px;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.1s ease;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    touch-action: manipulation;
  }
  .btn:hover { background: var(--bg-btn-hover); }
  .btn:active, .btn.pressed {
    background: var(--bg-btn-active);
    color: #fff;
    transform: scale(0.96);
    box-shadow: 0 0 12px rgba(59, 130, 246, 0.5);
  }
  .btn-hint { font-size: 0.65rem; color: var(--text-dim); margin-top: 2px; }
  .btn:active .btn-hint, .btn.pressed .btn-hint { color: rgba(255,255,255,0.8); }
  .btn-soft { flex: 1; }
  .dpad-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-gap: 6px;
    width: 180px;
    margin: 0 auto;
  }
  .dpad-grid .btn { height: 48px; }
  .numpad-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-gap: 6px;
    width: 100%;
  }
  .numpad-grid .btn { height: 38px; font-size: 0.85rem; }
  .keyboard-hints {
    margin-top: 14px;
    font-size: 0.72rem;
    color: var(--text-dim);
    text-align: center;
    line-height: 1.5;
  }
  .keyboard-hints kbd {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 1px 5px;
    color: var(--accent);
  }
</style>
</head>
<body>
<header>
  <h1><span>🎮</span> RustJava Web Remote</h1>
  <div class="status-badge"><span class="status-dot"></span> <span id="fps-label">Подключено (127.0.0.1:17420)</span></div>
</header>

<div class="phone-container">
  <div class="screen-bezel">
    <img id="screen" src="/screenshot" alt="LCD Screen">
  </div>

  <div class="controls">
    <div class="softkeys">
      <button class="btn btn-soft" data-key="lsoft"><span>LSoft</span><span class="btn-hint">Q / F1</span></button>
      <button class="btn btn-soft" data-key="rsoft"><span>RSoft</span><span class="btn-hint">E / F2</span></button>
    </div>

    <div class="dpad-grid">
      <div></div>
      <button class="btn" data-key="up"><span>▲</span><span class="btn-hint">Up</span></button>
      <div></div>
      <button class="btn" data-key="left"><span>◀</span><span class="btn-hint">Left</span></button>
      <button class="btn" data-key="fire" style="background: #2563eb; color: #fff;"><span>OK</span><span class="btn-hint">Enter</span></button>
      <button class="btn" data-key="right"><span>▶</span><span class="btn-hint">Right</span></button>
      <div></div>
      <button class="btn" data-key="down"><span>▼</span><span class="btn-hint">Down</span></button>
      <div></div>
    </div>

    <div class="numpad-grid">
      <button class="btn" data-key="1">1</button>
      <button class="btn" data-key="2">2</button>
      <button class="btn" data-key="3">3</button>
      <button class="btn" data-key="4">4</button>
      <button class="btn" data-key="5">5</button>
      <button class="btn" data-key="6">6</button>
      <button class="btn" data-key="7">7</button>
      <button class="btn" data-key="8">8</button>
      <button class="btn" data-key="9">9</button>
      <button class="btn" data-key="*">*</button>
      <button class="btn" data-key="0">0</button>
      <button class="btn" data-key="#">#</button>
    </div>
  </div>
</div>

<div class="keyboard-hints">
  Клавиатура: <kbd>Стрелки</kbd>/<kbd>WASD</kbd> — ход, <kbd>Enter</kbd>/<kbd>Space</kbd> — огонь, <kbd>Q</kbd> — левая софт, <kbd>E</kbd> — правая софт, <kbd>0-9</kbd> — клавиши. Профиль и коды клавиш берутся из JAR.
</div>

<script>
  const screenImg = document.getElementById('screen');
  const fpsLabel = document.getElementById('fps-label');
  let fetching = false;
  let frameCount = 0;
  let lastFpsTime = performance.now();

  async function updateScreen() {
    if (fetching) return;
    fetching = true;
    try {
      const resp = await fetch('/screenshot?t=' + Date.now(), { cache: 'no-store' });
      if (resp.ok) {
        const blob = await resp.blob();
        const oldSrc = screenImg.src;
        screenImg.src = URL.createObjectURL(blob);
        if (oldSrc.startsWith('blob:')) URL.revokeObjectURL(oldSrc);
        frameCount++;
        const now = performance.now();
        if (now - lastFpsTime >= 1000) {
          const fps = Math.round((frameCount * 1000) / (now - lastFpsTime));
          fpsLabel.innerText = 'В сети • ' + fps + ' FPS';
          frameCount = 0;
          lastFpsTime = now;
        }
      }
    } catch (e) {
      fpsLabel.innerText = 'Ожидание эмулятора...';
    } finally {
      fetching = false;
    }
  }
  setInterval(updateScreen, 80);

  function sendKey(key, action = 'tap') {
    fetch('/' + action + '?key=' + encodeURIComponent(key), { method: 'GET' }).catch(() => {});
  }

  document.querySelectorAll('button[data-key]').forEach(btn => {
    const key = btn.getAttribute('data-key');
    btn.addEventListener('mousedown', (e) => { e.preventDefault(); sendKey(key, 'down'); btn.classList.add('pressed'); });
    btn.addEventListener('mouseup', (e) => { e.preventDefault(); sendKey(key, 'up'); btn.classList.remove('pressed'); });
    btn.addEventListener('mouseleave', () => { if (btn.classList.contains('pressed')) { sendKey(key, 'up'); btn.classList.remove('pressed'); } });
    btn.addEventListener('touchstart', (e) => { e.preventDefault(); sendKey(key, 'down'); btn.classList.add('pressed'); });
    btn.addEventListener('touchend', (e) => { e.preventDefault(); sendKey(key, 'up'); btn.classList.remove('pressed'); });
  });

  const keyMap = {
    'ArrowUp': 'up', 'ArrowDown': 'down', 'ArrowLeft': 'left', 'ArrowRight': 'right',
    'Enter': 'fire', ' ': 'fire',
    'KeyQ': 'lsoft', 'F1': 'lsoft',
    'KeyE': 'rsoft', 'F2': 'rsoft', 'Escape': 'rsoft', 'Backspace': 'rsoft',
    'Digit0': '0', 'Digit1': '1', 'Digit2': '2', 'Digit3': '3', 'Digit4': '4',
    'Digit5': '5', 'Digit6': '6', 'Digit7': '7', 'Digit8': '8', 'Digit9': '9',
    'Numpad0': '0', 'Numpad1': '1', 'Numpad2': '2', 'Numpad3': '3', 'Numpad4': '4',
    'Numpad5': '5', 'Numpad6': '6', 'Numpad7': '7', 'Numpad8': '8', 'Numpad9': '9'
  };

  window.addEventListener('keydown', (e) => {
    const mapped = keyMap[e.code] || keyMap[e.key];
    if (mapped) {
      e.preventDefault();
      sendKey(mapped, 'down');
      const b = document.querySelector('button[data-key="' + mapped + '"]');
      if (b) b.classList.add('pressed');
    }
  });

  window.addEventListener('keyup', (e) => {
    const mapped = keyMap[e.code] || keyMap[e.key];
    if (mapped) {
      e.preventDefault();
      sendKey(mapped, 'up');
      const b = document.querySelector('button[data-key="' + mapped + '"]');
      if (b) b.classList.remove('pressed');
    }
  });
</script>
</body>
</html>
"##;
