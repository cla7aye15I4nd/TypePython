//! TIR unresolved expression representation
//!
//! This module defines expressions used during the lowering and type inference phase.
//! Unlike the final TirExpr, these expressions can contain TirTypeUnresolved with TypeVar
//! variants representing types that haven't been fully inferred yet.
//!
//! After constraint solving, these expressions are converted to fully resolved TirExpr
//! via the resolve module.

use crate::ast::{BinOperator, BoolOp, CompareOp, Constant, UnaryOp};

use super::expr::VarRef; // VarRef is shared between resolved and unresolved TIR
use super::ids::{ClassId, FieldId, FuncId};
use super::types_unresolved::TirTypeUnresolved;

/// Typed expression with embedded type information (unresolved version).
/// The type may contain TypeVar variants that will be resolved during constraint solving.
#[derive(Debug, Clone)]
pub struct TirExprUnresolved {
    /// The expression kind
    pub kind: TirExprKindUnresolved,
    /// The type of this expression (may contain TypeVar)
    pub ty: TirTypeUnresolved,
}

impl TirExprUnresolved {
    /// Create a new TirExprUnresolved with the given kind and type
    pub fn new(kind: TirExprKindUnresolved, ty: TirTypeUnresolved) -> Self {
        TirExprUnresolved { kind, ty }
    }
}

/// Expression variants (unresolved version)
#[derive(Debug, Clone)]
pub enum TirExprKindUnresolved {
    /// Constant value (int, str, None)
    Constant(Constant),

    /// Variable reference (local, param, global, self)
    Var(VarRef),

    /// Binary operation: left op right
    BinOp {
        left: Box<TirExprUnresolved>,
        op: BinOperator,
        right: Box<TirExprUnresolved>,
    },

    /// Comparison: left op right
    /// Chained comparisons are desugared during lowering
    Compare {
        left: Box<TirExprUnresolved>,
        op: CompareOp,
        right: Box<TirExprUnresolved>,
    },

    /// Boolean operation: a and b, a or b
    BoolOp {
        op: BoolOp,
        values: Vec<TirExprUnresolved>,
    },

    /// Unary operation: not x, -x
    UnaryOp {
        op: UnaryOp,
        operand: Box<TirExprUnresolved>,
    },

    /// Function call (resolved to FuncId)
    /// For method calls, args[0] is the receiver (self)
    Call {
        func: FuncId,
        args: Vec<TirExprUnresolved>,
    },

    /// Class constructor: ClassName(args)
    Construct {
        class: ClassId,
        args: Vec<TirExprUnresolved>,
    },

    /// Field access: obj.field
    FieldAccess {
        object: Box<TirExprUnresolved>,
        class: ClassId,
        field: FieldId,
    },

    /// List literal: [a, b, c]
    /// The elem_ty may contain TypeVar for empty lists
    List {
        elements: Vec<TirExprUnresolved>,
        elem_ty: TirTypeUnresolved,
    },

    /// Bytes literal: b"hello"
    Bytes { data: Vec<u8> },
}
