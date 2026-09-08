use alloc::{boxed::Box, format, vec::Vec};

use classfile::{AttributeInfoCode, ConstantPoolReference, MethodParamKind};
use jvm::{ClassInstance, JavaError, JavaValue, Jvm, Result, runtime::JavaLangString};

use crate::{array_class_instance::ArrayClassInstanceImpl, stack_frame::StackFrame};

use super::Interpreter;

impl Interpreter {
    pub(super) async fn load_array_one(jvm: &Jvm, array: Option<Box<dyn ClassInstance>>, index: i32) -> Result<JavaValue> {
        if index < 0 {
            return Err(jvm.exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index}")).await);
        }

        let Some(array) = array else {
            return Err(jvm.exception("java/lang/NullPointerException", "Array is null").await);
        };

        let Some(array_instance) = array.as_array_instance() else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Not an array").await);
        };

        let index = index as usize;
        let array_size = array_instance.length();
        if index >= array_size {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {array_size}"))
                .await);
        }

        array_instance.load_one(index)
    }

    pub(super) async fn store_array_one(jvm: &Jvm, array: &mut Option<Box<dyn ClassInstance>>, index: i32, value: JavaValue) -> Result<()> {
        if index < 0 {
            return Err(jvm.exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index}")).await);
        }

        let Some(array) = array else {
            return Err(jvm.exception("java/lang/NullPointerException", "Array is null").await);
        };

        let Some(array_instance) = array.as_array_instance_mut() else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Not an array").await);
        };

        let index = index as usize;
        let array_size = array_instance.length();
        if index >= array_size {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", &format!("{index} >= {array_size}"))
                .await);
        }

        array_instance.store_one(index, value)
    }

    pub(super) async fn find_exception_handler(jvm: &Jvm, exception: &dyn ClassInstance, code_attribute: &AttributeInfoCode, pc: u32) -> Option<u32> {
        for exception_table in &code_attribute.exception_table {
            if exception_table.start_pc <= pc as u16 && exception_table.end_pc > pc as u16 {
                if exception_table.catch_type.is_none() {
                    return Some(exception_table.handler_pc as _);
                }

                let catch_type = exception_table.catch_type.as_ref().unwrap();
                if jvm.is_instance(exception, catch_type) {
                    return Some(exception_table.handler_pc as _);
                }
            }
        }

        None
    }

    pub(super) fn try_load_array_one_stack(array: Option<&dyn ClassInstance>, index: i32) -> Option<JavaValue> {
        if index < 0 {
            return None;
        }

        let array = array?;
        if let Some(array_instance) = array.as_any().downcast_ref::<ArrayClassInstanceImpl>() {
            let index = index as usize;
            if index >= array_instance.len() {
                return None;
            }
            return Some(array_instance.load_one_stack(index));
        }

        let array_instance = array.as_array_instance()?;
        let index = index as usize;
        if index >= array_instance.length() {
            return None;
        }

        array_instance.load_one(index).ok().map(Self::to_stack_frame_type)
    }

    pub(super) fn try_store_array_one_stack(array: &mut Box<dyn ClassInstance>, index: i32, value: JavaValue) -> bool {
        if index < 0 {
            return false;
        }

        if let Some(array_instance) = array.as_any_mut().downcast_mut::<ArrayClassInstanceImpl>() {
            let index = index as usize;
            if index >= array_instance.len() {
                return false;
            }
            array_instance.store_one_stack(index, value);
            return true;
        }

        let Some(array_instance) = array.as_array_instance_mut() else {
            return false;
        };
        let index = index as usize;
        if index >= array_instance.length() {
            return false;
        }

        array_instance.store_one(index, value).is_ok()
    }

    pub(super) fn int_value(value: JavaValue) -> i32 {
        match value {
            JavaValue::Boolean(value) => value as i32,
            JavaValue::Byte(value) => value as i32,
            JavaValue::Char(value) => value as i32,
            JavaValue::Short(value) => value as i32,
            JavaValue::Int(value) => value,
            _ => panic!("Expected integer-compatible value, got {value:?}"),
        }
    }

    pub(super) fn int_value_ref(value: &JavaValue) -> i32 {
        match value {
            JavaValue::Boolean(value) => *value as i32,
            JavaValue::Byte(value) => *value as i32,
            JavaValue::Char(value) => *value as i32,
            JavaValue::Short(value) => *value as i32,
            JavaValue::Int(value) => *value,
            _ => panic!("Expected integer-compatible value, got {value:?}"),
        }
    }

    pub(super) fn long_value(value: JavaValue) -> i64 {
        match value {
            JavaValue::Long(value) => value,
            _ => panic!("Expected long value, got {value:?}"),
        }
    }

    pub(super) fn float_value(value: JavaValue) -> f32 {
        match value {
            JavaValue::Float(value) => value,
            _ => panic!("Expected float value, got {value:?}"),
        }
    }

    pub(super) fn double_value(value: JavaValue) -> f64 {
        match value {
            JavaValue::Double(value) => value,
            _ => panic!("Expected double value, got {value:?}"),
        }
    }

    pub(super) fn int_binary<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(i32, i32) -> i32,
    {
        let value2 = Self::int_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::int_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Int(operation(value1, value2)));
    }

    pub(super) fn int_shift<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(i32, i32) -> i32,
    {
        let value2 = Self::int_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::int_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Int(operation(value1, value2)));
    }

    pub(super) fn long_binary<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(i64, i64) -> i64,
    {
        let value2 = Self::long_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::long_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Long(operation(value1, value2)));
    }

    pub(super) fn long_shift<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(i64, i32) -> i64,
    {
        let value2 = Self::int_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::long_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Long(operation(value1, value2)));
    }

    pub(super) fn float_binary<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(f32, f32) -> f32,
    {
        let value2 = Self::float_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::float_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Float(operation(value1, value2)));
    }

    pub(super) fn float_compare(stack_frame: &mut StackFrame, nan_value: i32) {
        let value2 = Self::float_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::float_value(stack_frame.operand_stack.pop().unwrap());

        if value1.is_nan() || value2.is_nan() {
            stack_frame.operand_stack.push(JavaValue::Int(nan_value));
        } else {
            stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
        }
    }

    pub(super) fn double_binary<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let value2 = Self::double_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::double_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Double(operation(value1, value2)));
    }

    pub(super) fn double_compare(stack_frame: &mut StackFrame, nan_value: i32) {
        let value2 = Self::double_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::double_value(stack_frame.operand_stack.pop().unwrap());

        if value1.is_nan() || value2.is_nan() {
            stack_frame.operand_stack.push(JavaValue::Int(nan_value));
        } else {
            stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
        }
    }

    pub(super) fn constant_to_value_fast(constant: &ConstantPoolReference) -> Option<JavaValue> {
        Some(match constant {
            ConstantPoolReference::Integer(x) => JavaValue::Int(*x),
            ConstantPoolReference::Float(x) => JavaValue::Float(*x),
            ConstantPoolReference::Long(x) => JavaValue::Long(*x),
            ConstantPoolReference::Double(x) => JavaValue::Double(*x),
            _ => return None,
        })
    }

    pub(super) fn integer_condition<T>(stack_frame: &mut StackFrame, pred: T) -> bool
    where
        T: Fn(i32, i32) -> bool,
    {
        let value2 = stack_frame.operand_stack.pop().unwrap().into();
        let value1 = stack_frame.operand_stack.pop().unwrap().into();

        pred(value1, value2)
    }

    pub(super) fn integer_condition_single<T>(stack_frame: &mut StackFrame, pred: T) -> bool
    where
        T: Fn(i32) -> bool,
    {
        let value = stack_frame.operand_stack.pop().unwrap().into();

        pred(value)
    }

    pub(super) fn extract_invoke_params(stack_frame: &mut StackFrame, param_kinds: &[MethodParamKind]) -> Vec<JavaValue> {
        let mut values = Vec::with_capacity(param_kinds.len());

        for kind in param_kinds.iter().rev() {
            let value = stack_frame.operand_stack.pop().unwrap();
            values.push(match kind {
                MethodParamKind::Boolean => JavaValue::Boolean(i32::from(value) & 1 != 0),
                MethodParamKind::Byte => JavaValue::Byte(i32::from(value) as _),
                MethodParamKind::Char => JavaValue::Char(i32::from(value) as _),
                MethodParamKind::Short => JavaValue::Short(i32::from(value) as _),
                MethodParamKind::Other => value,
            });
        }

        values.reverse();

        values
    }

    pub(super) fn push_invoke_result(stack_frame: &mut StackFrame, value: JavaValue) {
        match value {
            JavaValue::Void => {}
            _ => stack_frame.operand_stack.push(Self::to_stack_frame_type(value)),
        }
    }

    // operand stack has only integer for small number types, so convert it to the field type
    pub(super) fn to_field_type(descriptor: &str, value: JavaValue) -> JavaValue {
        match descriptor {
            "Z" => JavaValue::Boolean(i32::from(value) & 1 != 0),
            "B" => JavaValue::Byte(i32::from(value) as i8),
            "C" => JavaValue::Char(i32::from(value) as u16),
            "S" => JavaValue::Short(i32::from(value) as i16),
            _ => value,
        }
    }

    // convert into stack frame type, which is always integer for number types
    pub(super) fn to_stack_frame_type(value: JavaValue) -> JavaValue {
        match value {
            JavaValue::Boolean(x) => JavaValue::Int(x as _),
            JavaValue::Byte(x) => JavaValue::Int(x as _),
            JavaValue::Char(x) => JavaValue::Int(x as _),
            JavaValue::Short(x) => JavaValue::Int(x as _),
            _ => value,
        }
    }

    pub(super) async fn invalid_cp(jvm: &Jvm, kind: &str) -> JavaError {
        jvm.exception("java/lang/LinkageError", &format!("invalid constant pool {kind}")).await
    }

    pub(super) async fn constant_to_value(jvm: &Jvm, constant: &ConstantPoolReference) -> Result<JavaValue> {
        Ok(match constant {
            ConstantPoolReference::Integer(x) => JavaValue::Int(*x),
            ConstantPoolReference::Float(x) => JavaValue::Float(*x),
            ConstantPoolReference::Long(x) => JavaValue::Long(*x),
            ConstantPoolReference::Double(x) => JavaValue::Double(*x),
            ConstantPoolReference::String(x) => JavaValue::Object(Some(JavaLangString::intern_rust_string(jvm, x).await?)),
            ConstantPoolReference::Class(x) => JavaValue::Object(Some(jvm.resolve_class(x).await?.java_class())),
            _ => return Err(Self::invalid_cp(jvm, "ldc").await),
        })
    }

    #[async_recursion::async_recursion]
    pub(super) async fn new_multi_array(jvm: &Jvm, array_class: &str, dimensions: &[i32]) -> Result<Box<dyn ClassInstance>> {
        let mut array = jvm.instantiate_array(&array_class[1..], dimensions[0] as _).await?;

        if dimensions.len() > 1 {
            for i in 0..dimensions[0] {
                let element = Self::new_multi_array(jvm, &array_class[1..], &dimensions[1..]).await?;
                jvm.store_array(&mut array, i as _, [element]).await?;
            }
        }

        Ok(array)
    }
}
