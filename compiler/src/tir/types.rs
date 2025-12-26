//! TIR type representation (resolved)
//!
//! Types in TIR are fully resolved - class types use ClassId instead of strings.
//! List types are unified with class types - each list[T] gets its own ClassId.
//!
//! IMPORTANT: This enum does NOT contain TypeVar. TypeVar only exists in TirTypeUnresolved.
//! This design provides compile-time guarantees that unresolved types cannot reach codegen.
//! See types_unresolved.rs for the unresolved variant used during type inference.

use super::ids::ClassId;
use std::hash::{Hash, Hasher};

/// Fully resolved type in TIR.
/// Unlike AST types, class references use ClassId instead of string names.
/// List types are represented as Class with is_builtin=true and element_type set.
///
/// CRITICAL: This enum has no TypeVar variant. TypeVar only exists in TirTypeUnresolved.
/// This architectural design ensures that unresolved types cannot reach codegen.
/// The resolve module validates that all TypeVars are resolved before conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TirType {
    /// Integer type (i64)
    Int,

    /// Float type (f64)
    Float,

    /// Boolean type (i1 in LLVM)
    Bool,

    /// Void type (for functions with no return value)
    Void,

    /// Class instance type - resolved to ClassId
    /// This includes both user-defined classes and built-in types (str, list, bytes, bytearray).
    /// For list types, the ClassId corresponds to a TirClass with is_builtin=true.
    Class(ClassId),
}

impl Hash for TirType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            TirType::Int => {}
            TirType::Float => {}
            TirType::Bool => {}
            TirType::Void => {}
            TirType::Class(id) => id.hash(state),
        }
    }
}
