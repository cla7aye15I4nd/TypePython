//! Python AST types - mirrors Python's ast module exactly
//! Reference: https://docs.python.org/3/library/ast.html
//!
//! This module provides Rust types that directly correspond to Python's AST nodes,
//! enabling seamless deserialization of JSON produced by Python's ast module.
//!
//! The module also provides legacy types used by the codegen system, with conversion
//! functions from Python AST to the legacy format.

use serde::{Deserialize, Serialize};
use std::process::Command;

// ============================================================================
// Legacy AST types for codegen compatibility
// These are the types that the codegen system expects
// ============================================================================

/// Legacy Program type for codegen
#[derive(Debug, Clone)]
pub struct Program {
    pub imports: Vec<Import>,
    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
    pub statements: Vec<Statement>,
}

/// Legacy Import type for codegen
#[derive(Debug, Clone)]
pub struct Import {
    pub module_path: Vec<String>,
}

/// Legacy Class type for codegen
#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub base: Option<String>,
    pub fields: Vec<ClassField>,
    pub methods: Vec<Method>,
}

/// Legacy ClassField type for codegen
#[derive(Debug, Clone)]
pub struct ClassField {
    pub name: String,
    pub field_type: Type,
    pub default: Option<Expression>,
}

/// Legacy Method type for codegen
#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

/// Legacy Function type for codegen
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

/// Legacy Parameter type for codegen
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

/// Legacy Statement enum for codegen
#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl {
        name: String,
        var_type: Type,
        value: Expression,
    },
    AnnotatedAssignment {
        target: Expression,
        var_type: Type,
        value: Expression,
    },
    Assignment {
        target: Expression,
        value: Expression,
    },
    TupleUnpackAssignment {
        targets: Vec<Expression>,
        value: Expression,
    },
    AugAssignment {
        target: Expression,
        op: AugAssignOp,
        value: Expression,
    },
    If {
        condition: Expression,
        then_block: Vec<Statement>,
        elif_clauses: Vec<(Expression, Vec<Statement>)>,
        else_block: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        targets: Vec<String>,
        iter: Expression,
        body: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },
    Return(Option<Expression>),
    Break,
    Continue,
    Pass,
    Delete(Expression),
    Expr(Expression),
    Try {
        body: Vec<Statement>,
        handlers: Vec<LegacyExceptHandler>,
        else_block: Option<Vec<Statement>>,
        finally_block: Option<Vec<Statement>>,
    },
    Raise {
        exception: Option<Expression>,
        cause: Option<Expression>,
    },
    Assert {
        test: Expression,
        msg: Option<Expression>,
    },
    Global {
        names: Vec<String>,
    },
    Nonlocal {
        names: Vec<String>,
    },
}

/// Legacy ExceptHandler for codegen
#[derive(Debug, Clone)]
pub struct LegacyExceptHandler {
    pub exception_types: Vec<String>,
    pub name: Option<String>,
    pub body: Vec<Statement>,
}

/// Legacy Expression enum for codegen
#[derive(Debug, Clone)]
pub enum Expression {
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    BytesLit(Vec<u8>),
    BoolLit(bool),
    NoneLit,
    Var(String),
    List(Vec<Expression>),
    Tuple(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),
    Set(Vec<Expression>),
    BinOp {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expression>,
    },
    Call {
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    Attribute {
        object: Box<Expression>,
        attr: String,
    },
    Subscript {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    Slice {
        start: Option<Box<Expression>>,
        stop: Option<Box<Expression>>,
        step: Option<Box<Expression>>,
    },
    Yield {
        value: Option<Box<Expression>>,
        is_from: bool,
    },
    ListComprehension {
        element: Box<Expression>,
        clauses: Vec<ComprehensionClause>,
    },
    DictComprehension {
        key: Box<Expression>,
        value: Box<Expression>,
        clauses: Vec<ComprehensionClause>,
    },
    SetComprehension {
        element: Box<Expression>,
        clauses: Vec<ComprehensionClause>,
    },
    GeneratorExpression {
        element: Box<Expression>,
        clauses: Vec<ComprehensionClause>,
    },
    Ternary {
        condition: Box<Expression>,
        true_value: Box<Expression>,
        false_value: Box<Expression>,
    },
}

/// Legacy ComprehensionClause for codegen
#[derive(Debug, Clone)]
pub struct ComprehensionClause {
    pub target: Vec<String>,
    pub iterable: Box<Expression>,
    pub conditions: Vec<Expression>,
}

// ============================================================================
// Compatibility types for codegen
// These map to the old AST types used by the codegen system
// ============================================================================

/// Unified binary operation type for codegen compatibility
/// Maps Python's separate Operator, CmpOp, and BoolOpKind to a single enum
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic (from Operator)
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,
    // Bitwise (from Operator)
    BitOr,
    BitXor,
    BitAnd,
    LShift,
    RShift,
    // Comparison (from CmpOp)
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Is,
    IsNot,
    In,
    NotIn,
    // Logical (from BoolOpKind)
    And,
    Or,
}

impl BinaryOp {
    /// Convert from Python AST Operator
    pub fn from_operator(op: &Operator) -> Self {
        match op {
            Operator::Add => BinaryOp::Add,
            Operator::Sub => BinaryOp::Sub,
            Operator::Mult => BinaryOp::Mul,
            Operator::Div => BinaryOp::Div,
            Operator::FloorDiv => BinaryOp::FloorDiv,
            Operator::Mod => BinaryOp::Mod,
            Operator::Pow => BinaryOp::Pow,
            Operator::BitOr => BinaryOp::BitOr,
            Operator::BitXor => BinaryOp::BitXor,
            Operator::BitAnd => BinaryOp::BitAnd,
            Operator::LShift => BinaryOp::LShift,
            Operator::RShift => BinaryOp::RShift,
            Operator::MatMult => BinaryOp::Mul, // Treat @ as * for now
        }
    }

    /// Convert from Python AST CmpOp
    pub fn from_cmp_op(op: &CmpOp) -> Self {
        match op {
            CmpOp::Eq => BinaryOp::Eq,
            CmpOp::NotEq => BinaryOp::Ne,
            CmpOp::Lt => BinaryOp::Lt,
            CmpOp::LtE => BinaryOp::Le,
            CmpOp::Gt => BinaryOp::Gt,
            CmpOp::GtE => BinaryOp::Ge,
            CmpOp::Is => BinaryOp::Is,
            CmpOp::IsNot => BinaryOp::IsNot,
            CmpOp::In => BinaryOp::In,
            CmpOp::NotIn => BinaryOp::NotIn,
        }
    }

    /// Convert from Python AST BoolOpKind
    pub fn from_bool_op(op: &BoolOpKind) -> Self {
        match op {
            BoolOpKind::And => BinaryOp::And,
            BoolOpKind::Or => BinaryOp::Or,
        }
    }
}

/// Unified unary operation type for codegen compatibility
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,    // USub
    Pos,    // UAdd
    Not,    // Not
    BitNot, // Invert
}

impl UnaryOp {
    /// Convert from Python AST UnaryOpKind
    pub fn from_unary_op_kind(op: &UnaryOpKind) -> Self {
        match op {
            UnaryOpKind::USub => UnaryOp::Neg,
            UnaryOpKind::UAdd => UnaryOp::Pos,
            UnaryOpKind::Not => UnaryOp::Not,
            UnaryOpKind::Invert => UnaryOp::BitNot,
        }
    }
}

/// Augmented assignment operation type
#[derive(Debug, Clone, PartialEq)]
pub enum AugAssignOp {
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,
    BitOr,
    BitXor,
    BitAnd,
    LShift,
    RShift,
}

impl AugAssignOp {
    /// Convert from Python AST Operator
    pub fn from_operator(op: &Operator) -> Self {
        match op {
            Operator::Add => AugAssignOp::Add,
            Operator::Sub => AugAssignOp::Sub,
            Operator::Mult => AugAssignOp::Mul,
            Operator::Div => AugAssignOp::Div,
            Operator::FloorDiv => AugAssignOp::FloorDiv,
            Operator::Mod => AugAssignOp::Mod,
            Operator::Pow => AugAssignOp::Pow,
            Operator::BitOr => AugAssignOp::BitOr,
            Operator::BitXor => AugAssignOp::BitXor,
            Operator::BitAnd => AugAssignOp::BitAnd,
            Operator::LShift => AugAssignOp::LShift,
            Operator::RShift => AugAssignOp::RShift,
            Operator::MatMult => AugAssignOp::Mul,
        }
    }

    /// Convert to BinaryOp for codegen
    pub fn to_binary_op(&self) -> BinaryOp {
        match self {
            AugAssignOp::Add => BinaryOp::Add,
            AugAssignOp::Sub => BinaryOp::Sub,
            AugAssignOp::Mul => BinaryOp::Mul,
            AugAssignOp::Div => BinaryOp::Div,
            AugAssignOp::FloorDiv => BinaryOp::FloorDiv,
            AugAssignOp::Mod => BinaryOp::Mod,
            AugAssignOp::Pow => BinaryOp::Pow,
            AugAssignOp::BitOr => BinaryOp::BitOr,
            AugAssignOp::BitXor => BinaryOp::BitXor,
            AugAssignOp::BitAnd => BinaryOp::BitAnd,
            AugAssignOp::LShift => BinaryOp::LShift,
            AugAssignOp::RShift => BinaryOp::RShift,
        }
    }
}

/// Type annotation - represents Python type hints
/// This is used internally by codegen for type tracking
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Bytes,
    None,
    Range,
    List(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Set(Box<Type>),
    Tuple(Vec<Type>),
    Custom(String),
}

impl Type {
    /// Parse a type from a Python AST expression (type annotation)
    pub fn from_expr(expr: &Expr) -> Result<Self, String> {
        match expr {
            Expr::Name(name) => match name.id.as_str() {
                "int" => Ok(Type::Int),
                "float" => Ok(Type::Float),
                "bool" => Ok(Type::Bool),
                "str" => Ok(Type::Str),
                "bytes" => Ok(Type::Bytes),
                "None" => Ok(Type::None),
                "range" => Ok(Type::Range),
                other => Ok(Type::Custom(other.to_string())),
            },
            Expr::Subscript(sub) => {
                let base = if let Expr::Name(name) = sub.value.as_ref() {
                    name.id.as_str()
                } else {
                    return Err("Invalid generic type base".to_string());
                };

                match base {
                    "list" => {
                        let elem = Type::from_expr(&sub.slice)?;
                        Ok(Type::List(Box::new(elem)))
                    }
                    "dict" => {
                        if let Expr::Tuple(tuple) = sub.slice.as_ref() {
                            if tuple.elts.len() == 2 {
                                let key = Type::from_expr(&tuple.elts[0])?;
                                let val = Type::from_expr(&tuple.elts[1])?;
                                return Ok(Type::Dict(Box::new(key), Box::new(val)));
                            }
                        }
                        Err("Dict type requires two type arguments".to_string())
                    }
                    "set" => {
                        let elem = Type::from_expr(&sub.slice)?;
                        Ok(Type::Set(Box::new(elem)))
                    }
                    "tuple" => {
                        if let Expr::Tuple(tuple) = sub.slice.as_ref() {
                            let types: Result<Vec<_>, _> =
                                tuple.elts.iter().map(Type::from_expr).collect();
                            Ok(Type::Tuple(types?))
                        } else {
                            let elem = Type::from_expr(&sub.slice)?;
                            Ok(Type::Tuple(vec![elem]))
                        }
                    }
                    other => Ok(Type::Custom(other.to_string())),
                }
            }
            Expr::Constant(c) => {
                if c.value.is_none() {
                    Ok(Type::None)
                } else if let Some(s) = c.value.as_str() {
                    // Forward reference as string
                    Ok(Type::Custom(s.to_string()))
                } else {
                    Err("Invalid type annotation".to_string())
                }
            }
            _ => Err(format!("Unsupported type annotation: {:?}", expr)),
        }
    }
}

/// Location information for AST nodes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Location {
    #[serde(default)]
    pub lineno: Option<u32>,
    #[serde(default)]
    pub col_offset: Option<u32>,
    #[serde(default)]
    pub end_lineno: Option<u32>,
    #[serde(default)]
    pub end_col_offset: Option<u32>,
}

/// Top-level module - the root of a parsed Python file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub type_ignores: Vec<TypeIgnore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeIgnore {
    pub lineno: u32,
    pub tag: String,
}

// ============================================================================
// Statements
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Stmt {
    FunctionDef(FunctionDef),
    AsyncFunctionDef(AsyncFunctionDef),
    ClassDef(ClassDef),
    Return(Return),
    Delete(Delete),
    Assign(Assign),
    AugAssign(AugAssign),
    AnnAssign(AnnAssign),
    For(For),
    AsyncFor(AsyncFor),
    While(While),
    If(If),
    With(With),
    AsyncWith(AsyncWith),
    Match(Match),
    Raise(Raise),
    Try(Try),
    TryStar(TryStar),
    Assert(Assert),
    #[serde(rename = "Import")]
    Import(PyImport),
    ImportFrom(ImportFrom),
    Global(Global),
    Nonlocal(Nonlocal),
    Expr(ExprStmt),
    Pass(Pass),
    Break(Break),
    Continue(Continue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub args: Arguments,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub decorator_list: Vec<Expr>,
    pub returns: Option<Expr>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncFunctionDef {
    pub name: String,
    pub args: Arguments,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub decorator_list: Vec<Expr>,
    pub returns: Option<Expr>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    pub name: String,
    #[serde(default)]
    pub bases: Vec<Expr>,
    #[serde(default)]
    pub keywords: Vec<Keyword>,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub decorator_list: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Return {
    pub value: Option<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delete {
    pub targets: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assign {
    pub targets: Vec<Expr>,
    pub value: Expr,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugAssign {
    pub target: Expr,
    pub op: Operator,
    pub value: Expr,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnAssign {
    pub target: Expr,
    pub annotation: Expr,
    pub value: Option<Expr>,
    pub simple: i32,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct For {
    pub target: Expr,
    pub iter: Expr,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncFor {
    pub target: Expr,
    pub iter: Expr,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct While {
    pub test: Expr,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct If {
    pub test: Expr,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct With {
    pub items: Vec<WithItem>,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncWith {
    pub items: Vec<WithItem>,
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub subject: Expr,
    pub cases: Vec<MatchCase>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Raise {
    pub exc: Option<Expr>,
    pub cause: Option<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Try {
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub handlers: Vec<PyExceptHandler>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(default)]
    pub finalbody: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryStar {
    pub body: Vec<Stmt>,
    #[serde(default)]
    pub handlers: Vec<PyExceptHandler>,
    #[serde(default)]
    pub orelse: Vec<Stmt>,
    #[serde(default)]
    pub finalbody: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assert {
    pub test: Expr,
    pub msg: Option<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyImport {
    pub names: Vec<Alias>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportFrom {
    pub module: Option<String>,
    pub names: Vec<Alias>,
    pub level: i32,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Global {
    pub names: Vec<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nonlocal {
    pub names: Vec<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExprStmt {
    pub value: Expr,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pass {
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Break {
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Continue {
    #[serde(flatten)]
    pub location: Location,
}

// ============================================================================
// Expressions
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Expr {
    BoolOp(BoolOpExpr),
    NamedExpr(NamedExpr),
    BinOp(BinOpExpr),
    UnaryOp(UnaryOpExpr),
    Lambda(Lambda),
    IfExp(IfExp),
    Dict(DictExpr),
    Set(SetExpr),
    ListComp(ListComp),
    SetComp(SetComp),
    DictComp(DictComp),
    GeneratorExp(GeneratorExp),
    Await(Await),
    Yield(Yield),
    YieldFrom(YieldFrom),
    Compare(Compare),
    Call(Call),
    FormattedValue(FormattedValue),
    JoinedStr(JoinedStr),
    Constant(Constant),
    Attribute(Attribute),
    Subscript(Subscript),
    Starred(Starred),
    Name(Name),
    List(ListExpr),
    Tuple(TupleExpr),
    Slice(Slice),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolOpExpr {
    pub op: BoolOpKind,
    pub values: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedExpr {
    pub target: Box<Expr>,
    pub value: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinOpExpr {
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryOpExpr {
    pub op: UnaryOpKind,
    pub operand: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lambda {
    pub args: Box<Arguments>,
    pub body: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfExp {
    pub test: Box<Expr>,
    pub body: Box<Expr>,
    pub orelse: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictExpr {
    pub keys: Vec<Option<Expr>>, // None for **kwargs unpacking
    pub values: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetExpr {
    pub elts: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListComp {
    pub elt: Box<Expr>,
    pub generators: Vec<Comprehension>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetComp {
    pub elt: Box<Expr>,
    pub generators: Vec<Comprehension>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictComp {
    pub key: Box<Expr>,
    pub value: Box<Expr>,
    pub generators: Vec<Comprehension>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorExp {
    pub elt: Box<Expr>,
    pub generators: Vec<Comprehension>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Await {
    pub value: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Yield {
    pub value: Option<Box<Expr>>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldFrom {
    pub value: Box<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compare {
    pub left: Box<Expr>,
    pub ops: Vec<CmpOp>,
    pub comparators: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    pub func: Box<Expr>,
    #[serde(default)]
    pub args: Vec<Expr>,
    #[serde(default)]
    pub keywords: Vec<Keyword>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattedValue {
    pub value: Box<Expr>,
    pub conversion: i32,
    pub format_spec: Option<Box<Expr>>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinedStr {
    pub values: Vec<Expr>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    pub value: ConstantValue,
    #[serde(flatten)]
    pub location: Location,
}

/// Constant values in Python AST
/// The order matters for serde untagged deserialization - more specific types first
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConstantValue {
    /// None value - must check for null
    None,
    /// Boolean (must come before Int since Python bool is subclass of int)
    Bool(bool),
    /// Integer
    Int(i64),
    /// Float
    Float(f64),
    /// String
    Str(String),
    /// Bytes literal
    Bytes {
        #[serde(rename = "_bytes")]
        bytes: Vec<u8>,
    },
    /// Complex number
    Complex {
        #[serde(rename = "_complex")]
        complex: ComplexValue,
    },
    /// Ellipsis (...)
    Ellipsis {
        #[serde(rename = "_ellipsis")]
        ellipsis: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexValue {
    pub real: f64,
    pub imag: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub value: Box<Expr>,
    pub attr: String,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscript {
    pub value: Box<Expr>,
    pub slice: Box<Expr>,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Starred {
    pub value: Box<Expr>,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub id: String,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExpr {
    pub elts: Vec<Expr>,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TupleExpr {
    pub elts: Vec<Expr>,
    pub ctx: ExprContext,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slice {
    pub lower: Option<Box<Expr>>,
    pub upper: Option<Box<Expr>>,
    pub step: Option<Box<Expr>>,
    #[serde(flatten)]
    pub location: Location,
}

// ============================================================================
// Operators
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum BoolOpKind {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum UnaryOpKind {
    Invert,
    Not,
    UAdd,
    USub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum CmpOp {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum ExprContext {
    #[default]
    Load,
    Store,
    Del,
}

// ============================================================================
// Other nodes
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comprehension {
    pub target: Expr,
    pub iter: Expr,
    #[serde(default)]
    pub ifs: Vec<Expr>,
    #[serde(default)]
    pub is_async: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyExceptHandler {
    #[serde(rename = "type")]
    pub type_: Option<Expr>,
    pub name: Option<String>,
    pub body: Vec<Stmt>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arguments {
    #[serde(default)]
    pub posonlyargs: Vec<Arg>,
    #[serde(default)]
    pub args: Vec<Arg>,
    pub vararg: Option<Box<Arg>>,
    #[serde(default)]
    pub kwonlyargs: Vec<Arg>,
    #[serde(default)]
    pub kw_defaults: Vec<Option<Expr>>,
    pub kwarg: Option<Box<Arg>>,
    #[serde(default)]
    pub defaults: Vec<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arg {
    pub arg: String,
    pub annotation: Option<Box<Expr>>,
    #[serde(default)]
    pub type_comment: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub arg: Option<String>, // None for **kwargs
    pub value: Expr,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    pub asname: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithItem {
    pub context_expr: Expr,
    pub optional_vars: Option<Expr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchCase {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Vec<Stmt>,
}

// Pattern matching (Python 3.10+)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum Pattern {
    MatchValue(MatchValue),
    MatchSingleton(MatchSingleton),
    MatchSequence(MatchSequence),
    MatchMapping(MatchMapping),
    MatchClass(MatchClass),
    MatchStar(MatchStar),
    MatchAs(MatchAs),
    MatchOr(MatchOr),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchValue {
    pub value: Expr,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSingleton {
    pub value: ConstantValue,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSequence {
    pub patterns: Vec<Pattern>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchMapping {
    pub keys: Vec<Expr>,
    pub patterns: Vec<Pattern>,
    pub rest: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchClass {
    pub cls: Expr,
    pub patterns: Vec<Pattern>,
    pub kwd_attrs: Vec<String>,
    pub kwd_patterns: Vec<Pattern>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchStar {
    pub name: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchAs {
    pub pattern: Option<Box<Pattern>>,
    pub name: Option<String>,
    #[serde(flatten)]
    pub location: Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchOr {
    pub patterns: Vec<Pattern>,
    #[serde(flatten)]
    pub location: Location,
}

// ============================================================================
// Parsing
// ============================================================================

/// Parse a Python file using the Python AST serializer script
pub fn parse_file(path: &std::path::Path) -> Result<Module, String> {
    let script_path = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .map(|p| p.join("../scripts/ast_to_json.py"))
        .unwrap_or_else(|| std::path::PathBuf::from("scripts/ast_to_json.py"));

    let output = Command::new("python3")
        .arg(&script_path)
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run Python AST parser: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!("Python AST parser failed: {}\n{}", stderr, stdout));
    }

    let json = String::from_utf8_lossy(&output.stdout);
    parse_json(&json)
}

/// Parse JSON from Python ast_to_json.py into Module
pub fn parse_json(json: &str) -> Result<Module, String> {
    // Check for error response
    if json.contains("\"_error\"") {
        #[derive(Deserialize)]
        struct ErrorResponse {
            #[serde(rename = "_error")]
            error: String,
            #[serde(default)]
            msg: Option<String>,
            #[serde(default)]
            lineno: Option<u32>,
        }

        if let Ok(err) = serde_json::from_str::<ErrorResponse>(json) {
            return Err(format!(
                "{}: {} (line {:?})",
                err.error,
                err.msg.unwrap_or_default(),
                err.lineno
            ));
        }
    }

    // The JSON has _type: "Module" at top level
    #[derive(Deserialize)]
    struct ModuleWrapper {
        body: Vec<Stmt>,
        #[serde(default)]
        type_ignores: Vec<TypeIgnore>,
    }

    let wrapper: ModuleWrapper =
        serde_json::from_str(json).map_err(|e| format!("JSON parse error: {}", e))?;

    Ok(Module {
        body: wrapper.body,
        type_ignores: wrapper.type_ignores,
    })
}

// ============================================================================
// Helper methods
// ============================================================================

impl Expr {
    /// Get the name if this is a Name expression
    pub fn as_name(&self) -> Option<&str> {
        match self {
            Expr::Name(n) => Some(&n.id),
            _ => None,
        }
    }

    /// Get the location of this expression
    pub fn location(&self) -> &Location {
        match self {
            Expr::BoolOp(e) => &e.location,
            Expr::NamedExpr(e) => &e.location,
            Expr::BinOp(e) => &e.location,
            Expr::UnaryOp(e) => &e.location,
            Expr::Lambda(e) => &e.location,
            Expr::IfExp(e) => &e.location,
            Expr::Dict(e) => &e.location,
            Expr::Set(e) => &e.location,
            Expr::ListComp(e) => &e.location,
            Expr::SetComp(e) => &e.location,
            Expr::DictComp(e) => &e.location,
            Expr::GeneratorExp(e) => &e.location,
            Expr::Await(e) => &e.location,
            Expr::Yield(e) => &e.location,
            Expr::YieldFrom(e) => &e.location,
            Expr::Compare(e) => &e.location,
            Expr::Call(e) => &e.location,
            Expr::FormattedValue(e) => &e.location,
            Expr::JoinedStr(e) => &e.location,
            Expr::Constant(e) => &e.location,
            Expr::Attribute(e) => &e.location,
            Expr::Subscript(e) => &e.location,
            Expr::Starred(e) => &e.location,
            Expr::Name(e) => &e.location,
            Expr::List(e) => &e.location,
            Expr::Tuple(e) => &e.location,
            Expr::Slice(e) => &e.location,
        }
    }
}

impl ConstantValue {
    /// Check if this is None
    pub fn is_none(&self) -> bool {
        matches!(self, ConstantValue::None)
    }

    /// Try to get as bytes
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            ConstantValue::Bytes { bytes } => Some(bytes),
            _ => None,
        }
    }

    /// Try to get as integer
    pub fn as_int(&self) -> Option<i64> {
        match self {
            ConstantValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Try to get as float
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ConstantValue::Float(f) => Some(*f),
            ConstantValue::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Try to get as string
    pub fn as_str(&self) -> Option<&str> {
        match self {
            ConstantValue::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as bool
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ConstantValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

// ============================================================================
// Visitor pattern for legacy AST traversal (used by codegen)
// ============================================================================

/// Visitor trait for AST traversal - uses legacy types for codegen compatibility
pub mod visitor {
    use super::*;

    /// The Visitor trait defines methods for visiting each AST node type.
    pub trait Visitor {
        type Result;
        type Error;

        /// Visit a program node
        fn visit_program(&mut self, program: &Program) -> Result<Self::Result, Self::Error>;

        /// Visit an import statement
        fn visit_import(&mut self, import: &Import) -> Result<(), Self::Error>;

        /// Visit a function declaration
        fn visit_function(&mut self, function: &Function) -> Result<(), Self::Error>;

        /// Called when entering a function (before visiting body)
        fn enter_function(&mut self, function: &Function) -> Result<(), Self::Error>;

        /// Called when exiting a function (after visiting body)
        fn exit_function(&mut self, function: &Function) -> Result<(), Self::Error>;

        /// Visit a class declaration
        fn visit_class(&mut self, class: &Class) -> Result<(), Self::Error>;

        /// Visit a statement
        fn visit_statement(&mut self, statement: &Statement) -> Result<(), Self::Error> {
            match statement {
                Statement::VarDecl {
                    name,
                    var_type,
                    value,
                } => self.visit_var_decl(name, var_type, value),
                Statement::AnnotatedAssignment {
                    target,
                    var_type,
                    value,
                } => self.visit_annotated_assignment(target, var_type, value),
                Statement::Assignment { target, value } => self.visit_assignment(target, value),
                Statement::TupleUnpackAssignment { targets, value } => {
                    self.visit_tuple_unpack_assignment(targets, value)
                }
                Statement::AugAssignment { target, op, value } => {
                    self.visit_aug_assignment(target, op, value)
                }
                Statement::If {
                    condition,
                    then_block,
                    elif_clauses,
                    else_block,
                } => self.visit_if(condition, then_block, elif_clauses, else_block),
                Statement::While { condition, body } => self.visit_while(condition, body),
                Statement::For {
                    targets,
                    iter,
                    body,
                    else_block,
                } => self.visit_for(targets, iter, body, else_block),
                Statement::Return(expr) => self.visit_return(expr.as_ref()),
                Statement::Break => self.visit_break(),
                Statement::Continue => self.visit_continue(),
                Statement::Pass => self.visit_pass(),
                Statement::Delete(target) => self.visit_delete(target),
                Statement::Expr(expr) => self.visit_expr_statement(expr),
                Statement::Try {
                    body,
                    handlers,
                    else_block,
                    finally_block,
                } => self.visit_try(body, handlers, else_block, finally_block),
                Statement::Raise { exception, cause } => self.visit_raise(exception, cause),
                Statement::Assert { test, msg } => self.visit_assert(test, msg),
                Statement::Global { names } => self.visit_global(names),
                Statement::Nonlocal { names } => self.visit_nonlocal(names),
            }
        }

        /// Visit variable declaration
        fn visit_var_decl(
            &mut self,
            name: &str,
            var_type: &Type,
            value: &Expression,
        ) -> Result<(), Self::Error>;

        /// Visit annotated assignment (e.g., self.x: int = 5)
        fn visit_annotated_assignment(
            &mut self,
            target: &Expression,
            var_type: &Type,
            value: &Expression,
        ) -> Result<(), Self::Error>;

        /// Visit assignment statement
        fn visit_assignment(
            &mut self,
            target: &Expression,
            value: &Expression,
        ) -> Result<(), Self::Error>;

        /// Visit tuple unpacking assignment
        fn visit_tuple_unpack_assignment(
            &mut self,
            targets: &[Expression],
            value: &Expression,
        ) -> Result<(), Self::Error>;

        /// Visit augmented assignment statement
        fn visit_aug_assignment(
            &mut self,
            target: &Expression,
            op: &AugAssignOp,
            value: &Expression,
        ) -> Result<(), Self::Error>;

        /// Visit if statement
        fn visit_if(
            &mut self,
            condition: &Expression,
            then_block: &[Statement],
            elif_clauses: &[(Expression, Vec<Statement>)],
            else_block: &Option<Vec<Statement>>,
        ) -> Result<(), Self::Error>;

        /// Visit while statement
        fn visit_while(
            &mut self,
            condition: &Expression,
            body: &[Statement],
        ) -> Result<(), Self::Error>;

        /// Visit for statement
        fn visit_for(
            &mut self,
            targets: &[String],
            iter: &Expression,
            body: &[Statement],
            else_block: &Option<Vec<Statement>>,
        ) -> Result<(), Self::Error>;

        /// Visit return statement
        fn visit_return(&mut self, expr: Option<&Expression>) -> Result<(), Self::Error>;

        /// Visit break statement
        fn visit_break(&mut self) -> Result<(), Self::Error>;

        /// Visit continue statement
        fn visit_continue(&mut self) -> Result<(), Self::Error>;

        /// Visit pass statement
        fn visit_pass(&mut self) -> Result<(), Self::Error>;

        /// Visit delete statement
        fn visit_delete(&mut self, target: &Expression) -> Result<(), Self::Error>;

        /// Visit expression statement
        fn visit_expr_statement(&mut self, expr: &Expression) -> Result<(), Self::Error>;

        /// Visit try statement
        fn visit_try(
            &mut self,
            body: &[Statement],
            handlers: &[LegacyExceptHandler],
            else_block: &Option<Vec<Statement>>,
            finally_block: &Option<Vec<Statement>>,
        ) -> Result<(), Self::Error>;

        /// Visit raise statement
        fn visit_raise(
            &mut self,
            exception: &Option<Expression>,
            cause: &Option<Expression>,
        ) -> Result<(), Self::Error>;

        /// Visit assert statement
        fn visit_assert(
            &mut self,
            test: &Expression,
            msg: &Option<Expression>,
        ) -> Result<(), Self::Error>;

        /// Visit global statement
        fn visit_global(&mut self, names: &[String]) -> Result<(), Self::Error>;

        /// Visit nonlocal statement
        fn visit_nonlocal(&mut self, names: &[String]) -> Result<(), Self::Error>;
    }
}

/// Type alias for backwards compatibility - maps to LegacyExceptHandler
pub type ExceptHandler = LegacyExceptHandler;

// ============================================================================
// Conversion from Python AST to Legacy AST
// ============================================================================

impl Program {
    /// Convert a Python AST Module to a legacy Program
    pub fn from_module(module: &Module) -> Result<Self, String> {
        let mut imports = Vec::new();
        let mut classes = Vec::new();
        let mut functions = Vec::new();
        let mut statements = Vec::new();

        for stmt in &module.body {
            match stmt {
                Stmt::Import(imp) => {
                    for alias in &imp.names {
                        imports.push(Import {
                            module_path: alias.name.split('.').map(String::from).collect(),
                        });
                    }
                }
                Stmt::ImportFrom(imp) => {
                    if let Some(module_name) = &imp.module {
                        imports.push(Import {
                            module_path: module_name.split('.').map(String::from).collect(),
                        });
                    }
                }
                Stmt::ClassDef(class_def) => {
                    classes.push(Class::from_class_def(class_def)?);
                }
                Stmt::FunctionDef(func_def) => {
                    functions.push(Function::from_function_def(func_def)?);
                }
                _ => {
                    statements.push(Statement::from_stmt(stmt)?);
                }
            }
        }

        Ok(Program {
            imports,
            classes,
            functions,
            statements,
        })
    }
}

impl Class {
    /// Convert a Python AST ClassDef to a legacy Class
    pub fn from_class_def(class_def: &ClassDef) -> Result<Self, String> {
        let name = class_def.name.clone();

        // Get base class if any
        let base = class_def.bases.first().and_then(|expr| {
            if let Expr::Name(n) = expr {
                Some(n.id.clone())
            } else {
                None
            }
        });

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        for stmt in &class_def.body {
            match stmt {
                Stmt::FunctionDef(func_def) => {
                    methods.push(Method::from_function_def(func_def)?);
                }
                Stmt::AnnAssign(ann) => {
                    // Class field with type annotation
                    if let Expr::Name(n) = &ann.target {
                        let field_type = Type::from_expr(&ann.annotation)?;
                        let default = ann.value.as_ref().map(Expression::from_expr).transpose()?;
                        fields.push(ClassField {
                            name: n.id.clone(),
                            field_type,
                            default,
                        });
                    }
                }
                Stmt::Pass(_) => {}
                _ => {}
            }
        }

        Ok(Class {
            name,
            base,
            fields,
            methods,
        })
    }
}

impl Method {
    /// Convert a Python AST FunctionDef to a legacy Method
    pub fn from_function_def(func_def: &FunctionDef) -> Result<Self, String> {
        let name = func_def.name.clone();

        let mut params = Vec::new();
        for arg in &func_def.args.args {
            let param_type = if let Some(ann) = &arg.annotation {
                Type::from_expr(ann)?
            } else if arg.arg == "self" {
                Type::Custom("Self".to_string())
            } else {
                Type::Custom("Any".to_string())
            };
            params.push(Parameter {
                name: arg.arg.clone(),
                param_type,
            });
        }

        let return_type = if let Some(ret) = &func_def.returns {
            Type::from_expr(ret)?
        } else {
            Type::None
        };

        let mut body = Vec::new();
        for stmt in &func_def.body {
            body.push(Statement::from_stmt(stmt)?);
        }

        Ok(Method {
            name,
            params,
            return_type,
            body,
        })
    }
}

impl Function {
    /// Convert a Python AST FunctionDef to a legacy Function
    pub fn from_function_def(func_def: &FunctionDef) -> Result<Self, String> {
        let name = func_def.name.clone();

        let mut params = Vec::new();
        for arg in &func_def.args.args {
            let param_type = if let Some(ann) = &arg.annotation {
                Type::from_expr(ann)?
            } else {
                Type::Custom("Any".to_string())
            };
            params.push(Parameter {
                name: arg.arg.clone(),
                param_type,
            });
        }

        let return_type = if let Some(ret) = &func_def.returns {
            Type::from_expr(ret)?
        } else {
            Type::None
        };

        let mut body = Vec::new();
        for stmt in &func_def.body {
            body.push(Statement::from_stmt(stmt)?);
        }

        Ok(Function {
            name,
            params,
            return_type,
            body,
        })
    }
}

impl Statement {
    /// Convert a Python AST Stmt to a legacy Statement
    pub fn from_stmt(stmt: &Stmt) -> Result<Self, String> {
        match stmt {
            Stmt::Assign(assign) => {
                let value = Expression::from_expr(&assign.value)?;

                if assign.targets.len() == 1 {
                    let target = &assign.targets[0];

                    // Check if it's a tuple unpack
                    if let Expr::Tuple(tuple) = target {
                        let targets: Result<Vec<_>, _> =
                            tuple.elts.iter().map(Expression::from_expr).collect();
                        return Ok(Statement::TupleUnpackAssignment {
                            targets: targets?,
                            value,
                        });
                    }

                    Ok(Statement::Assignment {
                        target: Expression::from_expr(target)?,
                        value,
                    })
                } else {
                    // Multiple targets - treat first as main target
                    Ok(Statement::Assignment {
                        target: Expression::from_expr(&assign.targets[0])?,
                        value,
                    })
                }
            }
            Stmt::AnnAssign(ann) => {
                let target = Expression::from_expr(&ann.target)?;
                let var_type = Type::from_expr(&ann.annotation)?;

                if let Some(val) = &ann.value {
                    let value = Expression::from_expr(val)?;

                    // Check if target is a simple name (VarDecl) or complex (AnnotatedAssignment)
                    if let Expr::Name(n) = &ann.target {
                        Ok(Statement::VarDecl {
                            name: n.id.clone(),
                            var_type,
                            value,
                        })
                    } else {
                        Ok(Statement::AnnotatedAssignment {
                            target,
                            var_type,
                            value,
                        })
                    }
                } else {
                    // Annotation without value - create a placeholder
                    Ok(Statement::VarDecl {
                        name: if let Expression::Var(n) = &target {
                            n.clone()
                        } else {
                            "".to_string()
                        },
                        var_type,
                        value: Expression::NoneLit,
                    })
                }
            }
            Stmt::AugAssign(aug) => Ok(Statement::AugAssignment {
                target: Expression::from_expr(&aug.target)?,
                op: AugAssignOp::from_operator(&aug.op),
                value: Expression::from_expr(&aug.value)?,
            }),
            Stmt::If(if_stmt) => {
                let condition = Expression::from_expr(&if_stmt.test)?;
                let then_block: Result<Vec<_>, _> =
                    if_stmt.body.iter().map(Statement::from_stmt).collect();

                // Process elif/else chain
                let mut elif_clauses = Vec::new();
                let mut else_block = None;

                let mut current_else = &if_stmt.orelse;
                while !current_else.is_empty() {
                    if current_else.len() == 1 {
                        if let Stmt::If(elif) = &current_else[0] {
                            let elif_cond = Expression::from_expr(&elif.test)?;
                            let elif_body: Result<Vec<_>, _> =
                                elif.body.iter().map(Statement::from_stmt).collect();
                            elif_clauses.push((elif_cond, elif_body?));
                            current_else = &elif.orelse;
                            continue;
                        }
                    }
                    // Not an elif, this is the final else block
                    let else_stmts: Result<Vec<_>, _> =
                        current_else.iter().map(Statement::from_stmt).collect();
                    else_block = Some(else_stmts?);
                    break;
                }

                Ok(Statement::If {
                    condition,
                    then_block: then_block?,
                    elif_clauses,
                    else_block,
                })
            }
            Stmt::While(while_stmt) => {
                let condition = Expression::from_expr(&while_stmt.test)?;
                let body: Result<Vec<_>, _> =
                    while_stmt.body.iter().map(Statement::from_stmt).collect();
                Ok(Statement::While {
                    condition,
                    body: body?,
                })
            }
            Stmt::For(for_stmt) => {
                let iter = Expression::from_expr(&for_stmt.iter)?;
                let body: Result<Vec<_>, _> =
                    for_stmt.body.iter().map(Statement::from_stmt).collect();

                // Extract target names
                let targets = extract_target_names(&for_stmt.target);

                let else_block = if for_stmt.orelse.is_empty() {
                    None
                } else {
                    let stmts: Result<Vec<_>, _> =
                        for_stmt.orelse.iter().map(Statement::from_stmt).collect();
                    Some(stmts?)
                };

                Ok(Statement::For {
                    targets,
                    iter,
                    body: body?,
                    else_block,
                })
            }
            Stmt::Return(ret) => {
                let value = ret.value.as_ref().map(Expression::from_expr).transpose()?;
                Ok(Statement::Return(value))
            }
            Stmt::Break(_) => Ok(Statement::Break),
            Stmt::Continue(_) => Ok(Statement::Continue),
            Stmt::Pass(_) => Ok(Statement::Pass),
            Stmt::Delete(del) => {
                if let Some(first) = del.targets.first() {
                    Ok(Statement::Delete(Expression::from_expr(first)?))
                } else {
                    Ok(Statement::Pass)
                }
            }
            Stmt::Expr(expr_stmt) => Ok(Statement::Expr(Expression::from_expr(&expr_stmt.value)?)),
            Stmt::Try(try_stmt) => {
                let body: Result<Vec<_>, _> =
                    try_stmt.body.iter().map(Statement::from_stmt).collect();

                let handlers: Result<Vec<LegacyExceptHandler>, String> = try_stmt
                    .handlers
                    .iter()
                    .map(|h| {
                        let exception_types = if let Some(type_expr) = &h.type_ {
                            if let Expr::Name(n) = type_expr {
                                vec![n.id.clone()]
                            } else if let Expr::Tuple(t) = type_expr {
                                t.elts
                                    .iter()
                                    .filter_map(|e| {
                                        if let Expr::Name(n) = e {
                                            Some(n.id.clone())
                                        } else {
                                            None
                                        }
                                    })
                                    .collect()
                            } else {
                                vec![]
                            }
                        } else {
                            vec![]
                        };

                        let handler_body: Result<Vec<Statement>, String> =
                            h.body.iter().map(Statement::from_stmt).collect();

                        Ok(LegacyExceptHandler {
                            exception_types,
                            name: h.name.clone(),
                            body: handler_body?,
                        })
                    })
                    .collect();

                let else_block = if try_stmt.orelse.is_empty() {
                    None
                } else {
                    let stmts: Result<Vec<_>, _> =
                        try_stmt.orelse.iter().map(Statement::from_stmt).collect();
                    Some(stmts?)
                };

                let finally_block = if try_stmt.finalbody.is_empty() {
                    None
                } else {
                    let stmts: Result<Vec<_>, _> = try_stmt
                        .finalbody
                        .iter()
                        .map(Statement::from_stmt)
                        .collect();
                    Some(stmts?)
                };

                Ok(Statement::Try {
                    body: body?,
                    handlers: handlers?,
                    else_block,
                    finally_block,
                })
            }
            Stmt::Raise(raise) => {
                let exception = raise.exc.as_ref().map(Expression::from_expr).transpose()?;
                let cause = raise
                    .cause
                    .as_ref()
                    .map(Expression::from_expr)
                    .transpose()?;
                Ok(Statement::Raise { exception, cause })
            }
            Stmt::Assert(assert) => {
                let test = Expression::from_expr(&assert.test)?;
                let msg = assert.msg.as_ref().map(Expression::from_expr).transpose()?;
                Ok(Statement::Assert { test, msg })
            }
            Stmt::Global(global) => Ok(Statement::Global {
                names: global.names.clone(),
            }),
            Stmt::Nonlocal(nonlocal) => Ok(Statement::Nonlocal {
                names: nonlocal.names.clone(),
            }),
            _ => Err(format!("Unsupported statement: {:?}", stmt)),
        }
    }
}

impl Expression {
    /// Convert a Python AST Expr to a legacy Expression
    pub fn from_expr(expr: &Expr) -> Result<Self, String> {
        match expr {
            Expr::Constant(c) => match &c.value {
                ConstantValue::Int(i) => Ok(Expression::IntLit(*i)),
                ConstantValue::Float(f) => Ok(Expression::FloatLit(*f)),
                ConstantValue::Bool(b) => Ok(Expression::BoolLit(*b)),
                ConstantValue::Str(s) => Ok(Expression::StrLit(s.clone())),
                ConstantValue::Bytes { bytes } => Ok(Expression::BytesLit(bytes.clone())),
                ConstantValue::None => Ok(Expression::NoneLit),
                ConstantValue::Complex { .. } => Err("Complex numbers not supported".to_string()),
                ConstantValue::Ellipsis { .. } => Err("Ellipsis not supported".to_string()),
            },
            Expr::Name(n) => Ok(Expression::Var(n.id.clone())),
            Expr::List(l) => {
                let elts: Result<Vec<_>, _> = l.elts.iter().map(Expression::from_expr).collect();
                Ok(Expression::List(elts?))
            }
            Expr::Tuple(t) => {
                let elts: Result<Vec<_>, _> = t.elts.iter().map(Expression::from_expr).collect();
                Ok(Expression::Tuple(elts?))
            }
            Expr::Dict(d) => {
                let mut pairs = Vec::new();
                for (k, v) in d.keys.iter().zip(d.values.iter()) {
                    if let Some(key) = k {
                        pairs.push((Expression::from_expr(key)?, Expression::from_expr(v)?));
                    }
                }
                Ok(Expression::Dict(pairs))
            }
            Expr::Set(s) => {
                let elts: Result<Vec<_>, _> = s.elts.iter().map(Expression::from_expr).collect();
                Ok(Expression::Set(elts?))
            }
            Expr::BinOp(b) => {
                let op = BinaryOp::from_operator(&b.op);
                let left = Box::new(Expression::from_expr(&b.left)?);
                let right = Box::new(Expression::from_expr(&b.right)?);
                Ok(Expression::BinOp { op, left, right })
            }
            Expr::UnaryOp(u) => {
                let op = UnaryOp::from_unary_op_kind(&u.op);
                let operand = Box::new(Expression::from_expr(&u.operand)?);
                Ok(Expression::UnaryOp { op, operand })
            }
            Expr::BoolOp(b) => {
                let op = BinaryOp::from_bool_op(&b.op);
                // Chain boolean operations left to right
                let mut iter = b.values.iter();
                let first = Expression::from_expr(iter.next().ok_or("Empty BoolOp")?)?;
                let result = iter.try_fold(first, |acc, val| {
                    Ok::<_, String>(Expression::BinOp {
                        op: op.clone(),
                        left: Box::new(acc),
                        right: Box::new(Expression::from_expr(val)?),
                    })
                })?;
                Ok(result)
            }
            Expr::Compare(c) => {
                // Chain comparisons
                let mut left = Expression::from_expr(&c.left)?;
                let mut result = None;

                for (op, comparator) in c.ops.iter().zip(c.comparators.iter()) {
                    let bin_op = BinaryOp::from_cmp_op(op);
                    let right = Expression::from_expr(comparator)?;

                    let comparison = Expression::BinOp {
                        op: bin_op,
                        left: Box::new(left.clone()),
                        right: Box::new(right.clone()),
                    };

                    result = Some(match result {
                        None => comparison,
                        Some(prev) => Expression::BinOp {
                            op: BinaryOp::And,
                            left: Box::new(prev),
                            right: Box::new(comparison),
                        },
                    });

                    left = right;
                }

                result.ok_or_else(|| "Empty comparison".to_string())
            }
            Expr::Call(c) => {
                let func = Box::new(Expression::from_expr(&c.func)?);
                let args: Result<Vec<_>, _> = c.args.iter().map(Expression::from_expr).collect();
                Ok(Expression::Call { func, args: args? })
            }
            Expr::Attribute(a) => {
                let object = Box::new(Expression::from_expr(&a.value)?);
                Ok(Expression::Attribute {
                    object,
                    attr: a.attr.clone(),
                })
            }
            Expr::Subscript(s) => {
                let object = Box::new(Expression::from_expr(&s.value)?);
                let index = Box::new(Expression::from_expr(&s.slice)?);
                Ok(Expression::Subscript { object, index })
            }
            Expr::Slice(s) => {
                let start = s
                    .lower
                    .as_ref()
                    .map(|e| Expression::from_expr(e).map(Box::new))
                    .transpose()?;
                let stop = s
                    .upper
                    .as_ref()
                    .map(|e| Expression::from_expr(e).map(Box::new))
                    .transpose()?;
                let step = s
                    .step
                    .as_ref()
                    .map(|e| Expression::from_expr(e).map(Box::new))
                    .transpose()?;
                Ok(Expression::Slice { start, stop, step })
            }
            Expr::Yield(y) => {
                let value = y
                    .value
                    .as_ref()
                    .map(|e| Expression::from_expr(e).map(Box::new))
                    .transpose()?;
                Ok(Expression::Yield {
                    value,
                    is_from: false,
                })
            }
            Expr::YieldFrom(y) => {
                let value = Some(Box::new(Expression::from_expr(&y.value)?));
                Ok(Expression::Yield {
                    value,
                    is_from: true,
                })
            }
            Expr::ListComp(l) => {
                let element = Box::new(Expression::from_expr(&l.elt)?);
                let clauses: Result<Vec<_>, _> = l
                    .generators
                    .iter()
                    .map(ComprehensionClause::from_comprehension)
                    .collect();
                Ok(Expression::ListComprehension {
                    element,
                    clauses: clauses?,
                })
            }
            Expr::DictComp(d) => {
                let key = Box::new(Expression::from_expr(&d.key)?);
                let value = Box::new(Expression::from_expr(&d.value)?);
                let clauses: Result<Vec<_>, _> = d
                    .generators
                    .iter()
                    .map(ComprehensionClause::from_comprehension)
                    .collect();
                Ok(Expression::DictComprehension {
                    key,
                    value,
                    clauses: clauses?,
                })
            }
            Expr::SetComp(s) => {
                let element = Box::new(Expression::from_expr(&s.elt)?);
                let clauses: Result<Vec<_>, _> = s
                    .generators
                    .iter()
                    .map(ComprehensionClause::from_comprehension)
                    .collect();
                Ok(Expression::SetComprehension {
                    element,
                    clauses: clauses?,
                })
            }
            Expr::GeneratorExp(g) => {
                let element = Box::new(Expression::from_expr(&g.elt)?);
                let clauses: Result<Vec<_>, _> = g
                    .generators
                    .iter()
                    .map(ComprehensionClause::from_comprehension)
                    .collect();
                Ok(Expression::GeneratorExpression {
                    element,
                    clauses: clauses?,
                })
            }
            Expr::IfExp(i) => {
                let condition = Box::new(Expression::from_expr(&i.test)?);
                let true_value = Box::new(Expression::from_expr(&i.body)?);
                let false_value = Box::new(Expression::from_expr(&i.orelse)?);
                Ok(Expression::Ternary {
                    condition,
                    true_value,
                    false_value,
                })
            }
            Expr::Starred(s) => {
                // Treat starred as the inner expression for now
                Expression::from_expr(&s.value)
            }
            _ => Err(format!("Unsupported expression: {:?}", expr)),
        }
    }
}

impl ComprehensionClause {
    /// Convert a Python AST Comprehension to a legacy ComprehensionClause
    pub fn from_comprehension(comp: &Comprehension) -> Result<Self, String> {
        let target = extract_target_names(&comp.target);
        let iterable = Box::new(Expression::from_expr(&comp.iter)?);
        let conditions: Result<Vec<_>, _> = comp.ifs.iter().map(Expression::from_expr).collect();

        Ok(ComprehensionClause {
            target,
            iterable,
            conditions: conditions?,
        })
    }
}

/// Extract variable names from a target expression (for for-loops and comprehensions)
fn extract_target_names(expr: &Expr) -> Vec<String> {
    match expr {
        Expr::Name(n) => vec![n.id.clone()],
        Expr::Tuple(t) => t.elts.iter().flat_map(extract_target_names).collect(),
        Expr::List(l) => l.elts.iter().flat_map(extract_target_names).collect(),
        _ => vec![],
    }
}
