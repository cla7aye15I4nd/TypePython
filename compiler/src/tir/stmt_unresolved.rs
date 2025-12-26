//! TIR unresolved statement representation
//!
//! This module defines statements used during the lowering and type inference phase.
//! Unlike the final TirStmt, these statements can contain TirTypeUnresolved with TypeVar
//! variants representing types that haven't been fully inferred yet.
//!
//! After constraint solving, these statements are converted to fully resolved TirStmt
//! via the resolve module.

use crate::ast::BinOperator;

use super::expr::VarRef; // VarRef is shared between resolved and unresolved TIR
use super::expr_unresolved::TirExprUnresolved;
use super::ids::{ClassId, FieldId, LocalId};
use super::types_unresolved::TirTypeUnresolved;

/// An except handler in a try statement (unresolved version)
#[derive(Debug, Clone)]
pub struct TirExceptHandlerUnresolved {
    /// Exception class to catch (None = catch all)
    pub exc_class: Option<ClassId>,
    /// Local variable to bind the exception to
    pub local: Option<LocalId>,
    /// Handler body (may contain unresolved types)
    pub body: Vec<TirStmtUnresolved>,
}

/// An lvalue - something that can be assigned to (unresolved version)
#[derive(Debug, Clone)]
pub enum TirLValueUnresolved {
    /// Variable (local, param, global)
    Var(VarRef),

    /// Field access: obj.field
    Field {
        object: Box<TirExprUnresolved>,
        class: ClassId,
        field: FieldId,
    },
}

/// Typed statement (unresolved version)
/// Types in these statements may contain TypeVar that will be resolved during constraint solving.
#[derive(Debug, Clone)]
pub enum TirStmtUnresolved {
    /// Variable declaration with initialization.
    /// Created when a new local variable is introduced.
    Let {
        local: LocalId,
        ty: TirTypeUnresolved,
        init: TirExprUnresolved,
    },

    /// Assignment to existing variable or lvalue.
    Assign {
        target: TirLValueUnresolved,
        value: TirExprUnresolved,
    },

    /// Augmented assignment: target op= value
    AugAssign {
        target: VarRef,
        op: BinOperator,
        value: TirExprUnresolved,
    },

    /// Expression statement (for side effects)
    Expr(TirExprUnresolved),

    /// Return statement
    Return(Option<TirExprUnresolved>),

    /// If statement
    If {
        cond: TirExprUnresolved,
        then_body: Vec<TirStmtUnresolved>,
        else_body: Vec<TirStmtUnresolved>,
    },

    /// While loop
    While {
        cond: TirExprUnresolved,
        body: Vec<TirStmtUnresolved>,
    },

    /// Try/except/finally statement
    Try {
        body: Vec<TirStmtUnresolved>,
        handlers: Vec<TirExceptHandlerUnresolved>,
        orelse: Vec<TirStmtUnresolved>, // else clause (runs if no exception)
        finalbody: Vec<TirStmtUnresolved>, // finally clause (always runs)
    },

    /// Raise an exception
    Raise {
        exc: Option<TirExprUnresolved>, // None for bare 'raise' (re-raise)
    },
}
