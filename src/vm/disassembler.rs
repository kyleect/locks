use super::chunk::Chunk;
use super::op;

/// Disassemble byte code chunks in to text
pub struct Disassembler<'a> {
    chunk: &'a Chunk,
}

impl<'a> Disassembler<'a> {
    pub fn new(chunk: &Chunk) -> Disassembler<'_> {
        Disassembler { chunk }
    }

    /// Visualize the byte code as text
    pub fn disassemble(&self, level: Option<usize>) -> String {
        let level = level.unwrap_or(0);

        let mut out = String::new();

        let mut op_idx = 0;

        while op_idx < self.chunk.ops.len() {
            let (op_byte_size, disassembled_byte_idx, disassembled_op) =
                self.disassemble_op(op_idx, level);

            let spacer = if level == 0 {
                String::new()
            } else {
                let s = if level == 1 { "" } else { " " };
                format!("{space:width$}{bar} ", space = s, bar = "|", width = (level - 1) * 2)
            };

            let op_string = &format!("{spacer}{disassembled_byte_idx} {disassembled_op}");
            out.push_str(op_string);

            op_idx += op_byte_size;
        }

        out
    }

    pub fn disassemble_op(&self, op_idx: usize, level: usize) -> (usize, String, String) {
        let op_idx_str = format!("{op_idx:04}");

        let spacer = if level == 0 {
            String::new()
        } else {
            let s = if level == 1 { "" } else { " " };
            format!("{space:width$}{bar} ", space = s, bar = "|", width = (level - 1) * 2)
        };

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
            op::MODULUS => self.disassemble_op_simple("OP_MODULUS"),
            op::NOT => self.disassemble_op_simple("OP_NOT"),
            op::NEGATE => self.disassemble_op_simple("OP_NEGATE"),
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
                        "{spacer}{offset:04} CAPTURE [{label} -> {upvalue_idx}]\n"
                    ));
                }

                let result =
                    Disassembler::new(unsafe { &(*function).chunk }).disassemble(Some(level + 1));

                closure_str.push_str(&result);

                op_idx_b += 1;

                let op_idx_inc = op_idx_b - op_idx;

                (op_idx_inc, closure_str)
            }
            op::CLOSE_UPVALUE => self.disassemble_op_simple("OP_CLOSE_UPVALUE"),
            op::RETURN => self.disassemble_op_simple("OP_RETURN"),
            op::CLASS => self.disassemble_op_constant("OP_CLASS", op_idx),
            op::INHERIT => self.disassemble_op_simple("OP_INHERIT"),
            op::CREATE_LIST => self.disassemble_op_byte("OP_CREATE_LIST", op_idx),
            op::GET_INDEX => self.disassemble_op_simple("OP_GET_INDEX"),
            op::SET_INDEX => self.disassemble_op_simple("OP_SET_INDEX"),
            op::METHOD => self.disassemble_op_constant("OP_METHOD", op_idx),
            op::PACKAGE => self.disassemble_op_constant("OP_PACKAGE", op_idx),
            op::FIELD => self.disassemble_op_constant("OP_FIELD", op_idx),
            op::STATIC_FIELD => self.disassemble_op_constant("OP_STATIC_FIELD", op_idx),
            op::STATIC_METHOD => self.disassemble_op_constant("OP_STATIC_METHOD", op_idx),
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
    use super::Disassembler;
    use crate::vm::{Compiler, Gc};

    parameterized_test::create! { assert_disassembly, (code, disassembly), {
        let mut gc = Gc::default();

        let program = match crate::syntax::parse(&code, code.len()) {
            Ok(program) => program,
            Err(error) => {
                panic!("There was a parsing error! {:?}", error);
            }
        };

        match Compiler::compile(&program, &mut gc) {
            Ok(function) => {
                let chunk = unsafe { &(*function).chunk };

                let d = Disassembler { chunk: &chunk };

                let result = d.disassemble(None);

                assert_eq!(result, disassembly);
            },
            Err(errors) => {
                assert!(false, "There was a compile error! {:?}", errors);
            }
        }
    }}

    assert_disassembly! {
        list_get_index: (
            "[1, 2][0];",
            "\
            0000 OP_CONSTANT         0 == '1'\n\
            0002 OP_CONSTANT         1 == '2'\n\
            0004 OP_CREATE_LIST      2\n\
            0006 OP_CONSTANT         2 == '0'\n\
            0008 OP_GET_INDEX\n\
            0009 OP_POP\n\
            0010 OP_NIL\n\
            0011 OP_RETURN\n"
        ),
        list_set_index: (
            "[1, 2][0] = 2;",
            "\
            0000 OP_CONSTANT         0 == '1'\n\
            0002 OP_CONSTANT         1 == '2'\n\
            0004 OP_CREATE_LIST      2\n\
            0006 OP_CONSTANT         2 == '0'\n\
            0008 OP_CONSTANT         1 == '2'\n\
            0010 OP_SET_INDEX\n\
            0011 OP_POP\n\
            0012 OP_NIL\n\
            0013 OP_RETURN\n"
        ),
        list: (
            "[1, 2, 5];",
            "\
            0000 OP_CONSTANT         0 == '1'\n\
            0002 OP_CONSTANT         1 == '2'\n\
            0004 OP_CONSTANT         2 == '5'\n\
            0006 OP_CREATE_LIST      3\n\
            0008 OP_POP\n\
            0009 OP_NIL\n\
            0010 OP_RETURN\n"
        ),
        assignment_string: (
            "let a = \"Hello\";",
            "\
            0000 OP_CONSTANT         0 == 'Hello'\n\
            0002 OP_DEFINE_GLOBAL    1 == 'a'\n\
            0004 OP_NIL\n\
            0005 OP_RETURN\n"
        ),
        assignment_number: (
            "let a = 123;",
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
        modulus: (
            "10 % 2;",
            "\
            0000 OP_CONSTANT         0 == '10'\n\
            0002 OP_CONSTANT         1 == '2'\n\
            0004 OP_MODULUS\n\
            0005 OP_POP\n\
            0006 OP_NIL\n\
            0007 OP_RETURN\n"
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
            let a = 123;\n\
            let b = -a;\n\
            let c = a + b;",
            concat!(
                "0000 OP_CONSTANT         0 == '123'\n",
                "0002 OP_DEFINE_GLOBAL    1 == 'a'\n",
                "0004 OP_GET_GLOBAL       1 == 'a'\n",
                "0006 OP_NEGATE\n",
                "0007 OP_DEFINE_GLOBAL    2 == 'b'\n",
                "0009 OP_GET_GLOBAL       1 == 'a'\n",
                "0011 OP_GET_GLOBAL       2 == 'b'\n",
                "0013 OP_ADD\n",
                "0014 OP_DEFINE_GLOBAL    3 == 'c'\n",
                "0016 OP_NIL\n",
                "0017 OP_RETURN\n"
            )
        ),
        fn_define_empty: (
            "fn sum (a, b) { }",
            concat!(
                "0000 OP_CLOSURE          0 == '<fn sum arity=2>'\n",
                "| 0000 OP_NIL\n",
                "| 0001 OP_RETURN\n",
                "0002 OP_DEFINE_GLOBAL    1 == 'sum'\n",
                "0004 OP_NIL\n",
                "0005 OP_RETURN\n"
            )
        ),
        fn_define_empty_no_args: (
            "fn sum () { }",
            concat!(
                "0000 OP_CLOSURE          0 == '<fn sum arity=0>'\n",
                "| 0000 OP_NIL\n",
                "| 0001 OP_RETURN\n",
                "0002 OP_DEFINE_GLOBAL    1 == 'sum'\n",
                "0004 OP_NIL\n",
                "0005 OP_RETURN\n"
            )
        ),
        fn_define: (
            "fn sum (a, b) { return a + b; }",
            concat!(
                "0000 OP_CLOSURE          0 == '<fn sum arity=2>'\n",
                "| 0000 OP_GET_LOCAL        1\n",
                "| 0002 OP_GET_LOCAL        2\n",
                "| 0004 OP_ADD\n",
                "| 0005 OP_RETURN\n",
                "0002 OP_DEFINE_GLOBAL    1 == 'sum'\n",
                "0004 OP_NIL\n",
                "0005 OP_RETURN\n"
            )
        ),
        fn_call: (
            "\
            fn sum (a, b) { return a + b; }\n\
            sum(100, 200);",
            concat!(
                "0000 OP_CLOSURE          0 == '<fn sum arity=2>'\n",
                "| 0000 OP_GET_LOCAL        1\n",
                "| 0002 OP_GET_LOCAL        2\n",
                "| 0004 OP_ADD\n",
                "| 0005 OP_RETURN\n",
                "0002 OP_DEFINE_GLOBAL    1 == 'sum'\n",
                "0004 OP_GET_GLOBAL       1 == 'sum'\n",
                "0006 OP_CONSTANT         2 == '100'\n",
                "0008 OP_CONSTANT         3 == '200'\n",
                "0010 OP_CALL             2\n",
                "0012 OP_POP\n",
                "0013 OP_NIL\n",
                "0014 OP_RETURN\n"
            )
        ),
        jump_to_false: (
            "\
            if (false) {
                println(1);
            } else {
                println(2);
            }",
            concat!(
                "0000 OP_FALSE\n",
                "0001 OP_JUMP_IF_FALSE    1 -> 15\n",
                "0004 OP_POP\n",
                "0005 OP_GET_GLOBAL       0 == 'println'\n",
                "0007 OP_CONSTANT         1 == '1'\n",
                "0009 OP_CALL             1\n",
                "0011 OP_POP\n",
                "0012 OP_JUMP            12 -> 23\n",
                "0015 OP_POP\n",
                "0016 OP_GET_GLOBAL       0 == 'println'\n",
                "0018 OP_CONSTANT         2 == '2'\n",
                "0020 OP_CALL             1\n",
                "0022 OP_POP\n",
                "0023 OP_NIL\n",
                "0024 OP_RETURN\n"
            )
        ),
        closure: (
            r#"
            let f;

            fn foo(param) {
                fn f_() {
                    return param;
                }
                
                f = f_;
            }
            
            foo("param");

            f(); // out: param
            "#,
            concat!(
                "0000 OP_NIL\n",
                "0001 OP_DEFINE_GLOBAL    0 == 'f'\n",
                "0003 OP_CLOSURE          1 == '<fn foo arity=1>'\n",
                "| 0000 OP_CLOSURE          0 == '<fn f_ arity=0>'\n",
                "| 0001 CAPTURE [local -> 1]\n",
                "  | 0000 OP_GET_UPVALUE      0\n",
                "  | 0002 OP_RETURN\n",
                "| 0004 OP_GET_LOCAL        2\n",
                "| 0006 OP_SET_GLOBAL       1 == 'f'\n",
                "| 0008 OP_POP\n",
                "| 0009 OP_NIL\n",
                "| 0010 OP_RETURN\n",
                "0005 OP_DEFINE_GLOBAL    2 == 'foo'\n",
                "0007 OP_GET_GLOBAL       2 == 'foo'\n",
                "0009 OP_CONSTANT         3 == 'param'\n",
                "0011 OP_CALL             1\n",
                "0013 OP_POP\n",
                "0014 OP_GET_GLOBAL       0 == 'f'\n",
                "0016 OP_CALL             0\n",
                "0018 OP_POP\n",
                "0019 OP_NIL\n",
                "0020 OP_RETURN\n",
            ),
        ),
        class_init_and_method_call: (
            "\
            class Greeter {
              let greeting;

              fn init(greeting) {
                this.greeting = greeting;
              }
              
              fn greet(name) {
                return this.greeting + \" \" + name;
              }
            }
              
            let greeter = Greeter(\"Hello\");
              
            println(greeter.greet(\"World\")); // out: Hello World",
            concat!(
                "0000 OP_CLASS            0 == 'Greeter'\n0002 OP_DEFINE_GLOBAL    0 == 'Greeter'\n0004 OP_GET_GLOBAL       1 == 'Object'\n0006 OP_GET_GLOBAL       0 == 'Greeter'\n0008 OP_INHERIT\n0009 OP_GET_GLOBAL       0 == 'Greeter'\n0011 OP_NIL\n0012 OP_FIELD            2 == 'greeting'\n0014 OP_POP\n0015 OP_GET_GLOBAL       0 == 'Greeter'\n0017 OP_CLOSURE          3 == '<fn init arity=1>'\n| 0000 OP_GET_LOCAL        1\n| 0002 OP_GET_LOCAL        0\n| 0004 OP_SET_PROPERTY     0 == 'greeting'\n| 0006 OP_POP\n| 0007 OP_GET_LOCAL        0\n| 0009 OP_RETURN\n0019 OP_METHOD           4 == 'init'\n0021 OP_CLOSURE          5 == '<fn greet arity=1>'\n| 0000 OP_GET_LOCAL        0\n| 0002 OP_GET_PROPERTY     0 == 'greeting'\n| 0004 OP_CONSTANT         1 == ' '\n| 0006 OP_ADD\n| 0007 OP_GET_LOCAL        1\n| 0009 OP_ADD\n| 0010 OP_RETURN\n0023 OP_METHOD           6 == 'greet'\n0025 OP_POP\n0026 OP_POP\n0027 OP_GET_GLOBAL       0 == 'Greeter'\n0029 OP_CONSTANT         7 == 'Hello'\n0031 OP_CALL             1\n0033 OP_DEFINE_GLOBAL    8 == 'greeter'\n0035 OP_GET_GLOBAL       9 == 'println'\n0037 OP_GET_GLOBAL       8 == 'greeter'\n0039 OP_GET_PROPERTY     6 == 'greet'\n0041 OP_CONSTANT        10 == 'World'\n0043 OP_CALL             1\n0045 OP_CALL             1\n0047 OP_POP\n0048 OP_NIL\n0049 OP_RETURN\n"
            )
        ),
        class_static_field: (
            "\
            class Test {
              static let value = 100;
            }
              
            println(Test.value); // out: 100",
            concat!(
                "0000 OP_CLASS            0 == 'Test'\n0002 OP_DEFINE_GLOBAL    0 == 'Test'\n0004 OP_GET_GLOBAL       1 == 'Object'\n0006 OP_GET_GLOBAL       0 == 'Test'\n0008 OP_INHERIT\n0009 OP_GET_GLOBAL       0 == 'Test'\n0011 OP_CONSTANT         2 == '100'\n0013 OP_STATIC_FIELD     3 == 'value'\n0015 OP_POP\n0016 OP_POP\n0017 OP_GET_GLOBAL       4 == 'println'\n0019 OP_GET_GLOBAL       0 == 'Test'\n0021 OP_GET_PROPERTY     3 == 'value'\n0023 OP_CALL             1\n0025 OP_POP\n0026 OP_NIL\n0027 OP_RETURN\n"
            )
        ),
    }
}
