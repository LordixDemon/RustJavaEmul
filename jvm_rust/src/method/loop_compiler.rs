use alloc::{boxed::Box, vec::Vec};

use jvm::{JavaType, JavaValue, Jvm, JvmCallback, Result};

use super::general::{FieldSite, Op};
use super::gfx::GfxSession;
use crate::class_instance::ClassInstanceImpl;
use crate::profile;

const REGS: usize = 64;

#[derive(Clone, Copy)]
pub(super) enum Tok {
    Load(u16),
    Iconst(i32),
    IfIcmpGe(u32),
    Iinc(u16, i16),
    Goto(u32),
    Store(u16),
    Jump,
    Other,
}

pub(super) struct CountedLoop {
    pub header: usize,
    pub body_start: usize,
    pub body_end: usize,
    pub after: usize,
    pub i_local: u16,
    pub limit_local: Option<u16>,
    pub limit_imm: Option<i32>,
    pub step: i16,
}

pub(super) fn find_counted_loop(len: usize, tok: impl Fn(usize) -> Tok) -> Option<CountedLoop> {
    if len < 5 {
        return None;
    }
    for header in 0..len - 4 {
        let Tok::Load(i_local) = tok(header) else {
            continue;
        };
        let (limit_local, limit_imm) = match tok(header + 1) {
            Tok::Load(index) => (Some(index), None),
            Tok::Iconst(value) => (None, Some(value)),
            _ => continue,
        };
        let Tok::IfIcmpGe(end) = tok(header + 2) else {
            continue;
        };
        let after = end as usize;
        if after < header + 5 || after > len {
            continue;
        }
        let Tok::Goto(back) = tok(after - 1) else {
            continue;
        };
        if back as usize != header {
            continue;
        }
        let Tok::Iinc(inc_local, step) = tok(after - 2) else {
            continue;
        };
        if inc_local != i_local || step == 0 {
            continue;
        }
        let body_start = header + 3;
        let body_end = after - 2;
        let mut ok = true;
        for index in body_start..body_end {
            match tok(index) {
                Tok::IfIcmpGe(_) | Tok::Goto(_) | Tok::Jump => ok = false,
                Tok::Store(slot) if slot == i_local || Some(slot) == limit_local => ok = false,
                Tok::Iinc(slot, _) if slot == i_local => ok = false,
                _ => {}
            }
        }
        for index in 0..header {
            if matches!(tok(index), Tok::IfIcmpGe(_) | Tok::Goto(_) | Tok::Jump) {
                ok = false;
            }
        }
        for index in after..len {
            match tok(index) {
                Tok::IfIcmpGe(_) | Tok::Goto(_) | Tok::Jump => ok = false,
                Tok::Other | Tok::Load(_) | Tok::Iconst(_) | Tok::Iinc(_, _) | Tok::Store(_) => {}
            }
        }
        if ok {
            return Some(CountedLoop {
                header,
                body_start,
                body_end,
                after,
                i_local,
                limit_local,
                limit_imm,
                step,
            });
        }
    }
    None
}

#[derive(Clone, Copy)]
enum Src {
    R(u8),
    I(i32),
}

#[derive(Clone, Copy)]
enum Rop {
    Mov(u8, Src),
    Add(u8, Src, Src),
    Sub(u8, Src, Src),
    Mul(u8, Src, Src),
    Div(u8, Src, Src),
    Rem(u8, Src, Src),
    And(u8, Src, Src),
    Or(u8, Src, Src),
    Xor(u8, Src, Src),
    Shl(u8, Src, Src),
    Shr(u8, Src, Src),
    Ushr(u8, Src, Src),
    Neg(u8, Src),
    I2b(u8, Src),
    I2c(u8, Src),
    I2s(u8, Src),
    SetColor(Src),
    FillRect(Src, Src, Src, Src),
    DrawLine(Src, Src, Src, Src),
    FillTriangle(Src, Src, Src, Src, Src, Src),
}

enum ExecError {
    DivByZero,
    Npe,
}

struct CompiledLoop {
    prologue: Box<[Rop]>,
    body: Box<[Rop]>,
    epilogue: Box<[Rop]>,
    i_local: u8,
    limit_local: Option<u8>,
    limit_imm: Option<i32>,
    step: i32,
    return_reg: Option<u8>,
    return_type: JavaType,
    gfx_field: Option<(ArcName, ArcName)>,
    int_fields: Box<[(u8, ArcName)]>,
}

type ArcName = alloc::sync::Arc<str>;

#[async_trait::async_trait]
impl JvmCallback for CompiledLoop {
    async fn call(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue> {
        profile::int_compiler_call();
        match self.execute(&args) {
            Ok(value) => Ok(value),
            Err(ExecError::DivByZero) => Err(jvm.exception("java/lang/ArithmeticException", "/ by zero").await),
            Err(ExecError::Npe) => Err(jvm.exception("java/lang/NullPointerException", "").await),
        }
    }

    fn call_sync(&self, _jvm: &Jvm, args: &[JavaValue]) -> Option<Result<JavaValue>> {
        profile::int_compiler_call();
        match self.execute(args) {
            Ok(value) => Some(Ok(value)),
            Err(_) => None,
        }
    }
}

impl CompiledLoop {
    fn execute(&self, args: &[JavaValue]) -> core::result::Result<JavaValue, ExecError> {
        let mut regs = [0i32; REGS];
        load_int_args(&mut regs, args);
        let this = args.first().and_then(|value| match value {
            JavaValue::Object(Some(obj)) => ClassInstanceImpl::from_instance(obj.as_ref()),
            _ => None,
        });
        for (reg, name) in self.int_fields.iter() {
            let Some(this) = this.as_ref() else {
                return Err(ExecError::Npe);
            };
            regs[*reg as usize] = this.named_i32(name);
        }
        let graphics = match self.gfx_field.as_ref() {
            Some((name, desc)) => {
                let this = this.as_ref().ok_or(ExecError::Npe)?;
                Some(this.named_instance(name, desc).ok_or(ExecError::Npe)?)
            }
            None => None,
        };
        let mut session = match graphics.as_ref() {
            Some(graphics) => GfxSession::warm(graphics),
            None => None,
        };
        run_rops(&self.prologue, &mut regs, session.as_mut())?;
        let mut i = regs[self.i_local as usize];
        let limit = if let Some(reg) = self.limit_local {
            regs[reg as usize]
        } else {
            self.limit_imm.unwrap_or(0)
        };
        let step = self.step;
        if let Some(session) = session.as_mut() {
            while i < limit {
                regs[self.i_local as usize] = i;
                run_rops(&self.body, &mut regs, Some(session))?;
                i = i.wrapping_add(step);
            }
            regs[self.i_local as usize] = i;
        } else if !run_int_fast(&self.body, &mut regs, self.i_local, i, limit, step) {
            while i < limit {
                regs[self.i_local as usize] = i;
                run_int_rops(&self.body, &mut regs)?;
                i = i.wrapping_add(step);
            }
            regs[self.i_local as usize] = i;
        }
        run_rops(&self.epilogue, &mut regs, session.as_mut())?;
        if let (Some(session), Some(graphics)) = (session, graphics.as_ref()) {
            session.finish(graphics);
        }
        let value = self.return_reg.map_or(0, |reg| regs[reg as usize]);
        Ok(match self.return_type {
            JavaType::Boolean => JavaValue::Boolean(value & 1 != 0),
            JavaType::Char => JavaValue::Char(value as u16),
            JavaType::Byte => JavaValue::Byte(value as i8),
            JavaType::Short => JavaValue::Short(value as i16),
            JavaType::Void => JavaValue::Void,
            _ => JavaValue::Int(value),
        })
    }
}

fn load_int_args(regs: &mut [i32], args: &[JavaValue]) {
    let mut index = 0usize;
    for arg in args {
        if index >= regs.len() {
            break;
        }
        match arg {
            JavaValue::Boolean(value) => {
                regs[index] = i32::from(*value);
                index += 1;
            }
            JavaValue::Byte(value) => {
                regs[index] = i32::from(*value);
                index += 1;
            }
            JavaValue::Char(value) => {
                regs[index] = i32::from(*value);
                index += 1;
            }
            JavaValue::Short(value) => {
                regs[index] = i32::from(*value);
                index += 1;
            }
            JavaValue::Int(value) => {
                regs[index] = *value;
                index += 1;
            }
            JavaValue::Object(_) => index += 1,
            JavaValue::Long(_) | JavaValue::Double(_) => index += 2,
            JavaValue::Float(_) | JavaValue::Void => index += 1,
        }
    }
}

fn run_rops(ops: &[Rop], regs: &mut [i32], mut session: Option<&mut GfxSession>) -> core::result::Result<(), ExecError> {
    for op in ops {
        match *op {
            Rop::Mov(dst, src) => regs[dst as usize] = val(src, regs),
            Rop::Add(dst, a, b) => regs[dst as usize] = val(a, regs).wrapping_add(val(b, regs)),
            Rop::Sub(dst, a, b) => regs[dst as usize] = val(a, regs).wrapping_sub(val(b, regs)),
            Rop::Mul(dst, a, b) => regs[dst as usize] = val(a, regs).wrapping_mul(val(b, regs)),
            Rop::Div(dst, a, b) => regs[dst as usize] = java_idiv(val(a, regs), val(b, regs))?,
            Rop::Rem(dst, a, b) => regs[dst as usize] = java_irem(val(a, regs), val(b, regs))?,
            Rop::And(dst, a, b) => regs[dst as usize] = val(a, regs) & val(b, regs),
            Rop::Or(dst, a, b) => regs[dst as usize] = val(a, regs) | val(b, regs),
            Rop::Xor(dst, a, b) => regs[dst as usize] = val(a, regs) ^ val(b, regs),
            Rop::Shl(dst, a, b) => regs[dst as usize] = val(a, regs).wrapping_shl(val(b, regs) as u32),
            Rop::Shr(dst, a, b) => regs[dst as usize] = val(a, regs).wrapping_shr(val(b, regs) as u32),
            Rop::Ushr(dst, a, b) => regs[dst as usize] = (val(a, regs) as u32).wrapping_shr(val(b, regs) as u32) as i32,
            Rop::Neg(dst, src) => regs[dst as usize] = val(src, regs).wrapping_neg(),
            Rop::I2b(dst, src) => regs[dst as usize] = val(src, regs) as i8 as i32,
            Rop::I2c(dst, src) => regs[dst as usize] = val(src, regs) as u16 as i32,
            Rop::I2s(dst, src) => regs[dst as usize] = val(src, regs) as i16 as i32,
            Rop::SetColor(rgb) => session.as_mut().ok_or(ExecError::Npe)?.set_color(val(rgb, regs)),
            Rop::FillRect(x, y, w, h) => session.as_mut().ok_or(ExecError::Npe)?.fill_rect(val(x, regs), val(y, regs), val(w, regs), val(h, regs)),
            Rop::DrawLine(x1, y1, x2, y2) => {
                session
                    .as_mut()
                    .ok_or(ExecError::Npe)?
                    .draw_line(val(x1, regs), val(y1, regs), val(x2, regs), val(y2, regs));
            }
            Rop::FillTriangle(x1, y1, x2, y2, x3, y3) => {
                session.as_mut().ok_or(ExecError::Npe)?.fill_triangle(
                    val(x1, regs),
                    val(y1, regs),
                    val(x2, regs),
                    val(y2, regs),
                    val(x3, regs),
                    val(y3, regs),
                );
            }
        }
    }
    Ok(())
}

#[inline(always)]
fn val(src: Src, regs: &[i32]) -> i32 {
    match src {
        Src::R(index) => regs[index as usize],
        Src::I(value) => value,
    }
}

fn run_int_rops(ops: &[Rop], regs: &mut [i32]) -> core::result::Result<(), ExecError> {
    run_rops(ops, regs, None)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AccOp {
    AddI,
    SubI,
    XorI,
    AndI,
    OrI,
    AddImm(i32),
    XorImm(i32),
    MulImm(i32),
    XorIMul(i32),
    AddIMul(i32),
    RotL(u32),
    ShlImm(u32),
    UshrImm(u32),
    Neg,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sym {
    Acc,
    I,
    Imm(i32),
    IMul(i32),
    AccShl(u32),
    AccUshr(u32),
    Other,
}

fn run_int_fast(body: &[Rop], regs: &mut [i32], i_local: u8, start: i32, limit: i32, step: i32) -> bool {
    let Some(acc) = find_accumulator(body, i_local) else {
        return false;
    };
    let Some(ops) = lower_acc_ops(body, i_local, acc) else {
        return false;
    };
    let a = regs[acc as usize];
    let (a, i) = run_acc_ops(&ops, a, start, limit, step);
    regs[acc as usize] = a;
    regs[i_local as usize] = i;
    true
}

fn find_accumulator(body: &[Rop], i_local: u8) -> Option<u8> {
    let mut assigned = [false; REGS];
    let mut live_in = [false; REGS];
    for op in body {
        for src in rop_srcs(*op).into_iter().flatten() {
            if let Src::R(reg) = src {
                let index = reg as usize;
                if index < REGS && !assigned[index] && reg != i_local {
                    live_in[index] = true;
                }
            }
        }
        if let Some(dst) = rop_dst(*op) {
            let index = dst as usize;
            if index < REGS {
                assigned[index] = true;
            }
        }
    }
    let mut acc = None;
    for index in 0..REGS {
        if live_in[index] && assigned[index] {
            if acc.is_some() {
                return None;
            }
            acc = Some(index as u8);
        }
    }
    acc
}

fn rop_dst(op: Rop) -> Option<u8> {
    match op {
        Rop::Mov(dst, _)
        | Rop::Add(dst, _, _)
        | Rop::Sub(dst, _, _)
        | Rop::Mul(dst, _, _)
        | Rop::Div(dst, _, _)
        | Rop::Rem(dst, _, _)
        | Rop::And(dst, _, _)
        | Rop::Or(dst, _, _)
        | Rop::Xor(dst, _, _)
        | Rop::Shl(dst, _, _)
        | Rop::Shr(dst, _, _)
        | Rop::Ushr(dst, _, _)
        | Rop::Neg(dst, _)
        | Rop::I2b(dst, _)
        | Rop::I2c(dst, _)
        | Rop::I2s(dst, _) => Some(dst),
        Rop::SetColor(_) | Rop::FillRect(_, _, _, _) | Rop::DrawLine(_, _, _, _) | Rop::FillTriangle(_, _, _, _, _, _) => None,
    }
}

fn rop_srcs(op: Rop) -> [Option<Src>; 6] {
    match op {
        Rop::Mov(_, src) | Rop::Neg(_, src) | Rop::I2b(_, src) | Rop::I2c(_, src) | Rop::I2s(_, src) | Rop::SetColor(src) => {
            [Some(src), None, None, None, None, None]
        }
        Rop::Add(_, a, b)
        | Rop::Sub(_, a, b)
        | Rop::Mul(_, a, b)
        | Rop::Div(_, a, b)
        | Rop::Rem(_, a, b)
        | Rop::And(_, a, b)
        | Rop::Or(_, a, b)
        | Rop::Xor(_, a, b)
        | Rop::Shl(_, a, b)
        | Rop::Shr(_, a, b)
        | Rop::Ushr(_, a, b) => [Some(a), Some(b), None, None, None, None],
        Rop::FillRect(a, b, c, d) | Rop::DrawLine(a, b, c, d) => [Some(a), Some(b), Some(c), Some(d), None, None],
        Rop::FillTriangle(a, b, c, d, e, f) => [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)],
    }
}

fn lower_acc_ops(body: &[Rop], i_local: u8, acc: u8) -> Option<Vec<AccOp>> {
    let mut env = [Sym::Other; REGS];
    env[acc as usize] = Sym::Acc;
    env[i_local as usize] = Sym::I;
    let mut ops = Vec::new();
    for op in body {
        match *op {
            Rop::Mov(dst, src) => {
                let value = src_sym(src, &env, i_local, acc);
                if dst == acc {
                    match value {
                        Sym::Acc => {}
                        _ => return None,
                    }
                }
                env[dst as usize] = value;
            }
            Rop::Add(dst, a, b) => {
                let out = match (src_sym(a, &env, i_local, acc), src_sym(b, &env, i_local, acc)) {
                    (Sym::Acc, Sym::I) | (Sym::I, Sym::Acc) if dst == acc => {
                        ops.push(AccOp::AddI);
                        Sym::Acc
                    }
                    (Sym::Acc, Sym::Imm(value)) | (Sym::Imm(value), Sym::Acc) if dst == acc => {
                        ops.push(AccOp::AddImm(value));
                        Sym::Acc
                    }
                    (Sym::Acc, Sym::IMul(value)) | (Sym::IMul(value), Sym::Acc) if dst == acc => {
                        ops.push(AccOp::AddIMul(value));
                        Sym::Acc
                    }
                    (Sym::I, Sym::Imm(value)) | (Sym::Imm(value), Sym::I) => Sym::IMul(value),
                    _ => Sym::Other,
                };
                if dst == acc && out != Sym::Acc {
                    return None;
                }
                env[dst as usize] = out;
            }
            Rop::Sub(dst, a, b) => {
                let out = match (src_sym(a, &env, i_local, acc), src_sym(b, &env, i_local, acc)) {
                    (Sym::Acc, Sym::I) if dst == acc => {
                        ops.push(AccOp::SubI);
                        Sym::Acc
                    }
                    _ => Sym::Other,
                };
                if dst == acc && out != Sym::Acc {
                    return None;
                }
                env[dst as usize] = out;
            }
            Rop::Mul(dst, a, b) => {
                let out = match (src_sym(a, &env, i_local, acc), src_sym(b, &env, i_local, acc)) {
                    (Sym::I, Sym::Imm(value)) | (Sym::Imm(value), Sym::I) => Sym::IMul(value),
                    (Sym::Acc, Sym::Imm(value)) | (Sym::Imm(value), Sym::Acc) if dst == acc => {
                        ops.push(AccOp::MulImm(value));
                        Sym::Acc
                    }
                    _ => Sym::Other,
                };
                if dst == acc && out != Sym::Acc {
                    return None;
                }
                env[dst as usize] = out;
            }
            Rop::Xor(dst, a, b) => {
                let out = match (src_sym(a, &env, i_local, acc), src_sym(b, &env, i_local, acc)) {
                    (Sym::Acc, Sym::I) | (Sym::I, Sym::Acc) if dst == acc => {
                        ops.push(AccOp::XorI);
                        Sym::Acc
                    }
                    (Sym::Acc, Sym::IMul(value)) | (Sym::IMul(value), Sym::Acc) if dst == acc => {
                        ops.push(AccOp::XorIMul(value));
                        Sym::Acc
                    }
                    (Sym::Acc, Sym::Imm(value)) | (Sym::Imm(value), Sym::Acc) if dst == acc => {
                        ops.push(AccOp::XorImm(value));
                        Sym::Acc
                    }
                    _ => Sym::Other,
                };
                if dst == acc && out != Sym::Acc {
                    return None;
                }
                env[dst as usize] = out;
            }
            Rop::And(dst, a, b) => {
                let out = match (src_sym(a, &env, i_local, acc), src_sym(b, &env, i_local, acc)) {
                    (Sym::Acc, Sym::I) | (Sym::I, Sym::Acc) if dst == acc => {
                        ops.push(AccOp::AndI);
                        Sym::Acc
                    }
                    _ => Sym::Other,
                };
                if dst == acc && out != Sym::Acc {
                    return None;
                }
                env[dst as usize] = out;
            }
            Rop::Or(dst, a, b) => {
                let sa = src_sym(a, &env, i_local, acc);
                let sb = src_sym(b, &env, i_local, acc);
                let out = match (sa, sb) {
                    (Sym::AccShl(left), Sym::AccUshr(right)) | (Sym::AccUshr(right), Sym::AccShl(left))
                        if dst == acc && left < 32 && right == 32 - left =>
                    {
                        ops.push(AccOp::RotL(left));
                        Sym::Acc
                    }
                    (Sym::Acc, Sym::I) | (Sym::I, Sym::Acc) if dst == acc => {
                        ops.push(AccOp::OrI);
                        Sym::Acc
                    }
                    _ => Sym::Other,
                };
                if dst == acc && out != Sym::Acc {
                    return None;
                }
                env[dst as usize] = out;
            }
            Rop::Shl(dst, a, b) => {
                let sa = src_sym(a, &env, i_local, acc);
                let out = match (sa, src_sym(b, &env, i_local, acc)) {
                    (Sym::Acc, Sym::Imm(value)) if value >= 0 => {
                        if dst == acc {
                            ops.push(AccOp::ShlImm(value as u32));
                            Sym::Acc
                        } else {
                            Sym::AccShl(value as u32)
                        }
                    }
                    _ => Sym::Other,
                };
                if dst == acc && out != Sym::Acc {
                    return None;
                }
                env[dst as usize] = out;
            }
            Rop::Ushr(dst, a, b) => {
                let sa = src_sym(a, &env, i_local, acc);
                let out = match (sa, src_sym(b, &env, i_local, acc)) {
                    (Sym::Acc, Sym::Imm(value)) if value >= 0 => {
                        if dst == acc {
                            ops.push(AccOp::UshrImm(value as u32));
                            Sym::Acc
                        } else {
                            Sym::AccUshr(value as u32)
                        }
                    }
                    _ => Sym::Other,
                };
                if dst == acc && out != Sym::Acc {
                    return None;
                }
                env[dst as usize] = out;
            }
            Rop::Neg(dst, src) if dst == acc && src_sym(src, &env, i_local, acc) == Sym::Acc => {
                ops.push(AccOp::Neg);
                env[dst as usize] = Sym::Acc;
            }
            Rop::Div(_, _, _) | Rop::Rem(_, _, _) | Rop::Shr(_, _, _) | Rop::I2b(_, _) | Rop::I2c(_, _) | Rop::I2s(_, _) => return None,
            Rop::SetColor(_) | Rop::FillRect(_, _, _, _) | Rop::DrawLine(_, _, _, _) | Rop::FillTriangle(_, _, _, _, _, _) => return None,
            _ => return None,
        }
    }
    if ops.is_empty() {
        return None;
    }
    Some(ops)
}

fn src_sym(src: Src, env: &[Sym; REGS], i_local: u8, acc: u8) -> Sym {
    match src {
        Src::I(value) => Sym::Imm(value),
        Src::R(reg) if reg == i_local => Sym::I,
        Src::R(reg) if reg == acc => env[reg as usize],
        Src::R(reg) if (reg as usize) < REGS => env[reg as usize],
        Src::R(_) => Sym::Other,
    }
}

fn run_acc_ops(ops: &[AccOp], a: i32, start: i32, limit: i32, step: i32) -> (i32, i32) {
    if matches!(ops, [AccOp::AddI, AccOp::XorIMul(3), AccOp::RotL(1)]) {
        return kernel_add_xor3_rotl1(a, start, limit, step);
    }
    if matches!(ops, [AccOp::AddI]) {
        return kernel_add_i(a, start, limit, step);
    }
    let mut a = a;
    let mut i = start;
    while i < limit {
        for op in ops {
            a = apply_acc(*op, a, i);
        }
        i = i.wrapping_add(step);
    }
    (a, i)
}

#[inline(always)]
fn apply_acc(op: AccOp, a: i32, i: i32) -> i32 {
    match op {
        AccOp::AddI => a.wrapping_add(i),
        AccOp::SubI => a.wrapping_sub(i),
        AccOp::XorI => a ^ i,
        AccOp::AndI => a & i,
        AccOp::OrI => a | i,
        AccOp::AddImm(value) => a.wrapping_add(value),
        AccOp::XorImm(value) => a ^ value,
        AccOp::MulImm(value) => a.wrapping_mul(value),
        AccOp::XorIMul(value) => a ^ i.wrapping_mul(value),
        AccOp::AddIMul(value) => a.wrapping_add(i.wrapping_mul(value)),
        AccOp::RotL(amount) => a.rotate_left(amount),
        AccOp::ShlImm(amount) => a.wrapping_shl(amount),
        AccOp::UshrImm(amount) => (a as u32).wrapping_shr(amount) as i32,
        AccOp::Neg => a.wrapping_neg(),
    }
}

#[inline(never)]
fn kernel_add_xor3_rotl1(mut a: i32, start: i32, limit: i32, step: i32) -> (i32, i32) {
    if step == 1 && start >= 0 && limit >= start {
        for i in start as u32..limit as u32 {
            let i = i as i32;
            a = a.wrapping_add(i);
            a ^= i.wrapping_mul(3);
            a = a.rotate_left(1);
        }
        return (a, limit);
    }
    let mut i = start;
    while i < limit {
        a = a.wrapping_add(i);
        a ^= i.wrapping_mul(3);
        a = a.rotate_left(1);
        i = i.wrapping_add(step);
    }
    (a, i)
}

fn kernel_add_i(a: i32, start: i32, limit: i32, step: i32) -> (i32, i32) {
    if step == 1 && start >= 0 && limit >= start {
        let n = (limit as i64) - (start as i64);
        let sum = n * (start as i64 + (limit as i64 - 1)) / 2;
        return (a.wrapping_add(sum as i32), limit);
    }
    let mut a = a;
    let mut i = start;
    while i < limit {
        a = a.wrapping_add(i);
        i = i.wrapping_add(step);
    }
    (a, i)
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

pub(super) fn try_gfx(ops: &[Op], fields: &[FieldSite], max_locals: usize, return_type: JavaType) -> Option<Box<dyn JvmCallback>> {
    if max_locals >= REGS {
        return None;
    }
    let counted = find_counted_loop(ops.len(), |index| op_tok(ops[index]))?;
    let mut lowerer = Lowerer::new(max_locals, fields);
    let mut prologue = lowerer.lower_ops(&ops[..counted.header])?;
    lowerer.in_body = true;
    lowerer.assigned.clear();
    let body = lowerer.lower_ops(&ops[counted.body_start..counted.body_end])?;
    prologue.append(&mut lowerer.hoist);
    lowerer.in_body = false;
    let (epilogue, return_reg) = lowerer.lower_epilogue(&ops[counted.after..])?;
    if counted.i_local as usize >= REGS {
        return None;
    }
    Some(Box::new(CompiledLoop {
        prologue: prologue.into_boxed_slice(),
        body: body.into_boxed_slice(),
        epilogue: epilogue.into_boxed_slice(),
        i_local: counted.i_local as u8,
        limit_local: counted.limit_local.map(|index| index as u8),
        limit_imm: counted.limit_imm,
        step: i32::from(counted.step),
        return_reg,
        return_type,
        gfx_field: lowerer.gfx_field,
        int_fields: lowerer.int_fields.into_boxed_slice(),
    }))
}

pub(super) fn try_int(ops: &[IntTok], max_locals: usize, return_type: JavaType) -> Option<Box<dyn JvmCallback>> {
    if max_locals >= REGS {
        return None;
    }
    let counted = find_counted_loop(ops.len(), |index| ops[index].tok())?;
    let mut lowerer = Lowerer::new(max_locals, &[]);
    let mut prologue = lowerer.lower_int(&ops[..counted.header])?;
    lowerer.in_body = true;
    lowerer.assigned.clear();
    let body = lowerer.lower_int(&ops[counted.body_start..counted.body_end])?;
    prologue.append(&mut lowerer.hoist);
    lowerer.in_body = false;
    let (epilogue, return_reg) = lowerer.lower_int_epilogue(&ops[counted.after..])?;
    Some(Box::new(CompiledLoop {
        prologue: prologue.into_boxed_slice(),
        body: body.into_boxed_slice(),
        epilogue: epilogue.into_boxed_slice(),
        i_local: counted.i_local as u8,
        limit_local: counted.limit_local.map(|index| index as u8),
        limit_imm: counted.limit_imm,
        step: i32::from(counted.step),
        return_reg,
        return_type,
        gfx_field: None,
        int_fields: Box::new([]),
    }))
}

#[derive(Clone, Copy)]
pub(super) enum IntTok {
    Load(u16),
    Iconst(i32),
    Store(u16),
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
    IfIcmpGe(u32),
    Goto(u32),
    Ireturn,
    Return,
    Jump,
    Other,
}

impl IntTok {
    fn tok(self) -> Tok {
        match self {
            IntTok::Load(index) => Tok::Load(index),
            IntTok::Iconst(value) => Tok::Iconst(value),
            IntTok::Store(index) => Tok::Store(index),
            IntTok::Iinc(index, amount) => Tok::Iinc(index, amount),
            IntTok::IfIcmpGe(target) => Tok::IfIcmpGe(target),
            IntTok::Goto(target) => Tok::Goto(target),
            IntTok::Jump => Tok::Jump,
            _ => Tok::Other,
        }
    }
}

#[derive(Clone, Copy)]
enum V {
    Int(Src),
    Obj,
}

struct Lowerer<'a> {
    fields: &'a [FieldSite],
    next_temp: u8,
    gfx_field: Option<(ArcName, ArcName)>,
    int_fields: Vec<(u8, ArcName)>,
    int_field_regs: Vec<(u16, u8)>,
    invariant: Vec<u8>,
    hoist: Vec<Rop>,
    assigned: Vec<u8>,
    in_body: bool,
}

impl<'a> Lowerer<'a> {
    fn new(max_locals: usize, fields: &'a [FieldSite]) -> Self {
        Self {
            fields,
            next_temp: max_locals.min(REGS - 1) as u8,
            gfx_field: None,
            int_fields: Vec::new(),
            int_field_regs: Vec::new(),
            invariant: Vec::new(),
            hoist: Vec::new(),
            assigned: Vec::new(),
            in_body: false,
        }
    }

    fn is_invariant(&self, src: Src) -> bool {
        match src {
            Src::I(_) => true,
            Src::R(reg) => self.invariant.contains(&reg) && !self.assigned.contains(&reg),
        }
    }

    fn mark_assigned(&mut self, dst: u8) {
        if !self.assigned.contains(&dst) {
            self.assigned.push(dst);
        }
    }

    fn alloc(&mut self) -> Option<u8> {
        let reg = self.next_temp;
        if reg as usize + 1 >= REGS {
            return None;
        }
        self.next_temp += 1;
        Some(reg)
    }

    fn int_field_reg(&mut self, field: u16) -> Option<u8> {
        if let Some((_, reg)) = self.int_field_regs.iter().copied().find(|(index, _)| *index == field) {
            return Some(reg);
        }
        let site = self.fields.get(field as usize)?;
        if !matches!(site.descriptor.as_ref(), "I" | "Z" | "B" | "C" | "S") {
            return None;
        }
        let reg = self.alloc()?;
        self.int_fields.push((reg, site.name.clone()));
        self.int_field_regs.push((field, reg));
        if !self.invariant.contains(&reg) {
            self.invariant.push(reg);
        }
        Some(reg)
    }

    fn note_gfx(&mut self, field: u16) -> Option<()> {
        let site = self.fields.get(field as usize)?;
        if !site.descriptor.as_ref().starts_with('L') {
            return None;
        }
        match &self.gfx_field {
            Some((name, desc)) if **name == *site.name && **desc == *site.descriptor => Some(()),
            Some(_) => None,
            None => {
                self.gfx_field = Some((site.name.clone(), site.descriptor.clone()));
                Some(())
            }
        }
    }

    fn binop(&mut self, stack: &mut Vec<V>, rops: &mut Vec<Rop>, op: fn(u8, Src, Src) -> Rop) -> Option<()> {
        let b = pop_int(stack)?;
        let a = pop_int(stack)?;
        let dst = self.alloc()?;
        if self.in_body && self.is_invariant(a) && self.is_invariant(b) {
            self.hoist.push(op(dst, a, b));
            self.invariant.push(dst);
        } else {
            rops.push(op(dst, a, b));
            self.mark_assigned(dst);
        }
        stack.push(V::Int(Src::R(dst)));
        Some(())
    }

    fn unop(&mut self, stack: &mut Vec<V>, rops: &mut Vec<Rop>, op: fn(u8, Src) -> Rop) -> Option<()> {
        let src = pop_int(stack)?;
        let dst = self.alloc()?;
        rops.push(op(dst, src));
        self.mark_assigned(dst);
        stack.push(V::Int(Src::R(dst)));
        Some(())
    }

    fn store(&mut self, stack: &mut Vec<V>, rops: &mut Vec<Rop>, index: u16) -> Option<()> {
        let src = pop_int(stack)?;
        let dst = u8::try_from(index).ok()?;
        match src {
            Src::R(reg) if reg == dst => {}
            Src::R(temp) if rewrite_dst(rops.last_mut(), temp, dst) => self.mark_assigned(dst),
            src => {
                rops.push(Rop::Mov(dst, src));
                self.mark_assigned(dst);
            }
        }
        Some(())
    }

    fn lower_ops(&mut self, ops: &[Op]) -> Option<Vec<Rop>> {
        let mut stack = Vec::new();
        let mut rops = Vec::new();
        for op in ops {
            match *op {
                Op::Nop | Op::Pop2 => {}
                Op::Pop => {
                    stack.pop();
                }
                Op::Dup => {
                    let top = *stack.last()?;
                    stack.push(top);
                }
                Op::Iconst(value) => stack.push(V::Int(Src::I(value))),
                Op::Load(index) => stack.push(V::Int(Src::R(u8::try_from(index).ok()?))),
                Op::Store(index) | Op::StoreWide(index) => self.store(&mut stack, &mut rops, index)?,
                Op::Iinc(index, amount) => {
                    let dst = u8::try_from(index).ok()?;
                    rops.push(Rop::Add(dst, Src::R(dst), Src::I(i32::from(amount))));
                    self.mark_assigned(dst);
                }
                Op::Iadd => self.binop(&mut stack, &mut rops, Rop::Add)?,
                Op::Isub => self.binop(&mut stack, &mut rops, Rop::Sub)?,
                Op::Imul => self.binop(&mut stack, &mut rops, Rop::Mul)?,
                Op::Idiv => self.binop(&mut stack, &mut rops, Rop::Div)?,
                Op::Irem => self.binop(&mut stack, &mut rops, Rop::Rem)?,
                Op::Iand => self.binop(&mut stack, &mut rops, Rop::And)?,
                Op::Ior => self.binop(&mut stack, &mut rops, Rop::Or)?,
                Op::Ixor => self.binop(&mut stack, &mut rops, Rop::Xor)?,
                Op::Ishl => self.binop(&mut stack, &mut rops, Rop::Shl)?,
                Op::Ishr => self.binop(&mut stack, &mut rops, Rop::Shr)?,
                Op::Iushr => self.binop(&mut stack, &mut rops, Rop::Ushr)?,
                Op::Ineg => self.unop(&mut stack, &mut rops, Rop::Neg)?,
                Op::I2b => self.unop(&mut stack, &mut rops, Rop::I2b)?,
                Op::I2c => self.unop(&mut stack, &mut rops, Rop::I2c)?,
                Op::I2s => self.unop(&mut stack, &mut rops, Rop::I2s)?,
                Op::GetThisField(index) => {
                    if let Some(reg) = self.int_field_reg(index) {
                        stack.push(V::Int(Src::R(reg)));
                    } else {
                        self.note_gfx(index)?;
                        stack.push(V::Obj);
                    }
                }
                Op::GfxSetColor(_) => {
                    let rgb = pop_int(&mut stack)?;
                    pop_obj(&mut stack)?;
                    rops.push(Rop::SetColor(rgb));
                }
                Op::GfxFillRect(_) => {
                    let height = pop_int(&mut stack)?;
                    let width = pop_int(&mut stack)?;
                    let y = pop_int(&mut stack)?;
                    let x = pop_int(&mut stack)?;
                    pop_obj(&mut stack)?;
                    rops.push(Rop::FillRect(x, y, width, height));
                }
                Op::GfxDrawLine(_) => {
                    let y2 = pop_int(&mut stack)?;
                    let x2 = pop_int(&mut stack)?;
                    let y1 = pop_int(&mut stack)?;
                    let x1 = pop_int(&mut stack)?;
                    pop_obj(&mut stack)?;
                    rops.push(Rop::DrawLine(x1, y1, x2, y2));
                }
                Op::GfxFillTriangle(_) => {
                    let y3 = pop_int(&mut stack)?;
                    let x3 = pop_int(&mut stack)?;
                    let y2 = pop_int(&mut stack)?;
                    let x2 = pop_int(&mut stack)?;
                    let y1 = pop_int(&mut stack)?;
                    let x1 = pop_int(&mut stack)?;
                    pop_obj(&mut stack)?;
                    rops.push(Rop::FillTriangle(x1, y1, x2, y2, x3, y3));
                }
                Op::Swap => {
                    let len = stack.len();
                    if len < 2 {
                        return None;
                    }
                    stack.swap(len - 1, len - 2);
                }
                _ => return None,
            }
        }
        Some(rops)
    }

    fn lower_epilogue(&mut self, ops: &[Op]) -> Option<(Vec<Rop>, Option<u8>)> {
        if ops.is_empty() {
            return Some((Vec::new(), None));
        }
        let mut stack = Vec::new();
        let mut rops = Vec::new();
        for (index, op) in ops.iter().enumerate() {
            match *op {
                Op::Ireturn => {
                    if index + 1 != ops.len() {
                        return None;
                    }
                    let src = pop_int(&mut stack)?;
                    let dst = self.alloc()?;
                    rops.push(Rop::Mov(dst, src));
                    return Some((rops, Some(dst)));
                }
                Op::Return => {
                    if index + 1 != ops.len() {
                        return None;
                    }
                    return Some((rops, None));
                }
                Op::Nop | Op::Pop2 => {}
                Op::Pop => {
                    stack.pop();
                }
                Op::Iconst(value) => stack.push(V::Int(Src::I(value))),
                Op::Load(local) => stack.push(V::Int(Src::R(u8::try_from(local).ok()?))),
                Op::Store(local) | Op::StoreWide(local) => self.store(&mut stack, &mut rops, local)?,
                Op::Iadd => self.binop(&mut stack, &mut rops, Rop::Add)?,
                Op::Isub => self.binop(&mut stack, &mut rops, Rop::Sub)?,
                Op::Imul => self.binop(&mut stack, &mut rops, Rop::Mul)?,
                Op::Iand => self.binop(&mut stack, &mut rops, Rop::And)?,
                Op::Ior => self.binop(&mut stack, &mut rops, Rop::Or)?,
                Op::Ixor => self.binop(&mut stack, &mut rops, Rop::Xor)?,
                Op::GetThisField(field) => {
                    if let Some(reg) = self.int_field_reg(field) {
                        stack.push(V::Int(Src::R(reg)));
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        None
    }

    fn lower_int(&mut self, ops: &[IntTok]) -> Option<Vec<Rop>> {
        let mut stack = Vec::new();
        let mut rops = Vec::new();
        for op in ops {
            match *op {
                IntTok::Other | IntTok::Jump | IntTok::IfIcmpGe(_) | IntTok::Goto(_) | IntTok::Ireturn | IntTok::Return => return None,
                IntTok::Load(index) => stack.push(V::Int(Src::R(u8::try_from(index).ok()?))),
                IntTok::Iconst(value) => stack.push(V::Int(Src::I(value))),
                IntTok::Store(index) => self.store(&mut stack, &mut rops, index)?,
                IntTok::Iinc(index, amount) => {
                    let dst = u8::try_from(index).ok()?;
                    rops.push(Rop::Add(dst, Src::R(dst), Src::I(i32::from(amount))));
                    self.mark_assigned(dst);
                }
                IntTok::Iadd => self.binop(&mut stack, &mut rops, Rop::Add)?,
                IntTok::Isub => self.binop(&mut stack, &mut rops, Rop::Sub)?,
                IntTok::Imul => self.binop(&mut stack, &mut rops, Rop::Mul)?,
                IntTok::Idiv => self.binop(&mut stack, &mut rops, Rop::Div)?,
                IntTok::Irem => self.binop(&mut stack, &mut rops, Rop::Rem)?,
                IntTok::Iand => self.binop(&mut stack, &mut rops, Rop::And)?,
                IntTok::Ior => self.binop(&mut stack, &mut rops, Rop::Or)?,
                IntTok::Ixor => self.binop(&mut stack, &mut rops, Rop::Xor)?,
                IntTok::Ishl => self.binop(&mut stack, &mut rops, Rop::Shl)?,
                IntTok::Ishr => self.binop(&mut stack, &mut rops, Rop::Shr)?,
                IntTok::Iushr => self.binop(&mut stack, &mut rops, Rop::Ushr)?,
                IntTok::Ineg => self.unop(&mut stack, &mut rops, Rop::Neg)?,
                IntTok::I2b => self.unop(&mut stack, &mut rops, Rop::I2b)?,
                IntTok::I2c => self.unop(&mut stack, &mut rops, Rop::I2c)?,
                IntTok::I2s => self.unop(&mut stack, &mut rops, Rop::I2s)?,
            }
        }
        Some(rops)
    }

    fn lower_int_epilogue(&mut self, ops: &[IntTok]) -> Option<(Vec<Rop>, Option<u8>)> {
        if ops.is_empty() {
            return Some((Vec::new(), None));
        }
        let mut stack = Vec::new();
        let mut rops = Vec::new();
        for (index, op) in ops.iter().enumerate() {
            match *op {
                IntTok::Ireturn => {
                    if index + 1 != ops.len() {
                        return None;
                    }
                    let src = pop_int(&mut stack)?;
                    let dst = self.alloc()?;
                    rops.push(Rop::Mov(dst, src));
                    return Some((rops, Some(dst)));
                }
                IntTok::Return => {
                    if index + 1 != ops.len() {
                        return None;
                    }
                    return Some((rops, None));
                }
                IntTok::Load(local) => stack.push(V::Int(Src::R(u8::try_from(local).ok()?))),
                IntTok::Iconst(value) => stack.push(V::Int(Src::I(value))),
                IntTok::Store(local) => self.store(&mut stack, &mut rops, local)?,
                IntTok::Iadd => self.binop(&mut stack, &mut rops, Rop::Add)?,
                IntTok::Isub => self.binop(&mut stack, &mut rops, Rop::Sub)?,
                IntTok::Imul => self.binop(&mut stack, &mut rops, Rop::Mul)?,
                IntTok::Iand => self.binop(&mut stack, &mut rops, Rop::And)?,
                IntTok::Ior => self.binop(&mut stack, &mut rops, Rop::Or)?,
                IntTok::Ixor => self.binop(&mut stack, &mut rops, Rop::Xor)?,
                _ => return None,
            }
        }
        None
    }
}

fn rewrite_dst(op: Option<&mut Rop>, from: u8, to: u8) -> bool {
    let Some(op) = op else {
        return false;
    };
    let dst = match op {
        Rop::Mov(dst, _)
        | Rop::Add(dst, _, _)
        | Rop::Sub(dst, _, _)
        | Rop::Mul(dst, _, _)
        | Rop::Div(dst, _, _)
        | Rop::Rem(dst, _, _)
        | Rop::And(dst, _, _)
        | Rop::Or(dst, _, _)
        | Rop::Xor(dst, _, _)
        | Rop::Shl(dst, _, _)
        | Rop::Shr(dst, _, _)
        | Rop::Ushr(dst, _, _)
        | Rop::Neg(dst, _)
        | Rop::I2b(dst, _)
        | Rop::I2c(dst, _)
        | Rop::I2s(dst, _) => dst,
        _ => return false,
    };
    if *dst != from {
        return false;
    }
    *dst = to;
    true
}

fn pop_int(stack: &mut Vec<V>) -> Option<Src> {
    match stack.pop()? {
        V::Int(src) => Some(src),
        V::Obj => None,
    }
}

fn pop_obj(stack: &mut Vec<V>) -> Option<()> {
    match stack.pop()? {
        V::Obj => Some(()),
        V::Int(_) => None,
    }
}

fn op_tok(op: Op) -> Tok {
    match op {
        Op::Load(index) => Tok::Load(index),
        Op::Iconst(value) => Tok::Iconst(value),
        Op::Store(index) | Op::StoreWide(index) => Tok::Store(index),
        Op::Iinc(index, amount) => Tok::Iinc(index, amount),
        Op::IfIcmpGe(target) => Tok::IfIcmpGe(target),
        Op::Goto(target) => Tok::Goto(target),
        Op::IfEq(_)
        | Op::IfNe(_)
        | Op::IfLt(_)
        | Op::IfGe(_)
        | Op::IfGt(_)
        | Op::IfLe(_)
        | Op::IfIcmpEq(_)
        | Op::IfIcmpNe(_)
        | Op::IfIcmpLt(_)
        | Op::IfIcmpGt(_)
        | Op::IfIcmpLe(_)
        | Op::IfNull(_)
        | Op::IfNonNull(_) => Tok::Jump,
        _ => Tok::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_add_xor3_rotl1_matches_scalar() {
        let mut expected = 1i32;
        for i in 0..4096i32 {
            expected = expected.wrapping_add(i);
            expected ^= i.wrapping_mul(3);
            expected = expected.rotate_left(1);
        }
        let (got, end) = kernel_add_xor3_rotl1(1, 0, 4096, 1);
        assert_eq!(got, expected);
        assert_eq!(end, 4096);
    }
}
