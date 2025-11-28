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
    List(Box<Type>),            // List[int], etc.
    Dict(Box<Type>, Box<Type>), // Dict[str, int], etc.
    Set(Box<Type>),             // Set[int], etc.
    Tuple(Vec<Type>),           // Tuple[int, str, ...], etc.
    Custom(String),             // Custom class types
}

#[derive(Debug, Clone)]
pub struct Program {
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
    pub classes: Vec<Class>,
    pub statements: Vec<Statement>,
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

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub base_class: Option<String>,
    pub members: Vec<ClassMember>,
}

#[derive(Debug, Clone)]
pub enum ClassMember {
    Method(Function),
    Field {
        name: String,
        field_type: Type,
        value: Expression,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl {
        name: String,
        var_type: Type,
        value: Expression,
    },
    Assignment {
        target: AssignTarget,
        value: Expression,
    },
    AugAssignment {
        target: AssignTarget,
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
        var: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
    Break,
    Continue,
    Pass,
    Expr(Expression),
}

#[derive(Debug, Clone)]
pub enum AssignTarget {
    Var(String),
    Attribute {
        object: Box<Expression>,
        attr: String,
    },
    Subscript {
        object: Box<Expression>,
        index: Box<Expression>,
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
