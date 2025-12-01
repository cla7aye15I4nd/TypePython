//! Type Registry - Generates type-specialized functions for complex nested types
//!
//! This module provides infrastructure for generating functions specific to
//! complex nested types like List[List[int]], Dict[str, List[int]], etc.
//!
//! The key insight is that for nested container types, we need to:
//! 1. Track which complex types are used in the program
//! 2. Generate specialized append/getitem/setitem functions that properly
//!    handle the type conversions (pointer <-> i64)
//! 3. Maintain type information through all operations

use super::PyType;
use std::collections::HashSet;

/// Registry for tracking and generating type-specialized functions
#[derive(Default)]
pub struct TypeRegistry {
    /// Set of complex types that need specialized function generation
    /// Stored as mangled type names (e.g., "list_list_int")
    registered_types: HashSet<String>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate a mangled name for a PyType
    /// Examples:
    /// - Int -> "int"
    /// - List(Int) -> "list_int"
    /// - List(List(Int)) -> "list_list_int"
    /// - Dict(Str, List(Int)) -> "dict_str_list_int"
    pub fn mangle_type(ty: &PyType) -> String {
        match ty {
            PyType::Int => "int".to_string(),
            PyType::Float => "float".to_string(),
            PyType::Bool => "bool".to_string(),
            PyType::Str => "str".to_string(),
            PyType::Bytes => "bytes".to_string(),
            PyType::None => "none".to_string(),
            PyType::List(elem) => format!("list_{}", Self::mangle_type(elem)),
            PyType::Dict(k, v) => {
                format!("dict_{}_{}", Self::mangle_type(k), Self::mangle_type(v))
            }
            PyType::Set(elem) => format!("set_{}", Self::mangle_type(elem)),
            PyType::Tuple(elems) => {
                let elem_names: Vec<String> = elems.iter().map(Self::mangle_type).collect();
                format!("tuple_{}", elem_names.join("_"))
            }
            PyType::Instance(inst) => inst.class_name.replace("__builtin_", ""),
            PyType::Function(_) => "fn".to_string(),
            PyType::Module => "module".to_string(),
        }
    }

    /// Check if a type is a "complex" type that needs specialized function generation
    /// A complex type is one where the element type is itself a container
    pub fn is_complex_type(ty: &PyType) -> bool {
        match ty {
            PyType::List(elem) => matches!(
                elem.as_ref(),
                PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) | PyType::Tuple(_)
            ),
            PyType::Dict(_, v) => matches!(
                v.as_ref(),
                PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) | PyType::Tuple(_)
            ),
            PyType::Set(elem) => matches!(
                elem.as_ref(),
                PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) | PyType::Tuple(_)
            ),
            _ => false,
        }
    }

    /// Get the depth of nesting for a type
    /// Examples:
    /// - Int -> 0
    /// - List(Int) -> 1
    /// - List(List(Int)) -> 2
    pub fn nesting_depth(ty: &PyType) -> usize {
        match ty {
            PyType::List(elem) => 1 + Self::nesting_depth(elem),
            PyType::Dict(_, v) => 1 + Self::nesting_depth(v),
            PyType::Set(elem) => 1 + Self::nesting_depth(elem),
            PyType::Tuple(elems) => 1 + elems.iter().map(Self::nesting_depth).max().unwrap_or(0),
            _ => 0,
        }
    }

    /// Register a type for specialized function generation
    /// Returns true if the type was newly registered
    pub fn register_type(&mut self, ty: &PyType) -> bool {
        if Self::is_complex_type(ty) {
            let mangled = Self::mangle_type(ty);
            self.registered_types.insert(mangled)
        } else {
            false
        }
    }

    /// Check if a type has been registered
    pub fn is_registered(&self, ty: &PyType) -> bool {
        let mangled = Self::mangle_type(ty);
        self.registered_types.contains(&mangled)
    }

    /// Get the function name for list operations based on element type
    /// This handles both simple types (uses existing C functions) and
    /// complex nested types (uses generic pointer-based functions)
    pub fn get_list_append_fn(elem_type: &PyType) -> &'static str {
        match elem_type {
            PyType::Str => "str_list_append",
            PyType::Float => "float_list_append",
            PyType::Bool => "bool_list_append",
            // For nested containers and other types, use generic int64 storage
            _ => "list_append",
        }
    }

    /// Get the function name for list creation with capacity
    pub fn get_list_with_capacity_fn(elem_type: &PyType) -> &'static str {
        match elem_type {
            PyType::Str => "str_list_with_capacity",
            PyType::Float => "float_list_with_capacity",
            PyType::Bool => "bool_list_with_capacity",
            // For nested containers and other types, use generic int64 storage
            _ => "list_with_capacity",
        }
    }

    /// Get the function name for list getitem based on element type
    pub fn get_list_getitem_fn(elem_type: &PyType) -> &'static str {
        match elem_type {
            PyType::Bool => "bool_list_getitem",
            PyType::Float => "float_list_getitem",
            PyType::Str => "str_list_getitem",
            // For nested containers and other types, use generic int64 storage
            _ => "list_getitem",
        }
    }

    /// Check if the element type requires pointer-to-int conversion for storage
    pub fn needs_ptr_to_int(elem_type: &PyType) -> bool {
        matches!(
            elem_type,
            PyType::List(_)
                | PyType::Dict(_, _)
                | PyType::Set(_)
                | PyType::Tuple(_)
                | PyType::Bytes
                | PyType::Instance(_)
        )
    }

    /// Check if the element type requires int-to-ptr conversion when reading
    pub fn needs_int_to_ptr(elem_type: &PyType) -> bool {
        Self::needs_ptr_to_int(elem_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mangle_type() {
        assert_eq!(TypeRegistry::mangle_type(&PyType::Int), "int");
        assert_eq!(
            TypeRegistry::mangle_type(&PyType::List(Box::new(PyType::Int))),
            "list_int"
        );
        assert_eq!(
            TypeRegistry::mangle_type(&PyType::List(Box::new(PyType::List(Box::new(PyType::Int))))),
            "list_list_int"
        );
    }

    #[test]
    fn test_is_complex_type() {
        assert!(!TypeRegistry::is_complex_type(&PyType::Int));
        assert!(!TypeRegistry::is_complex_type(&PyType::List(Box::new(
            PyType::Int
        ))));
        assert!(TypeRegistry::is_complex_type(&PyType::List(Box::new(
            PyType::List(Box::new(PyType::Int))
        ))));
    }

    #[test]
    fn test_nesting_depth() {
        assert_eq!(TypeRegistry::nesting_depth(&PyType::Int), 0);
        assert_eq!(
            TypeRegistry::nesting_depth(&PyType::List(Box::new(PyType::Int))),
            1
        );
        assert_eq!(
            TypeRegistry::nesting_depth(&PyType::List(Box::new(PyType::List(Box::new(
                PyType::Int
            ))))),
            2
        );
    }
}
