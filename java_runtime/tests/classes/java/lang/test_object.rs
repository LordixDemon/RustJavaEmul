use core::{
    panic,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use alloc::{boxed::Box, collections::btree_map::BTreeMap, sync::Arc};

use java_runtime::{RuntimeClock, SpawnCallback, classes::java::lang::Object};
use jvm::{ClassInstanceRef, JavaError, Jvm, Result};

use test_utils::{TestRuntime, create_test_jvm};

#[tokio::test]
async fn test_wait() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let notified = Arc::new(AtomicBool::new(false));

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;

    struct Notifier {
        jvm: Jvm,
        notified: Arc<AtomicBool>,
        runtime: TestRuntime,
        target: ClassInstanceRef<Object>,
    }

    #[async_trait::async_trait]
    impl SpawnCallback for Notifier {
        async fn call(&self) -> Result<()> {
            self.jvm.attach_thread()?;

            self.runtime.sleep(Duration::from_millis(100)).await;
            self.notified.store(true, Ordering::Relaxed);
            let target: Box<dyn jvm::ClassInstance> = self.target.clone().into();
            self.jvm.monitor_enter(&target).await?;
            let _: () = self.jvm.invoke_virtual(&self.target, "notify", "()V", ()).await?;
            self.jvm.monitor_exit(&target).await?;

            self.jvm.detach_thread()?;

            Ok(())
        }
    }

    runtime.spawn(
        &jvm,
        Box::new(Notifier {
            jvm: jvm.clone(),
            notified: notified.clone(),
            runtime: runtime.clone(),
            target: object.clone().into(),
        }),
    );

    assert!(!notified.load(Ordering::Relaxed));
    jvm.monitor_enter(&object).await?;
    let _: () = jvm.invoke_virtual(&object, "wait", "()V", ()).await?;
    jvm.monitor_exit(&object).await?;
    assert!(notified.load(Ordering::Relaxed));

    Ok(())
}
#[tokio::test]
async fn test_wait_timeout() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let notified = Arc::new(AtomicBool::new(false));

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;

    struct Notifier {
        jvm: Jvm,
        notified: Arc<AtomicBool>,
        runtime: TestRuntime,
        target: ClassInstanceRef<Object>,
    }

    #[async_trait::async_trait]
    impl SpawnCallback for Notifier {
        async fn call(&self) -> Result<()> {
            self.jvm.attach_thread()?;

            self.runtime.sleep(Duration::from_millis(1000)).await;
            self.notified.store(true, Ordering::Relaxed);
            let target: Box<dyn jvm::ClassInstance> = self.target.clone().into();
            self.jvm.monitor_enter(&target).await?;
            let _: () = self.jvm.invoke_virtual(&self.target, "notify", "()V", ()).await?;
            self.jvm.monitor_exit(&target).await?;

            self.jvm.detach_thread()?;

            Ok(())
        }
    }

    runtime.spawn(
        &jvm,
        Box::new(Notifier {
            jvm: jvm.clone(),
            notified: notified.clone(),
            runtime: runtime.clone(),
            target: object.clone().into(),
        }),
    );

    assert!(!notified.load(Ordering::Relaxed));
    jvm.monitor_enter(&object).await?;
    let _: () = jvm.invoke_virtual(&object, "wait", "(J)V", (100i64,)).await?;
    jvm.monitor_exit(&object).await?;
    assert!(!notified.load(Ordering::Relaxed));

    Ok(())
}

#[tokio::test]
async fn test_clone_not_cloneable() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;

    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&object, "clone", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(java_exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };

    let class_name = java_exception.class_definition().name();
    assert_eq!(class_name, "java/lang/CloneNotSupportedException");

    Ok(())
}

#[tokio::test]
async fn test_hash_code_is_stable_for_same_object() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime).await?;

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;

    let first: i32 = jvm.invoke_virtual(&object, "hashCode", "()I", ()).await?;
    let second: i32 = jvm.invoke_virtual(&object, "hashCode", "()I", ()).await?;

    assert_eq!(first, second);

    Ok(())
}

#[tokio::test]
async fn test_hash_code_is_not_constant_across_objects() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime).await?;

    let mut hashes = hashbrown::HashSet::new();

    for _ in 0..32 {
        let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
        let hash: i32 = jvm.invoke_virtual(&object, "hashCode", "()I", ()).await?;
        hashes.insert(hash);
    }

    assert!(hashes.len() > 1);

    Ok(())
}

#[tokio::test]
async fn test_invoke_virtual_on_null_is_npe() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime).await?;
    let null: ClassInstanceRef<Object> = None.into();
    let result: Result<i32> = jvm.invoke_virtual(&null, "hashCode", "()I", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("expected NullPointerException");
    };
    assert_eq!(exception.class_definition().name(), "java/lang/NullPointerException");
    Ok(())
}
