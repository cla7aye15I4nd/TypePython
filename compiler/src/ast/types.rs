/// Type annotations in Python source
#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    /// int type
    Int,
    /// float type
    Float,
    /// str type
    Str,
    /// bool type
    Bool,
    /// bytes type (immutable byte sequence)
    Bytes,
    /// bytearray type (mutable byte sequence)
    ByteArray,
    /// list[int] type
    List(Box<TypeAnnotation>),
    /// Class name type (e.g., Point, Rectangle)
    ClassName(String),
}

/// A complete Python module
#[derive(Debug, Clone)]
pub struct Module {
    pub id: crate::ast::ModuleName,
    pub path: std::path::PathBuf,
    pub imports: Vec<crate::ast::ImportInfo>,
    pub body: Vec<Stmt>,
}

/// Function argument with optional type annotation
#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub annotation: Option<TypeAnnotation>,
}

/// An except handler clause in a try statement
#[derive(Debug, Clone)]
pub struct ExceptHandler {
    /// Exception type to catch (None = catch all exceptions)
    pub exc_type: Option<String>,
    /// Variable name to bind the exception to (e.g., `as e`)
    pub name: Option<String>,
    /// Handler body
    pub body: Vec<Stmt>,
}

/// Items that can appear in a class body
#[derive(Debug, Clone)]
pub enum ClassBodyItem {
    /// Field definition with type annotation (e.g., x: int)
    FieldDef {
        name: String,
        annotation: TypeAnnotation,
    },
    /// Method definition (function with implicit self parameter)
    MethodDef {
        name: String,
        args: Vec<Arg>,
        return_type: Option<TypeAnnotation>,
        body: Vec<Stmt>,
    },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOperator {
    Add,      // +
    Sub,      // -
    Mult,     // *
    Div,      // /
    FloorDiv, // //
    Mod,      // %
    Pow,      // **
    LShift,   // <<
    RShift,   // >>
    BitOr,    // |
    BitXor,   // ^
    BitAnd,   // &
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompareOp {
    Eq,    // ==
    NotEq, // !=
    Lt,    // <
    LtE,   // <=
    Gt,    // >
    GtE,   // >=
}

/// Boolean operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoolOp {
    And, // and
    Or,  // or
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Not,  // not
    USub, // - (unary minus)
}

/// Statements
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Function definition
    FunctionDef {
        name: String,
        args: Vec<Arg>,
        return_type: Option<TypeAnnotation>,
        body: Vec<Stmt>,
    },

    /// Class definition
    ClassDef {
        name: String,
        base: Option<String>,
        body: Vec<ClassBodyItem>,
    },

    /// If statement with optional else
    If {
        test: Expr,
        body: Vec<Stmt>,
        orelse: Vec<Stmt>,
    },

    /// While loop
    While { test: Expr, body: Vec<Stmt> },

    /// For loop
    For {
        target: String,
        iter: Expr,
        body: Vec<Stmt>,
    },

    /// Return statement
    Return { value: Option<Expr> },

    /// Assignment with optional type annotation
    /// Target can be Name, Attribute (a.b.c), or Subscript (a[0][1])
    Assign {
        target: Expr,
        value: Expr,
        type_annotation: Option<TypeAnnotation>,
    },

    /// Augmented assignment (+=, -=, etc.)
    AugAssign {
        target: String,
        op: BinOperator,
        value: Expr,
    },

    /// Expression statement (e.g., function call)
    Expr { value: Expr },

    /// Try/except/finally statement
    Try {
        body: Vec<Stmt>,
        handlers: Vec<ExceptHandler>,
        orelse: Vec<Stmt>,    // else clause (runs if no exception)
        finalbody: Vec<Stmt>, // finally clause (always runs)
    },

    /// Raise statement
    Raise {
        exc: Option<Expr>, // None for bare 'raise' (re-raise)
    },
}

/// Constant values
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Bytes(Vec<u8>),
    None,
}

/// Expressions
#[derive(Debug, Clone)]
pub enum Expr {
    /// Constant value
    Constant(Constant),

    /// Variable name
    Name(String),

    /// Binary operation
    BinOp {
        left: Box<Expr>,
        op: BinOperator,
        right: Box<Expr>,
    },

    /// Comparison
    Compare {
        left: Box<Expr>,
        ops: Vec<CompareOp>,
        comparators: Vec<Expr>,
    },

    /// Boolean operation (and, or)
    BoolOp { op: BoolOp, values: Vec<Expr> },

    /// Unary operation (not, -)
    UnaryOp { op: UnaryOp, operand: Box<Expr> },

    /// Function call
    Call { func: Box<Expr>, args: Vec<Expr> },

    /// List literal
    List { elts: Vec<Expr> },

    /// Subscript (e.g., list[0])
    Subscript { value: Box<Expr>, index: Box<Expr> },

    /// Attribute access (e.g., obj.field)
    Attribute { value: Box<Expr>, attr: String },
}
