use alloc::boxed::Box;

use classfile::Opcode;
use jvm::{ClassInstance, JavaChar, JavaType, JavaValue, Jvm};

use crate::stack_frame::StackFrame;

use super::{ExecuteNext, InstructionCache, Interpreter, MethodBytecodeCache};

impl Interpreter {
    #[allow(clippy::too_many_lines)]
    pub(super) fn execute_fast_opcode<const PROFILE: bool>(
        jvm: &Jvm,
        instruction_index: usize,
        current_offset: u32,
        opcode: &Opcode,
        stack_frame: &mut StackFrame,
        return_type: &JavaType,
        cache: &MethodBytecodeCache,
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
            Opcode::Bastore | Opcode::Castore | Opcode::Dastore | Opcode::Fastore | Opcode::Iastore | Opcode::Lastore | Opcode::Sastore => {
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
            Opcode::Getfield(_) => {
                if let Some(InstructionCache::InstanceField(lock)) = cache.entries.get(instruction_index) {
                    let guard = lock.read();
                    if let Some(field) = guard.as_ref() {
                        if let Some(JavaValue::Object(Some(instance))) = stack_frame.operand_stack.last() {
                            let value = jvm.get_resolved_instance_field(field, instance).ok()?;
                            stack_frame.operand_stack.pop();
                            stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
                            return Some(ExecuteNext::Continue);
                        }
                    }
                }
                return None;
            }
            Opcode::Getstatic(_) => {
                if let Some(InstructionCache::StaticField(lock)) = cache.entries.get(instruction_index) {
                    let guard = lock.read();
                    if let Some(field) = guard.as_ref() {
                        let value = jvm.get_resolved_static_field(field).ok()?;
                        stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
                        return Some(ExecuteNext::Continue);
                    }
                }
                return None;
            }
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
                    jvm.is_instance(&*instance, x.try_as_class()?)
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
            Opcode::Monitorenter | Opcode::Monitorexit | Opcode::Aastore | Opcode::Unknown(_) => return None,
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
                if let Some(InstructionCache::InstanceField(lock)) = cache.entries.get(instruction_index) {
                    let guard = lock.read();
                    if let Some(field) = guard.as_ref() {
                        let len = stack_frame.operand_stack.len();
                        if len >= 2 {
                            let x = x.try_as_field_ref()?;
                            let value = Self::to_field_type(&x.descriptor, stack_frame.operand_stack[len - 1].clone());
                            if let JavaValue::Object(Some(instance)) = &mut stack_frame.operand_stack[len - 2] {
                                if jvm.put_resolved_instance_field(field, instance, value).is_ok() {
                                    stack_frame.operand_stack.pop();
                                    stack_frame.operand_stack.pop();
                                    return Some(ExecuteNext::Continue);
                                }
                            }
                        }
                    }
                }
                return None;
            }
            Opcode::Putstatic(x) => {
                if let Some(InstructionCache::StaticField(lock)) = cache.entries.get(instruction_index) {
                    let guard = lock.read();
                    if let Some(field) = guard.as_ref() {
                        if let Some(top) = stack_frame.operand_stack.last() {
                            let x = x.try_as_field_ref()?;
                            let value = Self::to_field_type(&x.descriptor, top.clone());
                            if jvm.put_resolved_static_field(field, value).is_ok() {
                                stack_frame.operand_stack.pop();
                                return Some(ExecuteNext::Continue);
                            }
                        }
                    }
                }
                return None;
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
}
