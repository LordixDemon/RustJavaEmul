mod class_define;
pub(crate) mod clock;
#[cfg(feature = "desktop-window")]
mod control_server;
#[cfg(feature = "desktop-window")]
mod controls;
mod device;
mod fs;
#[cfg(feature = "desktop-window")]
mod gpu_window;
mod http;
mod io;
mod rms;
mod screen;
#[cfg(feature = "desktop-window")]
mod screenshot;
mod spawn;
mod wasm_sleep;
mod window;

use alloc::{collections::BTreeMap, sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicI32, AtomicU32, AtomicU64};
use std::io::Write;

use java_runtime::{DeviceProfile, File};
use jvm::ClassInstance;
use parking_lot::Mutex;

use crate::profile;

use spawn::SpawnedTask;

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(crate) use wasm_sleep::{browser_timer_diagnostics, jvm_profile_diagnostics, reset_jvm_profile_diagnostics};

pub(crate) type RmsStores = Arc<Mutex<BTreeMap<String, Vec<Option<Vec<i8>>>>>>;
pub(crate) type VirtualFiles = Arc<Mutex<BTreeMap<String, Vec<u8>>>>;

pub(crate) struct WriteWrapper<T>
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
        self.write.lock().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.write.lock().flush()
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
    pub(crate) stdout: WriteWrapper<T>,
    pub(crate) file_table: Arc<Mutex<BTreeMap<u32, Box<dyn File>>>>,
    pub(crate) next_fd: Arc<AtomicU32>,
    pub(crate) spawned_tasks: Arc<Mutex<Vec<SpawnedTask>>>,
    pub(crate) screen: Arc<Mutex<screen::ScreenState>>,
    pub(crate) current_displayable: Arc<Mutex<Option<Box<dyn ClassInstance>>>>,
    pub(crate) java_threads: Arc<Mutex<BTreeMap<u64, Box<dyn ClassInstance>>>>,
    pub(crate) game_key_states: Arc<AtomicI32>,
    pub(crate) yield_counter: Arc<AtomicU64>,
    pub(crate) rms_stores: RmsStores,
    pub(crate) virtual_files: VirtualFiles,
    pub(crate) profile: Arc<Mutex<DeviceProfile>>,
    pub(crate) session_label: Arc<Mutex<String>>,
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
            screen: Arc::new(Mutex::new(screen::ScreenState::new(screen_width, screen_height))),
            current_displayable: Arc::new(Mutex::new(None)),
            java_threads: Arc::new(Mutex::new(BTreeMap::new())),
            game_key_states: Arc::new(AtomicI32::new(0)),
            yield_counter: Arc::new(AtomicU64::new(0)),
            rms_stores: Arc::new(Mutex::new(rms::initial_rms_stores())),
            virtual_files: Arc::new(Mutex::new(BTreeMap::new())),
            profile: Arc::new(Mutex::new(DeviceProfile::Generic)),
            session_label: Arc::new(Mutex::new("RustJava".to_string())),
        }
    }

    pub fn set_device_profile(&self, profile: DeviceProfile) {
        *self.profile.lock() = profile;
    }

    pub fn device_profile(&self) -> DeviceProfile {
        *self.profile.lock()
    }

    pub fn set_session_label(&self, label: impl Into<String>) {
        *self.session_label.lock() = label.into();
    }

    pub fn session_label(&self) -> String {
        self.session_label.lock().clone()
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
            profile: self.profile.clone(),
            session_label: self.session_label.clone(),
        }
    }
}

impl<T> Drop for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    fn drop(&mut self) {
        if Arc::strong_count(&self.spawned_tasks) <= 1 {
            self.abort_spawned();
        }
    }
}
