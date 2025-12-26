//! TIR unresolved type representation
//!
//! This module defines types used during the lowering and type inference phase.
//! Unlike the final TirType, TirTypeUnresolved can contain TypeVar variants
//! representing types that haven't been fully inferred yet.
//!
//! After constraint solving, these types are converted to fully resolved TirType
//! via the resolve module, providing compile-time guarantees that TypeVars never
//! reach codegen.

use super::ids::ClassId;
use std::collections::HashMap;

/// Unresolved type in TIR - used during lowering and type inference.
/// This type can contain TypeVar variants representing unresolved types.
/// After constraint solving, these are converted to fully resolved TirType.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TirTypeUnresolved {
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
    /// For generic containers, the ClassId corresponds to a class whose type_params may contain TypeVars.
    Class(ClassId),

    /// Type variable - unresolved type during type inference
    /// The u32 is a unique type variable ID
    /// This must be resolved via constraint solving before conversion to TirType
    ///
    /// For generic containers (list[?T], dict[?K, ?V], set[?T]), we use Class(class_id)
    /// where ClassData.type_params contains TypeVar.
    TypeVar(u32),
}

impl TirTypeUnresolved {
    /// Check if this is a numeric type (for arithmetic operations)
    pub fn is_numeric(&self) -> bool {
        matches!(self, TirTypeUnresolved::Int | TirTypeUnresolved::Float)
    }

    /// Check if this is a boolean type
    pub fn is_boolean(&self) -> bool {
        matches!(self, TirTypeUnresolved::Bool)
    }

    /// Get the class ID if this is a class type, None otherwise
    pub fn class_id(&self) -> Option<ClassId> {
        match self {
            TirTypeUnresolved::Class(id) => Some(*id),
            _ => None,
        }
    }

    /// Check if this type is compatible with another type for assignment/argument passing
    ///
    /// Note: For generic containers, we use Class(id) where ClassData.type_params
    /// may contain TypeVar. Type checking will be handled during constraint solving.
    pub fn is_compatible_with(&self, other: &TirTypeUnresolved) -> bool {
        match (self, other) {
            (TirTypeUnresolved::Int, TirTypeUnresolved::Int) => true,
            (TirTypeUnresolved::Float, TirTypeUnresolved::Float) => true,
            // Allow int to be compatible with float (implicit promotion)
            (TirTypeUnresolved::Int, TirTypeUnresolved::Float) => true,
            (TirTypeUnresolved::Float, TirTypeUnresolved::Int) => true,
            (TirTypeUnresolved::Bool, TirTypeUnresolved::Bool) => true,
            (TirTypeUnresolved::Class(a), TirTypeUnresolved::Class(b)) => a == b,
            // Type variables are compatible with anything (will be unified during constraint solving)
            (TirTypeUnresolved::TypeVar(_), _) | (_, TirTypeUnresolved::TypeVar(_)) => true,
            _ => false,
        }
    }

    /// Apply type variable substitutions recursively
    /// This resolves TypeVars to their inferred concrete types after constraint solving
    ///
    /// Note: For containers, ClassData.type_params are substituted separately
    /// by the constraint solver.
    pub fn substitute(&self, substitutions: &HashMap<u32, TirTypeUnresolved>) -> TirTypeUnresolved {
        match self {
            TirTypeUnresolved::TypeVar(id) => {
                // Follow transitive substitutions: if T1 -> T2 and T2 -> Int, resolve to Int
                substitutions
                    .get(id)
                    .map(|t| t.substitute(substitutions))
                    .unwrap_or_else(|| self.clone())
            }
            _ => self.clone(),
        }
    }

    /// Convert from TirType to TirTypeUnresolved
    /// This is a simple conversion since TirType is a subset of TirTypeUnresolved
    /// (TirType has no TypeVar variant, TirTypeUnresolved does)
    pub fn from_tir_type(ty: &super::types::TirType) -> Self {
        match ty {
            super::types::TirType::Int => TirTypeUnresolved::Int,
            super::types::TirType::Float => TirTypeUnresolved::Float,
            super::types::TirType::Bool => TirTypeUnresolved::Bool,
            super::types::TirType::Void => TirTypeUnresolved::Void,
            super::types::TirType::Class(id) => TirTypeUnresolved::Class(*id),
        }
    }

    /// Convert to TirType
    /// PANICS if this is a TypeVar - TypeVar cannot be converted to resolved TirType.
    /// Use resolve::resolve_type() instead which properly validates and returns Result.
    pub fn to_tir_type(&self) -> super::types::TirType {
        match self {
            TirTypeUnresolved::Int => super::types::TirType::Int,
            TirTypeUnresolved::Float => super::types::TirType::Float,
            TirTypeUnresolved::Bool => super::types::TirType::Bool,
            TirTypeUnresolved::Void => super::types::TirType::Void,
            TirTypeUnresolved::Class(id) => super::types::TirType::Class(*id),
            TirTypeUnresolved::TypeVar(id) => {
                panic!(
                    "Cannot convert TypeVar({}) to TirType - use resolve::resolve_type() instead",
                    id
                )
            }
        }
    }

    /// Check if this type contains a specific type variable (for occurs-check)
    /// The occurs-check prevents infinite types like T = list[T]
    ///
    /// Note: For containers, ClassData.type_params should also be checked.
    pub fn contains_type_var(&self, var_id: u32) -> bool {
        match self {
            TirTypeUnresolved::TypeVar(id) => *id == var_id,
            _ => false,
        }
    }
}
