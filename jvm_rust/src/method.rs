use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::{
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

use classfile::{AttributeInfo, AttributeInfoCode, ConstantPoolReference, FieldMethodref, MethodInfo, Opcode};
use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstance, JavaError, JavaType, JavaValue, Jvm, JvmCallback, Method, ResolvedInstanceField, Result};

use crate::{array_class_instance::ArrayClassInstanceImpl, interpreter::Interpreter, profile};

pub enum MethodBody {
    ByteCode(AttributeInfoCode),
    Rust(Box<dyn JvmCallback>),
}

impl MethodBody {
    pub fn from_rust(callback: Box<dyn JvmCallback>) -> Self {
        Self::Rust(callback)
    }
}

impl Debug for MethodBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MethodBody::ByteCode(_) => write!(f, "ByteCode"),
            MethodBody::Rust(_) => write!(f, "Rust"),
        }
    }
}

#[derive(Debug)]
struct MethodInner {
    name: String,
    descriptor: String,
    frame_name: Arc<str>,
    profile_name: Arc<str>,
    return_type: JavaType,
    body: Option<MethodBody>,
    access_flags: MethodAccessFlags,
}

#[derive(Clone, Debug)]
pub struct MethodImpl {
    inner: Arc<MethodInner>,
}

impl MethodImpl {
    pub fn new(name: &str, descriptor: &str, body: MethodBody, access_flags: MethodAccessFlags) -> Self {
        Self::new_with_profile_name(name, descriptor, body, access_flags, alloc::format!("{name}{descriptor}").into())
    }

    fn new_with_profile_name(name: &str, descriptor: &str, body: MethodBody, access_flags: MethodAccessFlags, profile_name: Arc<str>) -> Self {
        let return_type = JavaType::parse(descriptor).as_method().1.clone();
        let frame_name = alloc::format!("{name}{descriptor}").into();
        Self {
            inner: Arc::new(MethodInner {
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                frame_name,
                profile_name,
                return_type,
                body: Some(body),
                access_flags,
            }),
        }
    }

    pub fn from_method_proto<C, Context>(proto: JavaMethodProto<C>, context: Context) -> Self
    where
        C: ?Sized + 'static + Send,
        Context: Sync + Send + DerefMut + Deref<Target = C> + Clone + 'static,
    {
        Self::from_method_proto_with_owner("", proto, context)
    }

    pub fn from_method_proto_with_owner<C, Context>(owner: &str, proto: JavaMethodProto<C>, context: Context) -> Self
    where
        C: ?Sized + 'static + Send,
        Context: Sync + Send + DerefMut + Deref<Target = C> + Clone + 'static,
    {
        struct MethodProxy<C, Context>
        where
            C: ?Sized,
            Context: Sync + Send + DerefMut + Deref<Target = C> + Clone,
        {
            body: Box<dyn java_class_proto::MethodBody<JavaError, C>>,
            context: Context,
        }

        #[async_trait::async_trait]
        impl<C, Context> JvmCallback for MethodProxy<C, Context>
        where
            C: ?Sized + Send,
            Context: Sync + Send + DerefMut + Deref<Target = C> + Clone,
        {
            async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
                let mut context = self.context.clone();

                self.body.call(jvm, &mut context, args).await
            }
        }

        let profile_name = method_profile_name(owner, &proto.name, &proto.descriptor);

        Self::new_with_profile_name(
            &proto.name,
            &proto.descriptor,
            MethodBody::Rust(Box::new(MethodProxy { body: proto.body, context })),
            proto.access_flags,
            profile_name,
        )
    }

    pub fn from_method_info(method_info: MethodInfo) -> Self {
        Self::from_method_info_with_owner("", method_info)
    }

    pub fn from_method_info_with_owner(owner: &str, method_info: MethodInfo) -> Self {
        let return_type = JavaType::parse(&method_info.descriptor).as_method().1.clone();
        let frame_name = alloc::format!("{}{}", method_info.name, method_info.descriptor).into();
        let profile_name = method_profile_name(owner, &method_info.name, &method_info.descriptor);
        let body = Self::extract_body(method_info.attributes).map(|code| {
            if method_info.descriptor.as_str() == "(J)I" && is_fixed_point_sqrt_long_to_int(&code) {
                profile::record_bytecode_intrinsic_install();
                MethodBody::Rust(Box::new(FixedPointSqrtLongToInt))
            } else if method_info.descriptor.as_str() == "(I)I" && is_fixed_point_inverse_sqrt(&code) {
                profile::record_bytecode_intrinsic_install();
                MethodBody::Rust(Box::new(FixedPointInverseSqrt))
            } else if let Some(intrinsic) = try_fixed_point_vector_array_transform(&code) {
                profile::record_bytecode_intrinsic_install();
                MethodBody::Rust(Box::new(intrinsic))
            } else if let Some(intrinsic) = try_fixed_point_matrix_compose(&code) {
                profile::record_bytecode_intrinsic_install();
                MethodBody::Rust(Box::new(intrinsic))
            } else if let Some(intrinsic) = try_fixed_point_vector_dot(&code) {
                profile::record_bytecode_intrinsic_install();
                MethodBody::Rust(Box::new(intrinsic))
            } else if let Some(intrinsic) = try_fixed_point_vector_normalize(&code) {
                profile::record_bytecode_intrinsic_install();
                MethodBody::Rust(Box::new(intrinsic))
            } else if let Some(intrinsic) = try_fixed_point_int_array_radius(&code) {
                profile::record_bytecode_intrinsic_install();
                MethodBody::Rust(Box::new(intrinsic))
            } else if let Some(intrinsic) = try_fixed_point_matrix_inverse_transform(&code) {
                profile::record_bytecode_intrinsic_install();
                MethodBody::Rust(Box::new(intrinsic))
            } else {
                MethodBody::ByteCode(code)
            }
        });
        Self {
            inner: Arc::new(MethodInner {
                name: method_info.name.to_string(),
                descriptor: method_info.descriptor.to_string(),
                frame_name,
                profile_name,
                return_type,
                body,
                access_flags: method_info.access_flags,
            }),
        }
    }

    fn extract_body(attributes: Vec<AttributeInfo>) -> Option<AttributeInfoCode> {
        for attribute in attributes {
            if let AttributeInfo::Code(x) = attribute {
                return Some(x);
            }
        }

        None
    }
}

#[async_trait::async_trait]
impl Method for MethodImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn descriptor(&self) -> String {
        self.inner.descriptor.clone()
    }

    fn frame_name(&self) -> Arc<str> {
        self.inner.frame_name.clone()
    }

    fn access_flags(&self) -> MethodAccessFlags {
        self.inner.access_flags
    }

    async fn run(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        Ok(match &self.inner.body.as_ref().unwrap() {
            MethodBody::ByteCode(x) => Interpreter::run(jvm, x, args, &self.inner.return_type, self.inner.profile_name.clone()).await?,
            MethodBody::Rust(x) => {
                tracing::trace!(
                    target: "rustjava_native",
                    "native.enter {}{} args={:?}",
                    self.inner.name,
                    self.inner.descriptor,
                    args
                );
                x.call(jvm, args).await?
            }
        })
    }
}

fn method_profile_name(owner: &str, name: &str, descriptor: &str) -> Arc<str> {
    if owner.is_empty() {
        alloc::format!("{name}{descriptor}").into()
    } else {
        alloc::format!("{owner}.{name}{descriptor}").into()
    }
}

struct FixedPointSqrtLongToInt;

struct FixedPointInverseSqrt;

#[async_trait::async_trait]
impl JvmCallback for FixedPointSqrtLongToInt {
    async fn call(&self, _jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::fixed_point_sqrt_intrinsic_call();
        let input: i64 = args[0].clone().into();

        Ok(JavaValue::Int(fixed_point_sqrt_long_to_int(input)))
    }
}

#[async_trait::async_trait]
impl JvmCallback for FixedPointInverseSqrt {
    async fn call(&self, _jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::inverse_sqrt_intrinsic_call();
        let input: i32 = args[0].clone().into();

        Ok(JavaValue::Int(fixed_point_inverse_sqrt(input)))
    }
}

fn fixed_point_sqrt_long_to_int(input: i64) -> i32 {
    if input == 0 {
        return 0;
    }

    let mut step = i32::MAX;
    let mut guess = i64::from(step);

    for _ in (0..=30).rev() {
        step >>= 1;
        let squared = guess.wrapping_mul(guess) >> 12;
        let delta = input.wrapping_sub(squared);

        if delta > 0 {
            guess = guess.wrapping_add(i64::from(step));
        } else if delta < 0 {
            guess = guess.wrapping_sub(i64::from(step));
        } else {
            return guess as i32;
        }
    }

    guess as i32
}

fn fixed_point_inverse_sqrt(input: i32) -> i32 {
    let half = input >> 1;
    let bits = ((input as f32) * 2.4414062e-4f32).to_bits() as i32;
    let guess_bits = 1_597_463_174i32.wrapping_sub(bits >> 1);
    let guess = (f32::from_bits(guess_bits as u32) * 4096.0f32) as i32;
    let correction = 6144i32.wrapping_sub(fixed_mul_shift_i32(fixed_mul_shift_i32(half, guess), guess));
    java_abs_i32(fixed_mul_shift_i32(guess, correction))
}

fn java_abs_i32(value: i32) -> i32 {
    if value < 0 { value.wrapping_neg() } else { value }
}

#[derive(Clone)]
struct Vector3FieldRefs {
    x: FieldMethodref,
    y: FieldMethodref,
    z: FieldMethodref,
}

#[derive(Clone)]
struct Matrix3FieldRefs {
    m00: FieldMethodref,
    m01: FieldMethodref,
    m02: FieldMethodref,
    m10: FieldMethodref,
    m11: FieldMethodref,
    m12: FieldMethodref,
    m20: FieldMethodref,
    m21: FieldMethodref,
    m22: FieldMethodref,
}

struct FixedPointVectorArrayTransform {
    matrix: Matrix3FieldRefs,
    vector: Vector3FieldRefs,
}

struct FixedPointMatrixCompose {
    scale: Vector3FieldRefs,
    matrix: Matrix3FieldRefs,
    translation: Vector3FieldRefs,
    dirty: FieldMethodref,
}

struct FixedPointVectorDot {
    vector: Vector3FieldRefs,
}

struct FixedPointVectorNormalize {
    vector: Vector3FieldRefs,
}

struct FixedPointIntArrayRadius {
    source: FieldMethodref,
    dest: FieldMethodref,
    radius: FieldMethodref,
}

struct FixedPointMatrixInverseTransform {
    scale: Vector3FieldRefs,
    matrix: Matrix3FieldRefs,
    translation: Vector3FieldRefs,
    vector: Vector3FieldRefs,
}

struct ResolvedVector3Fields {
    x: ResolvedInstanceField,
    y: ResolvedInstanceField,
    z: ResolvedInstanceField,
}

struct ResolvedMatrix3Fields {
    m00: ResolvedInstanceField,
    m01: ResolvedInstanceField,
    m02: ResolvedInstanceField,
    m10: ResolvedInstanceField,
    m11: ResolvedInstanceField,
    m12: ResolvedInstanceField,
    m20: ResolvedInstanceField,
    m21: ResolvedInstanceField,
    m22: ResolvedInstanceField,
}

struct ResolvedMatrixComposeFields {
    scale: ResolvedVector3Fields,
    matrix: ResolvedMatrix3Fields,
    translation: ResolvedVector3Fields,
    dirty: ResolvedInstanceField,
}

struct ResolvedMatrixInverseTransformFields {
    scale: ResolvedVector3Fields,
    matrix: ResolvedMatrix3Fields,
    translation: ResolvedVector3Fields,
    vector: ResolvedVector3Fields,
}

#[async_trait::async_trait]
impl JvmCallback for FixedPointVectorArrayTransform {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::vector_array_transform_intrinsic_call();

        let matrix = object_arg(jvm, &args, 0, "matrix").await?;
        let src_array = object_arg(jvm, &args, 1, "source array").await?;
        let dst_array = object_arg(jvm, &args, 2, "destination array").await?;

        let Some(src_array_impl) = src_array.as_any().downcast_ref::<ArrayClassInstanceImpl>() else {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "source argument is not an array")
                .await);
        };
        let Some(dst_array_impl) = dst_array.as_any().downcast_ref::<ArrayClassInstanceImpl>() else {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "destination argument is not an array")
                .await);
        };

        let matrix_fields = self.resolve_matrix_fields(jvm).await?;
        let mut vector_fields = self.resolve_vector_fields(jvm).await?;

        let m00 = int_field(jvm, &matrix_fields.m00, &matrix)?;
        let m01 = int_field(jvm, &matrix_fields.m01, &matrix)?;
        let m02 = int_field(jvm, &matrix_fields.m02, &matrix)?;
        let m10 = int_field(jvm, &matrix_fields.m10, &matrix)?;
        let m11 = int_field(jvm, &matrix_fields.m11, &matrix)?;
        let m12 = int_field(jvm, &matrix_fields.m12, &matrix)?;
        let m20 = int_field(jvm, &matrix_fields.m20, &matrix)?;
        let m21 = int_field(jvm, &matrix_fields.m21, &matrix)?;
        let m22 = int_field(jvm, &matrix_fields.m22, &matrix)?;

        for index in (0..dst_array_impl.len()).rev() {
            let src = object_from_value(jvm, src_array_impl.load_one_stack(index), "source vector").await?;
            let mut dst = object_from_value(jvm, dst_array_impl.load_one_stack(index), "destination vector").await?;

            let (x, y, z) = vector_values(jvm, &vector_fields, &src)?;
            let out_x = transform_row(m00, m01, m02, x, y, z);
            set_int_field(jvm, &mut vector_fields.x, &mut dst, out_x)?;

            let (x, y, z) = vector_values(jvm, &vector_fields, &src)?;
            let out_y = transform_row(m10, m11, m12, x, y, z);
            set_int_field(jvm, &mut vector_fields.y, &mut dst, out_y)?;

            let (x, y, z) = vector_values(jvm, &vector_fields, &src)?;
            let out_z = transform_row(m20, m21, m22, x, y, z);
            set_int_field(jvm, &mut vector_fields.z, &mut dst, out_z)?;
        }

        Ok(JavaValue::Object(Some(dst_array)))
    }
}

#[async_trait::async_trait]
impl JvmCallback for FixedPointMatrixCompose {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::matrix_compose_intrinsic_call();

        let lhs = object_arg(jvm, &args, 0, "left matrix").await?;
        let rhs = object_arg(jvm, &args, 1, "right matrix").await?;
        let mut out = object_arg(jvm, &args, 2, "output matrix").await?;
        let mut fields = self.resolve_fields(jvm).await?;

        let out_x = int_field(jvm, &fields.translation.x, &lhs)?.wrapping_add(fixed_mul_shift_i32(
            int_field(jvm, &fields.scale.x, &lhs)?,
            transform_row(
                int_field(jvm, &fields.matrix.m00, &lhs)?,
                int_field(jvm, &fields.matrix.m01, &lhs)?,
                int_field(jvm, &fields.matrix.m02, &lhs)?,
                int_field(jvm, &fields.translation.x, &rhs)?,
                int_field(jvm, &fields.translation.y, &rhs)?,
                int_field(jvm, &fields.translation.z, &rhs)?,
            ),
        ));
        set_int_field(jvm, &mut fields.translation.x, &mut out, out_x)?;

        let out_y = int_field(jvm, &fields.translation.y, &lhs)?.wrapping_add(fixed_mul_shift_i32(
            int_field(jvm, &fields.scale.y, &lhs)?,
            transform_row(
                int_field(jvm, &fields.matrix.m10, &lhs)?,
                int_field(jvm, &fields.matrix.m11, &lhs)?,
                int_field(jvm, &fields.matrix.m12, &lhs)?,
                int_field(jvm, &fields.translation.x, &rhs)?,
                int_field(jvm, &fields.translation.y, &rhs)?,
                int_field(jvm, &fields.translation.z, &rhs)?,
            ),
        ));
        set_int_field(jvm, &mut fields.translation.y, &mut out, out_y)?;

        let out_z = int_field(jvm, &fields.translation.z, &lhs)?.wrapping_add(fixed_mul_shift_i32(
            int_field(jvm, &fields.scale.z, &lhs)?,
            transform_row(
                int_field(jvm, &fields.matrix.m20, &lhs)?,
                int_field(jvm, &fields.matrix.m21, &lhs)?,
                int_field(jvm, &fields.matrix.m22, &lhs)?,
                int_field(jvm, &fields.translation.x, &rhs)?,
                int_field(jvm, &fields.translation.y, &rhs)?,
                int_field(jvm, &fields.translation.z, &rhs)?,
            ),
        ));
        set_int_field(jvm, &mut fields.translation.z, &mut out, out_z)?;

        let out_scale_x = fixed_mul_shift_i32(int_field(jvm, &fields.scale.x, &lhs)?, int_field(jvm, &fields.scale.x, &rhs)?);
        set_int_field(jvm, &mut fields.scale.x, &mut out, out_scale_x)?;
        let out_scale_y = fixed_mul_shift_i32(int_field(jvm, &fields.scale.y, &lhs)?, int_field(jvm, &fields.scale.y, &rhs)?);
        set_int_field(jvm, &mut fields.scale.y, &mut out, out_scale_y)?;
        let out_scale_z = fixed_mul_shift_i32(int_field(jvm, &fields.scale.z, &lhs)?, int_field(jvm, &fields.scale.z, &rhs)?);
        set_int_field(jvm, &mut fields.scale.z, &mut out, out_scale_z)?;

        let out_m00 = transform_row(
            int_field(jvm, &fields.matrix.m00, &lhs)?,
            int_field(jvm, &fields.matrix.m01, &lhs)?,
            int_field(jvm, &fields.matrix.m02, &lhs)?,
            int_field(jvm, &fields.matrix.m00, &rhs)?,
            int_field(jvm, &fields.matrix.m10, &rhs)?,
            int_field(jvm, &fields.matrix.m20, &rhs)?,
        );
        set_int_field(jvm, &mut fields.matrix.m00, &mut out, out_m00)?;

        let out_m10 = transform_row(
            int_field(jvm, &fields.matrix.m10, &lhs)?,
            int_field(jvm, &fields.matrix.m11, &lhs)?,
            int_field(jvm, &fields.matrix.m12, &lhs)?,
            int_field(jvm, &fields.matrix.m00, &rhs)?,
            int_field(jvm, &fields.matrix.m10, &rhs)?,
            int_field(jvm, &fields.matrix.m20, &rhs)?,
        );
        set_int_field(jvm, &mut fields.matrix.m10, &mut out, out_m10)?;

        let out_m20 = transform_row(
            int_field(jvm, &fields.matrix.m20, &lhs)?,
            int_field(jvm, &fields.matrix.m21, &lhs)?,
            int_field(jvm, &fields.matrix.m22, &lhs)?,
            int_field(jvm, &fields.matrix.m00, &rhs)?,
            int_field(jvm, &fields.matrix.m10, &rhs)?,
            int_field(jvm, &fields.matrix.m20, &rhs)?,
        );
        set_int_field(jvm, &mut fields.matrix.m20, &mut out, out_m20)?;

        let out_m01 = transform_row(
            int_field(jvm, &fields.matrix.m00, &lhs)?,
            int_field(jvm, &fields.matrix.m01, &lhs)?,
            int_field(jvm, &fields.matrix.m02, &lhs)?,
            int_field(jvm, &fields.matrix.m01, &rhs)?,
            int_field(jvm, &fields.matrix.m11, &rhs)?,
            int_field(jvm, &fields.matrix.m21, &rhs)?,
        );
        set_int_field(jvm, &mut fields.matrix.m01, &mut out, out_m01)?;

        let out_m11 = transform_row(
            int_field(jvm, &fields.matrix.m10, &lhs)?,
            int_field(jvm, &fields.matrix.m11, &lhs)?,
            int_field(jvm, &fields.matrix.m12, &lhs)?,
            int_field(jvm, &fields.matrix.m01, &rhs)?,
            int_field(jvm, &fields.matrix.m11, &rhs)?,
            int_field(jvm, &fields.matrix.m21, &rhs)?,
        );
        set_int_field(jvm, &mut fields.matrix.m11, &mut out, out_m11)?;

        let out_m21 = transform_row(
            int_field(jvm, &fields.matrix.m20, &lhs)?,
            int_field(jvm, &fields.matrix.m21, &lhs)?,
            int_field(jvm, &fields.matrix.m22, &lhs)?,
            int_field(jvm, &fields.matrix.m01, &rhs)?,
            int_field(jvm, &fields.matrix.m11, &rhs)?,
            int_field(jvm, &fields.matrix.m21, &rhs)?,
        );
        set_int_field(jvm, &mut fields.matrix.m21, &mut out, out_m21)?;

        let out_m02 = transform_row(
            int_field(jvm, &fields.matrix.m00, &lhs)?,
            int_field(jvm, &fields.matrix.m01, &lhs)?,
            int_field(jvm, &fields.matrix.m02, &lhs)?,
            int_field(jvm, &fields.matrix.m02, &rhs)?,
            int_field(jvm, &fields.matrix.m12, &rhs)?,
            int_field(jvm, &fields.matrix.m22, &rhs)?,
        );
        set_int_field(jvm, &mut fields.matrix.m02, &mut out, out_m02)?;

        let out_m12 = transform_row(
            int_field(jvm, &fields.matrix.m10, &lhs)?,
            int_field(jvm, &fields.matrix.m11, &lhs)?,
            int_field(jvm, &fields.matrix.m12, &lhs)?,
            int_field(jvm, &fields.matrix.m02, &rhs)?,
            int_field(jvm, &fields.matrix.m12, &rhs)?,
            int_field(jvm, &fields.matrix.m22, &rhs)?,
        );
        set_int_field(jvm, &mut fields.matrix.m12, &mut out, out_m12)?;

        let out_m22 = transform_row(
            int_field(jvm, &fields.matrix.m20, &lhs)?,
            int_field(jvm, &fields.matrix.m21, &lhs)?,
            int_field(jvm, &fields.matrix.m22, &lhs)?,
            int_field(jvm, &fields.matrix.m02, &rhs)?,
            int_field(jvm, &fields.matrix.m12, &rhs)?,
            int_field(jvm, &fields.matrix.m22, &rhs)?,
        );
        set_int_field(jvm, &mut fields.matrix.m22, &mut out, out_m22)?;

        set_bool_field(jvm, &mut fields.dirty, &mut out, true)?;

        Ok(JavaValue::Object(Some(out)))
    }
}

#[async_trait::async_trait]
impl JvmCallback for FixedPointVectorDot {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::vector_dot_intrinsic_call();

        let lhs = object_arg(jvm, &args, 0, "left vector").await?;
        let rhs = object_arg(jvm, &args, 1, "right vector").await?;
        let fields = self.resolve_vector_fields(jvm).await?;
        let (lhs_x, lhs_y, lhs_z) = vector_values(jvm, &fields, &lhs)?;
        let (rhs_x, rhs_y, rhs_z) = vector_values(jvm, &fields, &rhs)?;

        Ok(JavaValue::Int(transform_row(lhs_x, lhs_y, lhs_z, rhs_x, rhs_y, rhs_z)))
    }
}

#[async_trait::async_trait]
impl JvmCallback for FixedPointVectorNormalize {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::vector_normalize_intrinsic_call();

        let mut vector = object_arg(jvm, &args, 0, "vector").await?;
        let mut fields = self.resolve_vector_fields(jvm).await?;
        let (x, y, z) = vector_values(jvm, &fields, &vector)?;
        let scale = fixed_point_inverse_sqrt(fixed_point_length_squared_i32(x, y, z));

        set_int_field(jvm, &mut fields.x, &mut vector, fixed_mul_shift_i32(x, scale))?;
        set_int_field(jvm, &mut fields.y, &mut vector, fixed_mul_shift_i32(y, scale))?;
        set_int_field(jvm, &mut fields.z, &mut vector, fixed_mul_shift_i32(z, scale))?;

        Ok(JavaValue::Void)
    }
}

#[async_trait::async_trait]
impl JvmCallback for FixedPointIntArrayRadius {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::int_array_radius_intrinsic_call();

        let mut this = object_arg(jvm, &args, 0, "bounds").await?;
        let source = object_arg(jvm, &args, 1, "source int array").await?;
        let dest = object_arg(jvm, &args, 2, "destination int array").await?;
        let mut source_field = resolve_field_ref(jvm, &self.source).await?;
        let mut dest_field = resolve_field_ref(jvm, &self.dest).await?;
        let mut radius_field = resolve_field_ref(jvm, &self.radius).await?;

        set_object_field(jvm, &mut source_field, &mut this, source.clone())?;
        set_object_field(jvm, &mut dest_field, &mut this, dest)?;

        let Some(source_array) = source.as_any().downcast_ref::<ArrayClassInstanceImpl>() else {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "source argument is not an int array")
                .await);
        };

        let mut max_squared = 0i32;
        let mut index = 0usize;
        while index < source_array.len() {
            let x: i32 = source_array.load_one_stack(index).into();
            let y: i32 = source_array.load_one_stack(index + 1).into();
            let z: i32 = source_array.load_one_stack(index + 2).into();
            let squared = x.wrapping_mul(x).wrapping_add(y.wrapping_mul(y)).wrapping_add(z.wrapping_mul(z));
            if max_squared < squared {
                max_squared = squared;
            }
            index += 3;
        }

        set_int_field(jvm, &mut radius_field, &mut this, fixed_point_sqrt_long_to_int(i64::from(max_squared)))?;

        Ok(JavaValue::Void)
    }
}

#[async_trait::async_trait]
impl JvmCallback for FixedPointMatrixInverseTransform {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::matrix_inverse_transform_intrinsic_call();

        let matrix = object_arg(jvm, &args, 0, "matrix").await?;
        let mut vector = object_arg(jvm, &args, 1, "vector").await?;
        let mut fields = self.resolve_fields(jvm).await?;

        let inv_x = inverse_scale(jvm, int_field(jvm, &fields.scale.x, &matrix)?).await?;
        let inv_y = inverse_scale(jvm, int_field(jvm, &fields.scale.y, &matrix)?).await?;
        let inv_z = inverse_scale(jvm, int_field(jvm, &fields.scale.z, &matrix)?).await?;

        let trans_x = int_field(jvm, &fields.translation.x, &matrix)?;
        let trans_y = int_field(jvm, &fields.translation.y, &matrix)?;
        let trans_z = int_field(jvm, &fields.translation.z, &matrix)?;

        let offset_x = fixed_mul_shift_i32(
            inv_x,
            fixed_mul_shift_i32(int_field(jvm, &fields.matrix.m00, &matrix)?, trans_x)
                .wrapping_neg()
                .wrapping_sub(fixed_mul_shift_i32(int_field(jvm, &fields.matrix.m10, &matrix)?, trans_y))
                .wrapping_sub(fixed_mul_shift_i32(int_field(jvm, &fields.matrix.m20, &matrix)?, trans_z)),
        );
        let offset_y = fixed_mul_shift_i32(
            inv_y,
            fixed_mul_shift_i32(int_field(jvm, &fields.matrix.m01, &matrix)?, trans_x)
                .wrapping_neg()
                .wrapping_sub(fixed_mul_shift_i32(int_field(jvm, &fields.matrix.m11, &matrix)?, trans_y))
                .wrapping_sub(fixed_mul_shift_i32(int_field(jvm, &fields.matrix.m21, &matrix)?, trans_z)),
        );
        let offset_z = fixed_mul_shift_i32(
            inv_z,
            fixed_mul_shift_i32(int_field(jvm, &fields.matrix.m02, &matrix)?, trans_x)
                .wrapping_neg()
                .wrapping_sub(fixed_mul_shift_i32(int_field(jvm, &fields.matrix.m12, &matrix)?, trans_y))
                .wrapping_sub(fixed_mul_shift_i32(int_field(jvm, &fields.matrix.m22, &matrix)?, trans_z)),
        );

        let (x, y, z) = vector_values(jvm, &fields.vector, &vector)?;

        let out_x = fixed_mul_shift_i32(fixed_mul_shift_i32(inv_x, int_field(jvm, &fields.matrix.m00, &matrix)?), x)
            .wrapping_add(fixed_mul_shift_i32(
                fixed_mul_shift_i32(inv_y, int_field(jvm, &fields.matrix.m10, &matrix)?),
                y,
            ))
            .wrapping_add(fixed_mul_shift_i32(
                fixed_mul_shift_i32(inv_z, int_field(jvm, &fields.matrix.m20, &matrix)?),
                z,
            ))
            .wrapping_add(offset_x);
        set_int_field(jvm, &mut fields.vector.x, &mut vector, out_x)?;

        let out_y = fixed_mul_shift_i32(fixed_mul_shift_i32(inv_x, int_field(jvm, &fields.matrix.m01, &matrix)?), x)
            .wrapping_add(fixed_mul_shift_i32(
                fixed_mul_shift_i32(inv_y, int_field(jvm, &fields.matrix.m11, &matrix)?),
                y,
            ))
            .wrapping_add(fixed_mul_shift_i32(
                fixed_mul_shift_i32(inv_z, int_field(jvm, &fields.matrix.m21, &matrix)?),
                z,
            ))
            .wrapping_add(offset_y);
        set_int_field(jvm, &mut fields.vector.y, &mut vector, out_y)?;

        let out_z = fixed_mul_shift_i32(fixed_mul_shift_i32(inv_x, int_field(jvm, &fields.matrix.m02, &matrix)?), x)
            .wrapping_add(fixed_mul_shift_i32(
                fixed_mul_shift_i32(inv_y, int_field(jvm, &fields.matrix.m12, &matrix)?),
                y,
            ))
            .wrapping_add(fixed_mul_shift_i32(
                fixed_mul_shift_i32(inv_z, int_field(jvm, &fields.matrix.m22, &matrix)?),
                z,
            ))
            .wrapping_add(offset_z);
        set_int_field(jvm, &mut fields.vector.z, &mut vector, out_z)?;

        Ok(JavaValue::Object(Some(vector)))
    }
}

impl FixedPointVectorArrayTransform {
    async fn resolve_matrix_fields(&self, jvm: &Jvm) -> Result<ResolvedMatrix3Fields> {
        Ok(ResolvedMatrix3Fields {
            m00: resolve_field_ref(jvm, &self.matrix.m00).await?,
            m01: resolve_field_ref(jvm, &self.matrix.m01).await?,
            m02: resolve_field_ref(jvm, &self.matrix.m02).await?,
            m10: resolve_field_ref(jvm, &self.matrix.m10).await?,
            m11: resolve_field_ref(jvm, &self.matrix.m11).await?,
            m12: resolve_field_ref(jvm, &self.matrix.m12).await?,
            m20: resolve_field_ref(jvm, &self.matrix.m20).await?,
            m21: resolve_field_ref(jvm, &self.matrix.m21).await?,
            m22: resolve_field_ref(jvm, &self.matrix.m22).await?,
        })
    }

    async fn resolve_vector_fields(&self, jvm: &Jvm) -> Result<ResolvedVector3Fields> {
        Ok(ResolvedVector3Fields {
            x: resolve_field_ref(jvm, &self.vector.x).await?,
            y: resolve_field_ref(jvm, &self.vector.y).await?,
            z: resolve_field_ref(jvm, &self.vector.z).await?,
        })
    }
}

impl FixedPointMatrixCompose {
    async fn resolve_fields(&self, jvm: &Jvm) -> Result<ResolvedMatrixComposeFields> {
        Ok(ResolvedMatrixComposeFields {
            scale: ResolvedVector3Fields {
                x: resolve_field_ref(jvm, &self.scale.x).await?,
                y: resolve_field_ref(jvm, &self.scale.y).await?,
                z: resolve_field_ref(jvm, &self.scale.z).await?,
            },
            matrix: ResolvedMatrix3Fields {
                m00: resolve_field_ref(jvm, &self.matrix.m00).await?,
                m01: resolve_field_ref(jvm, &self.matrix.m01).await?,
                m02: resolve_field_ref(jvm, &self.matrix.m02).await?,
                m10: resolve_field_ref(jvm, &self.matrix.m10).await?,
                m11: resolve_field_ref(jvm, &self.matrix.m11).await?,
                m12: resolve_field_ref(jvm, &self.matrix.m12).await?,
                m20: resolve_field_ref(jvm, &self.matrix.m20).await?,
                m21: resolve_field_ref(jvm, &self.matrix.m21).await?,
                m22: resolve_field_ref(jvm, &self.matrix.m22).await?,
            },
            translation: ResolvedVector3Fields {
                x: resolve_field_ref(jvm, &self.translation.x).await?,
                y: resolve_field_ref(jvm, &self.translation.y).await?,
                z: resolve_field_ref(jvm, &self.translation.z).await?,
            },
            dirty: resolve_field_ref(jvm, &self.dirty).await?,
        })
    }
}

impl FixedPointVectorDot {
    async fn resolve_vector_fields(&self, jvm: &Jvm) -> Result<ResolvedVector3Fields> {
        Ok(ResolvedVector3Fields {
            x: resolve_field_ref(jvm, &self.vector.x).await?,
            y: resolve_field_ref(jvm, &self.vector.y).await?,
            z: resolve_field_ref(jvm, &self.vector.z).await?,
        })
    }
}

impl FixedPointVectorNormalize {
    async fn resolve_vector_fields(&self, jvm: &Jvm) -> Result<ResolvedVector3Fields> {
        Ok(ResolvedVector3Fields {
            x: resolve_field_ref(jvm, &self.vector.x).await?,
            y: resolve_field_ref(jvm, &self.vector.y).await?,
            z: resolve_field_ref(jvm, &self.vector.z).await?,
        })
    }
}

impl FixedPointMatrixInverseTransform {
    async fn resolve_fields(&self, jvm: &Jvm) -> Result<ResolvedMatrixInverseTransformFields> {
        Ok(ResolvedMatrixInverseTransformFields {
            scale: ResolvedVector3Fields {
                x: resolve_field_ref(jvm, &self.scale.x).await?,
                y: resolve_field_ref(jvm, &self.scale.y).await?,
                z: resolve_field_ref(jvm, &self.scale.z).await?,
            },
            matrix: ResolvedMatrix3Fields {
                m00: resolve_field_ref(jvm, &self.matrix.m00).await?,
                m01: resolve_field_ref(jvm, &self.matrix.m01).await?,
                m02: resolve_field_ref(jvm, &self.matrix.m02).await?,
                m10: resolve_field_ref(jvm, &self.matrix.m10).await?,
                m11: resolve_field_ref(jvm, &self.matrix.m11).await?,
                m12: resolve_field_ref(jvm, &self.matrix.m12).await?,
                m20: resolve_field_ref(jvm, &self.matrix.m20).await?,
                m21: resolve_field_ref(jvm, &self.matrix.m21).await?,
                m22: resolve_field_ref(jvm, &self.matrix.m22).await?,
            },
            translation: ResolvedVector3Fields {
                x: resolve_field_ref(jvm, &self.translation.x).await?,
                y: resolve_field_ref(jvm, &self.translation.y).await?,
                z: resolve_field_ref(jvm, &self.translation.z).await?,
            },
            vector: ResolvedVector3Fields {
                x: resolve_field_ref(jvm, &self.vector.x).await?,
                y: resolve_field_ref(jvm, &self.vector.y).await?,
                z: resolve_field_ref(jvm, &self.vector.z).await?,
            },
        })
    }
}

async fn resolve_field_ref(jvm: &Jvm, field: &FieldMethodref) -> Result<ResolvedInstanceField> {
    jvm.resolve_instance_field(&field.class, &field.name, &field.descriptor).await
}

async fn object_arg(jvm: &Jvm, args: &[JavaValue], index: usize, label: &str) -> Result<Box<dyn ClassInstance>> {
    let Some(value) = args.get(index).cloned() else {
        return Err(jvm
            .exception("java/lang/IllegalArgumentException", &alloc::format!("missing {label} argument"))
            .await);
    };
    object_from_value(jvm, value, label).await
}

async fn object_from_value(jvm: &Jvm, value: JavaValue, label: &str) -> Result<Box<dyn ClassInstance>> {
    match value {
        JavaValue::Object(Some(object)) => Ok(object),
        JavaValue::Object(None) => Err(jvm.exception("java/lang/NullPointerException", label).await),
        _ => Err(jvm
            .exception("java/lang/IllegalArgumentException", &alloc::format!("{label} is not an object"))
            .await),
    }
}

fn vector_values(jvm: &Jvm, fields: &ResolvedVector3Fields, vector: &Box<dyn ClassInstance>) -> Result<(i32, i32, i32)> {
    Ok((
        int_field(jvm, &fields.x, vector)?,
        int_field(jvm, &fields.y, vector)?,
        int_field(jvm, &fields.z, vector)?,
    ))
}

fn int_field(jvm: &Jvm, field: &ResolvedInstanceField, object: &Box<dyn ClassInstance>) -> Result<i32> {
    Ok(jvm.get_resolved_instance_field(field, object)?.into())
}

fn set_int_field(jvm: &Jvm, field: &mut ResolvedInstanceField, object: &mut Box<dyn ClassInstance>, value: i32) -> Result<()> {
    jvm.put_resolved_instance_field(field, object, JavaValue::Int(value))
}

fn set_bool_field(jvm: &Jvm, field: &mut ResolvedInstanceField, object: &mut Box<dyn ClassInstance>, value: bool) -> Result<()> {
    jvm.put_resolved_instance_field(field, object, JavaValue::Boolean(value))
}

fn set_object_field(jvm: &Jvm, field: &mut ResolvedInstanceField, object: &mut Box<dyn ClassInstance>, value: Box<dyn ClassInstance>) -> Result<()> {
    jvm.put_resolved_instance_field(field, object, JavaValue::Object(Some(value)))
}

fn transform_row(m0: i32, m1: i32, m2: i32, x: i32, y: i32, z: i32) -> i32 {
    fixed_mul_shift_i32(m0, x)
        .wrapping_add(fixed_mul_shift_i32(m1, y))
        .wrapping_add(fixed_mul_shift_i32(m2, z))
}

fn fixed_mul_shift_i32(lhs: i32, rhs: i32) -> i32 {
    lhs.wrapping_mul(rhs) >> 12
}

async fn inverse_scale(jvm: &Jvm, scale: i32) -> Result<i32> {
    if scale == 0 {
        return Err(jvm.exception("java/lang/ArithmeticException", "Division by zero").await);
    }
    Ok(16_777_216i32.wrapping_div(scale))
}

fn fixed_point_length_squared_i32(x: i32, y: i32, z: i32) -> i32 {
    (((i64::from(x) * i64::from(x)) >> 12)
        .wrapping_add((i64::from(y) * i64::from(y)) >> 12)
        .wrapping_add((i64::from(z) * i64::from(z)) >> 12)) as i32
}

struct VectorArrayTransformRow {
    matrix: Vector3FieldRefs,
    vector: Vector3FieldRefs,
    output: FieldMethodref,
}

struct MatrixComposeTranslationRow {
    translation: FieldMethodref,
    scale: FieldMethodref,
    matrix: Vector3FieldRefs,
    rhs_translation: Vector3FieldRefs,
    output: FieldMethodref,
}

struct MatrixComposeScaleAssign {
    lhs: FieldMethodref,
    rhs: FieldMethodref,
    output: FieldMethodref,
}

struct MatrixComposeMatrixAssign {
    lhs: Vector3FieldRefs,
    rhs: Vector3FieldRefs,
    output: FieldMethodref,
}

struct MatrixInverseOffset {
    matrix_column: Vector3FieldRefs,
    translation: Vector3FieldRefs,
}

struct MatrixInverseOutput {
    matrix_column: Vector3FieldRefs,
    output: FieldMethodref,
}

fn try_fixed_point_vector_array_transform(code: &AttributeInfoCode) -> Option<FixedPointVectorArrayTransform> {
    let ops = &code.code_sequence;
    let mut index = 0usize;

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(2)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Arraylength))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iconst(1)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Isub))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Istore(3)))?;

    let loop_start_offset = expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(3)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iflt(_)))?;

    let row_x = parse_vector_array_transform_row(ops, &mut index)?;
    let row_y = parse_vector_array_transform_row(ops, &mut index)?;
    let row_z = parse_vector_array_transform_row(ops, &mut index)?;

    if !same_vector_fields(&row_x.vector, &row_y.vector) || !same_vector_fields(&row_x.vector, &row_z.vector) {
        return None;
    }
    if !same_field_ref(&row_x.output, &row_x.vector.x)
        || !same_field_ref(&row_y.output, &row_x.vector.y)
        || !same_field_ref(&row_z.output, &row_x.vector.z)
    {
        return None;
    }

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iinc(3, -1)))?;
    let goto_offset = expect_op(ops, &mut index, |op| matches!(op, Opcode::Goto(_)))?;
    let expected_goto = loop_start_offset as i32 - goto_offset as i32;
    if !matches!(&ops[index - 1].1, Opcode::Goto(offset) if i32::from(*offset) == expected_goto) {
        return None;
    }
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(2)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Areturn))?;

    if index != ops.len() {
        return None;
    }

    Some(FixedPointVectorArrayTransform {
        matrix: Matrix3FieldRefs {
            m00: row_x.matrix.x,
            m01: row_x.matrix.y,
            m02: row_x.matrix.z,
            m10: row_y.matrix.x,
            m11: row_y.matrix.y,
            m12: row_y.matrix.z,
            m20: row_z.matrix.x,
            m21: row_z.matrix.y,
            m22: row_z.matrix.z,
        },
        vector: row_x.vector,
    })
}

fn parse_vector_array_transform_row(ops: &[(u32, Opcode)], index: &mut usize) -> Option<VectorArrayTransformRow> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(2)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iload(3)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aaload))?;

    let (matrix_x, vector_x) = parse_vector_array_transform_term(ops, index)?;
    let (matrix_y, vector_y) = parse_vector_array_transform_term(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    let (matrix_z, vector_z) = parse_vector_array_transform_term(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    let output = expect_putfield(ops, index)?;

    Some(VectorArrayTransformRow {
        matrix: Vector3FieldRefs {
            x: matrix_x,
            y: matrix_y,
            z: matrix_z,
        },
        vector: Vector3FieldRefs {
            x: vector_x,
            y: vector_y,
            z: vector_z,
        },
        output,
    })
}

fn parse_vector_array_transform_term(ops: &[(u32, Opcode)], index: &mut usize) -> Option<(FieldMethodref, FieldMethodref)> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let matrix = expect_getfield(ops, index)?;
    if matrix.descriptor.as_str() != "I" {
        return None;
    }

    expect_op(ops, index, |op| matches!(op, Opcode::Aload(1)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iload(3)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aaload))?;
    let vector = expect_getfield(ops, index)?;
    if vector.descriptor.as_str() != "I" {
        return None;
    }

    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;

    Some((matrix, vector))
}

fn try_fixed_point_vector_dot(code: &AttributeInfoCode) -> Option<FixedPointVectorDot> {
    let ops = &code.code_sequence;
    let mut index = 0usize;

    let (lhs_x, rhs_x) = parse_vector_dot_term(ops, &mut index)?;
    let (lhs_y, rhs_y) = parse_vector_dot_term(ops, &mut index)?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iadd))?;
    let (lhs_z, rhs_z) = parse_vector_dot_term(ops, &mut index)?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iadd))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Ireturn))?;

    if index != ops.len() || !same_field_ref(&lhs_x, &rhs_x) || !same_field_ref(&lhs_y, &rhs_y) || !same_field_ref(&lhs_z, &rhs_z) {
        return None;
    }

    Some(FixedPointVectorDot {
        vector: Vector3FieldRefs {
            x: lhs_x,
            y: lhs_y,
            z: lhs_z,
        },
    })
}

fn parse_vector_dot_term(ops: &[(u32, Opcode)], index: &mut usize) -> Option<(FieldMethodref, FieldMethodref)> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let lhs = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(1)))?;
    let rhs = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;

    Some((lhs, rhs))
}

fn try_fixed_point_vector_normalize(code: &AttributeInfoCode) -> Option<FixedPointVectorNormalize> {
    let ops = &code.code_sequence;
    let mut index = 0usize;

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Astore(2)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Dup))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Astore(1)))?;

    let x = parse_vector_normalize_first_square(ops, &mut index)?;
    let y = parse_vector_normalize_next_square(ops, &mut index)?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Ladd))?;
    let z = parse_vector_normalize_next_square(ops, &mut index)?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Ladd))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::L2i))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Invokestatic(_)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Istore(3)))?;

    parse_vector_normalize_store(ops, &mut index, &x)?;
    parse_vector_normalize_store(ops, &mut index, &y)?;
    parse_vector_normalize_store(ops, &mut index, &z)?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(2)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Pop))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Return))?;

    if index != ops.len() {
        return None;
    }

    Some(FixedPointVectorNormalize {
        vector: Vector3FieldRefs { x, y, z },
    })
}

fn parse_vector_normalize_first_square(ops: &[(u32, Opcode)], index: &mut usize) -> Option<FieldMethodref> {
    let first = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::I2l))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(1)))?;
    let second = expect_int_getfield(ops, index)?;
    if !same_field_ref(&first, &second) {
        return None;
    }
    expect_op(ops, index, |op| matches!(op, Opcode::I2l))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Lmul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Lshr))?;
    Some(first)
}

fn parse_vector_normalize_next_square(ops: &[(u32, Opcode)], index: &mut usize) -> Option<FieldMethodref> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(1)))?;
    parse_vector_normalize_first_square(ops, index)
}

fn parse_vector_normalize_store(ops: &[(u32, Opcode)], index: &mut usize, expected_field: &FieldMethodref) -> Option<()> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(2)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(1)))?;
    let input = expect_int_getfield(ops, index)?;
    if !same_field_ref(&input, expected_field) {
        return None;
    }
    expect_op(ops, index, |op| matches!(op, Opcode::Iload(3)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;
    let output = expect_putfield(ops, index)?;
    if !same_field_ref(&output, expected_field) {
        return None;
    }
    Some(())
}

fn try_fixed_point_int_array_radius(code: &AttributeInfoCode) -> Option<FixedPointIntArrayRadius> {
    let ops = &code.code_sequence;
    let mut index = 0usize;

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(1)))?;
    let source = expect_putfield(ops, &mut index)?;
    if source.descriptor.as_str() != "[I" {
        return None;
    }

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(2)))?;
    let dest = expect_putfield(ops, &mut index)?;
    if dest.descriptor.as_str() != "[I" {
        return None;
    }

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iconst(0)))?;
    let radius = expect_putfield(ops, &mut index)?;
    if radius.descriptor.as_str() != "I" {
        return None;
    }

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iconst(0)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Istore(1)))?;
    let loop_start = expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(1)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    let length_source = expect_getfield(ops, &mut index)?;
    if !same_field_ref(&length_source, &source) {
        return None;
    }
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Arraylength))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::IfIcmpge(_)))?;

    parse_int_array_radius_component_square(ops, &mut index, &source, 0)?;
    parse_int_array_radius_component_square(ops, &mut index, &source, 1)?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iadd))?;
    parse_int_array_radius_component_square(ops, &mut index, &source, 2)?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iadd))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Istore(2)))?;

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    let compare_radius = expect_int_getfield(ops, &mut index)?;
    if !same_field_ref(&compare_radius, &radius) {
        return None;
    }
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(2)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::IfIcmpge(_)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(2)))?;
    let update_radius = expect_putfield(ops, &mut index)?;
    if !same_field_ref(&update_radius, &radius) {
        return None;
    }

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iinc(1, 3)))?;
    let goto_offset = expect_op(ops, &mut index, |op| matches!(op, Opcode::Goto(_)))?;
    let expected_goto = loop_start as i32 - goto_offset as i32;
    if !matches!(&ops[index - 1].1, Opcode::Goto(offset) if i32::from(*offset) == expected_goto) {
        return None;
    }

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(0)))?;
    let sqrt_radius = expect_int_getfield(ops, &mut index)?;
    if !same_field_ref(&sqrt_radius, &radius) {
        return None;
    }
    expect_op(ops, &mut index, |op| matches!(op, Opcode::I2l))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Invokestatic(_)))?;
    let output_radius = expect_putfield(ops, &mut index)?;
    if !same_field_ref(&output_radius, &radius) {
        return None;
    }
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Return))?;

    if index != ops.len() {
        return None;
    }

    Some(FixedPointIntArrayRadius { source, dest, radius })
}

fn parse_int_array_radius_component_square(ops: &[(u32, Opcode)], index: &mut usize, source: &FieldMethodref, addend: i32) -> Option<()> {
    parse_int_array_radius_component_load(ops, index, source, addend)?;
    parse_int_array_radius_component_load(ops, index, source, addend)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    Some(())
}

fn parse_int_array_radius_component_load(ops: &[(u32, Opcode)], index: &mut usize, source: &FieldMethodref, addend: i32) -> Option<()> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let field = expect_getfield(ops, index)?;
    if !same_field_ref(&field, source) {
        return None;
    }
    expect_op(ops, index, |op| matches!(op, Opcode::Iload(1)))?;
    if addend > 0 {
        expect_iconst(ops, index, addend)?;
        expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    }
    expect_op(ops, index, |op| matches!(op, Opcode::Iaload))?;
    Some(())
}

fn try_fixed_point_matrix_inverse_transform(code: &AttributeInfoCode) -> Option<FixedPointMatrixInverseTransform> {
    let ops = &code.code_sequence;
    let mut index = 0usize;

    let scale_x = parse_matrix_inverse_scale(ops, &mut index, 2)?;
    let scale_y = parse_matrix_inverse_scale(ops, &mut index, 3)?;
    let scale_z = parse_matrix_inverse_scale(ops, &mut index, 4)?;

    let offset_x = parse_matrix_inverse_offset(ops, &mut index, 2, 5)?;
    let offset_y = parse_matrix_inverse_offset(ops, &mut index, 3, 6)?;
    let offset_z = parse_matrix_inverse_offset(ops, &mut index, 4, 7)?;

    if !same_vector_fields(&offset_x.translation, &offset_y.translation) || !same_vector_fields(&offset_x.translation, &offset_z.translation) {
        return None;
    }

    let vector_x = parse_matrix_inverse_input_store(ops, &mut index, 8)?;
    let vector_y = parse_matrix_inverse_input_store(ops, &mut index, 9)?;
    let vector_z = parse_matrix_inverse_input_store(ops, &mut index, 10)?;

    let out_x = parse_matrix_inverse_output(ops, &mut index, (2, 3, 4), (8, 9, 10), 5)?;
    let out_y = parse_matrix_inverse_output(ops, &mut index, (2, 3, 4), (8, 9, 10), 6)?;
    let out_z = parse_matrix_inverse_output(ops, &mut index, (2, 3, 4), (8, 9, 10), 7)?;

    if !same_vector_fields(&out_x.matrix_column, &offset_x.matrix_column)
        || !same_vector_fields(&out_y.matrix_column, &offset_y.matrix_column)
        || !same_vector_fields(&out_z.matrix_column, &offset_z.matrix_column)
        || !same_field_ref(&out_x.output, &vector_x)
        || !same_field_ref(&out_y.output, &vector_y)
        || !same_field_ref(&out_z.output, &vector_z)
    {
        return None;
    }

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(1)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Areturn))?;
    if index != ops.len() {
        return None;
    }

    Some(FixedPointMatrixInverseTransform {
        scale: Vector3FieldRefs {
            x: scale_x,
            y: scale_y,
            z: scale_z,
        },
        matrix: Matrix3FieldRefs {
            m00: offset_x.matrix_column.x,
            m01: offset_y.matrix_column.x,
            m02: offset_z.matrix_column.x,
            m10: offset_x.matrix_column.y,
            m11: offset_y.matrix_column.y,
            m12: offset_z.matrix_column.y,
            m20: offset_x.matrix_column.z,
            m21: offset_y.matrix_column.z,
            m22: offset_z.matrix_column.z,
        },
        translation: offset_x.translation,
        vector: Vector3FieldRefs {
            x: vector_x,
            y: vector_y,
            z: vector_z,
        },
    })
}

fn parse_matrix_inverse_scale(ops: &[(u32, Opcode)], index: &mut usize, slot: u16) -> Option<FieldMethodref> {
    expect_op(ops, index, |op| matches!(op, Opcode::Ldc(ConstantPoolReference::Integer(16_777_216))))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let scale = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Idiv))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Istore(actual) if *actual == slot))?;
    Some(scale)
}

fn parse_matrix_inverse_offset(ops: &[(u32, Opcode)], index: &mut usize, scale_slot: u16, output_slot: u16) -> Option<MatrixInverseOffset> {
    expect_op(ops, index, |op| matches!(op, Opcode::Iload(actual) if *actual == scale_slot))?;
    let (matrix_x, trans_x) = parse_matrix_inverse_self_term(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ineg))?;
    let (matrix_y, trans_y) = parse_matrix_inverse_self_term(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Isub))?;
    let (matrix_z, trans_z) = parse_matrix_inverse_self_term(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Isub))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Istore(actual) if *actual == output_slot))?;

    Some(MatrixInverseOffset {
        matrix_column: Vector3FieldRefs {
            x: matrix_x,
            y: matrix_y,
            z: matrix_z,
        },
        translation: Vector3FieldRefs {
            x: trans_x,
            y: trans_y,
            z: trans_z,
        },
    })
}

fn parse_matrix_inverse_self_term(ops: &[(u32, Opcode)], index: &mut usize) -> Option<(FieldMethodref, FieldMethodref)> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let lhs = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let rhs = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;
    Some((lhs, rhs))
}

fn parse_matrix_inverse_input_store(ops: &[(u32, Opcode)], index: &mut usize, slot: u16) -> Option<FieldMethodref> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(1)))?;
    let field = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Istore(actual) if *actual == slot))?;
    Some(field)
}

fn parse_matrix_inverse_output(
    ops: &[(u32, Opcode)],
    index: &mut usize,
    scale_slots: (u16, u16, u16),
    input_slots: (u16, u16, u16),
    offset_slot: u16,
) -> Option<MatrixInverseOutput> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(1)))?;
    let matrix_x = parse_matrix_inverse_scaled_input_term(ops, index, scale_slots.0, input_slots.0)?;
    let matrix_y = parse_matrix_inverse_scaled_input_term(ops, index, scale_slots.1, input_slots.1)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    let matrix_z = parse_matrix_inverse_scaled_input_term(ops, index, scale_slots.2, input_slots.2)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iload(actual) if *actual == offset_slot))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    let output = expect_putfield(ops, index)?;
    if output.descriptor.as_str() != "I" {
        return None;
    }

    Some(MatrixInverseOutput {
        matrix_column: Vector3FieldRefs {
            x: matrix_x,
            y: matrix_y,
            z: matrix_z,
        },
        output,
    })
}

fn parse_matrix_inverse_scaled_input_term(ops: &[(u32, Opcode)], index: &mut usize, scale_slot: u16, input_slot: u16) -> Option<FieldMethodref> {
    expect_op(ops, index, |op| matches!(op, Opcode::Iload(actual) if *actual == scale_slot))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let matrix = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iload(actual) if *actual == input_slot))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;
    Some(matrix)
}

fn try_fixed_point_matrix_compose(code: &AttributeInfoCode) -> Option<FixedPointMatrixCompose> {
    let ops = &code.code_sequence;
    let mut index = 0usize;

    let translation_x = parse_matrix_compose_translation_row(ops, &mut index)?;
    let translation_y = parse_matrix_compose_translation_row(ops, &mut index)?;
    let translation_z = parse_matrix_compose_translation_row(ops, &mut index)?;

    if !same_field_ref(&translation_x.translation, &translation_x.output)
        || !same_field_ref(&translation_y.translation, &translation_y.output)
        || !same_field_ref(&translation_z.translation, &translation_z.output)
        || !same_vector_fields(&translation_x.rhs_translation, &translation_y.rhs_translation)
        || !same_vector_fields(&translation_x.rhs_translation, &translation_z.rhs_translation)
    {
        return None;
    }

    let scale_x = parse_matrix_compose_scale_assign(ops, &mut index)?;
    let scale_y = parse_matrix_compose_scale_assign(ops, &mut index)?;
    let scale_z = parse_matrix_compose_scale_assign(ops, &mut index)?;

    if !is_same_field_product_output(&scale_x)
        || !is_same_field_product_output(&scale_y)
        || !is_same_field_product_output(&scale_z)
        || !same_field_ref(&translation_x.scale, &scale_x.output)
        || !same_field_ref(&translation_y.scale, &scale_y.output)
        || !same_field_ref(&translation_z.scale, &scale_z.output)
    {
        return None;
    }

    let matrix = Matrix3FieldRefs {
        m00: translation_x.matrix.x.clone(),
        m01: translation_x.matrix.y.clone(),
        m02: translation_x.matrix.z.clone(),
        m10: translation_y.matrix.x.clone(),
        m11: translation_y.matrix.y.clone(),
        m12: translation_y.matrix.z.clone(),
        m20: translation_z.matrix.x.clone(),
        m21: translation_z.matrix.y.clone(),
        m22: translation_z.matrix.z.clone(),
    };
    let col0 = Vector3FieldRefs {
        x: matrix.m00.clone(),
        y: matrix.m10.clone(),
        z: matrix.m20.clone(),
    };
    let col1 = Vector3FieldRefs {
        x: matrix.m01.clone(),
        y: matrix.m11.clone(),
        z: matrix.m21.clone(),
    };
    let col2 = Vector3FieldRefs {
        x: matrix.m02.clone(),
        y: matrix.m12.clone(),
        z: matrix.m22.clone(),
    };

    let m00 = parse_matrix_compose_matrix_assign(ops, &mut index)?;
    let m10 = parse_matrix_compose_matrix_assign(ops, &mut index)?;
    let m20 = parse_matrix_compose_matrix_assign(ops, &mut index)?;
    let m01 = parse_matrix_compose_matrix_assign(ops, &mut index)?;
    let m11 = parse_matrix_compose_matrix_assign(ops, &mut index)?;
    let m21 = parse_matrix_compose_matrix_assign(ops, &mut index)?;
    let m02 = parse_matrix_compose_matrix_assign(ops, &mut index)?;
    let m12 = parse_matrix_compose_matrix_assign(ops, &mut index)?;
    let m22 = parse_matrix_compose_matrix_assign(ops, &mut index)?;

    if !is_matrix_product_assign(&m00, &translation_x.matrix, &col0, &matrix.m00)
        || !is_matrix_product_assign(&m10, &translation_y.matrix, &col0, &matrix.m10)
        || !is_matrix_product_assign(&m20, &translation_z.matrix, &col0, &matrix.m20)
        || !is_matrix_product_assign(&m01, &translation_x.matrix, &col1, &matrix.m01)
        || !is_matrix_product_assign(&m11, &translation_y.matrix, &col1, &matrix.m11)
        || !is_matrix_product_assign(&m21, &translation_z.matrix, &col1, &matrix.m21)
        || !is_matrix_product_assign(&m02, &translation_x.matrix, &col2, &matrix.m02)
        || !is_matrix_product_assign(&m12, &translation_y.matrix, &col2, &matrix.m12)
        || !is_matrix_product_assign(&m22, &translation_z.matrix, &col2, &matrix.m22)
    {
        return None;
    }

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(2)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iconst(1)))?;
    let dirty = expect_putfield(ops, &mut index)?;
    if dirty.descriptor.as_str() != "Z" {
        return None;
    }
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Aload(2)))?;
    expect_op(ops, &mut index, |op| matches!(op, Opcode::Areturn))?;

    if index != ops.len() {
        return None;
    }

    Some(FixedPointMatrixCompose {
        scale: Vector3FieldRefs {
            x: scale_x.output,
            y: scale_y.output,
            z: scale_z.output,
        },
        matrix,
        translation: Vector3FieldRefs {
            x: translation_x.output,
            y: translation_y.output,
            z: translation_z.output,
        },
        dirty,
    })
}

fn parse_matrix_compose_translation_row(ops: &[(u32, Opcode)], index: &mut usize) -> Option<MatrixComposeTranslationRow> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(2)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let translation = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let scale = expect_int_getfield(ops, index)?;

    let (matrix_x, rhs_x) = parse_matrix_compose_term(ops, index)?;
    let (matrix_y, rhs_y) = parse_matrix_compose_term(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    let (matrix_z, rhs_z) = parse_matrix_compose_term(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    let output = expect_putfield(ops, index)?;
    if output.descriptor.as_str() != "I" {
        return None;
    }

    Some(MatrixComposeTranslationRow {
        translation,
        scale,
        matrix: Vector3FieldRefs {
            x: matrix_x,
            y: matrix_y,
            z: matrix_z,
        },
        rhs_translation: Vector3FieldRefs {
            x: rhs_x,
            y: rhs_y,
            z: rhs_z,
        },
        output,
    })
}

fn parse_matrix_compose_scale_assign(ops: &[(u32, Opcode)], index: &mut usize) -> Option<MatrixComposeScaleAssign> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(2)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let lhs = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(1)))?;
    let rhs = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;
    let output = expect_putfield(ops, index)?;
    if output.descriptor.as_str() != "I" {
        return None;
    }

    Some(MatrixComposeScaleAssign { lhs, rhs, output })
}

fn parse_matrix_compose_matrix_assign(ops: &[(u32, Opcode)], index: &mut usize) -> Option<MatrixComposeMatrixAssign> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(2)))?;
    let (lhs_x, rhs_x) = parse_matrix_compose_term(ops, index)?;
    let (lhs_y, rhs_y) = parse_matrix_compose_term(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    let (lhs_z, rhs_z) = parse_matrix_compose_term(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Iadd))?;
    let output = expect_putfield(ops, index)?;
    if output.descriptor.as_str() != "I" {
        return None;
    }

    Some(MatrixComposeMatrixAssign {
        lhs: Vector3FieldRefs {
            x: lhs_x,
            y: lhs_y,
            z: lhs_z,
        },
        rhs: Vector3FieldRefs {
            x: rhs_x,
            y: rhs_y,
            z: rhs_z,
        },
        output,
    })
}

fn parse_matrix_compose_term(ops: &[(u32, Opcode)], index: &mut usize) -> Option<(FieldMethodref, FieldMethodref)> {
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(0)))?;
    let lhs = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Aload(1)))?;
    let rhs = expect_int_getfield(ops, index)?;
    expect_op(ops, index, |op| matches!(op, Opcode::Imul))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Bipush(12)))?;
    expect_op(ops, index, |op| matches!(op, Opcode::Ishr))?;

    Some((lhs, rhs))
}

fn is_same_field_product_output(assign: &MatrixComposeScaleAssign) -> bool {
    same_field_ref(&assign.lhs, &assign.rhs) && same_field_ref(&assign.lhs, &assign.output)
}

fn is_matrix_product_assign(
    assign: &MatrixComposeMatrixAssign,
    expected_lhs: &Vector3FieldRefs,
    expected_rhs: &Vector3FieldRefs,
    expected_output: &FieldMethodref,
) -> bool {
    same_vector_fields(&assign.lhs, expected_lhs) && same_vector_fields(&assign.rhs, expected_rhs) && same_field_ref(&assign.output, expected_output)
}

fn expect_op<F>(ops: &[(u32, Opcode)], index: &mut usize, predicate: F) -> Option<u32>
where
    F: FnOnce(&Opcode) -> bool,
{
    let (offset, opcode) = ops.get(*index)?;
    if !predicate(opcode) {
        return None;
    }

    *index += 1;
    Some(*offset)
}

fn expect_iconst(ops: &[(u32, Opcode)], index: &mut usize, expected: i32) -> Option<u32> {
    expect_op(ops, index, |op| match (expected, op) {
        (value, Opcode::Iconst(actual)) => value == i32::from(*actual),
        (value, Opcode::Bipush(actual)) => value == i32::from(*actual),
        (value, Opcode::Sipush(actual)) => value == i32::from(*actual),
        _ => false,
    })
}

fn expect_getfield(ops: &[(u32, Opcode)], index: &mut usize) -> Option<FieldMethodref> {
    let (_, opcode) = ops.get(*index)?;
    let Opcode::Getfield(field) = opcode else {
        return None;
    };
    *index += 1;
    Some(field.as_field_ref().clone())
}

fn expect_int_getfield(ops: &[(u32, Opcode)], index: &mut usize) -> Option<FieldMethodref> {
    let field = expect_getfield(ops, index)?;
    if field.descriptor.as_str() != "I" {
        return None;
    }
    Some(field)
}

fn expect_putfield(ops: &[(u32, Opcode)], index: &mut usize) -> Option<FieldMethodref> {
    let (_, opcode) = ops.get(*index)?;
    let Opcode::Putfield(field) = opcode else {
        return None;
    };
    *index += 1;
    Some(field.as_field_ref().clone())
}

fn same_vector_fields(lhs: &Vector3FieldRefs, rhs: &Vector3FieldRefs) -> bool {
    same_field_ref(&lhs.x, &rhs.x) && same_field_ref(&lhs.y, &rhs.y) && same_field_ref(&lhs.z, &rhs.z)
}

fn same_field_ref(lhs: &FieldMethodref, rhs: &FieldMethodref) -> bool {
    lhs.class.as_str() == rhs.class.as_str() && lhs.name.as_str() == rhs.name.as_str() && lhs.descriptor.as_str() == rhs.descriptor.as_str()
}

fn is_fixed_point_inverse_sqrt(code: &AttributeInfoCode) -> bool {
    let ops = &code.code_sequence;
    let mut index = 0usize;

    expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(0))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Iconst(1))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Ishr)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Istore(1))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(0))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::I2f)).is_some()
        && expect_op(
            ops,
            &mut index,
            |op| matches!(op, Opcode::Ldc(ConstantPoolReference::Float(value)) if value.to_bits() == 2.4414062e-4f32.to_bits()),
        )
        .is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Fmul)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Invokestatic(_))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Istore(0))).is_some()
        && expect_op(ops, &mut index, |op| {
            matches!(op, Opcode::Ldc(ConstantPoolReference::Integer(1_597_463_174)))
        })
        .is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(0))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Iconst(1))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Ishr)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Isub)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Dup)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Istore(0))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Invokestatic(_))).is_some()
        && expect_op(
            ops,
            &mut index,
            |op| matches!(op, Opcode::Ldc(ConstantPoolReference::Float(value)) if value.to_bits() == 4096.0f32.to_bits()),
        )
        .is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Fmul)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::F2i)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Dup)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Istore(0))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Sipush(6144))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(1))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(0))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Imul)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Bipush(12))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Ishr)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Iload(0))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Imul)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Bipush(12))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Ishr)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Isub)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Imul)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Bipush(12))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Ishr)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Dup)).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Istore(0))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Invokestatic(_))).is_some()
        && expect_op(ops, &mut index, |op| matches!(op, Opcode::Ireturn)).is_some()
        && index == ops.len()
}

fn is_fixed_point_sqrt_long_to_int(code: &AttributeInfoCode) -> bool {
    let ops = &code.code_sequence;
    if ops.len() != 55 {
        return false;
    }

    matches_offset_op(&ops[0], 0, |op| matches!(op, Opcode::Lload(0)))
        && matches_offset_op(&ops[1], 1, |op| matches!(op, Opcode::Lconst(0)))
        && matches_offset_op(&ops[2], 2, |op| matches!(op, Opcode::Lcmp))
        && matches_offset_op(&ops[3], 3, |op| matches!(op, Opcode::Ifne(5)))
        && matches_offset_op(&ops[4], 6, |op| matches!(op, Opcode::Iconst(0)))
        && matches_offset_op(&ops[5], 7, |op| matches!(op, Opcode::Ireturn))
        && matches_offset_op(
            &ops[6],
            8,
            |op| matches!(op, Opcode::Ldc(ConstantPoolReference::Integer(value)) if *value == i32::MAX),
        )
        && matches_offset_op(&ops[7], 10, |op| matches!(op, Opcode::Dup))
        && matches_offset_op(&ops[8], 11, |op| matches!(op, Opcode::Istore(2)))
        && matches_offset_op(&ops[9], 12, |op| matches!(op, Opcode::I2l))
        && matches_offset_op(&ops[10], 13, |op| matches!(op, Opcode::Lstore(4)))
        && matches_offset_op(&ops[11], 15, |op| matches!(op, Opcode::Bipush(30)))
        && matches_offset_op(&ops[12], 17, |op| matches!(op, Opcode::Istore(3)))
        && matches_offset_op(&ops[13], 18, |op| matches!(op, Opcode::Iload(3)))
        && matches_offset_op(&ops[14], 19, |op| matches!(op, Opcode::Iflt(62)))
        && matches_offset_op(&ops[15], 22, |op| matches!(op, Opcode::Iload(2)))
        && matches_offset_op(&ops[16], 23, |op| matches!(op, Opcode::Iconst(1)))
        && matches_offset_op(&ops[17], 24, |op| matches!(op, Opcode::Ishr))
        && matches_offset_op(&ops[18], 25, |op| matches!(op, Opcode::Istore(2)))
        && matches_offset_op(&ops[19], 26, |op| matches!(op, Opcode::Lload(0)))
        && matches_offset_op(&ops[20], 27, |op| matches!(op, Opcode::Lload(4)))
        && matches_offset_op(&ops[21], 29, |op| matches!(op, Opcode::Lload(4)))
        && matches_offset_op(&ops[22], 31, |op| matches!(op, Opcode::Lmul))
        && matches_offset_op(&ops[23], 32, |op| matches!(op, Opcode::Bipush(12)))
        && matches_offset_op(&ops[24], 34, |op| matches!(op, Opcode::Lshr))
        && matches_offset_op(&ops[25], 35, |op| matches!(op, Opcode::Lsub))
        && matches_offset_op(&ops[26], 36, |op| matches!(op, Opcode::Dup2))
        && matches_offset_op(&ops[27], 37, |op| matches!(op, Opcode::Lstore(7)))
        && matches_offset_op(&ops[28], 39, |op| matches!(op, Opcode::Lconst(0)))
        && matches_offset_op(&ops[29], 40, |op| matches!(op, Opcode::Lcmp))
        && matches_offset_op(&ops[30], 41, |op| matches!(op, Opcode::Ifle(13)))
        && matches_offset_op(&ops[31], 44, |op| matches!(op, Opcode::Lload(4)))
        && matches_offset_op(&ops[32], 46, |op| matches!(op, Opcode::Iload(2)))
        && matches_offset_op(&ops[33], 47, |op| matches!(op, Opcode::I2l))
        && matches_offset_op(&ops[34], 48, |op| matches!(op, Opcode::Ladd))
        && matches_offset_op(&ops[35], 49, |op| matches!(op, Opcode::Lstore(4)))
        && matches_offset_op(&ops[36], 51, |op| matches!(op, Opcode::Goto(24)))
        && matches_offset_op(&ops[37], 54, |op| matches!(op, Opcode::Lload(7)))
        && matches_offset_op(&ops[38], 56, |op| matches!(op, Opcode::Lconst(0)))
        && matches_offset_op(&ops[39], 57, |op| matches!(op, Opcode::Lcmp))
        && matches_offset_op(&ops[40], 58, |op| matches!(op, Opcode::Ifge(13)))
        && matches_offset_op(&ops[41], 61, |op| matches!(op, Opcode::Lload(4)))
        && matches_offset_op(&ops[42], 63, |op| matches!(op, Opcode::Iload(2)))
        && matches_offset_op(&ops[43], 64, |op| matches!(op, Opcode::I2l))
        && matches_offset_op(&ops[44], 65, |op| matches!(op, Opcode::Lsub))
        && matches_offset_op(&ops[45], 66, |op| matches!(op, Opcode::Lstore(4)))
        && matches_offset_op(&ops[46], 68, |op| matches!(op, Opcode::Goto(7)))
        && matches_offset_op(&ops[47], 71, |op| matches!(op, Opcode::Lload(4)))
        && matches_offset_op(&ops[48], 73, |op| matches!(op, Opcode::L2i))
        && matches_offset_op(&ops[49], 74, |op| matches!(op, Opcode::Ireturn))
        && matches_offset_op(&ops[50], 75, |op| matches!(op, Opcode::Iinc(3, -1)))
        && matches_offset_op(&ops[51], 78, |op| matches!(op, Opcode::Goto(-60)))
        && matches_offset_op(&ops[52], 81, |op| matches!(op, Opcode::Lload(4)))
        && matches_offset_op(&ops[53], 83, |op| matches!(op, Opcode::L2i))
        && matches_offset_op(&ops[54], 84, |op| matches!(op, Opcode::Ireturn))
}

fn matches_offset_op<F>((offset, opcode): &(u32, Opcode), expected_offset: u32, predicate: F) -> bool
where
    F: FnOnce(&Opcode) -> bool,
{
    *offset == expected_offset && predicate(opcode)
}
