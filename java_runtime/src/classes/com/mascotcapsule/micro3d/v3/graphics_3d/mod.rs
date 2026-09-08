use java_class_proto::JavaFieldProto;
use java_constants::FieldAccessFlags;

// class com.mascotcapsule.micro3d.v3.Graphics3D
pub struct Graphics3D;

pub(super) fn static_int_field(name: &str) -> JavaFieldProto {
    JavaFieldProto::new(name, "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL)
}

mod api;
mod draw;
