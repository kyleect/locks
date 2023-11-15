use super::{chunk::Chunk, op};

pub struct Disassembler<'a> {
    chunk: &'a Chunk,
}

impl<'a> Disassembler<'a> {
    pub fn disassemble(&self) -> String {
        let mut out = String::new();

        let mut idx = 0;

        while idx < self.chunk.ops.len() {
            let (byte_size, disassembled_byte_idx, disassembled_op) = self.disassemble_op(idx);

            let a = &format!("{disassembled_byte_idx} {disassembled_op}");

            out.push_str(a);

            idx += byte_size;
        }

        out
    }

    fn disassemble_op(&self, idx: usize) -> (usize, String, String) {
        let byte_idx_str = format!("{idx:04}");

        let (byte_inc, op_str): (usize, String) = match self.chunk.ops[idx] {
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

                let constant_idx = self.chunk.ops[idx + closure_idx_inc];
                let constant = &self.chunk.constants[constant_idx as usize];
                let name = "OP_CLOSURE";
                closure_str.push_str(&format!("{name:16} {constant_idx:>4} '{constant}'\n"));

                let function = unsafe { constant.as_object().function };
                for _ in 0..unsafe { (*function).upvalue_count } {
                    let offset = idx + closure_idx_inc;

                    closure_idx_inc += 1;
                    let is_local = self.chunk.ops[idx + closure_idx_inc];
                    let label = if is_local == 0 { "upvalue" } else { "local" };

                    closure_idx_inc += 1;
                    let upvalue_idx = self.chunk.ops[idx + closure_idx_inc];

                    closure_str.push_str(&format!(
                        "{offset:04} |                     {label} {upvalue_idx}\n"
                    ));
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
        let byte = self.chunk.ops[idx + 1];
        let string = format!("{name:16} {byte:>4}\n");

        (2, string)
    }

    fn debug_op_constant(&self, name: &str, idx: usize) -> (usize, String) {
        let constant_idx = self.chunk.ops[idx + 1];
        let constant = &self.chunk.constants[constant_idx as usize];
        let string = format!("{name:16} {constant_idx:>4} '{constant}'\n");

        (2, string)
    }

    fn debug_op_invoke(&self, name: &str, idx: usize) -> (usize, String) {
        let constant_idx = self.chunk.ops[idx + 1];
        let constant = &self.chunk.constants[constant_idx as usize];
        let arg_count = self.chunk.ops[idx + 2];
        let string = format!("{name:16} ({arg_count} args) {constant_idx:>4} '{constant}'\n");

        (3, string)
    }

    fn debug_op_jump(&self, name: &str, idx: usize, is_forward: bool) -> (usize, String) {
        let to_offset = u16::from_le_bytes([self.chunk.ops[idx + 1], self.chunk.ops[idx + 2]]);
        let offset_sign = if is_forward { 1 } else { -1 };
        // The +3 is to account for the 3 byte jump instruction.
        let to_idx = (idx as isize) + (to_offset as isize) * offset_sign + 3;
        let string = format!("{name:16} {idx:>4} -> {to_idx}\n");

        (3, string)
    }
}

#[cfg(test)]
mod tests {
    use crate::vm::{Compiler, Gc};

    use super::Disassembler;

    parameterized_test::create! { assert_disassembly, (code, disassembly), {
        let mut gc = Gc::default();

        if let Ok(function) = Compiler::compile(&code, code.len(), &mut gc) {
            let chunk = unsafe { &(*function).chunk };

            let d = Disassembler { chunk: &chunk };

            let result = d.disassemble();

            assert_eq!(result, disassembly);
        } else {
            assert!(false, "FAIL");
        }
    }}

    assert_disassembly! {
        jump_to_false: (
            "\
            if (false) {
                print 1;
            } else {
                print 2;
            }",
            "\
            0000 OP_FALSE\n\
            0001 OP_JUMP_IF_FALSE    1 -> 11\n\
            0004 OP_POP\n\
            0005 OP_CONSTANT         0 '1'\n\
            0007 OP_PRINT\n\
            0008 OP_JUMP             8 -> 15\n\
            0011 OP_POP\n\
            0012 OP_CONSTANT         1 '2'\n\
            0014 OP_PRINT\n\
            0015 OP_NIL\n\
            0016 OP_RETURN\n"
        ),
    }
}
