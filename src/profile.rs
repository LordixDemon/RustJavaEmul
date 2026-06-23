#![cfg_attr(not(feature = "desktop-window"), allow(dead_code))]

use std::{
    env,
    sync::{
        OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

use jvm_rust::ProfileSnapshot as JvmRustProfileSnapshot;

pub static WINDOW_LOOP: TimeCounter = TimeCounter::new("window_loop");
pub static WINDOW_UPDATE: TimeCounter = TimeCounter::new("window_update");
pub static WINDOW_FRAME_COPY: TimeCounter = TimeCounter::new("window_frame_copy");
pub static WINDOW_FRAME_UPLOAD: TimeCounter = TimeCounter::new("window_frame_upload");
pub static WINDOW_INPUT: TimeCounter = TimeCounter::new("window_input");
pub static SCREEN_DRAW_PIXELS: TimeCounter = TimeCounter::new("screen_draw_pixels");
pub static SCREEN_FILL_RECT: TimeCounter = TimeCounter::new("screen_fill_rect");
pub static SCREEN_PRESENT: TimeCounter = TimeCounter::new("screen_present");
pub static DECODE_IMAGE: TimeCounter = TimeCounter::new("decode_image");
pub static DISPATCH_KEY: TimeCounter = TimeCounter::new("dispatch_key");
pub static RUNTIME_SLEEP: TimeCounter = TimeCounter::new("runtime_sleep");
pub static RUNTIME_YIELD: TimeCounter = TimeCounter::new("runtime_yield");

pub struct TimeCounter {
    name: &'static str,
    calls: AtomicU64,
    nanos: AtomicU64,
}

impl TimeCounter {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: AtomicU64::new(0),
            nanos: AtomicU64::new(0),
        }
    }

    fn snapshot(&'static self) -> TimeCounterSnapshot {
        TimeCounterSnapshot {
            name: self.name,
            calls: self.calls.load(Ordering::Relaxed),
            nanos: self.nanos.load(Ordering::Relaxed),
        }
    }

    fn reset(&self) {
        self.calls.store(0, Ordering::Relaxed);
        self.nanos.store(0, Ordering::Relaxed);
    }

    fn record(&self, elapsed: Duration) {
        self.calls.fetch_add(1, Ordering::Relaxed);
        self.nanos
            .fetch_add(elapsed.as_nanos().min(u128::from(u64::MAX)) as u64, Ordering::Relaxed);
    }
}

#[derive(Clone, Copy)]
struct TimeCounterSnapshot {
    name: &'static str,
    calls: u64,
    nanos: u64,
}

pub struct Timer {
    counter: Option<&'static TimeCounter>,
    start: Option<Instant>,
}

impl Drop for Timer {
    fn drop(&mut self) {
        if let (Some(counter), Some(start)) = (self.counter, self.start) {
            counter.record(start.elapsed());
        }
    }
}

#[inline]
pub fn enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| env::var_os("RUSTJAVA_PROFILE").is_some())
}

#[inline]
pub fn timer(counter: &'static TimeCounter) -> Timer {
    let enabled = enabled();
    Timer {
        counter: enabled.then_some(counter),
        start: enabled.then(Instant::now),
    }
}

pub fn reset() {
    for counter in counters() {
        counter.reset();
    }
}

pub fn report(jvm_rust: JvmRustProfileSnapshot, elapsed: Duration) -> String {
    let mut lines = Vec::new();
    lines.push(format!("profile {:.2}s", elapsed.as_secs_f64()));

    let mut counters = counters().map(TimeCounter::snapshot);
    counters.sort_by_key(|counter| core::cmp::Reverse(counter.nanos));
    for counter in counters {
        if counter.calls == 0 {
            continue;
        }
        lines.push(format!(
            "  {:<18} calls {:>8} time {:>8.3}ms avg {:>7.3}us",
            counter.name,
            counter.calls,
            counter.nanos as f64 / 1_000_000.0,
            counter.nanos as f64 / counter.calls as f64 / 1_000.0
        ));
    }

    lines.push(format!(
        "  jvm opcodes={} fast={} slow={} methods={} jumps={} returns={} exceptions={}",
        jvm_rust.opcodes,
        jvm_rust.fast_opcodes,
        jvm_rust.slow_opcodes,
        jvm_rust.interpreter_runs,
        jvm_rust.jumps,
        jvm_rust.returns,
        jvm_rust.java_exceptions
    ));
    lines.push(format!(
        "  invoke virtual={} special={} static={} interface={}",
        jvm_rust.invoke_virtual, jvm_rust.invoke_special, jvm_rust.invoke_static, jvm_rust.invoke_interface
    ));
    lines.push(format!(
        "  arrays new={} load1={} store1={} load_bulk={} store_bulk={} copy={} raw_read={} raw_write={}",
        jvm_rust.array_new,
        jvm_rust.array_load_one,
        jvm_rust.array_store_one,
        jvm_rust.array_load_bulk,
        jvm_rust.array_store_bulk,
        jvm_rust.array_copy,
        jvm_rust.array_raw_read,
        jvm_rust.array_raw_write
    ));
    let load1_by_type = format_array_type_counts(&jvm_rust.array_load_one_by_type);
    if !load1_by_type.is_empty() {
        lines.push(format!("  arrays load1_by_type {load1_by_type}"));
    }
    let store1_by_type = format_array_type_counts(&jvm_rust.array_store_one_by_type);
    if !store1_by_type.is_empty() {
        lines.push(format!("  arrays store1_by_type {store1_by_type}"));
    }

    lines.join("\n")
}

fn format_array_type_counts(counts: &[u64; jvm_rust::ARRAY_TYPE_COUNT]) -> String {
    const NAMES: [&str; jvm_rust::ARRAY_TYPE_COUNT] = ["bool", "byte", "char", "short", "int", "long", "float", "double", "ref"];

    NAMES
        .iter()
        .zip(counts)
        .filter(|(_, count)| **count != 0)
        .map(|(name, count)| format!("{name}={count}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn counters() -> [&'static TimeCounter; 12] {
    [
        &WINDOW_LOOP,
        &WINDOW_UPDATE,
        &WINDOW_FRAME_COPY,
        &WINDOW_FRAME_UPLOAD,
        &WINDOW_INPUT,
        &SCREEN_DRAW_PIXELS,
        &SCREEN_FILL_RECT,
        &SCREEN_PRESENT,
        &DECODE_IMAGE,
        &DISPATCH_KEY,
        &RUNTIME_SLEEP,
        &RUNTIME_YIELD,
    ]
}
