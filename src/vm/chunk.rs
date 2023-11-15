use std::ops::Index;

use arrayvec::ArrayVec;

use crate::error::{OverflowError, Result};
use crate::types::Span;
use crate::vm::op;
use crate::vm::value::Value;

#[derive(Debug, Default)]
pub struct Chunk {
    pub ops: Vec<u8>,
    pub constants: ArrayVec<Value, 256>,
    pub spans: VecRun<Span>,
}

impl Chunk {
    pub fn write_u8(&mut self, byte: u8, span: &Span) {
        self.ops.push(byte);
        self.spans.push(span.clone());
    }

    /// Writes a constant to the [`Chunk`] and returns its index. If an equal
    /// [`Value`] is already present, then its index is returned instead.
    pub fn write_constant(&mut self, value: Value, span: &Span) -> Result<u8> {
        let idx = match self.constants.iter().position(|&constant| constant == value) {
            Some(idx) => idx,
            None => {
                self.constants
                    .try_push(value)
                    .map_err(|_| (OverflowError::TooManyConstants.into(), span.clone()))?;
                self.constants.len() - 1
            }
        };
        Ok(idx.try_into().expect("constant index overflow"))
    }

    pub fn debug(&self, name: Option<&str>) {
        if let Some(name) = name {
            eprintln!("== {name} ==");
        }

        let result = self.disassemble();

        eprintln!("{result}");
    }

    pub fn disassemble(&self) -> String {
        let mut out = String::from("");

        let mut idx = 0;

        while idx < self.ops.len() {
            let (byte_size, disassembled_byte_idx, disassembled_op) = self.disassemble_op(idx);

            out.push_str(&format!("{disassembled_byte_idx} {disassembled_op}"));

            idx += byte_size;
        }

        out
    }

    pub fn disassemble_op(&self, idx: usize) -> (usize, String, String) {
        let byte_idx_str = format!("{idx:04}");

        let (byte_inc, op_str): (usize, String) = match self.ops[idx] {
            op::CONSTANT => self.debug_op_constant("OP_CONSTANT", idx),
            op::NIL => self.debug_op_simple("OP_NIL"),
            op::TRUE => self.debug_op_simple("OP_TRUE"),
            op::FALSE => self.debug_op_simple("OP_FALSE"),
            op::POP => self.debug_op_simple("OP_POP"),
            op::GET_LOCAL => self.debug_op_byte("OP_GET_LOCAL", idx),
            op::SET_LOCAL => self.debug_op_byte("OP_SET_LOCAL", idx),
            op::GET_GLOBAL => self.debug_op_constant("OP_GET_GLOBAL", idx),
            op::DEFINE_GLOBAL => self.debug_op_constant("OP_DEFINE_GLOBAL", idx),
            op::SET_GLOBAL => self.debug_op_constant("OP_SET_GLOBAL", idx),
            op::GET_UPVALUE => self.debug_op_byte("OP_GET_UPVALUE", idx),
            op::SET_UPVALUE => self.debug_op_byte("OP_SET_UPVALUE", idx),
            op::GET_PROPERTY => self.debug_op_constant("OP_GET_PROPERTY", idx),
            op::SET_PROPERTY => self.debug_op_constant("OP_SET_PROPERTY", idx),
            op::GET_SUPER => self.debug_op_constant("OP_GET_SUPER", idx),
            op::EQUAL => self.debug_op_simple("OP_EQUAL"),
            op::NOT_EQUAL => self.debug_op_simple("OP_NOT_EQUAL"),
            op::GREATER => self.debug_op_simple("OP_GREATER"),
            op::GREATER_EQUAL => self.debug_op_simple("OP_GREATER_EQUAL"),
            op::LESS => self.debug_op_simple("OP_LESS"),
            op::LESS_EQUAL => self.debug_op_simple("OP_LESS_EQUAL"),
            op::ADD => self.debug_op_simple("OP_ADD"),
            op::SUBTRACT => self.debug_op_simple("OP_SUBTRACT"),
            op::MULTIPLY => self.debug_op_simple("OP_MULTIPLY"),
            op::DIVIDE => self.debug_op_simple("OP_DIVIDE"),
            op::NOT => self.debug_op_simple("OP_NOT"),
            op::NEGATE => self.debug_op_simple("OP_NEGATE"),
            op::PRINT => self.debug_op_simple("OP_PRINT"),
            op::JUMP => self.debug_op_jump("OP_JUMP", idx, true),
            op::JUMP_IF_FALSE => self.debug_op_jump("OP_JUMP_IF_FALSE", idx, true),
            op::LOOP => self.debug_op_jump("OP_LOOP", idx, false),
            op::CALL => self.debug_op_byte("OP_CALL", idx),
            op::INVOKE => self.debug_op_invoke("OP_INVOKE", idx),
            op::SUPER_INVOKE => self.debug_op_invoke("OP_SUPER_INVOKE", idx),
            op::CLOSURE => {
                let mut closure_str = String::from("");
                let mut closure_idx_inc = 0;

                closure_idx_inc += 1;

                let constant_idx = self.ops[idx + closure_idx_inc];
                let constant = &self.constants[constant_idx as usize];
                let name = "OP_CLOSURE";
                closure_str.push_str(&format!("{name:16} {constant_idx:>4} '{constant}'"));
                closure_str.push('\n');

                let function = unsafe { constant.as_object().function };
                for _ in 0..unsafe { (*function).upvalue_count } {
                    let offset = idx + closure_idx_inc;

                    closure_idx_inc += 1;
                    let is_local = self.ops[idx + closure_idx_inc];
                    let label = if is_local == 0 { "upvalue" } else { "local" };

                    closure_idx_inc += 1;
                    let upvalue_idx = self.ops[idx + closure_idx_inc];

                    closure_str.push_str(&format!(
                        "{offset:04} |                     {label} {upvalue_idx}"
                    ));
                    closure_str.push('\n');
                }

                (closure_idx_inc, closure_str)
            }
            op::CLOSE_UPVALUE => self.debug_op_simple("OP_CLOSE_UPVALUE"),
            op::RETURN => self.debug_op_simple("OP_RETURN"),
            op::CLASS => self.debug_op_constant("OP_CLASS", idx),
            op::INHERIT => self.debug_op_simple("OP_INHERIT"),
            op::METHOD => self.debug_op_constant("OP_METHOD", idx),
            byte => self.debug_op_simple(&format!("OP_UNKNOWN({byte:#X})")),
        };

        (byte_inc, byte_idx_str, op_str)
    }

    fn debug_op_simple(&self, name: &str) -> (usize, String) {
        (1, format!("{name}\n"))
    }

    fn debug_op_byte(&self, name: &str, idx: usize) -> (usize, String) {
        let byte = self.ops[idx + 1];
        let string = format!("{name:16} {byte:>4}\n");

        (2, string)
    }

    fn debug_op_constant(&self, name: &str, idx: usize) -> (usize, String) {
        let constant_idx = self.ops[idx + 1];
        let constant = &self.constants[constant_idx as usize];
        let string = format!("{name:16} {constant_idx:>4} '{constant}'\n");

        (2, string)
    }

    fn debug_op_invoke(&self, name: &str, idx: usize) -> (usize, String) {
        let constant_idx = self.ops[idx + 1];
        let constant = &self.constants[constant_idx as usize];
        let arg_count = self.ops[idx + 2];
        let string = format!("{name:16} ({arg_count} args) {constant_idx:>4} '{constant}'\n");

        (3, string)
    }

    fn debug_op_jump(&self, name: &str, idx: usize, is_forward: bool) -> (usize, String) {
        let to_offset = u16::from_le_bytes([self.ops[idx + 1], self.ops[idx + 2]]);
        let offset_sign = if is_forward { 1 } else { -1 };
        // The +3 is to account for the 3 byte jump instruction.
        let to_idx = (idx as isize) + (to_offset as isize) * offset_sign + 3;
        let string = format!("{name:16} {idx:>4} -> {to_idx}\n");

        (3, string)
    }
}

/// Run-length encoded [`Vec`]. Useful for storing data with a lot of contiguous
/// runs of the same value.
#[derive(Debug, Default)]
pub struct VecRun<T> {
    values: Vec<Run<T>>,
}

impl<T: Eq> VecRun<T> {
    fn push(&mut self, value: T) {
        match self.values.last_mut() {
            Some(run) if run.value == value && run.count < u8::MAX => {
                run.count += 1;
            }
            _ => self.values.push(Run { value, count: 1 }),
        };
    }
}

impl<T> Index<usize> for VecRun<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut count = index;
        for run in &self.values {
            match count.checked_sub(run.count as usize) {
                Some(remaining) => count = remaining,
                None => return &run.value,
            }
        }
        panic!("index out of bounds");
    }
}

#[derive(Debug)]
struct Run<T> {
    value: T,
    count: u8,
}
