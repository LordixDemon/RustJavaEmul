use alloc::vec::Vec;

use jvm::JavaValue;

#[derive(Default)]
pub struct StackFrame {
    pub local_variables: Vec<JavaValue>,
    pub operand_stack: Vec<JavaValue>,
}

impl StackFrame {
    pub fn with_capacity(max_stack: usize, max_locals: usize) -> Self {
        Self {
            local_variables: Vec::with_capacity(max_locals),
            operand_stack: Vec::with_capacity(max_stack),
        }
    }
}
