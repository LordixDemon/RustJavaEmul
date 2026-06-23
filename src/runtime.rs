#[cfg(feature = "desktop-window")]
mod controls;
#[cfg(feature = "desktop-window")]
mod gpu_window;
mod io;

use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};
use core::{
    sync::atomic::{AtomicI32, AtomicU32, AtomicU64, Ordering},
    time::Duration,
};
use std::{
    env, fs,
    io::{Cursor, Write, stderr, stdin},
    sync::Mutex,
};

#[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
use std::time::{SystemTime, UNIX_EPOCH};

use java_runtime::{
    DecodedImage, File, FileDescriptorId, FileStat, FileType, IOError, IOResult, RT_RUSTJAR, Runtime, SpawnCallback, get_runtime_class_proto,
};
use jvm::{ClassDefinition, ClassInstance, Jvm};
use jvm_rust::{ArrayClassDefinitionImpl, ClassDefinitionImpl};

use crate::profile;

use self::io::{FileImpl, InputStreamFile, WriteStreamFile};

tokio::task_local! {
    static TASK_ID: u64;
}

static LAST_TASK_ID: AtomicU64 = AtomicU64::new(1);
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

type RmsStores = Arc<Mutex<BTreeMap<String, Vec<Option<Vec<i8>>>>>>;
type VirtualFiles = Arc<Mutex<BTreeMap<String, Vec<u8>>>>;

#[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
type SpawnedTask = tokio::task::JoinHandle<()>;
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
type SpawnedTask = ();

struct WriteWrapper<T>
where
    T: Sync + Send + Write + 'static,
{
    write: Arc<Mutex<T>>,
}

impl<T> Write for WriteWrapper<T>
where
    T: Sync + Send + Write + 'static,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.write.lock().unwrap().flush()
    }
}

impl<T> Clone for WriteWrapper<T>
where
    T: Sync + Send + Write + 'static,
{
    fn clone(&self) -> Self {
        Self { write: self.write.clone() }
    }
}

pub struct RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    stdout: WriteWrapper<T>,
    file_table: Arc<Mutex<BTreeMap<u32, Box<dyn File>>>>,
    next_fd: Arc<AtomicU32>,
    spawned_tasks: Arc<Mutex<Vec<SpawnedTask>>>,
    screen: Arc<Mutex<ScreenState>>,
    current_displayable: Arc<Mutex<Option<Box<dyn ClassInstance>>>>,
    java_threads: Arc<Mutex<BTreeMap<u64, Box<dyn ClassInstance>>>>,
    game_key_states: Arc<AtomicI32>,
    yield_counter: Arc<AtomicU64>,
    rms_stores: RmsStores,
    virtual_files: VirtualFiles,
}

#[cfg_attr(not(feature = "desktop-window"), allow(dead_code))]
struct ScreenState {
    width: usize,
    height: usize,
    front_pixels: Vec<u32>,
    back_pixels: Vec<u32>,
    generation: u64,
    present_requested: bool,
}

impl<T> RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    pub fn new(stdout: T, screen_width: usize, screen_height: usize) -> Self {
        jvm_rust::set_profile_enabled(profile::enabled());

        Self {
            stdout: WriteWrapper {
                write: Arc::new(Mutex::new(stdout)),
            },
            file_table: Arc::new(Mutex::new(BTreeMap::new())),
            next_fd: Arc::new(AtomicU32::new(1)),
            spawned_tasks: Arc::new(Mutex::new(Vec::new())),
            screen: Arc::new(Mutex::new(ScreenState::new(screen_width, screen_height))),
            current_displayable: Arc::new(Mutex::new(None)),
            java_threads: Arc::new(Mutex::new(BTreeMap::new())),
            game_key_states: Arc::new(AtomicI32::new(0)),
            yield_counter: Arc::new(AtomicU64::new(0)),
            rms_stores: Arc::new(Mutex::new(initial_rms_stores())),
            virtual_files: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    fn register_file(&self, file: Box<dyn File>) -> FileDescriptorId {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
        self.file_table.lock().unwrap().insert(fd, file);
        FileDescriptorId::new(fd)
    }

    pub fn abort_spawned(&self) {
        #[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
        for task in self.spawned_tasks.lock().unwrap().drain(..) {
            task.abort();
        }
        #[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
        self.spawned_tasks.lock().unwrap().clear();
    }

    pub fn add_virtual_file(&self, path: impl Into<String>, data: Vec<u8>) {
        self.virtual_files.lock().unwrap().insert(path.into(), data);
    }

    pub fn screen_dimensions(&self) -> (usize, usize) {
        let screen = self.screen.lock().unwrap();
        (screen.width, screen.height)
    }

    pub fn copy_presented_frame(&self, last_generation: u64, output: &mut Vec<u32>) -> Option<(usize, usize, u64)> {
        let Ok(mut screen) = self.screen.try_lock() else {
            return None;
        };
        screen.publish_pending_present();
        if screen.generation == last_generation {
            return None;
        }

        output.resize(screen.front_pixels.len(), 0);
        output.copy_from_slice(&screen.front_pixels);
        Some((screen.width, screen.height, screen.generation))
    }

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

    async fn dispatch_canvas_key(&self, jvm: &Jvm, method: &str, key_code: i32) -> anyhow::Result<()> {
        let _timer = profile::timer(&profile::DISPATCH_KEY);
        let diagnostics = input_diag_enabled();
        let previous_states = self.game_key_states.load(Ordering::Relaxed);
        self.update_game_key_state(method, key_code);
        let next_states = self.game_key_states.load(Ordering::Relaxed);

        let displayable = self.current_displayable.lock().unwrap().clone();
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
        let Some(mask) = game_key_mask(key_code) else {
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
    let mut last = JVM_PROFILE_LAST.lock().unwrap();
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
        "jvm delta opcodes={} fast={} slow={} methods={} jumps={} returns={} exceptions={} invoke v/s/st/i={}/{}/{}/{} arrays new={} load1={} store1={} loadBulk={} storeBulk={} copy={} raw r/w={}/{}",
        opcodes,
        fast,
        slow,
        methods,
        jumps,
        returns,
        exceptions,
        invoke_virtual,
        invoke_special,
        invoke_static,
        invoke_interface,
        array_new,
        array_load_one,
        array_store_one,
        array_load_bulk,
        array_store_bulk,
        array_copy,
        raw_read,
        raw_write
    )
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(crate) fn reset_jvm_profile_diagnostics() {
    jvm_rust::reset_profile();
    *JVM_PROFILE_LAST.lock().unwrap() = Some(jvm_rust::profile_snapshot());
}

#[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
#[allow(dead_code)]
pub(crate) fn browser_timer_diagnostics() -> String {
    "wasmSleep unavailable".to_string()
}

impl ScreenState {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            front_pixels: vec![0; width * height],
            back_pixels: vec![0; width * height],
            generation: 0,
            present_requested: false,
        }
    }

    fn draw_pixels(&mut self, x: i32, y: i32, width: i32, height: i32, pixels: &[i32], process_alpha: bool) {
        self.draw_pixels_strided(x, y, width, height, pixels, width, 0, 0, process_alpha);
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_pixels_strided(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        source_width: i32,
        source_x: i32,
        source_y: i32,
        process_alpha: bool,
    ) {
        if width <= 0 || height <= 0 || source_width <= 0 {
            return;
        }

        let dst_x0 = x.max(0);
        let dst_y0 = y.max(0);
        let dst_x1 = (x + width).min(self.width as i32);
        let dst_y1 = (y + height).min(self.height as i32);
        if dst_x0 >= dst_x1 || dst_y0 >= dst_y1 {
            return;
        }

        let copy_width = (dst_x1 - dst_x0) as usize;
        let copy_height = (dst_y1 - dst_y0) as usize;
        let src_x0 = source_x + dst_x0 - x;
        let src_y0 = source_y + dst_y0 - y;
        if src_x0 < 0 || src_y0 < 0 {
            return;
        }

        let dst_x0 = dst_x0 as usize;
        let dst_y0 = dst_y0 as usize;
        let src_x0 = src_x0 as usize;
        let src_y0 = src_y0 as usize;
        let source_width = source_width as usize;

        tracing::info!(
            target: "rustjava_render",
            "screen.drawPixels rect={}x{}+{}+{} source={}x?+{}+{} alpha={}",
            copy_width,
            copy_height,
            dst_x0,
            dst_y0,
            source_width,
            src_x0,
            src_y0,
            process_alpha
        );

        for row in 0..copy_height {
            let src_start = (src_y0 + row) * source_width + src_x0;
            let src_end = src_start + copy_width;
            let Some(src_row) = pixels.get(src_start..src_end) else {
                return;
            };
            let dst_start = (dst_y0 + row) * self.width + dst_x0;
            let dst_row = &mut self.back_pixels[dst_start..dst_start + copy_width];

            if process_alpha {
                for (dst, src) in dst_row.iter_mut().zip(src_row) {
                    *dst = compose_screen(*dst, *src, true);
                }
            } else {
                for (dst, src) in dst_row.iter_mut().zip(src_row) {
                    *dst = *src as u32 & 0x00ff_ffff;
                }
            }
        }
    }

    fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, argb: i32) {
        if width <= 0 || height <= 0 {
            return;
        }

        let x0 = x.max(0) as usize;
        let y0 = y.max(0) as usize;
        let x1 = (x + width).min(self.width as i32).max(0) as usize;
        let y1 = (y + height).min(self.height as i32).max(0) as usize;
        if x0 >= x1 || y0 >= y1 {
            return;
        }

        let alpha = ((argb as u32) >> 24) & 0xff;
        if alpha == 0 {
            return;
        }

        let color = argb as u32 & 0x00ff_ffff;
        tracing::info!(
            target: "rustjava_render",
            "screen.fillRect rect={}x{}+{}+{} alpha={} color={:#08x}",
            x1 - x0,
            y1 - y0,
            x0,
            y0,
            alpha,
            color
        );

        for py in y0..y1 {
            let row = &mut self.back_pixels[py * self.width + x0..py * self.width + x1];
            if alpha == 0xff {
                row.fill(color);
            } else {
                for dst in row {
                    *dst = compose_screen(*dst, argb, true);
                }
            }
        }
    }

    fn present(&mut self) {
        tracing::info!(
            target: "rustjava_render",
            "screen.present.request gen={} pending={}",
            self.generation,
            self.present_requested
        );
        self.present_requested = true;
    }

    #[cfg_attr(not(feature = "desktop-window"), allow(dead_code))]
    fn publish_pending_present(&mut self) {
        if !self.present_requested {
            return;
        }

        self.front_pixels.copy_from_slice(&self.back_pixels);
        self.generation += 1;
        if tracing::enabled!(target: "rustjava_render", tracing::Level::INFO) {
            tracing::info!(
                target: "rustjava_render",
                "screen.publish gen={} size={}x{} hash={:#018x}",
                self.generation,
                self.width,
                self.height,
                pixel_hash(&self.front_pixels)
            );
        }
        self.present_requested = false;
    }
}

#[async_trait::async_trait]
impl<T> Runtime for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    async fn sleep(&self, duration: Duration) {
        let _timer = profile::timer(&profile::RUNTIME_SLEEP);
        #[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
        browser_sleep(duration).await;
        #[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
        tokio::time::sleep(duration).await;
    }

    async fn r#yield(&self) {
        let _timer = profile::timer(&profile::RUNTIME_YIELD);
        let interval = yield_interval();
        let should_yield = interval <= 1 || self.yield_counter.fetch_add(1, Ordering::Relaxed) % interval == 0;
        if should_yield {
            #[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
            browser_yield().await;
            #[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
            tokio::task::yield_now().await;
        }
    }

    fn spawn(&self, _jvm: &Jvm, callback: Box<dyn SpawnCallback>) {
        let task_id = LAST_TASK_ID.fetch_add(1, Ordering::SeqCst) + 1;
        let current_java_thread = callback.current_java_thread();
        let java_threads = self.java_threads.clone();

        #[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
        {
            wasm_bindgen_futures::spawn_local(async move {
                TASK_ID
                    .scope(task_id, async move {
                        if let Some(thread) = current_java_thread {
                            java_threads.lock().unwrap().insert(task_id, thread);
                        }

                        let result = callback.call().await;
                        java_threads.lock().unwrap().remove(&task_id);
                        result.unwrap();
                    })
                    .await;
            });
            self.spawned_tasks.lock().unwrap().push(());
        }

        #[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
        {
            let handle = tokio::spawn(async move {
                TASK_ID
                    .scope(task_id, async move {
                        if let Some(thread) = current_java_thread {
                            java_threads.lock().unwrap().insert(task_id, thread);
                        }

                        let result = callback.call().await;
                        java_threads.lock().unwrap().remove(&task_id);
                        result.unwrap();
                    })
                    .await;
            });
            self.spawned_tasks.lock().unwrap().push(handle);
        }
    }

    fn now(&self) -> u64 {
        current_time_millis()
    }

    fn current_task_id(&self) -> u64 {
        TASK_ID.try_with(|x| *x).unwrap_or(0)
    }

    fn current_java_thread(&self) -> Option<Box<dyn ClassInstance>> {
        self.java_threads.lock().unwrap().get(&self.current_task_id()).cloned()
    }

    fn set_current_java_thread(&self, thread: Option<Box<dyn ClassInstance>>) {
        let task_id = self.current_task_id();
        let mut java_threads = self.java_threads.lock().unwrap();
        if let Some(thread) = thread {
            java_threads.insert(task_id, thread);
        } else {
            java_threads.remove(&task_id);
        }
    }

    fn stdin(&self) -> IOResult<FileDescriptorId> {
        let file = Box::new(InputStreamFile::new(stdin()));
        Ok(self.register_file(file))
    }

    fn stdout(&self) -> IOResult<FileDescriptorId> {
        let file = Box::new(WriteStreamFile::new(self.stdout.clone()));
        Ok(self.register_file(file))
    }

    fn stderr(&self) -> IOResult<FileDescriptorId> {
        let file = Box::new(WriteStreamFile::new(stderr()));
        Ok(self.register_file(file))
    }

    async fn open(&self, path: &str, write: bool) -> IOResult<FileDescriptorId> {
        if !write {
            if let Some(data) = self.virtual_files.lock().unwrap().get(path).cloned() {
                let file = Box::new(io::MemoryFile::new(data));
                return Ok(self.register_file(file));
            }
        }
        let file = Box::new(FileImpl::open(path, write)?);
        Ok(self.register_file(file))
    }

    fn get_file(&self, fd: FileDescriptorId) -> IOResult<Box<dyn File>> {
        self.file_table.lock().unwrap().get(&fd.id()).cloned().ok_or(IOError::NotFound)
    }

    fn close_file(&self, fd: FileDescriptorId) {
        self.file_table.lock().unwrap().remove(&fd.id());
    }

    async fn unlink(&self, path: &str) -> IOResult<()> {
        fs::remove_file(path).map_err(|_| IOError::NotFound) // TODO error conversion
    }

    async fn metadata(&self, path: &str) -> IOResult<FileStat> {
        if let Some(data) = self.virtual_files.lock().unwrap().get(path) {
            return Ok(FileStat {
                size: data.len() as u64,
                r#type: FileType::File,
            });
        }

        let metadata = fs::metadata(path);
        if let Ok(metadata) = metadata {
            let file_type = if metadata.is_dir() { FileType::Directory } else { FileType::File };

            Ok(FileStat {
                size: metadata.len(),
                r#type: file_type,
            })
        } else {
            Err(IOError::NotFound) // TODO error conversion
        }
    }

    async fn find_rustjar_class(&self, _jvm: &Jvm, classpath: &str, class: &str) -> jvm::Result<Option<Box<dyn ClassDefinition>>> {
        if classpath == RT_RUSTJAR {
            let proto = get_runtime_class_proto(class);
            if let Some(proto) = proto {
                return Ok(Some(Box::new(ClassDefinitionImpl::from_class_proto(
                    proto,
                    Box::new(self.clone()) as Box<_>,
                ))));
            }
        }

        Ok(None)
    }

    async fn define_class(&self, _jvm: &Jvm, data: &[u8]) -> jvm::Result<Box<dyn ClassDefinition>> {
        ClassDefinitionImpl::from_classfile(data).map(|x| Box::new(x) as Box<_>)
    }

    async fn define_array_class(&self, _jvm: &Jvm, element_type_name: &str) -> jvm::Result<Box<dyn ClassDefinition>> {
        Ok(Box::new(ArrayClassDefinitionImpl::new(element_type_name)))
    }

    fn decode_image(&self, data: &[u8]) -> Option<DecodedImage> {
        let _timer = profile::timer(&profile::DECODE_IMAGE);
        decode_png(data).or_else(|_| decode_bmp(data)).ok()
    }

    fn screen_width(&self) -> i32 {
        self.screen.lock().unwrap().width as i32
    }

    fn screen_height(&self) -> i32 {
        self.screen.lock().unwrap().height as i32
    }

    fn screen_draw_pixels(&self, x: i32, y: i32, width: i32, height: i32, pixels: &[i32], process_alpha: bool) {
        let _timer = profile::timer(&profile::SCREEN_DRAW_PIXELS);
        self.screen.lock().unwrap().draw_pixels(x, y, width, height, pixels, process_alpha);
    }

    fn screen_draw_pixels_strided(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        source_width: i32,
        source_x: i32,
        source_y: i32,
        process_alpha: bool,
    ) {
        let _timer = profile::timer(&profile::SCREEN_DRAW_PIXELS);
        self.screen
            .lock()
            .unwrap()
            .draw_pixels_strided(x, y, width, height, pixels, source_width, source_x, source_y, process_alpha);
    }

    fn screen_fill_rect(&self, x: i32, y: i32, width: i32, height: i32, argb: i32) {
        let _timer = profile::timer(&profile::SCREEN_FILL_RECT);
        self.screen.lock().unwrap().fill_rect(x, y, width, height, argb);
    }

    fn screen_present(&self) {
        let _timer = profile::timer(&profile::SCREEN_PRESENT);
        self.screen.lock().unwrap().present();
    }

    fn set_current_displayable(&self, displayable: Option<Box<dyn ClassInstance>>) {
        if input_diag_enabled() {
            match &displayable {
                Some(displayable) => eprintln!("[display] setCurrent {}", displayable.class_definition().name()),
                None => eprintln!("[display] setCurrent <none>"),
            }
        }
        *self.current_displayable.lock().unwrap() = displayable;
    }

    fn is_current_displayable(&self, displayable: &dyn ClassInstance) -> bool {
        self.current_displayable
            .lock()
            .unwrap()
            .as_deref()
            .is_some_and(|current| current.equals(displayable).unwrap_or(false))
    }

    fn game_key_states(&self) -> i32 {
        self.game_key_states.load(Ordering::Relaxed)
    }

    fn rms_open_record_store(&self, name: &str, create_if_necessary: bool) -> bool {
        let mut stores = self.rms_stores.lock().unwrap();
        if stores.contains_key(name) {
            return true;
        }
        if !create_if_necessary {
            return false;
        }

        stores.insert(name.to_owned(), Vec::new());
        true
    }

    fn rms_num_records(&self, name: &str) -> i32 {
        self.rms_stores
            .lock()
            .unwrap()
            .get(name)
            .map(|records| records.iter().filter(|record| record.is_some()).count() as i32)
            .unwrap_or(0)
    }

    fn rms_next_record_id(&self, name: &str) -> i32 {
        self.rms_stores
            .lock()
            .unwrap()
            .get(name)
            .map(|records| records.len() as i32 + 1)
            .unwrap_or(1)
    }

    fn rms_add_record(&self, name: &str, data: &[i8]) -> i32 {
        let mut stores = self.rms_stores.lock().unwrap();
        let records = stores.entry(name.to_owned()).or_default();
        records.push(Some(data.to_vec()));
        records.len() as i32
    }

    fn rms_delete_record(&self, name: &str, record_id: i32) {
        if record_id <= 0 {
            return;
        }

        if let Some(records) = self.rms_stores.lock().unwrap().get_mut(name) {
            if let Some(record) = records.get_mut(record_id as usize - 1) {
                *record = None;
            }
        }
    }

    fn rms_get_record(&self, name: &str, record_id: i32) -> Option<Vec<i8>> {
        if record_id <= 0 {
            return None;
        }

        self.rms_stores
            .lock()
            .unwrap()
            .get(name)
            .and_then(|records| records.get(record_id as usize - 1))
            .and_then(Clone::clone)
    }

    fn rms_set_record(&self, name: &str, record_id: i32, data: &[i8]) {
        if record_id <= 0 {
            return;
        }

        let mut stores = self.rms_stores.lock().unwrap();
        let records = stores.entry(name.to_owned()).or_default();
        while records.len() < record_id as usize {
            records.push(None);
        }
        records[record_id as usize - 1] = Some(data.to_vec());
    }

    fn rms_list_record_stores(&self) -> Vec<String> {
        self.rms_stores.lock().unwrap().keys().cloned().collect()
    }
}

impl<T> Clone for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stdout: self.stdout.clone(),
            file_table: self.file_table.clone(),
            next_fd: self.next_fd.clone(),
            spawned_tasks: self.spawned_tasks.clone(),
            screen: self.screen.clone(),
            current_displayable: self.current_displayable.clone(),
            java_threads: self.java_threads.clone(),
            game_key_states: self.game_key_states.clone(),
            yield_counter: self.yield_counter.clone(),
            rms_stores: self.rms_stores.clone(),
            virtual_files: self.virtual_files.clone(),
        }
    }
}

fn game_key_mask(key_code: i32) -> Option<i32> {
    match key_code {
        -1 => Some(0x0002),
        -3 => Some(0x0004),
        -4 => Some(0x0020),
        -2 => Some(0x0040),
        -5 => Some(0x0100),
        -6 => Some(0x0200),
        -7 => Some(0x0400),
        x if x == b'1' as i32 => Some(0x0200),
        x if x == b'3' as i32 => Some(0x0400),
        x if x == b'7' as i32 => Some(0x0800),
        x if x == b'9' as i32 => Some(0x1000),
        _ => None,
    }
}

fn decode_png(data: &[u8]) -> anyhow::Result<DecodedImage> {
    let mut decoder = png::Decoder::new(Cursor::new(data));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info()?;
    let output_buffer_size = reader
        .output_buffer_size()
        .ok_or_else(|| anyhow::anyhow!("PNG output buffer size is unknown"))?;
    let mut buf = vec![0; output_buffer_size];
    let info = reader.next_frame(&mut buf)?;
    let bytes = &buf[..info.buffer_size()];

    let mut argb = Vec::with_capacity(info.width as usize * info.height as usize);
    match info.color_type {
        png::ColorType::Rgb => {
            for px in bytes.chunks_exact(3) {
                argb.push(argb_pixel(255, px[0], px[1], px[2]));
            }
        }
        png::ColorType::Rgba => {
            for px in bytes.chunks_exact(4) {
                argb.push(argb_pixel(px[3], px[0], px[1], px[2]));
            }
        }
        png::ColorType::Grayscale => {
            for &v in bytes {
                argb.push(argb_pixel(255, v, v, v));
            }
        }
        png::ColorType::GrayscaleAlpha => {
            for px in bytes.chunks_exact(2) {
                argb.push(argb_pixel(px[1], px[0], px[0], px[0]));
            }
        }
        png::ColorType::Indexed => anyhow::bail!("indexed PNG was not expanded"),
    }

    Ok(DecodedImage {
        width: info.width as i32,
        height: info.height as i32,
        argb,
    })
}

fn decode_bmp(data: &[u8]) -> anyhow::Result<DecodedImage> {
    if data.len() < 54 || &data[..2] != b"BM" {
        anyhow::bail!("not a BMP image");
    }

    let pixel_offset = read_u32_le(data, 10)? as usize;
    let dib_size = read_u32_le(data, 14)? as usize;
    if dib_size < 40 || data.len() < 14 + dib_size {
        anyhow::bail!("unsupported BMP DIB header");
    }

    let width = read_i32_le(data, 18)?;
    let raw_height = read_i32_le(data, 22)?;
    let planes = read_u16_le(data, 26)?;
    let bits_per_pixel = read_u16_le(data, 28)?;
    let compression = read_u32_le(data, 30)?;
    let colors_used = read_u32_le(data, 46).unwrap_or(0) as usize;

    if width <= 0 || raw_height == 0 || planes != 1 || compression != 0 {
        anyhow::bail!("unsupported BMP format");
    }

    let top_down = raw_height < 0;
    let width = width as usize;
    let height = raw_height.unsigned_abs() as usize;
    let row_stride = ((width * bits_per_pixel as usize).div_ceil(32)) * 4;
    let palette = bmp_palette(data, dib_size, bits_per_pixel, colors_used)?;

    if pixel_offset + row_stride.saturating_mul(height) > data.len() {
        anyhow::bail!("truncated BMP pixels");
    }

    let mut argb = Vec::with_capacity(width * height);
    for y in 0..height {
        let src_y = if top_down { y } else { height - 1 - y };
        let row_offset = pixel_offset + src_y * row_stride;
        match bits_per_pixel {
            32 => {
                for x in 0..width {
                    let offset = row_offset + x * 4;
                    let b = data[offset];
                    let g = data[offset + 1];
                    let r = data[offset + 2];
                    let a = data[offset + 3];
                    argb.push(argb_pixel(if a == 0 { 255 } else { a }, r, g, b));
                }
            }
            24 => {
                for x in 0..width {
                    let offset = row_offset + x * 3;
                    argb.push(argb_pixel(255, data[offset + 2], data[offset + 1], data[offset]));
                }
            }
            8 => {
                for x in 0..width {
                    let index = data[row_offset + x] as usize;
                    argb.push(*palette.get(index).unwrap_or(&0xffff_00ffu32.cast_signed()));
                }
            }
            4 => {
                for x in 0..width {
                    let packed = data[row_offset + x / 2];
                    let index = if x % 2 == 0 { packed >> 4 } else { packed & 0x0f } as usize;
                    argb.push(*palette.get(index).unwrap_or(&0xffff_00ffu32.cast_signed()));
                }
            }
            1 => {
                for x in 0..width {
                    let packed = data[row_offset + x / 8];
                    let index = ((packed >> (7 - (x % 8))) & 1) as usize;
                    argb.push(*palette.get(index).unwrap_or(&0xffff_00ffu32.cast_signed()));
                }
            }
            _ => anyhow::bail!("unsupported BMP bit depth {bits_per_pixel}"),
        }
    }

    Ok(DecodedImage {
        width: width as i32,
        height: height as i32,
        argb,
    })
}

fn bmp_palette(data: &[u8], dib_size: usize, bits_per_pixel: u16, colors_used: usize) -> anyhow::Result<Vec<i32>> {
    if bits_per_pixel > 8 {
        return Ok(Vec::new());
    }

    let max_colors = 1usize << bits_per_pixel;
    let colors = if colors_used == 0 { max_colors } else { colors_used.min(max_colors) };
    let palette_offset = 14 + dib_size;
    let palette_bytes = colors * 4;
    if palette_offset + palette_bytes > data.len() {
        anyhow::bail!("truncated BMP palette");
    }

    let mut palette = Vec::with_capacity(colors);
    for index in 0..colors {
        let offset = palette_offset + index * 4;
        palette.push(argb_pixel(255, data[offset + 2], data[offset + 1], data[offset]));
    }

    Ok(palette)
}

fn read_u16_le(data: &[u8], offset: usize) -> anyhow::Result<u16> {
    let Some(bytes) = data.get(offset..offset + 2) else {
        anyhow::bail!("truncated u16");
    };
    Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
}

fn read_u32_le(data: &[u8], offset: usize) -> anyhow::Result<u32> {
    let Some(bytes) = data.get(offset..offset + 4) else {
        anyhow::bail!("truncated u32");
    };
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

fn read_i32_le(data: &[u8], offset: usize) -> anyhow::Result<i32> {
    let Some(bytes) = data.get(offset..offset + 4) else {
        anyhow::bail!("truncated i32");
    };
    Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
}

fn argb_pixel(a: u8, r: u8, g: u8, b: u8) -> i32 {
    ((a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32) as i32
}

fn compose_screen(dst: u32, src: i32, process_alpha: bool) -> u32 {
    let src = src as u32;
    if !process_alpha {
        return src & 0x00ff_ffff;
    }

    let alpha = (src >> 24) & 0xff;
    if alpha == 0xff {
        return src & 0x00ff_ffff;
    }
    if alpha == 0 {
        return dst;
    }

    let inv = 255 - alpha;
    let r = (((src >> 16) & 0xff) * alpha + ((dst >> 16) & 0xff) * inv) / 255;
    let g = (((src >> 8) & 0xff) * alpha + ((dst >> 8) & 0xff) * inv) / 255;
    let b = ((src & 0xff) * alpha + (dst & 0xff) * inv) / 255;

    (r << 16) | (g << 8) | b
}

#[cfg_attr(not(feature = "desktop-window"), allow(dead_code))]
fn pixel_hash(pixels: &[u32]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for pixel in pixels {
        hash ^= *pixel as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[cfg(feature = "desktop-window")]
fn window_target_fps() -> usize {
    env::var("RUSTJAVA_WINDOW_FPS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|fps| *fps > 0)
        .unwrap_or(60)
}

fn yield_interval() -> u64 {
    static INTERVAL: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *INTERVAL.get_or_init(|| {
        env::var("RUSTJAVA_YIELD_INTERVAL")
            .ok()
            .and_then(|value| value.parse().ok())
            .filter(|interval| *interval > 0)
            .unwrap_or(64)
    })
}

fn input_diag_enabled() -> bool {
    env::var_os("RUSTJAVA_INPUT_DIAG").is_some() || env::var_os("RUSTJAVA_DIAG").is_some()
}

fn initial_rms_stores() -> BTreeMap<String, Vec<Option<Vec<i8>>>> {
    let mut stores = BTreeMap::new();
    let Ok(presets) = env::var("RUSTJAVA_RMS_PRESET") else {
        return stores;
    };

    for entry in presets.split(';').map(str::trim).filter(|entry| !entry.is_empty()) {
        let Some((name, records)) = entry.split_once('=').or_else(|| entry.split_once(':')) else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() {
            continue;
        }

        let records = records
            .split(',')
            .map(str::trim)
            .filter(|record| !record.is_empty())
            .filter_map(parse_hex_record)
            .map(Some)
            .collect::<Vec<_>>();
        if !records.is_empty() {
            stores.insert(name.to_owned(), records);
        }
    }

    stores
}

fn parse_hex_record(record: &str) -> Option<Vec<i8>> {
    let hex = record
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace() && *ch != '_' && *ch != '-')
        .collect::<String>();
    if hex.is_empty() || hex.len() % 2 != 0 {
        return None;
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for offset in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[offset..offset + 2], 16).ok()?;
        bytes.push(byte as i8);
    }
    Some(bytes)
}

#[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
fn current_time_millis() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
fn current_time_millis() -> u64 {
    js_sys::Date::now() as u64
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
async fn browser_sleep(duration: Duration) {
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
async fn browser_yield() {
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
            if let Some(waker) = state.waker.lock().unwrap().take() {
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

        *self.state.waker.lock().unwrap() = Some(cx.waker().clone());
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
            if let Some(waker) = state.waker.lock().unwrap().take() {
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

        *self.state.waker.lock().unwrap() = Some(cx.waker().clone());
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
