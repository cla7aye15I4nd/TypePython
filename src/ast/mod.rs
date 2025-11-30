/// AST representation for the TypePython language
pub mod parser;
pub mod visitor;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Bytes, // C-style null-terminated strings
    None,
    Range,                      // range type
    List(Box<Type>),            // List[int], etc.
    Dict(Box<Type>, Box<Type>), // Dict[str, int], etc.
    Set(Box<Type>),             // Set[int], etc.
    Tuple(Vec<Type>),           // Tuple[int, str, ...], etc.
    Custom(String),             // Custom class types
}

#[derive(Debug, Clone)]
pub struct Program {
    pub imports: Vec<Import>,
    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub base: Option<String>,
    pub fields: Vec<ClassField>,
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone)]
pub struct ClassField {
    pub name: String,
    pub field_type: Type,
    pub default: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub params: Vec<Parameter>, // First param should be 'self' for instance methods
    pub return_type: Type,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub module_path: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

/// Exception handler for try/except blocks
#[derive(Debug, Clone)]
pub struct ExceptHandler {
    /// Exception types to catch (empty = bare except)
    pub exception_types: Vec<String>,
    /// Binding name for "as e" syntax
    pub name: Option<String>,
    /// Handler body
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl {
        name: String,
        var_type: Type,
        value: Expression,
    },
    Assignment {
        target: Expression,
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
        targets: Vec<String>, // Single var: ["x"], tuple unpacking: ["x", "y"]
        iter: Expression,
        body: Vec<Statement>,
        else_block: Option<Vec<Statement>>, // Optional else clause
    },
    Return(Option<Expression>),
    Break,
    Continue,
    Pass,
    Delete(Expression),
    Expr(Expression),
    /// Try-except-finally statement
    Try {
        body: Vec<Statement>,
        handlers: Vec<ExceptHandler>,
        else_block: Option<Vec<Statement>>,
        finally_block: Option<Vec<Statement>>,
    },
    /// Raise statement
    Raise {
        exception: Option<Expression>,
        cause: Option<Expression>, // for "raise X from Y"
    },
    /// Assert statement
    Assert {
        test: Expression,
        msg: Option<Expression>,
    },
    /// Global statement - declares variables as global
    Global {
        names: Vec<String>,
    },
    /// Nonlocal statement - declares variables as nonlocal
    Nonlocal {
        names: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub enum AugAssignOp {
    Add,      // +=
    Sub,      // -=
    Mul,      // *=
    Div,      // /=
    FloorDiv, // //=
    Mod,      // %=
    Pow,      // **=
    BitOr,    // |=
    BitXor,   // ^=
    BitAnd,   // &=
    LShift,   // <<=
    RShift,   // >>=
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    BytesLit(String), // Bytes literal b"..."
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
    /// Yield expression for generators
    Yield {
        value: Option<Box<Expression>>,
        is_from: bool, // yield from vs plain yield
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    // Logical
    And,
    Or,
    // Bitwise
    BitOr,
    BitXor,
    BitAnd,
    LShift,
    RShift,
    // Membership/Identity
    In,
    NotIn,
    Is,
    IsNot,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,    // -
    Pos,    // +
    Not,    // not
    BitNot, // ~
}
