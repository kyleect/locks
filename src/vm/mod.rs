mod allocator;
mod chunk;
mod compiler;
mod disassembler;
mod gc;
mod object;
mod op;
mod util;
mod value;

use std::hash::BuildHasherDefault;
use std::io::Write;
use std::{mem, ptr};

use arrayvec::ArrayVec;
pub use compiler::Compiler;
pub use disassembler::Disassembler;
pub use gc::Gc;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use rustc_hash::FxHasher;

use crate::error::{
    AttributeError, Error, ErrorS, IndexError, IoError, NameError, OverflowError, Result, TypeError,
};
use crate::vm::allocator::GLOBAL;
use crate::vm::gc::GcAlloc;
use crate::vm::object::{
    Native, ObjectBoundMethod, ObjectClass, ObjectClosure, ObjectFunction, ObjectInstance,
    ObjectList, ObjectNative, ObjectPackage, ObjectString, ObjectType, ObjectUpvalue,
};
use crate::vm::value::Value;

const GC_HEAP_GROW_FACTOR: usize = 2;
const FRAMES_MAX: usize = 64;
const STACK_MAX: usize = FRAMES_MAX * STACK_MAX_PER_FRAME;
const STACK_MAX_PER_FRAME: usize = u8::MAX as usize + 1;

#[derive(Debug)]
pub struct VM {
    pub globals: HashMap<*mut ObjectString, Value, BuildHasherDefault<FxHasher>>,
    pub open_upvalues: Vec<*mut ObjectUpvalue>,

    pub gc: Gc,
    next_gc: usize,

    /// `frames` is the current stack of frames running in the [`VM`].
    ///
    /// The topmost frame points to the currently running closure, but does not
    /// include a valid instruction pointer / stack pointer.
    frames: ArrayVec<CallFrame, FRAMES_MAX>,
    frame: CallFrame,

    /// `stack` can be safely accessed without bounds checking because:
    /// - Each frame can store a theoretical maximum of `STACK_MAX_PER_FRAME`
    ///   values on the stack.
    /// - The frame count can never exceed `MAX_FRAMES`, otherwise we throw a
    ///   stack overflow error.
    /// - Thus, we can statically allocate a stack of size
    ///   `STACK_MAX = FRAMES_MAX * STACK_MAX_PER_FRAME` and we are
    ///   guaranteed to never exceed this size.
    stack: Box<[Value; STACK_MAX]>,
    /// Pointer to the next empty slot in the stack
    stack_top: *mut Value,

    // String allocated for the "init" constructor method on classes
    init_string: *mut ObjectString,
    pub source: String,
}

impl VM {
    pub fn run(&mut self, source: &str, stdout: &mut impl Write) -> Result<(), Vec<ErrorS>> {
        // This will change with each call to `run`
        let offset = self.source.len();

        // Add current source to self.source
        // This helps us keep track of what the offset should be on future calls to `run`
        self.source.reserve(source.len() + 1);
        self.source.push_str(source);
        self.source.push('\n');

        let program = crate::syntax::parse(source, offset)?;

        let function = Compiler::compile(&program, &mut self.gc)?;

        #[cfg(feature = "pprof")]
        let guard = pprof::ProfilerGuardBuilder::default()
            .blocklist(&["libc", "libgcc", "pthread", "vdso"])
            .build()
            .expect("could not start pprof");

        self.run_function(function, stdout).map_err(|e| vec![e])?;

        #[cfg(feature = "pprof")]
        {
            let report = guard.report().build().expect("error generating profiler report");
            let file =
                std::fs::File::create("flamegraph.svg").expect("could not create flamegraph file");
            report.flamegraph(file).expect("error writing to flamegraph file");

            let profile = report.pprof().expect("error generating pprof report");
            let mut content = Vec::new();
            pprof::protos::Message::encode(&profile, &mut content)
                .expect("error encoding pprof report");
            std::fs::write("profile.pb", content).expect("error writing pprof report to file");
        }

        Ok(())
    }

    /// Run a compiled function from the compiler
    ///
    /// This is only called by vm.run
    fn run_function(
        &mut self,
        function: *mut ObjectFunction,
        stdout: &mut impl Write,
    ) -> Result<()> {
        self.stack_top = self.stack.as_mut_ptr();

        self.frames.clear();

        self.frame = CallFrame {
            closure: self.gc.alloc(ObjectClosure::new(function, Vec::new())),
            ip: unsafe { (*function).chunk.ops.as_ptr() },
            stack: self.stack_top,
        };

        let disassembler = Disassembler::new(unsafe { &(*function).chunk });

        loop {
            if cfg!(feature = "vm-trace") {
                let function = unsafe { (*self.frame.closure).function };
                let idx = unsafe { self.frame.ip.offset_from((*function).chunk.ops.as_ptr()) };
                let (_, op_idx_str, op_str) = disassembler.disassemble_op(idx as usize, 0);
                println!("{} {}", op_idx_str, op_str);
            }

            match self.read_u8() {
                op::CONSTANT => self.op_constant(),
                op::NIL => self.op_nil(),
                op::TRUE => self.op_true(),
                op::FALSE => self.op_false(),
                op::POP => self.op_pop(),
                op::GET_LOCAL => self.op_get_local(),
                op::SET_LOCAL => self.op_set_local(),
                op::GET_GLOBAL => self.op_get_global(),
                op::DEFINE_GLOBAL => self.op_define_global(),
                op::SET_GLOBAL => self.op_set_global(),
                op::GET_UPVALUE => self.op_get_upvalue(),
                op::SET_UPVALUE => self.op_set_upvalue(),
                op::GET_PROPERTY => self.op_get_property(),
                op::SET_PROPERTY => self.op_set_property(),
                op::GET_SUPER => self.op_get_super(),
                op::EQUAL => self.op_equal(),
                op::NOT_EQUAL => self.op_not_equal(),
                op::GREATER => self.op_greater(),
                op::GREATER_EQUAL => self.op_greater_equal(),
                op::LESS => self.op_less(),
                op::LESS_EQUAL => self.op_less_equal(),
                op::ADD => self.op_add(),
                op::SUBTRACT => self.op_subtract(),
                op::MULTIPLY => self.op_multiply(),
                op::DIVIDE => self.op_divide(),
                op::MODULUS => self.op_modulus(),
                op::NOT => self.op_not(),
                op::NEGATE => self.op_negate(),
                op::JUMP => self.op_jump(),
                op::JUMP_IF_FALSE => self.op_jump_if_false(),
                op::LOOP => self.op_loop(),
                op::CALL => self.op_call(stdout),
                op::INVOKE => self.op_invoke(stdout),
                op::SUPER_INVOKE => self.op_super_invoke(),
                op::CLOSURE => self.op_closure(),
                op::CLOSE_UPVALUE => self.op_close_upvalue(),
                op::RETURN => {
                    let value = self.pop();
                    self.close_upvalues(self.frame.stack);

                    self.stack_top = self.frame.stack;
                    match self.frames.pop() {
                        Some(frame) => self.frame = frame,
                        None => break,
                    }
                    self.push(value);

                    Ok(())
                }
                op::CLASS => self.op_class(),
                op::INHERIT => self.op_inherit(),
                op::METHOD => self.op_method(),
                op::FIELD => self.op_field(),
                op::CREATE_LIST => self.op_create_list(),
                op::GET_INDEX => self.op_get_index(),
                op::SET_INDEX => self.op_set_index(),
                op::PACKAGE => self.op_package(),
                _ => util::unreachable(),
            }?;

            if cfg!(feature = "vm-trace") {
                eprint!("     ");
                let mut stack_ptr = self.frame.stack;
                while stack_ptr < self.stack_top {
                    eprint!("[ {} ]", unsafe { *stack_ptr });
                    stack_ptr = unsafe { stack_ptr.add(1) };
                }
                eprintln!();
            }
        }

        debug_assert_eq!(
            self.frame.stack, self.stack_top,
            "VM finished executing but stack is not empty"
        );
        Ok(())
    }

    fn op_create_list(&mut self) -> Result<()> {
        let length = self.read_u8();

        let mut values = vec![];

        for _ in 0..length {
            let value = self.pop();

            values.push(value);
        }

        values.reverse();

        let list = self.gc.alloc(ObjectList::new(values));
        let value = list.into();

        self.push(value);

        Ok(())
    }

    fn op_get_index(&mut self) -> Result<()> {
        let index = self.pop();
        let target = self.pop();

        if target.is_object() {
            if target.as_object().type_() == ObjectType::List {
                let list = unsafe { &(*target.as_object().list) };
                let list_idx = index.as_number() as usize;

                if list_idx >= list.values.len() {
                    return self.err(IndexError::OutOfBounds {
                        wanted_index: list_idx,
                        length: list.values.len(),
                    });
                }

                let value = list.values[list_idx];
                self.push(value);
            } else {
                return self.err(TypeError::NotIndexable {
                    type_: target.as_object().type_().to_string(),
                });
            }
        } else {
            return self.err(TypeError::NotIndexable { type_: target.type_().to_string() });
        }

        Ok(())
    }

    fn op_package(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };

        if cfg!(feature = "vm-trace") {
            eprintln!("Pushing package on to stack");
        }

        let package = self.alloc(ObjectPackage::new(name)).into();

        if cfg!(feature = "vm-trace") {
            eprintln!("Pushed package on to stack");
        }

        self.push(package);

        Ok(())
    }

    fn op_set_index(&mut self) -> Result<()> {
        let value = self.pop();
        let index = self.pop();
        let target = self.pop();

        if target.is_object() {
            if target.as_object().type_() == ObjectType::List {
                let list = unsafe { &mut (*target.as_object().list) };
                let list_idx = index.as_number() as usize;

                if list_idx >= list.values.len() {
                    return self.err(IndexError::OutOfBounds {
                        wanted_index: list_idx,
                        length: list.values.len(),
                    });
                }

                list.values[list_idx] = value;
                self.push(value);
            } else {
                return self.err(TypeError::NotIndexable { type_: target.type_().to_string() });
            }
        }

        Ok(())
    }

    /// Get a constant [`Value`] from the [`CallFrame`]'s constant's table
    /// push it on to the stack.
    ///
    /// The index of the constant is [`CallFrame`]'s instruction pointer (IP)
    ///
    /// Increments current [`CallFrame`]'s IP + 1.
    fn op_constant(&mut self) -> Result<()> {
        let constant = self.read_value();
        self.push(constant);
        Ok(())
    }

    /// Push a `nil` value on to the stack
    fn op_nil(&mut self) -> Result<()> {
        self.push(Value::NIL);
        Ok(())
    }

    /// Push a `true` value on to the stack
    fn op_true(&mut self) -> Result<()> {
        self.push(Value::TRUE);
        Ok(())
    }

    /// Push a `false` value on to the stack
    fn op_false(&mut self) -> Result<()> {
        self.push(Value::FALSE);
        Ok(())
    }

    /// Pop value from the stack
    fn op_pop(&mut self) -> Result<()> {
        self.pop();
        Ok(())
    }

    fn op_get_local(&mut self) -> Result<()> {
        let stack_idx = self.read_u8() as usize;
        let local = unsafe { *self.frame.stack.add(stack_idx) };
        self.push(local);
        Ok(())
    }

    fn op_set_local(&mut self) -> Result<()> {
        let stack_idx = self.read_u8() as usize;
        let local = unsafe { self.frame.stack.add(stack_idx) };
        let value = self.peek(0);
        unsafe { *local = *value };
        Ok(())
    }

    fn op_get_global(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        match self.globals.get(&name) {
            Some(&value) => {
                self.push(value);
                Ok(())
            }
            None => self.err(NameError::NotDefined { name: unsafe { (*name).value.to_string() } }),
        }
    }

    fn op_define_global(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let value = self.pop();

        match self.globals.get(&name) {
            Some(_) => {
                self.err(NameError::AlreadyDefined { name: unsafe { (*name).value.to_string() } })
            }
            None => {
                self.globals.insert(name, value);
                Ok(())
            }
        }
    }

    fn op_set_global(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let value = unsafe { *self.peek(0) };
        match self.globals.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                Ok(())
            }
            Entry::Vacant(_) => {
                self.err(NameError::NotDefined { name: unsafe { (*name).value.to_string() } })
            }
        }
    }

    fn op_get_upvalue(&mut self) -> Result<()> {
        let upvalue_idx = self.read_u8() as usize;
        let object = *unsafe { (*self.frame.closure).upvalues.get_unchecked(upvalue_idx) };
        let value = unsafe { *(*object).location };
        self.push(value);
        Ok(())
    }

    fn op_set_upvalue(&mut self) -> Result<()> {
        let upvalue_idx = self.read_u8() as usize;
        let object = *unsafe { (*self.frame.closure).upvalues.get_unchecked(upvalue_idx) };
        let value = unsafe { (*object).location };
        unsafe { *value = *self.peek(0) };
        Ok(())
    }

    fn op_get_property(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let instance = {
            let value = unsafe { *self.peek(0) };
            let object = value.as_object();

            if value.is_object() && object.type_() == ObjectType::Instance {
                unsafe { object.instance }
            } else {
                return self.err(AttributeError::NoSuchAttribute {
                    type_: value.type_().to_string(),
                    name: unsafe { (*name).value.to_string() },
                });
            }
        };

        match unsafe { (*instance).fields.get(&name) } {
            Some(&field) => {
                self.pop();
                self.push(field);
            }
            None => match unsafe { (*(*instance).class).methods.get(&name) } {
                Some(&method) => {
                    let bound_method = self.alloc(ObjectBoundMethod::new(instance, method));
                    self.pop();
                    self.push(bound_method.into());
                }
                None => {
                    return self.err(AttributeError::NoSuchAttribute {
                        type_: unsafe { (*(*(*instance).class).name).value.to_string() },
                        name: unsafe { (*name).value.to_string() },
                    });
                }
            },
        }

        Ok(())
    }

    fn op_set_property(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let instance = {
            let value = self.pop();
            let object = value.as_object();

            if value.is_object() && object.type_() == ObjectType::Instance {
                unsafe { object.instance }
            } else {
                return self.err(AttributeError::NoSuchAttribute {
                    type_: value.type_().to_string(),
                    name: unsafe { (*name).value.to_string() },
                });
            }
        };
        let value = unsafe { *self.peek(0) };
        let has_field = unsafe { (*instance).fields.get(&name) };

        if let Some(_) = has_field {
            unsafe { (*instance).fields.insert(name, value) };
            return Ok(());
        }

        let class_has_method = unsafe { (*(*instance).class).methods.get(&name) };

        if let Some(_) = class_has_method {
            return self.err(TypeError::InvalidMethodAssignment {
                name: unsafe { (*name).value.to_owned() },
                type_: unsafe { (*(*(*instance).class).name).value.to_owned() },
            });
        }

        self.err(AttributeError::NoSuchAttribute {
            type_: unsafe { (*(*(*instance).class).name).value.to_string() },
            name: unsafe { (*name).value.to_string() },
        })
    }

    fn op_get_super(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let super_ = unsafe { self.pop().as_object().class };
        match unsafe { (*super_).methods.get(&name) } {
            Some(&method) => {
                let instance = unsafe { (*self.peek(0)).as_object().instance };
                let bound_method = self.alloc(ObjectBoundMethod::new(instance, method));
                self.pop();
                self.push(bound_method.into());
            }
            None => {
                return self.err(AttributeError::NoSuchAttribute {
                    type_: unsafe { (*(*super_).name).value.to_string() },
                    name: unsafe { (*name).value.to_string() },
                });
            }
        }
        Ok(())
    }

    fn op_equal(&mut self) -> Result<()> {
        self.binary_op(|a, b| Value::from(a == b));
        Ok(())
    }

    fn op_not_equal(&mut self) -> Result<()> {
        self.binary_op(|a, b| Value::from(a != b));
        Ok(())
    }

    fn op_greater(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a > b), ">")
    }

    fn op_greater_equal(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a >= b), ">=")
    }

    fn op_less(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a < b), "<")
    }

    fn op_less_equal(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a <= b), "<=")
    }

    /// Pop last two values from the stack and add them
    ///
    /// Both values must be of the same type
    ///
    /// Both values must be either numbers or strings
    fn op_add(&mut self) -> Result<()> {
        let b = self.pop();
        let a = self.pop();

        if a.is_number() && b.is_number() {
            self.push((a.as_number() + b.as_number()).into());
            return Ok(());
        }

        if a.is_object() && b.is_object() {
            let a = a.as_object();
            let b = b.as_object();

            if a.type_() == ObjectType::String && b.type_() == ObjectType::String {
                let result = unsafe { [(*a.string).value, (*b.string).value] }.concat();
                let result = Value::from(self.alloc(result));
                self.push(result);
                return Ok(());
            }
        }

        self.err(TypeError::UnsupportedOperandInfix {
            op: "+".to_string(),
            lt_type: a.type_().to_string(),
            rt_type: b.type_().to_string(),
        })
    }

    fn op_subtract(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a - b), "-")
    }

    fn op_multiply(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a * b), "*")
    }

    fn op_divide(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a / b), "/")
    }

    fn op_modulus(&mut self) -> Result<()> {
        self.binary_op_number(|a, b| Value::from(a % b), "%")
    }

    fn op_not(&mut self) -> Result<()> {
        let value = self.pop();
        self.push(!value);
        Ok(())
    }

    fn op_negate(&mut self) -> Result<()> {
        let value = self.pop();
        if value.is_number() {
            self.push(Value::from(-value.as_number()));
            Ok(())
        } else {
            self.err(TypeError::UnsupportedOperandPrefix {
                op: "-".to_string(),
                rt_type: value.type_().to_string(),
            })
        }
    }

    fn op_jump(&mut self) -> Result<()> {
        let offset = self.read_u16() as usize;
        self.frame.ip = unsafe { self.frame.ip.add(offset) };
        Ok(())
    }

    fn op_jump_if_false(&mut self) -> Result<()> {
        let offset = self.read_u16() as usize;
        let value = self.peek(0);
        if !(unsafe { *value }.to_bool()) {
            self.frame.ip = unsafe { self.frame.ip.add(offset) };
        }
        Ok(())
    }

    fn op_loop(&mut self) -> Result<()> {
        let offset = self.read_u16() as usize;
        self.frame.ip = unsafe { self.frame.ip.sub(offset) };
        Ok(())
    }

    /// Attempt to call [`Value`] on the top of the VM's stack as callable
    ///
    /// [`Value`] must be an [`Object`] of one of the following [`ObjectType`]:
    ///
    /// - [`ObjectType::BoundMethod`]
    /// - [`ObjectType::Class`]
    /// - [`ObjectType::Closure`]
    /// - [`ObjectType::Native`]
    ///
    /// Will return [`Err(TypeError::NotCallable)`] otherwise.
    ///
    /// This consumes 1 byte op for the `arg_count`
    fn op_call(&mut self, stdout: &mut impl Write) -> Result<()> {
        let arg_count = self.read_u8() as usize;
        let callee = unsafe { *self.peek(arg_count) };
        self.call_value(callee, arg_count, stdout)
    }

    fn op_invoke(&mut self, stdout: &mut impl Write) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let arg_count = self.read_u8() as usize;
        let instance = unsafe { (*self.peek(arg_count)).as_object().instance };

        match unsafe { (*instance).fields.get(&name) } {
            Some(&value) => self.call_value(value, arg_count, stdout),
            None => match unsafe { (*(*instance).class).methods.get(&name) } {
                Some(&method) => self.call_closure(method, arg_count),
                None => self.err(AttributeError::NoSuchAttribute {
                    type_: unsafe { (*(*(*instance).class).name).value.to_string() },
                    name: unsafe { (*name).value.to_string() },
                }),
            },
        }
    }

    fn op_super_invoke(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let arg_count = self.read_u8() as usize;
        let super_ = unsafe { self.pop().as_object().class };

        match unsafe { (*super_).methods.get(&name) } {
            Some(&method) => self.call_closure(method, arg_count),
            None => self.err(AttributeError::NoSuchAttribute {
                type_: unsafe { (*(*super_).name).value.to_string() },
                name: unsafe { (*name).value.to_string() },
            }),
        }
    }

    /// Create an [`ObjectClosure`] from the [`ObjectFunction`] ([`Value`]) at the
    /// VM's instruction pointer (IP).
    ///
    /// This also captures the required [`ObjectUpvalue`]s as defined by the [`ObjectFunction`]'s
    /// `upvalue_count`. The values to capture are stored in the VM's `open_upvalues`.
    ///
    /// The [`ObjectClosure`] is pushed on to the VM's stack.
    fn op_closure(&mut self) -> Result<()> {
        let function = unsafe { self.read_value().as_object().function };

        let upvalue_count = unsafe { (*function).upvalue_count } as usize;
        let mut upvalues = Vec::with_capacity(upvalue_count);

        for _ in 0..upvalue_count {
            let is_local = self.read_u8();
            let upvalue_idx = self.read_u8() as usize;

            let upvalue = if is_local != 0 {
                let location = unsafe { self.frame.stack.add(upvalue_idx) };
                self.capture_upvalue(location)
            } else {
                unsafe { *(*self.frame.closure).upvalues.get_unchecked(upvalue_idx) }
            };
            upvalues.push(upvalue);
        }

        let closure = self.alloc(ObjectClosure::new(function, upvalues));
        self.push(closure.into());
        Ok(())
    }

    fn op_close_upvalue(&mut self) -> Result<()> {
        let last = self.peek(0);
        self.close_upvalues(last);
        self.pop();
        Ok(())
    }

    /// Create an [`ObjectClass`] from the [`Value`] at the VM's instruction pointer (IP).
    ///
    /// The [`ObjectClass`] is pushed on to the VM's stack.
    fn op_class(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let class = self.alloc(ObjectClass::new(name)).into();
        self.push(class);
        Ok(())
    }

    /// Marks the [`ObjectClass`] on the top of the VM's stack as a child inheriting from a super/parent class
    ///
    /// This consumes 0 additional ops
    ///
    /// This pops the child [`ObjectClass`] of the VM's stack.
    fn op_inherit(&mut self) -> Result<()> {
        let class = unsafe { self.pop().as_object().class };
        let super_ = {
            let value = unsafe { *self.peek(0) };
            let object = value.as_object();

            if value.is_object() && object.type_() == ObjectType::Class {
                unsafe { object.class }
            } else {
                return self
                    .err(TypeError::SuperclassInvalidType { type_: value.type_().to_string() });
            }
        };

        unsafe { (*class).fields = (*super_).fields.clone() };
        unsafe { (*class).methods = (*super_).methods.clone() };
        Ok(())
    }

    /// Define a field on the [`ObjectClass`] on the top of the VM's stack
    ///
    /// This consumes 2 byte ops for field name, & access modifier
    ///
    /// This pop's the [`Value`] from the VM's stack for the field value.
    fn op_field(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let value = self.pop();
        let class = unsafe { (*self.peek(0)).as_object().class };
        unsafe { (*class).fields.insert(name, value) };
        Ok(())
    }

    /// Define a method on the [`ObjectClass`] on the top of the VM's stack
    ///
    /// This consumes 1 byte op for method name.
    ///
    /// This pop's the [`ObjectClosure`] from the VM's stack for the field value.
    ///
    /// The next [`Value`] on the VM's stack is used as the [`ObjectClass`] for the method.
    fn op_method(&mut self) -> Result<()> {
        let name = unsafe { self.read_value().as_object().string };
        let method = unsafe { self.pop().as_object().closure };
        let class = unsafe { (*self.peek(0)).as_object().class };
        unsafe { (*class).methods.insert(name, method) };
        Ok(())
    }

    /// Allocate memory for object
    fn alloc<T>(&mut self, object: impl GcAlloc<T>) -> T {
        // If gc is on
        if !cfg!(feature = "gc-off")
            // If gc-stress is enabled
            && (cfg!(feature = "gc-stress")
            // Or memory needs to be freed
            || GLOBAL.allocated_bytes() > self.next_gc)
        {
            self.gc();
        }

        // Allocate memory using the garbage collector
        self.gc.alloc(object)
    }

    fn gc(&mut self) {
        if cfg!(feature = "gc-trace") {
            eprintln!("-- gc begin");
        }

        self.gc.mark(self.init_string);

        let mut stack_ptr = self.stack.as_ptr();
        while stack_ptr < self.stack_top {
            self.gc.mark(unsafe { *stack_ptr });
            stack_ptr = unsafe { stack_ptr.add(1) };
        }

        for (&name, &value) in &self.globals {
            self.gc.mark(name);
            self.gc.mark(value);
        }

        self.gc.mark(self.frame.closure);
        for frame in &self.frames {
            self.gc.mark(frame.closure);
        }

        for &upvalue in &self.open_upvalues {
            self.gc.mark(upvalue);
        }

        self.gc.trace();
        self.gc.sweep();

        self.next_gc = GLOBAL.allocated_bytes() * GC_HEAP_GROW_FACTOR;

        if cfg!(feature = "gc-trace") {
            eprintln!("-- gc end");
        }
    }

    /// Attempt to call [`Value`] as callable
    ///
    /// [`Value`] must be an [`Object`] of one of the following [`ObjectType`]:
    ///
    /// - [`ObjectType::BoundMethod`]
    /// - [`ObjectType::Class`]
    /// - [`ObjectType::Closure`]
    /// - [`ObjectType::Native`]
    ///
    /// Will return [`Err(TypeError::NotCallable)`] otherwise.
    fn call_value(
        &mut self,
        value: Value,
        arg_count: usize,
        stdout: &mut impl Write,
    ) -> Result<()> {
        if value.is_object() {
            let object = value.as_object();
            match object.type_() {
                ObjectType::BoundMethod => {
                    self.call_bound_method(unsafe { object.bound_method }, arg_count)
                }
                ObjectType::Class => self.call_class(unsafe { object.class }, arg_count),
                ObjectType::Closure => self.call_closure(unsafe { object.closure }, arg_count),
                ObjectType::Native => self.call_native(unsafe { object.native }, arg_count, stdout),
                _ => self.err(TypeError::NotCallable { type_: value.type_().to_string() }),
            }
        } else {
            self.err(TypeError::NotCallable { type_: value.type_().to_string() })
        }
    }

    /// Call an [`ObjectInstance`]'s method
    ///
    /// This works by setting [`CallFrame`]'s `stack` to the [`ObjectInstance`]
    /// when calling the method's [`ObjectClosure`].
    fn call_bound_method(
        &mut self,
        method: *mut ObjectBoundMethod,
        arg_count: usize,
    ) -> Result<()> {
        // Replace the `OP_GET_PROPERTY` op with the method's bound
        // [`ObjectInstance`] (e.g. `this`). This is used below.
        unsafe { *self.peek(arg_count) = (*method).this.into() };

        // The [`ObjectInstance`] replaced above is set as the closure's
        // callee. This is done by setting the [`CallFrame`]'s `stack` to
        // the [`ObjectInstance`].
        self.call_closure(unsafe { (*method).closure }, arg_count)
    }

    /// Create a new [`ObjectInstance`] of an [`ObjectClass`].
    ///
    /// Calls the `init` method if it exists on the [`ObjectClass`].
    fn call_class(&mut self, class: *mut ObjectClass, arg_count: usize) -> Result<()> {
        let instance = self.alloc(ObjectInstance::new(class));

        // Replace the [`ObjectClass`] in the VM's stack with the new [`ObjectInstance`]
        unsafe { *self.peek(arg_count) = Value::from(instance) };

        // Call the constructor/init method if it exists on the class
        match unsafe { (*class).methods.get(&self.init_string) } {
            Some(&init) => self.call_closure(init, arg_count),
            None if arg_count != 0 => self.err(TypeError::ArityMismatch {
                name: unsafe { (*self.init_string).value.to_string() },
                exp_args: 0,
                got_args: arg_count,
            }),
            None => Ok(()),
        }
    }

    /// Call [`ObjectClosure`] (function or method).
    ///
    /// This creates a new [`CallFrame`] while pushing the old one
    /// on to the [`CallFrame`] stack.
    fn call_closure(&mut self, closure: *mut ObjectClosure, arg_count: usize) -> Result<()> {
        if self.frames.len() >= self.frames.capacity() {
            return self.err(OverflowError::StackOverflow);
        }

        let function = unsafe { (*closure).function };
        let arity = unsafe { (*function).arity } as usize;
        if arg_count != arity {
            return self.err(TypeError::ArityMismatch {
                name: unsafe { (*(*function).name).value }.to_string(),
                exp_args: arity,
                got_args: arg_count,
            });
        }

        let frame = CallFrame {
            closure,
            // Points to the closure's function chunk
            ip: unsafe { (*function).chunk.ops.as_ptr() },
            // Points to the Value of the callee
            stack: self.peek(arg_count),
        };
        unsafe { self.frames.push_unchecked(mem::replace(&mut self.frame, frame)) };

        Ok(())
    }

    /// Call [`ObjectNative`] function
    ///
    /// These are functions provided by the language runtime and not written in the language
    fn call_native(
        &mut self,
        native: *mut ObjectNative,
        arg_count: usize,
        stdout: &mut impl Write,
    ) -> Result<()> {
        let value = match { unsafe { (*native).native } } {
            Native::Clock => {
                self.pop();

                if arg_count != 0 {
                    return self.err(TypeError::ArityMismatch {
                        name: "clock".to_string(),
                        exp_args: 0,
                        got_args: arg_count,
                    });
                }
                util::now().into()
            }
            Native::Length => {
                if arg_count != 1 {
                    return self.err(TypeError::ArityMismatch {
                        name: "len".to_string(),
                        exp_args: 1,
                        got_args: arg_count,
                    });
                }

                let value = self.pop();
                self.pop();

                if !value.is_object() {
                    return self.err(TypeError::NoLength { type_: value.to_string() });
                }

                let obj = value.as_object();

                match obj.type_() {
                    ObjectType::List => {
                        let list = unsafe { (obj).list };
                        let length = unsafe { (*list).values.len() };

                        (length as f64).into()
                    }
                    ObjectType::String => {
                        let string = unsafe { (obj).string };
                        let length = unsafe { (*string).value.len() };

                        (length as f64).into()
                    }
                    _ => {
                        return self.err(TypeError::NoLength { type_: obj.type_().to_string() });
                    }
                }
            }
            Native::Print => {
                if arg_count != 1 {
                    return self.err(TypeError::ArityMismatch {
                        name: "print".to_string(),
                        exp_args: 1,
                        got_args: arg_count,
                    });
                }

                let arg = self.pop();
                self.pop();

                match write!(stdout, "{arg}")
                    .or_else(|_| self.err(IoError::WriteError { file: "stdout".to_string() }))
                {
                    Ok(_) => Value::NIL,
                    Err(x) => {
                        return Err(x);
                    }
                }
            }
            Native::PrintLn => {
                if arg_count != 1 {
                    return self.err(TypeError::ArityMismatch {
                        name: "println".to_string(),
                        exp_args: 1,
                        got_args: arg_count,
                    });
                }

                let arg = self.pop();
                self.pop();

                match writeln!(stdout, "{arg}")
                    .or_else(|_| self.err(IoError::WriteError { file: "stdout".to_string() }))
                {
                    Ok(_) => Value::NIL,
                    Err(x) => {
                        return Err(x);
                    }
                }
            }
            Native::TypeOf => {
                if arg_count != 1 {
                    return self.err(TypeError::ArityMismatch {
                        name: "typeof".to_string(),
                        exp_args: 1,
                        got_args: arg_count,
                    });
                }

                let value = self.pop();
                self.pop();

                let obj_type = match value.type_() {
                    value::ValueType::Nil => "nil",
                    value::ValueType::Bool => "boolean",
                    value::ValueType::Number => "number",
                    value::ValueType::Object(type_) => match type_ {
                        ObjectType::BoundMethod => "function",
                        ObjectType::Class => "class",
                        ObjectType::Closure => "function",
                        ObjectType::Function => "function",
                        ObjectType::Native => "function",
                        ObjectType::Instance => "instance",
                        ObjectType::String => "string",
                        ObjectType::List => "list",
                        ObjectType::Package => "package",
                        ObjectType::Upvalue => "upvalue",
                    },
                };

                let type_str = self.alloc(ObjectString::new(&obj_type)).into();

                type_str
            }
        };

        self.push(value);

        Ok(())
    }

    /// Binary operator that acts on any [`Value`].
    fn binary_op(&mut self, op: fn(Value, Value) -> Value) {
        let b = self.pop();
        let a = self.pop();
        self.push(op(a, b));
    }

    /// Binary operator that acts on numbers.
    fn binary_op_number(&mut self, op: fn(f64, f64) -> Value, op_str: &str) -> Result<()> {
        let b = self.pop();
        let a = self.pop();

        if a.is_number() && b.is_number() {
            let value = op(a.as_number(), b.as_number());
            self.push(value);
            Ok(())
        } else {
            self.err(TypeError::UnsupportedOperandInfix {
                op: op_str.to_string(),
                lt_type: a.type_().to_string(),
                rt_type: b.type_().to_string(),
            })
        }
    }

    /// Read (and return) the bytecode op at [`CallFrame`]'s (and underlying [`Chunk`]'s)
    /// instruction pointer (IP).
    ///
    /// Increments current [`CallFrame`]'s IP + 1.
    fn read_u8(&mut self) -> u8 {
        let byte = unsafe { *self.frame.ip };
        self.frame.ip = unsafe { self.frame.ip.add(1) };
        byte
    }

    /// Read (and return) the next 2 bytecode ops at [`CallFrame`]'s (and underlying [`Chunk`]'s)
    /// instruction pointer (IP).
    ///
    /// Increments current [`CallFrame`]'s IP + 2.
    fn read_u16(&mut self) -> u16 {
        let byte1 = self.read_u8();
        let byte2 = self.read_u8();
        u16::from_le_bytes([byte1, byte2])
    }

    /// Return the [`Value`] located at the [`CallFrame`]'s instruction pointer (IP).
    ///
    /// The bytecode op at the [`CallFrame`]'s instruction pointer (IP) is the index of
    /// constant in the [`CallFrame`]'s constant table.
    ///
    /// Increments current [`CallFrame`]'s IP + 1.
    fn read_value(&mut self) -> Value {
        let constant_idx = self.read_u8() as usize;
        let function = unsafe { (*self.frame.closure).function };
        *unsafe { (*function).chunk.constants.get_unchecked(constant_idx) }
    }

    /// Pushes a [`Value`] to the stack.
    ///
    /// Increments the VM's `stack_top + 1`
    fn push(&mut self, value: Value) {
        unsafe { *self.stack_top = value };
        self.stack_top = unsafe { self.stack_top.add(1) };
    }

    /// Pops a [`Value`] from the stack.
    ///
    /// Decrements the VM's `stack_top - 1`
    fn pop(&mut self) -> Value {
        self.stack_top = unsafe { self.stack_top.sub(1) };
        unsafe { *self.stack_top }
    }

    /// Preview what [`Value`] is from `n` slots back in the stack.
    ///
    /// This consumes no ops and doesn't affect the VM's stack
    fn peek(&mut self, n: usize) -> *mut Value {
        unsafe { self.stack_top.sub(n + 1) }
    }

    fn capture_upvalue(&mut self, location: *mut Value) -> *mut ObjectUpvalue {
        match self.open_upvalues.iter().find(|&&upvalue| unsafe { (*upvalue).location } == location)
        {
            Some(&upvalue) => upvalue,
            None => {
                let upvalue = self.alloc(ObjectUpvalue::new(location));
                self.open_upvalues.push(upvalue);
                upvalue
            }
        }
    }

    fn close_upvalues(&mut self, last: *mut Value) {
        for idx in (0..self.open_upvalues.len()).rev() {
            let upvalue = *unsafe { self.open_upvalues.get_unchecked(idx) };
            if last <= unsafe { (*upvalue).location } {
                unsafe { (*upvalue).closed = *(*upvalue).location };
                unsafe { (*upvalue).location = &mut (*upvalue).closed };
                self.open_upvalues.swap_remove(idx);
            }
        }
    }

    /// Wraps an [`Error`] in a span using the offset of the last executed
    /// instruction.
    #[cold]
    fn err(&self, err: impl Into<Error>) -> Result<()> {
        let function = unsafe { (*self.frame.closure).function };
        let idx = unsafe { self.frame.ip.offset_from((*function).chunk.ops.as_ptr()) } as usize;
        let span = unsafe { (*function).chunk.spans[idx - 1].clone() };
        Err((err.into(), span))
    }
}

impl Default for VM {
    fn default() -> Self {
        let mut gc = Gc::default();

        let mut globals = HashMap::with_capacity_and_hasher(256, BuildHasherDefault::default());

        let init_string = gc.alloc("init");

        globals.insert(gc.alloc("clock"), gc.alloc(ObjectNative::new(Native::Clock)).into());
        globals.insert(gc.alloc("len"), gc.alloc(ObjectNative::new(Native::Length)).into());
        globals.insert(gc.alloc("print"), gc.alloc(ObjectNative::new(Native::Print)).into());
        globals.insert(gc.alloc("println"), gc.alloc(ObjectNative::new(Native::PrintLn)).into());
        globals.insert(gc.alloc("typeof"), gc.alloc(ObjectNative::new(Native::TypeOf)).into());

        let vm = Self {
            globals,
            open_upvalues: Vec::with_capacity(256),
            gc,
            next_gc: 1024 * 1024,
            frames: ArrayVec::new(),
            frame: CallFrame {
                closure: ptr::null_mut(),
                ip: ptr::null_mut(),
                stack: ptr::null_mut(),
            },
            stack: Box::new([Value::default(); STACK_MAX]),
            stack_top: ptr::null_mut(),
            init_string,
            source: String::new(),
        };

        vm
    }
}

#[derive(Debug)]
pub struct CallFrame {
    closure: *mut ObjectClosure,
    /// Instruction pointer for the current Chunk.
    ///
    /// Accessing `ip` without bounds checking is safe, assuming that the
    /// compiler always outputs correct code. The program stops execution
    /// when it reaches `op::RETURN`.
    ip: *const u8,
    stack: *mut Value,
}
