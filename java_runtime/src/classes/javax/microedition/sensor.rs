use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

pub struct ChannelInfo;
pub struct Data;
pub struct DataListener;
pub struct MeasurementRange;
pub struct SensorConnection;
pub struct SensorInfo;
pub struct SensorManager;
pub struct Unit;

impl ChannelInfo {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/sensor/ChannelInfo",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getDataType", "()I", Self::get_data_type, Default::default()),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new("getAccuracy", "()F", Self::get_accuracy, Default::default()),
                JavaMethodProto::new("getScale", "()I", Self::get_scale, Default::default()),
                JavaMethodProto::new("getUnit", "()Ljavax/microedition/sensor/Unit;", Self::get_unit, Default::default()),
                JavaMethodProto::new(
                    "getMeasurementRanges",
                    "()[Ljavax/microedition/sensor/MeasurementRange;",
                    Self::get_measurement_ranges,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn get_data_type(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(2)
    }
    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "axis").await.map(Into::into)
    }
    async fn get_accuracy(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(0.01)
    }
    async fn get_scale(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }
    async fn get_unit(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Unit>> {
        Ok(jvm.new_class("javax/microedition/sensor/Unit", "()V", ()).await?.into())
    }
    async fn get_measurement_ranges(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<MeasurementRange>>>> {
        Ok(jvm.instantiate_array("Ljavax/microedition/sensor/MeasurementRange;", 0).await?.into())
    }
}

impl MeasurementRange {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/sensor/MeasurementRange",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(DDD)V", Self::init, Default::default()),
                JavaMethodProto::new("getSmallestValue", "()D", Self::get_smallest, Default::default()),
                JavaMethodProto::new("getLargestValue", "()D", Self::get_largest, Default::default()),
                JavaMethodProto::new("getResolution", "()D", Self::get_resolution, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("smallest", "D", Default::default()),
                JavaFieldProto::new("largest", "D", Default::default()),
                JavaFieldProto::new("resolution", "D", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, smallest: f64, largest: f64, resolution: f64) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "smallest", "D", smallest).await?;
        jvm.put_field(&mut this, "largest", "D", largest).await?;
        jvm.put_field(&mut this, "resolution", "D", resolution).await?;
        Ok(())
    }
    async fn get_smallest(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        jvm.get_field(&this, "smallest", "D").await
    }
    async fn get_largest(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        jvm.get_field(&this, "largest", "D").await
    }
    async fn get_resolution(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        jvm.get_field(&this, "resolution", "D").await
    }
}

impl Unit {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/sensor/Unit",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "m/s^2").await.map(Into::into)
    }
}

impl Data {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/sensor/Data",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getDoubleValues", "()[D", Self::get_double_values, Default::default()),
                JavaMethodProto::new("getIntValues", "()[I", Self::get_int_values, Default::default()),
                JavaMethodProto::new(
                    "getChannelInfo",
                    "()Ljavax/microedition/sensor/ChannelInfo;",
                    Self::get_channel_info,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn get_double_values(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<f64>>> {
        Ok(jvm.instantiate_array("D", 0).await?.into())
    }
    async fn get_int_values(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i32>>> {
        Ok(jvm.instantiate_array("I", 0).await?.into())
    }
    async fn get_channel_info(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<ChannelInfo>> {
        Ok(jvm.new_class("javax/microedition/sensor/ChannelInfo", "()V", ()).await?.into())
    }
}

impl DataListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/sensor/DataListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl SensorInfo {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/sensor/SensorInfo",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getChannelInfos",
                    "()[Ljavax/microedition/sensor/ChannelInfo;",
                    Self::get_channel_infos,
                    Default::default(),
                ),
                JavaMethodProto::new("getUrl", "()Ljava/lang/String;", Self::get_url, Default::default()),
                JavaMethodProto::new("getQuantity", "()Ljava/lang/String;", Self::get_quantity, Default::default()),
                JavaMethodProto::new("getContextType", "()Ljava/lang/String;", Self::get_context_type, Default::default()),
                JavaMethodProto::new("getConnectionType", "()I", Self::get_connection_type, Default::default()),
                JavaMethodProto::new("getDescription", "()Ljava/lang/String;", Self::get_description, Default::default()),
                JavaMethodProto::new("getModel", "()Ljava/lang/String;", Self::get_model, Default::default()),
                JavaMethodProto::new("getMaxBufferSize", "()I", Self::get_max_buffer_size, Default::default()),
                JavaMethodProto::new("isAvailable", "()Z", Self::is_available, Default::default()),
                JavaMethodProto::new(
                    "isAvailabilityPushSupported",
                    "()Z",
                    Self::is_availability_push_supported,
                    Default::default(),
                ),
                JavaMethodProto::new("isConditionPushSupported", "()Z", Self::is_condition_push_supported, Default::default()),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;)Ljava/lang/Object;",
                    Self::get_property,
                    Default::default(),
                ),
                JavaMethodProto::new("getPropertyNames", "()[Ljava/lang/String;", Self::get_property_names, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("CONTEXT_TYPE_USER", "Ljava/lang/String;", FieldAccessFlags::STATIC)],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn get_channel_infos(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<ChannelInfo>>>> {
        Ok(jvm.instantiate_array("Ljavax/microedition/sensor/ChannelInfo;", 0).await?.into())
    }
    async fn get_url(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "").await.map(Into::into)
    }
    async fn get_quantity(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "acceleration").await.map(Into::into)
    }
    async fn get_context_type(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "user").await.map(Into::into)
    }
    async fn get_connection_type(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1)
    }
    async fn get_description(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "Accelerometer").await.map(Into::into)
    }
    async fn get_model(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "Generic").await.map(Into::into)
    }
    async fn get_max_buffer_size(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(256)
    }
    async fn is_available(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(true)
    }
    async fn is_availability_push_supported(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }
    async fn is_condition_push_supported(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }
    async fn get_property(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<crate::classes::java::lang::Object>> {
        Ok(ClassInstanceRef::new(None))
    }
    async fn get_property_names(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<String>>>> {
        Ok(jvm.instantiate_array("Ljava/lang/String;", 0).await?.into())
    }
}

impl SensorConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/sensor/SensorConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getSensorInfo",
                    "()Ljavax/microedition/sensor/SensorInfo;",
                    Self::get_sensor_info,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setDataListener",
                    "(Ljavax/microedition/sensor/DataListener;I)V",
                    Self::set_data_listener,
                    Default::default(),
                ),
                JavaMethodProto::new("getState", "()I", Self::get_state, Default::default()),
                JavaMethodProto::new("getData", "(I)[Ljavax/microedition/sensor/Data;", Self::get_data, Default::default()),
                JavaMethodProto::new(
                    "getData",
                    "(IJZZZ)[Ljavax/microedition/sensor/Data;",
                    Self::get_data_full,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn get_sensor_info(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<SensorInfo>> {
        Ok(jvm.new_class("javax/microedition/sensor/SensorInfo", "()V", ()).await?.into())
    }
    async fn get_state(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1)
    }
    async fn get_data(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _buffer_size: i32,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<Data>>>> {
        Ok(jvm.instantiate_array("Ljavax/microedition/sensor/Data;", 0).await?.into())
    }
    async fn get_data_full(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _buffer_size: i32,
        _buffering_period: i64,
        _is_timestamp_included: bool,
        _is_uncertainty_included: bool,
        _is_validity_included: bool,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<Data>>>> {
        Ok(jvm.instantiate_array("Ljavax/microedition/sensor/Data;", 0).await?.into())
    }
    stub_void! {
        set_data_listener(_listener: ClassInstanceRef<DataListener>, _buffer_size: i32);
    }
}

impl SensorManager {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/sensor/SensorManager",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "findSensors",
                    "(Ljava/lang/String;)[Ljavax/microedition/sensor/SensorInfo;",
                    Self::find_sensors,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "findSensors",
                    "(Ljava/lang/String;Ljava/lang/String;)[Ljavax/microedition/sensor/SensorInfo;",
                    Self::find_sensors_quantity,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn find_sensors(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _url: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<SensorInfo>>>> {
        Ok(jvm.instantiate_array("Ljavax/microedition/sensor/SensorInfo;", 0).await?.into())
    }
    async fn find_sensors_quantity(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _quantity: ClassInstanceRef<String>,
        _context: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<SensorInfo>>>> {
        Ok(jvm.instantiate_array("Ljavax/microedition/sensor/SensorInfo;", 0).await?.into())
    }
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        ChannelInfo,
        Data,
        DataListener,
        MeasurementRange,
        SensorConnection,
        SensorInfo,
        SensorManager,
        Unit
    ]
}
