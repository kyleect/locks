use std::fmt::{self, Debug, Display, Formatter};

pub use crate::types::Spanned;

pub type StmtS = Spanned<Stmt>;
pub type ExprS = Spanned<Expr>;

#[derive(Debug, Default)]
pub struct Program {
    pub stmts: Vec<StmtS>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(StmtBlock),
    Class(StmtClass),
    Expr(StmtExpr),
    For(Box<StmtFor>),
    Fn(StmtFn),
    If(Box<StmtIf>),
    Print(StmtPrint),
    Return(StmtReturn),
    Assign(StmtAssign),
    While(Box<StmtWhile>),
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtBlock {
    pub stmts: Vec<StmtS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtClass {
    pub name: String,
    pub super_: Option<ExprS>,
    pub methods: Vec<Spanned<StmtFn>>,
    pub fields: Vec<StmtFieldAssign>,
}

/// An expression statement evaluates an expression and discards the result.
#[derive(Clone, Debug, PartialEq)]
pub struct StmtExpr {
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtFor {
    pub init: Option<StmtS>,
    pub cond: Option<ExprS>,
    pub incr: Option<ExprS>,
    pub body: StmtS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtFn {
    pub name: String,
    pub params: Vec<String>,
    pub body: StmtBlock,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtIf {
    pub cond: ExprS,
    pub then: StmtS,
    pub else_: Option<StmtS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtPrint {
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtReturn {
    pub value: Option<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtAssign {
    pub identifier: Identifier,
    pub value: Option<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtFieldAssign {
    pub access_modifier: Option<AccessModifier>,
    pub field: Spanned<StmtAssign>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AccessModifier {
    Invalid,
    Private,
    Public,
}

impl From<&str> for AccessModifier {
    fn from(value: &str) -> Self {
        let access_modifier = match value {
            "public" => AccessModifier::Public,
            "private" => AccessModifier::Private,
            _ => AccessModifier::Invalid,
        };
        access_modifier
    }
}

impl Display for AccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl Debug for AccessModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtWhile {
    pub cond: ExprS,
    pub body: StmtS,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Assign(Box<ExprAssign>),
    Call(Box<ExprCall>),
    Get(Box<ExprGet>),
    Infix(Box<ExprInfix>),
    Literal(ExprLiteral),
    Prefix(Box<ExprPrefix>),
    Set(Box<ExprSet>),
    Super(ExprSuper),
    Identifier(ExprIdentifier),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAssign {
    pub identifier: Identifier,
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCall {
    pub callee: ExprS,
    pub args: Vec<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprGet {
    pub object: ExprS,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprLiteral {
    Bool(bool),
    Nil,
    Number(f64),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprInfix {
    pub lt: ExprS,
    pub op: OpInfix,
    pub rt: ExprS,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OpInfix {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    LogicAnd,
    LogicOr,
}

impl Display for OpInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op = match self {
            OpInfix::Add => "+",
            OpInfix::Subtract => "-",
            OpInfix::Multiply => "*",
            OpInfix::Divide => "/",
            OpInfix::Modulus => "%",
            OpInfix::Less => "<",
            OpInfix::LessEqual => "<=",
            OpInfix::Greater => ">",
            OpInfix::GreaterEqual => ">=",
            OpInfix::Equal => "==",
            OpInfix::NotEqual => "!=",
            OpInfix::LogicAnd => "and",
            OpInfix::LogicOr => "or",
        };
        write!(f, "{op}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPrefix {
    pub op: OpPrefix,
    pub rt: ExprS,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OpPrefix {
    Negate,
    Not,
}

impl Display for OpPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op = match self {
            OpPrefix::Negate => "-",
            OpPrefix::Not => "!",
        };
        write!(f, "{op}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprSet {
    pub object: ExprS,
    pub name: String,
    pub value: ExprS,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExprSuper {
    pub super_: Identifier,
    pub name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExprIdentifier {
    pub identifier: Identifier,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identifier {
    pub name: String,
    /// This field is initialized as [`None`] by the parser, and is later
    /// filled by the resolver.
    pub depth: Option<usize>,
}
