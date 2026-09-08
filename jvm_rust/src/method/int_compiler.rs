use alloc::{boxed::Box, vec, vec::Vec};

use classfile::{AttributeInfoCode, Opcode};
use jvm::{JavaType, JavaValue, Jvm, JvmCallback, Result};

use crate::profile;

#[derive(Clone, Copy)]
enum IntOp {
    Nop,
    Pop,
    Dup,
    Swap,
    Iconst(i32),
    Iload(u16),
    Istore(u16),
    Iinc(u16, i16),
    Iadd,
    Isub,
    Imul,
    Idiv,
    Irem,
    Ineg,
    Iand,
    Ior,
    Ixor,
    Ishl,
    Ishr,
    Iushr,
    I2b,
    I2c,
    I2s,
    IfEq(u32),
    IfNe(u32),
    IfLt(u32),
    IfGe(u32),
    IfGt(u32),
    IfLe(u32),
    IfIcmpEq(u32),
    IfIcmpNe(u32),
    IfIcmpLt(u32),
    IfIcmpGe(u32),
    IfIcmpGt(u32),
    IfIcmpLe(u32),
    Goto(u32),
    Ireturn,
    Return,
}

pub(super) fn maybe_compile(descriptor: &str, code: &AttributeInfoCode) -> Option<Box<dyn JvmCallback>> {
    if !code.exception_table.is_empty()
        || code.max_locals as usize > 256
        || code.max_stack as usize > 64
        || code.code_sequence.is_empty()
        || code.code_sequence.len() > 2048
    {
        return None;
    }
    if is_int_method(descriptor)
        && let Some(compiled) = try_compile_int(descriptor, code)
    {
        return Some(compiled);
    }
    super::general::maybe_compile(descriptor, code)
}

fn try_compile_int(descriptor: &str, code: &AttributeInfoCode) -> Option<Box<dyn JvmCallback>> {
    let mut ops = Vec::with_capacity(code.code_sequence.len());
    for (offset, opcode) in &code.code_sequence {
        ops.push(compile_opcode(code, *offset, opcode)?);
    }
    let return_type = JavaType::parse(descriptor).as_method().1.clone();
    let max_locals = code.max_locals as usize;
    if let Some(compiled) = super::loop_compiler::try_int(&int_toks(&ops), max_locals, return_type.clone()) {
        return Some(compiled);
    }

    Some(Box::new(CompiledIntMethod {
        ops: ops.into_boxed_slice(),
        max_locals,
        max_stack: code.max_stack as usize,
        return_type,
    }))
}

fn int_toks(ops: &[IntOp]) -> Vec<super::loop_compiler::IntTok> {
    ops.iter()
        .map(|op| match *op {
            IntOp::Iload(index) => super::loop_compiler::IntTok::Load(index),
            IntOp::Iconst(value) => super::loop_compiler::IntTok::Iconst(value),
            IntOp::Istore(index) => super::loop_compiler::IntTok::Store(index),
            IntOp::Iinc(index, amount) => super::loop_compiler::IntTok::Iinc(index, amount),
            IntOp::Iadd => super::loop_compiler::IntTok::Iadd,
            IntOp::Isub => super::loop_compiler::IntTok::Isub,
            IntOp::Imul => super::loop_compiler::IntTok::Imul,
            IntOp::Idiv => super::loop_compiler::IntTok::Idiv,
            IntOp::Irem => super::loop_compiler::IntTok::Irem,
            IntOp::Ineg => super::loop_compiler::IntTok::Ineg,
            IntOp::Iand => super::loop_compiler::IntTok::Iand,
            IntOp::Ior => super::loop_compiler::IntTok::Ior,
            IntOp::Ixor => super::loop_compiler::IntTok::Ixor,
            IntOp::Ishl => super::loop_compiler::IntTok::Ishl,
            IntOp::Ishr => super::loop_compiler::IntTok::Ishr,
            IntOp::Iushr => super::loop_compiler::IntTok::Iushr,
            IntOp::I2b => super::loop_compiler::IntTok::I2b,
            IntOp::I2c => super::loop_compiler::IntTok::I2c,
            IntOp::I2s => super::loop_compiler::IntTok::I2s,
            IntOp::IfIcmpGe(target) => super::loop_compiler::IntTok::IfIcmpGe(target),
            IntOp::Goto(target) => super::loop_compiler::IntTok::Goto(target),
            IntOp::Ireturn => super::loop_compiler::IntTok::Ireturn,
            IntOp::Return => super::loop_compiler::IntTok::Return,
            IntOp::IfEq(_)
            | IntOp::IfNe(_)
            | IntOp::IfLt(_)
            | IntOp::IfGe(_)
            | IntOp::IfGt(_)
            | IntOp::IfLe(_)
            | IntOp::IfIcmpEq(_)
            | IntOp::IfIcmpNe(_)
            | IntOp::IfIcmpLt(_)
            | IntOp::IfIcmpGt(_)
            | IntOp::IfIcmpLe(_) => super::loop_compiler::IntTok::Jump,
            _ => super::loop_compiler::IntTok::Other,
        })
        .collect()
}

fn is_int_method(descriptor: &str) -> bool {
    let Some(params_and_ret) = descriptor.strip_prefix('(') else {
        return false;
    };
    let Some(ret_index) = params_and_ret.find(')') else {
        return false;
    };
    let params = &params_and_ret[..ret_index];
    let ret = &params_and_ret[ret_index + 1..];
    if !matches!(ret, "I" | "Z" | "B" | "C" | "S" | "V") {
        return false;
    }
    params.bytes().all(|byte| matches!(byte, b'I' | b'Z' | b'B' | b'C' | b'S'))
}

fn compile_opcode(code: &AttributeInfoCode, offset: u32, opcode: &Opcode) -> Option<IntOp> {
    Some(match opcode {
        Opcode::Nop => IntOp::Nop,
        Opcode::Pop => IntOp::Pop,
        Opcode::Dup => IntOp::Dup,
        Opcode::Swap => IntOp::Swap,
        Opcode::Iconst(value) => IntOp::Iconst(*value as i32),
        Opcode::Bipush(value) => IntOp::Iconst(*value as i32),
        Opcode::Sipush(value) => IntOp::Iconst(*value as i32),
        Opcode::Ldc(classfile::ConstantPoolReference::Integer(value))
        | Opcode::LdcW(classfile::ConstantPoolReference::Integer(value)) => IntOp::Iconst(*value),
        Opcode::Iload(index) => IntOp::Iload(*index),
        Opcode::Istore(index) => IntOp::Istore(*index),
        Opcode::Iinc(index, amount) => IntOp::Iinc(*index, *amount),
        Opcode::Iadd => IntOp::Iadd,
        Opcode::Isub => IntOp::Isub,
        Opcode::Imul => IntOp::Imul,
        Opcode::Idiv => IntOp::Idiv,
        Opcode::Irem => IntOp::Irem,
        Opcode::Ineg => IntOp::Ineg,
        Opcode::Iand => IntOp::Iand,
        Opcode::Ior => IntOp::Ior,
        Opcode::Ixor => IntOp::Ixor,
        Opcode::Ishl => IntOp::Ishl,
        Opcode::Ishr => IntOp::Ishr,
        Opcode::Iushr => IntOp::Iushr,
        Opcode::I2b => IntOp::I2b,
        Opcode::I2c => IntOp::I2c,
        Opcode::I2s => IntOp::I2s,
        Opcode::Ifeq(rel) => IntOp::IfEq(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifne(rel) => IntOp::IfNe(jump_index(code, offset, *rel as i32)?),
        Opcode::Iflt(rel) => IntOp::IfLt(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifge(rel) => IntOp::IfGe(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifgt(rel) => IntOp::IfGt(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifle(rel) => IntOp::IfLe(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmpeq(rel) => IntOp::IfIcmpEq(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmpne(rel) => IntOp::IfIcmpNe(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmplt(rel) => IntOp::IfIcmpLt(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmpge(rel) => IntOp::IfIcmpGe(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmpgt(rel) => IntOp::IfIcmpGt(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmple(rel) => IntOp::IfIcmpLe(jump_index(code, offset, *rel as i32)?),
        Opcode::Goto(rel) => IntOp::Goto(jump_index(code, offset, *rel as i32)?),
        Opcode::GotoW(rel) => IntOp::Goto(jump_index(code, offset, *rel)?),
        Opcode::Ireturn => IntOp::Ireturn,
        Opcode::Return => IntOp::Return,
        _ => return None,
    })
}

fn jump_index(code: &AttributeInfoCode, offset: u32, rel: i32) -> Option<u32> {
    let target = offset.wrapping_add(rel as u32);
    code.code_offsets.binary_search(&target).ok().map(|index| index as u32)
}

struct CompiledIntMethod {
    ops: Box<[IntOp]>,
    max_locals: usize,
    max_stack: usize,
    return_type: JavaType,
}

enum ExecError {
    DivByZero,
}

#[async_trait::async_trait]
impl JvmCallback for CompiledIntMethod {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::int_compiler_call();
        match self.execute(&args) {
            Ok(value) => Ok(value),
            Err(ExecError::DivByZero) => Err(jvm.exception("java/lang/ArithmeticException", "/ by zero").await),
        }
    }

    fn call_sync(&self, _jvm: &Jvm, args: &[JavaValue]) -> Option<Result<JavaValue>> {
        profile::int_compiler_call();
        match self.execute(args) {
            Ok(value) => Some(Ok(value)),
            Err(ExecError::DivByZero) => None,
        }
    }
}

impl CompiledIntMethod {
    fn execute(&self, args: &[JavaValue]) -> core::result::Result<JavaValue, ExecError> {
        let mut locals = vec![0i32; self.max_locals];
        let mut local_index = 0usize;
        for arg in args {
            if local_index >= locals.len() {
                break;
            }
            match arg {
                JavaValue::Boolean(value) => {
                    locals[local_index] = i32::from(*value);
                    local_index += 1;
                }
                JavaValue::Byte(value) => {
                    locals[local_index] = i32::from(*value);
                    local_index += 1;
                }
                JavaValue::Char(value) => {
                    locals[local_index] = i32::from(*value);
                    local_index += 1;
                }
                JavaValue::Short(value) => {
                    locals[local_index] = i32::from(*value);
                    local_index += 1;
                }
                JavaValue::Int(value) => {
                    locals[local_index] = *value;
                    local_index += 1;
                }
                JavaValue::Object(_) => local_index += 1,
                JavaValue::Long(_) | JavaValue::Float(_) | JavaValue::Double(_) | JavaValue::Void => local_index += 1,
            }
        }

        let mut stack = Vec::with_capacity(self.max_stack);
        let mut pc = 0usize;
        while pc < self.ops.len() {
            match self.ops[pc] {
                IntOp::Nop => pc += 1,
                IntOp::Pop => {
                    stack.pop();
                    pc += 1;
                }
                IntOp::Dup => {
                    if let Some(&top) = stack.last() {
                        stack.push(top);
                    }
                    pc += 1;
                }
                IntOp::Swap => {
                    let len = stack.len();
                    if len >= 2 {
                        stack.swap(len - 1, len - 2);
                    }
                    pc += 1;
                }
                IntOp::Iconst(value) => {
                    stack.push(value);
                    pc += 1;
                }
                IntOp::Iload(index) => {
                    stack.push(locals[index as usize]);
                    pc += 1;
                }
                IntOp::Istore(index) => {
                    locals[index as usize] = stack.pop().unwrap_or(0);
                    pc += 1;
                }
                IntOp::Iinc(index, amount) => {
                    locals[index as usize] = locals[index as usize].wrapping_add(i32::from(amount));
                    pc += 1;
                }
                IntOp::Iadd => binop(&mut stack, i32::wrapping_add, &mut pc),
                IntOp::Isub => binop(&mut stack, i32::wrapping_sub, &mut pc),
                IntOp::Imul => binop(&mut stack, i32::wrapping_mul, &mut pc),
                IntOp::Idiv => {
                    let b = stack.pop().unwrap_or(0);
                    let a = stack.pop().unwrap_or(0);
                    stack.push(java_idiv(a, b)?);
                    pc += 1;
                }
                IntOp::Irem => {
                    let b = stack.pop().unwrap_or(0);
                    let a = stack.pop().unwrap_or(0);
                    stack.push(java_irem(a, b)?);
                    pc += 1;
                }
                IntOp::Ineg => {
                    if let Some(top) = stack.last_mut() {
                        *top = top.wrapping_neg();
                    }
                    pc += 1;
                }
                IntOp::Iand => binop(&mut stack, core::ops::BitAnd::bitand, &mut pc),
                IntOp::Ior => binop(&mut stack, core::ops::BitOr::bitor, &mut pc),
                IntOp::Ixor => binop(&mut stack, core::ops::BitXor::bitxor, &mut pc),
                IntOp::Ishl => binop(&mut stack, |a, b| a.wrapping_shl(b as u32), &mut pc),
                IntOp::Ishr => binop(&mut stack, |a, b| a.wrapping_shr(b as u32), &mut pc),
                IntOp::Iushr => binop(&mut stack, |a, b| (a as u32).wrapping_shr(b as u32) as i32, &mut pc),
                IntOp::I2b => {
                    truncate(&mut stack, |value| value as i8 as i32);
                    pc += 1;
                }
                IntOp::I2c => {
                    truncate(&mut stack, |value| value as u16 as i32);
                    pc += 1;
                }
                IntOp::I2s => {
                    truncate(&mut stack, |value| value as i16 as i32);
                    pc += 1;
                }
                IntOp::IfEq(target) => branch1(&mut stack, &mut pc, target, |value| value == 0),
                IntOp::IfNe(target) => branch1(&mut stack, &mut pc, target, |value| value != 0),
                IntOp::IfLt(target) => branch1(&mut stack, &mut pc, target, |value| value < 0),
                IntOp::IfGe(target) => branch1(&mut stack, &mut pc, target, |value| value >= 0),
                IntOp::IfGt(target) => branch1(&mut stack, &mut pc, target, |value| value > 0),
                IntOp::IfLe(target) => branch1(&mut stack, &mut pc, target, |value| value <= 0),
                IntOp::IfIcmpEq(target) => branch2(&mut stack, &mut pc, target, |a, b| a == b),
                IntOp::IfIcmpNe(target) => branch2(&mut stack, &mut pc, target, |a, b| a != b),
                IntOp::IfIcmpLt(target) => branch2(&mut stack, &mut pc, target, |a, b| a < b),
                IntOp::IfIcmpGe(target) => branch2(&mut stack, &mut pc, target, |a, b| a >= b),
                IntOp::IfIcmpGt(target) => branch2(&mut stack, &mut pc, target, |a, b| a > b),
                IntOp::IfIcmpLe(target) => branch2(&mut stack, &mut pc, target, |a, b| a <= b),
                IntOp::Goto(target) => pc = target as usize,
                IntOp::Ireturn => {
                    let value = stack.pop().unwrap_or(0);
                    return Ok(match self.return_type {
                        JavaType::Boolean => JavaValue::Boolean(value & 1 != 0),
                        JavaType::Char => JavaValue::Char(value as u16),
                        JavaType::Byte => JavaValue::Byte(value as i8),
                        JavaType::Short => JavaValue::Short(value as i16),
                        _ => JavaValue::Int(value),
                    });
                }
                IntOp::Return => return Ok(JavaValue::Void),
            }
        }

        Ok(JavaValue::Void)
    }
}

fn binop(stack: &mut Vec<i32>, op: impl Fn(i32, i32) -> i32, pc: &mut usize) {
    let b = stack.pop().unwrap_or(0);
    let a = stack.pop().unwrap_or(0);
    stack.push(op(a, b));
    *pc += 1;
}

fn truncate(stack: &mut [i32], op: impl Fn(i32) -> i32) {
    if let Some(top) = stack.last_mut() {
        *top = op(*top);
    }
}

fn branch1(stack: &mut Vec<i32>, pc: &mut usize, target: u32, pred: impl Fn(i32) -> bool) {
    let value = stack.pop().unwrap_or(0);
    if pred(value) {
        *pc = target as usize;
    } else {
        *pc += 1;
    }
}

fn branch2(stack: &mut Vec<i32>, pc: &mut usize, target: u32, pred: impl Fn(i32, i32) -> bool) {
    let b = stack.pop().unwrap_or(0);
    let a = stack.pop().unwrap_or(0);
    if pred(a, b) {
        *pc = target as usize;
    } else {
        *pc += 1;
    }
}

fn java_idiv(a: i32, b: i32) -> core::result::Result<i32, ExecError> {
    if b == 0 {
        return Err(ExecError::DivByZero);
    }
    if a == i32::MIN && b == -1 {
        return Ok(a);
    }
    Ok(a / b)
}

fn java_irem(a: i32, b: i32) -> core::result::Result<i32, ExecError> {
    if b == 0 {
        return Err(ExecError::DivByZero);
    }
    if a == i32::MIN && b == -1 {
        return Ok(0);
    }
    Ok(a % b)
}
