use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

pub struct UUID;
pub struct DeviceClass;
pub struct DataElement;
pub struct RemoteDevice;
pub struct ServiceRecord;
pub struct DiscoveryListener;
pub struct DiscoveryAgent;
pub struct LocalDevice;
pub struct L2CAPConnection;
pub struct L2CAPConnectionNotifier;

simple_exception!(
    BluetoothStateException,
    "javax/bluetooth/BluetoothStateException",
    "java/io/IOException",
    "javax.bluetooth.BluetoothStateException"
);
simple_exception!(
    ServiceRegistrationException,
    "javax/bluetooth/ServiceRegistrationException",
    "java/io/IOException",
    "javax.bluetooth.ServiceRegistrationException"
);

pub struct BluetoothConnectionException;

impl BluetoothConnectionException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/BluetoothConnectionException",
            parent_class: Some("java/io/IOException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
                JavaMethodProto::new("getStatus", "()I", Self::get_status, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("status", "I", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/io/IOException", "<init>", "()V", ()).await
    }
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        jvm.invoke_special(&this, "java/io/IOException", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
    async fn get_status(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "status", "I").await
    }
}

mod data_element;
mod device_class;
mod discovery;
mod discovery_listener;
mod l2cap;
mod l2cap_notifier;
mod local_device;
mod remote_device;
mod service_record;
mod uuid;

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        BluetoothConnectionException,
        BluetoothStateException,
        DataElement,
        DeviceClass,
        DiscoveryAgent,
        DiscoveryListener,
        L2CAPConnection,
        L2CAPConnectionNotifier,
        LocalDevice,
        RemoteDevice,
        ServiceRecord,
        ServiceRegistrationException,
        UUID,
    ]
}
