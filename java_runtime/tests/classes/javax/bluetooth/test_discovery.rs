use alloc::{boxed::Box, collections::BTreeMap, vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::javax::bluetooth::{DeviceClass, DiscoveryAgent, LocalDevice, RemoteDevice, ServiceRecord},
};
use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

struct TestListener;

impl TestListener {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TestDiscoveryListener",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/bluetooth/DiscoveryListener"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "deviceDiscovered",
                    "(Ljavax/bluetooth/RemoteDevice;Ljavax/bluetooth/DeviceClass;)V",
                    Self::device_discovered,
                    Default::default(),
                ),
                JavaMethodProto::new("inquiryCompleted", "(I)V", Self::inquiry_completed, Default::default()),
                JavaMethodProto::new(
                    "servicesDiscovered",
                    "(I[Ljavax/bluetooth/ServiceRecord;)V",
                    Self::services_discovered,
                    Default::default(),
                ),
                JavaMethodProto::new("serviceSearchCompleted", "(II)V", Self::service_search_completed, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("devices", "I", Default::default()),
                JavaFieldProto::new("inquiryType", "I", Default::default()),
                JavaFieldProto::new("inquiryDone", "I", Default::default()),
                JavaFieldProto::new("searchResp", "I", Default::default()),
                JavaFieldProto::new("searchDone", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        // inquiryType/searchResp stay 0 until callbacks; use *Done flags so 0 is a valid discType.
        Ok(())
    }

    async fn device_discovered(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        _device: ClassInstanceRef<RemoteDevice>,
        _cod: ClassInstanceRef<DeviceClass>,
    ) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "devices", "I").await?;
        jvm.put_field(&mut this, "devices", "I", count + 1).await
    }

    async fn inquiry_completed(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, disc_type: i32) -> Result<()> {
        jvm.put_field(&mut this, "inquiryType", "I", disc_type).await?;
        jvm.put_field(&mut this, "inquiryDone", "I", 1).await
    }

    async fn services_discovered(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _trans: i32,
        _records: ClassInstanceRef<Array<ServiceRecord>>,
    ) -> Result<()> {
        Ok(())
    }

    async fn service_search_completed(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, _trans: i32, resp: i32) -> Result<()> {
        jvm.put_field(&mut this, "searchResp", "I", resp).await?;
        jvm.put_field(&mut this, "searchDone", "I", 1).await
    }
}

async fn test_jvm_with_listener() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        TestListener::as_proto(),
        Box::new(runtime) as Box<_>,
    ));
    jvm.register_class(class, None).await?;
    Ok(jvm)
}

async fn wait_field(jvm: &Jvm, instance: &Box<dyn jvm::ClassInstance>, field: &str, expected: i32) -> Result<i32> {
    for _ in 0..400 {
        let value: i32 = jvm.get_field(instance, field, "I").await?;
        if value == expected {
            return Ok(value);
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    jvm.get_field(instance, field, "I").await
}

#[tokio::test]
async fn test_discovery_listener_constants() -> Result<()> {
    let jvm = test_utils::test_jvm().await?;
    let completed: i32 = jvm
        .get_static_field("javax/bluetooth/DiscoveryListener", "INQUIRY_COMPLETED", "I")
        .await?;
    let terminated: i32 = jvm
        .get_static_field("javax/bluetooth/DiscoveryListener", "INQUIRY_TERMINATED", "I")
        .await?;
    let error: i32 = jvm.get_static_field("javax/bluetooth/DiscoveryListener", "INQUIRY_ERROR", "I").await?;
    let search_done: i32 = jvm
        .get_static_field("javax/bluetooth/DiscoveryListener", "SERVICE_SEARCH_COMPLETED", "I")
        .await?;
    let no_records: i32 = jvm
        .get_static_field("javax/bluetooth/DiscoveryListener", "SERVICE_SEARCH_NO_RECORDS", "I")
        .await?;
    assert_eq!(completed, 0);
    assert_eq!(terminated, 5);
    assert_eq!(error, 7);
    assert_eq!(search_done, 1);
    assert_eq!(no_records, 4);
    Ok(())
}

#[tokio::test]
async fn test_start_inquiry_notifies_listener() -> Result<()> {
    let jvm = test_jvm_with_listener().await?;
    let listener = jvm.new_class("TestDiscoveryListener", "()V", ()).await?;
    let local: ClassInstanceRef<LocalDevice> = jvm
        .invoke_static("javax/bluetooth/LocalDevice", "getLocalDevice", "()Ljavax/bluetooth/LocalDevice;", ())
        .await?;
    let agent: ClassInstanceRef<DiscoveryAgent> = jvm
        .invoke_virtual(&local, "getDiscoveryAgent", "()Ljavax/bluetooth/DiscoveryAgent;", ())
        .await?;
    let started: bool = jvm
        .invoke_virtual(
            &agent,
            "startInquiry",
            "(ILjavax/bluetooth/DiscoveryListener;)Z",
            (0x9e8b33i32, listener.clone()),
        )
        .await?;
    assert!(started);

    let done = wait_field(&jvm, &listener, "inquiryDone", 1).await?;
    let devices: i32 = jvm.get_field(&listener, "devices", "I").await?;
    let disc_type: i32 = jvm.get_field(&listener, "inquiryType", "I").await?;
    assert_eq!(done, 1);
    assert_eq!(devices, 1);
    assert_eq!(disc_type, 0);
    Ok(())
}

#[tokio::test]
async fn test_search_services_notifies_listener() -> Result<()> {
    let jvm = test_jvm_with_listener().await?;
    let listener = jvm.new_class("TestDiscoveryListener", "()V", ()).await?;
    let local: ClassInstanceRef<LocalDevice> = jvm
        .invoke_static("javax/bluetooth/LocalDevice", "getLocalDevice", "()Ljavax/bluetooth/LocalDevice;", ())
        .await?;
    let agent: ClassInstanceRef<DiscoveryAgent> = jvm
        .invoke_virtual(&local, "getDiscoveryAgent", "()Ljavax/bluetooth/DiscoveryAgent;", ())
        .await?;
    let addr = jvm::runtime::JavaLangString::from_rust_string(&jvm, "000000000001").await?;
    let device = jvm.new_class("javax/bluetooth/RemoteDevice", "(Ljava/lang/String;)V", (addr,)).await?;
    let uuids = jvm.instantiate_array("Ljavax/bluetooth/UUID;", 0).await?;
    let attrs = jvm.instantiate_array("I", 0).await?;
    let trans: i32 = jvm
        .invoke_virtual(
            &agent,
            "searchServices",
            "([I[Ljavax/bluetooth/UUID;Ljavax/bluetooth/RemoteDevice;Ljavax/bluetooth/DiscoveryListener;)I",
            (attrs, uuids, device, listener.clone()),
        )
        .await?;
    assert!(trans > 0);

    let done = wait_field(&jvm, &listener, "searchDone", 1).await?;
    let resp: i32 = jvm.get_field(&listener, "searchResp", "I").await?;
    assert_eq!(done, 1);
    assert_eq!(resp, 1);
    Ok(())
}

#[tokio::test]
async fn test_device_class_major_mask() -> Result<()> {
    let jvm = test_utils::test_jvm().await?;
    let cod = jvm.new_class("javax/bluetooth/DeviceClass", "(I)V", (0x200i32,)).await?;
    let major: i32 = jvm.invoke_virtual(&cod, "getMajorDeviceClass", "()I", ()).await?;
    assert_eq!(major, 0x200);
    Ok(())
}

#[tokio::test]
async fn test_local_device_is_power_on() -> Result<()> {
    let jvm = test_utils::test_jvm().await?;
    let on: bool = jvm.invoke_static("javax/bluetooth/LocalDevice", "isPowerOn", "()Z", ()).await?;
    assert!(on);
    Ok(())
}
