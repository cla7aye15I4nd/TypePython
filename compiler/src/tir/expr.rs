//! TIR expression representation
//!
//! Expressions in TIR carry their type directly and use resolved IDs for all references.

use crate::ast::{BinOperator, BoolOp, CompareOp, UnaryOp};

use super::ids::{ClassId, FieldId, FuncId, GlobalId, LocalId, ModuleId};
use super::types::TirType;

/// TIR-specific constant type.
/// Unlike AST's Constant, this does not include Bytes (handled by TirExprKind::Bytes).
/// This ensures codegen never has to handle impossible cases.
#[derive(Debug, Clone, PartialEq)]
pub enum TirConstant {
    /// Integer constant
    Int(i64),
    /// Float constant
    Float(f64),
    /// String constant
    Str(String),
    /// Boolean constant
    Bool(bool),
    /// None constant
    None,
}

/// Reference to a variable, distinguishing its scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarRef {
    /// Local variable in current function
    Local(LocalId),

    /// Function parameter (by index)
    Param(u32),

    /// Global variable in a module
    Global(ModuleId, GlobalId),

    /// 'self' reference in methods
    SelfRef,
}

/// Typed expression with embedded type information.
#[derive(Debug, Clone)]
pub struct TirExpr {
    /// The expression kind
    pub kind: TirExprKind,
    /// The resolved type of this expression
    pub ty: TirType,
}

impl TirExpr {
    /// Create a new TirExpr with the given kind and type
    pub fn new(kind: TirExprKind, ty: TirType) -> Self {
        TirExpr { kind, ty }
    }
}

/// Expression variants
#[derive(Debug, Clone)]
pub enum TirExprKind {
    /// Constant value (int, float, str, bool, None)
    /// Uses TirConstant which excludes Bytes (handled by TirExprKind::Bytes)
    Constant(TirConstant),

    /// Variable reference (local, param, global, self)
    Var(VarRef),

    /// Binary operation: left op right
    BinOp {
        left: Box<TirExpr>,
        op: BinOperator,
        right: Box<TirExpr>,
    },

    /// Comparison: left op right
    /// Chained comparisons are desugared during lowering
    Compare {
        left: Box<TirExpr>,
        op: CompareOp,
        right: Box<TirExpr>,
    },

    /// Boolean operation: a and b, a or b
    BoolOp { op: BoolOp, values: Vec<TirExpr> },

    /// Unary operation: not x, -x
    UnaryOp { op: UnaryOp, operand: Box<TirExpr> },

    /// Function call (resolved to FuncId)
    /// For method calls, args[0] is the receiver (self)
    Call { func: FuncId, args: Vec<TirExpr> },

    /// Class constructor: ClassName(args)
    /// Note: range() uses TirExprKind::Range instead
    Construct { class: ClassId, args: Vec<TirExpr> },

    /// Range construction: range(stop), range(start, stop), range(start, stop, step)
    /// Separated from Construct to avoid runtime dispatch in codegen
    Range {
        /// Start value (None means 0)
        start: Option<Box<TirExpr>>,
        /// Stop value (required)
        stop: Box<TirExpr>,
        /// Step value (None means 1)
        step: Option<Box<TirExpr>>,
    },

    /// Field access: obj.field
    FieldAccess {
        object: Box<TirExpr>,
        class: ClassId,
        field: FieldId,
    },

    /// List literal: [a, b, c]
    List {
        elements: Vec<TirExpr>,
        elem_ty: TirType,
    },

    /// Bytes literal: b"hello"
    Bytes { data: Vec<u8> },
}
