//! Built-in class definitions for GlobalSymbols
//!
//! This module contains the implementation of built-in Python types
//! (list, bytearray, bytes, str) as separate files for better organization.

/// Register methods on a builtin class with auto-incrementing MethodId.
/// Supports both shared and unique methods for generic types.
///
/// Usage for non-generic types (all methods shared):
/// ```ignore
/// register_methods!(self, class_id, "bytearray",
///     "append" => (vec![TirType::Int], TirType::Void),
///     "__len__" => (vec![], TirType::Int),
/// );
/// ```
///
/// Usage for generic types (mix of unique and shared methods):
/// ```ignore
/// register_methods!(self, class_id, "list",
///     unique "append" => (vec![element_type.clone()], TirType::Void),
///     shared "__len__" => (vec![], TirType::Int),
/// );
/// ```
macro_rules! register_methods {
    // Entry point - start counting from 0
    ($self:expr, $class_id:expr, $class_name:expr, $($rest:tt)*) => {
        register_methods!(@impl $self, $class_id, $class_name, 0, $($rest)*)
    };

    // Register a UNIQUE method (for generic types with type-dependent signatures)
    (@impl $self:expr, $class_id:expr, $class_name:expr, $idx:expr,
        unique $method_name:expr => ($params:expr, $ret_type:expr) $(, $($rest:tt)*)?) => {
        {
            let runtime_name = format!("__pyc___builtin___{}_{}", $class_name, $method_name);
            let func_id = $self.alloc_func($params, $ret_type);
            // Unique cache key per class_id to avoid signature collisions
            $self.builtin_runtime_funcs
                .insert(format!("{}_{}", runtime_name, $class_id.0), func_id);
            // Map to shared C runtime function name
            $self.runtime_func_names
                .insert(func_id, runtime_name);
            $self.methods.insert(
                ($class_id, $method_name.to_string()),
                ($crate::tir::ids::MethodId($idx), func_id),
            );
            $self.class_data[$class_id.index()]
                .methods
                .push(($method_name.to_string(), func_id));
        }
        $(register_methods!(@impl $self, $class_id, $class_name, $idx + 1, $($rest)*);)?
    };

    // Register a SHARED method (reuses FuncId across all instances)
    (@impl $self:expr, $class_id:expr, $class_name:expr, $idx:expr,
        shared $method_name:expr => ($params:expr, $ret_type:expr) $(, $($rest:tt)*)?) => {
        {
            let runtime_name = format!("__pyc___builtin___{}_{}", $class_name, $method_name);
            let func_id = $self.get_or_create_runtime_func(&runtime_name, $params, $ret_type);
            $self.methods.insert(
                ($class_id, $method_name.to_string()),
                ($crate::tir::ids::MethodId($idx), func_id),
            );
            $self.class_data[$class_id.index()]
                .methods
                .push(($method_name.to_string(), func_id));
        }
        $(register_methods!(@impl $self, $class_id, $class_name, $idx + 1, $($rest)*);)?
    };

    // Default: no marker = shared (backwards compatible)
    (@impl $self:expr, $class_id:expr, $class_name:expr, $idx:expr,
        $method_name:expr => ($params:expr, $ret_type:expr) $(, $($rest:tt)*)?) => {
        {
            let runtime_name = format!("__pyc___builtin___{}_{}", $class_name, $method_name);
            let func_id = $self.get_or_create_runtime_func(&runtime_name, $params, $ret_type);
            $self.methods.insert(
                ($class_id, $method_name.to_string()),
                ($crate::tir::ids::MethodId($idx), func_id),
            );
            $self.class_data[$class_id.index()]
                .methods
                .push(($method_name.to_string(), func_id));
        }
        $(register_methods!(@impl $self, $class_id, $class_name, $idx + 1, $($rest)*);)?
    };

    // Terminal case - no more methods
    (@impl $self:expr, $class_id:expr, $class_name:expr, $idx:expr,) => {};
}

/// Initialize a builtin class with its qualified name.
/// Returns the ClassId if already exists, otherwise allocates a new one.
///
/// # Arguments
/// * `$self` - &mut GlobalSymbols
/// * `$key` - ClassKey for the class
/// * `$class_name` - Class name (e.g., "str") - qualified name is derived
macro_rules! init_builtin_class {
    ($self:expr, $key:expr, $class_name:expr) => {{
        if let Some(&class_id) = $self.classes.get(&$key) {
            return class_id;
        }
        let class_id = $self.alloc_class();
        $self.classes.insert($key, class_id);
        $self.class_data[class_id.index()].qualified_name = format!("__builtin__.{}", $class_name);
        class_id
    }};
}

mod bytearray;
mod bytes;
mod exception;
mod list;
mod list_iterator;
mod range;
mod str_class;

// Re-export nothing - all methods are impl blocks on GlobalSymbols
