use alloc::{
    boxed::Box,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};
use core::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use parking_lot::Mutex;

use crate::{RuntimeClassProto, RuntimeContext, SpawnCallback, classes::java::lang::Runnable};

// class java.lang.Thread
pub struct Thread;

static THREAD_SLEEP_CALLS: AtomicU64 = AtomicU64::new(0);
static THREAD_SLEEP_TOTAL_MS: AtomicU64 = AtomicU64::new(0);
static THREAD_SLEEP_LAST_MS: AtomicU64 = AtomicU64::new(0);
static THREAD_SLEEP_MAX_MS: AtomicU64 = AtomicU64::new(0);
static THREAD_SLEEP_ZERO_CALLS: AtomicU64 = AtomicU64::new(0);
static THREAD_SLEEP_SHORT_CALLS: AtomicU64 = AtomicU64::new(0);
static THREAD_SLEEP_FRAME_CALLS: AtomicU64 = AtomicU64::new(0);
static THREAD_SLEEP_LONG_CALLS: AtomicU64 = AtomicU64::new(0);
static THREAD_ACTIVE_BETWEEN_SLEEP_LAST_MS: AtomicU64 = AtomicU64::new(0);
static THREAD_ACTIVE_BETWEEN_SLEEP_MAX_MS: AtomicU64 = AtomicU64::new(0);
static THREAD_ACTIVE_BETWEEN_SLEEP_TOTAL_MS: AtomicU64 = AtomicU64::new(0);
static THREAD_ACTIVE_BETWEEN_SLEEP_SAMPLES: AtomicU64 = AtomicU64::new(0);
static THREAD_ACTIVE_BETWEEN_SLEEP_LE_4MS: AtomicU64 = AtomicU64::new(0);
static THREAD_ACTIVE_BETWEEN_SLEEP_5_10MS: AtomicU64 = AtomicU64::new(0);
static THREAD_ACTIVE_BETWEEN_SLEEP_11_20MS: AtomicU64 = AtomicU64::new(0);
static THREAD_ACTIVE_BETWEEN_SLEEP_21_40MS: AtomicU64 = AtomicU64::new(0);
static THREAD_ACTIVE_BETWEEN_SLEEP_OVER_40MS: AtomicU64 = AtomicU64::new(0);
static THREAD_SLEEP_LAST_BY_TASK: Mutex<Vec<ThreadSleepTaskSample>> = Mutex::new(Vec::new());
static THREAD_ACTIVE_LAST_TRACE: Mutex<Option<RustString>> = Mutex::new(None);

#[derive(Clone, Copy)]
struct ThreadSleepTaskSample {
    task_id: u64,
    wake_ms: u64,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct ThreadSleepStats {
    pub(crate) calls: u64,
    pub(crate) total_ms: u64,
    pub(crate) last_ms: u64,
    pub(crate) max_ms: u64,
    pub(crate) zero_calls: u64,
    pub(crate) short_calls: u64,
    pub(crate) frame_calls: u64,
    pub(crate) long_calls: u64,
    pub(crate) active_samples: u64,
    pub(crate) active_total_ms: u64,
    pub(crate) active_last_ms: u64,
    pub(crate) active_max_ms: u64,
    pub(crate) active_le_4ms: u64,
    pub(crate) active_5_10ms: u64,
    pub(crate) active_11_20ms: u64,
    pub(crate) active_21_40ms: u64,
    pub(crate) active_over_40ms: u64,
}

impl Thread {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Thread",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/Runnable;)V", Self::init_with_runnable, Default::default()),
                JavaMethodProto::new("start", "()V", Self::start, Default::default()),
                JavaMethodProto::new("join", "()V", Self::join, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
                JavaMethodProto::new("isAlive", "()Z", Self::is_alive, Default::default()),
                JavaMethodProto::new("sleep", "(J)V", Self::sleep, MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC),
                JavaMethodProto::new("yield", "()V", Self::r#yield, MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC),
                JavaMethodProto::new("setPriority", "(I)V", Self::set_priority, Default::default()),
                JavaMethodProto::new(
                    "currentThread",
                    "()Ljava/lang/Thread;",
                    Self::current_thread,
                    MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC,
                ),
                // rustjava internal
                JavaMethodProto::new("<init>", "(Z)V", Self::init_internal, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("id", "J", Default::default()),
                JavaFieldProto::new("target", "Ljava/lang/Runnable;", Default::default()),
                JavaFieldProto::new("alive", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_runnable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Runnable>,
    ) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({:?}, {:?})", &this, &target);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "target", "Ljava/lang/Runnable;", target).await?;

        Ok(())
    }

    async fn init_internal(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, internal: bool) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({:?}, {:?})", &this, internal);

        let id = context.current_task_id();
        jvm.put_field(&mut this, "id", "J", id as i64).await?;

        Ok(())
    }

    async fn start(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::start({:?})", &this);

        struct ThreadStartProxy {
            jvm: Jvm,
            thread_id: i32,
            this: ClassInstanceRef<Thread>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for ThreadStartProxy {
            async fn call(&self) -> Result<()> {
                tracing::trace!(id = self.thread_id, "Thread start");

                self.jvm.attach_thread()?;

                let result: Result<()> = self.jvm.invoke_virtual(&self.this, "run", "()V", []).await;

                if let Err(jvm::JavaError::JavaException(x)) = result {
                    let string_writer = self.jvm.new_class("java/io/StringWriter", "()V", ()).await.unwrap();
                    let print_writer = self
                        .jvm
                        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
                        .await
                        .unwrap();

                    let _: () = self
                        .jvm
                        .invoke_virtual(&x, "printStackTrace", "(Ljava/io/PrintWriter;)V", (print_writer,))
                        .await
                        .unwrap();

                    let trace = self
                        .jvm
                        .invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", [])
                        .await
                        .unwrap();

                    tracing::error!(
                        "Uncaught exception in thread {}:\n{}",
                        self.thread_id,
                        JavaLangString::to_rust_string(&self.jvm, &trace).await.unwrap()
                    );
                } else {
                    result?;
                }

                self.jvm.detach_thread()?;

                let mut this = self.this.clone();
                self.jvm.put_field(&mut this, "alive", "Z", false).await.unwrap();
                self.jvm.object_notify(&self.this, usize::MAX);

                Ok(())
            }

            fn current_java_thread(&self) -> Option<Box<dyn jvm::ClassInstance>> {
                Some(self.this.clone().into())
            }
        }

        jvm.put_field(&mut this, "alive", "Z", true).await?;

        let id: i32 = jvm.invoke_virtual(&this, "hashCode", "()I", ()).await?;

        context.spawn(
            jvm,
            Box::new(ThreadStartProxy {
                jvm: jvm.clone(),
                thread_id: id,
                this: this.clone(),
            }),
        );

        Ok(())
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::run({:?})", &this);

        let target: ClassInstanceRef<Runnable> = jvm.get_field(&this, "target", "Ljava/lang/Runnable;").await?;
        if !target.is_null() {
            let _: () = jvm.invoke_virtual(&target, "run", "()V", ()).await?;
        }

        Ok(())
    }

    async fn join(jvm: &Jvm, _context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::join({:?})", &this);

        loop {
            let listener = jvm.object_listen(&this);
            let alive: bool = jvm.get_field(&this, "alive", "Z").await?;
            if !alive {
                return Ok(());
            }
            listener.await;
        }
    }

    async fn is_alive(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Thread::isAlive({:?})", &this);
        let alive: bool = jvm.get_field(&this, "alive", "Z").await?;
        Ok(alive)
    }

    async fn sleep(jvm: &Jvm, context: &mut RuntimeContext, duration: i64) -> Result<()> {
        tracing::debug!("java.lang.Thread::sleep({:?})", duration);

        let duration = duration.max(0) as u64;
        let task_id = context.current_task_id();
        let trace = Self::sleep_caller_trace(jvm);
        Self::record_sleep_call(task_id, context.now(), duration, trace);
        context.sleep(Duration::from_millis(duration)).await;
        Self::record_sleep_wake(task_id, context.now());

        Ok(())
    }

    async fn r#yield(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.lang.Thread::yield()");
        context.r#yield().await;

        Ok(())
    }

    async fn set_priority(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Thread>, new_priority: i32) -> Result<()> {
        tracing::warn!("stub java.lang.Thread::setPriority({:?}, {:?})", &this, new_priority);

        Ok(())
    }

    async fn current_thread(jvm: &Jvm, context: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::trace!("java.lang.Thread::currentThread()");

        if let Some(thread) = context.current_java_thread() {
            return Ok(thread.into());
        }

        let thread = jvm.new_class("java/lang/Thread", "(Z)V", (true,)).await?;
        context.set_current_java_thread(Some(thread.clone()));

        Ok(thread.into())
    }

    pub(crate) fn sleep_stats() -> ThreadSleepStats {
        ThreadSleepStats {
            calls: THREAD_SLEEP_CALLS.load(Ordering::Relaxed),
            total_ms: THREAD_SLEEP_TOTAL_MS.load(Ordering::Relaxed),
            last_ms: THREAD_SLEEP_LAST_MS.load(Ordering::Relaxed),
            max_ms: THREAD_SLEEP_MAX_MS.load(Ordering::Relaxed),
            zero_calls: THREAD_SLEEP_ZERO_CALLS.load(Ordering::Relaxed),
            short_calls: THREAD_SLEEP_SHORT_CALLS.load(Ordering::Relaxed),
            frame_calls: THREAD_SLEEP_FRAME_CALLS.load(Ordering::Relaxed),
            long_calls: THREAD_SLEEP_LONG_CALLS.load(Ordering::Relaxed),
            active_samples: THREAD_ACTIVE_BETWEEN_SLEEP_SAMPLES.load(Ordering::Relaxed),
            active_total_ms: THREAD_ACTIVE_BETWEEN_SLEEP_TOTAL_MS.load(Ordering::Relaxed),
            active_last_ms: THREAD_ACTIVE_BETWEEN_SLEEP_LAST_MS.load(Ordering::Relaxed),
            active_max_ms: THREAD_ACTIVE_BETWEEN_SLEEP_MAX_MS.load(Ordering::Relaxed),
            active_le_4ms: THREAD_ACTIVE_BETWEEN_SLEEP_LE_4MS.load(Ordering::Relaxed),
            active_5_10ms: THREAD_ACTIVE_BETWEEN_SLEEP_5_10MS.load(Ordering::Relaxed),
            active_11_20ms: THREAD_ACTIVE_BETWEEN_SLEEP_11_20MS.load(Ordering::Relaxed),
            active_21_40ms: THREAD_ACTIVE_BETWEEN_SLEEP_21_40MS.load(Ordering::Relaxed),
            active_over_40ms: THREAD_ACTIVE_BETWEEN_SLEEP_OVER_40MS.load(Ordering::Relaxed),
        }
    }

    pub(crate) fn active_last_trace() -> RustString {
        THREAD_ACTIVE_LAST_TRACE.lock().clone().unwrap_or_else(|| "none".to_string())
    }

    fn record_sleep_call(task_id: u64, now_ms: u64, duration_ms: u64, trace: RustString) {
        Self::record_active_between_sleep(task_id, now_ms, trace);
        THREAD_SLEEP_CALLS.fetch_add(1, Ordering::Relaxed);
        THREAD_SLEEP_TOTAL_MS.fetch_add(duration_ms, Ordering::Relaxed);
        THREAD_SLEEP_LAST_MS.store(duration_ms, Ordering::Relaxed);
        THREAD_SLEEP_MAX_MS.fetch_max(duration_ms, Ordering::Relaxed);
        match duration_ms {
            0 => {
                THREAD_SLEEP_ZERO_CALLS.fetch_add(1, Ordering::Relaxed);
            }
            1..=5 => {
                THREAD_SLEEP_SHORT_CALLS.fetch_add(1, Ordering::Relaxed);
            }
            6..=20 => {
                THREAD_SLEEP_FRAME_CALLS.fetch_add(1, Ordering::Relaxed);
            }
            _ => {
                THREAD_SLEEP_LONG_CALLS.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    fn record_sleep_wake(task_id: u64, wake_ms: u64) {
        let mut samples = THREAD_SLEEP_LAST_BY_TASK.lock();
        if let Some(sample) = samples.iter_mut().find(|sample| sample.task_id == task_id) {
            sample.wake_ms = wake_ms;
            return;
        }

        if samples.len() >= 16 {
            samples.remove(0);
        }
        samples.push(ThreadSleepTaskSample { task_id, wake_ms });
    }

    fn record_active_between_sleep(task_id: u64, now_ms: u64, trace: RustString) {
        let mut samples = THREAD_SLEEP_LAST_BY_TASK.lock();
        if let Some(sample) = samples.iter_mut().find(|sample| sample.task_id == task_id) {
            let active_ms = now_ms.saturating_sub(sample.wake_ms);
            drop(samples);
            Self::record_active_sample(active_ms, trace);
        }
    }

    fn record_active_sample(active_ms: u64, trace: RustString) {
        THREAD_ACTIVE_BETWEEN_SLEEP_SAMPLES.fetch_add(1, Ordering::Relaxed);
        THREAD_ACTIVE_BETWEEN_SLEEP_TOTAL_MS.fetch_add(active_ms, Ordering::Relaxed);
        THREAD_ACTIVE_BETWEEN_SLEEP_LAST_MS.store(active_ms, Ordering::Relaxed);
        THREAD_ACTIVE_BETWEEN_SLEEP_MAX_MS.fetch_max(active_ms, Ordering::Relaxed);
        *THREAD_ACTIVE_LAST_TRACE.lock() = Some(trace);
        let bucket = if active_ms <= 4 {
            &THREAD_ACTIVE_BETWEEN_SLEEP_LE_4MS
        } else if active_ms <= 10 {
            &THREAD_ACTIVE_BETWEEN_SLEEP_5_10MS
        } else if active_ms <= 20 {
            &THREAD_ACTIVE_BETWEEN_SLEEP_11_20MS
        } else if active_ms <= 40 {
            &THREAD_ACTIVE_BETWEEN_SLEEP_21_40MS
        } else {
            &THREAD_ACTIVE_BETWEEN_SLEEP_OVER_40MS
        };
        bucket.fetch_add(1, Ordering::Relaxed);
    }

    fn sleep_caller_trace(jvm: &Jvm) -> RustString {
        let mut output = RustString::new();
        let mut count = 0;
        for frame in jvm.stack_trace() {
            if frame.starts_with("java/lang/Thread.sleep") {
                continue;
            }
            if !output.is_empty() {
                output.push('<');
            }
            output.push_str(&frame);
            count += 1;
            if count >= 4 {
                break;
            }
        }

        if output.is_empty() { "none".to_string() } else { output }
    }
}
