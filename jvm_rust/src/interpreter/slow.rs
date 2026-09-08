use alloc::{boxed::Box, format, vec::Vec};

use classfile::Opcode;
use jvm::{ClassInstance, JavaChar, JavaError, JavaType, JavaValue, Jvm, Result};

use crate::{profile, stack_frame::StackFrame};

use super::{ExecuteNext, InstructionCache, Interpreter, MethodBytecodeCache, ResolvedInstanceMethodSite};

impl Interpreter {
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn execute_opcode<const PROFILE: bool>(
        jvm: &Jvm,
        instruction_index: usize,
        current_offset: u32,
        opcode: &Opcode,
        stack_frame: &mut StackFrame,
        return_type: &JavaType,
        cache: &MethodBytecodeCache,
    ) -> Result<ExecuteNext> {
        match opcode {
            Opcode::Aaload | Opcode::Baload | Opcode::Caload | Opcode::Daload | Opcode::Faload | Opcode::Iaload | Opcode::Laload | Opcode::Saload => {
                // TODO type checking
                let index: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let array: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                let value = Self::load_array_one(jvm, array, index).await?;

                stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
            }
            Opcode::Aastore => {
                let value = stack_frame.operand_stack.pop().unwrap();
                let index: i32 = stack_frame.operand_stack.pop().unwrap().into();
                let mut array: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();
                if let Some(array_ref) = array.as_ref()
                    && let JavaValue::Object(Some(stored)) = &value
                    && let Some(element_class) = Self::array_element_class_name(&array_ref.class_definition().name())
                    && !jvm.is_instance(&**stored, element_class)
                {
                    return Err(jvm.exception("java/lang/ArrayStoreException", element_class).await);
                }
                Self::store_array_one(jvm, &mut array, index, value).await?;
            }
            Opcode::Bastore | Opcode::Castore | Opcode::Dastore | Opcode::Fastore | Opcode::Iastore | Opcode::Lastore | Opcode::Sastore => {
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
                let Some(class_name) = x.try_as_class() else {
                    return Err(Self::invalid_cp(jvm, "class").await);
                };
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

                let Some(class_name) = x.try_as_class() else {
                    return Err(Self::invalid_cp(jvm, "class").await);
                };
                if !top_stack.is_none() && !jvm.is_instance(&**top_stack.as_ref().unwrap(), class_name) {
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
                let Some(x) = x.try_as_field_ref() else {
                    return Err(Self::invalid_cp(jvm, "Fieldref").await);
                };
                let instance: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if instance.is_none() {
                    return Err(jvm.exception("java/lang/NullPointerException", "null").await);
                }

                let instance = instance.unwrap();
                let cached = match cache.entries.get(instruction_index) {
                    Some(InstructionCache::InstanceField(lock)) => lock.read().as_ref().cloned(),
                    _ => None,
                };
                let field = match cached {
                    Some(field) => field,
                    None => {
                        let field = jvm.resolve_instance_field(&x.class, &x.name, &x.descriptor).await?;
                        if let Some(InstructionCache::InstanceField(lock)) = cache.entries.get(instruction_index) {
                            *lock.write() = Some(field.clone());
                        }
                        field
                    }
                };

                let value = jvm.get_resolved_instance_field(&field, &instance)?;

                stack_frame.operand_stack.push(Self::to_stack_frame_type(value));
            }
            Opcode::Getstatic(x) => {
                let Some(x) = x.try_as_field_ref() else {
                    return Err(Self::invalid_cp(jvm, "Fieldref").await);
                };
                let cached = match cache.entries.get(instruction_index) {
                    Some(InstructionCache::StaticField(lock)) => lock.read().as_ref().cloned(),
                    _ => None,
                };
                let field = match cached {
                    Some(field) => field,
                    None => {
                        let field = jvm.resolve_static_field(&x.class, &x.name, &x.descriptor).await?;
                        if let Some(InstructionCache::StaticField(lock)) = cache.entries.get(instruction_index) {
                            *lock.write() = Some(field.clone());
                        }
                        field
                    }
                };
                let value = jvm.get_resolved_static_field(&field)?;

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

                let Some(class_name) = x.try_as_class() else {
                    return Err(Self::invalid_cp(jvm, "class").await);
                };
                let result = if let Some(instance) = instance {
                    jvm.is_instance(&*instance, class_name)
                } else {
                    false
                };
                stack_frame.operand_stack.push(JavaValue::Int(result as _));
            }
            Opcode::Invokedynamic(_) => {
                return Err(jvm.exception("java/lang/LinkageError", "invokedynamic not supported").await);
            }
            Opcode::Invokeinterface(x, _count, _zero) => {
                if PROFILE {
                    profile::invoke_interface();
                }
                let Some(x) = x.try_as_interface_method_ref() else {
                    return Err(Self::invalid_cp(jvm, "InterfaceMethodref").await);
                };
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
                let cached = match cache.entries.get(instruction_index) {
                    Some(InstructionCache::InstanceMethod(lock)) => {
                        let guard = lock.read();
                        guard.as_ref().and_then(|site| {
                            if site.receiver_class == receiver_class {
                                Some(site.method.clone())
                            } else {
                                None
                            }
                        })
                    }
                    _ => None,
                };

                let method = match cached {
                    Some(m) => m,
                    None => {
                        let m = jvm.resolve_virtual_method_for_instance(&instance, &x.name, &x.descriptor).await?;
                        if let Some(InstructionCache::InstanceMethod(lock)) = cache.entries.get(instruction_index) {
                            *lock.write() = Some(ResolvedInstanceMethodSite {
                                receiver_class,
                                method: m.clone(),
                            });
                        }
                        m
                    }
                };

                let result = jvm.invoke_resolved_instance(&method, &instance, params.into_boxed_slice()).await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Invokespecial(x) => {
                if PROFILE {
                    profile::invoke_special();
                }
                let Some(x) = x.try_as_method_ref() else {
                    return Err(Self::invalid_cp(jvm, "Methodref").await);
                };
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
                let cached = match cache.entries.get(instruction_index) {
                    Some(InstructionCache::SpecialMethod(lock)) => lock.read().as_ref().cloned(),
                    _ => None,
                };
                let method = match cached {
                    Some(method) => method,
                    None => {
                        let method = jvm.resolve_special_method(&x.class, &x.name, &x.descriptor).await?;
                        if let Some(InstructionCache::SpecialMethod(lock)) = cache.entries.get(instruction_index) {
                            *lock.write() = Some(method.clone());
                        }
                        method
                    }
                };
                let result = jvm.invoke_resolved_instance(&method, &instance, params.into_boxed_slice()).await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Invokestatic(x) => {
                if PROFILE {
                    profile::invoke_static();
                }
                let Some(x) = x.try_as_method_ref() else {
                    return Err(Self::invalid_cp(jvm, "Methodref").await);
                };
                let params = Self::extract_invoke_params(stack_frame, &x.method_param_kinds);

                let cached = match cache.entries.get(instruction_index) {
                    Some(InstructionCache::StaticMethod(lock)) => lock.read().as_ref().cloned(),
                    _ => None,
                };
                let method = match cached {
                    Some(method) => method,
                    None => {
                        let method = jvm.resolve_static_method(&x.class, &x.name, &x.descriptor).await?;
                        if let Some(InstructionCache::StaticMethod(lock)) = cache.entries.get(instruction_index) {
                            *lock.write() = Some(method.clone());
                        }
                        method
                    }
                };
                let result = jvm.invoke_resolved_static(&method, params.into_boxed_slice()).await?;
                Self::push_invoke_result(stack_frame, result);
            }
            Opcode::Invokevirtual(x) => {
                if PROFILE {
                    profile::invoke_virtual();
                }
                let Some(x) = x.try_as_method_ref() else {
                    return Err(Self::invalid_cp(jvm, "Methodref").await);
                };
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
                let cached = match cache.entries.get(instruction_index) {
                    Some(InstructionCache::InstanceMethod(lock)) => {
                        let guard = lock.read();
                        guard.as_ref().and_then(|site| {
                            if site.receiver_class == receiver_class {
                                Some(site.method.clone())
                            } else {
                                None
                            }
                        })
                    }
                    _ => None,
                };

                let method = match cached {
                    Some(m) => m,
                    None => {
                        let m = jvm.resolve_virtual_method_for_instance(&instance, &x.name, &x.descriptor).await?;
                        if let Some(InstructionCache::InstanceMethod(lock)) = cache.entries.get(instruction_index) {
                            *lock.write() = Some(ResolvedInstanceMethodSite {
                                receiver_class,
                                method: m.clone(),
                            });
                        }
                        m
                    }
                };

                let result = jvm.invoke_resolved_instance(&method, &instance, params.into_boxed_slice()).await?;
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
                let object: Option<Box<dyn ClassInstance>> = stack_value.into();
                let Some(object) = object else {
                    return Err(jvm.exception("java/lang/NullPointerException", "monitorenter").await);
                };
                jvm.monitor_enter(&object).await?;
            }
            Opcode::Monitorexit => {
                let stack_value = stack_frame.operand_stack.pop().unwrap();
                let object: Option<Box<dyn ClassInstance>> = stack_value.into();
                let Some(object) = object else {
                    return Err(jvm.exception("java/lang/NullPointerException", "monitorexit").await);
                };
                jvm.monitor_exit(&object).await?;
            }
            Opcode::Multianewarray(x, d) => {
                let mut dimensions: Vec<i32> = (0..*d).map(|_| stack_frame.operand_stack.pop().unwrap().into()).collect();
                dimensions.reverse();
                for dim in &dimensions {
                    if *dim < 0 {
                        return Err(jvm.exception("java/lang/NegativeArraySizeException", &format!("{dim}")).await);
                    }
                }

                let Some(class_name) = x.try_as_class() else {
                    return Err(Self::invalid_cp(jvm, "class").await);
                };
                let array = Self::new_multi_array(jvm, class_name, &dimensions).await?;

                stack_frame.operand_stack.push(JavaValue::Object(Some(array)));
            }
            Opcode::New(x) => {
                let Some(class_name) = x.try_as_class() else {
                    return Err(Self::invalid_cp(jvm, "class").await);
                };
                let class = jvm.instantiate_class(class_name).await?;

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
                    _ => return Err(jvm.exception("java/lang/LinkageError", &format!("invalid newarray type {x}")).await),
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
                let Some(x) = x.try_as_field_ref() else {
                    return Err(Self::invalid_cp(jvm, "Fieldref").await);
                };
                let value = stack_frame.operand_stack.pop().unwrap();
                let mut instance: Option<Box<dyn ClassInstance>> = stack_frame.operand_stack.pop().unwrap().into();

                if instance.is_none() {
                    return Err(jvm.exception("java/lang/NullPointerException", "null").await);
                }

                let value = Self::to_field_type(&x.descriptor, value);

                let cached = match cache.entries.get(instruction_index) {
                    Some(InstructionCache::InstanceField(lock)) => lock.read().as_ref().cloned(),
                    _ => None,
                };
                let field = match cached {
                    Some(field) => field,
                    None => {
                        let field = jvm.resolve_instance_field(&x.class, &x.name, &x.descriptor).await?;
                        if let Some(InstructionCache::InstanceField(lock)) = cache.entries.get(instruction_index) {
                            *lock.write() = Some(field.clone());
                        }
                        field
                    }
                };

                jvm.put_resolved_instance_field(&field, instance.as_mut().unwrap(), value)?;
            }
            Opcode::Putstatic(x) => {
                let Some(x) = x.try_as_field_ref() else {
                    return Err(Self::invalid_cp(jvm, "Fieldref").await);
                };
                let value = Self::to_field_type(&x.descriptor, stack_frame.operand_stack.pop().unwrap());

                let cached = match cache.entries.get(instruction_index) {
                    Some(InstructionCache::StaticField(lock)) => lock.read().as_ref().cloned(),
                    _ => None,
                };
                let field = match cached {
                    Some(field) => field,
                    None => {
                        let field = jvm.resolve_static_field(&x.class, &x.name, &x.descriptor).await?;
                        if let Some(InstructionCache::StaticField(lock)) = cache.entries.get(instruction_index) {
                            *lock.write() = Some(field.clone());
                        }
                        field
                    }
                };
                jvm.put_resolved_static_field(&field, value)?
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
                return Err(jvm.exception("java/lang/LinkageError", "wide not supported").await);
            }
            Opcode::Unknown(opcode) => {
                return Err(jvm.exception("java/lang/LinkageError", &format!("unknown opcode {opcode}")).await);
            }
        }

        Ok(ExecuteNext::Continue)
    }
}
