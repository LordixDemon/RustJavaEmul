use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use parking_lot::Mutex;

static ENABLED: AtomicBool = AtomicBool::new(false);

pub const ARRAY_TYPE_COUNT: usize = 9;

static INTERPRETER_RUNS: AtomicU64 = AtomicU64::new(0);
static OPCODES: AtomicU64 = AtomicU64::new(0);
static FAST_OPCODES: AtomicU64 = AtomicU64::new(0);
static SLOW_OPCODES: AtomicU64 = AtomicU64::new(0);
static JUMPS: AtomicU64 = AtomicU64::new(0);
static RETURNS: AtomicU64 = AtomicU64::new(0);
static JAVA_EXCEPTIONS: AtomicU64 = AtomicU64::new(0);
static INVOKE_VIRTUAL: AtomicU64 = AtomicU64::new(0);
static INVOKE_SPECIAL: AtomicU64 = AtomicU64::new(0);
static INVOKE_STATIC: AtomicU64 = AtomicU64::new(0);
static INVOKE_INTERFACE: AtomicU64 = AtomicU64::new(0);
static ARRAY_NEW: AtomicU64 = AtomicU64::new(0);
static ARRAY_LOAD_ONE: AtomicU64 = AtomicU64::new(0);
static ARRAY_STORE_ONE: AtomicU64 = AtomicU64::new(0);
static ARRAY_LOAD_BULK: AtomicU64 = AtomicU64::new(0);
static ARRAY_STORE_BULK: AtomicU64 = AtomicU64::new(0);
static ARRAY_COPY: AtomicU64 = AtomicU64::new(0);
static ARRAY_RAW_READ: AtomicU64 = AtomicU64::new(0);
static ARRAY_RAW_WRITE: AtomicU64 = AtomicU64::new(0);
static BYTECODE_INTRINSICS_INSTALLED: AtomicU64 = AtomicU64::new(0);
static FIXED_POINT_SQRT_INTRINSIC_CALLS: AtomicU64 = AtomicU64::new(0);
static VECTOR_ARRAY_TRANSFORM_INTRINSIC_CALLS: AtomicU64 = AtomicU64::new(0);
static MATRIX_COMPOSE_INTRINSIC_CALLS: AtomicU64 = AtomicU64::new(0);
static INVERSE_SQRT_INTRINSIC_CALLS: AtomicU64 = AtomicU64::new(0);
static VECTOR_DOT_INTRINSIC_CALLS: AtomicU64 = AtomicU64::new(0);
static VECTOR_NORMALIZE_INTRINSIC_CALLS: AtomicU64 = AtomicU64::new(0);
static INT_ARRAY_RADIUS_INTRINSIC_CALLS: AtomicU64 = AtomicU64::new(0);
static MATRIX_INVERSE_TRANSFORM_INTRINSIC_CALLS: AtomicU64 = AtomicU64::new(0);
static INT_COMPILER_INSTALLED: AtomicU64 = AtomicU64::new(0);
static INT_COMPILER_CALLS: AtomicU64 = AtomicU64::new(0);
static ARRAY_LOAD_ONE_BY_TYPE: [AtomicU64; ARRAY_TYPE_COUNT] = [const { AtomicU64::new(0) }; ARRAY_TYPE_COUNT];
static ARRAY_STORE_ONE_BY_TYPE: [AtomicU64; ARRAY_TYPE_COUNT] = [const { AtomicU64::new(0) }; ARRAY_TYPE_COUNT];
static METHOD_OPCODE_COUNTERS: Mutex<Vec<MethodOpcodeCounter>> = Mutex::new(Vec::new());

#[derive(Clone)]
struct MethodOpcodeCounter {
    name: Arc<str>,
    calls: u64,
    opcodes: u64,
    max_opcodes: u64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ProfileSnapshot {
    pub interpreter_runs: u64,
    pub opcodes: u64,
    pub fast_opcodes: u64,
    pub slow_opcodes: u64,
    pub jumps: u64,
    pub returns: u64,
    pub java_exceptions: u64,
    pub invoke_virtual: u64,
    pub invoke_special: u64,
    pub invoke_static: u64,
    pub invoke_interface: u64,
    pub array_new: u64,
    pub array_load_one: u64,
    pub array_store_one: u64,
    pub array_load_bulk: u64,
    pub array_store_bulk: u64,
    pub array_copy: u64,
    pub array_raw_read: u64,
    pub array_raw_write: u64,
    pub array_load_one_by_type: [u64; ARRAY_TYPE_COUNT],
    pub array_store_one_by_type: [u64; ARRAY_TYPE_COUNT],
}

pub fn set_enabled(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
}

#[inline(always)]
pub(crate) fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn snapshot() -> ProfileSnapshot {
    ProfileSnapshot {
        interpreter_runs: INTERPRETER_RUNS.load(Ordering::Relaxed),
        opcodes: OPCODES.load(Ordering::Relaxed),
        fast_opcodes: FAST_OPCODES.load(Ordering::Relaxed),
        slow_opcodes: SLOW_OPCODES.load(Ordering::Relaxed),
        jumps: JUMPS.load(Ordering::Relaxed),
        returns: RETURNS.load(Ordering::Relaxed),
        java_exceptions: JAVA_EXCEPTIONS.load(Ordering::Relaxed),
        invoke_virtual: INVOKE_VIRTUAL.load(Ordering::Relaxed),
        invoke_special: INVOKE_SPECIAL.load(Ordering::Relaxed),
        invoke_static: INVOKE_STATIC.load(Ordering::Relaxed),
        invoke_interface: INVOKE_INTERFACE.load(Ordering::Relaxed),
        array_new: ARRAY_NEW.load(Ordering::Relaxed),
        array_load_one: ARRAY_LOAD_ONE.load(Ordering::Relaxed),
        array_store_one: ARRAY_STORE_ONE.load(Ordering::Relaxed),
        array_load_bulk: ARRAY_LOAD_BULK.load(Ordering::Relaxed),
        array_store_bulk: ARRAY_STORE_BULK.load(Ordering::Relaxed),
        array_copy: ARRAY_COPY.load(Ordering::Relaxed),
        array_raw_read: ARRAY_RAW_READ.load(Ordering::Relaxed),
        array_raw_write: ARRAY_RAW_WRITE.load(Ordering::Relaxed),
        array_load_one_by_type: core::array::from_fn(|index| ARRAY_LOAD_ONE_BY_TYPE[index].load(Ordering::Relaxed)),
        array_store_one_by_type: core::array::from_fn(|index| ARRAY_STORE_ONE_BY_TYPE[index].load(Ordering::Relaxed)),
    }
}

pub fn reset() {
    for counter in counters() {
        counter.store(0, Ordering::Relaxed);
    }
    for counter in ARRAY_LOAD_ONE_BY_TYPE.iter().chain(ARRAY_STORE_ONE_BY_TYPE.iter()) {
        counter.store(0, Ordering::Relaxed);
    }
    METHOD_OPCODE_COUNTERS.lock().clear();
}

pub fn method_opcode_report_and_reset(limit: usize) -> String {
    let mut counters = METHOD_OPCODE_COUNTERS.lock();
    if counters.is_empty() {
        return "jvm methods none".to_string();
    }

    counters.sort_unstable_by(|lhs, rhs| rhs.opcodes.cmp(&lhs.opcodes).then_with(|| rhs.calls.cmp(&lhs.calls)));

    let mut output = String::from("jvm methods");
    for counter in counters.iter().take(limit) {
        if counter.opcodes == 0 {
            continue;
        }
        output.push(' ');
        output.push_str(counter.name.as_ref());
        output.push_str(":calls=");
        output.push_str(&counter.calls.to_string());
        output.push_str(",op=");
        output.push_str(&counter.opcodes.to_string());
        output.push_str(",max=");
        output.push_str(&counter.max_opcodes.to_string());
    }

    for counter in counters.iter_mut() {
        counter.calls = 0;
        counter.opcodes = 0;
        counter.max_opcodes = 0;
    }

    if output == "jvm methods" {
        output.push_str(" none");
    }
    output
}

pub fn intrinsic_report_and_reset() -> String {
    let installed = BYTECODE_INTRINSICS_INSTALLED.load(Ordering::Relaxed);
    let fixed_sqrt = FIXED_POINT_SQRT_INTRINSIC_CALLS.swap(0, Ordering::Relaxed);
    let vector_array_transform = VECTOR_ARRAY_TRANSFORM_INTRINSIC_CALLS.swap(0, Ordering::Relaxed);
    let matrix_compose = MATRIX_COMPOSE_INTRINSIC_CALLS.swap(0, Ordering::Relaxed);
    let inverse_sqrt = INVERSE_SQRT_INTRINSIC_CALLS.swap(0, Ordering::Relaxed);
    let vector_dot = VECTOR_DOT_INTRINSIC_CALLS.swap(0, Ordering::Relaxed);
    let vector_normalize = VECTOR_NORMALIZE_INTRINSIC_CALLS.swap(0, Ordering::Relaxed);
    let int_array_radius = INT_ARRAY_RADIUS_INTRINSIC_CALLS.swap(0, Ordering::Relaxed);
    let matrix_inverse_transform = MATRIX_INVERSE_TRANSFORM_INTRINSIC_CALLS.swap(0, Ordering::Relaxed);
    let int_compiler_installed = INT_COMPILER_INSTALLED.load(Ordering::Relaxed);
    let int_compiler_calls = INT_COMPILER_CALLS.swap(0, Ordering::Relaxed);

    if installed == 0
        && fixed_sqrt == 0
        && vector_array_transform == 0
        && matrix_compose == 0
        && inverse_sqrt == 0
        && vector_dot == 0
        && vector_normalize == 0
        && int_array_radius == 0
        && matrix_inverse_transform == 0
        && int_compiler_installed == 0
        && int_compiler_calls == 0
    {
        return "jvm intrinsics none".to_string();
    }

    alloc::format!(
        "jvm intrinsics installed={installed} intCompilerInstalled={int_compiler_installed} intCompilerCalls={int_compiler_calls} fixedSqrtLongToIntCalls={fixed_sqrt} vectorArrayTransformCalls={vector_array_transform} matrixComposeCalls={matrix_compose} inverseSqrtCalls={inverse_sqrt} vectorDotCalls={vector_dot} vectorNormalizeCalls={vector_normalize} intArrayRadiusCalls={int_array_radius} matrixInverseTransformCalls={matrix_inverse_transform}"
    )
}

pub(crate) fn record_bytecode_intrinsic_install() {
    inc(&BYTECODE_INTRINSICS_INSTALLED);
}

pub(crate) fn record_int_compiler_install() {
    inc(&INT_COMPILER_INSTALLED);
}

#[inline(always)]
pub(crate) fn int_compiler_call() {
    if enabled() {
        inc(&INT_COMPILER_CALLS);
    }
}

pub(crate) fn fixed_point_sqrt_intrinsic_call() {
    inc(&FIXED_POINT_SQRT_INTRINSIC_CALLS);
}

pub(crate) fn vector_array_transform_intrinsic_call() {
    inc(&VECTOR_ARRAY_TRANSFORM_INTRINSIC_CALLS);
}

pub(crate) fn matrix_compose_intrinsic_call() {
    inc(&MATRIX_COMPOSE_INTRINSIC_CALLS);
}

pub(crate) fn inverse_sqrt_intrinsic_call() {
    inc(&INVERSE_SQRT_INTRINSIC_CALLS);
}

pub(crate) fn vector_dot_intrinsic_call() {
    inc(&VECTOR_DOT_INTRINSIC_CALLS);
}

pub(crate) fn vector_normalize_intrinsic_call() {
    inc(&VECTOR_NORMALIZE_INTRINSIC_CALLS);
}

pub(crate) fn int_array_radius_intrinsic_call() {
    inc(&INT_ARRAY_RADIUS_INTRINSIC_CALLS);
}

pub(crate) fn matrix_inverse_transform_intrinsic_call() {
    inc(&MATRIX_INVERSE_TRANSFORM_INTRINSIC_CALLS);
}

pub(crate) fn record_method_opcodes(name: Arc<str>, opcodes: u64) {
    if opcodes == 0 {
        return;
    }

    let mut counters = METHOD_OPCODE_COUNTERS.lock();
    if let Some(counter) = counters.iter_mut().find(|counter| counter.name.as_ref() == name.as_ref()) {
        counter.calls += 1;
        counter.opcodes += opcodes;
        counter.max_opcodes = counter.max_opcodes.max(opcodes);
        return;
    }

    counters.push(MethodOpcodeCounter {
        name,
        calls: 1,
        opcodes,
        max_opcodes: opcodes,
    });
}

fn counters() -> [&'static AtomicU64; 30] {
    [
        &INTERPRETER_RUNS,
        &OPCODES,
        &FAST_OPCODES,
        &SLOW_OPCODES,
        &JUMPS,
        &RETURNS,
        &JAVA_EXCEPTIONS,
        &INVOKE_VIRTUAL,
        &INVOKE_SPECIAL,
        &INVOKE_STATIC,
        &INVOKE_INTERFACE,
        &ARRAY_NEW,
        &ARRAY_LOAD_ONE,
        &ARRAY_STORE_ONE,
        &ARRAY_LOAD_BULK,
        &ARRAY_STORE_BULK,
        &ARRAY_COPY,
        &ARRAY_RAW_READ,
        &ARRAY_RAW_WRITE,
        &BYTECODE_INTRINSICS_INSTALLED,
        &FIXED_POINT_SQRT_INTRINSIC_CALLS,
        &VECTOR_ARRAY_TRANSFORM_INTRINSIC_CALLS,
        &MATRIX_COMPOSE_INTRINSIC_CALLS,
        &INVERSE_SQRT_INTRINSIC_CALLS,
        &VECTOR_DOT_INTRINSIC_CALLS,
        &VECTOR_NORMALIZE_INTRINSIC_CALLS,
        &INT_ARRAY_RADIUS_INTRINSIC_CALLS,
        &MATRIX_INVERSE_TRANSFORM_INTRINSIC_CALLS,
        &INT_COMPILER_INSTALLED,
        &INT_COMPILER_CALLS,
    ]
}

#[inline(always)]
fn inc(counter: &AtomicU64) {
    counter.fetch_add(1, Ordering::Relaxed);
}

#[inline(always)]
pub(crate) fn interpreter_run() {
    inc(&INTERPRETER_RUNS);
}

#[inline(always)]
pub(crate) fn opcode() {
    inc(&OPCODES);
}

#[inline(always)]
pub(crate) fn fast_opcode() {
    inc(&FAST_OPCODES);
}

#[inline(always)]
pub(crate) fn slow_opcode() {
    inc(&SLOW_OPCODES);
}

#[inline(always)]
pub(crate) fn jump() {
    inc(&JUMPS);
}

#[inline(always)]
pub(crate) fn return_value() {
    inc(&RETURNS);
}

#[inline(always)]
pub(crate) fn java_exception() {
    inc(&JAVA_EXCEPTIONS);
}

#[inline(always)]
pub(crate) fn invoke_virtual() {
    inc(&INVOKE_VIRTUAL);
}

#[inline(always)]
pub(crate) fn invoke_special() {
    inc(&INVOKE_SPECIAL);
}

#[inline(always)]
pub(crate) fn invoke_static() {
    inc(&INVOKE_STATIC);
}

#[inline(always)]
pub(crate) fn invoke_interface() {
    inc(&INVOKE_INTERFACE);
}

#[inline(always)]
pub(crate) fn array_new() {
    if enabled() {
        inc(&ARRAY_NEW);
    }
}

#[inline(always)]
pub(crate) fn array_load_one_typed(type_index: usize) {
    if enabled() {
        inc(&ARRAY_LOAD_ONE);
        inc(&ARRAY_LOAD_ONE_BY_TYPE[type_index.min(ARRAY_TYPE_COUNT - 1)]);
    }
}

#[inline(always)]
pub(crate) fn array_store_one_typed(type_index: usize) {
    if enabled() {
        inc(&ARRAY_STORE_ONE);
        inc(&ARRAY_STORE_ONE_BY_TYPE[type_index.min(ARRAY_TYPE_COUNT - 1)]);
    }
}

#[inline(always)]
pub(crate) fn array_load_bulk() {
    if enabled() {
        inc(&ARRAY_LOAD_BULK);
    }
}

#[inline(always)]
pub(crate) fn array_store_bulk() {
    if enabled() {
        inc(&ARRAY_STORE_BULK);
    }
}

#[inline(always)]
pub(crate) fn array_copy() {
    if enabled() {
        inc(&ARRAY_COPY);
    }
}

#[inline(always)]
pub(crate) fn array_raw_read() {
    if enabled() {
        inc(&ARRAY_RAW_READ);
    }
}

#[inline(always)]
pub(crate) fn array_raw_write() {
    if enabled() {
        inc(&ARRAY_RAW_WRITE);
    }
}
