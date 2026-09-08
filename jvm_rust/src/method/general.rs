use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};

use classfile::{AttributeInfoCode, ConstantPoolReference, Opcode};
use jvm::{ClassInstance, JavaError, JavaType, JavaValue, Jvm, JvmCallback, ResolvedInstanceField, ResolvedMethod, ResolvedStaticField, Result};
use parking_lot::RwLock;

use crate::{class_instance::ClassInstanceImpl, profile};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    I,
    J,
    F,
    D,
    A,
}

#[derive(Clone, Copy)]
pub(super) enum Op {
    Nop,
    Pop,
    Pop2,
    Dup,
    Dup2,
    Swap,
    Aconst,
    Iconst(i32),
    Lconst(i64),
    Dconst(f64),
    Load(u16),
    Store(u16),
    StoreWide(u16),
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
    I2l,
    L2i,
    I2d,
    D2i,
    Ladd,
    Lsub,
    Lmul,
    Ldiv,
    Lrem,
    Lneg,
    Land,
    Lor,
    Lxor,
    Lshl,
    Lshr,
    Lushr,
    Lcmp,
    Dadd,
    Dsub,
    Dmul,
    Ddiv,
    Dcmpl,
    Dcmpg,
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
    IfNull(u32),
    IfNonNull(u32),
    Goto(u32),
    GetField(u16),
    GetThisField(u16),
    PutField(u16),
    GetStatic(u16),
    InvokeVirtual(u16),
    InvokeStatic(u16),
    InvokeInterface(u16),
    InvokeSpecial(u16),
    GfxSetColor(u16),
    GfxFillRect(u16),
    GfxDrawLine(u16),
    GfxFillTriangle(u16),
    GfxDrawRgb(u16),
    GfxDrawImage(u16),
    GfxDrawRegion(u16),
    GfxSetClip(u16),
    GfxClipRect(u16),
    GfxTranslate(u16),
    CurrentTimeMillis(u16),
    Ireturn,
    Lreturn,
    Dreturn,
    Areturn,
    Return,
}

impl Op {
    fn is_leaf(self) -> bool {
        !matches!(
            self,
            Op::GetStatic(_) | Op::InvokeVirtual(_) | Op::InvokeStatic(_) | Op::InvokeInterface(_) | Op::InvokeSpecial(_)
        )
    }

    fn remap_jump(&mut self, removed: usize) {
        let target = match self {
            Op::IfEq(target)
            | Op::IfNe(target)
            | Op::IfLt(target)
            | Op::IfGe(target)
            | Op::IfGt(target)
            | Op::IfLe(target)
            | Op::IfIcmpEq(target)
            | Op::IfIcmpNe(target)
            | Op::IfIcmpLt(target)
            | Op::IfIcmpGe(target)
            | Op::IfIcmpGt(target)
            | Op::IfIcmpLe(target)
            | Op::IfNull(target)
            | Op::IfNonNull(target)
            | Op::Goto(target) => target,
            _ => return,
        };
        if *target as usize >= removed {
            *target -= 1;
        }
    }
}

fn peephole_get_this_field(ops: &mut Vec<Op>) {
    let mut index = 0;
    while index + 1 < ops.len() {
        if let (Op::Load(0), Op::GetField(field)) = (ops[index], ops[index + 1]) {
            ops[index] = Op::GetThisField(field);
            ops.remove(index + 1);
            for op in ops.iter_mut() {
                op.remap_jump(index + 1);
            }
            continue;
        }
        index += 1;
    }
}

pub(super) struct FieldSite {
    pub(super) class: Arc<str>,
    pub(super) name: Arc<str>,
    pub(super) descriptor: Arc<str>,
    pub(super) resolved: RwLock<Option<ResolvedInstanceField>>,
}

struct StaticFieldSite {
    class: Arc<str>,
    name: Arc<str>,
    descriptor: Arc<str>,
    resolved: RwLock<Option<ResolvedStaticField>>,
}

struct InvokeSite {
    class: Arc<str>,
    name: Arc<str>,
    descriptor: Arc<str>,
    argc: u8,
    kinds: Box<[u8]>,
    cache: RwLock<Option<(String, ResolvedMethod)>>,
}

pub(super) fn maybe_compile(descriptor: &str, code: &AttributeInfoCode) -> Option<Box<dyn JvmCallback>> {
    let stacks = analyze(code)?;
    let mut fields = Vec::new();
    let mut static_fields = Vec::new();
    let mut invokes = Vec::new();
    let mut ops = Vec::with_capacity(code.code_sequence.len());
    for (index, (offset, opcode)) in code.code_sequence.iter().enumerate() {
        let stack = stacks.get(index)?;
        ops.push(compile_opcode(
            code,
            *offset,
            opcode,
            stack,
            &mut fields,
            &mut static_fields,
            &mut invokes,
        )?);
    }

    peephole_get_this_field(&mut ops);
    if let Some(compiled) = super::loop_compiler::try_gfx(&ops, &fields, code.max_locals as usize, JavaType::parse(descriptor).as_method().1.clone())
    {
        return Some(compiled);
    }
    let ops = ops.into_boxed_slice();
    let sync = ops.iter().copied().all(Op::is_leaf);
    Some(Box::new(CompiledMethod {
        ops,
        fields: fields.into_boxed_slice(),
        static_fields: static_fields.into_boxed_slice(),
        invokes: invokes.into_boxed_slice(),
        max_locals: code.max_locals as usize,
        max_stack: code.max_stack as usize,
        return_type: JavaType::parse(descriptor).as_method().1.clone(),
        sync,
    }))
}

fn analyze(code: &AttributeInfoCode) -> Option<Vec<Vec<Kind>>> {
    let len = code.code_sequence.len();
    let mut incoming: Vec<Option<Vec<Kind>>> = vec![None; len];
    incoming[0] = Some(Vec::new());
    let mut changed = true;
    let mut guard = 0u32;
    while changed {
        changed = false;
        guard += 1;
        if guard > 4096 {
            return None;
        }
        for index in 0..len {
            let Some(stack) = incoming[index].clone() else {
                continue;
            };
            let (offset, opcode) = &code.code_sequence[index];
            let (out, branch, fall_through) = apply_kind(code, *offset, opcode, stack)?;
            if fall_through && index + 1 < len {
                changed |= merge_stack(&mut incoming[index + 1], &out)?;
            }
            if let Some(target) = branch {
                let target = target as usize;
                if target >= len {
                    return None;
                }
                changed |= merge_stack(&mut incoming[target], &out)?;
            }
        }
    }
    let mut stacks = Vec::with_capacity(len);
    for slot in incoming {
        stacks.push(slot.unwrap_or_default());
    }
    Some(stacks)
}

fn merge_stack(slot: &mut Option<Vec<Kind>>, incoming: &[Kind]) -> Option<bool> {
    match slot {
        None => {
            *slot = Some(incoming.to_vec());
            Some(true)
        }
        Some(existing) if existing.as_slice() == incoming => Some(false),
        Some(_) => None,
    }
}

fn apply_kind(code: &AttributeInfoCode, offset: u32, opcode: &Opcode, mut stack: Vec<Kind>) -> Option<(Vec<Kind>, Option<u32>, bool)> {
    let mut branch = None;
    let mut fall_through = true;
    match opcode {
        Opcode::Nop | Opcode::Iinc(_, _) => {}
        Opcode::Pop => {
            pop_kind(&mut stack, None)?;
        }
        Opcode::Pop2 => {
            let top = pop_kind(&mut stack, None)?;
            if !matches!(top, Kind::J | Kind::D) {
                pop_kind(&mut stack, None)?;
            }
        }
        Opcode::Dup => {
            let top = *stack.last()?;
            if matches!(top, Kind::J | Kind::D) {
                return None;
            }
            stack.push(top);
        }
        Opcode::Dup2 => {
            let top = *stack.last()?;
            if matches!(top, Kind::J | Kind::D) {
                stack.push(top);
            } else {
                let a = pop_kind(&mut stack, None)?;
                let b = pop_kind(&mut stack, None)?;
                stack.push(b);
                stack.push(a);
                stack.push(b);
                stack.push(a);
            }
        }
        Opcode::Swap => {
            let a = pop_kind(&mut stack, None)?;
            let b = pop_kind(&mut stack, None)?;
            if matches!(a, Kind::J | Kind::D) || matches!(b, Kind::J | Kind::D) {
                return None;
            }
            stack.push(a);
            stack.push(b);
        }
        Opcode::Iconst(_) | Opcode::Bipush(_) | Opcode::Sipush(_) => stack.push(Kind::I),
        Opcode::Lconst(_) => stack.push(Kind::J),
        Opcode::Dconst(_) => stack.push(Kind::D),
        Opcode::Fconst(_) => stack.push(Kind::F),
        Opcode::Ldc(constant) | Opcode::LdcW(constant) => stack.push(kind_from_constant(constant)?),
        Opcode::Ldc2W(constant) => stack.push(kind_from_constant(constant)?),
        Opcode::Iload(_) | Opcode::Istore(_) => {
            if matches!(opcode, Opcode::Iload(_)) {
                stack.push(Kind::I);
            } else {
                pop_kind(&mut stack, Some(Kind::I))?;
            }
        }
        Opcode::Lload(_) => stack.push(Kind::J),
        Opcode::Lstore(_) => {
            pop_kind(&mut stack, Some(Kind::J))?;
        }
        Opcode::Fload(_) => stack.push(Kind::F),
        Opcode::Fstore(_) => {
            pop_kind(&mut stack, Some(Kind::F))?;
        }
        Opcode::Dload(_) => stack.push(Kind::D),
        Opcode::Dstore(_) => {
            pop_kind(&mut stack, Some(Kind::D))?;
        }
        Opcode::Aload(_) => stack.push(Kind::A),
        Opcode::Astore(_) => {
            pop_kind(&mut stack, Some(Kind::A))?;
        }
        Opcode::Iadd | Opcode::Isub | Opcode::Imul | Opcode::Idiv | Opcode::Irem | Opcode::Iand | Opcode::Ior | Opcode::Ixor => {
            pop_kind(&mut stack, Some(Kind::I))?;
            pop_kind(&mut stack, Some(Kind::I))?;
            stack.push(Kind::I);
        }
        Opcode::Ishl | Opcode::Ishr | Opcode::Iushr => {
            pop_kind(&mut stack, Some(Kind::I))?;
            pop_kind(&mut stack, Some(Kind::I))?;
            stack.push(Kind::I);
        }
        Opcode::Ineg | Opcode::I2b | Opcode::I2c | Opcode::I2s => {
            pop_kind(&mut stack, Some(Kind::I))?;
            stack.push(Kind::I);
        }
        Opcode::I2l => {
            pop_kind(&mut stack, Some(Kind::I))?;
            stack.push(Kind::J);
        }
        Opcode::L2i => {
            pop_kind(&mut stack, Some(Kind::J))?;
            stack.push(Kind::I);
        }
        Opcode::I2d => {
            pop_kind(&mut stack, Some(Kind::I))?;
            stack.push(Kind::D);
        }
        Opcode::D2i => {
            pop_kind(&mut stack, Some(Kind::D))?;
            stack.push(Kind::I);
        }
        Opcode::Ladd | Opcode::Lsub | Opcode::Lmul | Opcode::Ldiv | Opcode::Lrem | Opcode::Land | Opcode::Lor | Opcode::Lxor => {
            pop_kind(&mut stack, Some(Kind::J))?;
            pop_kind(&mut stack, Some(Kind::J))?;
            stack.push(Kind::J);
        }
        Opcode::Lshl | Opcode::Lshr | Opcode::Lushr => {
            pop_kind(&mut stack, Some(Kind::I))?;
            pop_kind(&mut stack, Some(Kind::J))?;
            stack.push(Kind::J);
        }
        Opcode::Lneg => {
            pop_kind(&mut stack, Some(Kind::J))?;
            stack.push(Kind::J);
        }
        Opcode::Lcmp => {
            pop_kind(&mut stack, Some(Kind::J))?;
            pop_kind(&mut stack, Some(Kind::J))?;
            stack.push(Kind::I);
        }
        Opcode::Dadd | Opcode::Dsub | Opcode::Dmul | Opcode::Ddiv => {
            pop_kind(&mut stack, Some(Kind::D))?;
            pop_kind(&mut stack, Some(Kind::D))?;
            stack.push(Kind::D);
        }
        Opcode::Dcmpl | Opcode::Dcmpg => {
            pop_kind(&mut stack, Some(Kind::D))?;
            pop_kind(&mut stack, Some(Kind::D))?;
            stack.push(Kind::I);
        }
        Opcode::Ifeq(rel) | Opcode::Ifne(rel) | Opcode::Iflt(rel) | Opcode::Ifge(rel) | Opcode::Ifgt(rel) | Opcode::Ifle(rel) => {
            pop_kind(&mut stack, Some(Kind::I))?;
            branch = Some(jump_index(code, offset, *rel as i32)?);
        }
        Opcode::IfIcmpeq(rel)
        | Opcode::IfIcmpne(rel)
        | Opcode::IfIcmplt(rel)
        | Opcode::IfIcmpge(rel)
        | Opcode::IfIcmpgt(rel)
        | Opcode::IfIcmple(rel) => {
            pop_kind(&mut stack, Some(Kind::I))?;
            pop_kind(&mut stack, Some(Kind::I))?;
            branch = Some(jump_index(code, offset, *rel as i32)?);
        }
        Opcode::Ifnull(rel) | Opcode::Ifnonnull(rel) => {
            pop_kind(&mut stack, Some(Kind::A))?;
            branch = Some(jump_index(code, offset, *rel as i32)?);
        }
        Opcode::Goto(rel) => {
            branch = Some(jump_index(code, offset, *rel as i32)?);
            fall_through = false;
        }
        Opcode::GotoW(rel) => {
            branch = Some(jump_index(code, offset, *rel)?);
            fall_through = false;
        }
        Opcode::Getfield(field) => {
            pop_kind(&mut stack, Some(Kind::A))?;
            stack.push(kind_from_field(field)?);
        }
        Opcode::Putfield(field) => {
            pop_kind(&mut stack, Some(kind_from_field(field)?))?;
            pop_kind(&mut stack, Some(Kind::A))?;
        }
        Opcode::Getstatic(field) => stack.push(kind_from_field(field)?),
        Opcode::Invokevirtual(method) | Opcode::Invokespecial(method) => {
            apply_invoke(&mut stack, method, true)?;
        }
        Opcode::Invokestatic(method) => {
            apply_invoke(&mut stack, method, false)?;
        }
        Opcode::Invokeinterface(method, _, _) => {
            apply_invoke(&mut stack, method, true)?;
        }
        Opcode::Checkcast(_) => {
            pop_kind(&mut stack, Some(Kind::A))?;
            stack.push(Kind::A);
        }
        Opcode::AconstNull => stack.push(Kind::A),
        Opcode::Ireturn => {
            pop_kind(&mut stack, Some(Kind::I))?;
            fall_through = false;
        }
        Opcode::Lreturn => {
            pop_kind(&mut stack, Some(Kind::J))?;
            fall_through = false;
        }
        Opcode::Dreturn => {
            pop_kind(&mut stack, Some(Kind::D))?;
            fall_through = false;
        }
        Opcode::Areturn => {
            pop_kind(&mut stack, Some(Kind::A))?;
            fall_through = false;
        }
        Opcode::Return => {
            fall_through = false;
        }
        _ => return None,
    }
    Some((stack, branch, fall_through))
}

fn pop_kind(stack: &mut Vec<Kind>, expected: Option<Kind>) -> Option<Kind> {
    let kind = stack.pop()?;
    if let Some(expected) = expected
        && kind != expected
    {
        return None;
    }
    Some(kind)
}

fn kind_from_constant(constant: &ConstantPoolReference) -> Option<Kind> {
    Some(match constant {
        ConstantPoolReference::Integer(_) => Kind::I,
        ConstantPoolReference::Float(_) => Kind::F,
        ConstantPoolReference::Long(_) => Kind::J,
        ConstantPoolReference::Double(_) => Kind::D,
        ConstantPoolReference::String(_) => Kind::A,
        _ => return None,
    })
}

fn kind_from_field(field: &ConstantPoolReference) -> Option<Kind> {
    let field = field.try_as_field_ref()?;
    kind_from_descriptor(&field.descriptor)
}

fn kind_from_descriptor(descriptor: &str) -> Option<Kind> {
    Some(match descriptor.as_bytes().first()? {
        b'I' | b'Z' | b'B' | b'C' | b'S' => Kind::I,
        b'J' => Kind::J,
        b'F' => Kind::F,
        b'D' => Kind::D,
        b'L' | b'[' => Kind::A,
        _ => return None,
    })
}

fn apply_invoke(stack: &mut Vec<Kind>, method: &ConstantPoolReference, instance: bool) -> Option<()> {
    let method = method
        .try_as_method_ref()
        .or_else(|| method.try_as_interface_method_ref())?;
    let params = param_kinds(&method.descriptor)?;
    for kind in params.iter().rev() {
        pop_kind(stack, Some(*kind))?;
    }
    if instance {
        pop_kind(stack, Some(Kind::A))?;
    }
    if let Some(ret) = return_kind(&method.descriptor) {
        stack.push(ret);
    }
    Some(())
}

fn param_kinds(descriptor: &str) -> Option<Vec<Kind>> {
    let rest = descriptor.strip_prefix('(')?;
    let end = rest.find(')')?;
    let mut kinds = Vec::new();
    let bytes = &rest.as_bytes()[..end];
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'I' | b'Z' | b'B' | b'C' | b'S' => {
                kinds.push(Kind::I);
                index += 1;
            }
            b'J' => {
                kinds.push(Kind::J);
                index += 1;
            }
            b'F' => {
                kinds.push(Kind::F);
                index += 1;
            }
            b'D' => {
                kinds.push(Kind::D);
                index += 1;
            }
            b'L' => {
                kinds.push(Kind::A);
                index += 1;
                while index < bytes.len() && bytes[index] != b';' {
                    index += 1;
                }
                if index >= bytes.len() {
                    return None;
                }
                index += 1;
            }
            b'[' => {
                kinds.push(Kind::A);
                while index < bytes.len() && bytes[index] == b'[' {
                    index += 1;
                }
                if index < bytes.len() && bytes[index] == b'L' {
                    index += 1;
                    while index < bytes.len() && bytes[index] != b';' {
                        index += 1;
                    }
                }
                if index >= bytes.len() {
                    return None;
                }
                index += 1;
            }
            _ => return None,
        }
    }
    Some(kinds)
}

fn return_kind(descriptor: &str) -> Option<Kind> {
    let ret = descriptor.rsplit_once(')')?.1;
    if ret == "V" {
        return None;
    }
    kind_from_descriptor(ret)
}

fn jump_index(code: &AttributeInfoCode, offset: u32, rel: i32) -> Option<u32> {
    let target = offset.wrapping_add(rel as u32);
    code.code_offsets.binary_search(&target).ok().map(|index| index as u32)
}

fn compile_opcode(
    code: &AttributeInfoCode,
    offset: u32,
    opcode: &Opcode,
    stack: &[Kind],
    fields: &mut Vec<FieldSite>,
    static_fields: &mut Vec<StaticFieldSite>,
    invokes: &mut Vec<InvokeSite>,
) -> Option<Op> {
    Some(match opcode {
        Opcode::Nop | Opcode::Checkcast(_) => Op::Nop,
        Opcode::Pop => Op::Pop,
        Opcode::Pop2 => match stack.last() {
            Some(Kind::J | Kind::D) => Op::Pop,
            _ => Op::Pop2,
        },
        Opcode::Dup => Op::Dup,
        Opcode::Dup2 => match stack.last() {
            Some(Kind::J | Kind::D) => Op::Dup,
            _ => Op::Dup2,
        },
        Opcode::Swap => Op::Swap,
        Opcode::Iconst(value) => Op::Iconst(*value as i32),
        Opcode::Bipush(value) => Op::Iconst(*value as i32),
        Opcode::Sipush(value) => Op::Iconst(*value as i32),
        Opcode::Lconst(value) => Op::Lconst(i64::from(*value)),
        Opcode::Dconst(value) => Op::Dconst(if *value == 0 { 0.0 } else { 1.0 }),
        Opcode::Ldc(ConstantPoolReference::Integer(value)) | Opcode::LdcW(ConstantPoolReference::Integer(value)) => Op::Iconst(*value),
        Opcode::Ldc(ConstantPoolReference::Long(value)) | Opcode::LdcW(ConstantPoolReference::Long(value)) | Opcode::Ldc2W(ConstantPoolReference::Long(value)) => {
            Op::Lconst(*value)
        }
        Opcode::Ldc(ConstantPoolReference::Double(value)) | Opcode::LdcW(ConstantPoolReference::Double(value)) | Opcode::Ldc2W(ConstantPoolReference::Double(value)) => {
            Op::Dconst(*value)
        }
        Opcode::Iload(index) | Opcode::Lload(index) | Opcode::Fload(index) | Opcode::Dload(index) | Opcode::Aload(index) => Op::Load(*index),
        Opcode::Istore(index) | Opcode::Fstore(index) | Opcode::Astore(index) => Op::Store(*index),
        Opcode::Lstore(index) | Opcode::Dstore(index) => Op::StoreWide(*index),
        Opcode::Iinc(index, amount) => Op::Iinc(*index, *amount),
        Opcode::Iadd => Op::Iadd,
        Opcode::Isub => Op::Isub,
        Opcode::Imul => Op::Imul,
        Opcode::Idiv => Op::Idiv,
        Opcode::Irem => Op::Irem,
        Opcode::Ineg => Op::Ineg,
        Opcode::Iand => Op::Iand,
        Opcode::Ior => Op::Ior,
        Opcode::Ixor => Op::Ixor,
        Opcode::Ishl => Op::Ishl,
        Opcode::Ishr => Op::Ishr,
        Opcode::Iushr => Op::Iushr,
        Opcode::I2b => Op::I2b,
        Opcode::I2c => Op::I2c,
        Opcode::I2s => Op::I2s,
        Opcode::I2l => Op::I2l,
        Opcode::L2i => Op::L2i,
        Opcode::I2d => Op::I2d,
        Opcode::D2i => Op::D2i,
        Opcode::Ladd => Op::Ladd,
        Opcode::Lsub => Op::Lsub,
        Opcode::Lmul => Op::Lmul,
        Opcode::Ldiv => Op::Ldiv,
        Opcode::Lrem => Op::Lrem,
        Opcode::Lneg => Op::Lneg,
        Opcode::Land => Op::Land,
        Opcode::Lor => Op::Lor,
        Opcode::Lxor => Op::Lxor,
        Opcode::Lshl => Op::Lshl,
        Opcode::Lshr => Op::Lshr,
        Opcode::Lushr => Op::Lushr,
        Opcode::Lcmp => Op::Lcmp,
        Opcode::Dadd => Op::Dadd,
        Opcode::Dsub => Op::Dsub,
        Opcode::Dmul => Op::Dmul,
        Opcode::Ddiv => Op::Ddiv,
        Opcode::Dcmpl => Op::Dcmpl,
        Opcode::Dcmpg => Op::Dcmpg,
        Opcode::Ifeq(rel) => Op::IfEq(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifne(rel) => Op::IfNe(jump_index(code, offset, *rel as i32)?),
        Opcode::Iflt(rel) => Op::IfLt(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifge(rel) => Op::IfGe(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifgt(rel) => Op::IfGt(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifle(rel) => Op::IfLe(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmpeq(rel) => Op::IfIcmpEq(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmpne(rel) => Op::IfIcmpNe(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmplt(rel) => Op::IfIcmpLt(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmpge(rel) => Op::IfIcmpGe(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmpgt(rel) => Op::IfIcmpGt(jump_index(code, offset, *rel as i32)?),
        Opcode::IfIcmple(rel) => Op::IfIcmpLe(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifnull(rel) => Op::IfNull(jump_index(code, offset, *rel as i32)?),
        Opcode::Ifnonnull(rel) => Op::IfNonNull(jump_index(code, offset, *rel as i32)?),
        Opcode::Goto(rel) => Op::Goto(jump_index(code, offset, *rel as i32)?),
        Opcode::GotoW(rel) => Op::Goto(jump_index(code, offset, *rel)?),
        Opcode::Getfield(field) => Op::GetField(intern_field(fields, field)?),
        Opcode::Putfield(field) => Op::PutField(intern_field(fields, field)?),
        Opcode::Getstatic(field) => Op::GetStatic(intern_static_field(static_fields, field)?),
        Opcode::Invokevirtual(method) => {
            let index = intern_invoke(invokes, method, false)?;
            gfx_virtual_op(method, index).unwrap_or(Op::InvokeVirtual(index))
        }
        Opcode::Invokestatic(method) => {
            let index = intern_invoke(invokes, method, false)?;
            if is_current_time_millis(method) {
                Op::CurrentTimeMillis(index)
            } else {
                Op::InvokeStatic(index)
            }
        }
        Opcode::Invokespecial(method) => Op::InvokeSpecial(intern_invoke(invokes, method, false)?),
        Opcode::Invokeinterface(method, _, _) => Op::InvokeInterface(intern_invoke(invokes, method, true)?),
        Opcode::AconstNull => Op::Aconst,
        Opcode::Ireturn => Op::Ireturn,
        Opcode::Lreturn => Op::Lreturn,
        Opcode::Dreturn => Op::Dreturn,
        Opcode::Areturn => Op::Areturn,
        Opcode::Return => Op::Return,
        _ => return None,
    })
}

fn gfx_virtual_op(method: &ConstantPoolReference, index: u16) -> Option<Op> {
    let method = method.try_as_method_ref()?;
    if method.class.as_str() != "javax/microedition/lcdui/Graphics" {
        return None;
    }
    Some(match (method.name.as_str(), method.descriptor.as_str()) {
        ("setColor", "(I)V") => Op::GfxSetColor(index),
        ("fillRect", "(IIII)V") => Op::GfxFillRect(index),
        ("drawLine", "(IIII)V") => Op::GfxDrawLine(index),
        ("fillTriangle", "(IIIIII)V") => Op::GfxFillTriangle(index),
        ("drawRGB", "([IIIIIIIZ)V") => Op::GfxDrawRgb(index),
        ("drawImage", "(Ljavax/microedition/lcdui/Image;III)V") => Op::GfxDrawImage(index),
        ("drawRegion", "(Ljavax/microedition/lcdui/Image;IIIIIIII)V") => Op::GfxDrawRegion(index),
        ("setClip", "(IIII)V") => Op::GfxSetClip(index),
        ("clipRect", "(IIII)V") => Op::GfxClipRect(index),
        ("translate", "(II)V") => Op::GfxTranslate(index),
        _ => return None,
    })
}

fn is_current_time_millis(method: &ConstantPoolReference) -> bool {
    method.try_as_method_ref().is_some_and(|method| {
        method.class.as_str() == "java/lang/System" && method.name.as_str() == "currentTimeMillis" && method.descriptor.as_str() == "()J"
    })
}

fn intern_field(fields: &mut Vec<FieldSite>, field: &ConstantPoolReference) -> Option<u16> {
    let field = field.try_as_field_ref()?;
    if let Some(index) = fields.iter().position(|site| &*site.class == field.class.as_str() && &*site.name == field.name.as_str() && &*site.descriptor == field.descriptor.as_str()) {
        return Some(index as u16);
    }
    let index = fields.len() as u16;
    fields.push(FieldSite {
        class: Arc::from(field.class.as_str()),
        name: Arc::from(field.name.as_str()),
        descriptor: Arc::from(field.descriptor.as_str()),
        resolved: RwLock::new(None),
    });
    Some(index)
}

fn intern_static_field(fields: &mut Vec<StaticFieldSite>, field: &ConstantPoolReference) -> Option<u16> {
    let field = field.try_as_field_ref()?;
    let index = fields.len() as u16;
    fields.push(StaticFieldSite {
        class: Arc::from(field.class.as_str()),
        name: Arc::from(field.name.as_str()),
        descriptor: Arc::from(field.descriptor.as_str()),
        resolved: RwLock::new(None),
    });
    Some(index)
}

fn intern_invoke(invokes: &mut Vec<InvokeSite>, method: &ConstantPoolReference, interface: bool) -> Option<u16> {
    let method = if interface {
        method.try_as_interface_method_ref()
    } else {
        method.try_as_method_ref().or_else(|| method.try_as_interface_method_ref())
    }?;
    let kinds = method
        .method_param_kinds
        .iter()
        .map(|kind| match kind {
            classfile::MethodParamKind::Boolean => 1u8,
            classfile::MethodParamKind::Byte => 2,
            classfile::MethodParamKind::Char => 3,
            classfile::MethodParamKind::Short => 4,
            classfile::MethodParamKind::Other => 0,
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let index = invokes.len() as u16;
    invokes.push(InvokeSite {
        class: Arc::from(method.class.as_str()),
        name: Arc::from(method.name.as_str()),
        descriptor: Arc::from(method.descriptor.as_str()),
        argc: kinds.len() as u8,
        kinds,
        cache: RwLock::new(None),
    });
    Some(index)
}

struct CompiledMethod {
    ops: Box<[Op]>,
    fields: Box<[FieldSite]>,
    static_fields: Box<[StaticFieldSite]>,
    invokes: Box<[InvokeSite]>,
    max_locals: usize,
    max_stack: usize,
    return_type: JavaType,
    sync: bool,
}

enum ExecError {
    DivByZero,
    Npe,
    Java(JavaError),
    NeedAsync,
}

enum Step {
    Continue,
    Return(JavaValue),
    GetStatic(u16),
    ResolveField(u16),
    InvokeInstance { index: u16, special: bool },
    InvokeStatic(u16),
    InvokePrepared {
        index: u16,
        special: bool,
        instance: Box<dyn ClassInstance>,
        params: Vec<JavaValue>,
    },
}

#[derive(Clone)]
enum Slot {
    V,
    I(i32),
    J(i64),
    D(f64),
    A(ClassInstanceImpl),
    X(JavaValue),
}

impl Slot {
    fn from_value(value: JavaValue) -> Self {
        match value {
            JavaValue::Void => Slot::V,
            JavaValue::Boolean(value) => Slot::I(i32::from(value)),
            JavaValue::Byte(value) => Slot::I(i32::from(value)),
            JavaValue::Char(value) => Slot::I(i32::from(value)),
            JavaValue::Short(value) => Slot::I(i32::from(value)),
            JavaValue::Int(value) => Slot::I(value),
            JavaValue::Long(value) => Slot::J(value),
            JavaValue::Float(value) => Slot::D(f64::from(value)),
            JavaValue::Double(value) => Slot::D(value),
            JavaValue::Object(Some(obj)) => match ClassInstanceImpl::from_instance(obj.as_ref()) {
                Some(instance) => Slot::A(instance),
                None => Slot::X(JavaValue::Object(Some(obj))),
            },
            other => Slot::X(other),
        }
    }

    fn into_value(self) -> JavaValue {
        match self {
            Slot::V => JavaValue::Void,
            Slot::I(value) => JavaValue::Int(value),
            Slot::J(value) => JavaValue::Long(value),
            Slot::D(value) => JavaValue::Double(value),
            Slot::A(instance) => JavaValue::Object(Some(Box::new(instance))),
            Slot::X(value) => value,
        }
    }

    fn as_int(&self) -> i32 {
        match self {
            Slot::I(value) => *value,
            Slot::J(value) => *value as i32,
            Slot::X(value) => as_int(value),
            _ => 0,
        }
    }
}

#[async_trait::async_trait]
impl JvmCallback for CompiledMethod {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::int_compiler_call();
        if self.sync {
            match self.execute_sync(jvm, &args) {
                Ok(value) => return Ok(value),
                Err(ExecError::NeedAsync) => {}
                Err(error) => return Self::map_error(jvm, error).await,
            }
        }
        match self.execute(jvm, &args).await {
            Ok(value) => Ok(value),
            Err(error) => Self::map_error(jvm, error).await,
        }
    }

    fn call_sync(&self, jvm: &Jvm, args: &[JavaValue]) -> Option<Result<JavaValue>> {
        if !self.sync {
            return None;
        }
        profile::int_compiler_call();
        match self.execute_sync(jvm, args) {
            Ok(value) => Some(Ok(value)),
            Err(ExecError::Java(error)) => Some(Err(error)),
            Err(_) => None,
        }
    }
}

impl CompiledMethod {
    async fn map_error(jvm: &Jvm, error: ExecError) -> Result<JavaValue> {
        match error {
            ExecError::DivByZero => Err(jvm.exception("java/lang/ArithmeticException", "/ by zero").await),
            ExecError::Npe => Err(jvm.exception("java/lang/NullPointerException", "").await),
            ExecError::Java(error) => Err(error),
            ExecError::NeedAsync => Err(jvm.exception("java/lang/InternalError", "compiled method needed async").await),
        }
    }

    fn locals_from(&self, args: &[JavaValue]) -> Vec<JavaValue> {
        let mut locals = vec![JavaValue::Void; self.max_locals];
        let mut local_index = 0usize;
        for arg in args {
            if local_index >= locals.len() {
                break;
            }
            let wide = matches!(arg, JavaValue::Long(_) | JavaValue::Double(_));
            locals[local_index] = to_stack(arg.clone());
            local_index += 1;
            if wide {
                if local_index < locals.len() {
                    locals[local_index] = JavaValue::Void;
                }
                local_index += 1;
            }
        }
        locals
    }

    fn execute_sync(&self, jvm: &Jvm, args: &[JavaValue]) -> core::result::Result<JavaValue, ExecError> {
        let mut locals: Vec<Slot> = self.locals_from(args).into_iter().map(Slot::from_value).collect();
        let mut stack: Vec<Slot> = Vec::with_capacity(self.max_stack);
        let mut pc = 0usize;
        while pc < self.ops.len() {
        match self.ops[pc] {
                Op::Nop => pc += 1,
                Op::Pop => {
                    stack.pop();
                    pc += 1;
                }
                Op::Pop2 => {
                    stack.pop();
                    stack.pop();
                    pc += 1;
                }
                Op::Dup => {
                    if let Some(top) = stack.last().cloned() {
                        stack.push(top);
                    }
                    pc += 1;
                }
                Op::Dup2 => {
                    let len = stack.len();
                    if len >= 2 {
                        stack.push(stack[len - 2].clone());
                        stack.push(stack[len - 1].clone());
                    }
                    pc += 1;
                }
                Op::Swap => {
                    let len = stack.len();
                    if len >= 2 {
                        stack.swap(len - 1, len - 2);
                    }
                    pc += 1;
                }
                Op::Aconst => {
                    stack.push(Slot::X(JavaValue::Object(None)));
                    pc += 1;
                }
                Op::Iconst(value) => {
                    stack.push(Slot::I(value));
                    pc += 1;
                }
                Op::Lconst(value) => {
                    stack.push(Slot::J(value));
                    pc += 1;
                }
                Op::Dconst(value) => {
                    stack.push(Slot::D(value));
                    pc += 1;
                }
                Op::Load(index) => {
                    stack.push(locals[index as usize].clone());
                    pc += 1;
                }
                Op::Store(index) => {
                    locals[index as usize] = stack.pop().unwrap_or(Slot::V);
                    pc += 1;
                }
                Op::StoreWide(index) => {
                    locals[index as usize] = stack.pop().unwrap_or(Slot::V);
                    let next = index as usize + 1;
                    if next < locals.len() {
                        locals[next] = Slot::V;
                    }
                    pc += 1;
                }
                Op::Iinc(index, amount) => {
                    let value = locals[index as usize].as_int().wrapping_add(i32::from(amount));
                    locals[index as usize] = Slot::I(value);
                    pc += 1;
                }
                Op::Iadd => slot_ibinop(&mut stack, i32::wrapping_add, &mut pc),
                Op::Isub => slot_ibinop(&mut stack, i32::wrapping_sub, &mut pc),
                Op::Imul => slot_ibinop(&mut stack, i32::wrapping_mul, &mut pc),
                Op::Idiv => {
                    let b = slot_pop_int(&mut stack);
                    let a = slot_pop_int(&mut stack);
                    stack.push(Slot::I(java_idiv(a, b)?));
                    pc += 1;
                }
                Op::Irem => {
                    let b = slot_pop_int(&mut stack);
                    let a = slot_pop_int(&mut stack);
                    stack.push(Slot::I(java_irem(a, b)?));
                    pc += 1;
                }
                Op::Ineg => {
                    let value = slot_pop_int(&mut stack);
                    stack.push(Slot::I(value.wrapping_neg()));
                    pc += 1;
                }
                Op::Iand => slot_ibinop(&mut stack, core::ops::BitAnd::bitand, &mut pc),
                Op::Ior => slot_ibinop(&mut stack, core::ops::BitOr::bitor, &mut pc),
                Op::Ixor => slot_ibinop(&mut stack, core::ops::BitXor::bitxor, &mut pc),
                Op::Ishl => slot_ibinop(&mut stack, |a, b| a.wrapping_shl(b as u32), &mut pc),
                Op::Ishr => slot_ibinop(&mut stack, |a, b| a.wrapping_shr(b as u32), &mut pc),
                Op::Iushr => slot_ibinop(&mut stack, |a, b| (a as u32).wrapping_shr(b as u32) as i32, &mut pc),
                Op::I2b => {
                    let value = slot_pop_int(&mut stack);
                    stack.push(Slot::I(value as i8 as i32));
                    pc += 1;
                }
                Op::I2c => {
                    let value = slot_pop_int(&mut stack);
                    stack.push(Slot::I(value as u16 as i32));
                    pc += 1;
                }
                Op::I2s => {
                    let value = slot_pop_int(&mut stack);
                    stack.push(Slot::I(value as i16 as i32));
                    pc += 1;
                }
                Op::I2l => {
                    let value = slot_pop_int(&mut stack);
                    stack.push(Slot::J(i64::from(value)));
                    pc += 1;
                }
                Op::L2i => {
                    let value = slot_pop_long(&mut stack);
                    stack.push(Slot::I(value as i32));
                    pc += 1;
                }
                Op::I2d => {
                    let value = slot_pop_int(&mut stack);
                    stack.push(Slot::D(f64::from(value)));
                    pc += 1;
                }
                Op::D2i => {
                    let value = slot_pop_double(&mut stack);
                    stack.push(Slot::I(value as i32));
                    pc += 1;
                }
                Op::Ladd => slot_lbinop(&mut stack, i64::wrapping_add, &mut pc),
                Op::Lsub => slot_lbinop(&mut stack, i64::wrapping_sub, &mut pc),
                Op::Lmul => slot_lbinop(&mut stack, i64::wrapping_mul, &mut pc),
                Op::Ldiv => {
                    let b = slot_pop_long(&mut stack);
                    let a = slot_pop_long(&mut stack);
                    stack.push(Slot::J(java_ldiv(a, b)?));
                    pc += 1;
                }
                Op::Lrem => {
                    let b = slot_pop_long(&mut stack);
                    let a = slot_pop_long(&mut stack);
                    stack.push(Slot::J(java_lrem(a, b)?));
                    pc += 1;
                }
                Op::Lneg => {
                    let value = slot_pop_long(&mut stack);
                    stack.push(Slot::J(value.wrapping_neg()));
                    pc += 1;
                }
                Op::Land => slot_lbinop(&mut stack, core::ops::BitAnd::bitand, &mut pc),
                Op::Lor => slot_lbinop(&mut stack, core::ops::BitOr::bitor, &mut pc),
                Op::Lxor => slot_lbinop(&mut stack, core::ops::BitXor::bitxor, &mut pc),
                Op::Lshl => {
                    let shift = slot_pop_int(&mut stack);
                    let value = slot_pop_long(&mut stack);
                    stack.push(Slot::J(value.wrapping_shl(shift as u32)));
                    pc += 1;
                }
                Op::Lshr => {
                    let shift = slot_pop_int(&mut stack);
                    let value = slot_pop_long(&mut stack);
                    stack.push(Slot::J(value.wrapping_shr(shift as u32)));
                    pc += 1;
                }
                Op::Lushr => {
                    let shift = slot_pop_int(&mut stack);
                    let value = slot_pop_long(&mut stack);
                    stack.push(Slot::J((value as u64).wrapping_shr(shift as u32) as i64));
                    pc += 1;
                }
                Op::Lcmp => {
                    let b = slot_pop_long(&mut stack);
                    let a = slot_pop_long(&mut stack);
                    stack.push(Slot::I(match a.cmp(&b) {
                        core::cmp::Ordering::Less => -1,
                        core::cmp::Ordering::Equal => 0,
                        core::cmp::Ordering::Greater => 1,
                    }));
                    pc += 1;
                }
                Op::Dadd => slot_dbinop(&mut stack, |a, b| a + b, &mut pc),
                Op::Dsub => slot_dbinop(&mut stack, |a, b| a - b, &mut pc),
                Op::Dmul => slot_dbinop(&mut stack, |a, b| a * b, &mut pc),
                Op::Ddiv => slot_dbinop(&mut stack, |a, b| a / b, &mut pc),
                Op::Dcmpl => {
                    let b = slot_pop_double(&mut stack);
                    let a = slot_pop_double(&mut stack);
                    stack.push(Slot::I(dcmp(a, b, -1)));
                    pc += 1;
                }
                Op::Dcmpg => {
                    let b = slot_pop_double(&mut stack);
                    let a = slot_pop_double(&mut stack);
                    stack.push(Slot::I(dcmp(a, b, 1)));
                    pc += 1;
                }
                Op::IfEq(target) => slot_branch1(&mut stack, &mut pc, target, |value| value == 0),
                Op::IfNe(target) => slot_branch1(&mut stack, &mut pc, target, |value| value != 0),
                Op::IfLt(target) => slot_branch1(&mut stack, &mut pc, target, |value| value < 0),
                Op::IfGe(target) => slot_branch1(&mut stack, &mut pc, target, |value| value >= 0),
                Op::IfGt(target) => slot_branch1(&mut stack, &mut pc, target, |value| value > 0),
                Op::IfLe(target) => slot_branch1(&mut stack, &mut pc, target, |value| value <= 0),
                Op::IfIcmpEq(target) => slot_branch2(&mut stack, &mut pc, target, |a, b| a == b),
                Op::IfIcmpNe(target) => slot_branch2(&mut stack, &mut pc, target, |a, b| a != b),
                Op::IfIcmpLt(target) => slot_branch2(&mut stack, &mut pc, target, |a, b| a < b),
                Op::IfIcmpGe(target) => slot_branch2(&mut stack, &mut pc, target, |a, b| a >= b),
                Op::IfIcmpGt(target) => slot_branch2(&mut stack, &mut pc, target, |a, b| a > b),
                Op::IfIcmpLe(target) => slot_branch2(&mut stack, &mut pc, target, |a, b| a <= b),
                Op::IfNull(target) => {
                    let value = stack.pop().unwrap_or(Slot::X(JavaValue::Object(None)));
                    if matches!(value, Slot::X(JavaValue::Object(None))) {
                        pc = target as usize;
                    } else {
                        pc += 1;
                    }
                }
                Op::IfNonNull(target) => {
                    let value = stack.pop().unwrap_or(Slot::X(JavaValue::Object(None)));
                    if matches!(value, Slot::A(_) | Slot::X(JavaValue::Object(Some(_)))) {
                        pc = target as usize;
                    } else {
                        pc += 1;
                    }
                }
                Op::Goto(target) => pc = target as usize,
                Op::GetField(index) => {
                    let site = &self.fields[index as usize];
                    let guard = site.resolved.read();
                    let Some(resolved) = guard.as_ref() else {
                        return Err(ExecError::NeedAsync);
                    };
                    let instance = slot_pop_any(&mut stack)?;
                    let value = jvm.get_resolved_instance_field(resolved, &instance).map_err(ExecError::Java)?;
                    stack.push(Slot::from_value(to_stack(value)));
                    pc += 1;
                }
                Op::GetThisField(index) => {
                    let site = &self.fields[index as usize];
                    let guard = site.resolved.read();
                    let Some(resolved) = guard.as_ref() else {
                        return Err(ExecError::NeedAsync);
                    };
                    let instance = match locals.first() {
                        Some(Slot::A(instance)) => Box::new(instance.clone()) as Box<dyn ClassInstance>,
                        Some(Slot::X(JavaValue::Object(Some(instance)))) => instance.clone(),
                        _ => return Err(ExecError::Npe),
                    };
                    let value = jvm.get_resolved_instance_field(resolved, &instance).map_err(ExecError::Java)?;
                    stack.push(Slot::from_value(to_stack(value)));
                    pc += 1;
                }
                Op::PutField(index) => {
                    let site = &self.fields[index as usize];
                    let guard = site.resolved.read();
                    let Some(resolved) = guard.as_ref() else {
                        return Err(ExecError::NeedAsync);
                    };
                    let value = stack.pop().unwrap_or(Slot::V).into_value();
                    let mut instance = slot_pop_any(&mut stack)?;
                    let value = to_field(&site.descriptor, value);
                    jvm.put_resolved_instance_field(resolved, &mut instance, value).map_err(ExecError::Java)?;
                    pc += 1;
                }
                Op::GetStatic(_) => return Err(ExecError::NeedAsync),
                Op::GfxSetColor(_) => {
                    let rgb = slot_pop_int(&mut stack);
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::set_color_fast(&graphics, rgb) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::GfxFillRect(_) => {
                    let height = slot_pop_int(&mut stack);
                    let width = slot_pop_int(&mut stack);
                    let y = slot_pop_int(&mut stack);
                    let x = slot_pop_int(&mut stack);
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::fill_rect_fast(&graphics, x, y, width, height) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::GfxDrawLine(_) => {
                    let y2 = slot_pop_int(&mut stack);
                    let x2 = slot_pop_int(&mut stack);
                    let y1 = slot_pop_int(&mut stack);
                    let x1 = slot_pop_int(&mut stack);
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::draw_line_fast(&graphics, x1, y1, x2, y2) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::GfxFillTriangle(_) => {
                    let y3 = slot_pop_int(&mut stack);
                    let x3 = slot_pop_int(&mut stack);
                    let y2 = slot_pop_int(&mut stack);
                    let x2 = slot_pop_int(&mut stack);
                    let y1 = slot_pop_int(&mut stack);
                    let x1 = slot_pop_int(&mut stack);
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::fill_triangle_fast(&graphics, x1, y1, x2, y2, x3, y3) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::GfxDrawRgb(_) => {
                    let process_alpha = slot_pop_int(&mut stack) != 0;
                    let height = slot_pop_int(&mut stack);
                    let width = slot_pop_int(&mut stack);
                    let y = slot_pop_int(&mut stack);
                    let x = slot_pop_int(&mut stack);
                    let scan_length = slot_pop_int(&mut stack);
                    let offset = slot_pop_int(&mut stack);
                    let rgb = slot_pop_any(&mut stack)?;
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::draw_rgb(&graphics, rgb.as_ref(), offset, scan_length, x, y, width, height, process_alpha) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::GfxDrawImage(_) => {
                    let anchor = slot_pop_int(&mut stack);
                    let y = slot_pop_int(&mut stack);
                    let x = slot_pop_int(&mut stack);
                    let image = slot_pop_obj(&mut stack)?;
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::draw_image(&graphics, &image, x, y, anchor) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::GfxDrawRegion(_) => {
                    let anchor = slot_pop_int(&mut stack);
                    let dest_y = slot_pop_int(&mut stack);
                    let dest_x = slot_pop_int(&mut stack);
                    let transform = slot_pop_int(&mut stack);
                    let height = slot_pop_int(&mut stack);
                    let width = slot_pop_int(&mut stack);
                    let src_y = slot_pop_int(&mut stack);
                    let src_x = slot_pop_int(&mut stack);
                    let image = slot_pop_obj(&mut stack)?;
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::draw_region(&graphics, &image, src_x, src_y, width, height, transform, dest_x, dest_y, anchor) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::GfxSetClip(_) => {
                    let height = slot_pop_int(&mut stack);
                    let width = slot_pop_int(&mut stack);
                    let y = slot_pop_int(&mut stack);
                    let x = slot_pop_int(&mut stack);
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::set_clip(&graphics, x, y, width, height) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::GfxClipRect(_) => {
                    let height = slot_pop_int(&mut stack);
                    let width = slot_pop_int(&mut stack);
                    let y = slot_pop_int(&mut stack);
                    let x = slot_pop_int(&mut stack);
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::clip_rect(&graphics, x, y, width, height) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::GfxTranslate(_) => {
                    let y = slot_pop_int(&mut stack);
                    let x = slot_pop_int(&mut stack);
                    let graphics = slot_pop_obj(&mut stack)?;
                    if !super::gfx::translate(&graphics, x, y) {
                        return Err(ExecError::NeedAsync);
                    }
                    pc += 1;
                }
                Op::CurrentTimeMillis(_) => {
                    if let Some(now) = jvm.now_millis() {
                        stack.push(Slot::J(now));
                        pc += 1;
                    } else {
                        return Err(ExecError::NeedAsync);
                    }
                }
                Op::InvokeVirtual(_) => return Err(ExecError::NeedAsync),
                Op::InvokeInterface(_) => return Err(ExecError::NeedAsync),
                Op::InvokeSpecial(_) => return Err(ExecError::NeedAsync),
                Op::InvokeStatic(_) => return Err(ExecError::NeedAsync),
                Op::Ireturn => return Ok(return_int(&self.return_type, slot_pop_int(&mut stack))),
                Op::Lreturn => return Ok(JavaValue::Long(slot_pop_long(&mut stack))),
                Op::Dreturn => return Ok(JavaValue::Double(slot_pop_double(&mut stack))),
                Op::Areturn => return Ok(stack.pop().unwrap_or(Slot::X(JavaValue::Object(None))).into_value()),
                Op::Return => return Ok(JavaValue::Void),
            }
        }
        Ok(JavaValue::Void)
    }

    async fn execute(&self, jvm: &Jvm, args: &[JavaValue]) -> core::result::Result<JavaValue, ExecError> {
        let mut locals = self.locals_from(args);
        let mut stack: Vec<JavaValue> = Vec::with_capacity(self.max_stack);
        let mut pc = 0usize;
        while pc < self.ops.len() {
            match self.step(jvm, &mut locals, &mut stack, &mut pc)? {
                Step::Continue => {}
                Step::Return(value) => return Ok(value),
                Step::GetStatic(index) => {
                    let site = &self.static_fields[index as usize];
                    let resolved = {
                        let cached = site.resolved.read().clone();
                        match cached {
                            Some(resolved) => resolved,
                            None => {
                                let resolved = jvm
                                    .resolve_static_field(&site.class, &site.name, &site.descriptor)
                                    .await
                                    .map_err(ExecError::Java)?;
                                *site.resolved.write() = Some(resolved.clone());
                                resolved
                            }
                        }
                    };
                    let value = jvm.get_resolved_static_field(&resolved).map_err(ExecError::Java)?;
                    stack.push(to_stack(value));
                    pc += 1;
                }
                Step::ResolveField(index) => {
                    let site = &self.fields[index as usize];
                    let resolved = jvm
                        .resolve_instance_field(&site.class, &site.name, &site.descriptor)
                        .await
                        .map_err(ExecError::Java)?;
                    *site.resolved.write() = Some(resolved);
                }
                Step::InvokeInstance { index, special } => {
                    self.invoke_instance(jvm, &mut stack, index, special).await?;
                    pc += 1;
                }
                Step::InvokeStatic(index) => {
                    self.invoke_static(jvm, &mut stack, index).await?;
                    pc += 1;
                }
                Step::InvokePrepared {
                    index,
                    special,
                    instance,
                    params,
                } => {
                    self.invoke_prepared(jvm, &mut stack, instance, params, index, special).await?;
                    pc += 1;
                }
            }
        }
        Ok(JavaValue::Void)
    }

    fn step(
        &self,
        jvm: &Jvm,
        locals: &mut [JavaValue],
        stack: &mut Vec<JavaValue>,
        pc: &mut usize,
    ) -> core::result::Result<Step, ExecError> {
        match self.ops[*pc] {
                Op::Nop => *pc += 1,
                Op::Pop => {
                    stack.pop();
                    *pc += 1;
                }
                Op::Pop2 => {
                    stack.pop();
                    stack.pop();
                    *pc += 1;
                }
                Op::Dup => {
                    if let Some(top) = stack.last().cloned() {
                        stack.push(top);
                    }
                    *pc += 1;
                }
                Op::Dup2 => {
                    let len = stack.len();
                    if len >= 2 {
                        stack.push(stack[len - 2].clone());
                        stack.push(stack[len - 1].clone());
                    }
                    *pc += 1;
                }
                Op::Swap => {
                    let len = stack.len();
                    if len >= 2 {
                        stack.swap(len - 1, len - 2);
                    }
                    *pc += 1;
                }
                Op::Aconst => {
                    stack.push(JavaValue::Object(None));
                    *pc += 1;
                }
                Op::Iconst(value) => {
                    stack.push(JavaValue::Int(value));
                    *pc += 1;
                }
                Op::Lconst(value) => {
                    stack.push(JavaValue::Long(value));
                    *pc += 1;
                }
                Op::Dconst(value) => {
                    stack.push(JavaValue::Double(value));
                    *pc += 1;
                }
                Op::Load(index) => {
                    stack.push(locals[index as usize].clone());
                    *pc += 1;
                }
                Op::Store(index) => {
                    locals[index as usize] = stack.pop().unwrap_or(JavaValue::Void);
                    *pc += 1;
                }
                Op::StoreWide(index) => {
                    locals[index as usize] = stack.pop().unwrap_or(JavaValue::Void);
                    let next = index as usize + 1;
                    if next < locals.len() {
                        locals[next] = JavaValue::Void;
                    }
                    *pc += 1;
                }
                Op::Iinc(index, amount) => {
                    let value = as_int(&locals[index as usize]).wrapping_add(i32::from(amount));
                    locals[index as usize] = JavaValue::Int(value);
                    *pc += 1;
                }
                Op::Iadd => ibinop(stack, i32::wrapping_add, pc),
                Op::Isub => ibinop(stack, i32::wrapping_sub, pc),
                Op::Imul => ibinop(stack, i32::wrapping_mul, pc),
                Op::Idiv => {
                    let b = pop_int(stack);
                    let a = pop_int(stack);
                    stack.push(JavaValue::Int(java_idiv(a, b)?));
                    *pc += 1;
                }
                Op::Irem => {
                    let b = pop_int(stack);
                    let a = pop_int(stack);
                    stack.push(JavaValue::Int(java_irem(a, b)?));
                    *pc += 1;
                }
                Op::Ineg => {
                    let value = pop_int(stack);
                    stack.push(JavaValue::Int(value.wrapping_neg()));
                    *pc += 1;
                }
                Op::Iand => ibinop(stack, core::ops::BitAnd::bitand, pc),
                Op::Ior => ibinop(stack, core::ops::BitOr::bitor, pc),
                Op::Ixor => ibinop(stack, core::ops::BitXor::bitxor, pc),
                Op::Ishl => ibinop(stack, |a, b| a.wrapping_shl(b as u32), pc),
                Op::Ishr => ibinop(stack, |a, b| a.wrapping_shr(b as u32), pc),
                Op::Iushr => ibinop(stack, |a, b| (a as u32).wrapping_shr(b as u32) as i32, pc),
                Op::I2b => {
                    let value = pop_int(stack);
                    stack.push(JavaValue::Int(value as i8 as i32));
                    *pc += 1;
                }
                Op::I2c => {
                    let value = pop_int(stack);
                    stack.push(JavaValue::Int(value as u16 as i32));
                    *pc += 1;
                }
                Op::I2s => {
                    let value = pop_int(stack);
                    stack.push(JavaValue::Int(value as i16 as i32));
                    *pc += 1;
                }
                Op::I2l => {
                    let value = pop_int(stack);
                    stack.push(JavaValue::Long(i64::from(value)));
                    *pc += 1;
                }
                Op::L2i => {
                    let value = pop_long(stack);
                    stack.push(JavaValue::Int(value as i32));
                    *pc += 1;
                }
                Op::I2d => {
                    let value = pop_int(stack);
                    stack.push(JavaValue::Double(f64::from(value)));
                    *pc += 1;
                }
                Op::D2i => {
                    let value = pop_double(stack);
                    stack.push(JavaValue::Int(value as i32));
                    *pc += 1;
                }
                Op::Ladd => lbinop(stack, i64::wrapping_add, pc),
                Op::Lsub => lbinop(stack, i64::wrapping_sub, pc),
                Op::Lmul => lbinop(stack, i64::wrapping_mul, pc),
                Op::Ldiv => {
                    let b = pop_long(stack);
                    let a = pop_long(stack);
                    stack.push(JavaValue::Long(java_ldiv(a, b)?));
                    *pc += 1;
                }
                Op::Lrem => {
                    let b = pop_long(stack);
                    let a = pop_long(stack);
                    stack.push(JavaValue::Long(java_lrem(a, b)?));
                    *pc += 1;
                }
                Op::Lneg => {
                    let value = pop_long(stack);
                    stack.push(JavaValue::Long(value.wrapping_neg()));
                    *pc += 1;
                }
                Op::Land => lbinop(stack, core::ops::BitAnd::bitand, pc),
                Op::Lor => lbinop(stack, core::ops::BitOr::bitor, pc),
                Op::Lxor => lbinop(stack, core::ops::BitXor::bitxor, pc),
                Op::Lshl => {
                    let shift = pop_int(stack);
                    let value = pop_long(stack);
                    stack.push(JavaValue::Long(value.wrapping_shl(shift as u32)));
                    *pc += 1;
                }
                Op::Lshr => {
                    let shift = pop_int(stack);
                    let value = pop_long(stack);
                    stack.push(JavaValue::Long(value.wrapping_shr(shift as u32)));
                    *pc += 1;
                }
                Op::Lushr => {
                    let shift = pop_int(stack);
                    let value = pop_long(stack);
                    stack.push(JavaValue::Long((value as u64).wrapping_shr(shift as u32) as i64));
                    *pc += 1;
                }
                Op::Lcmp => {
                    let b = pop_long(stack);
                    let a = pop_long(stack);
                    stack.push(JavaValue::Int(match a.cmp(&b) {
                        core::cmp::Ordering::Less => -1,
                        core::cmp::Ordering::Equal => 0,
                        core::cmp::Ordering::Greater => 1,
                    }));
                    *pc += 1;
                }
                Op::Dadd => dbinop(stack, |a, b| a + b, pc),
                Op::Dsub => dbinop(stack, |a, b| a - b, pc),
                Op::Dmul => dbinop(stack, |a, b| a * b, pc),
                Op::Ddiv => dbinop(stack, |a, b| a / b, pc),
                Op::Dcmpl => {
                    let b = pop_double(stack);
                    let a = pop_double(stack);
                    stack.push(JavaValue::Int(dcmp(a, b, -1)));
                    *pc += 1;
                }
                Op::Dcmpg => {
                    let b = pop_double(stack);
                    let a = pop_double(stack);
                    stack.push(JavaValue::Int(dcmp(a, b, 1)));
                    *pc += 1;
                }
                Op::IfEq(target) => branch1(stack, pc, target, |value| value == 0),
                Op::IfNe(target) => branch1(stack, pc, target, |value| value != 0),
                Op::IfLt(target) => branch1(stack, pc, target, |value| value < 0),
                Op::IfGe(target) => branch1(stack, pc, target, |value| value >= 0),
                Op::IfGt(target) => branch1(stack, pc, target, |value| value > 0),
                Op::IfLe(target) => branch1(stack, pc, target, |value| value <= 0),
                Op::IfIcmpEq(target) => branch2(stack, pc, target, |a, b| a == b),
                Op::IfIcmpNe(target) => branch2(stack, pc, target, |a, b| a != b),
                Op::IfIcmpLt(target) => branch2(stack, pc, target, |a, b| a < b),
                Op::IfIcmpGe(target) => branch2(stack, pc, target, |a, b| a >= b),
                Op::IfIcmpGt(target) => branch2(stack, pc, target, |a, b| a > b),
                Op::IfIcmpLe(target) => branch2(stack, pc, target, |a, b| a <= b),
                Op::IfNull(target) => {
                    let value = stack.pop().unwrap_or(JavaValue::Object(None));
                    if matches!(value, JavaValue::Object(None)) {
                        *pc = target as usize;
                    } else {
                        *pc += 1;
                    }
                }
                Op::IfNonNull(target) => {
                    let value = stack.pop().unwrap_or(JavaValue::Object(None));
                    if matches!(value, JavaValue::Object(Some(_))) {
                        *pc = target as usize;
                    } else {
                        *pc += 1;
                    }
                }
                Op::Goto(target) => *pc = target as usize,
                Op::GetField(index) => {
                    let site = &self.fields[index as usize];
                    let guard = site.resolved.read();
                    if let Some(resolved) = guard.as_ref() {
                        let instance = pop_object(stack)?;
                        let value = jvm.get_resolved_instance_field(resolved, &instance).map_err(ExecError::Java)?;
                        stack.push(to_stack(value));
                        *pc += 1;
                    } else {
                        drop(guard);
                        return Ok(Step::ResolveField(index));
                    }
                }
                Op::GetThisField(index) => {
                    let site = &self.fields[index as usize];
                    let guard = site.resolved.read();
                    if let Some(resolved) = guard.as_ref() {
                        let instance = match locals.first() {
                            Some(JavaValue::Object(Some(obj))) => obj,
                            _ => return Err(ExecError::Npe),
                        };
                        let value = jvm.get_resolved_instance_field(resolved, instance).map_err(ExecError::Java)?;
                        stack.push(to_stack(value));
                        *pc += 1;
                    } else {
                        drop(guard);
                        return Ok(Step::ResolveField(index));
                    }
                }
                Op::PutField(index) => {
                    let site = &self.fields[index as usize];
                    let guard = site.resolved.read();
                    if let Some(resolved) = guard.as_ref() {
                        let value = stack.pop().unwrap_or(JavaValue::Void);
                        let mut instance = pop_object(stack)?;
                        let value = to_field(&site.descriptor, value);
                        jvm.put_resolved_instance_field(resolved, &mut instance, value).map_err(ExecError::Java)?;
                        *pc += 1;
                    } else {
                        drop(guard);
                        return Ok(Step::ResolveField(index));
                    }
                }
                Op::GetStatic(index) => return Ok(Step::GetStatic(index)),
                Op::GfxSetColor(index) => {
                    let rgb = pop_int(stack);
                    let graphics = pop_object(stack)?;
                    if !super::gfx::set_color(&*graphics, rgb) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![JavaValue::Int(rgb)],
                        });
                    }
                    *pc += 1;
                }
                Op::GfxFillRect(index) => {
                    let height = pop_int(stack);
                    let width = pop_int(stack);
                    let y = pop_int(stack);
                    let x = pop_int(stack);
                    let graphics = pop_object(stack)?;
                    if !super::gfx::fill_rect(&*graphics, x, y, width, height) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![JavaValue::Int(x), JavaValue::Int(y), JavaValue::Int(width), JavaValue::Int(height)],
                        });
                    }
                    *pc += 1;
                }
                Op::GfxDrawLine(index) => {
                    let y2 = pop_int(stack);
                    let x2 = pop_int(stack);
                    let y1 = pop_int(stack);
                    let x1 = pop_int(stack);
                    let graphics = pop_object(stack)?;
                    if !super::gfx::draw_line(&*graphics, x1, y1, x2, y2) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![JavaValue::Int(x1), JavaValue::Int(y1), JavaValue::Int(x2), JavaValue::Int(y2)],
                        });
                    }
                    *pc += 1;
                }
                Op::GfxFillTriangle(index) => {
                    let y3 = pop_int(stack);
                    let x3 = pop_int(stack);
                    let y2 = pop_int(stack);
                    let x2 = pop_int(stack);
                    let y1 = pop_int(stack);
                    let x1 = pop_int(stack);
                    let graphics = pop_object(stack)?;
                    if !super::gfx::fill_triangle(&*graphics, x1, y1, x2, y2, x3, y3) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![
                                JavaValue::Int(x1),
                                JavaValue::Int(y1),
                                JavaValue::Int(x2),
                                JavaValue::Int(y2),
                                JavaValue::Int(x3),
                                JavaValue::Int(y3)
                            ],
                        });
                    }
                    *pc += 1;
                }
                Op::GfxDrawRgb(index) => {
                    let process_alpha = pop_int(stack) != 0;
                    let height = pop_int(stack);
                    let width = pop_int(stack);
                    let y = pop_int(stack);
                    let x = pop_int(stack);
                    let scan_length = pop_int(stack);
                    let offset = pop_int(stack);
                    let rgb = pop_object(stack)?;
                    let graphics = pop_object(stack)?;
                    if !super::gfx::draw_rgb(&*graphics, &*rgb, offset, scan_length, x, y, width, height, process_alpha) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![
                                JavaValue::Object(Some(rgb)),
                                JavaValue::Int(offset),
                                JavaValue::Int(scan_length),
                                JavaValue::Int(x),
                                JavaValue::Int(y),
                                JavaValue::Int(width),
                                JavaValue::Int(height),
                                JavaValue::Boolean(process_alpha)
                            ],
                        });
                    }
                    *pc += 1;
                }
                Op::GfxDrawImage(index) => {
                    let anchor = pop_int(stack);
                    let y = pop_int(stack);
                    let x = pop_int(stack);
                    let image = pop_object(stack)?;
                    let graphics = pop_object(stack)?;
                    if !super::gfx::draw_image(&*graphics, &*image, x, y, anchor) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![JavaValue::Object(Some(image)), JavaValue::Int(x), JavaValue::Int(y), JavaValue::Int(anchor)],
                        });
                    }
                    *pc += 1;
                }
                Op::GfxDrawRegion(index) => {
                    let anchor = pop_int(stack);
                    let dest_y = pop_int(stack);
                    let dest_x = pop_int(stack);
                    let transform = pop_int(stack);
                    let height = pop_int(stack);
                    let width = pop_int(stack);
                    let src_y = pop_int(stack);
                    let src_x = pop_int(stack);
                    let image = pop_object(stack)?;
                    let graphics = pop_object(stack)?;
                    if !super::gfx::draw_region(&*graphics, &*image, src_x, src_y, width, height, transform, dest_x, dest_y, anchor) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![
                                JavaValue::Object(Some(image)),
                                JavaValue::Int(src_x),
                                JavaValue::Int(src_y),
                                JavaValue::Int(width),
                                JavaValue::Int(height),
                                JavaValue::Int(transform),
                                JavaValue::Int(dest_x),
                                JavaValue::Int(dest_y),
                                JavaValue::Int(anchor)
                            ],
                        });
                    }
                    *pc += 1;
                }
                Op::GfxSetClip(index) => {
                    let height = pop_int(stack);
                    let width = pop_int(stack);
                    let y = pop_int(stack);
                    let x = pop_int(stack);
                    let graphics = pop_object(stack)?;
                    if !super::gfx::set_clip(&*graphics, x, y, width, height) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![JavaValue::Int(x), JavaValue::Int(y), JavaValue::Int(width), JavaValue::Int(height)],
                        });
                    }
                    *pc += 1;
                }
                Op::GfxClipRect(index) => {
                    let height = pop_int(stack);
                    let width = pop_int(stack);
                    let y = pop_int(stack);
                    let x = pop_int(stack);
                    let graphics = pop_object(stack)?;
                    if !super::gfx::clip_rect(&*graphics, x, y, width, height) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![JavaValue::Int(x), JavaValue::Int(y), JavaValue::Int(width), JavaValue::Int(height)],
                        });
                    }
                    *pc += 1;
                }
                Op::GfxTranslate(index) => {
                    let y = pop_int(stack);
                    let x = pop_int(stack);
                    let graphics = pop_object(stack)?;
                    if !super::gfx::translate(&*graphics, x, y) {
                        return Ok(Step::InvokePrepared {
                            index,
                            special: false,
                            instance: graphics,
                            params: alloc::vec![JavaValue::Int(x), JavaValue::Int(y)],
                        });
                    }
                    *pc += 1;
                }
                Op::CurrentTimeMillis(index) => {
                    if let Some(now) = jvm.now_millis() {
                        stack.push(JavaValue::Long(now));
                        *pc += 1;
                    } else {
                        return Ok(Step::InvokeStatic(index));
                    }
                }
                Op::InvokeVirtual(index) => return Ok(Step::InvokeInstance { index, special: false }),
                Op::InvokeInterface(index) => return Ok(Step::InvokeInstance { index, special: false }),
                Op::InvokeSpecial(index) => return Ok(Step::InvokeInstance { index, special: true }),
                Op::InvokeStatic(index) => return Ok(Step::InvokeStatic(index)),
                Op::Ireturn => return Ok(Step::Return(return_int(&self.return_type, pop_int(stack)))),
                Op::Lreturn => return Ok(Step::Return(JavaValue::Long(pop_long(stack)))),
                Op::Dreturn => return Ok(Step::Return(JavaValue::Double(pop_double(stack)))),
                Op::Areturn => return Ok(Step::Return(stack.pop().unwrap_or(JavaValue::Object(None)))),
                Op::Return => return Ok(Step::Return(JavaValue::Void)),
            }
        Ok(Step::Continue)
    }

    async fn invoke_instance(
        &self,
        jvm: &Jvm,
        stack: &mut Vec<JavaValue>,
        index: u16,
        special: bool,
    ) -> core::result::Result<(), ExecError> {
        let site = &self.invokes[index as usize];
        let mut params = pop_params(stack, site);
        let instance = pop_object(stack)?;
        convert_params(&mut params, &site.kinds);
        self.invoke_prepared(jvm, stack, instance, params, index, special).await
    }

    async fn invoke_prepared(
        &self,
        jvm: &Jvm,
        stack: &mut Vec<JavaValue>,
        instance: Box<dyn ClassInstance>,
        params: Vec<JavaValue>,
        index: u16,
        special: bool,
    ) -> core::result::Result<(), ExecError> {
        let site = &self.invokes[index as usize];
        let receiver_class = instance.class_definition().name();
        let resolved = {
            let cached = site.cache.read();
            match cached.as_ref() {
                Some((class, method)) if class == &receiver_class => Some(method.clone()),
                _ => None,
            }
        };
        let resolved = match resolved {
            Some(resolved) => resolved,
            None => {
                let resolved = if special {
                    jvm.resolve_special_method(&site.class, &site.name, &site.descriptor)
                        .await
                        .map_err(ExecError::Java)?
                } else {
                    jvm.resolve_virtual_method_for_instance(&instance, &site.name, &site.descriptor)
                        .await
                        .map_err(ExecError::Java)?
                };
                *site.cache.write() = Some((receiver_class, resolved.clone()));
                resolved
            }
        };
        let mut args = Vec::with_capacity(params.len() + 1);
        args.push(JavaValue::Object(Some(instance)));
        args.extend(params);
        match resolved.run_sync(jvm, &args) {
            Some(Ok(result)) => {
                push_result(stack, result);
                return Ok(());
            }
            Some(Err(error)) => return Err(ExecError::Java(error)),
            None => {}
        }
        let instance = match args.remove(0) {
            JavaValue::Object(Some(instance)) => instance,
            _ => return Err(ExecError::Npe),
        };
        let result = jvm
            .invoke_resolved_instance(&resolved, &instance, args.into_boxed_slice())
            .await
            .map_err(ExecError::Java)?;
        push_result(stack, result);
        Ok(())
    }

    async fn invoke_static(&self, jvm: &Jvm, stack: &mut Vec<JavaValue>, index: u16) -> core::result::Result<(), ExecError> {
        let site = &self.invokes[index as usize];
        let mut params = pop_params(stack, site);
        convert_params(&mut params, &site.kinds);
        let resolved = {
            let cached = site.cache.read();
            cached.as_ref().map(|(_, method)| method.clone())
        };
        let resolved = match resolved {
            Some(resolved) => resolved,
            None => {
                let resolved = jvm
                    .resolve_static_method(&site.class, &site.name, &site.descriptor)
                    .await
                    .map_err(ExecError::Java)?;
                *site.cache.write() = Some((site.class.to_string(), resolved.clone()));
                resolved
            }
        };
        match resolved.run_sync(jvm, &params) {
            Some(Ok(result)) => {
                push_result(stack, result);
                return Ok(());
            }
            Some(Err(error)) => return Err(ExecError::Java(error)),
            None => {}
        }
        let result = jvm
            .invoke_resolved_static(&resolved, params.into_boxed_slice())
            .await
            .map_err(ExecError::Java)?;
        push_result(stack, result);
        Ok(())
    }
}

fn pop_params(stack: &mut Vec<JavaValue>, site: &InvokeSite) -> Vec<JavaValue> {
    let count = site.argc as usize;
    let mut params = Vec::with_capacity(count);
    for _ in 0..count {
        params.push(stack.pop().unwrap_or(JavaValue::Void));
    }
    params.reverse();
    params
}

fn convert_params(params: &mut [JavaValue], kinds: &[u8]) {
    for (param, kind) in params.iter_mut().zip(kinds.iter()) {
        *param = match kind {
            1 => JavaValue::Boolean(as_int(param) & 1 != 0),
            2 => JavaValue::Byte(as_int(param) as i8),
            3 => JavaValue::Char(as_int(param) as u16),
            4 => JavaValue::Short(as_int(param) as i16),
            _ => continue,
        };
    }
}

fn push_result(stack: &mut Vec<JavaValue>, value: JavaValue) {
    if !matches!(value, JavaValue::Void) {
        stack.push(to_stack(value));
    }
}

fn pop_object(stack: &mut Vec<JavaValue>) -> core::result::Result<Box<dyn ClassInstance>, ExecError> {
    match stack.pop() {
        Some(JavaValue::Object(Some(instance))) => Ok(instance),
        _ => Err(ExecError::Npe),
    }
}

fn to_stack(value: JavaValue) -> JavaValue {
    match value {
        JavaValue::Boolean(value) => JavaValue::Int(i32::from(value)),
        JavaValue::Byte(value) => JavaValue::Int(i32::from(value)),
        JavaValue::Char(value) => JavaValue::Int(i32::from(value)),
        JavaValue::Short(value) => JavaValue::Int(i32::from(value)),
        other => other,
    }
}

fn to_field(descriptor: &str, value: JavaValue) -> JavaValue {
    match descriptor {
        "Z" => JavaValue::Boolean(as_int(&value) & 1 != 0),
        "B" => JavaValue::Byte(as_int(&value) as i8),
        "C" => JavaValue::Char(as_int(&value) as u16),
        "S" => JavaValue::Short(as_int(&value) as i16),
        _ => value,
    }
}

fn as_int(value: &JavaValue) -> i32 {
    match value {
        JavaValue::Int(value) => *value,
        JavaValue::Boolean(value) => i32::from(*value),
        JavaValue::Byte(value) => i32::from(*value),
        JavaValue::Char(value) => i32::from(*value),
        JavaValue::Short(value) => i32::from(*value),
        JavaValue::Long(value) => *value as i32,
        _ => 0,
    }
}

fn pop_int(stack: &mut Vec<JavaValue>) -> i32 {
    as_int(&stack.pop().unwrap_or(JavaValue::Int(0)))
}

fn pop_long(stack: &mut Vec<JavaValue>) -> i64 {
    match stack.pop() {
        Some(JavaValue::Long(value)) => value,
        Some(value) => i64::from(as_int(&value)),
        None => 0,
    }
}

fn pop_double(stack: &mut Vec<JavaValue>) -> f64 {
    match stack.pop() {
        Some(JavaValue::Double(value)) => value,
        Some(JavaValue::Float(value)) => f64::from(value),
        Some(value) => f64::from(as_int(&value)),
        None => 0.0,
    }
}

fn slot_pop_int(stack: &mut Vec<Slot>) -> i32 {
    stack.pop().map(|slot| slot.as_int()).unwrap_or(0)
}

fn slot_pop_long(stack: &mut Vec<Slot>) -> i64 {
    match stack.pop() {
        Some(Slot::J(value)) => value,
        Some(slot) => i64::from(slot.as_int()),
        None => 0,
    }
}

fn slot_pop_double(stack: &mut Vec<Slot>) -> f64 {
    match stack.pop() {
        Some(Slot::D(value)) => value,
        Some(slot) => f64::from(slot.as_int()),
        None => 0.0,
    }
}

fn slot_pop_obj(stack: &mut Vec<Slot>) -> core::result::Result<ClassInstanceImpl, ExecError> {
    match stack.pop() {
        Some(Slot::A(instance)) => Ok(instance),
        Some(Slot::X(JavaValue::Object(Some(obj)))) => ClassInstanceImpl::from_instance(obj.as_ref()).ok_or(ExecError::Npe),
        _ => Err(ExecError::Npe),
    }
}

fn slot_pop_any(stack: &mut Vec<Slot>) -> core::result::Result<Box<dyn ClassInstance>, ExecError> {
    match stack.pop() {
        Some(Slot::A(instance)) => Ok(Box::new(instance)),
        Some(Slot::X(JavaValue::Object(Some(obj)))) => Ok(obj),
        _ => Err(ExecError::Npe),
    }
}

fn slot_ibinop(stack: &mut Vec<Slot>, op: impl Fn(i32, i32) -> i32, pc: &mut usize) {
    let b = slot_pop_int(stack);
    let a = slot_pop_int(stack);
    stack.push(Slot::I(op(a, b)));
    *pc += 1;
}

fn slot_lbinop(stack: &mut Vec<Slot>, op: impl Fn(i64, i64) -> i64, pc: &mut usize) {
    let b = slot_pop_long(stack);
    let a = slot_pop_long(stack);
    stack.push(Slot::J(op(a, b)));
    *pc += 1;
}

fn slot_dbinop(stack: &mut Vec<Slot>, op: impl Fn(f64, f64) -> f64, pc: &mut usize) {
    let b = slot_pop_double(stack);
    let a = slot_pop_double(stack);
    stack.push(Slot::D(op(a, b)));
    *pc += 1;
}

fn slot_branch1(stack: &mut Vec<Slot>, pc: &mut usize, target: u32, pred: impl Fn(i32) -> bool) {
    let value = slot_pop_int(stack);
    if pred(value) {
        *pc = target as usize;
    } else {
        *pc += 1;
    }
}

fn slot_branch2(stack: &mut Vec<Slot>, pc: &mut usize, target: u32, pred: impl Fn(i32, i32) -> bool) {
    let b = slot_pop_int(stack);
    let a = slot_pop_int(stack);
    if pred(a, b) {
        *pc = target as usize;
    } else {
        *pc += 1;
    }
}

fn ibinop(stack: &mut Vec<JavaValue>, op: impl Fn(i32, i32) -> i32, pc: &mut usize) {
    let b = pop_int(stack);
    let a = pop_int(stack);
    stack.push(JavaValue::Int(op(a, b)));
    *pc += 1;
}

fn lbinop(stack: &mut Vec<JavaValue>, op: impl Fn(i64, i64) -> i64, pc: &mut usize) {
    let b = pop_long(stack);
    let a = pop_long(stack);
    stack.push(JavaValue::Long(op(a, b)));
    *pc += 1;
}

fn dbinop(stack: &mut Vec<JavaValue>, op: impl Fn(f64, f64) -> f64, pc: &mut usize) {
    let b = pop_double(stack);
    let a = pop_double(stack);
    stack.push(JavaValue::Double(op(a, b)));
    *pc += 1;
}

fn branch1(stack: &mut Vec<JavaValue>, pc: &mut usize, target: u32, pred: impl Fn(i32) -> bool) {
    let value = pop_int(stack);
    if pred(value) {
        *pc = target as usize;
    } else {
        *pc += 1;
    }
}

fn branch2(stack: &mut Vec<JavaValue>, pc: &mut usize, target: u32, pred: impl Fn(i32, i32) -> bool) {
    let b = pop_int(stack);
    let a = pop_int(stack);
    if pred(a, b) {
        *pc = target as usize;
    } else {
        *pc += 1;
    }
}

fn dcmp(a: f64, b: f64, nan: i32) -> i32 {
    if a.is_nan() || b.is_nan() {
        nan
    } else if a > b {
        1
    } else if a == b {
        0
    } else {
        -1
    }
}

fn return_int(return_type: &JavaType, value: i32) -> JavaValue {
    match return_type {
        JavaType::Boolean => JavaValue::Boolean(value & 1 != 0),
        JavaType::Char => JavaValue::Char(value as u16),
        JavaType::Byte => JavaValue::Byte(value as i8),
        JavaType::Short => JavaValue::Short(value as i16),
        _ => JavaValue::Int(value),
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

fn java_ldiv(a: i64, b: i64) -> core::result::Result<i64, ExecError> {
    if b == 0 {
        return Err(ExecError::DivByZero);
    }
    if a == i64::MIN && b == -1 {
        return Ok(a);
    }
    Ok(a / b)
}

fn java_lrem(a: i64, b: i64) -> core::result::Result<i64, ExecError> {
    if b == 0 {
        return Err(ExecError::DivByZero);
    }
    if a == i64::MIN && b == -1 {
        return Ok(0);
    }
    Ok(a % b)
}
