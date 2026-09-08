use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

pub struct Coordinates;
pub struct QualifiedCoordinates;
pub struct Criteria;
pub struct Location;
pub struct LocationListener;
pub struct LocationProvider;

simple_exception!(
    LocationException,
    "javax/microedition/location/LocationException",
    "java/lang/Exception",
    "javax.microedition.location.LocationException"
);

impl Coordinates {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/location/Coordinates",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getLatitude", "()D", Self::get_latitude, Default::default()),
                JavaMethodProto::new("getLongitude", "()D", Self::get_longitude, Default::default()),
                JavaMethodProto::new("getAltitude", "()F", Self::get_altitude, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("latitude", "D", Default::default()),
                JavaFieldProto::new("longitude", "D", Default::default()),
                JavaFieldProto::new("altitude", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn get_latitude(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        jvm.get_field(&this, "latitude", "D").await
    }
    async fn get_longitude(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        jvm.get_field(&this, "longitude", "D").await
    }
    async fn get_altitude(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "altitude", "F").await
    }
}

impl QualifiedCoordinates {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/location/QualifiedCoordinates",
            parent_class: Some("javax/microedition/location/Coordinates"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getHorizontalAccuracy", "()F", Self::get_horizontal, Default::default()),
                JavaMethodProto::new("getVerticalAccuracy", "()F", Self::get_vertical, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("horizontalAccuracy", "F", Default::default()),
                JavaFieldProto::new("verticalAccuracy", "F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/location/Coordinates", "<init>", "()V", ())
            .await
    }
    async fn get_horizontal(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "horizontalAccuracy", "F").await
    }
    async fn get_vertical(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "verticalAccuracy", "F").await
    }
}

impl Criteria {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/location/Criteria",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("setAddressInfoRequired", "(Z)V", Self::set_address_info_required, Default::default()),
                JavaMethodProto::new("setAltitudeRequired", "(Z)V", Self::set_altitude_required, Default::default()),
                JavaMethodProto::new("setCostAllowed", "(Z)V", Self::set_cost_allowed, Default::default()),
                JavaMethodProto::new("setHorizontalAccuracy", "(I)V", Self::set_horizontal_accuracy, Default::default()),
                JavaMethodProto::new(
                    "setPreferredPowerConsumption",
                    "(I)V",
                    Self::set_preferred_power_consumption,
                    Default::default(),
                ),
                JavaMethodProto::new("setPreferredResponseTime", "(I)V", Self::set_preferred_response_time, Default::default()),
                JavaMethodProto::new(
                    "setSpeedAndCourseRequired",
                    "(Z)V",
                    Self::set_speed_and_course_required,
                    Default::default(),
                ),
                JavaMethodProto::new("setVerticalAccuracy", "(I)V", Self::set_vertical_accuracy, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    stub_void! {
        set_address_info_required(_required: bool);
        set_altitude_required(_required: bool);
        set_cost_allowed(_allowed: bool);
        set_horizontal_accuracy(_accuracy: i32);
        set_preferred_power_consumption(_level: i32);
        set_preferred_response_time(_time: i32);
        set_speed_and_course_required(_required: bool);
        set_vertical_accuracy(_accuracy: i32);
    }
}

impl Location {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/location/Location",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getCourse", "()F", Self::get_course, Default::default()),
                JavaMethodProto::new(
                    "getQualifiedCoordinates",
                    "()Ljavax/microedition/location/QualifiedCoordinates;",
                    Self::get_qualified_coordinates,
                    Default::default(),
                ),
                JavaMethodProto::new("getSpeed", "()F", Self::get_speed, Default::default()),
                JavaMethodProto::new("getTimestamp", "()J", Self::get_timestamp, Default::default()),
                JavaMethodProto::new("isValid", "()Z", Self::is_valid, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn get_course(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(0.0)
    }
    async fn get_qualified_coordinates(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<QualifiedCoordinates>> {
        Ok(jvm.new_class("javax/microedition/location/QualifiedCoordinates", "()V", ()).await?.into())
    }
    async fn get_speed(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(0.0)
    }
    async fn get_timestamp(jvm: &Jvm, context: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        let _ = jvm;
        Ok(context.now() as i64)
    }
    async fn is_valid(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(true)
    }
}

impl LocationListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/location/LocationListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

impl LocationProvider {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/location/LocationProvider",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getInstance",
                    "(Ljavax/microedition/location/Criteria;)Ljavax/microedition/location/LocationProvider;",
                    Self::get_instance,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getLastKnownLocation",
                    "()Ljavax/microedition/location/Location;",
                    Self::get_last_known_location,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getLocation",
                    "(I)Ljavax/microedition/location/Location;",
                    Self::get_location,
                    Default::default(),
                ),
                JavaMethodProto::new("getState", "()I", Self::get_state, Default::default()),
                JavaMethodProto::new("reset", "()V", Self::reset, Default::default()),
                JavaMethodProto::new(
                    "setLocationListener",
                    "(Ljavax/microedition/location/LocationListener;III)V",
                    Self::set_location_listener,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("AVAILABLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TEMPORARILY_UNAVAILABLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("OUT_OF_SERVICE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext, _criteria: ClassInstanceRef<Criteria>) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("javax/microedition/location/LocationProvider", "()V", ()).await?.into())
    }
    async fn get_last_known_location(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Location>> {
        Ok(jvm.new_class("javax/microedition/location/Location", "()V", ()).await?.into())
    }
    async fn get_location(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _timeout: i32) -> Result<ClassInstanceRef<Location>> {
        Ok(jvm.new_class("javax/microedition/location/Location", "()V", ()).await?.into())
    }
    async fn get_state(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1)
    }
    stub_void! {
        reset();
        set_location_listener(_listener: ClassInstanceRef<LocationListener>, _interval: i32, _timeout: i32, _max_age: i32);
    }
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        Coordinates,
        Criteria,
        Location,
        LocationException,
        LocationListener,
        LocationProvider,
        QualifiedCoordinates,
    ]
}
