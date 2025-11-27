/// AST representation for the TypePython language
pub mod parser;
pub mod visitor;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    None,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub imports: Vec<Import>,
    pub functions: Vec<Function>,
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
pub enum Statement {
    VarDecl {
        name: String,
        var_type: Type,
        value: Expression,
    },
    Assignment {
        name: String,
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
    Return(Option<Expression>),
    Pass,
    Expr(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    BoolLit(bool),
    NoneLit(),
    Var(String),
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
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}
