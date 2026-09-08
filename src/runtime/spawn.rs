use alloc::boxed::Box;
use core::sync::atomic::{AtomicU64, Ordering};
use std::io::Write;

use java_runtime::SpawnCallback;
use jvm::ClassInstance;

use super::RuntimeImpl;

tokio::task_local! {
    pub(super) static TASK_ID: u64;
}

static LAST_TASK_ID: AtomicU64 = AtomicU64::new(1);

#[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
pub(super) type SpawnedTask = tokio::task::JoinHandle<()>;
#[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
pub(super) type SpawnedTask = ();

fn log_spawned_task_error(error: impl core::fmt::Display) {
    tracing::error!("spawned java task failed: {error}");
    #[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
    crate::browser::browser_log(format!("spawned java task failed: {error}"));
}

impl<T> RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    pub fn abort_spawned(&self) {
        #[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
        for task in self.spawned_tasks.lock().drain(..) {
            task.abort();
        }
        #[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
        self.spawned_tasks.lock().clear();
    }

    pub fn current_displayable(&self) -> Option<Box<dyn ClassInstance>> {
        self.current_displayable.lock().clone()
    }

    pub(super) fn spawn_java(&self, callback: Box<dyn SpawnCallback>) {
        let task_id = LAST_TASK_ID.fetch_add(1, Ordering::SeqCst) + 1;
        let current_java_thread = callback.current_java_thread();
        let java_threads = self.java_threads.clone();

        #[cfg(all(target_arch = "wasm32", feature = "browser-window"))]
        {
            wasm_bindgen_futures::spawn_local(async move {
                TASK_ID
                    .scope(task_id, async move {
                        if let Some(thread) = current_java_thread {
                            java_threads.lock().insert(task_id, thread);
                        }

                        let result = callback.call().await;
                        java_threads.lock().remove(&task_id);
                        if let Err(error) = result {
                            log_spawned_task_error(error);
                        }
                    })
                    .await;
            });
        }

        #[cfg(not(all(target_arch = "wasm32", feature = "browser-window")))]
        {
            let handle = tokio::spawn(async move {
                TASK_ID
                    .scope(task_id, async move {
                        if let Some(thread) = current_java_thread {
                            java_threads.lock().insert(task_id, thread);
                        }

                        let result = callback.call().await;
                        java_threads.lock().remove(&task_id);
                        if let Err(error) = result {
                            log_spawned_task_error(error);
                        }
                    })
                    .await;
            });
            let mut tasks = self.spawned_tasks.lock();
            tasks.retain(|task| !task.is_finished());
            tasks.push(handle);
        }
    }

    pub(super) fn host_task_id(&self) -> u64 {
        TASK_ID.try_with(|x| *x).unwrap_or(0)
    }

    pub(super) fn current_java_thread_for_task(&self) -> Option<Box<dyn ClassInstance>> {
        self.java_threads.lock().get(&self.host_task_id()).cloned()
    }

    pub(super) fn set_java_thread_for_task(&self, thread: Option<Box<dyn ClassInstance>>) {
        let task_id = self.host_task_id();
        let mut java_threads = self.java_threads.lock();
        if let Some(thread) = thread {
            java_threads.insert(task_id, thread);
        } else {
            java_threads.remove(&task_id);
        }
    }
}
