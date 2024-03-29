use std::convert::TryInto;
use std::mem;

use arrayvec::ArrayVec;

use crate::error::{ErrorS, NameError, OverflowError, Result, SyntaxError};
use crate::syntax::ast::{
    Expr, ExprLiteral, ExprS, Identifier, OpInfix, OpPrefix, Program, Stmt, StmtFn, StmtReturn,
    StmtS,
};
use crate::types::Span;
use crate::vm::gc::Gc;
use crate::vm::object::{ObjectFunction, ObjectString};
use crate::vm::op;
use crate::vm::value::Value;

#[derive(Debug)]
pub struct Compiler {
    ctx: CompilerCtx,
    class_ctx: Vec<ClassCtx>,
    package_ctx: PackageCtx,
}

impl Compiler {
    /// Creates a compiler for a new script.
    pub fn new(gc: &mut Gc) -> Self {
        let name = gc.alloc("");
        Self {
            ctx: CompilerCtx {
                function: gc.alloc(ObjectFunction::new(name, 0)),
                type_: FunctionType::Script,
                locals: ArrayVec::new(),
                upvalues: ArrayVec::new(),
                parent: None,
                scope_depth: 0,
            },
            class_ctx: Vec::new(),
            package_ctx: PackageCtx { name: None },
        }
    }

    /// Compile source string in to script function
    pub fn compile(program: &Program, gc: &mut Gc) -> Result<*mut ObjectFunction, Vec<ErrorS>> {
        let mut compiler = Self::new(gc);

        if let Some(package_name) = &program.package {
            let package = gc.alloc(package_name.to_owned());

            compiler.emit_u8(op::PACKAGE, &NO_SPAN);
            let _ = compiler.emit_constant(package.into(), &NO_SPAN);

            compiler.package_ctx.name = Some(package);

            let _ = compiler.compile_expr(
                &(Expr::Literal(ExprLiteral::String(package_name.to_string())), NO_SPAN),
                gc,
            );

            let _ = compiler.declare_local("package", &NO_SPAN);
            compiler.define_local();

            let _ = compiler.set_variable("package", &NO_SPAN, gc);
        }

        for stmt in &program.stmts {
            compiler.compile_stmt(stmt, gc).map_err(|e| vec![e])?;
        }

        if let Some(_) = &program.package {
            compiler.emit_u8(op::POP, &NO_SPAN);
            compiler.package_ctx.name = None;
        }

        compiler.emit_u8(op::NIL, &NO_SPAN);
        compiler.emit_u8(op::RETURN, &NO_SPAN);

        Ok(compiler.ctx.function)
    }

    /// Compile an statement into bytecode ops.
    /// Write ops to [`Chunk`].
    fn compile_stmt(&mut self, (stmt, span): &StmtS, gc: &mut Gc) -> Result<()> {
        match stmt {
            Stmt::Block(block) => {
                self.begin_scope();
                for stmt in &block.stmts {
                    self.compile_stmt(stmt, gc)?;
                }
                self.end_scope(span);
            }
            Stmt::Class(class) => {
                let has_super = class.super_.is_some();

                let name = gc.alloc(&class.name).into();
                // Allocates class by name
                // Pushes class `Value` on to VM's stack
                self.emit_u8(op::CLASS, span);
                // Emit the name of the class
                // This get's consumed when the VM works `op::CLASS`
                self.emit_constant(name, span)?;

                if self.is_global() {
                    // Add class `Value` by name to VM's globals
                    // This pops the `Value` from `op::CLASS` off the VM's stack
                    self.emit_u8(op::DEFINE_GLOBAL, span);
                    self.emit_constant(name, span)?;
                } else {
                    // The `Value` from `op::CLASS` is popped from VM's stack
                    // when `Local` is no longer in scope
                    self.declare_local(&class.name, span)?;
                    self.define_local();
                }

                self.class_ctx.push(ClassCtx { has_super });

                // Does the class extend another class?
                if let Some(super_) = &class.super_ {
                    match &super_.0 {
                        Expr::Identifier(identifier) => {
                            if identifier.identifier.name == class.name {
                                return Err((
                                    NameError::ClassInheritFromSelf {
                                        name: class.name.to_string(),
                                    }
                                    .into(),
                                    span.clone(),
                                ));
                            }
                        }
                        _ => unreachable!(),
                    };

                    self.begin_scope();
                    self.declare_local("super", &NO_SPAN)?;
                    self.define_local();

                    // Get parent class `Value` and push it on the VM's stack
                    self.compile_expr(super_, gc)?;
                    // Get new class `Value` and push it on the VM's stack
                    self.get_variable(
                        &Identifier { name: class.name.clone(), package: None, depth: None },
                        span,
                        gc,
                    )?;
                    // This will consume both values just pushed on VM's stack
                    self.emit_u8(op::INHERIT, span);
                } else {
                    let base_class = String::from("Object");

                    if class.name != base_class {
                        if let Some(last) = self.class_ctx.last_mut() {
                            last.has_super = true;
                        }

                        self.begin_scope();
                        self.declare_local("super", &NO_SPAN)?;
                        self.define_local();

                        self.get_variable(
                            &Identifier { name: base_class, package: None, depth: None },
                            span,
                            gc,
                        )?;

                        self.get_variable(
                            &Identifier { name: class.name.clone(), package: None, depth: None },
                            span,
                            gc,
                        )?;

                        // This will consume both values just pushed on VM's stack
                        self.emit_u8(op::INHERIT, span);
                    }
                }

                // Initialize class fields, if they exist
                if !class.fields.is_empty() {
                    // Get class `Value` by name and push it on to the VM's stack
                    self.get_variable(
                        &Identifier { name: class.name.clone(), package: None, depth: None },
                        span,
                        gc,
                    )?;

                    for (field_assign, span) in &class.fields {
                        let name = &field_assign.identifier.name;

                        if name == "init" {
                            return Err((
                                NameError::ReservedName { name: (&name).to_string() }.into(),
                                span.clone(),
                            ));
                        }

                        // Compile value expression if it exists and push it on the VM's stack.
                        //
                        // This value is the field's default value.
                        // Otherwise its initalized to `nil`.
                        match &field_assign.value {
                            Some(value) => self.compile_expr(value, gc)?,
                            None => self.emit_u8(op::NIL, span),
                        }

                        self.emit_u8(op::FIELD, span);
                        // Emit field name's constant index
                        let name = gc.alloc(name).into();
                        self.emit_constant(name, span)?;
                    }

                    self.emit_u8(op::POP, span);
                }

                // Initialize class fields, if they exist
                if !class.static_fields.is_empty() {
                    // Get class `Value` by name and push it on to the VM's stack
                    self.get_variable(
                        &Identifier { name: class.name.clone(), package: None, depth: None },
                        span,
                        gc,
                    )?;

                    for (field_assign, span) in &class.static_fields {
                        let name = &field_assign.identifier.name;

                        // Compile value expression if it exists and push it on the VM's stack.
                        //
                        // This value is the field's default value.
                        // Otherwise its initalized to `nil`.
                        match &field_assign.value {
                            Some(value) => self.compile_expr(value, gc)?,
                            None => self.emit_u8(op::NIL, span),
                        }

                        self.emit_u8(op::STATIC_FIELD, span);
                        // Emit field name's constant index
                        let name = gc.alloc(name).into();
                        self.emit_constant(name, span)?;
                    }

                    self.emit_u8(op::POP, span);
                }

                // Initialize class methods, if they exist
                if !class.methods.is_empty() {
                    self.get_variable(
                        &Identifier { name: class.name.clone(), package: None, depth: None },
                        span,
                        gc,
                    )?;
                    for (method, span) in &class.methods {
                        let type_ = if method.name == "init" {
                            FunctionType::Initializer
                        } else {
                            FunctionType::Method
                        };
                        self.compile_function(method, span, type_, gc)?;

                        let name = gc.alloc(&method.name).into();
                        self.emit_u8(op::METHOD, span);
                        self.emit_constant(name, span)?;
                    }
                    self.emit_u8(op::POP, span);
                }

                // Initialize class methods, if they exist
                if !class.static_methods.is_empty() {
                    self.get_variable(
                        &Identifier { name: class.name.clone(), package: None, depth: None },
                        span,
                        gc,
                    )?;
                    for (method, span) in &class.static_methods {
                        self.compile_function(method, span, FunctionType::Method, gc)?;

                        let name = gc.alloc(&method.name).into();
                        self.emit_u8(op::STATIC_METHOD, span);
                        self.emit_constant(name, span)?;
                    }
                    self.emit_u8(op::POP, span);
                }

                if let Some(x) = self.class_ctx.last() {
                    if x.has_super {
                        self.end_scope(&NO_SPAN);
                    }
                }

                self.class_ctx.pop().expect("attempted to pop the global context");
            }
            Stmt::Error => panic!("tried to compile despite parser errors"),
            Stmt::Expr(expr) => {
                self.compile_expr(&expr.value, gc)?;
                self.emit_u8(op::POP, span);
            }
            Stmt::For(for_) => {
                self.begin_scope();

                // Evaluate init statement. This may be an expression, a
                // variable assignment, or nothing at all.
                if let Some(init) = &for_.init {
                    self.compile_stmt(init, gc)?;
                }

                // START:
                let loop_start = self.start_loop();

                // Evaluate the condition, if it exists.
                let mut jump_to_end = None;
                if let Some(cond) = &for_.cond {
                    self.compile_expr(cond, gc)?;
                    // If the condition is false, go to END.
                    jump_to_end = Some(self.emit_jump(op::JUMP_IF_FALSE, span));
                    // Discard the condition.
                    self.emit_u8(op::POP, span);
                }

                // Evaluate the body.
                self.compile_stmt(&for_.body, gc)?;

                // Evaluate the increment expression, if it exists.
                if let Some(incr) = &for_.incr {
                    self.compile_expr(incr, gc)?;
                    // Discard the result of the expression.
                    self.emit_u8(op::POP, span);
                }

                // Go to START.
                self.emit_loop(loop_start, span)?;
                // END:
                if let Some(jump_to_end) = jump_to_end {
                    self.patch_jump(jump_to_end, span)?;
                    // Discard the condition.
                    self.emit_u8(op::POP, span);
                }

                self.end_scope(span);
            }
            Stmt::Fn(fun) => {
                self.compile_function(fun, span, FunctionType::Function, gc)?;
                if self.is_global() {
                    let name = gc.alloc(&fun.name).into();
                    self.emit_u8(op::DEFINE_GLOBAL, span);
                    self.emit_constant(name, span)?;
                } else {
                    self.declare_local(&fun.name, span)?;
                    self.define_local();
                }
            }
            Stmt::If(if_) => {
                self.compile_expr(&if_.cond, gc)?;
                // If the condition is false, go to ELSE.
                let jump_to_else = self.emit_jump(op::JUMP_IF_FALSE, span);
                // Discard the condition.
                self.emit_u8(op::POP, span);
                // Evaluate the if branch.
                self.compile_stmt(&if_.then, gc)?;
                // Go to END.
                let jump_to_end = self.emit_jump(op::JUMP, span);

                // ELSE:
                self.patch_jump(jump_to_else, span)?;
                self.emit_u8(op::POP, span); // Discard the condition.
                if let Some(else_) = &if_.else_ {
                    self.compile_stmt(else_, gc)?;
                }

                // END:
                self.patch_jump(jump_to_end, span)?;
            }
            Stmt::Return(return_) => {
                match self.ctx.type_ {
                    FunctionType::Script => {
                        return Err((SyntaxError::ReturnOutsideFunction.into(), span.clone()));
                    }
                    FunctionType::Initializer => match return_.value {
                        Some(_) => {
                            return Err((SyntaxError::ReturnInInitializer.into(), span.clone()));
                        }
                        None => {
                            self.emit_u8(op::GET_LOCAL, span);
                            self.emit_u8(0, span);
                        }
                    },
                    FunctionType::Function | FunctionType::Method => match &return_.value {
                        Some(value) => self.compile_expr(value, gc)?,
                        None => self.emit_u8(op::NIL, span),
                    },
                }
                self.emit_u8(op::RETURN, span);
            }
            Stmt::Assign(assign) => {
                let name = &assign.identifier.name;
                if self.is_global() {
                    let name = gc.alloc(name);
                    match &assign.value {
                        Some(value) => self.compile_expr(value, gc)?,
                        None => self.emit_u8(op::NIL, span),
                    }
                    self.emit_u8(op::DEFINE_GLOBAL, span);
                    self.emit_constant(name.into(), span)?;
                } else {
                    self.declare_local(name, span)?;
                    match &assign.value {
                        Some(value) => self.compile_expr(value, gc)?,
                        None => self.emit_u8(op::NIL, span),
                    }
                    self.define_local();
                }
            }
            Stmt::While(while_) => {
                // START:
                let loop_start = self.start_loop();

                // Evaluate condition.
                self.compile_expr(&while_.cond, gc)?;
                // If the condition is false, go to END.
                let jump_to_end = self.emit_jump(op::JUMP_IF_FALSE, span);
                // Discard the condition.
                self.emit_u8(op::POP, span);
                // Evaluate the body of the loop.
                self.compile_stmt(&while_.body, gc)?;
                // Go to START.
                self.emit_loop(loop_start, span)?;

                // END:
                self.patch_jump(jump_to_end, span)?;
                // Discard the condition.
                self.emit_u8(op::POP, span);
            }
        }
        Ok(())
    }

    fn compile_function(
        &mut self,
        fun: &StmtFn,
        span: &Span,
        type_: FunctionType,
        gc: &mut Gc,
    ) -> Result<()> {
        let name = gc.alloc(&fun.name);
        let arity = fun
            .params
            .len()
            .try_into()
            .map_err(|_| (OverflowError::TooManyParams.into(), span.clone()))?;

        let ctx = CompilerCtx {
            function: gc.alloc(ObjectFunction::new(name, arity)),
            type_,
            locals: ArrayVec::new(),
            upvalues: ArrayVec::new(),
            parent: None,
            scope_depth: self.ctx.scope_depth + 1,
        };
        self.begin_ctx(ctx);

        match type_ {
            FunctionType::Initializer | FunctionType::Method => self.declare_local("this", span),
            FunctionType::Function | FunctionType::Script => self.declare_local(&fun.name, span),
        }?;
        self.define_local();

        for param in &fun.params {
            self.declare_local(param, span)?;
            self.define_local();
        }

        for stmt in &fun.body.stmts {
            self.compile_stmt(stmt, gc)?;
        }

        // Implicit return at the end of the function.
        if unsafe { (*self.ctx.function).chunk.ops.last() } != Some(&op::RETURN) {
            let stmt = (Stmt::Return(StmtReturn { value: None }), NO_SPAN);
            self.compile_stmt(&stmt, gc)?;
        }

        let (function, upvalues) = self.end_ctx();
        let value = function.into();
        self.emit_u8(op::CLOSURE, span);
        self.emit_constant(value, span)?;

        for upvalue in &upvalues {
            self.emit_u8(upvalue.is_local.into(), span);
            self.emit_u8(upvalue.idx, span);
        }

        Ok(())
    }

    /// Compile an expression into bytecode ops.
    /// Write ops to [`Chunk`].
    fn compile_expr(&mut self, (expr, span): &ExprS, gc: &mut Gc) -> Result<()> {
        match expr {
            Expr::Assign(assign) => {
                self.compile_expr(&assign.value, gc)?;
                self.set_variable(&assign.identifier.name, span, gc)?;
            }
            Expr::Call(call) => {
                let arg_count = call
                    .args
                    .len()
                    .try_into()
                    .map_err(|_| (OverflowError::TooManyArgs.into(), span.clone()))?;

                self.compile_expr(&call.callee, gc)?;
                for arg in &call.args {
                    self.compile_expr(arg, gc)?;
                }

                let ops = unsafe { &mut (*self.ctx.function).chunk.ops };

                match ops.len().checked_sub(2) {
                    Some(idx) if ops[idx] == op::GET_SUPER => ops[idx] = op::SUPER_INVOKE,
                    Some(_) | None => self.emit_u8(op::CALL, span),
                }

                self.emit_u8(arg_count, span);
            }
            Expr::Get(get) => {
                self.compile_expr(&get.object, gc)?;

                let name = gc.alloc(&get.name).into();
                self.emit_u8(op::GET_PROPERTY, span);
                self.emit_constant(name, span)?;
            }
            Expr::GetIndex(get_index) => {
                let target = &get_index.target;
                self.compile_expr(target, gc)?;

                let index = &get_index.index;

                let value = (*index).into();
                self.emit_u8(op::CONSTANT, span);
                self.emit_constant(value, span)?;

                self.emit_u8(op::GET_INDEX, span);
            }
            Expr::SetIndex(set_index) => {
                let target = &set_index.target;
                self.compile_expr(target, gc)?;

                let index = &set_index.index;

                let value = (*index).into();
                self.emit_u8(op::CONSTANT, span);
                self.emit_constant(value, span)?;

                let value = &set_index.value;
                self.compile_expr(value, gc)?;

                self.emit_u8(op::SET_INDEX, span);
            }
            Expr::Infix(infix) => {
                self.compile_expr(&infix.lt, gc)?;
                match infix.op {
                    OpInfix::Add => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::ADD, span);
                    }
                    OpInfix::Subtract => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::SUBTRACT, span);
                    }
                    OpInfix::Multiply => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::MULTIPLY, span);
                    }
                    OpInfix::Divide => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::DIVIDE, span);
                    }
                    OpInfix::Modulus => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::MODULUS, span);
                    }
                    OpInfix::Less => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::LESS, span);
                    }
                    OpInfix::LessEqual => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::LESS_EQUAL, span);
                    }
                    OpInfix::Greater => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::GREATER, span);
                    }
                    OpInfix::GreaterEqual => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::GREATER_EQUAL, span);
                    }
                    OpInfix::Equal => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::EQUAL, span);
                    }
                    OpInfix::NotEqual => {
                        self.compile_expr(&infix.rt, gc)?;
                        self.emit_u8(op::NOT_EQUAL, span);
                    }
                    OpInfix::LogicAnd => {
                        // If the first expression is false, go to END.
                        let jump_to_end = self.emit_jump(op::JUMP_IF_FALSE, span);
                        // Otherwise, evaluate the right expression.
                        self.emit_u8(op::POP, span);
                        self.compile_expr(&infix.rt, gc)?;

                        // END:
                        // Short-circuit to the end.
                        self.patch_jump(jump_to_end, span)?;
                    }
                    OpInfix::LogicOr => {
                        // If the first expression is false, go to RIGHT_EXPR.
                        let jump_to_right_expr = self.emit_jump(op::JUMP_IF_FALSE, span);
                        // Otherwise, go to END.
                        let jump_to_end = self.emit_jump(op::JUMP, span);

                        // RIGHT_EXPR:
                        self.patch_jump(jump_to_right_expr, span)?;
                        // Discard the left value.
                        self.emit_u8(op::POP, span);
                        // Evaluate the right expression.
                        self.compile_expr(&infix.rt, gc)?;

                        // END:
                        // Short-circuit to the end.
                        self.patch_jump(jump_to_end, span)?;
                    }
                };
            }
            Expr::Literal(literal) => match literal {
                ExprLiteral::Bool(true) => self.emit_u8(op::TRUE, span),
                ExprLiteral::Bool(false) => self.emit_u8(op::FALSE, span),
                ExprLiteral::Nil => self.emit_u8(op::NIL, span),
                ExprLiteral::Number(number) => {
                    let value = (*number).into();
                    self.emit_u8(op::CONSTANT, span);
                    self.emit_constant(value, span)?;
                }
                ExprLiteral::String(string) => {
                    let string = gc.alloc(string);
                    unsafe { (*string).common.is_marked = true };
                    let value = string.into();
                    self.emit_u8(op::CONSTANT, span);
                    self.emit_constant(value, span)?;
                }
                ExprLiteral::List(values) => {
                    for value in values.iter() {
                        let _ = self.compile_expr(value.into(), gc);
                    }

                    self.emit_u8(op::CREATE_LIST, span);

                    let list_length = values
                        .len()
                        .try_into()
                        .map_err(|_| (OverflowError::TooManyArgs.into(), span.clone()))?;

                    self.emit_u8(list_length, span);
                }
            },
            Expr::Prefix(prefix) => {
                self.compile_expr(&prefix.rt, gc)?;
                match prefix.op {
                    OpPrefix::Negate => self.emit_u8(op::NEGATE, span),
                    OpPrefix::Not => self.emit_u8(op::NOT, span),
                };
            }
            Expr::Set(set) => {
                self.compile_expr(&set.value, gc)?;
                self.compile_expr(&set.object, gc)?;

                let name = gc.alloc(&set.name).into();
                self.emit_u8(op::SET_PROPERTY, span);
                self.emit_constant(name, span)?;
            }
            Expr::Super(super_) => {
                match self.class_ctx.last() {
                    Some(class_ctx) => {
                        if !class_ctx.has_super {
                            return Err((SyntaxError::SuperWithoutSuperclass.into(), span.clone()));
                        }
                    }
                    None => return Err((SyntaxError::SuperOutsideClass.into(), span.clone())),
                }

                self.get_variable(
                    &Identifier { name: "this".to_owned(), package: None, depth: None },
                    span,
                    gc,
                )?;
                self.get_variable(
                    &Identifier { name: "super".to_owned(), package: None, depth: None },
                    span,
                    gc,
                )?;

                self.emit_u8(op::GET_SUPER, span);
                let name = gc.alloc(&super_.name).into();
                self.emit_constant(name, span)?;
            }
            Expr::Identifier(identifier) => self.get_variable(&identifier.identifier, span, gc)?,
        }
        Ok(())
    }

    /// Pushes the current ctx to parent and assigns it to the given ctx.
    ///
    /// Emits no byte ops
    fn begin_ctx(&mut self, ctx: CompilerCtx) {
        let ctx = mem::replace(&mut self.ctx, ctx);
        self.ctx.parent = Some(Box::new(ctx));
    }

    /// Pops the current ctx and extracts a [`Function`] from it.
    ///
    /// Emits no byte ops
    fn end_ctx(&mut self) -> (*mut ObjectFunction, ArrayVec<Upvalue, 256>) {
        let parent = self.ctx.parent.take().expect("tried to end context in a script");
        let ctx = mem::replace(&mut self.ctx, *parent);
        (ctx.function, ctx.upvalues)
    }

    /// Attempts to resolve a varible by name in order of scope: local, closure, global.
    ///
    /// This variable's value will get pushed to the [`VM`]'s stack.
    ///
    /// Emits one of the following byte code formats:
    ///
    /// - locals
    ///   - `0000 OP_GET_LOCAL`
    ///   - `0001 local_idx`
    /// - closure upvalues
    ///   - `0000 OP_GET_UPVALUE`
    ///   - `0001 upvalue_idx`
    /// - globals
    ///   - `0000 OP_GET_GLOBAL`
    ///   - `0001 chunk_constant_idx`
    fn get_variable(&mut self, idenifier: &Identifier, span: &Span, gc: &mut Gc) -> Result<()> {
        let name = idenifier.name.as_str();

        if name == "this" && self.class_ctx.is_empty() {
            return Err((SyntaxError::ThisOutsideClass.into(), span.clone()));
        }
        if let Some(local_idx) = self.ctx.resolve_local(name, false, span)? {
            self.emit_u8(op::GET_LOCAL, span);
            self.emit_u8(local_idx, span);
        } else if let Some(upvalue_idx) = self.ctx.resolve_upvalue(name, span)? {
            self.emit_u8(op::GET_UPVALUE, span);
            self.emit_u8(upvalue_idx, span);
        } else {
            let name = gc.alloc(name);
            self.emit_u8(op::GET_GLOBAL, span);
            self.emit_constant(name.into(), span)?;
        }
        Ok(())
    }

    /// Attempts to resolve a varible by name in order of scope: local, closure, global then set it's value
    ///
    /// Emits one of the following byte code formats:
    ///
    /// - locals
    ///   - `0000 OP_SET_LOCAL`
    ///   - `0001 local_idx`
    /// - closure upvalues
    ///   - `0000 OP_SET_UPVALUE`
    ///   - `0001 upvalue_idx`
    /// - globals
    ///   - `0000 OP_SET_GLOBAL`
    ///   - `0001 chunk_constant_idx`
    fn set_variable(&mut self, name: &str, span: &Span, gc: &mut Gc) -> Result<()> {
        if let Some(local_idx) = self.ctx.resolve_local(name, false, span)? {
            self.emit_u8(op::SET_LOCAL, span);
            self.emit_u8(local_idx, span);
        } else if let Some(upvalue_idx) = self.ctx.resolve_upvalue(name, span)? {
            self.emit_u8(op::SET_UPVALUE, span);
            self.emit_u8(upvalue_idx, span);
        } else {
            let name = gc.alloc(name);
            self.emit_u8(op::SET_GLOBAL, span);
            self.emit_constant(name.into(), span)?;
        }
        Ok(())
    }

    /// Add [`Local`] variable to [`CompilerCtx`] by `name`
    ///
    /// Emits no byte ops
    fn declare_local(&mut self, name: &str, span: &Span) -> Result<()> {
        for local in self.ctx.locals.iter().rev() {
            if local.depth < self.ctx.scope_depth {
                break;
            }
            if local.name == name {
                return Err((
                    NameError::AlreadyDefined { name: name.to_string() }.into(),
                    span.clone(),
                ));
            }
        }

        let local = Local {
            name: name.to_string(),
            depth: self.ctx.scope_depth,
            is_initialized: false,
            is_captured: false,
        };
        self.ctx
            .locals
            .try_push(local)
            .map_err(|_| (OverflowError::TooManyLocals.into(), span.clone()))
    }

    /// Set `is_initialized = true` on the last declared [`Local`] variable in [`CompilerCtx`].
    /// Will panic on an undeclared (e.g. undefined) [`Local`].
    ///
    /// Emits no byte ops
    fn define_local(&mut self) {
        self.ctx
            .locals
            .last_mut()
            .expect("tried to define a local without declaring it")
            .is_initialized = true;
    }

    /// A jump takes 1 byte for the instruction followed by 2 bytes for the
    /// offset. The offset is initialized as a dummy value, and is later patched
    /// to the correct value.
    ///
    /// It returns the index of the offset which is to be patched.
    fn emit_jump(&mut self, opcode: u8, span: &Span) -> usize {
        self.emit_u8(opcode, span);
        self.emit_u8(0xFF, span);
        self.emit_u8(0xFF, span);
        unsafe { (*self.ctx.function).chunk.ops.len() - 2 }
    }

    /// Takes the index of the jump offset to be patched as input, and patches
    /// it to point to the current instruction.
    fn patch_jump(&mut self, offset_idx: usize, span: &Span) -> Result<()> {
        // The extra -2 is to account for the space taken by the offset.
        let offset = unsafe { (*self.ctx.function).chunk.ops.len() - 2 - offset_idx };
        let offset =
            offset.try_into().map_err(|_| (OverflowError::JumpTooLarge.into(), span.clone()))?;
        let offset = u16::to_le_bytes(offset);
        unsafe {
            [
                (*self.ctx.function).chunk.ops[offset_idx],
                (*self.ctx.function).chunk.ops[offset_idx + 1],
            ] = offset
        };
        Ok(())
    }

    /// Returns the number of ops in the [`Chunk`].
    ///
    /// Emits no byte ops
    fn start_loop(&self) -> usize {
        unsafe { (*self.ctx.function).chunk.ops.len() }
    }

    fn emit_loop(&mut self, start_idx: usize, span: &Span) -> Result<()> {
        // The extra +3 is to account for the space taken by the instruction and
        // the offset.
        let offset = unsafe { (*self.ctx.function).chunk.ops.len() } + 3 - start_idx;
        let offset =
            offset.try_into().map_err(|_| (OverflowError::JumpTooLarge.into(), span.clone()))?;
        let offset = u16::to_le_bytes(offset);

        self.emit_u8(op::LOOP, span);
        self.emit_u8(offset[0], span);
        self.emit_u8(offset[1], span);

        Ok(())
    }

    fn begin_scope(&mut self) {
        self.ctx.scope_depth += 1;
    }

    fn end_scope(&mut self, span: &Span) {
        self.ctx.scope_depth -= 1;

        // Remove all locals that are no longer in scope.
        while let Some(local) = self.ctx.locals.last() {
            // Is local outside the current scope?
            if local.depth > self.ctx.scope_depth {
                // Was local captured from a closure?
                if local.is_captured {
                    self.emit_u8(op::CLOSE_UPVALUE, span);
                } else {
                    self.emit_u8(op::POP, span);
                }

                // Clean up compiler context
                self.ctx.locals.pop();
            } else {
                break;
            }
        }
    }

    /// Write 1 byte op to [`Chunk`]
    fn emit_u8(&mut self, byte: u8, span: &Span) {
        unsafe { (*self.ctx.function).chunk.write_u8(byte, span) };
    }

    /// Add [`Value`] to [`Chunk`]'s constants table.
    ///
    /// If [`Value`] already exists in [`Chunk`]'s constants table, that index will be returned instead.
    ///
    /// Emits a 1 byte op with the format
    /// - `0000 chunk_constant_idx`
    fn emit_constant(&mut self, value: Value, span: &Span) -> Result<()> {
        let constant_idx = unsafe { (*self.ctx.function).chunk.write_constant(value, span)? };
        self.emit_u8(constant_idx, span);
        Ok(())
    }

    /// Checks if the current `ctx` is global.
    /// e.g. `self.ctx.scope_depth == 0`.
    fn is_global(&self) -> bool {
        self.ctx.scope_depth == 0
    }
}

#[derive(Debug)]
pub struct CompilerCtx {
    function: *mut ObjectFunction,
    type_: FunctionType,
    /// Local variables
    locals: ArrayVec<Local, 256>,
    /// Closure captured variables
    upvalues: ArrayVec<Upvalue, 256>,
    parent: Option<Box<CompilerCtx>>,
    /// The scope currently executing
    scope_depth: usize,
}

impl CompilerCtx {
    fn resolve_local(&mut self, name: &str, capture: bool, span: &Span) -> Result<Option<u8>> {
        match self.locals.iter_mut().enumerate().rfind(|(_, local)| local.name == name) {
            Some((idx, local)) => {
                if local.is_initialized {
                    if capture {
                        local.is_captured = true;
                    }
                    Ok(Some(idx.try_into().expect("local index overflow")))
                } else {
                    Err((
                        NameError::AccessInsideInitializer { name: name.to_string() }.into(),
                        span.clone(),
                    ))
                }
            }
            None => Ok(None),
        }
    }

    fn resolve_upvalue(&mut self, name: &str, span: &Span) -> Result<Option<u8>> {
        let local_idx = match &mut self.parent {
            Some(parent) => parent.resolve_local(name, true, span)?,
            None => return Ok(None),
        };

        if let Some(local_idx) = local_idx {
            let upvalue_idx = self.add_upvalue(local_idx, true, span)?;
            return Ok(Some(upvalue_idx));
        };

        let upvalue_idx = match &mut self.parent {
            Some(parent) => parent.resolve_upvalue(name, span)?,
            None => return Ok(None),
        };

        if let Some(upvalue_idx) = upvalue_idx {
            let upvalue_idx = self.add_upvalue(upvalue_idx, false, span)?;
            return Ok(Some(upvalue_idx));
        };

        Ok(None)
    }

    fn add_upvalue(&mut self, idx: u8, is_local: bool, span: &Span) -> Result<u8> {
        let upvalue = Upvalue { idx, is_local };
        let upvalue_idx = match self.upvalues.iter().position(|u| u == &upvalue) {
            Some(upvalue_idx) => upvalue_idx,
            None => {
                self.upvalues
                    .try_push(upvalue)
                    .map_err(|_| (OverflowError::TooManyUpvalues.into(), span.clone()))?;
                let upvalues = self.upvalues.len();
                unsafe {
                    (*self.function).upvalue_count =
                        upvalues.try_into().expect("upvalue index overflow")
                };
                upvalues - 1
            }
        };

        Ok(upvalue_idx.try_into().expect("upvalue index overflow"))
    }
}

#[derive(Debug)]
struct ClassCtx {
    has_super: bool,
}

#[derive(Debug)]
struct PackageCtx {
    name: Option<*mut ObjectString>,
}

/// Local Variable
#[derive(Debug, Default)]
struct Local {
    /// The name of the variable.
    name: String,
    /// The scope depth of the variable, i.e. the number of nested scopes that
    /// surround it. This starts at 1, because global scopes don't have local
    /// variables.
    depth: usize,
    is_initialized: bool,
    /// If the variable was captured as an upvalue of a closure
    is_captured: bool,
}

/// Closure Variable
///
/// A stack pointer to the closure variable's value is
#[derive(Debug, Eq, PartialEq)]
struct Upvalue {
    idx: u8,
    is_local: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FunctionType {
    /// A function that has been defined in code.
    Function,
    /// A class initializer.
    Initializer,
    /// A bound method.
    Method,
    /// The global-level function that is called when the program starts.
    Script,
}

const NO_SPAN: Span = 0..0;
