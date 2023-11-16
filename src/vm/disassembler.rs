use super::{chunk::Chunk, op};

/// Disassemble byte code chunks in to text
pub struct Disassembler<'a> {
    pub chunk: &'a Chunk,
}

impl<'a> Disassembler<'a> {
    /// Visualize the byte code as text
    pub fn disassemble(&self) -> String {
        let mut out = String::new();

        let mut op_idx = 0;

        while op_idx < self.chunk.ops.len() {
            let (op_byte_size, disassembled_byte_idx, disassembled_op) =
                self.disassemble_op(op_idx);

            let op_string = &format!("{disassembled_byte_idx} {disassembled_op}");
            out.push_str(op_string);

            op_idx += op_byte_size;
        }

        out
    }

    fn disassemble_op(&self, op_idx: usize) -> (usize, String, String) {
        let op_idx_str = format!("{op_idx:04}");

        let (op_idx_inc, op_str): (usize, String) = match self.chunk.ops[op_idx] {
            op::CONSTANT => self.disassemble_op_constant("OP_CONSTANT", op_idx),
            op::NIL => self.disassemble_op_simple("OP_NIL"),
            op::TRUE => self.disassemble_op_simple("OP_TRUE"),
            op::FALSE => self.disassemble_op_simple("OP_FALSE"),
            op::POP => self.disassemble_op_simple("OP_POP"),
            op::GET_LOCAL => self.disassemble_op_byte("OP_GET_LOCAL", op_idx),
            op::SET_LOCAL => self.disassemble_op_byte("OP_SET_LOCAL", op_idx),
            op::GET_GLOBAL => self.disassemble_op_constant("OP_GET_GLOBAL", op_idx),
            op::DEFINE_GLOBAL => self.disassemble_op_constant("OP_DEFINE_GLOBAL", op_idx),
            op::SET_GLOBAL => self.disassemble_op_constant("OP_SET_GLOBAL", op_idx),
            op::GET_UPVALUE => self.disassemble_op_byte("OP_GET_UPVALUE", op_idx),
            op::SET_UPVALUE => self.disassemble_op_byte("OP_SET_UPVALUE", op_idx),
            op::GET_PROPERTY => self.disassemble_op_constant("OP_GET_PROPERTY", op_idx),
            op::SET_PROPERTY => self.disassemble_op_constant("OP_SET_PROPERTY", op_idx),
            op::GET_SUPER => self.disassemble_op_constant("OP_GET_SUPER", op_idx),
            op::EQUAL => self.disassemble_op_simple("OP_EQUAL"),
            op::NOT_EQUAL => self.disassemble_op_simple("OP_NOT_EQUAL"),
            op::GREATER => self.disassemble_op_simple("OP_GREATER"),
            op::GREATER_EQUAL => self.disassemble_op_simple("OP_GREATER_EQUAL"),
            op::LESS => self.disassemble_op_simple("OP_LESS"),
            op::LESS_EQUAL => self.disassemble_op_simple("OP_LESS_EQUAL"),
            op::ADD => self.disassemble_op_simple("OP_ADD"),
            op::SUBTRACT => self.disassemble_op_simple("OP_SUBTRACT"),
            op::MULTIPLY => self.disassemble_op_simple("OP_MULTIPLY"),
            op::DIVIDE => self.disassemble_op_simple("OP_DIVIDE"),
            op::NOT => self.disassemble_op_simple("OP_NOT"),
            op::NEGATE => self.disassemble_op_simple("OP_NEGATE"),
            op::PRINT => self.disassemble_op_simple("OP_PRINT"),
            op::JUMP => self.disassemble_op_jump("OP_JUMP", op_idx, true),
            op::JUMP_IF_FALSE => self.disassemble_op_jump("OP_JUMP_IF_FALSE", op_idx, true),
            op::LOOP => self.disassemble_op_jump("OP_LOOP", op_idx, false),
            op::CALL => self.disassemble_op_byte("OP_CALL", op_idx),
            op::INVOKE => self.disassemble_op_invoke("OP_INVOKE", op_idx),
            op::SUPER_INVOKE => self.disassemble_op_invoke("OP_SUPER_INVOKE", op_idx),
            op::CLOSURE => {
                let mut op_idx_b = op_idx + 1;

                let mut closure_str = String::from("");

                let constant_idx = self.chunk.ops[op_idx_b];
                let constant = &self.chunk.constants[constant_idx as usize];
                let name = "OP_CLOSURE";
                closure_str.push_str(&format!("{name:16} {constant_idx:>4} == '{constant}'\n"));

                let function = unsafe { constant.as_object().function };
                for _ in 0..unsafe { (*function).upvalue_count } {
                    let offset = op_idx_b;

                    op_idx_b += 1;
                    let is_local = self.chunk.ops[op_idx_b];
                    let label = if is_local == 0 { "upvalue" } else { "local" };

                    op_idx_b += 1;
                    let upvalue_idx = self.chunk.ops[op_idx_b];

                    closure_str.push_str(&format!(
                        "{offset:04} |                     {label} {upvalue_idx}\n"
                    ));
                }

                op_idx_b += 1;

                let op_idx_inc = op_idx_b - op_idx;

                (op_idx_inc, closure_str)
            }
            op::CLOSE_UPVALUE => self.disassemble_op_simple("OP_CLOSE_UPVALUE"),
            op::RETURN => self.disassemble_op_simple("OP_RETURN"),
            op::CLASS => self.disassemble_op_constant("OP_CLASS", op_idx),
            op::INHERIT => self.disassemble_op_simple("OP_INHERIT"),
            op::METHOD => self.disassemble_op_constant("OP_METHOD", op_idx),
            byte => self.disassemble_op_simple(&format!("OP_UNKNOWN({byte:#X})")),
        };

        (op_idx_inc, op_idx_str, op_str)
    }

    fn disassemble_op_simple(&self, name: &str) -> (usize, String) {
        (1, format!("{name}\n"))
    }

    fn disassemble_op_byte(&self, name: &str, op_idx: usize) -> (usize, String) {
        let byte = self.chunk.ops[op_idx + 1];
        let string = format!("{name:16} {byte:>4}\n");

        (2, string)
    }

    fn disassemble_op_constant(&self, name: &str, op_idx: usize) -> (usize, String) {
        let constant_idx = self.chunk.ops[op_idx + 1];
        let constant = &self.chunk.constants[constant_idx as usize];
        let string = format!("{name:16} {constant_idx:>4} == '{constant}'\n");

        (2, string)
    }

    fn disassemble_op_invoke(&self, name: &str, op_idx: usize) -> (usize, String) {
        let constant_idx = self.chunk.ops[op_idx + 1];
        let constant = &self.chunk.constants[constant_idx as usize];
        let arg_count = self.chunk.ops[op_idx + 2];
        let string = format!("{name:16} ({arg_count} args) {constant_idx:>4} '{constant}'\n");

        (3, string)
    }

    fn disassemble_op_jump(&self, name: &str, op_idx: usize, is_forward: bool) -> (usize, String) {
        let to_offset =
            u16::from_le_bytes([self.chunk.ops[op_idx + 1], self.chunk.ops[op_idx + 2]]);
        let offset_sign = if is_forward { 1 } else { -1 };
        // The +3 is to account for the 3 byte jump instruction.
        let to_idx = (op_idx as isize) + (to_offset as isize) * offset_sign + 3;
        let string = format!("{name:16} {op_idx:>4} -> {to_idx}\n");

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
            assert!(false, "There was a compile error!");
        }
    }}

    assert_disassembly! {
        assignment_string: (
            "var a = \"Hello\";",
            "\
            0000 OP_CONSTANT         0 == 'Hello'\n\
            0002 OP_DEFINE_GLOBAL    1 == 'a'\n\
            0004 OP_NIL\n\
            0005 OP_RETURN\n"
        ),
        assignment_number: (
            "var a = 123;",
            "\
            0000 OP_CONSTANT         0 == '123'\n\
            0002 OP_DEFINE_GLOBAL    1 == 'a'\n\
            0004 OP_NIL\n\
            0005 OP_RETURN\n"
        ),
        number: (
            "123;",
            "\
            0000 OP_CONSTANT         0 == '123'\n\
            0002 OP_POP\n\
            0003 OP_NIL\n\
            0004 OP_RETURN\n"
        ),
        negative_number: (
            "-123;",
            "\
            0000 OP_CONSTANT         0 == '123'\n\
            0002 OP_NEGATE\n\
            0003 OP_POP\n\
            0004 OP_NIL\n\
            0005 OP_RETURN\n"
        ),
        assign_negated_number: (
            "\
            var a = 123;\n\
            var b = -a;\n\
            var c = a + b;\n\
            print c;",
            "\
            0000 OP_CONSTANT         0 == '123'\n\
            0002 OP_DEFINE_GLOBAL    1 == 'a'\n\
            0004 OP_GET_GLOBAL       1 == 'a'\n\
            0006 OP_NEGATE\n\
            0007 OP_DEFINE_GLOBAL    2 == 'b'\n\
            0009 OP_GET_GLOBAL       1 == 'a'\n\
            0011 OP_GET_GLOBAL       2 == 'b'\n\
            0013 OP_ADD\n\
            0014 OP_DEFINE_GLOBAL    3 == 'c'\n\
            0016 OP_GET_GLOBAL       3 == 'c'\n\
            0018 OP_PRINT\n\
            0019 OP_NIL\n\
            0020 OP_RETURN\n"
        ),
        fn_define_empty: (
            "fn sum (a, b) { }",
            "\
            0000 OP_CLOSURE          0 == '<function sum>'\n\
            0002 OP_DEFINE_GLOBAL    1 == 'sum'\n\
            0004 OP_NIL\n\
            0005 OP_RETURN\n"
        ),
        fn_define_empty_no_args: (
            "fn sum () { }",
            "\
            0000 OP_CLOSURE          0 == '<function sum>'\n\
            0002 OP_DEFINE_GLOBAL    1 == 'sum'\n\
            0004 OP_NIL\n\
            0005 OP_RETURN\n"
        ),
        fn_define: (
            "fn sum (a, b) { return a + b; }",
            "\
            0000 OP_CLOSURE          0 == '<function sum>'\n\
            0002 OP_DEFINE_GLOBAL    1 == 'sum'\n\
            0004 OP_NIL\n\
            0005 OP_RETURN\n"
        ),
        fn_call: (
            "\
            fn sum (a, b) { return a + b; }\n\
            sum(100, 200);",
            "\
            0000 OP_CLOSURE          0 == '<function sum>'\n\
            0002 OP_DEFINE_GLOBAL    1 == 'sum'\n\
            0004 OP_GET_GLOBAL       1 == 'sum'\n\
            0006 OP_CONSTANT         2 == '100'\n\
            0008 OP_CONSTANT         3 == '200'\n\
            0010 OP_CALL             2\n\
            0012 OP_POP\n\
            0013 OP_NIL\n\
            0014 OP_RETURN\n"
        ),
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
            0005 OP_CONSTANT         0 == '1'\n\
            0007 OP_PRINT\n\
            0008 OP_JUMP             8 -> 15\n\
            0011 OP_POP\n\
            0012 OP_CONSTANT         1 == '2'\n\
            0014 OP_PRINT\n\
            0015 OP_NIL\n\
            0016 OP_RETURN\n"
        ),
        arity: (
            "\
            class Foo {
                method0() { return \"no args\"; }
                method1(a) { return a; }
                method2(a, b) { return a + b; }
                method3(a, b, c) { return a + b + c; }
                method4(a, b, c, d) { return a + b + c + d; }
                method5(a, b, c, d, e) { return a + b + c + d + e; }
                method6(a, b, c, d, e, f) { return a + b + c + d + e + f; }
                method7(a, b, c, d, e, f, g) { return a + b + c + d + e + f + g; }
                method8(a, b, c, d, e, f, g, h) { return a + b + c + d + e + f + g + h; }
              }
              
              var foo = Foo();
              print foo.method0(); // out: no args
              print foo.method1(1); // out: 1
              print foo.method2(1, 2); // out: 3
              print foo.method3(1, 2, 3); // out: 6
              print foo.method4(1, 2, 3, 4); // out: 10
              print foo.method5(1, 2, 3, 4, 5); // out: 15
              print foo.method6(1, 2, 3, 4, 5, 6); // out: 21
              print foo.method7(1, 2, 3, 4, 5, 6, 7); // out: 28
              print foo.method8(1, 2, 3, 4, 5, 6, 7, 8); // out: 36
              ",
            "\
            0000 OP_CLASS            0 == 'Foo'\n\
            0002 OP_DEFINE_GLOBAL    0 == 'Foo'\n\
            0004 OP_GET_GLOBAL       0 == 'Foo'\n\
            0006 OP_CLOSURE          1 == '<function method0>'\n\
            0008 OP_METHOD           2 == 'method0'\n\
            0010 OP_CLOSURE          3 == '<function method1>'\n\
            0012 OP_METHOD           4 == 'method1'\n\
            0014 OP_CLOSURE          5 == '<function method2>'\n\
            0016 OP_METHOD           6 == 'method2'\n\
            0018 OP_CLOSURE          7 == '<function method3>'\n\
            0020 OP_METHOD           8 == 'method3'\n\
            0022 OP_CLOSURE          9 == '<function method4>'\n\
            0024 OP_METHOD          10 == 'method4'\n\
            0026 OP_CLOSURE         11 == '<function method5>'\n\
            0028 OP_METHOD          12 == 'method5'\n\
            0030 OP_CLOSURE         13 == '<function method6>'\n\
            0032 OP_METHOD          14 == 'method6'\n\
            0034 OP_CLOSURE         15 == '<function method7>'\n\
            0036 OP_METHOD          16 == 'method7'\n\
            0038 OP_CLOSURE         17 == '<function method8>'\n\
            0040 OP_METHOD          18 == 'method8'\n\
            0042 OP_POP\n\
            0043 OP_GET_GLOBAL       0 == 'Foo'\n\
            0045 OP_CALL             0\n\
            0047 OP_DEFINE_GLOBAL   19 == 'foo'\n\
            0049 OP_GET_GLOBAL      19 == 'foo'\n\
            0051 OP_INVOKE        (0 args)    2 'method0'\n\
            0054 OP_PRINT\n\
            0055 OP_GET_GLOBAL      19 == 'foo'\n\
            0057 OP_GET_PROPERTY     4 == 'method1'\n\
            0059 OP_CONSTANT        20 == '1'\n\
            0061 OP_CALL             1\n\
            0063 OP_PRINT\n\
            0064 OP_GET_GLOBAL      19 == 'foo'\n\
            0066 OP_GET_PROPERTY     6 == 'method2'\n\
            0068 OP_CONSTANT        20 == '1'\n\
            0070 OP_CONSTANT        21 == '2'\n\
            0072 OP_CALL             2\n\
            0074 OP_PRINT\n\
            0075 OP_GET_GLOBAL      19 == 'foo'\n\
            0077 OP_GET_PROPERTY     8 == 'method3'\n\
            0079 OP_CONSTANT        20 == '1'\n\
            0081 OP_CONSTANT        21 == '2'\n\
            0083 OP_CONSTANT        22 == '3'\n\
            0085 OP_CALL             3\n\
            0087 OP_PRINT\n\
            0088 OP_GET_GLOBAL      19 == 'foo'\n\
            0090 OP_GET_PROPERTY    10 == 'method4'\n\
            0092 OP_CONSTANT        20 == '1'\n\
            0094 OP_CONSTANT        21 == '2'\n\
            0096 OP_CONSTANT        22 == '3'\n\
            0098 OP_CONSTANT        23 == '4'\n\
            0100 OP_CALL             4\n\
            0102 OP_PRINT\n\
            0103 OP_GET_GLOBAL      19 == 'foo'\n\
            0105 OP_GET_PROPERTY    12 == 'method5'\n\
            0107 OP_CONSTANT        20 == '1'\n\
            0109 OP_CONSTANT        21 == '2'\n\
            0111 OP_CONSTANT        22 == '3'\n\
            0113 OP_CONSTANT        23 == '4'\n\
            0115 OP_CONSTANT        24 == '5'\n\
            0117 OP_CALL             5\n\
            0119 OP_PRINT\n\
            0120 OP_GET_GLOBAL      19 == 'foo'\n\
            0122 OP_GET_PROPERTY    14 == 'method6'\n\
            0124 OP_CONSTANT        20 == '1'\n\
            0126 OP_CONSTANT        21 == '2'\n\
            0128 OP_CONSTANT        22 == '3'\n\
            0130 OP_CONSTANT        23 == '4'\n\
            0132 OP_CONSTANT        24 == '5'\n\
            0134 OP_CONSTANT        25 == '6'\n\
            0136 OP_CALL             6\n\
            0138 OP_PRINT\n\
            0139 OP_GET_GLOBAL      19 == 'foo'\n\
            0141 OP_GET_PROPERTY    16 == 'method7'\n\
            0143 OP_CONSTANT        20 == '1'\n\
            0145 OP_CONSTANT        21 == '2'\n\
            0147 OP_CONSTANT        22 == '3'\n\
            0149 OP_CONSTANT        23 == '4'\n\
            0151 OP_CONSTANT        24 == '5'\n\
            0153 OP_CONSTANT        25 == '6'\n\
            0155 OP_CONSTANT        26 == '7'\n\
            0157 OP_CALL             7\n\
            0159 OP_PRINT\n\
            0160 OP_GET_GLOBAL      19 == 'foo'\n\
            0162 OP_GET_PROPERTY    18 == 'method8'\n\
            0164 OP_CONSTANT        20 == '1'\n\
            0166 OP_CONSTANT        21 == '2'\n\
            0168 OP_CONSTANT        22 == '3'\n\
            0170 OP_CONSTANT        23 == '4'\n\
            0172 OP_CONSTANT        24 == '5'\n\
            0174 OP_CONSTANT        25 == '6'\n\
            0176 OP_CONSTANT        26 == '7'\n\
            0178 OP_CONSTANT        27 == '8'\n\
            0180 OP_CALL             8\n\
            0182 OP_PRINT\n\
            0183 OP_NIL\n\
            0184 OP_RETURN\n"
        ),
    }
}
