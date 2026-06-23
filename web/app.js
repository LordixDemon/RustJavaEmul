import init, { RustJavaWeb } from "./pkg/rust_java.js";

const BUILD_ID = "20260622-v3-math-batch-1";
const canvas = document.getElementById("screen");
const fileInput = document.getElementById("jar");
const fpsNode = document.getElementById("fps");
const statusNode = document.getElementById("status");
const controls = document.getElementById("controls");
const logNode = document.getElementById("log");

let wasmReady = false;
let wasmPromise = null;
let emulator = null;
let emulatorPromise = null;
let frameCount = 0;
let lastFpsTime = performance.now();
let lastDiagnostic = "";
let lastWebDiagnosticTime = performance.now();
let lastFrameLoopTime = performance.now();
let rafDeltaTotal = 0;
let rafDeltaMax = 0;
let presentCalls = 0;
let presentChanged = 0;
let presentUnchanged = 0;
let presentTotalMs = 0;
let presentMaxMs = 0;
let lastPresentMs = 0;
let keyStateEvents = 0;
let keyCallbackEvents = 0;
let keyCallbackPending = 0;
const pressedKeys = new Set();
const activePointers = new Map();
let keyEventChain = Promise.resolve();

function appendLog(message) {
  const now = new Date();
  const time = now.toTimeString().slice(0, 8);
  const line = `[${time}] ${message}`;
  console.log(line);
  logNode.textContent = `${logNode.textContent}${line}\n`.slice(-24000);
  logNode.scrollTop = logNode.scrollHeight;
}

window.__rustjavaLog = appendLog;

function formatError(error) {
  if (!error) {
    return "unknown error";
  }
  if (error.stack) {
    return error.stack;
  }
  if (error.message) {
    return error.message;
  }
  return String(error);
}

function setStatus(text) {
  statusNode.textContent = text;
  statusNode.title = text;
}

function browserRenderInfo() {
  let webgl2 = false;
  try {
    const probe = document.createElement("canvas");
    webgl2 = Boolean(probe.getContext("webgl2"));
  } catch (_) {
    webgl2 = false;
  }
  return `browser secure=${window.isSecureContext} webgpu=${Boolean(navigator.gpu)} webgl2=${webgl2}`;
}

async function ensureEmulator() {
  if (!wasmPromise) {
    setStatus("wasm");
    appendLog("wasm init begin");
    wasmPromise = init({ module_or_path: `./pkg/rust_java_bg.wasm?v=${BUILD_ID}` }).then(() => {
      wasmReady = true;
      appendLog("wasm init complete");
    });
  }
  if (!wasmReady) {
    await wasmPromise;
  }
  if (!emulatorPromise) {
    setStatus("renderer");
    appendLog(browserRenderInfo());
    appendLog("renderer create begin");
    emulatorPromise = RustJavaWeb.create(canvas, 240, 320)
      .then((created) => {
        emulator = created;
        requestAnimationFrame(frameLoop);
        setStatus("ready");
        appendLog("renderer create complete");
        if (typeof created.rendererInfo === "function") {
          appendLog(created.rendererInfo());
        }
        return created;
      })
      .catch((error) => {
        appendLog(`renderer create error\n${formatError(error)}`);
        emulatorPromise = null;
        throw error;
      });
  }
  if (!emulator) {
    await emulatorPromise;
    wasmReady = true;
  }
  return emulator;
}

async function loadJarFile(file) {
  fileInput.disabled = true;
  setStatus("read");
  appendLog(`file selected name=${file.name} size=${file.size}`);
  const bytes = new Uint8Array(await file.arrayBuffer());
  appendLog(`file read complete bytes=${bytes.length}`);
  const emu = await ensureEmulator();
  setStatus("start");
  appendLog("game start begin");
  await emu.loadJar(file.name || "game.jar", bytes);
  setStatus("running");
  appendLog("game start complete");
  fileInput.disabled = false;
}

function frameLoop(time) {
  const rafDelta = time - lastFrameLoopTime;
  lastFrameLoopTime = time;
  if (rafDelta >= 0 && rafDelta < 5000) {
    rafDeltaTotal += rafDelta;
    rafDeltaMax = Math.max(rafDeltaMax, rafDelta);
  }

  if (emulator) {
    try {
      const presentStart = performance.now();
      const changed = emulator.present();
      lastPresentMs = performance.now() - presentStart;
      presentCalls++;
      presentTotalMs += lastPresentMs;
      presentMaxMs = Math.max(presentMaxMs, lastPresentMs);
      if (changed) {
        presentChanged++;
      } else {
        presentUnchanged++;
      }
      if (changed && time - lastFpsTime < 20) {
        appendRenderDiagnostic();
      }
    } catch (error) {
      appendLog(`present error\n${formatError(error)}`);
      setStatus("present error");
    }
  }

  frameCount++;
  const elapsed = time - lastFpsTime;
  if (elapsed >= 500) {
    const fps = Math.round((frameCount * 1000) / elapsed);
    fpsNode.textContent = `${fps} fps`;
    appendRenderDiagnostic();
    appendWebDiagnostic(time, fps);
    frameCount = 0;
    lastFpsTime = time;
  }

  requestAnimationFrame(frameLoop);
}

function appendWebDiagnostic(time, fps) {
  const elapsed = Math.max(1, time - lastWebDiagnosticTime);
  lastWebDiagnosticTime = time;
  const rafAvg = frameCount > 0 ? rafDeltaTotal / frameCount : 0;
  const presentAvg = presentCalls > 0 ? presentTotalMs / presentCalls : 0;
  appendLog(
    `web fps=${fps} rafAvg=${rafAvg.toFixed(1)}ms rafMax=${rafDeltaMax.toFixed(1)}ms ` +
      `present calls=${presentCalls} changed=${presentChanged} empty=${presentUnchanged} ` +
      `avg=${presentAvg.toFixed(2)}ms last=${lastPresentMs.toFixed(2)}ms max=${presentMaxMs.toFixed(2)}ms ` +
      `keys state=${keyStateEvents} cb=${keyCallbackEvents} pending=${keyCallbackPending} window=${elapsed.toFixed(0)}ms`
  );
  rafDeltaTotal = 0;
  rafDeltaMax = 0;
  presentCalls = 0;
  presentChanged = 0;
  presentUnchanged = 0;
  presentTotalMs = 0;
  presentMaxMs = 0;
  keyStateEvents = 0;
  keyCallbackEvents = 0;
}

function sendKey(keyCode, pressed) {
  const emu = emulator;
  if (!emu) {
    return;
  }

  if (typeof emu.setKeyState === "function") {
    emu.setKeyState(keyCode, pressed);
    keyStateEvents++;
  }

  if (isContinuousGameKey(keyCode)) {
    return;
  }

  keyCallbackEvents++;
  keyCallbackPending++;
  keyEventChain = keyEventChain
    .then(() => emu.key(keyCode, pressed))
    .catch((error) => {
      appendLog(`key event error\n${formatError(error)}`);
    })
    .finally(() => {
      keyCallbackPending = Math.max(0, keyCallbackPending - 1);
    });
}

function isContinuousGameKey(keyCode) {
  return keyCode === -1 || keyCode === -2 || keyCode === -3 || keyCode === -4;
}

function keyboardToMidp(event) {
  switch (event.code) {
    case "ArrowUp":
    case "KeyW":
      return -1;
    case "ArrowDown":
    case "KeyS":
      return -2;
    case "ArrowLeft":
    case "KeyA":
      return -3;
    case "ArrowRight":
    case "KeyD":
      return -4;
    case "Enter":
    case "Space":
      return -5;
    case "ShiftLeft":
      return -6;
    case "ShiftRight":
    case "Backspace":
      return -7;
    default:
      if (/^Digit[0-9]$/.test(event.code)) {
        return event.code.charCodeAt(5);
      }
      if (/^Numpad[0-9]$/.test(event.code)) {
        return event.code.charCodeAt(6);
      }
      return null;
  }
}

fileInput.addEventListener("change", () => {
  const file = fileInput.files?.[0];
  if (file) {
    loadJarFile(file).catch((error) => {
      const formatted = formatError(error);
      appendLog(`load error\n${formatted}`);
      setStatus(error?.message || String(error));
      fileInput.disabled = false;
    });
  }
});

function appendRenderDiagnostic() {
  if (!emulator) {
    return;
  }
  if (typeof emulator.diagnostics !== "function") {
    const diagnostic = "diagnostics unavailable: stale js/wasm pair, reload the page";
    if (diagnostic !== lastDiagnostic) {
      lastDiagnostic = diagnostic;
      appendLog(diagnostic);
    }
    return;
  }
  try {
    const diagnostic = emulator.diagnostics();
    if (diagnostic && diagnostic !== lastDiagnostic && diagnostic !== "v3: waiting for render") {
      lastDiagnostic = diagnostic;
      appendLog(diagnostic);
    }
  } catch (error) {
    appendLog(`diagnostics error\n${formatError(error)}`);
  }
}

window.addEventListener("error", (event) => {
  appendLog(`window error\n${event.message}\n${event.filename}:${event.lineno}:${event.colno}`);
  setStatus("js error");
});

window.addEventListener("unhandledrejection", (event) => {
  appendLog(`unhandled rejection\n${formatError(event.reason)}`);
  setStatus("promise error");
});

window.addEventListener("keydown", (event) => {
  const keyCode = keyboardToMidp(event);
  if (keyCode === null || pressedKeys.has(keyCode)) {
    return;
  }
  event.preventDefault();
  pressedKeys.add(keyCode);
  sendKey(keyCode, true);
});

window.addEventListener("keyup", (event) => {
  const keyCode = keyboardToMidp(event);
  if (keyCode === null) {
    return;
  }
  event.preventDefault();
  pressedKeys.delete(keyCode);
  sendKey(keyCode, false);
});

controls.addEventListener("pointerdown", (event) => {
  const button = event.target.closest("button[data-key]");
  if (!button) {
    return;
  }
  event.preventDefault();
  button.setPointerCapture(event.pointerId);
  button.classList.add("active");
  const keyCode = Number(button.dataset.key);
  activePointers.set(event.pointerId, { button, keyCode });
  sendKey(keyCode, true);
});

function releasePointer(event) {
  const active = activePointers.get(event.pointerId);
  if (!active) {
    return;
  }
  event.preventDefault();
  active.button.classList.remove("active");
  activePointers.delete(event.pointerId);
  sendKey(active.keyCode, false);
}

controls.addEventListener("pointerup", releasePointer);
controls.addEventListener("pointercancel", releasePointer);
controls.addEventListener("lostpointercapture", releasePointer);
controls.addEventListener("contextmenu", (event) => event.preventDefault());
controls.addEventListener("pointermove", (event) => event.preventDefault(), { passive: false });

ensureEmulator().catch((error) => {
  appendLog(`initialization error\n${formatError(error)}`);
  setStatus("init error");
});
