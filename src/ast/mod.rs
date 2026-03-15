//! Abstract Syntax Tree for QuinusLang

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<TopLevelItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelItem {
    Fn(FnDef),
    Struct(StructDef),
    Class(ClassDef),
    Mod(ModDef),
    Import(Import),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDef {
    pub name: String,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub fields: Vec<FieldDef>,
    pub init: Option<InitDef>,
    pub methods: Vec<MethodDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitDef {
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodDef {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Vec<Stmt>,
    pub is_virtual: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModDef {
    pub name: String,
    pub items: Vec<TopLevelItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Void,
    Array(Box<Type>),
    Named(String),
    // Spec type names
    U8, U16, U32, U64,
    I8, I16, I32, I64,
    Usize,
    F32, F64,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::Str => write!(f, "str"),
            Type::Void => write!(f, "void"),
            Type::Array(inner) => write!(f, "[{}]", inner),
            Type::Named(name) => write!(f, "{}", name),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::Usize => write!(f, "usize"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VarDecl { name: String, ty: Option<Type>, init: Expr, mutable: bool },
    Assign { target: AssignTarget, value: Expr },
    If { cond: Expr, then_body: Vec<Stmt>, else_body: Option<Vec<Stmt>> },
    For { init: Option<Box<Stmt>>, cond: Option<Expr>, step: Option<Box<Stmt>>, body: Vec<Stmt> },
    While { cond: Expr, body: Vec<Stmt> },
    ExprStmt(Expr),
    Return(Option<Expr>),
    TryCatch { try_body: Vec<Stmt>, catch_param: Option<String>, catch_body: Vec<Stmt> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignTarget {
    Ident(String),
    Index { base: Box<Expr>, index: Box<Expr> },
    Field { base: Box<Expr>, field: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    Unary { op: UnOp, operand: Box<Expr> },
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Index { base: Box<Expr>, index: Box<Expr> },
    Field { base: Box<Expr>, field: String },
    New { class: String, args: Vec<Expr> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg, Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}
