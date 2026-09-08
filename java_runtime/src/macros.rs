macro_rules! simple_exception {
    ($type_name:ident, $java_name:literal, $parent:literal, $debug_name:literal) => {
        pub struct $type_name;

        impl $type_name {
            pub fn as_proto() -> $crate::RuntimeClassProto {
                $crate::RuntimeClassProto {
                    name: $java_name,
                    parent_class: Some($parent),
                    interfaces: alloc::vec![],
                    methods: alloc::vec![
                        java_class_proto::JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                        java_class_proto::JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default(),),
                    ],
                    fields: alloc::vec![],
                    access_flags: Default::default(),
                }
            }

            async fn init(jvm: &jvm::Jvm, _: &mut $crate::RuntimeContext, this: jvm::ClassInstanceRef<Self>) -> jvm::Result<()> {
                tracing::debug!(concat!($debug_name, "::<init>({:?})"), &this);
                let _: () = jvm.invoke_special(&this, $parent, "<init>", "()V", ()).await?;
                Ok(())
            }

            async fn init_with_message(
                jvm: &jvm::Jvm,
                _: &mut $crate::RuntimeContext,
                this: jvm::ClassInstanceRef<Self>,
                message: jvm::ClassInstanceRef<$crate::classes::java::lang::String>,
            ) -> jvm::Result<()> {
                tracing::debug!(concat!($debug_name, "::<init>({:?}, {:?})"), &this, &message);
                let _: () = jvm
                    .invoke_special(&this, $parent, "<init>", "(Ljava/lang/String;)V", (message,))
                    .await?;
                Ok(())
            }
        }
    };
}

macro_rules! proto_factories {
    ($($ty:ident),+ $(,)?) => {
        [$($ty::as_proto as $crate::RuntimeClassProtoFactory),+]
    };
}

macro_rules! simple_object {
    ($type_name:ident, $java_name:literal) => {
        impl $type_name {
            pub fn as_proto() -> $crate::RuntimeClassProto {
                $crate::RuntimeClassProto {
                    name: $java_name,
                    parent_class: Some("java/lang/Object"),
                    interfaces: alloc::vec![],
                    methods: alloc::vec![java_class_proto::JavaMethodProto::new(
                        "<init>",
                        "()V",
                        Self::init,
                        Default::default()
                    )],
                    fields: alloc::vec![],
                    access_flags: Default::default(),
                }
            }

            async fn init(jvm: &jvm::Jvm, _: &mut $crate::RuntimeContext, this: jvm::ClassInstanceRef<Self>) -> jvm::Result<()> {
                jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
            }
        }
    };
}

macro_rules! stub_void {
    ($($name:ident ( $($arg:ident : $ty:ty),* $(,)? ));+ $(;)?) => {
        $(
            async fn $name(
                _: &jvm::Jvm,
                _: &mut $crate::RuntimeContext,
                _this: jvm::ClassInstanceRef<Self>,
                $($arg: $ty),*
            ) -> jvm::Result<()> {
                Ok(())
            }
        )+
    };
}
