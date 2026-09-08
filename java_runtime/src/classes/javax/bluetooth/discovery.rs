#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::{Runtime, RuntimeClassProto, RuntimeContext, SpawnCallback, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::{boxed::Box, format, vec};
#[allow(unused_imports)]
use core::{
    sync::atomic::{AtomicI32, Ordering},
    time::Duration,
};
#[allow(unused_imports)]
use dyn_clone::clone_box;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
#[allow(unused_imports)]
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
#[allow(unused_imports)]
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

impl DiscoveryAgent {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/DiscoveryAgent",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "cancelInquiry",
                    "(Ljavax/bluetooth/DiscoveryListener;)Z",
                    Self::cancel_inquiry,
                    Default::default(),
                ),
                JavaMethodProto::new("cancelServiceSearch", "(I)Z", Self::cancel_service_search, Default::default()),
                JavaMethodProto::new(
                    "searchServices",
                    "([I[Ljavax/bluetooth/UUID;Ljavax/bluetooth/RemoteDevice;Ljavax/bluetooth/DiscoveryListener;)I",
                    Self::search_services,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "startInquiry",
                    "(ILjavax/bluetooth/DiscoveryListener;)Z",
                    Self::start_inquiry,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "retrieveDevices",
                    "(I)[Ljavax/bluetooth/RemoteDevice;",
                    Self::retrieve_devices,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "selectService",
                    "(Ljavax/bluetooth/UUID;IZ)Ljava/lang/String;",
                    Self::select_service,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("GIAC", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LIAC", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("NOT_DISCOVERABLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("CACHED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PREKNOWN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/bluetooth/DiscoveryAgent";
        jvm.put_static_field(class, "GIAC", "I", 0x9e8b33).await?;
        jvm.put_static_field(class, "LIAC", "I", 0x9e8b00).await?;
        jvm.put_static_field(class, "NOT_DISCOVERABLE", "I", 0).await?;
        jvm.put_static_field(class, "CACHED", "I", 0).await?;
        jvm.put_static_field(class, "PREKNOWN", "I", 1).await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn cancel_inquiry(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _listener: ClassInstanceRef<DiscoveryListener>,
    ) -> Result<bool> {
        Ok(true)
    }

    pub(super) async fn cancel_service_search(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _trans: i32) -> Result<bool> {
        Ok(true)
    }

    pub(super) async fn search_services(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _attr: ClassInstanceRef<Array<i32>>,
        _uuid: ClassInstanceRef<Array<UUID>>,
        device: ClassInstanceRef<RemoteDevice>,
        listener: ClassInstanceRef<DiscoveryListener>,
    ) -> Result<i32> {
        if listener.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "listener").await);
        }
        static NEXT_TRANS: AtomicI32 = AtomicI32::new(1);
        let trans = NEXT_TRANS.fetch_add(1, Ordering::Relaxed);

        struct ServiceSearchProxy {
            jvm: Jvm,
            listener: ClassInstanceRef<DiscoveryListener>,
            device: ClassInstanceRef<RemoteDevice>,
            trans: i32,
            context: Box<dyn Runtime>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for ServiceSearchProxy {
            async fn call(&self) -> Result<()> {
                self.context.sleep(Duration::from_millis(5)).await;
                self.jvm.attach_thread()?;

                let result: Result<()> = async {
                    let mut record: ClassInstanceRef<ServiceRecord> = self.jvm.new_class("javax/bluetooth/ServiceRecord", "()V", ()).await?.into();
                    if !self.device.is_null() {
                        self.jvm
                            .put_field(&mut record, "host", "Ljavax/bluetooth/RemoteDevice;", self.device.clone())
                            .await?;
                    }
                    let mut records = self.jvm.instantiate_array("Ljavax/bluetooth/ServiceRecord;", 1).await?;
                    self.jvm.store_array(&mut records, 0, vec![record]).await?;
                    let _: () = self
                        .jvm
                        .invoke_virtual(
                            &self.listener,
                            "servicesDiscovered",
                            "(I[Ljavax/bluetooth/ServiceRecord;)V",
                            (self.trans, records),
                        )
                        .await?;
                    // SERVICE_SEARCH_COMPLETED
                    let _: () = self
                        .jvm
                        .invoke_virtual(&self.listener, "serviceSearchCompleted", "(II)V", (self.trans, 0x01i32))
                        .await?;
                    Ok(())
                }
                .await;

                if let Err(JavaError::JavaException(exception)) = result {
                    tracing::error!("Uncaught exception in DiscoveryAgent.searchServices: {exception:?}");
                } else {
                    result?;
                }

                self.jvm.detach_thread()?;
                Ok(())
            }
        }

        context.spawn(
            jvm,
            Box::new(ServiceSearchProxy {
                jvm: jvm.clone(),
                listener,
                device,
                trans,
                context: clone_box(context),
            }),
        );

        Ok(trans)
    }

    pub(super) async fn start_inquiry(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _access: i32,
        listener: ClassInstanceRef<DiscoveryListener>,
    ) -> Result<bool> {
        if listener.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "listener").await);
        }

        struct InquiryCompleteProxy {
            jvm: Jvm,
            listener: ClassInstanceRef<DiscoveryListener>,
            context: Box<dyn Runtime>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for InquiryCompleteProxy {
            async fn call(&self) -> Result<()> {
                self.context.sleep(Duration::from_millis(5)).await;
                self.jvm.attach_thread()?;

                let result: Result<()> = async {
                    let discovered: Result<()> = async {
                        let addr = JavaLangString::from_rust_string(&self.jvm, "000000000001").await?;
                        let device = self
                            .jvm
                            .new_class("javax/bluetooth/RemoteDevice", "(Ljava/lang/String;)V", (addr,))
                            .await?;
                        let cod = self.jvm.new_class("javax/bluetooth/DeviceClass", "(I)V", (0x200i32,)).await?;
                        self.jvm
                            .invoke_virtual(
                                &self.listener,
                                "deviceDiscovered",
                                "(Ljavax/bluetooth/RemoteDevice;Ljavax/bluetooth/DeviceClass;)V",
                                (device, cod),
                            )
                            .await
                    }
                    .await;
                    if let Err(JavaError::JavaException(exception)) = discovered {
                        tracing::error!("Uncaught exception in DiscoveryListener.deviceDiscovered: {exception:?}");
                    } else {
                        discovered?;
                    }
                    // INQUIRY_COMPLETED — always, so waiters on inquiryCompleted/notify are released.
                    let _: () = self.jvm.invoke_virtual(&self.listener, "inquiryCompleted", "(I)V", (0i32,)).await?;
                    Ok(())
                }
                .await;

                if let Err(JavaError::JavaException(exception)) = result {
                    tracing::error!("Uncaught exception in DiscoveryAgent.startInquiry: {exception:?}");
                } else {
                    result?;
                }

                self.jvm.detach_thread()?;
                Ok(())
            }
        }

        context.spawn(
            jvm,
            Box::new(InquiryCompleteProxy {
                jvm: jvm.clone(),
                listener,
                context: clone_box(context),
            }),
        );

        Ok(true)
    }

    pub(super) async fn retrieve_devices(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _option: i32,
    ) -> Result<ClassInstanceRef<Array<RemoteDevice>>> {
        Ok(jvm.instantiate_array("Ljavax/bluetooth/RemoteDevice;", 0).await?.into())
    }

    pub(super) async fn select_service(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _uuid: ClassInstanceRef<UUID>,
        _security: i32,
        _master: bool,
    ) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "").await.map(Into::into)
    }
}
