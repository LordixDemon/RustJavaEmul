use alloc::{boxed::Box, collections::BTreeMap};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::javax::microedition::media::{Player, PlayerListener},
};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

struct TestPlayerListener;

impl TestPlayerListener {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TestPlayerListener",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/PlayerListener"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "playerUpdate",
                    "(Ljavax/microedition/media/Player;Ljava/lang/String;Ljava/lang/Object;)V",
                    Self::player_update,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("started", "I", Default::default()),
                JavaFieldProto::new("ended", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn player_update(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        _player: ClassInstanceRef<Player>,
        event: ClassInstanceRef<java_runtime::classes::java::lang::String>,
        _data: ClassInstanceRef<java_runtime::classes::java::lang::Object>,
    ) -> Result<()> {
        let started = JavaLangString::intern_rust_string(jvm, "started").await?;
        let ended = JavaLangString::intern_rust_string(jvm, "endOfMedia").await?;
        let event: Box<dyn jvm::ClassInstance> = event.into();
        if event == started {
            jvm.put_field(&mut this, "started", "I", 1).await?;
        }
        if event == ended {
            jvm.put_field(&mut this, "ended", "I", 1).await?;
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_player_start_notifies_end_of_media() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        TestPlayerListener::as_proto(),
        Box::new(runtime) as Box<_>,
    ));
    jvm.register_class(class, None).await?;

    let player = jvm.new_class("javax/microedition/media/Player", "()V", ()).await?;
    let listener = jvm.new_class("TestPlayerListener", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(
            &player,
            "addPlayerListener",
            "(Ljavax/microedition/media/PlayerListener;)V",
            (ClassInstanceRef::<PlayerListener>::from(listener.clone()),),
        )
        .await?;
    let _: () = jvm.invoke_virtual(&player, "start", "()V", ()).await?;

    let started: i32 = jvm.get_field(&listener, "started", "I").await?;
    let ended: i32 = jvm.get_field(&listener, "ended", "I").await?;
    let state: i32 = jvm.invoke_virtual(&player, "getState", "()I", ()).await?;
    assert_eq!(started, 1);
    assert_eq!(ended, 1);
    assert_eq!(state, 300);
    Ok(())
}
