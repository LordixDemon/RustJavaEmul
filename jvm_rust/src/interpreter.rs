use alloc::{boxed::Box, format, string::String, sync::Arc, vec::Vec};

use classfile::{AttributeInfoCode, ConstantPoolReference, MethodParamKind, Opcode};
use jvm::{
    ClassInstance, JavaChar, JavaError, JavaType, JavaValue, Jvm, ResolvedInstanceField, ResolvedMethod, ResolvedStaticField, Result,
    runtime::JavaLangString,
};

use crate::{array_class_instance::ArrayClassInstanceImpl, profile, stack_frame::StackFrame};

enum ExecuteNext {
    Continue,
    Jump(u32),
    Return(JavaValue),
}

struct ResolvedInstanceMethod {
    receiver_class: String,
    method: ResolvedMethod,
}

struct ResolvedInstanceFieldSite {
    field: ResolvedInstanceField,
}

pub struct Interpreter;

impl Interpreter {
    pub async fn run(
        jvm: &Jvm,
        code_attribute: &AttributeInfoCode,
        args: Box<[JavaValue]>,
        return_type: &JavaType,
        frame_name: Arc<str>,
    ) -> Result<JavaValue> {
        if profile::enabled() {
            Self::run_inner::<true>(jvm, code_attribute, args, return_type, frame_name).await
        } else {
            Self::run_inner::<false>(jvm, code_attribute, args, return_type, frame_name).await
        }
    }

    async fn run_inner<const PROFILE: bool>(
        jvm: &Jvm,
        code_attribute: &AttributeInfoCode,
        args: Box<[JavaValue]>,
        return_type: &JavaType,
        frame_name: Arc<str>,
    ) -> Result<JavaValue> {
        if PROFILE {
            profile::interpreter_run();
        }

        let mut stack_frame = StackFrame::with_capacity(code_attribute.max_stack as usize, code_attribute.max_locals as usize);

        for value in args {
            let stack_value = Self::to_stack_frame_type(value);
            let wide = matches!(stack_value, JavaValue::Long(_) | JavaValue::Double(_));
            stack_frame.local_variables.push(stack_value);
            if wide {
                // long and double take two slots in local variables
                stack_frame.local_variables.push(JavaValue::Void);
            }
        }
        stack_frame.local_variables.resize(code_attribute.max_locals as usize, JavaValue::Void);

        let mut resolved_static_fields = Vec::with_capacity(code_attribute.code_sequence.len());
        resolved_static_fields.resize_with(code_attribute.code_sequence.len(), || None);
        let mut resolved_instance_fields = Vec::with_capacity(code_attribute.code_sequence.len());
        resolved_instance_fields.resize_with(code_attribute.code_sequence.len(), || None);
        let mut resolved_static_methods = Vec::with_capacity(code_attribute.code_sequence.len());
        resolved_static_methods.resize_with(code_attribute.code_sequence.len(), || None);
        let mut resolved_special_methods = Vec::with_capacity(code_attribute.code_sequence.len());
        resolved_special_methods.resize_with(code_attribute.code_sequence.len(), || None);
        let mut resolved_instance_methods = Vec::with_capacity(code_attribute.code_sequence.len());
        resolved_instance_methods.resize_with(code_attribute.code_sequence.len(), || None);
        let mut resolved_jumps = Vec::with_capacity(code_attribute.code_sequence.len());
        resolved_jumps.resize_with(code_attribute.code_sequence.len(), || None);

        let mut instruction_index = 0;
        let mut cooperative_opcode_count = 0usize;
        let mut method_opcodes = 0u64;
        while instruction_index < code_attribute.code_sequence.len() {
            let (offset, opcode) = &code_attribute.code_sequence[instruction_index];
            let offset = *offset;
            method_opcodes += 1;
            if PROFILE {
                profile::opcode();
            }
            Self::cooperate(&mut cooperative_opcode_count).await;

            let result = if let Some(result) = Self::execute_fast_opcode::<PROFILE>(jvm, offset, opcode, &mut stack_frame, return_type) {
                if PROFILE {
                    profile::fast_opcode();
                }
                Ok(result)
            } else {
                if PROFILE {
                    profile::slow_opcode();
                }
                tracing::trace!("Opcode {:?}", opcode);
                Self::execute_opcode::<PROFILE>(
                    jvm,
                    instruction_index,
                    offset,
                    opcode,
                    &mut stack_frame,
                    return_type,
                    &mut resolved_static_fields,
                    &mut resolved_instance_fields,
                    &mut resolved_static_methods,
                    &mut resolved_special_methods,
                    &mut resolved_instance_methods,
                )
                .await
            };
            match result {
                Ok(ExecuteNext::Continue) => {
                    instruction_index += 1;
                }
                Ok(ExecuteNext::Jump(offset)) => {
                    if PROFILE {
                        profile::jump();
                    }
                    instruction_index = Self::jump_to_instruction_cached(jvm, code_attribute, &mut resolved_jumps, instruction_index, offset).await?;
                }
                Ok(ExecuteNext::Return(value)) => {
                    if PROFILE {
                        profile::return_value();
                    }
                    profile::record_method_opcodes(frame_name, method_opcodes);
                    return Ok(value);
                }
                Err(JavaError::JavaException(e)) => {
                    if PROFILE {
                        profile::java_exception();
                    }
                    let exception_handler = Self::find_exception_handler(jvm, &*e, code_attribute, offset).await;
                    if let Some(x) = exception_handler {
                        stack_frame.operand_stack.clear();
                        stack_frame.operand_stack.push(JavaValue::Object(Some(e)));

                        instruction_index = Self::jump_to_instruction(jvm, code_attribute, x).await?;
                    } else {
                        profile::record_method_opcodes(frame_name, method_opcodes);
                        return Err(JavaError::JavaException(e));
                    }
                }
            }
        }

        profile::record_method_opcodes(frame_name, method_opcodes);
        Err(jvm
            .exception("java/lang/RuntimeException", "Control reached the end of a non-void method")
            .await)
    }

    async fn cooperate(_opcode_count: &mut usize) {}

    async fn jump_to_instruction(jvm: &Jvm, code_attribute: &AttributeInfoCode, offset: u32) -> Result<usize> {
        match code_attribute.code_offsets.binary_search(&offset) {
            Ok(index) => Ok(index),
            Err(_) => Err(jvm
                .exception("java/lang/IllegalArgumentException", &format!("Invalid bytecode jump target {offset}"))
                .await),
        }
    }

    async fn jump_to_instruction_cached(
        jvm: &Jvm,
        code_attribute: &AttributeInfoCode,
        resolved_jumps: &mut [Option<(u32, usize)>],
        instruction_index: usize,
        offset: u32,
    ) -> Result<usize> {
        if let Some((cached_offset, cached_index)) = resolved_jumps[instruction_index]
            && cached_offset == offset
        {
            return Ok(cached_index);
        }

        let target_index = Self::jump_to_instruction(jvm, code_attribute, offset).await?;
        resolved_jumps[instruction_index] = Some((offset, target_index));
        Ok(target_index)
    }

    fn execute_fast_opcode<const PROFILE: bool>(
        jvm: &Jvm,
        current_offset: u32,
        opcode: &Opcode,
        stack_frame: &mut StackFrame,
        return_type: &JavaType,
    ) -> Option<ExecuteNext> {
        match opcode {
            Opcode::Aaload | Opcode::Baload | Opcode::Caload | Opcode::Daload | Opcode::Faload | Opcode::Iaload | Opcode::Laload | Opcode::Saload => {
                let stack_len = stack_frame.operand_stack.len();
                if stack_len < 2 {
                    return None;
                }

                let index = Self::int_value_ref(&stack_frame.operand_stack[stack_len - 1]);
                let array = match &stack_frame.operand_stack[stack_len - 2] {
                    JavaValue::Object(array) => array.as_ref().map(|array| array.as_ref()),
                    _ => return None,
                };

                let value = Self::try_load_array_one_stack(array, index)?;
                stack_frame.operand_stack.pop();
                stack_frame.operand_stack.pop();
                stack_frame.operand_stack.push(value);
            }
            Opcode::Aastore
            | Opcode::Bastore
            | Opcode::Castore
            | Opcode::Dastore
            | Opcode::Fastore
            | Opcode::Iastore
            | Opcode::Lastore
            | Opcode::Sastore => {
                let stack_len = stack_frame.operand_stack.len();
                if stack_len < 3 {
                    return None;
                }

                let value = stack_frame.operand_stack[stack_len - 1].clone();
                let index = Self::int_value_ref(&stack_frame.operand_stack[stack_len - 2]);
                let stored = match &mut stack_frame.operand_stack[stack_len - 3] {
                    JavaValue::Object(Some(array)) => Self::try_store_array_one_stack(array, index, value),
                    _ => false,
                };

                if !stored {
                    return None;
                }

                stack_frame.operand_stack.truncate(stack_len - 3);
            }
            Opcode::AconstNull => stack_frame.operand_stack.push(JavaValue::Object(None)),
            Opcode::Aload(x) | Opcode::Dload(x) | Opcode::Fload(x) | Opcode::Iload(x) | Opcode::Lload(x) => {
                let value = stack_frame.local_variables[*x as usize].clone();
                stack_frame.operand_stack.push(value);
            }
            Opcode::Areturn | Opcode::Dreturn | Opcode::Freturn | Opcode::Ireturn | Opcode::Lreturn => {
                let return_value = stack_frame.operand_stack.pop().unwrap();
                if matches!(opcode, Opcode::Ireturn) {
                    let value = Self::int_value(return_value);
                    let value = match return_type {
                        JavaType::Boolean => JavaValue::Boolean(value & 1 != 0),
                        JavaType::Char => JavaValue::Char(value as _),
                        JavaType::Byte => JavaValue::Byte(value as _),
                        JavaType::Short => JavaValue::Short(value as _),
                        _ => JavaValue::Int(value),
                    };
                    return Some(ExecuteNext::Return(value));
                }
                return Some(ExecuteNext::Return(return_value));
            }
            Opcode::Arraylength => {
                let stack_len = stack_frame.operand_stack.len();
                let JavaValue::Object(Some(array)) = &stack_frame.operand_stack[stack_len - 1] else {
                    return None;
                };
                let array_instance = array.as_array_instance()?;

                let length = array_instance.length();
                stack_frame.operand_stack.pop();
                stack_frame.operand_stack.push(JavaValue::Int(length as _));
            }
            Opcode::Astore(x) | Opcode::Dstore(x) | Opcode::Fstore(x) | Opcode::Istore(x) | Opcode::Lstore(x) => {
                let value = stack_frame.operand_stack.pop().unwrap();
                stack_frame.local_variables[*x as usize] = value;
            }
            Opcode::Bipush(x) => stack_frame.operand_stack.push(JavaValue::Int(*x as i32)),
            Opcode::D2f => {
                let value = Self::double_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Float(value as _));
            }
            Opcode::D2i => {
                let value = Self::double_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Int(value as _));
            }
            Opcode::D2l => {
                let value = Self::double_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Long(value as _));
            }
            Opcode::Dadd => Self::double_binary(stack_frame, |x, y| x + y),
            Opcode::Dcmpg => Self::double_compare(stack_frame, 1),
            Opcode::Dcmpl => Self::double_compare(stack_frame, -1),
            Opcode::Dconst(x) => stack_frame.operand_stack.push(JavaValue::Double(*x as f64)),
            Opcode::Ddiv => Self::double_binary(stack_frame, |x, y| x / y),
            Opcode::Dmul => Self::double_binary(stack_frame, |x, y| x * y),
            Opcode::Dneg => {
                let value = Self::double_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Double(-value));
            }
            Opcode::Drem => Self::double_binary(stack_frame, |x, y| x % y),
            Opcode::Dsub => Self::double_binary(stack_frame, |x, y| x - y),
            Opcode::Dup => {
                let value = stack_frame.operand_stack.pop().unwrap();
                stack_frame.operand_stack.push(value.clone());
                stack_frame.operand_stack.push(value);
            }
            Opcode::Dup2 => {
                let value = stack_frame.operand_stack.pop().unwrap();
                if matches!(value, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.push(value.clone());
                    stack_frame.operand_stack.push(value);
                } else {
                    let value2 = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value2.clone());
                    stack_frame.operand_stack.push(value.clone());
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value);
                }
            }
            Opcode::Dup2X1 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                if matches!(value1, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                } else {
                    let value3 = stack_frame.operand_stack.pop().unwrap();

                    stack_frame.operand_stack.push(value2.clone());
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value3);
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                }
            }
            Opcode::Dup2X2 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                if matches!(value1, JavaValue::Long(_) | JavaValue::Double(_)) && matches!(value2, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                } else if matches!(value1, JavaValue::Long(_) | JavaValue::Double(_)) {
                    let value3 = stack_frame.operand_stack.pop().unwrap();

                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value3);
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                } else {
                    let value3 = stack_frame.operand_stack.pop().unwrap();

                    if matches!(value3, JavaValue::Long(_) | JavaValue::Double(_)) {
                        stack_frame.operand_stack.push(value2.clone());
                        stack_frame.operand_stack.push(value1.clone());
                        stack_frame.operand_stack.push(value3);
                        stack_frame.operand_stack.push(value2);
                        stack_frame.operand_stack.push(value1);
                    } else {
                        let value4 = stack_frame.operand_stack.pop().unwrap();

                        stack_frame.operand_stack.push(value2.clone());
                        stack_frame.operand_stack.push(value1.clone());
                        stack_frame.operand_stack.push(value4);
                        stack_frame.operand_stack.push(value3);
                        stack_frame.operand_stack.push(value2);
                        stack_frame.operand_stack.push(value1);
                    }
                }
            }
            Opcode::DupX1 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                stack_frame.operand_stack.push(value1.clone());
                stack_frame.operand_stack.push(value2);
                stack_frame.operand_stack.push(value1);
            }
            Opcode::DupX2 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();
                if matches!(value2, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                } else {
                    let value3 = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value3);
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                }
            }
            Opcode::F2d => {
                let value = Self::float_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Double(value as _));
            }
            Opcode::F2i => {
                let value = Self::float_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Int(value as _));
            }
            Opcode::F2l => {
                let value = Self::float_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Long(value as _));
            }
            Opcode::Fadd => Self::float_binary(stack_frame, |x, y| x + y),
            Opcode::Fcmpg => Self::float_compare(stack_frame, 1),
            Opcode::Fcmpl => Self::float_compare(stack_frame, -1),
            Opcode::Fconst(x) => stack_frame.operand_stack.push(JavaValue::Float(*x as f32)),
            Opcode::Fdiv => Self::float_binary(stack_frame, |x, y| x / y),
            Opcode::Fmul => Self::float_binary(stack_frame, |x, y| x * y),
            Opcode::Fneg => {
                let value = Self::float_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Float(-value));
            }
            Opcode::Frem => Self::float_binary(stack_frame, |x, y| x % y),
            Opcode::Fsub => Self::float_binary(stack_frame, |x, y| x - y),
            Opcode::Goto(x) => return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32)),
            Opcode::GotoW(x) => return Some(ExecuteNext::Jump((current_offset as i32 + *x) as u32)),
            Opcode::I2b => {
                let value = Self::int_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Int((value as i8) as i32));
            }
            Opcode::I2c => {
                let value = Self::int_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Int(value as JavaChar as _));
            }
            Opcode::I2d => {
                let value = Self::int_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Double(value as _));
            }
            Opcode::I2f => {
                let value = Self::int_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Float(value as _));
            }
            Opcode::I2l => {
                let value = Self::int_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Long(value as _));
            }
            Opcode::I2s => {
                let value = Self::int_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Int((value as i16) as i32));
            }
            Opcode::Iadd => Self::int_binary(stack_frame, i32::wrapping_add),
            Opcode::Iand => Self::int_binary(stack_frame, |x, y| x & y),
            Opcode::Iconst(x) => stack_frame.operand_stack.push(JavaValue::Int(*x as i32)),
            Opcode::IfAcmpeq(x) => {
                let value2: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                let value1: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value1 == value2 {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfAcmpne(x) => {
                let value2: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                let value1: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value1 != value2 {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpeq(x) => {
                if Self::integer_condition(stack_frame, |x, y| x == y) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpge(x) => {
                if Self::integer_condition(stack_frame, |x, y| x >= y) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpgt(x) => {
                if Self::integer_condition(stack_frame, |x, y| x > y) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmple(x) => {
                if Self::integer_condition(stack_frame, |x, y| x <= y) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmplt(x) => {
                if Self::integer_condition(stack_frame, |x, y| x < y) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpne(x) => {
                if Self::integer_condition(stack_frame, |x, y| x != y) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifeq(x) => {
                if Self::integer_condition_single(stack_frame, |x| x == 0) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifge(x) => {
                if Self::integer_condition_single(stack_frame, |x| x >= 0) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifgt(x) => {
                if Self::integer_condition_single(stack_frame, |x| x > 0) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifle(x) => {
                if Self::integer_condition_single(stack_frame, |x| x <= 0) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Iflt(x) => {
                if Self::integer_condition_single(stack_frame, |x| x < 0) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifne(x) => {
                if Self::integer_condition_single(stack_frame, |x| x != 0) {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifnonnull(x) => {
                let value: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value.is_some() {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifnull(x) => {
                let value: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value.is_none() {
                    return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Iinc(x, y) => {
                let value = Self::int_value_ref(&stack_frame.local_variables[*x as usize]);
                stack_frame.local_variables[*x as usize] = JavaValue::Int(value.wrapping_add(*y as i32));
            }
            Opcode::Imul => Self::int_binary(stack_frame, i32::wrapping_mul),
            Opcode::Ineg => {
                let value = Self::int_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Int(value.wrapping_neg()));
            }
            Opcode::Instanceof(x) => {
                let instance: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                let result = if let Some(instance) = instance {
                    jvm.is_instance(&*instance, x.as_class())
                } else {
                    false
                };
                stack_frame.operand_stack.push(JavaValue::Int(result as _));
            }
            Opcode::Ior => Self::int_binary(stack_frame, |x, y| x | y),
            Opcode::Ishl => Self::int_shift(stack_frame, |x, y| x << (y & 0x1f)),
            Opcode::Ishr => Self::int_shift(stack_frame, |x, y| x >> (y & 0x1f)),
            Opcode::Isub => Self::int_binary(stack_frame, i32::wrapping_sub),
            Opcode::Iushr => Self::int_shift(stack_frame, |x, y| ((x as u32) >> ((y as u32) & 0x1f)) as _),
            Opcode::Ixor => Self::int_binary(stack_frame, |x, y| x ^ y),
            Opcode::Jsr(x) => {
                stack_frame.operand_stack.push(JavaValue::Int(current_offset as i32 + 3));
                return Some(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
            }
            Opcode::JsrW(x) => {
                stack_frame.operand_stack.push(JavaValue::Int(current_offset as i32 + 5));
                return Some(ExecuteNext::Jump((current_offset as i32 + *x) as u32));
            }
            Opcode::L2d => {
                let value = Self::long_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Double(value as _));
            }
            Opcode::L2f => {
                let value = Self::long_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Float(value as _));
            }
            Opcode::L2i => {
                let value = Self::long_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Int(value as _));
            }
            Opcode::Ladd => Self::long_binary(stack_frame, i64::wrapping_add),
            Opcode::Land => Self::long_binary(stack_frame, |x, y| x & y),
            Opcode::Lcmp => {
                let value2 = Self::long_value(stack_frame.operand_stack.pop().unwrap());
                let value1 = Self::long_value(stack_frame.operand_stack.pop().unwrap());

                stack_frame.operand_stack.push(JavaValue::Int(value1.cmp(&value2) as _));
            }
            Opcode::Lconst(x) => stack_frame.operand_stack.push(JavaValue::Long(*x as i64)),
            Opcode::Ldc(x) | Opcode::LdcW(x) | Opcode::Ldc2W(x) => {
                stack_frame.operand_stack.push(Self::constant_to_value_fast(x)?);
            }
            Opcode::Lmul => Self::long_binary(stack_frame, i64::wrapping_mul),
            Opcode::Lneg => {
                let value = Self::long_value(stack_frame.operand_stack.pop().unwrap());
                stack_frame.operand_stack.push(JavaValue::Long(value.wrapping_neg()));
            }
            Opcode::Lor => Self::long_binary(stack_frame, |x, y| x | y),
            Opcode::Lshl => Self::long_shift(stack_frame, |x, y| x << (y & 0x3f)),
            Opcode::Lshr => Self::long_shift(stack_frame, |x, y| x >> (y & 0x3f)),
            Opcode::Lsub => Self::long_binary(stack_frame, i64::wrapping_sub),
            Opcode::Lushr => Self::long_shift(stack_frame, |x, y| ((x as u64) >> ((y as u64) & 0x3f)) as _),
            Opcode::Lxor => Self::long_binary(stack_frame, |x, y| x ^ y),
            Opcode::Lookupswitch(default, pairs) | Opcode::Tableswitch(default, pairs) => {
                let key = Self::int_value(stack_frame.operand_stack.pop().unwrap());

                for (k, offset) in pairs {
                    if *k == key {
                        return Some(ExecuteNext::Jump((current_offset as i32 + *offset) as u32));
                    }
                }

                return Some(ExecuteNext::Jump((current_offset as i32 + *default) as u32));
            }
            Opcode::Monitorenter | Opcode::Monitorexit => {
                stack_frame.operand_stack.pop().unwrap();
            }
            Opcode::Nop => {}
            Opcode::Pop => {
                stack_frame.operand_stack.pop().unwrap();
            }
            Opcode::Pop2 => {
                let value = stack_frame.operand_stack.pop().unwrap();
                if !matches!(value, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.pop().unwrap();
                }
            }
            Opcode::Ret(x) => {
                let value = Self::int_value_ref(&stack_frame.local_variables[*x as usize]);
                return Some(ExecuteNext::Jump(value as u32));
            }
            Opcode::Return => return Some(ExecuteNext::Return(JavaValue::Void)),
            Opcode::Sipush(x) => stack_frame.operand_stack.push(JavaValue::Int(*x as i32)),
            Opcode::Swap => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                stack_frame.operand_stack.push(value1);
                stack_frame.operand_stack.push(value2);
            }
            _ => return None,
        }

        Some(ExecuteNext::Continue)
    }

    #[allow(clippy::too_many_arguments)]
    async fn execute_opcode<const PROFILE: bool>(
        jvm: &Jvm,
        instruction_index: usize,
        current_offset: u32,
        opcode: &Opcode,
        stack_frame: &mut StackFrame,
        return_type: &JavaType,
        resolved_static_fields: &mut [Option<ResolvedStaticField>],
        resolved_instance_fields: &mut [Option<ResolvedInstanceFieldSite>],
        resolved_static_methods: &mut [Option<ResolvedMethod>],
        resolved_special_methods: &mut [Option<ResolvedMethod>],
        resolved_instance_methods: &mut [Option<ResolvedInstanceMethod>],
    ) -> Result<ExecuteNext> {
        match opcode {
            Opcode::Aaload | Opcode::Baload | Opcode::Caload | Opcode::Daload | Opcode::Faload | Opcode::Iaload | Opcode::Laload | Opcode::Saload => {
                // TODO type checking
                let index: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let array: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                let value = Self::load_array_one(jvm, array, index).await?;

                stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
            }
            Opcode::Aastore
            | Opcode::Bastore
            | Opcode::Castore
            | Opcode::Dastore
            | Opcode::Fastore
            | Opcode::Iastore
            | Opcode::Lastore
            | Opcode::Sastore => {
                // TODO type checking
                let value = stack_frame.operand_stack.pop().unwrap();
                let index: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let mut array: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                Self::store_array_one(jvm, &mut array, index, value).await?;
            }
            Opcode::AconstNull => stack_frame.operand_stack.push(JavaValue::Object(None)),
            Opcode::Aload(x) | Opcode::Dload(x) | Opcode::Fload(x) | Opcode::Iload(x) | Opcode::Lload(x) => {
                let value = stack_frame.local_variables[*x as usize].clone();
                stack_frame.operand_stack.push(value);
            }
            Opcode::Athrow => {
                let exception: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if exception.is_none() {
                    return Err(jvm.exception("java/lang/NullPointerException", "null").await);
                }

                return Err(JavaError::JavaException(exception.unwrap()));
            }
            Opcode::Anewarray(x) => {
                let length: i32 = stack_frame.operand_stack.pop().unwrap().into();
                if length < 0 {
                    return Err(jvm.exception("java/lang/NegativeArraySizeException", &format!("{length}")).await);
                }
                let class_name = x.as_class();
                let element_type_name = if class_name.starts_with('[') {
                    alloc::string::String::from(class_name)
                } else {
                    format!("L{class_name};")
                };
                let array = jvm.instantiate_array(&element_type_name, length as _).await?;

                stack_frame.operand_stack.push(JavaValue::Object(Some(array)));
            }
            Opcode::Areturn | Opcode::Dreturn | Opcode::Freturn | Opcode::Ireturn | Opcode::Lreturn => {
                let return_value = stack_frame.operand_stack.pop().unwrap();
                if matches!(opcode, Opcode::Ireturn) {
                    let value: i32 = return_value.into();
                    if *return_type == JavaType::Boolean {
                        return Ok(ExecuteNext::Return(JavaValue::Boolean(value & 1 != 0)));
                    } else if *return_type == JavaType::Char {
                        return Ok(ExecuteNext::Return(JavaValue::Char(value as _)));
                    } else if *return_type == JavaType::Byte {
                        return Ok(ExecuteNext::Return(JavaValue::Byte(value as _)));
                    } else if *return_type == JavaType::Short {
                        return Ok(ExecuteNext::Return(JavaValue::Short(value as _)));
                    } else {
                        return Ok(ExecuteNext::Return(JavaValue::Int(value)));
                    }
                }
                return Ok(ExecuteNext::Return(return_value));
            }
            Opcode::Arraylength => {
                let array: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                if array.is_none() {
                    return Err(jvm.exception("java/lang/NullPointerException", "Array is null").await);
                }

                let length = jvm.array_length(&array.unwrap()).await?;
                stack_frame.operand_stack.push(JavaValue::Int(length as _));
            }
            Opcode::Astore(x) | Opcode::Dstore(x) | Opcode::Fstore(x) | Opcode::Istore(x) | Opcode::Lstore(x) => {
                let value = stack_frame.operand_stack.pop();
                stack_frame.local_variables[*x as usize] = value.unwrap();
            }
            Opcode::Bipush(x) => stack_frame.operand_stack.push(JavaValue::Int(*x as i32)),
            Opcode::Checkcast(x) => {
                let top_stack: &Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.last().unwrap().into();

                if !top_stack.is_none() && !jvm.is_instance(&**top_stack.as_ref().unwrap(), x.as_class()) {
                    return Err(jvm.exception("java/lang/ClassCastException", "Invalid cast").await);
                }
            }
            Opcode::D2f => {
                let value: f64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Float(value as _));
            }
            Opcode::D2i => {
                let value: f64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as _));
            }
            Opcode::D2l => {
                let value: f64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Long(value as _));
            }
            Opcode::Dadd => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 + value2));
            }
            Opcode::Dcmpg => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                if value1.is_nan() || value2.is_nan() {
                    stack_frame.operand_stack.push(JavaValue::Int(1));
                } else {
                    stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
                }
            }
            Opcode::Dcmpl => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                if value1.is_nan() || value2.is_nan() {
                    stack_frame.operand_stack.push(JavaValue::Int(-1));
                } else {
                    stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
                }
            }
            Opcode::Dconst(x) => {
                stack_frame.operand_stack.push(JavaValue::Double(*x as f64));
            }
            Opcode::Ddiv => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 / value2));
            }
            Opcode::Dmul => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 * value2));
            }
            Opcode::Dneg => {
                let value: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(-value));
            }
            Opcode::Dup => {
                let value = stack_frame.operand_stack.pop().unwrap();
                stack_frame.operand_stack.push(value.clone());
                stack_frame.operand_stack.push(value);
            }
            Opcode::Dup2 => {
                let value = stack_frame.operand_stack.pop().unwrap();
                if matches!(value, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.push(value.clone());
                    stack_frame.operand_stack.push(value);
                } else {
                    let value2 = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value2.clone());
                    stack_frame.operand_stack.push(value.clone());
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value);
                }
            }
            Opcode::Dup2X1 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                if matches!(value1, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                } else {
                    let value3 = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value2.clone());
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value3);
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                }
            }
            Opcode::Dup2X2 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                if matches!(value1, JavaValue::Long(_) | JavaValue::Double(_)) && matches!(value2, JavaValue::Long(_) | JavaValue::Double(_)) {
                    // form4
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                } else if matches!(value1, JavaValue::Long(_) | JavaValue::Double(_)) {
                    // form2
                    let value3 = stack_frame.operand_stack.pop().unwrap();

                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value3);
                    stack_frame.operand_stack.push(value2);
                    stack_frame.operand_stack.push(value1);
                } else {
                    let value3 = stack_frame.operand_stack.pop().unwrap();

                    if matches!(value3, JavaValue::Long(_) | JavaValue::Double(_)) {
                        // form3
                        stack_frame.operand_stack.push(value2.clone());
                        stack_frame.operand_stack.push(value1.clone());
                        stack_frame.operand_stack.push(value3);
                        stack_frame.operand_stack.push(value2);
                        stack_frame.operand_stack.push(value1);
                    } else {
                        // form1
                        let value4 = stack_frame.operand_stack.pop().unwrap();

                        stack_frame.operand_stack.push(value2.clone());
                        stack_frame.operand_stack.push(value1.clone());
                        stack_frame.operand_stack.push(value4);
                        stack_frame.operand_stack.push(value3);
                        stack_frame.operand_stack.push(value2);
                        stack_frame.operand_stack.push(value1);
                    }
                }
            }
            Opcode::DupX1 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                stack_frame.operand_stack.push(value1.clone());
                stack_frame.operand_stack.push(value2.clone());
                stack_frame.operand_stack.push(value1);
            }
            Opcode::DupX2 => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();
                if matches!(value2, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value2.clone());
                    stack_frame.operand_stack.push(value1);
                } else {
                    let value3 = stack_frame.operand_stack.pop().unwrap();
                    stack_frame.operand_stack.push(value1.clone());
                    stack_frame.operand_stack.push(value3.clone());
                    stack_frame.operand_stack.push(value2.clone());
                    stack_frame.operand_stack.push(value1);
                }
            }
            Opcode::Drem => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 % value2));
            }
            Opcode::Dsub => {
                let value2: f64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Double(value1 - value2));
            }
            Opcode::F2d => {
                let value: f32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Double(value as _));
            }
            Opcode::F2i => {
                let value: f32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as _));
            }
            Opcode::F2l => {
                let value: f32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Long(value as _));
            }
            Opcode::Fadd => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 + value2));
            }
            Opcode::Fcmpg => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                if value1.is_nan() || value2.is_nan() {
                    stack_frame.operand_stack.push(JavaValue::Int(1));
                } else {
                    stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
                }
            }
            Opcode::Fcmpl => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                if value1.is_nan() || value2.is_nan() {
                    stack_frame.operand_stack.push(JavaValue::Int(-1));
                } else {
                    stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
                }
            }
            Opcode::Fconst(x) => {
                stack_frame.operand_stack.push(JavaValue::Float(*x as f32));
            }
            Opcode::Fdiv => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 / value2));
            }
            Opcode::Fmul => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 * value2));
            }
            Opcode::Fneg => {
                let value: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(-value));
            }
            Opcode::Frem => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 % value2));
            }
            Opcode::Fsub => {
                let value2: f32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: f32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Float(value1 - value2));
            }
            Opcode::Getfield(x) => {
                let x = x.as_field_ref();
                let instance: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if instance.is_none() {
                    return Err(jvm.exception("java/lang/NullPointerException", "null").await);
                }

                let instance = instance.unwrap();
                if resolved_instance_fields[instruction_index].is_none() {
                    let field = jvm.resolve_instance_field(&x.class, &x.name, &x.descriptor).await?;
                    resolved_instance_fields[instruction_index] = Some(ResolvedInstanceFieldSite { field });
                }

                let value = jvm.get_resolved_instance_field(&resolved_instance_fields[instruction_index].as_ref().unwrap().field, &instance)?;

                stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
            }
            Opcode::Getstatic(x) => {
                let x = x.as_field_ref();
                if resolved_static_fields[instruction_index].is_none() {
                    resolved_static_fields[instruction_index] = Some(jvm.resolve_static_field(&x.class, &x.name, &x.descriptor).await?);
                }
                let value = jvm.get_resolved_static_field(resolved_static_fields[instruction_index].as_ref().unwrap())?;

                stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
            }
            Opcode::Goto(x) => return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32)),
            Opcode::GotoW(x) => return Ok(ExecuteNext::Jump((current_offset as i32 + *x) as u32)),
            Opcode::I2b => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int((value as i8) as i32));
            }
            Opcode::I2c => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as JavaChar as _));
            }
            Opcode::I2d => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Double(value as _));
            }
            Opcode::I2f => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Float(value as _));
            }
            Opcode::I2l => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Long(value as _));
            }
            Opcode::I2s => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int((value as i16) as i32));
            }
            Opcode::Iadd => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1.wrapping_add(value2)));
            }
            Opcode::Iand => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 & value2));
            }
            Opcode::Iconst(x) => stack_frame.operand_stack.push(JavaValue::Int(*x as i32)),
            Opcode::Idiv => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                if value2 == 0 {
                    return Err(jvm.exception("java/lang/ArithmeticException", "Division by zero").await);
                }

                stack_frame.operand_stack.push(JavaValue::Int(value1.wrapping_div(value2)));
            }
            Opcode::IfAcmpeq(x) => {
                let value2: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                let value1: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value1 == value2 {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfAcmpne(x) => {
                let value2: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                let value1: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value1 != value2 {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpeq(x) => {
                if Self::integer_condition(stack_frame, |x, y| x == y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpge(x) => {
                if Self::integer_condition(stack_frame, |x, y| x >= y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpgt(x) => {
                if Self::integer_condition(stack_frame, |x, y| x > y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmple(x) => {
                if Self::integer_condition(stack_frame, |x, y| x <= y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmplt(x) => {
                if Self::integer_condition(stack_frame, |x, y| x < y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::IfIcmpne(x) => {
                if Self::integer_condition(stack_frame, |x, y| x != y) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifeq(x) => {
                if Self::integer_condition_single(stack_frame, |x| x == 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifge(x) => {
                if Self::integer_condition_single(stack_frame, |x| x >= 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifgt(x) => {
                if Self::integer_condition_single(stack_frame, |x| x > 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifle(x) => {
                if Self::integer_condition_single(stack_frame, |x| x <= 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Iflt(x) => {
                if Self::integer_condition_single(stack_frame, |x| x < 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifne(x) => {
                if Self::integer_condition_single(stack_frame, |x| x != 0) {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifnonnull(x) => {
                let value: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value.is_some() {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Ifnull(x) => {
                let value: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if value.is_none() {
                    return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
                }
            }
            Opcode::Iinc(x, y) => {
                let value = stack_frame.local_variables[*x as usize].clone();
                let value: i32 = value.into();

                stack_frame.local_variables[*x as usize] = JavaValue::Int(value.wrapping_add(*y as i32));
            }
            Opcode::Imul => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1.wrapping_mul(value2)));
            }
            Opcode::Ineg => {
                let value: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value.wrapping_neg()));
            }
            Opcode::Instanceof(x) => {
                let instance: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                let result = if let Some(instance) = instance {
                    jvm.is_instance(&*instance, x.as_class())
                } else {
                    false
                };
                stack_frame.operand_stack.push(JavaValue::Int(result as _));
            }
            Opcode::Invokedynamic(_) => {
                todo!()
            }
            Opcode::Invokeinterface(x, _count, _zero) => {
                if PROFILE {
                    profile::invoke_interface();
                }
                let x = x.as_interface_method_ref();
                let params = Self::extract_invoke_params(stack_frame, &x.method_param_kinds);

                let instance: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                if instance.is_none() {
                    return Err(jvm
                        .exception(
                            "java/lang/NullPointerException",
                            &format!("Method {}::{}:{} is called on null", x.class, x.name, x.descriptor),
                        )
                        .await);
                }

                let instance = instance.unwrap();
                let receiver_class = instance.class_definition().name();
                let needs_resolve = match &resolved_instance_methods[instruction_index] {
                    Some(resolved) => resolved.receiver_class != receiver_class,
                    None => true,
                };
                if needs_resolve {
                    let method = jvm.resolve_virtual_method_for_instance(&instance, &x.name, &x.descriptor).await?;
                    resolved_instance_methods[instruction_index] = Some(ResolvedInstanceMethod { receiver_class, method });
                }

                let result = jvm
                    .invoke_resolved_instance(
                        &resolved_instance_methods[instruction_index].as_ref().unwrap().method,
                        &instance,
                        params.into_boxed_slice(),
                    )
                    .await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Invokespecial(x) => {
                if PROFILE {
                    profile::invoke_special();
                }
                let x = x.as_method_ref();
                let params = Self::extract_invoke_params(stack_frame, &x.method_param_kinds);

                let instance: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                if instance.is_none() {
                    return Err(jvm
                        .exception(
                            "java/lang/NullPointerException",
                            &format!("Method {}::{}:{} is called on null", x.class, x.name, x.descriptor),
                        )
                        .await);
                }

                let instance = instance.unwrap();
                if resolved_special_methods[instruction_index].is_none() {
                    resolved_special_methods[instruction_index] = Some(jvm.resolve_special_method(&x.class, &x.name, &x.descriptor).await?);
                }
                let result = jvm
                    .invoke_resolved_instance(
                        resolved_special_methods[instruction_index].as_ref().unwrap(),
                        &instance,
                        params.into_boxed_slice(),
                    )
                    .await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Invokestatic(x) => {
                if PROFILE {
                    profile::invoke_static();
                }
                let x = x.as_method_ref();
                let params = Self::extract_invoke_params(stack_frame, &x.method_param_kinds);

                if resolved_static_methods[instruction_index].is_none() {
                    resolved_static_methods[instruction_index] = Some(jvm.resolve_static_method(&x.class, &x.name, &x.descriptor).await?);
                }
                let result = jvm
                    .invoke_resolved_static(resolved_static_methods[instruction_index].as_ref().unwrap(), params.into_boxed_slice())
                    .await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Invokevirtual(x) => {
                if PROFILE {
                    profile::invoke_virtual();
                }
                let x = x.as_method_ref();
                let params = Self::extract_invoke_params(stack_frame, &x.method_param_kinds);

                let instance: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                if instance.is_none() {
                    return Err(jvm
                        .exception(
                            "java/lang/NullPointerException",
                            &format!("Method {}::{}:{} is called on null", x.class, x.name, x.descriptor),
                        )
                        .await);
                }

                let instance = instance.unwrap();
                let receiver_class = instance.class_definition().name();
                let needs_resolve = match &resolved_instance_methods[instruction_index] {
                    Some(resolved) => resolved.receiver_class != receiver_class,
                    None => true,
                };
                if needs_resolve {
                    let method = jvm.resolve_virtual_method_for_instance(&instance, &x.name, &x.descriptor).await?;
                    resolved_instance_methods[instruction_index] = Some(ResolvedInstanceMethod { receiver_class, method });
                }

                let result = jvm
                    .invoke_resolved_instance(
                        &resolved_instance_methods[instruction_index].as_ref().unwrap().method,
                        &instance,
                        params.into_boxed_slice(),
                    )
                    .await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Ior => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 | value2));
            }
            Opcode::Irem => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                if value2 == 0 {
                    return Err(jvm.exception("java/lang/ArithmeticException", "Division by zero").await);
                }

                stack_frame.operand_stack.push(JavaValue::Int(value1.wrapping_rem(value2)));
            }
            Opcode::Ishl => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 << (value2 & 0x1f)));
            }
            Opcode::Ishr => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 >> (value2 & 0x1f)));
            }
            Opcode::Isub => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1.wrapping_sub(value2)));
            }
            Opcode::Iushr => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame
                    .operand_stack
                    .push(JavaValue::Int(((value1 as u32) >> ((value2 as u32) & 0x1f)) as _));
            }
            Opcode::Ixor => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i32 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1 ^ value2));
            }
            Opcode::Jsr(x) => {
                stack_frame.operand_stack.push(JavaValue::Int(current_offset as i32 + 3));

                return Ok(ExecuteNext::Jump((current_offset as i32 + *x as i32) as u32));
            }
            Opcode::JsrW(x) => {
                stack_frame.operand_stack.push(JavaValue::Int(current_offset as i32 + 5));

                return Ok(ExecuteNext::Jump((current_offset as i32 + *x) as u32));
            }
            Opcode::L2d => {
                let value: i64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Double(value as _));
            }
            Opcode::L2f => {
                let value: i64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Float(value as _));
            }
            Opcode::L2i => {
                let value: i64 = stack_frame.operand_stack.pop().unwrap().into();
                stack_frame.operand_stack.push(JavaValue::Int(value as _));
            }
            Opcode::Ladd => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1.wrapping_add(value2)));
            }
            Opcode::Land => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 & value2));
            }
            Opcode::Lcmp => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Int(value1.cmp(&value2) as _));
            }
            Opcode::Lconst(x) => stack_frame.operand_stack.push(JavaValue::Long(*x as i64)),
            Opcode::Ldc(x) | Opcode::LdcW(x) => stack_frame.operand_stack.push(Self::constant_to_value(jvm, x).await?),
            Opcode::Ldc2W(x) => stack_frame.operand_stack.push(Self::constant_to_value(jvm, x).await?),
            Opcode::Ldiv => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                if value2 == 0 {
                    return Err(jvm.exception("java/lang/ArithmeticException", "Division by zero").await);
                }

                stack_frame.operand_stack.push(JavaValue::Long(value1.wrapping_div(value2)));
            }
            Opcode::Lmul => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1.wrapping_mul(value2)));
            }
            Opcode::Lneg => {
                let value: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value.wrapping_neg()));
            }
            Opcode::Lor => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 | value2));
            }
            Opcode::Lrem => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                if value2 == 0 {
                    return Err(jvm.exception("java/lang/ArithmeticException", "Division by zero").await);
                }

                stack_frame.operand_stack.push(JavaValue::Long(value1.wrapping_rem(value2)));
            }
            Opcode::Lshl => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 << (value2 & 0x3f)));
            }
            Opcode::Lshr => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 >> (value2 & 0x3f)));
            }
            Opcode::Lsub => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1.wrapping_sub(value2)));
            }
            Opcode::Lushr => {
                let value2: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame
                    .operand_stack
                    .push(JavaValue::Long(((value1 as u64) >> ((value2 as u64) & 0x3f)) as _));
            }
            Opcode::Lxor => {
                let value2: i64 = stack_frame.operand_stack.pop().unwrap().into();
                let value1: i64 = stack_frame.operand_stack.pop().unwrap().into();

                stack_frame.operand_stack.push(JavaValue::Long(value1 ^ value2));
            }
            Opcode::Lookupswitch(default, pairs) | Opcode::Tableswitch(default, pairs) => {
                let key = stack_frame.operand_stack.pop().unwrap().into();

                for (k, offset) in pairs {
                    if *k == key {
                        return Ok(ExecuteNext::Jump((current_offset as i32 + *offset) as u32));
                    }
                }

                return Ok(ExecuteNext::Jump((current_offset as i32 + *default) as u32));
            }
            Opcode::Monitorenter => {
                let stack_value = stack_frame.operand_stack.pop().unwrap();
                tracing::warn!("Unimplemented monitorenter{stack_value:?}");
            }
            Opcode::Monitorexit => {
                let stack_value = stack_frame.operand_stack.pop().unwrap();
                tracing::warn!("Unimplemented monitorexit{stack_value:?}");
            }
            Opcode::Multianewarray(x, d) => {
                let mut dimensions: Vec<i32> = (0..*d).map(|_| stack_frame.operand_stack.pop().unwrap().into()).collect();
                dimensions.reverse();
                for dim in &dimensions {
                    if *dim < 0 {
                        return Err(jvm.exception("java/lang/NegativeArraySizeException", &format!("{dim}")).await);
                    }
                }

                let array = Self::new_multi_array(jvm, x.as_class(), &dimensions).await?;

                stack_frame.operand_stack.push(JavaValue::Object(Some(array)));
            }
            Opcode::New(x) => {
                let class = jvm.instantiate_class(x.as_class()).await?;

                stack_frame.operand_stack.push(JavaValue::Object(Some(class)));
            }
            Opcode::Newarray(x) => {
                let element_type_name = match x {
                    4 => "Z",
                    5 => "C",
                    6 => "F",
                    7 => "D",
                    8 => "B",
                    9 => "S",
                    10 => "I",
                    11 => "J",
                    _ => panic!("Invalid array type {}", x),
                };

                let length: i32 = stack_frame.operand_stack.pop().unwrap().into();
                if length < 0 {
                    return Err(jvm.exception("java/lang/NegativeArraySizeException", &format!("{length}")).await);
                }
                let array = jvm.instantiate_array(element_type_name, length as _).await?;

                stack_frame.operand_stack.push(JavaValue::Object(Some(array)));
            }
            Opcode::Nop => {}
            Opcode::Pop => {
                stack_frame.operand_stack.pop().unwrap();
            }
            Opcode::Pop2 => {
                let value = stack_frame.operand_stack.pop().unwrap();
                if !matches!(value, JavaValue::Long(_) | JavaValue::Double(_)) {
                    stack_frame.operand_stack.pop().unwrap();
                }
            }
            Opcode::Putfield(x) => {
                let x = x.as_field_ref();
                let value = stack_frame.operand_stack.pop().unwrap();
                let mut instance: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if instance.is_none() {
                    return Err(jvm.exception("java/lang/NullPointerException", "null").await);
                }

                let value = Self::to_field_type(&x.descriptor, value);

                if resolved_instance_fields[instruction_index].is_none() {
                    let field = jvm.resolve_instance_field(&x.class, &x.name, &x.descriptor).await?;
                    resolved_instance_fields[instruction_index] = Some(ResolvedInstanceFieldSite { field });
                }

                jvm.put_resolved_instance_field(
                    &mut resolved_instance_fields[instruction_index].as_mut().unwrap().field,
                    instance.as_mut().unwrap(),
                    value,
                )?;
            }
            Opcode::Putstatic(x) => {
                let x = x.as_field_ref();
                let value = Self::to_field_type(&x.descriptor, stack_frame.operand_stack.pop().unwrap());

                if resolved_static_fields[instruction_index].is_none() {
                    resolved_static_fields[instruction_index] = Some(jvm.resolve_static_field(&x.class, &x.name, &x.descriptor).await?);
                }
                jvm.put_resolved_static_field(resolved_static_fields[instruction_index].as_mut().unwrap(), value)?
            }
            Opcode::Ret(x) => {
                let value = stack_frame.local_variables[*x as usize].clone();
                let value: i32 = value.into();

                return Ok(ExecuteNext::Jump(value as u32));
            }
            Opcode::Return => return Ok(ExecuteNext::Return(JavaValue::Void)),
            Opcode::Sipush(x) => stack_frame.operand_stack.push(JavaValue::Int(*x as i32)),
            Opcode::Swap => {
                let value1 = stack_frame.operand_stack.pop().unwrap();
                let value2 = stack_frame.operand_stack.pop().unwrap();

                stack_frame.operand_stack.push(value1);
                stack_frame.operand_stack.push(value2);
            }
            Opcode::Wide => {
                todo!()
            }
        }

        Ok(ExecuteNext::Continue)
    }

    async fn load_array_one(jvm: &Jvm, array: Option<Box<dyn ClassInstance>>, index: i32) -> Result<JavaValue> {
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

    async fn store_array_one(jvm: &Jvm, array: &mut Option<Box<dyn ClassInstance>>, index: i32, value: JavaValue) -> Result<()> {
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

    async fn find_exception_handler(jvm: &Jvm, exception: &dyn ClassInstance, code_attribute: &AttributeInfoCode, pc: u32) -> Option<u32> {
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

    fn try_load_array_one_stack(array: Option<&dyn ClassInstance>, index: i32) -> Option<JavaValue> {
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

    fn try_store_array_one_stack(array: &mut Box<dyn ClassInstance>, index: i32, value: JavaValue) -> bool {
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

    fn int_value(value: JavaValue) -> i32 {
        match value {
            JavaValue::Boolean(value) => value as i32,
            JavaValue::Byte(value) => value as i32,
            JavaValue::Char(value) => value as i32,
            JavaValue::Short(value) => value as i32,
            JavaValue::Int(value) => value,
            _ => panic!("Expected integer-compatible value, got {value:?}"),
        }
    }

    fn int_value_ref(value: &JavaValue) -> i32 {
        match value {
            JavaValue::Boolean(value) => *value as i32,
            JavaValue::Byte(value) => *value as i32,
            JavaValue::Char(value) => *value as i32,
            JavaValue::Short(value) => *value as i32,
            JavaValue::Int(value) => *value,
            _ => panic!("Expected integer-compatible value, got {value:?}"),
        }
    }

    fn long_value(value: JavaValue) -> i64 {
        match value {
            JavaValue::Long(value) => value,
            _ => panic!("Expected long value, got {value:?}"),
        }
    }

    fn float_value(value: JavaValue) -> f32 {
        match value {
            JavaValue::Float(value) => value,
            _ => panic!("Expected float value, got {value:?}"),
        }
    }

    fn double_value(value: JavaValue) -> f64 {
        match value {
            JavaValue::Double(value) => value,
            _ => panic!("Expected double value, got {value:?}"),
        }
    }

    fn int_binary<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(i32, i32) -> i32,
    {
        let value2 = Self::int_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::int_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Int(operation(value1, value2)));
    }

    fn int_shift<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(i32, i32) -> i32,
    {
        let value2 = Self::int_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::int_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Int(operation(value1, value2)));
    }

    fn long_binary<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(i64, i64) -> i64,
    {
        let value2 = Self::long_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::long_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Long(operation(value1, value2)));
    }

    fn long_shift<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(i64, i32) -> i64,
    {
        let value2 = Self::int_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::long_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Long(operation(value1, value2)));
    }

    fn float_binary<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(f32, f32) -> f32,
    {
        let value2 = Self::float_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::float_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Float(operation(value1, value2)));
    }

    fn float_compare(stack_frame: &mut StackFrame, nan_value: i32) {
        let value2 = Self::float_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::float_value(stack_frame.operand_stack.pop().unwrap());

        if value1.is_nan() || value2.is_nan() {
            stack_frame.operand_stack.push(JavaValue::Int(nan_value));
        } else {
            stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
        }
    }

    fn double_binary<F>(stack_frame: &mut StackFrame, operation: F)
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let value2 = Self::double_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::double_value(stack_frame.operand_stack.pop().unwrap());

        stack_frame.operand_stack.push(JavaValue::Double(operation(value1, value2)));
    }

    fn double_compare(stack_frame: &mut StackFrame, nan_value: i32) {
        let value2 = Self::double_value(stack_frame.operand_stack.pop().unwrap());
        let value1 = Self::double_value(stack_frame.operand_stack.pop().unwrap());

        if value1.is_nan() || value2.is_nan() {
            stack_frame.operand_stack.push(JavaValue::Int(nan_value));
        } else {
            stack_frame.operand_stack.push(JavaValue::Int(value1.partial_cmp(&value2).unwrap() as _));
        }
    }

    fn constant_to_value_fast(constant: &ConstantPoolReference) -> Option<JavaValue> {
        Some(match constant {
            ConstantPoolReference::Integer(x) => JavaValue::Int(*x),
            ConstantPoolReference::Float(x) => JavaValue::Float(*x),
            ConstantPoolReference::Long(x) => JavaValue::Long(*x),
            ConstantPoolReference::Double(x) => JavaValue::Double(*x),
            _ => return None,
        })
    }

    fn integer_condition<T>(stack_frame: &mut StackFrame, pred: T) -> bool
    where
        T: Fn(i32, i32) -> bool,
    {
        let value2 = stack_frame.operand_stack.pop().unwrap().into();
        let value1 = stack_frame.operand_stack.pop().unwrap().into();

        pred(value1, value2)
    }

    fn integer_condition_single<T>(stack_frame: &mut StackFrame, pred: T) -> bool
    where
        T: Fn(i32) -> bool,
    {
        let value = stack_frame.operand_stack.pop().unwrap().into();

        pred(value)
    }

    fn extract_invoke_params(stack_frame: &mut StackFrame, param_kinds: &[MethodParamKind]) -> Vec<JavaValue> {
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

    fn push_invoke_result(stack_frame: &mut StackFrame, value: JavaValue) {
        match value {
            JavaValue::Void => {}
            _ => stack_frame.operand_stack.push(Self::to_stack_frame_type(value)),
        }
    }

    // operand stack has only integer for small number types, so convert it to the field type
    fn to_field_type(descriptor: &str, value: JavaValue) -> JavaValue {
        match descriptor {
            "Z" => JavaValue::Boolean(i32::from(value) & 1 != 0),
            "B" => JavaValue::Byte(i32::from(value) as i8),
            "C" => JavaValue::Char(i32::from(value) as u16),
            "S" => JavaValue::Short(i32::from(value) as i16),
            _ => value,
        }
    }

    // convert into stack frame type, which is always integer for number types
    fn to_stack_frame_type(value: JavaValue) -> JavaValue {
        match value {
            JavaValue::Boolean(x) => JavaValue::Int(x as _),
            JavaValue::Byte(x) => JavaValue::Int(x as _),
            JavaValue::Char(x) => JavaValue::Int(x as _),
            JavaValue::Short(x) => JavaValue::Int(x as _),
            _ => value,
        }
    }

    async fn constant_to_value(jvm: &Jvm, constant: &ConstantPoolReference) -> Result<JavaValue> {
        Ok(match constant {
            ConstantPoolReference::Integer(x) => JavaValue::Int(*x),
            ConstantPoolReference::Float(x) => JavaValue::Float(*x),
            ConstantPoolReference::Long(x) => JavaValue::Long(*x),
            ConstantPoolReference::Double(x) => JavaValue::Double(*x),
            ConstantPoolReference::String(x) => JavaValue::Object(Some(JavaLangString::from_rust_string(jvm, x).await?)),
            ConstantPoolReference::Class(x) => JavaValue::Object(Some(jvm.resolve_class(x).await?.java_class())),
            _ => unimplemented!(),
        })
    }

    #[async_recursion::async_recursion]
    async fn new_multi_array(jvm: &Jvm, array_class: &str, dimensions: &[i32]) -> Result<Box<dyn ClassInstance>> {
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
