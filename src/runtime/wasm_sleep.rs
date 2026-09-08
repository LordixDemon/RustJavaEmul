#![cfg_attr(not(all(target_arch = "wasm32", feature = "browser-window")), allow(dead_code, unused_imports))]

use alloc::{format, sync::Arc};
use core::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use parking_lot::Mutex;

use crate::profile;

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static JVM_PROFILE_LAST: Mutex<Option<jvm_rust::ProfileSnapshot>> = Mutex::new(None);

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_CALLS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_REQUESTED_MS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_ACTUAL_MS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_ACTUAL_US: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_LAST_REQUESTED_MS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_LAST_ACTUAL_US: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_MAX_ACTUAL_US: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_YIELD_CALLS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_SHORT_CALLS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_TIMER_CALLS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_ACTUAL_LE_1MS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_ACTUAL_1_4MS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_ACTUAL_4_8MS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_ACTUAL_8_16MS: AtomicU64 = AtomicU64::new(0);
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
static BROWSER_SLEEP_ACTUAL_OVER_16MS: AtomicU64 = AtomicU64::new(0);

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
const BROWSER_FAST_SLEEP_THRESHOLD_MS: u64 = 5;

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(crate) fn browser_timer_diagnostics() -> String {
    let calls = BROWSER_SLEEP_CALLS.load(Ordering::Relaxed);
    let requested = BROWSER_SLEEP_REQUESTED_MS.load(Ordering::Relaxed);
    let actual = BROWSER_SLEEP_ACTUAL_MS.load(Ordering::Relaxed);
    let actual_us = BROWSER_SLEEP_ACTUAL_US.load(Ordering::Relaxed);
    let avg_actual = if calls == 0 { 0.0 } else { actual_us as f64 / calls as f64 / 1000.0 };
    format!(
        "wasmSleep calls={} yield={} short={} timer={} reqTotal={}ms actualTotal={}ms actualAvg={:.2}ms reqLast={}ms actualLast={:.2}ms actualMax={:.2}ms actualBuckets <=1/1-4/4-8/8-16/>16={}/{}/{}/{}/{}",
        calls,
        BROWSER_SLEEP_YIELD_CALLS.load(Ordering::Relaxed),
        BROWSER_SLEEP_SHORT_CALLS.load(Ordering::Relaxed),
        BROWSER_SLEEP_TIMER_CALLS.load(Ordering::Relaxed),
        requested,
        actual,
        avg_actual,
        BROWSER_SLEEP_LAST_REQUESTED_MS.load(Ordering::Relaxed),
        BROWSER_SLEEP_LAST_ACTUAL_US.load(Ordering::Relaxed) as f64 / 1000.0,
        BROWSER_SLEEP_MAX_ACTUAL_US.load(Ordering::Relaxed) as f64 / 1000.0,
        BROWSER_SLEEP_ACTUAL_LE_1MS.load(Ordering::Relaxed),
        BROWSER_SLEEP_ACTUAL_1_4MS.load(Ordering::Relaxed),
        BROWSER_SLEEP_ACTUAL_4_8MS.load(Ordering::Relaxed),
        BROWSER_SLEEP_ACTUAL_8_16MS.load(Ordering::Relaxed),
        BROWSER_SLEEP_ACTUAL_OVER_16MS.load(Ordering::Relaxed)
    )
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(crate) fn jvm_profile_diagnostics() -> String {
    if !profile::enabled() {
        return format!(
            "jvm profile=off\n{}\n{}",
            jvm_rust::method_opcode_report_and_reset(8),
            jvm_rust::intrinsic_report_and_reset()
        );
    }

    let current = jvm_rust::profile_snapshot();
    let mut last = JVM_PROFILE_LAST.lock();
    let previous = last.replace(current).unwrap_or(current);
    let opcodes = current.opcodes.saturating_sub(previous.opcodes);
    let fast = current.fast_opcodes.saturating_sub(previous.fast_opcodes);
    let slow = current.slow_opcodes.saturating_sub(previous.slow_opcodes);
    let methods = current.interpreter_runs.saturating_sub(previous.interpreter_runs);
    let jumps = current.jumps.saturating_sub(previous.jumps);
    let returns = current.returns.saturating_sub(previous.returns);
    let exceptions = current.java_exceptions.saturating_sub(previous.java_exceptions);
    let invoke_virtual = current.invoke_virtual.saturating_sub(previous.invoke_virtual);
    let invoke_special = current.invoke_special.saturating_sub(previous.invoke_special);
    let invoke_static = current.invoke_static.saturating_sub(previous.invoke_static);
    let invoke_interface = current.invoke_interface.saturating_sub(previous.invoke_interface);
    let array_new = current.array_new.saturating_sub(previous.array_new);
    let array_load_one = current.array_load_one.saturating_sub(previous.array_load_one);
    let array_store_one = current.array_store_one.saturating_sub(previous.array_store_one);
    let array_load_bulk = current.array_load_bulk.saturating_sub(previous.array_load_bulk);
    let array_store_bulk = current.array_store_bulk.saturating_sub(previous.array_store_bulk);
    let array_copy = current.array_copy.saturating_sub(previous.array_copy);
    let raw_read = current.array_raw_read.saturating_sub(previous.array_raw_read);
    let raw_write = current.array_raw_write.saturating_sub(previous.array_raw_write);
    format!(
        "jvm delta opcodes={opcodes} fast={fast} slow={slow} methods={methods} jumps={jumps} returns={returns} exceptions={exceptions} invoke v/s/st/i={invoke_virtual}/{invoke_special}/{invoke_static}/{invoke_interface} arrays new={array_new} load1={array_load_one} store1={array_store_one} loadBulk={array_load_bulk} storeBulk={array_store_bulk} copy={array_copy} raw r/w={raw_read}/{raw_write}"
    )
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(crate) fn reset_jvm_profile_diagnostics() {
    jvm_rust::reset_profile();
    *JVM_PROFILE_LAST.lock() = Some(jvm_rust::profile_snapshot());
}

#[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
#[allow(dead_code)]
pub(crate) fn browser_timer_diagnostics() -> String {
    "wasmSleep unavailable".to_string()
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(super) async fn browser_sleep(duration: Duration) {
    let requested_ms = duration.as_millis().min(u64::MAX as u128) as u64;
    let start_ms = browser_performance_now_ms();
    if requested_ms == 0 {
        BROWSER_SLEEP_YIELD_CALLS.fetch_add(1, Ordering::Relaxed);
        BrowserMicroYieldFuture::new().await;
    } else if requested_ms <= BROWSER_FAST_SLEEP_THRESHOLD_MS {
        BROWSER_SLEEP_SHORT_CALLS.fetch_add(1, Ordering::Relaxed);
        BrowserTimerFuture::new(Duration::ZERO).await;
    } else {
        BROWSER_SLEEP_TIMER_CALLS.fetch_add(1, Ordering::Relaxed);
        BrowserTimerFuture::new(duration).await;
    }
    let actual_us = ((browser_performance_now_ms() - start_ms).max(0.0) * 1000.0).round() as u64;
    let actual_ms = actual_us / 1000;
    BROWSER_SLEEP_CALLS.fetch_add(1, Ordering::Relaxed);
    BROWSER_SLEEP_REQUESTED_MS.fetch_add(requested_ms, Ordering::Relaxed);
    BROWSER_SLEEP_ACTUAL_MS.fetch_add(actual_ms, Ordering::Relaxed);
    BROWSER_SLEEP_ACTUAL_US.fetch_add(actual_us, Ordering::Relaxed);
    BROWSER_SLEEP_LAST_REQUESTED_MS.store(requested_ms, Ordering::Relaxed);
    BROWSER_SLEEP_LAST_ACTUAL_US.store(actual_us, Ordering::Relaxed);
    BROWSER_SLEEP_MAX_ACTUAL_US.fetch_max(actual_us, Ordering::Relaxed);
    record_browser_sleep_bucket(actual_us);
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(super) async fn browser_yield() {
    BrowserMicroYieldFuture::new().await;
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
fn browser_performance_now_ms() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map(|performance| performance.now())
        .unwrap_or_else(js_sys::Date::now)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
fn record_browser_sleep_bucket(actual_us: u64) {
    let bucket = if actual_us <= 1_000 {
        &BROWSER_SLEEP_ACTUAL_LE_1MS
    } else if actual_us <= 4_000 {
        &BROWSER_SLEEP_ACTUAL_1_4MS
    } else if actual_us <= 8_000 {
        &BROWSER_SLEEP_ACTUAL_4_8MS
    } else if actual_us <= 16_000 {
        &BROWSER_SLEEP_ACTUAL_8_16MS
    } else {
        &BROWSER_SLEEP_ACTUAL_OVER_16MS
    };
    bucket.fetch_add(1, Ordering::Relaxed);
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r#"
export function rustjava_browser_micro_yield(callback) {
    const channel = new MessageChannel();
    channel.port1.onmessage = () => {
        channel.port1.close();
        channel.port2.close();
        callback();
    };
    channel.port2.postMessage(0);
}
"#)]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(catch)]
    fn rustjava_browser_micro_yield(callback: &js_sys::Function) -> Result<(), wasm_bindgen::JsValue>;
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
struct BrowserMicroYieldFuture {
    state: Arc<BrowserTimerState>,
    scheduled: bool,
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
impl BrowserMicroYieldFuture {
    fn new() -> Self {
        Self {
            state: Arc::new(BrowserTimerState {
                done: core::sync::atomic::AtomicBool::new(false),
                waker: Mutex::new(None),
            }),
            scheduled: false,
        }
    }

    fn schedule(&self) {
        use wasm_bindgen::{JsCast, closure::Closure};

        let state = self.state.clone();
        let callback = Closure::once_into_js(move || {
            state.done.store(true, Ordering::SeqCst);
            if let Some(waker) = state.waker.lock().take() {
                waker.wake();
            }
        });

        if rustjava_browser_micro_yield(callback.unchecked_ref()).is_err() {
            self.state.done.store(true, Ordering::SeqCst);
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
impl core::future::Future for BrowserMicroYieldFuture {
    type Output = ();

    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        if self.state.done.load(Ordering::SeqCst) {
            return core::task::Poll::Ready(());
        }

        *self.state.waker.lock() = Some(cx.waker().clone());
        if !self.scheduled {
            self.scheduled = true;
            self.schedule();
        }

        if self.state.done.load(Ordering::SeqCst) {
            core::task::Poll::Ready(())
        } else {
            core::task::Poll::Pending
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
struct BrowserTimerState {
    done: core::sync::atomic::AtomicBool,
    waker: Mutex<Option<core::task::Waker>>,
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
struct BrowserTimerFuture {
    state: Arc<BrowserTimerState>,
    timeout_ms: i32,
    scheduled: bool,
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
impl BrowserTimerFuture {
    fn new(duration: Duration) -> Self {
        Self {
            state: Arc::new(BrowserTimerState {
                done: core::sync::atomic::AtomicBool::new(false),
                waker: Mutex::new(None),
            }),
            timeout_ms: duration.as_millis().min(i32::MAX as u128) as i32,
            scheduled: false,
        }
    }

    fn schedule(&self) {
        use wasm_bindgen::{JsCast, closure::Closure};

        let state = self.state.clone();
        let callback = Closure::once_into_js(move || {
            state.done.store(true, Ordering::SeqCst);
            if let Some(waker) = state.waker.lock().take() {
                waker.wake();
            }
        });

        if let Some(window) = web_sys::window() {
            if window
                .set_timeout_with_callback_and_timeout_and_arguments_0(callback.unchecked_ref(), self.timeout_ms)
                .is_err()
            {
                self.state.done.store(true, Ordering::SeqCst);
            }
        } else {
            self.state.done.store(true, Ordering::SeqCst);
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
impl core::future::Future for BrowserTimerFuture {
    type Output = ();

    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        if self.state.done.load(Ordering::SeqCst) {
            return core::task::Poll::Ready(());
        }

        *self.state.waker.lock() = Some(cx.waker().clone());
        if !self.scheduled {
            self.scheduled = true;
            self.schedule();
        }

        if self.state.done.load(Ordering::SeqCst) {
            core::task::Poll::Ready(())
        } else {
            core::task::Poll::Pending
        }
    }
}
