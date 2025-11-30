//! PyValue - Core Python value type definitions
//!
//! PyValue is a flat enum where each variant encodes the Python type
//! and holds the specific LLVM value type directly (no BasicValueEnum).

use crate::ast::{BinaryOp, Type, UnaryOp};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue};
use std::collections::HashMap;

// ============================================================================
// Storage types - Union types for values (Immediate | Pointer)
// ============================================================================

/// Storage for integer values - either an immediate value or a pointer to one
#[derive(Clone, Copy)]
pub enum IntStorage<'ctx> {
    Immediate(IntValue<'ctx>),
    Pointer(PointerValue<'ctx>),
}

/// Storage for float values - either an immediate value or a pointer to one
#[derive(Clone, Copy)]
pub enum FloatStorage<'ctx> {
    Immediate(FloatValue<'ctx>),
    Pointer(PointerValue<'ctx>),
}

/// Storage for bool values - either an immediate value or a pointer to one
#[derive(Clone, Copy)]
pub enum BoolStorage<'ctx> {
    Immediate(IntValue<'ctx>), // i1
    Pointer(PointerValue<'ctx>),
}

/// Storage for None values - either an immediate value or a pointer to one
#[derive(Clone, Copy)]
pub enum NoneStorage<'ctx> {
    Immediate(IntValue<'ctx>), // i32
    Pointer(PointerValue<'ctx>),
}

/// Storage for pointer-based types (Str, Bytes) - either direct or via alloca
#[derive(Clone, Copy)]
pub enum PtrStorage<'ctx> {
    /// Direct pointer to the value
    Direct(PointerValue<'ctx>),
    /// Pointer to alloca containing the pointer to the value
    Alloca(PointerValue<'ctx>),
}

// ============================================================================
// Instance Fields - Named fields with types (like a named tuple/struct)
// ============================================================================

/// A named field with its type - used for Instance types
pub type InstanceFields = Vec<(String, PyType)>;

/// Instance type info - class name and fields bundled together
#[derive(Clone, Debug, PartialEq)]
pub struct InstanceType {
    pub class_name: String,
    pub fields: InstanceFields,
}

impl InstanceType {
    pub fn new(class_name: String, fields: InstanceFields) -> Self {
        Self { class_name, fields }
    }
}

/// Function type info - parameter types and return types
/// return_types is a Vec to handle multiple returns or no returns
/// - Empty Vec means void/None return
/// - Single element means single return type
/// - Multiple elements could be used for tuple returns in future
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType {
    pub param_types: Vec<PyType>,
    pub return_types: Vec<PyType>,
}

impl FunctionType {
    pub fn new(param_types: Vec<PyType>, return_type: PyType) -> Self {
        let return_types = match return_type {
            PyType::None => vec![],
            other => vec![other],
        };
        Self {
            param_types,
            return_types,
        }
    }

    /// Get the single return type (or None if void/empty)
    pub fn return_type(&self) -> PyType {
        self.return_types.first().cloned().unwrap_or(PyType::None)
    }
}

/// Builtin class names - used with Instance variant
pub mod iter_names {
    // Iterators
    pub const RANGE_ITERATOR: &str = "__builtin_range_iterator";
    pub const LIST_ITERATOR: &str = "__builtin_list_iterator";
    pub const STR_ITERATOR: &str = "__builtin_str_iterator";
    pub const BYTES_ITERATOR: &str = "__builtin_bytes_iterator";
    pub const ENUMERATE: &str = "__builtin_enumerate";
    pub const ZIP: &str = "__builtin_zip";
    pub const FILTER: &str = "__builtin_filter";
    pub const DICT_KEYS: &str = "__builtin_dict_keys";
    pub const DICT_VALUES: &str = "__builtin_dict_values";
    pub const DICT_ITEMS: &str = "__builtin_dict_items";
    pub const GENERATOR: &str = "__builtin_generator";
    // Other builtin types represented as Instance
    pub const RANGE: &str = "__builtin_range";
    pub const EXCEPTION: &str = "__builtin_exception";
}

/// Check if a class name is a builtin iterator (includes generators)
pub fn is_builtin_iterator(class_name: &str) -> bool {
    class_name.starts_with("__builtin_")
        && (class_name.contains("iterator")
            || matches!(
                class_name,
                iter_names::ENUMERATE
                    | iter_names::ZIP
                    | iter_names::FILTER
                    | iter_names::DICT_KEYS
                    | iter_names::DICT_VALUES
                    | iter_names::DICT_ITEMS
                    | iter_names::GENERATOR
            ))
}

/// Helper to get a field type from instance fields by name
pub fn get_field_type<'a>(fields: &'a InstanceFields, name: &str) -> Option<&'a PyType> {
    fields.iter().find(|(n, _)| n == name).map(|(_, t)| t)
}

/// Check if a PyType is a Range (Instance with RANGE class name)
pub fn is_range(ty: &PyType) -> bool {
    matches!(ty, PyType::Instance(inst) if inst.class_name == iter_names::RANGE)
}

/// Check if a PyType is an Exception (Instance with EXCEPTION class name)
pub fn is_exception(ty: &PyType) -> bool {
    matches!(ty, PyType::Instance(inst) if inst.class_name == iter_names::EXCEPTION)
}

// ============================================================================
// PyType - Simple type tag
// ============================================================================

/// Type tag for representing Python types
#[derive(Clone, Debug, PartialEq)]
pub enum PyType {
    Int,
    Float,
    Bool,
    Str,
    Bytes,
    None,
    List(Box<PyType>),
    Dict(Box<PyType>, Box<PyType>),
    Set(Box<PyType>),
    Tuple(Vec<PyType>), // Heterogeneous tuple with element types
    /// Instance of a class with named fields (like a named tuple/struct)
    /// Fields store type information for attributes
    /// For iterators/generators: fields like ("yield_type", PyType::Int)
    /// For user classes: fields like ("x", PyType::Int), ("name", PyType::Str)
    /// For Range: Instance with class_name = "__builtin_range"
    /// For Exception: Instance with class_name = "__builtin_exception"
    Instance(InstanceType),
    /// Function type with parameter and return types
    Function(FunctionType),
    Module,
}

/// Source type for enumerate iterator - determines which C functions to use
#[derive(Debug, Clone, PartialEq)]
pub enum EnumerateSource {
    List,  // enumerate(list) - uses enumerate_list/enumerate_list_next
    Str,   // enumerate(str) - uses enumerate_str/enumerate_str_next
    Bytes, // enumerate(bytes) - uses enumerate_bytes/enumerate_bytes_next
}

impl PyType {
    /// Create from AST Type
    pub fn from_ast_type(ty: &Type) -> Result<Self, String> {
        match ty {
            Type::Int => Ok(PyType::Int),
            Type::Float => Ok(PyType::Float),
            Type::Bool => Ok(PyType::Bool),
            Type::Bytes => Ok(PyType::Bytes),
            Type::Str => Ok(PyType::Str),
            Type::None => Ok(PyType::None),
            Type::Range => Ok(PyType::Instance(InstanceType::new(
                iter_names::RANGE.to_string(),
                vec![],
            ))),
            Type::List(elem) => Ok(PyType::List(Box::new(PyType::from_ast_type(elem)?))),
            Type::Dict(k, v) => Ok(PyType::Dict(
                Box::new(PyType::from_ast_type(k)?),
                Box::new(PyType::from_ast_type(v)?),
            )),
            Type::Set(elem) => Ok(PyType::Set(Box::new(PyType::from_ast_type(elem)?))),
            Type::Tuple(elems) => {
                let elem_types: Result<Vec<PyType>, String> =
                    elems.iter().map(PyType::from_ast_type).collect();
                Ok(PyType::Tuple(elem_types?))
            }
            Type::Custom(name) => Ok(PyType::Instance(InstanceType::new(name.clone(), vec![]))),
        }
    }

    /// Convert to LLVM type
    pub fn to_llvm<'ctx>(&self, ctx: &'ctx Context) -> inkwell::types::BasicTypeEnum<'ctx> {
        match self {
            PyType::Int => ctx.i64_type().into(),
            PyType::Float => ctx.f64_type().into(),
            PyType::Bool => ctx.bool_type().into(),
            PyType::Str | PyType::Bytes => ctx.ptr_type(inkwell::AddressSpace::default()).into(),
            PyType::None => ctx.i32_type().into(),
            PyType::List(_)
            | PyType::Dict(_, _)
            | PyType::Set(_)
            | PyType::Tuple(_)
            | PyType::Instance(_) => ctx.ptr_type(inkwell::AddressSpace::default()).into(),
            PyType::Function(_) | PyType::Module => ctx.i64_type().into(),
        }
    }

    /// Lookup a member (method) for this type - returns (symbol, return_type)
    pub fn lookup_member(&self, name: &str) -> Option<(&'static str, PyType)> {
        match self {
            PyType::Bytes => match name {
                // Case conversion - return bytes
                "upper" => Some(("bytes_upper", PyType::Bytes)),
                "lower" => Some(("bytes_lower", PyType::Bytes)),
                "capitalize" => Some(("bytes_capitalize", PyType::Bytes)),
                "title" => Some(("bytes_title", PyType::Bytes)),
                "swapcase" => Some(("bytes_swapcase", PyType::Bytes)),
                // Padding/alignment - return bytes
                "ljust" => Some(("bytes_ljust", PyType::Bytes)),
                "rjust" => Some(("bytes_rjust", PyType::Bytes)),
                "center" => Some(("bytes_center", PyType::Bytes)),
                "zfill" => Some(("bytes_zfill", PyType::Bytes)),
                // Stripping - return bytes
                "strip" => Some(("bytes_strip", PyType::Bytes)),
                "lstrip" => Some(("bytes_lstrip", PyType::Bytes)),
                "rstrip" => Some(("bytes_rstrip", PyType::Bytes)),
                // Search - return int
                "find" => Some(("bytes_find", PyType::Int)),
                "count" => Some(("bytes_count", PyType::Int)),
                // Predicates - return bool
                "startswith" => Some(("bytes_startswith", PyType::Bool)),
                "endswith" => Some(("bytes_endswith", PyType::Bool)),
                "isalnum" => Some(("bytes_isalnum", PyType::Bool)),
                "isalpha" => Some(("bytes_isalpha", PyType::Bool)),
                "isdigit" => Some(("bytes_isdigit", PyType::Bool)),
                "isspace" => Some(("bytes_isspace", PyType::Bool)),
                "islower" => Some(("bytes_islower", PyType::Bool)),
                "isupper" => Some(("bytes_isupper", PyType::Bool)),
                // Transform - return bytes
                "replace" => Some(("bytes_replace", PyType::Bytes)),
                _ => None,
            },
            PyType::Str => match name {
                // Case conversion - return str
                "upper" => Some(("str_upper", PyType::Str)),
                "lower" => Some(("str_lower", PyType::Str)),
                "capitalize" => Some(("str_capitalize", PyType::Str)),
                "title" => Some(("str_title", PyType::Str)),
                "swapcase" => Some(("str_swapcase", PyType::Str)),
                // Padding/alignment - return str
                "ljust" => Some(("str_ljust", PyType::Str)),
                "rjust" => Some(("str_rjust", PyType::Str)),
                "center" => Some(("str_center", PyType::Str)),
                "zfill" => Some(("str_zfill", PyType::Str)),
                // Stripping - return str
                "strip" => Some(("str_strip", PyType::Str)),
                "lstrip" => Some(("str_lstrip", PyType::Str)),
                "rstrip" => Some(("str_rstrip", PyType::Str)),
                // Search - return int
                "find" => Some(("str_find", PyType::Int)),
                "count" => Some(("str_count", PyType::Int)),
                // Predicates - return bool
                "startswith" => Some(("str_startswith", PyType::Bool)),
                "endswith" => Some(("str_endswith", PyType::Bool)),
                "isalnum" => Some(("str_isalnum", PyType::Bool)),
                "isalpha" => Some(("str_isalpha", PyType::Bool)),
                "isdigit" => Some(("str_isdigit", PyType::Bool)),
                "isspace" => Some(("str_isspace", PyType::Bool)),
                "islower" => Some(("str_islower", PyType::Bool)),
                "isupper" => Some(("str_isupper", PyType::Bool)),
                // Transform - return str
                "replace" => Some(("str_replace", PyType::Str)),
                // Split - return list[str]
                "split" => Some(("str_split", PyType::List(Box::new(PyType::Str)))),
                // Join - return str (takes list[str] as argument)
                "join" => Some(("str_list_join", PyType::Str)),
                _ => None,
            },
            PyType::List(elem_type) => {
                // Select the appropriate function based on element type
                let type_prefix = match elem_type.as_ref() {
                    PyType::Str => "str_list",
                    PyType::Float => "float_list",
                    PyType::Bool => "bool_list",
                    _ => "list",
                };
                match name {
                    // Mutating methods that return None
                    "append" => Some((
                        match type_prefix {
                            "str_list" => "str_list_append",
                            "float_list" => "float_list_append",
                            "bool_list" => "bool_list_append",
                            _ => "list_append",
                        },
                        PyType::None,
                    )),
                    "insert" => Some(("list_insert", PyType::None)),
                    "extend" => Some(("list_extend", PyType::None)),
                    "remove" => Some(("list_remove", PyType::None)),
                    "clear" => Some((
                        match type_prefix {
                            "str_list" => "str_list_clear",
                            "float_list" => "float_list_clear",
                            "bool_list" => "bool_list_clear",
                            _ => "list_clear",
                        },
                        PyType::None,
                    )),
                    "reverse" => Some(("list_reverse", PyType::None)),
                    "sort" => Some(("list_sort", PyType::None)),
                    // Methods returning values
                    "pop" => Some((
                        match type_prefix {
                            "str_list" => "str_list_pop",
                            "float_list" => "float_list_pop",
                            "bool_list" => "bool_list_pop",
                            _ => "list_pop",
                        },
                        elem_type.as_ref().clone(),
                    )),
                    "index" => Some(("list_index", PyType::Int)),
                    "count" => Some(("list_count", PyType::Int)),
                    // Methods returning new list
                    "copy" => Some((
                        match type_prefix {
                            "str_list" => "str_list_copy",
                            "float_list" => "float_list_copy",
                            "bool_list" => "bool_list_copy",
                            _ => "list_copy",
                        },
                        PyType::List(elem_type.clone()),
                    )),
                    _ => None,
                }
            }
            PyType::Set(elem_type) => {
                // Select the appropriate function based on element type
                let type_prefix = match elem_type.as_ref() {
                    PyType::Str => "str_set",
                    PyType::Float => "float_set",
                    PyType::Bool => "bool_set",
                    _ => "set",
                };
                match name {
                    // Void methods (mutating in-place)
                    "add" => Some((
                        match type_prefix {
                            "str_set" => "str_set_add",
                            "float_set" => "float_set_add",
                            "bool_set" => "bool_set_add",
                            _ => "set_add",
                        },
                        PyType::None,
                    )),
                    "remove" => Some((
                        match type_prefix {
                            "str_set" => "str_set_remove",
                            "float_set" => "float_set_remove",
                            "bool_set" => "bool_set_remove",
                            _ => "set_remove",
                        },
                        PyType::None,
                    )),
                    "discard" => Some((
                        match type_prefix {
                            "str_set" => "str_set_discard",
                            "float_set" => "float_set_discard",
                            "bool_set" => "bool_set_discard",
                            _ => "set_discard",
                        },
                        PyType::None,
                    )),
                    "clear" => Some((
                        match type_prefix {
                            "str_set" => "str_set_clear",
                            "float_set" => "float_set_clear",
                            "bool_set" => "bool_set_clear",
                            _ => "set_clear",
                        },
                        PyType::None,
                    )),
                    "update" => Some(("set_update", PyType::None)),
                    "difference_update" => Some(("set_difference_update", PyType::None)),
                    "intersection_update" => Some(("set_intersection_update", PyType::None)),
                    "symmetric_difference_update" => {
                        Some(("set_symmetric_difference_update", PyType::None))
                    }
                    // Methods returning an element
                    "pop" => Some((
                        match type_prefix {
                            "str_set" => "str_set_pop",
                            "float_set" => "float_set_pop",
                            "bool_set" => "bool_set_pop",
                            _ => "set_pop",
                        },
                        elem_type.as_ref().clone(),
                    )),
                    // Methods returning new set
                    "copy" => Some((
                        match type_prefix {
                            "str_set" => "str_set_copy",
                            "float_set" => "float_set_copy",
                            "bool_set" => "bool_set_copy",
                            _ => "set_copy",
                        },
                        PyType::Set(elem_type.clone()),
                    )),
                    "union" => Some(("set_union", PyType::Set(elem_type.clone()))),
                    "intersection" => Some(("set_intersection", PyType::Set(elem_type.clone()))),
                    "difference" => Some(("set_difference", PyType::Set(elem_type.clone()))),
                    "symmetric_difference" => {
                        Some(("set_symmetric_difference", PyType::Set(elem_type.clone())))
                    }
                    // Methods returning bool
                    "issubset" => Some(("set_issubset", PyType::Bool)),
                    "issuperset" => Some(("set_issuperset", PyType::Bool)),
                    "isdisjoint" => Some(("set_isdisjoint", PyType::Bool)),
                    _ => None,
                }
            }
            PyType::Dict(key_type, val_type) => {
                // Select the appropriate function based on key type
                let is_str_keyed = matches!(key_type.as_ref(), PyType::Str);
                match name {
                    // Methods returning values - different C functions for str vs int keys
                    "get" => Some((
                        if is_str_keyed {
                            "str_dict_get"
                        } else {
                            "dict_get"
                        },
                        val_type.as_ref().clone(),
                    )),
                    "pop" => Some((
                        if is_str_keyed {
                            "str_dict_pop"
                        } else {
                            "dict_pop"
                        },
                        val_type.as_ref().clone(),
                    )),
                    "setdefault" => Some((
                        if is_str_keyed {
                            "str_dict_setdefault"
                        } else {
                            "dict_setdefault"
                        },
                        val_type.as_ref().clone(),
                    )),
                    // Void methods
                    "clear" => Some((
                        if is_str_keyed {
                            "str_dict_clear"
                        } else {
                            "dict_clear"
                        },
                        PyType::None,
                    )),
                    "update" => Some((
                        if is_str_keyed {
                            "str_dict_update"
                        } else {
                            "dict_update"
                        },
                        PyType::None,
                    )),
                    // Methods returning new dict
                    "copy" => Some((
                        if is_str_keyed {
                            "str_dict_copy"
                        } else {
                            "dict_copy"
                        },
                        PyType::Dict(key_type.clone(), val_type.clone()),
                    )),
                    // View methods returning iterators
                    "keys" => Some((
                        if is_str_keyed {
                            "str_dict_keys"
                        } else {
                            "dict_keys"
                        },
                        PyType::Instance(InstanceType::new(
                            iter_names::DICT_KEYS.to_string(),
                            vec![("element_type".to_string(), key_type.as_ref().clone())],
                        )),
                    )),
                    "values" => Some((
                        if is_str_keyed {
                            "str_dict_values"
                        } else {
                            "dict_values"
                        },
                        PyType::Instance(InstanceType::new(
                            iter_names::DICT_VALUES.to_string(),
                            vec![("element_type".to_string(), val_type.as_ref().clone())],
                        )),
                    )),
                    "items" => Some((
                        if is_str_keyed {
                            "str_dict_items"
                        } else {
                            "dict_items"
                        },
                        PyType::Instance(InstanceType::new(
                            iter_names::DICT_ITEMS.to_string(),
                            vec![
                                ("key_type".to_string(), key_type.as_ref().clone()),
                                ("value_type".to_string(), val_type.as_ref().clone()),
                            ],
                        )),
                    )),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

// ============================================================================
// FunctionInfo, ModuleInfo, MacroKind
// ============================================================================

/// Function storage - runtime-specific data for function calls
/// This is the "storage" part of the Function variant, analogous to PtrStorage for containers
#[derive(Clone, Debug)]
pub struct FunctionStorage<'ctx> {
    pub mangled_name: String,
    pub bound_args: Vec<BasicValueEnum<'ctx>>,
    /// For builtin functions that need special codegen (type dispatch, etc.)
    pub macro_kind: Option<MacroKind>,
}

/// Function metadata for compile-time function references
/// Functions are declared lazily via get_or_declare when needed
/// For builtin macros, macro_kind specifies the builtin function dispatch
#[derive(Clone, Debug)]
pub struct FunctionInfo<'ctx> {
    pub storage: FunctionStorage<'ctx>,
    pub fn_type: FunctionType,
}

impl<'ctx> FunctionInfo<'ctx> {
    /// Create a FunctionInfo for a builtin or method
    pub fn new(mangled_name: &str, return_type: PyType) -> Self {
        FunctionInfo {
            storage: FunctionStorage {
                mangled_name: mangled_name.to_string(),
                bound_args: vec![],
                macro_kind: None,
            },
            fn_type: FunctionType::new(vec![], return_type),
        }
    }

    /// Create a FunctionInfo with a bound receiver (for method calls)
    pub fn bound(mangled_name: &str, return_type: PyType, bound_arg: BasicValueEnum<'ctx>) -> Self {
        FunctionInfo {
            storage: FunctionStorage {
                mangled_name: mangled_name.to_string(),
                bound_args: vec![bound_arg],
                macro_kind: None,
            },
            fn_type: FunctionType::new(vec![], return_type),
        }
    }

    /// Create a FunctionInfo for a builtin macro (special codegen dispatch)
    pub fn builtin_macro(kind: MacroKind) -> Self {
        FunctionInfo {
            storage: FunctionStorage {
                mangled_name: String::new(),
                bound_args: vec![],
                macro_kind: Some(kind),
            },
            // Return type is determined at expansion time
            fn_type: FunctionType::new(vec![], PyType::None),
        }
    }

    /// Create a FunctionInfo from an AST function definition
    pub fn from_ast(mangled_name: &str, func: &crate::ast::Function) -> Self {
        let param_types: Vec<PyType> = func
            .params
            .iter()
            .filter_map(|p| PyType::from_ast_type(&p.param_type).ok())
            .collect();

        let return_type = PyType::from_ast_type(&func.return_type).unwrap_or(PyType::None);

        FunctionInfo {
            storage: FunctionStorage {
                mangled_name: mangled_name.to_string(),
                bound_args: vec![],
                macro_kind: None,
            },
            fn_type: FunctionType::new(param_types, return_type),
        }
    }

    // Convenience accessors for backward compatibility
    pub fn mangled_name(&self) -> &str {
        &self.storage.mangled_name
    }

    pub fn param_types(&self) -> &[PyType] {
        &self.fn_type.param_types
    }

    pub fn return_type(&self) -> PyType {
        self.fn_type.return_type()
    }

    pub fn bound_args(&self) -> &[BasicValueEnum<'ctx>] {
        &self.storage.bound_args
    }

    pub fn macro_kind(&self) -> Option<MacroKind> {
        self.storage.macro_kind
    }

    /// Check if this is a builtin macro
    pub fn is_macro(&self) -> bool {
        self.storage.macro_kind.is_some()
    }

    /// Get the function, declaring it in the module if needed
    pub fn get_or_declare(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
    ) -> FunctionValue<'ctx> {
        // Check if it's a C builtin - use BUILTIN_TABLE for correct signature and symbol
        if let Some(builtin) =
            crate::codegen::builtins::BUILTIN_TABLE.get(self.storage.mangled_name.as_str())
        {
            // Check if already declared with the actual symbol name
            if let Some(f) = module.get_function(builtin.symbol) {
                return f;
            }
            let fn_type = builtin.to_llvm_fn_type(context);
            // Use the actual symbol name from the table (e.g., "__builtin_tpy_bytes_ljust")
            return module.add_function(builtin.symbol, fn_type, None);
        }

        // For user-defined functions, check if already declared
        if let Some(f) = module.get_function(&self.storage.mangled_name) {
            return f;
        }

        // Declare the function using stored param_types
        self.declare_in_module(context, module)
    }

    fn declare_in_module(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
    ) -> FunctionValue<'ctx> {
        use inkwell::types::{BasicMetadataTypeEnum, BasicType};

        if let Some(f) = module.get_function(&self.storage.mangled_name) {
            return f;
        }

        let llvm_param_types: Vec<BasicMetadataTypeEnum> = self
            .fn_type
            .param_types
            .iter()
            .map(|t| t.to_llvm(context).into())
            .collect();

        let return_type = self.fn_type.return_type();
        let fn_type = match &return_type {
            PyType::None => context.void_type().fn_type(&llvm_param_types, false),
            _ => {
                let ret_type = return_type.to_llvm(context);
                ret_type.fn_type(&llvm_param_types, false)
            }
        };

        module.add_function(&self.storage.mangled_name, fn_type, None)
    }
}

/// Module metadata
#[derive(Clone)]
pub struct ModuleInfo<'ctx> {
    pub name: String,
    pub members: HashMap<String, PyValue<'ctx>>,
}

/// Builtin macro kinds
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MacroKind {
    Print,
    Abs,
    Round,
    Min,
    Max,
    Pow,
    Len,
    Set,
    List,
    Dict,
    Range,
    Int,
    Float,
    Bool,
    Str,
    Bin,
    Hex,
    Oct,
    Chr,
    Ord,
    Ascii,
    Sum,
    Sorted,
    Reversed,
    Divmod,
    Any,
    All,
    Tuple,
    Enumerate,
    Zip,
    Filter,
    Iter,
    Next,
    Id,
    Repr,
    Frozenset,
    Getattr,
    Hasattr,
}

// ============================================================================
// PyValue - The main enum with specific LLVM types
// ============================================================================

/// Python value - type is encoded in the variant, LLVM type is specific
#[derive(Clone)]
pub enum PyValue<'ctx> {
    // Scalar types with storage unions (Immediate | Pointer)
    Int(IntStorage<'ctx>),
    Float(FloatStorage<'ctx>),
    Bool(BoolStorage<'ctx>),
    None(NoneStorage<'ctx>),
    // Pointer types with storage unions (Direct | Alloca)
    Str(PtrStorage<'ctx>),
    Bytes(PtrStorage<'ctx>),
    // Container types with element type info
    List(PtrStorage<'ctx>, Box<PyType>),
    Dict(PtrStorage<'ctx>, Box<PyType>, Box<PyType>),
    Set(PtrStorage<'ctx>, Box<PyType>),
    Tuple(PtrStorage<'ctx>, Vec<PyType>),
    /// Instance of a class with named fields (like a named tuple/struct)
    /// Fields store type information for attributes
    /// For generators: Instance with class_name = "__builtin_generator" and ("yield_type", T)
    /// For range: Instance with class_name = "__builtin_range"
    /// For exception: Instance with class_name = "__builtin_exception"
    Instance(PtrStorage<'ctx>, InstanceType),
    // Compile-time types - follow XXValue(Storage, Type) pattern
    /// Function - storage contains name/bound_args/macro_kind, type contains param/return types
    Function(FunctionStorage<'ctx>, FunctionType),
    Module(ModuleInfo<'ctx>),
}

impl<'ctx> PyValue<'ctx> {
    // ========================================================================
    // Constructors
    // ========================================================================

    pub fn int(value: IntValue<'ctx>) -> Self {
        PyValue::Int(IntStorage::Immediate(value))
    }

    pub fn float(value: FloatValue<'ctx>) -> Self {
        PyValue::Float(FloatStorage::Immediate(value))
    }

    pub fn bool(value: IntValue<'ctx>) -> Self {
        PyValue::Bool(BoolStorage::Immediate(value))
    }

    pub fn new_str(value: PointerValue<'ctx>) -> Self {
        PyValue::Str(PtrStorage::Direct(value))
    }

    pub fn bytes(value: PointerValue<'ctx>) -> Self {
        PyValue::Bytes(PtrStorage::Direct(value))
    }

    pub fn none(value: IntValue<'ctx>) -> Self {
        PyValue::None(NoneStorage::Immediate(value))
    }

    pub fn range(value: PointerValue<'ctx>) -> Self {
        PyValue::Instance(
            PtrStorage::Direct(value),
            InstanceType::new(iter_names::RANGE.to_string(), vec![]),
        )
    }

    pub fn function(info: FunctionInfo<'ctx>) -> Self {
        PyValue::Function(info.storage, info.fn_type)
    }

    pub fn module(info: ModuleInfo<'ctx>) -> Self {
        PyValue::Module(info)
    }

    /// Create a builtin macro function
    pub fn macro_fn(kind: MacroKind) -> Self {
        let info = FunctionInfo::builtin_macro(kind);
        PyValue::Function(info.storage, info.fn_type)
    }

    /// Create with pointer (for variables) - converts from BasicValueEnum
    pub fn new(value: BasicValueEnum<'ctx>, ty: PyType, ptr: Option<PointerValue<'ctx>>) -> Self {
        match (ty, ptr) {
            (PyType::Int, Some(p)) => PyValue::Int(IntStorage::Pointer(p)),
            (PyType::Int, None) => PyValue::Int(IntStorage::Immediate(value.into_int_value())),
            (PyType::Float, Some(p)) => PyValue::Float(FloatStorage::Pointer(p)),
            (PyType::Float, None) => {
                PyValue::Float(FloatStorage::Immediate(value.into_float_value()))
            }
            (PyType::Bool, Some(p)) => PyValue::Bool(BoolStorage::Pointer(p)),
            (PyType::Bool, None) => PyValue::Bool(BoolStorage::Immediate(value.into_int_value())),
            (PyType::Str, Some(p)) => PyValue::Str(PtrStorage::Alloca(p)),
            (PyType::Str, None) => PyValue::Str(PtrStorage::Direct(value.into_pointer_value())),
            (PyType::Bytes, Some(p)) => PyValue::Bytes(PtrStorage::Alloca(p)),
            (PyType::Bytes, None) => PyValue::Bytes(PtrStorage::Direct(value.into_pointer_value())),
            (PyType::None, Some(p)) => PyValue::None(NoneStorage::Pointer(p)),
            (PyType::None, None) => PyValue::None(NoneStorage::Immediate(value.into_int_value())),
            (PyType::List(elem), Some(p)) => PyValue::List(PtrStorage::Alloca(p), elem),
            (PyType::List(elem), None) => {
                PyValue::List(PtrStorage::Direct(value.into_pointer_value()), elem)
            }
            (PyType::Dict(k, v), Some(p)) => PyValue::Dict(PtrStorage::Alloca(p), k, v),
            (PyType::Dict(k, v), None) => {
                PyValue::Dict(PtrStorage::Direct(value.into_pointer_value()), k, v)
            }
            (PyType::Set(elem), Some(p)) => PyValue::Set(PtrStorage::Alloca(p), elem),
            (PyType::Set(elem), None) => {
                PyValue::Set(PtrStorage::Direct(value.into_pointer_value()), elem)
            }
            (PyType::Tuple(elem), Some(p)) => PyValue::Tuple(PtrStorage::Alloca(p), elem),
            (PyType::Tuple(elem), None) => {
                PyValue::Tuple(PtrStorage::Direct(value.into_pointer_value()), elem)
            }
            (PyType::Instance(inst), Some(p)) => PyValue::Instance(PtrStorage::Alloca(p), inst),
            (PyType::Instance(inst), None) => {
                PyValue::Instance(PtrStorage::Direct(value.into_pointer_value()), inst)
            }
            (PyType::Function(_), _) | (PyType::Module, _) => {
                panic!("Use specific constructors for Function/Module")
            }
        }
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Get the type tag
    pub fn ty(&self) -> PyType {
        match self {
            PyValue::Int(_) => PyType::Int,
            PyValue::Float(_) => PyType::Float,
            PyValue::Bool(_) => PyType::Bool,
            PyValue::Str(_) => PyType::Str,
            PyValue::Bytes(_) => PyType::Bytes,
            PyValue::None(_) => PyType::None,
            PyValue::List(_, elem) => PyType::List(elem.clone()),
            PyValue::Dict(_, k, v) => PyType::Dict(k.clone(), v.clone()),
            PyValue::Set(_, elem) => PyType::Set(elem.clone()),
            PyValue::Tuple(_, elem) => PyType::Tuple(elem.clone()),
            PyValue::Instance(_, inst) => PyType::Instance(inst.clone()),
            PyValue::Function(_, fn_type) => PyType::Function(fn_type.clone()),
            PyValue::Module(_) => PyType::Module,
        }
    }

    /// Get the LLVM value as BasicValueEnum (for compatibility)
    /// For storage types with Pointer variant, this returns the pointer, not loaded value
    pub fn value(&self) -> BasicValueEnum<'ctx> {
        match self {
            PyValue::Int(IntStorage::Immediate(v)) => (*v).into(),
            PyValue::Int(IntStorage::Pointer(p)) => (*p).into(),
            PyValue::Float(FloatStorage::Immediate(v)) => (*v).into(),
            PyValue::Float(FloatStorage::Pointer(p)) => (*p).into(),
            PyValue::Bool(BoolStorage::Immediate(v)) => (*v).into(),
            PyValue::Bool(BoolStorage::Pointer(p)) => (*p).into(),
            PyValue::None(NoneStorage::Immediate(v)) => (*v).into(),
            PyValue::None(NoneStorage::Pointer(p)) => (*p).into(),
            PyValue::Str(PtrStorage::Direct(v)) | PyValue::Str(PtrStorage::Alloca(v)) => {
                (*v).into()
            }
            PyValue::Bytes(PtrStorage::Direct(v)) | PyValue::Bytes(PtrStorage::Alloca(v)) => {
                (*v).into()
            }
            PyValue::List(PtrStorage::Direct(v), _) | PyValue::List(PtrStorage::Alloca(v), _) => {
                (*v).into()
            }
            PyValue::Dict(PtrStorage::Direct(v), _, _)
            | PyValue::Dict(PtrStorage::Alloca(v), _, _) => (*v).into(),
            PyValue::Set(PtrStorage::Direct(v), _) | PyValue::Set(PtrStorage::Alloca(v), _) => {
                (*v).into()
            }
            PyValue::Tuple(PtrStorage::Direct(v), _) | PyValue::Tuple(PtrStorage::Alloca(v), _) => {
                (*v).into()
            }
            PyValue::Instance(PtrStorage::Direct(v), _)
            | PyValue::Instance(PtrStorage::Alloca(v), _) => (*v).into(),
            PyValue::Function(_, _) => {
                // Functions don't have a direct LLVM value - use get_or_declare to call them
                panic!("Function has no direct LLVM value - use get_or_declare to call it")
            }
            PyValue::Module(_) => panic!("Module has no LLVM value"),
        }
    }

    /// Get the pointer (for addressable values - variables that can be reassigned)
    pub fn ptr(&self) -> Option<PointerValue<'ctx>> {
        match self {
            PyValue::Int(IntStorage::Pointer(p)) => Some(*p),
            PyValue::Float(FloatStorage::Pointer(p)) => Some(*p),
            PyValue::Bool(BoolStorage::Pointer(p)) => Some(*p),
            PyValue::None(NoneStorage::Pointer(p)) => Some(*p),
            PyValue::Str(PtrStorage::Alloca(p)) => Some(*p),
            PyValue::Bytes(PtrStorage::Alloca(p)) => Some(*p),
            PyValue::List(PtrStorage::Alloca(p), _) => Some(*p),
            PyValue::Dict(PtrStorage::Alloca(p), _, _) => Some(*p),
            PyValue::Set(PtrStorage::Alloca(p), _) => Some(*p),
            PyValue::Tuple(PtrStorage::Alloca(p), _) => Some(*p),
            _ => None,
        }
    }

    /// Get FloatValue (for Float) - panics if Pointer variant
    pub fn float_value(&self) -> FloatValue<'ctx> {
        match self {
            PyValue::Float(FloatStorage::Immediate(v)) => *v,
            PyValue::Float(FloatStorage::Pointer(_)) => {
                panic!("float_value called on pointer storage")
            }
            _ => panic!("float_value called on non-float type"),
        }
    }

    // For backwards compatibility
    pub fn runtime_value(&self) -> BasicValueEnum<'ctx> {
        self.value()
    }

    pub fn module_info(&self) -> &ModuleInfo<'ctx> {
        match self {
            PyValue::Module(info) => info,
            _ => panic!("module_info called on non-module"),
        }
    }

    pub fn get_function(&self) -> FunctionInfo<'ctx> {
        match self {
            PyValue::Function(storage, fn_type) => FunctionInfo {
                storage: storage.clone(),
                fn_type: fn_type.clone(),
            },
            _ => panic!("get_function called on non-function"),
        }
    }

    /// Get the macro kind if this is a builtin macro function
    pub fn get_macro_kind(&self) -> Option<MacroKind> {
        match self {
            PyValue::Function(storage, _) => storage.macro_kind,
            _ => None,
        }
    }

    // ========================================================================
    // Load/Store Operations
    // ========================================================================

    /// Load the current value from memory (for pointer storage types and container allocas)
    /// Returns an Immediate variant with the loaded value
    pub fn load(&self, builder: &Builder<'ctx>, ctx: &'ctx Context, name: &str) -> PyValue<'ctx> {
        let ptr_type = ctx.ptr_type(inkwell::AddressSpace::default());
        match self {
            PyValue::Int(IntStorage::Pointer(p)) => {
                let loaded = builder
                    .build_load(ctx.i64_type(), *p, name)
                    .unwrap()
                    .into_int_value();
                PyValue::Int(IntStorage::Immediate(loaded))
            }
            PyValue::Float(FloatStorage::Pointer(p)) => {
                let loaded = builder
                    .build_load(ctx.f64_type(), *p, name)
                    .unwrap()
                    .into_float_value();
                PyValue::Float(FloatStorage::Immediate(loaded))
            }
            PyValue::Bool(BoolStorage::Pointer(p)) => {
                let loaded = builder
                    .build_load(ctx.bool_type(), *p, name)
                    .unwrap()
                    .into_int_value();
                PyValue::Bool(BoolStorage::Immediate(loaded))
            }
            PyValue::None(NoneStorage::Pointer(p)) => {
                let loaded = builder
                    .build_load(ctx.i32_type(), *p, name)
                    .unwrap()
                    .into_int_value();
                PyValue::None(NoneStorage::Immediate(loaded))
            }
            // Container types with allocas - load the pointer from alloca
            PyValue::Str(PtrStorage::Alloca(alloca)) => {
                let loaded = builder
                    .build_load(ptr_type, *alloca, name)
                    .unwrap()
                    .into_pointer_value();
                PyValue::Str(PtrStorage::Direct(loaded))
            }
            PyValue::Bytes(PtrStorage::Alloca(alloca)) => {
                let loaded = builder
                    .build_load(ptr_type, *alloca, name)
                    .unwrap()
                    .into_pointer_value();
                PyValue::Bytes(PtrStorage::Direct(loaded))
            }
            PyValue::List(PtrStorage::Alloca(alloca), elem) => {
                let loaded = builder
                    .build_load(ptr_type, *alloca, name)
                    .unwrap()
                    .into_pointer_value();
                PyValue::List(PtrStorage::Direct(loaded), elem.clone())
            }
            PyValue::Dict(PtrStorage::Alloca(alloca), k, v) => {
                let loaded = builder
                    .build_load(ptr_type, *alloca, name)
                    .unwrap()
                    .into_pointer_value();
                PyValue::Dict(PtrStorage::Direct(loaded), k.clone(), v.clone())
            }
            PyValue::Set(PtrStorage::Alloca(alloca), elem) => {
                let loaded = builder
                    .build_load(ptr_type, *alloca, name)
                    .unwrap()
                    .into_pointer_value();
                PyValue::Set(PtrStorage::Direct(loaded), elem.clone())
            }
            PyValue::Tuple(PtrStorage::Alloca(alloca), elem) => {
                let loaded = builder
                    .build_load(ptr_type, *alloca, name)
                    .unwrap()
                    .into_pointer_value();
                PyValue::Tuple(PtrStorage::Direct(loaded), elem.clone())
            }
            // Instances (including builtin iterators, range, exception) can be loaded from allocas
            PyValue::Instance(PtrStorage::Alloca(alloca), inst) => {
                let loaded = builder
                    .build_load(ptr_type, *alloca, name)
                    .unwrap()
                    .into_pointer_value();
                PyValue::Instance(PtrStorage::Direct(loaded), inst.clone())
            }
            // Non-pointer types just return a clone
            _ => self.clone(),
        }
    }

    /// Store a value to this variable (must be a pointer storage type)
    pub fn store(
        &self,
        builder: &Builder<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        match self.ptr() {
            Some(p) => {
                builder.build_store(p, value).unwrap();
                Ok(())
            }
            None => Err("Cannot store to a non-addressable value".to_string()),
        }
    }

    /// Store another PyValue to this variable
    pub fn store_value(
        &self,
        builder: &Builder<'ctx>,
        value: &PyValue<'ctx>,
    ) -> Result<(), String> {
        self.store(builder, value.value())
    }

    /// Check if two PyValues have the same type
    pub fn same_type(&self, other: &PyValue<'ctx>) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    // ========================================================================
    // Binary Operations
    // ========================================================================

    pub fn binary_op(
        &self,
        cg: &CgCtx<'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match self {
            PyValue::Int(_) => super::int_ops::binary_op(self, cg, op, rhs),
            PyValue::Float(_) => super::float_ops::binary_op(self, cg, op, rhs),
            PyValue::Bool(_) => super::bool_ops::binary_op(self, cg, op, rhs),
            PyValue::Str(_) => super::str_ops::binary_op(self, cg, op, rhs),
            PyValue::Bytes(_) => super::bytes_ops::binary_op(self, cg, op, rhs),
            PyValue::None(_) => super::none_ops::binary_op(self, cg, op, rhs),
            PyValue::List(_, _) => super::list_ops::binary_op(self, cg, op, rhs),
            PyValue::Dict(_, _, _) => super::dict_ops::binary_op(self, cg, op, rhs),
            PyValue::Set(_, _) => super::set_ops::binary_op(self, cg, op, rhs),
            PyValue::Tuple(_, _) => Err("Binary operations not supported on tuples".to_string()),
            PyValue::Instance(_, _) => {
                Err("Binary operations not supported on instances".to_string())
            }
            PyValue::Function(_, _) => {
                Err("Binary operations not supported on functions".to_string())
            }
            PyValue::Module(_) => Err("Binary operations not supported on modules".to_string()),
        }
    }

    // ========================================================================
    // Unary Operations
    // ========================================================================

    pub fn unary_op(&self, cg: &CgCtx<'ctx>, op: &UnaryOp) -> Result<PyValue<'ctx>, String> {
        match self {
            PyValue::Int(_) => super::int_ops::unary_op(self, cg, op),
            PyValue::Float(_) => super::float_ops::unary_op(self, cg, op),
            PyValue::Bool(_) => super::bool_ops::unary_op(self, cg, op),
            PyValue::Str(_) => super::str_ops::unary_op(self, cg, op),
            PyValue::Bytes(_) => super::bytes_ops::unary_op(self, cg, op),
            PyValue::None(_) => super::none_ops::unary_op(self, cg, op),
            PyValue::List(_, _) => super::list_ops::unary_op(self, cg, op),
            PyValue::Dict(_, _, _) => super::dict_ops::unary_op(self, cg, op),
            PyValue::Set(_, _) => super::set_ops::unary_op(self, cg, op),
            PyValue::Tuple(_, _) => Err(format!("Unary operator {:?} not supported on tuple", op)),
            PyValue::Instance(_, inst) => {
                // Range supports 'not' operator
                if inst.class_name == iter_names::RANGE {
                    match op {
                        UnaryOp::Not => {
                            // not range: true if range is empty, false otherwise
                            let bool_val = cg.value_to_bool(self);
                            let negated = cg.builder.build_not(bool_val, "not_range").unwrap();
                            Ok(PyValue::bool(negated))
                        }
                        _ => Err(format!("Unary operator {:?} not supported on range", op)),
                    }
                } else {
                    Err(format!("Unary operator {:?} not supported on instance", op))
                }
            }
            PyValue::Function(_, _) => {
                Err("Unary operations not supported on functions".to_string())
            }
            PyValue::Module(_) => Err("Unary operations not supported on modules".to_string()),
        }
    }

    // ========================================================================
    // Module Operations
    // ========================================================================

    /// Get a member (method or attribute) from this value
    /// Works for modules (returns stored member) and types (returns bound method)
    pub fn get_member(&self, name: &str) -> Result<PyValue<'ctx>, String> {
        match self {
            PyValue::Module(info) => info
                .members
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Module '{}' has no member '{}'", info.name, name)),
            _ => {
                // For other types, look up the method
                let ty = self.ty();
                let (symbol, return_type) = ty
                    .lookup_member(name)
                    .ok_or_else(|| format!("{:?} has no member '{}'", ty, name))?;

                Ok(PyValue::function(FunctionInfo::bound(
                    symbol,
                    return_type,
                    self.value(),
                )))
            }
        }
    }

    pub fn add_member(&mut self, name: String, value: PyValue<'ctx>) -> Result<(), String> {
        match self {
            PyValue::Module(info) => {
                info.members.insert(name, value);
                Ok(())
            }
            _ => Err(format!("add_member called on non-module: {:?}", self.ty())),
        }
    }
}

// ============================================================================
// CgCtx - Code Generation Context
// ============================================================================

pub struct CgCtx<'ctx> {
    pub ctx: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,
}

impl<'ctx> CgCtx<'ctx> {
    pub fn new(ctx: &'ctx Context, module_name: &str) -> Self {
        let module = ctx.create_module(module_name);
        let builder = ctx.create_builder();
        Self {
            ctx,
            builder,
            module,
        }
    }

    /// Convert a PyValue to a boolean (i1)
    /// Note: Values should be loaded before calling this (use .load() first for Alloca variants)
    pub fn value_to_bool(&self, val: &PyValue<'ctx>) -> IntValue<'ctx> {
        match val {
            PyValue::Bool(BoolStorage::Immediate(v)) => *v,
            PyValue::Bool(BoolStorage::Pointer(p)) => {
                let loaded = self
                    .builder
                    .build_load(self.ctx.bool_type(), *p, "load_bool")
                    .unwrap()
                    .into_int_value();
                loaded
            }
            PyValue::Int(IntStorage::Immediate(v)) => {
                let zero = v.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, *v, zero, "to_bool")
                    .unwrap()
            }
            PyValue::Int(IntStorage::Pointer(p)) => {
                let loaded = self
                    .builder
                    .build_load(self.ctx.i64_type(), *p, "load_int")
                    .unwrap()
                    .into_int_value();
                let zero = loaded.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, loaded, zero, "to_bool")
                    .unwrap()
            }
            PyValue::Float(FloatStorage::Immediate(v)) => {
                let zero = self.ctx.f64_type().const_zero();
                self.builder
                    .build_float_compare(inkwell::FloatPredicate::ONE, *v, zero, "to_bool")
                    .unwrap()
            }
            PyValue::Float(FloatStorage::Pointer(p)) => {
                let loaded = self
                    .builder
                    .build_load(self.ctx.f64_type(), *p, "load_float")
                    .unwrap()
                    .into_float_value();
                let zero = self.ctx.f64_type().const_zero();
                self.builder
                    .build_float_compare(inkwell::FloatPredicate::ONE, loaded, zero, "to_bool")
                    .unwrap()
            }
            PyValue::None(_) => self.ctx.bool_type().const_zero(),
            PyValue::Str(PtrStorage::Direct(v)) | PyValue::Str(PtrStorage::Alloca(v)) => {
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "str_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[(*v).into()], "str_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "str_len");
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyValue::Bytes(PtrStorage::Direct(v)) | PyValue::Bytes(PtrStorage::Alloca(v)) => {
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "bytes_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[(*v).into()], "bytes_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "bytes_len");
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyValue::List(PtrStorage::Direct(v), _) | PyValue::List(PtrStorage::Alloca(v), _) => {
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "list_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[(*v).into()], "list_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "list_len");
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyValue::Dict(PtrStorage::Direct(v), _, _)
            | PyValue::Dict(PtrStorage::Alloca(v), _, _) => {
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "dict_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[(*v).into()], "dict_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "dict_len");
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyValue::Set(PtrStorage::Direct(v), _) | PyValue::Set(PtrStorage::Alloca(v), _) => {
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "set_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[(*v).into()], "set_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "set_len");
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyValue::Tuple(_, _) => self.ctx.bool_type().const_int(1, false),
            // Instances - most are truthy, but range checks length
            PyValue::Instance(storage, inst) => {
                if inst.class_name == iter_names::RANGE {
                    // Range is truthy if it has at least one element
                    let range_ptr = match storage {
                        PtrStorage::Alloca(alloca) => {
                            let ptr_type = self.ctx.ptr_type(inkwell::AddressSpace::default());
                            self.builder
                                .build_load(ptr_type, *alloca, "range_load")
                                .unwrap()
                                .into_pointer_value()
                        }
                        PtrStorage::Direct(v) => *v,
                    };
                    let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "range_len");
                    let len_call = self
                        .builder
                        .build_call(len_fn, &[range_ptr.into()], "range_len")
                        .unwrap();
                    let len = super::extract_int_result(len_call, "range_len");
                    let zero = len.get_type().const_zero();
                    self.builder
                        .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                        .unwrap()
                } else {
                    // Other instances are always truthy (unless __bool__ is implemented)
                    self.ctx.bool_type().const_int(1, false)
                }
            }
            PyValue::Function(_, _) | PyValue::Module(_) => {
                self.ctx.bool_type().const_int(1, false)
            }
        }
    }
}
