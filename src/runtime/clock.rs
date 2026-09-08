use alloc::boxed::Box;
use core::{sync::atomic::Ordering, time::Duration};
use std::io::Write;

use java_runtime::{RuntimeClock, SpawnCallback};
use jvm::{ClassInstance, Jvm};

use super::RuntimeImpl;
use crate::profile;

#[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
use std::time::{SystemTime, UNIX_EPOCH};

#[async_trait::async_trait]
impl<T> RuntimeClock for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    async fn sleep(&self, duration: Duration) {
        let _timer = profile::timer(&profile::RUNTIME_SLEEP);
        #[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
        super::wasm_sleep::browser_sleep(duration).await;
        #[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
        tokio::time::sleep(duration).await;
    }

    async fn r#yield(&self) {
        let _timer = profile::timer(&profile::RUNTIME_YIELD);
        let interval = yield_interval();
        let should_yield = interval <= 1 || self.yield_counter.fetch_add(1, Ordering::Relaxed) % interval == 0;
        if should_yield {
            #[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
            super::wasm_sleep::browser_yield().await;
            #[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
            tokio::task::yield_now().await;
        }
    }

    fn spawn(&self, _jvm: &Jvm, callback: Box<dyn SpawnCallback>) {
        self.spawn_java(callback);
    }

    fn now(&self) -> u64 {
        current_time_millis()
    }

    fn current_task_id(&self) -> u64 {
        self.host_task_id()
    }

    fn current_java_thread(&self) -> Option<Box<dyn ClassInstance>> {
        self.current_java_thread_for_task()
    }

    fn set_current_java_thread(&self, thread: Option<Box<dyn ClassInstance>>) {
        self.set_java_thread_for_task(thread);
    }
}

fn yield_interval() -> u64 {
    crate::config::get().yield_interval
}

#[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
pub(crate) fn current_time_millis() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(crate) fn current_time_millis() -> u64 {
    js_sys::Date::now() as u64
}
