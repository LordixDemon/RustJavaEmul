use alloc::{boxed::Box, sync::Arc};
use core::hash::BuildHasher;

use event_listener::EventListener;

use crate::{Result, class_instance::ClassInstance};

use super::{Jvm, ObjectMonitor};

impl Jvm {
    pub fn object_listen(&self, obj: &Box<dyn ClassInstance>) -> EventListener {
        self.get_or_create_monitor(obj).event.listen()
    }

    pub async fn object_wait(&self, obj: &Box<dyn ClassInstance>) -> Result<()> {
        let obj = self.ensure_not_null(obj).await?;
        let thread_id = (self.inner.get_current_thread_id)();
        let monitor = self.get_or_create_monitor(obj);
        let saved = {
            let mut state = monitor.inner.lock();
            if state.owner != Some(thread_id) {
                None
            } else {
                let count = state.count;
                state.owner = None;
                state.count = 0;
                Some(count)
            }
        };
        let Some(saved) = saved else {
            return Err(self.exception("java/lang/IllegalMonitorStateException", "current thread not owner").await);
        };
        monitor.event.notify(usize::MAX);
        monitor.event.listen().await;
        self.monitor_enter_with_count(obj, saved).await
    }

    pub fn object_notify(&self, obj: &Box<dyn ClassInstance>, count: usize) {
        let monitor = self.get_or_create_monitor(obj);
        monitor.event.notify(count);
    }

    pub async fn monitor_enter(&self, obj: &Box<dyn ClassInstance>) -> Result<()> {
        let obj = self.ensure_not_null(obj).await?;
        self.monitor_enter_with_count(obj, 1).await
    }

    async fn monitor_enter_with_count(&self, obj: &Box<dyn ClassInstance>, add: u32) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        let monitor = self.get_or_create_monitor(obj);
        loop {
            {
                let mut state = monitor.inner.lock();
                if state.owner.is_none() || state.owner == Some(thread_id) {
                    state.owner = Some(thread_id);
                    state.count = state.count.saturating_add(add);
                    return Ok(());
                }
            }
            monitor.event.listen().await;
        }
    }

    pub async fn monitor_exit(&self, obj: &Box<dyn ClassInstance>) -> Result<()> {
        let thread_id = (self.inner.get_current_thread_id)();
        let monitor = self.get_or_create_monitor(obj);
        let not_owner = {
            let mut state = monitor.inner.lock();
            if state.owner != Some(thread_id) {
                true
            } else {
                state.count = state.count.saturating_sub(1);
                if state.count == 0 {
                    state.owner = None;
                    drop(state);
                    monitor.event.notify(1);
                }
                false
            }
        };
        if not_owner {
            return Err(self.exception("java/lang/IllegalMonitorStateException", "current thread not owner").await);
        }
        Ok(())
    }
    fn get_or_create_monitor(&self, obj: &Box<dyn ClassInstance>) -> Arc<ObjectMonitor> {
        let key = self.inner.monitor_hasher.hash_one(obj);

        let monitors = self.inner.monitors.read();
        if let Some(monitor) = monitors.get(&key) {
            return monitor.clone();
        }
        drop(monitors);

        let mut monitors = self.inner.monitors.write();
        monitors.entry(key).or_insert_with(|| Arc::new(ObjectMonitor::new())).clone()
    }
}
