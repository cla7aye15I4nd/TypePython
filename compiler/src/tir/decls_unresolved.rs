//! TIR unresolved function declarations
//!
//! This module defines function declarations used during the lowering and type inference phase.
//! Unlike the final TirFunction, these can contain TirTypeUnresolved with TypeVar variants.
//!
//! After constraint solving, these are converted to fully resolved TirFunction via the resolve module.
//!
//! Note: TirClass is the same in both resolved and unresolved TIR, as its type_params are
//! stored separately in GlobalSymbols.class_data during lowering.

use super::ids::{ClassId, FuncId};
use super::stmt_unresolved::TirStmtUnresolved;
use super::types_unresolved::TirTypeUnresolved;

/// A typed function definition (unresolved version)
/// Types may contain TypeVar that will be resolved during constraint solving.
#[derive(Debug, Clone)]
pub struct TirFunctionUnresolved {
    /// Function ID for self-reference
    pub id: FuncId,

    /// Original name (for debugging/LLVM naming)
    pub name: String,

    /// Qualified name (module.function) for LLVM symbol
    pub qualified_name: String,

    /// Parameters: (name, type)
    /// Types may contain TypeVar
    pub params: Vec<(String, TirTypeUnresolved)>,

    /// Return type (may contain TypeVar)
    pub return_type: TirTypeUnresolved,

    /// Local variables (not including parameters)
    /// Indexed by LocalId
    /// Types may contain TypeVar
    pub locals: Vec<(String, TirTypeUnresolved)>,

    /// Function body (may contain unresolved types)
    pub body: Vec<TirStmtUnresolved>,

    /// If this is a method, which class it belongs to
    pub class: Option<ClassId>,

    /// If Some, this function is an external runtime function (no body to codegen)
    /// The value is the runtime function name to call (e.g., "list_len", "bytearray_append")
    pub runtime_name: Option<String>,
}
