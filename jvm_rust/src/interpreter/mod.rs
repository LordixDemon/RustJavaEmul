use alloc::{boxed::Box, format, string::String, sync::Arc, vec::Vec};

use classfile::{AttributeInfoCode, Opcode};
use jvm::{JavaError, JavaType, JavaValue, Jvm, ResolvedInstanceField, ResolvedMethod, ResolvedStaticField, Result};
use parking_lot::RwLock;

use crate::{profile, stack_frame::StackFrame};

pub(super) enum ExecuteNext {
    Continue,
    Jump(u32),
    Return(JavaValue),
}

pub struct ResolvedInstanceMethodSite {
    pub receiver_class: String,
    pub method: ResolvedMethod,
}

pub enum InstructionCache {
    None,
    Jump(usize),
    StaticField(RwLock<Option<ResolvedStaticField>>),
    InstanceField(RwLock<Option<ResolvedInstanceField>>),
    StaticMethod(RwLock<Option<ResolvedMethod>>),
    SpecialMethod(RwLock<Option<ResolvedMethod>>),
    InstanceMethod(RwLock<Option<ResolvedInstanceMethodSite>>),
}

pub struct MethodBytecodeCache {
    pub entries: Box<[InstructionCache]>,
}

impl core::fmt::Debug for MethodBytecodeCache {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MethodBytecodeCache({} entries)", self.entries.len())
    }
}

impl MethodBytecodeCache {
    pub fn from_code(code: &AttributeInfoCode) -> Self {
        let mut entries = Vec::with_capacity(code.code_sequence.len());
        for (offset, opcode) in &code.code_sequence {
            let offset = *offset;
            let entry = match opcode {
                Opcode::Goto(x)
                | Opcode::IfAcmpeq(x)
                | Opcode::IfAcmpne(x)
                | Opcode::IfIcmpeq(x)
                | Opcode::IfIcmpge(x)
                | Opcode::IfIcmpgt(x)
                | Opcode::IfIcmple(x)
                | Opcode::IfIcmplt(x)
                | Opcode::IfIcmpne(x)
                | Opcode::Ifeq(x)
                | Opcode::Ifge(x)
                | Opcode::Ifgt(x)
                | Opcode::Ifle(x)
                | Opcode::Iflt(x)
                | Opcode::Ifne(x)
                | Opcode::Ifnonnull(x)
                | Opcode::Ifnull(x)
                | Opcode::Jsr(x) => {
                    let target_byte_offset = (offset as i32 + *x as i32) as u32;
                    match code.code_offsets.binary_search(&target_byte_offset) {
                        Ok(target_idx) => InstructionCache::Jump(target_idx),
                        Err(_) => InstructionCache::None,
                    }
                }
                Opcode::GotoW(x) | Opcode::JsrW(x) => {
                    let target_byte_offset = (offset as i32 + *x) as u32;
                    match code.code_offsets.binary_search(&target_byte_offset) {
                        Ok(target_idx) => InstructionCache::Jump(target_idx),
                        Err(_) => InstructionCache::None,
                    }
                }
                Opcode::Getstatic(_) | Opcode::Putstatic(_) => InstructionCache::StaticField(RwLock::new(None)),
                Opcode::Getfield(_) | Opcode::Putfield(_) => InstructionCache::InstanceField(RwLock::new(None)),
                Opcode::Invokestatic(_) => InstructionCache::StaticMethod(RwLock::new(None)),
                Opcode::Invokespecial(_) => InstructionCache::SpecialMethod(RwLock::new(None)),
                Opcode::Invokevirtual(_) | Opcode::Invokeinterface(_, _, _) => InstructionCache::InstanceMethod(RwLock::new(None)),
                _ => InstructionCache::None,
            };
            entries.push(entry);
        }
        Self {
            entries: entries.into_boxed_slice(),
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub async fn run(
        jvm: &Jvm,
        code_attribute: &AttributeInfoCode,
        cache: &MethodBytecodeCache,
        args: Box<[JavaValue]>,
        return_type: &JavaType,
        frame_name: Arc<str>,
    ) -> Result<JavaValue> {
        if profile::enabled() {
            Self::run_inner::<true>(jvm, code_attribute, cache, args, return_type, frame_name).await
        } else {
            Self::run_inner::<false>(jvm, code_attribute, cache, args, return_type, frame_name).await
        }
    }

    async fn run_inner<const PROFILE: bool>(
        jvm: &Jvm,
        code_attribute: &AttributeInfoCode,
        cache: &MethodBytecodeCache,
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

        let mut instruction_index = 0;
        let mut method_opcodes = 0u64;
        while instruction_index < code_attribute.code_sequence.len() {
            let (offset, opcode) = &code_attribute.code_sequence[instruction_index];
            let offset = *offset;
            method_opcodes += 1;
            if PROFILE {
                profile::opcode();
            }
            if Self::opcode_may_trigger_gc(opcode) {
                Self::sync_gc_roots(jvm, &stack_frame);
            }

            let result = if let Some(result) =
                Self::execute_fast_opcode::<PROFILE>(jvm, instruction_index, offset, opcode, &mut stack_frame, return_type, cache)
            {
                if PROFILE {
                    profile::fast_opcode();
                }
                Ok(result)
            } else {
                if PROFILE {
                    profile::slow_opcode();
                }
                tracing::trace!("Opcode {:?}", opcode);
                Self::execute_opcode::<PROFILE>(jvm, instruction_index, offset, opcode, &mut stack_frame, return_type, cache).await
            };
            match result {
                Ok(ExecuteNext::Continue) => {
                    instruction_index += 1;
                }
                Ok(ExecuteNext::Jump(offset)) => {
                    if PROFILE {
                        profile::jump();
                    }
                    if let Some(InstructionCache::Jump(target_index)) = cache.entries.get(instruction_index) {
                        instruction_index = *target_index;
                    } else {
                        instruction_index = Self::jump_to_instruction(jvm, code_attribute, offset).await?;
                    }
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

    fn opcode_may_trigger_gc(opcode: &Opcode) -> bool {
        matches!(
            opcode,
            Opcode::Invokevirtual(_)
                | Opcode::Invokespecial(_)
                | Opcode::Invokestatic(_)
                | Opcode::Invokeinterface(_, _, _)
                | Opcode::Invokedynamic(_)
        )
    }

    fn sync_gc_roots(jvm: &Jvm, stack_frame: &StackFrame) {
        let object_count = stack_frame
            .local_variables
            .iter()
            .chain(&stack_frame.operand_stack)
            .filter(|value| matches!(value, JavaValue::Object(Some(_))))
            .count();

        let mut roots = Vec::with_capacity(object_count);
        for value in stack_frame.local_variables.iter().chain(&stack_frame.operand_stack) {
            if let JavaValue::Object(Some(obj)) = value {
                roots.push(obj.clone());
            }
        }
        jvm.set_current_frame_extra_roots(roots);
    }

    fn array_element_class_name(array_name: &str) -> Option<&str> {
        let rest = array_name.strip_prefix('[')?;
        if let Some(inner) = rest.strip_prefix('L').and_then(|s| s.strip_suffix(';')) {
            Some(inner)
        } else if rest.starts_with('[') {
            Some(rest)
        } else {
            None
        }
    }

    async fn jump_to_instruction(jvm: &Jvm, code_attribute: &AttributeInfoCode, offset: u32) -> Result<usize> {
        match code_attribute.code_offsets.binary_search(&offset) {
            Ok(index) => Ok(index),
            Err(_) => Err(jvm
                .exception("java/lang/IllegalArgumentException", &format!("Invalid bytecode jump target {offset}"))
                .await),
        }
    }
}

mod fast;
mod helpers;
mod slow;
