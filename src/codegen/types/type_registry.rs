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

/// Registry for tracking and generating type-specialized functions
#[derive(Default)]
pub struct TypeRegistry {}

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
