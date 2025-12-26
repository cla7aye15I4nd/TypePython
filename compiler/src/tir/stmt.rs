//! TIR statement representation
//!
//! Statements in TIR use resolved references and embedded types.

use crate::ast::BinOperator;

use super::expr::{TirExpr, VarRef};
use super::ids::{ClassId, FieldId, LocalId};
use super::types::TirType;

// Note: TirType is still used in TirStmt::Let

/// An except handler in a try statement
#[derive(Debug, Clone)]
pub struct TirExceptHandler {
    /// Exception class to catch (None = catch all)
    pub exc_class: Option<ClassId>,
    /// Local variable to bind the exception to
    pub local: Option<LocalId>,
    /// Handler body
    pub body: Vec<TirStmt>,
}

/// An lvalue - something that can be assigned to.
#[derive(Debug, Clone)]
pub enum TirLValue {
    /// Variable (local, param, global)
    Var(VarRef),

    /// Field access: obj.field
    Field {
        object: Box<TirExpr>,
        class: ClassId,
        field: FieldId,
    },
}

/// Typed statement
#[derive(Debug, Clone)]
pub enum TirStmt {
    /// Variable declaration with initialization.
    /// Created when a new local variable is introduced.
    Let {
        local: LocalId,
        ty: TirType,
        init: TirExpr,
    },

    /// Assignment to existing variable or lvalue.
    Assign { target: TirLValue, value: TirExpr },

    /// Augmented assignment: target op= value
    AugAssign {
        target: VarRef,
        op: BinOperator,
        value: TirExpr,
    },

    /// Expression statement (for side effects)
    Expr(TirExpr),

    /// Return statement
    Return(Option<TirExpr>),

    /// If statement
    If {
        cond: TirExpr,
        then_body: Vec<TirStmt>,
        else_body: Vec<TirStmt>,
    },

    /// While loop
    While { cond: TirExpr, body: Vec<TirStmt> },

    /// Try/except/finally statement
    Try {
        body: Vec<TirStmt>,
        handlers: Vec<TirExceptHandler>,
        orelse: Vec<TirStmt>,    // else clause (runs if no exception)
        finalbody: Vec<TirStmt>, // finally clause (always runs)
    },

    /// Raise an exception
    Raise {
        exc: Option<TirExpr>, // None for bare 'raise' (re-raise)
    },
}
