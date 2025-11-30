//! PyValue - Core Python value type definitions
//!
//! This module provides the core PyValue struct that combines an LLVM value
//! with its Python type information. Operations are implemented in separate modules.
//!
//! PyValue can represent:
//! - Runtime values (Int, Float, Bool, Bytes, None) with BasicValueEnum
//! - Compile-time constructs (Module, Function) with LLVM module/function pointers

use crate::ast::{BinaryOp, Type, UnaryOp};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use std::collections::HashMap;

/// Code generation context bundling LLVM context, builder, and module.
/// Owns the Builder and Module, holds a reference to Context.
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

    /// Convert a PyValue to a boolean (i1) for use in `and`/`or` operators
    pub fn value_to_bool(&self, val: &PyValue<'ctx>) -> inkwell::values::IntValue<'ctx> {
        match &val.ty {
            PyType::Bool => {
                // Already a bool
                val.value().into_int_value()
            }
            PyType::Int => {
                // Non-zero is true
                let int_val = val.value().into_int_value();
                let zero = int_val.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, int_val, zero, "to_bool")
                    .unwrap()
            }
            PyType::Float => {
                let float_val = val.value().into_float_value();
                let zero = self.ctx.f64_type().const_zero();
                self.builder
                    .build_float_compare(inkwell::FloatPredicate::ONE, float_val, zero, "to_bool")
                    .unwrap()
            }
            PyType::None => {
                // None is always falsy
                self.ctx.bool_type().const_zero()
            }
            PyType::Str => {
                // Str is truthy if non-empty (check length > 0)
                let ptr_val = val.value().into_pointer_value();
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "str_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "str_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "str_len").into_int_value();
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyType::Bytes => {
                // Bytes is truthy if non-empty (check length > 0)
                let ptr_val = val.value().into_pointer_value();
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "bytes_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "bytes_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "bytes_len").into_int_value();
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyType::Function | PyType::Module | PyType::Macro => {
                // Functions, modules, and macros are always truthy
                self.ctx.bool_type().const_int(1, false)
            }
            PyType::List(_) => {
                // List is truthy if non-empty (check length > 0)
                let ptr_val = val.value().into_pointer_value();
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "list_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "list_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "list_len").into_int_value();
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyType::Dict(_, _) => {
                // Dict is truthy if non-empty (check length > 0)
                let ptr_val = val.value().into_pointer_value();
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "dict_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "dict_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "dict_len").into_int_value();
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyType::Set(_) => {
                // Set is truthy if non-empty (check length > 0)
                let ptr_val = val.value().into_pointer_value();
                let len_fn = super::get_or_declare_builtin(&self.module, self.ctx, "set_len");
                let len_call = self
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "set_len")
                    .unwrap();
                let len = super::extract_int_result(len_call, "set_len").into_int_value();
                let zero = len.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, len, zero, "to_bool")
                    .unwrap()
            }
            PyType::Tuple(_) => {
                // Tuples from divmod are always non-empty, so truthy
                self.ctx.bool_type().const_int(1, false)
            }
        }
    }

    /// Convert PyType to LLVM BasicTypeEnum
    pub fn pytype_to_llvm(&self, ty: &PyType) -> inkwell::types::BasicTypeEnum<'ctx> {
        pytype_to_llvm(self.ctx, ty)
    }
}

/// Convert PyType to LLVM BasicTypeEnum (standalone function)
pub fn pytype_to_llvm<'ctx>(
    ctx: &'ctx Context,
    ty: &PyType,
) -> inkwell::types::BasicTypeEnum<'ctx> {
    match ty {
        PyType::Int => ctx.i64_type().into(),
        PyType::Float => ctx.f64_type().into(),
        PyType::Bool => ctx.bool_type().into(),
        PyType::Str => ctx.ptr_type(inkwell::AddressSpace::default()).into(),
        PyType::Bytes => ctx.ptr_type(inkwell::AddressSpace::default()).into(),
        PyType::None => ctx.i32_type().into(),
        PyType::List(_) => ctx.ptr_type(inkwell::AddressSpace::default()).into(),
        PyType::Dict(_, _) => ctx.ptr_type(inkwell::AddressSpace::default()).into(),
        PyType::Set(_) => ctx.ptr_type(inkwell::AddressSpace::default()).into(),
        PyType::Tuple(_) => ctx.ptr_type(inkwell::AddressSpace::default()).into(),
        PyType::Function | PyType::Module | PyType::Macro => ctx.i64_type().into(),
    }
}

/// Function metadata for compile-time function references
/// Note: The FunctionValue can become invalid if the underlying module is dropped.
/// Use `declare_in_module` to create a valid FunctionValue in a target module.
#[derive(Clone, Debug)]
pub struct FunctionInfo<'ctx> {
    /// The mangled function name (module_function)
    pub mangled_name: String,
    /// The LLVM function value (may be from a placeholder module, use with caution)
    pub function: FunctionValue<'ctx>,
    /// Parameter types
    pub param_types: Vec<PyType>,
    /// Return type
    pub return_type: PyType,
    /// Bound arguments (prepended to call args at call time)
    /// Used for method calls where the receiver is pre-bound
    pub bound_args: Vec<BasicValueEnum<'ctx>>,
}

impl<'ctx> FunctionInfo<'ctx> {
    /// Create a FunctionInfo from an AST function definition
    /// Creates a placeholder function in a temporary module for type info
    /// WARNING: The FunctionValue is from a temporary module and should not be used directly.
    /// Use `declare_in_module` to create a valid FunctionValue in the target module.
    pub fn from_ast(
        context: &'ctx inkwell::context::Context,
        mangled_name: &str,
        func: &crate::ast::Function,
    ) -> Self {
        use inkwell::types::{BasicMetadataTypeEnum, BasicType};

        // Create a placeholder module for this function declaration
        // NOTE: This placeholder module will be dropped, making the FunctionValue invalid!
        // We keep it only to satisfy the struct requirements, but we use param_types for actual declaration.
        let placeholder_module = context.create_module("__placeholder__");

        // Convert parameter types
        let param_types: Vec<PyType> = func
            .params
            .iter()
            .filter_map(|p| PyType::from_ast_type(&p.param_type).ok())
            .collect();

        let return_type = PyType::from_ast_type(&func.return_type).unwrap_or(PyType::None);

        // Build LLVM param types
        let llvm_param_types: Vec<BasicMetadataTypeEnum> = param_types
            .iter()
            .map(|p| pytype_to_llvm(context, p).into())
            .collect();

        // Build function type
        let fn_type = match return_type {
            PyType::None => context.void_type().fn_type(&llvm_param_types, false),
            _ => {
                let ret_type = pytype_to_llvm(context, &return_type);
                ret_type.fn_type(&llvm_param_types, false)
            }
        };

        let function = placeholder_module.add_function(mangled_name, fn_type, None);

        FunctionInfo {
            mangled_name: mangled_name.to_string(),
            function,
            param_types,
            return_type,
            bound_args: vec![],
        }
    }

    /// Declare this function in a target module, returning a valid FunctionValue
    pub fn declare_in_module(
        &self,
        context: &'ctx inkwell::context::Context,
        module: &inkwell::module::Module<'ctx>,
    ) -> FunctionValue<'ctx> {
        use inkwell::types::{BasicMetadataTypeEnum, BasicType};

        // Check if already declared
        if let Some(f) = module.get_function(&self.mangled_name) {
            return f;
        }

        // Build LLVM param types from PyType
        let llvm_param_types: Vec<BasicMetadataTypeEnum> = self
            .param_types
            .iter()
            .map(|t| pytype_to_llvm(context, t).into())
            .collect();

        // Build function type
        let fn_type = match &self.return_type {
            PyType::None => context.void_type().fn_type(&llvm_param_types, false),
            _ => {
                let ret_type = pytype_to_llvm(context, &self.return_type);
                ret_type.fn_type(&llvm_param_types, false)
            }
        };

        module.add_function(&self.mangled_name, fn_type, None)
    }
}

/// Module metadata containing its members (functions and submodules)
#[derive(Clone)]
pub struct ModuleInfo<'ctx> {
    /// Module name (e.g., "math" or "<builtin>.math")
    pub name: String,
    /// Members: functions and submodules accessible via this module
    /// Maps local name -> PyValue (which can be Function or Module)
    pub members: HashMap<String, PyValue<'ctx>>,
}

/// Builtin macro kinds - these expand at call time with special handling
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
    // Type conversion builtins
    Int,
    Float,
    Bool,
    Str,
    // String representation builtins
    Bin,
    Hex,
    Oct,
    Chr,
    Ord,
    Ascii,
    // Sequence builtins
    Sum,
    Sorted,
    Reversed,
    // Math builtins
    Divmod,
}

/// Python type enum - represents the type without an LLVM value
#[derive(Clone, Debug, PartialEq)]
pub enum PyType {
    // Runtime types (have BasicValueEnum)
    Int,
    Float,
    Bool,
    Str,
    Bytes,
    None,
    // Container types (heap-allocated, pointer values)
    List(Box<PyType>),              // Element type
    Dict(Box<PyType>, Box<PyType>), // Key type, Value type
    Set(Box<PyType>),               // Element type
    Tuple(Box<PyType>),             // Element type (for divmod results)
    // Compile-time types (have special LLVM values)
    Function,
    Module,
    /// Macro function - expands at call time with special handling
    /// Used for builtins like print, min, max that need type-based dispatch
    Macro,
}

impl PyType {
    /// Create a PyType from an AST Type
    pub fn from_ast_type(ty: &Type) -> Result<Self, String> {
        match ty {
            Type::Int => Ok(PyType::Int),
            Type::Float => Ok(PyType::Float),
            Type::Bool => Ok(PyType::Bool),
            Type::Bytes => Ok(PyType::Bytes),
            Type::Str => Ok(PyType::Str),
            Type::None => Ok(PyType::None),
            Type::List(elem_ty) => Ok(PyType::List(Box::new(PyType::from_ast_type(elem_ty)?))),
            Type::Dict(key_ty, val_ty) => Ok(PyType::Dict(
                Box::new(PyType::from_ast_type(key_ty)?),
                Box::new(PyType::from_ast_type(val_ty)?),
            )),
            Type::Set(elem_ty) => Ok(PyType::Set(Box::new(PyType::from_ast_type(elem_ty)?))),
            Type::Tuple(_) => Err("Tuple type not yet implemented".to_string()),
            Type::Custom(name) => Err(format!("Custom type '{}' not yet implemented", name)),
        }
    }
}

/// The inner value of a PyValue - either a runtime LLVM value or compile-time construct
#[derive(Clone)]
pub enum PyValueInner<'ctx> {
    /// Runtime value with optional address for lvalues
    Runtime {
        value: BasicValueEnum<'ctx>,
        ptr: Option<PointerValue<'ctx>>,
    },
    /// Compile-time function reference
    Function(FunctionInfo<'ctx>),
    /// Compile-time module reference
    Module(ModuleInfo<'ctx>),
    /// Macro function that expands at call time (e.g., print, min, max)
    Macro(MacroKind),
}

/// A Python value paired with its type information.
/// This combines an LLVM IR value with its Python-level type.
///
/// For runtime types (Int, Float, Bool, Bytes, None):
/// - Values can optionally have an address (pointer to stack allocation)
/// - With address: lvalue that can be loaded/stored (variables)
/// - Without address: rvalue/temporary (literals, expression results)
///
/// For compile-time types (Function, Module):
/// - Hold metadata about functions/modules for name resolution
/// - Function holds FunctionValue for calling
/// - Module holds member map for attribute access
#[derive(Clone)]
pub struct PyValue<'ctx> {
    pub ty: PyType,
    pub inner: PyValueInner<'ctx>,
}

impl<'ctx> PyValue<'ctx> {
    /// Create a new runtime Python value
    /// - Without ptr: rvalue (literals, expression results)
    /// - With ptr: lvalue (variables that can be stored to)
    pub fn new(value: BasicValueEnum<'ctx>, ty: PyType, ptr: Option<PointerValue<'ctx>>) -> Self {
        Self {
            ty,
            inner: PyValueInner::Runtime { value, ptr },
        }
    }

    /// Create a Python int value
    pub fn int(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Int, None)
    }

    /// Create a Python float value
    pub fn float(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Float, None)
    }

    /// Create a Python bool value
    pub fn bool(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Bool, None)
    }

    /// Create a Python str value
    pub fn new_str(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Str, None)
    }

    /// Create a Python bytes value
    pub fn bytes(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Bytes, None)
    }

    /// Create a Python none value
    pub fn none(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::None, None)
    }

    /// Create a Python function value
    pub fn function(info: FunctionInfo<'ctx>) -> Self {
        Self {
            ty: PyType::Function,
            inner: PyValueInner::Function(info),
        }
    }

    /// Create a Python module value
    pub fn module(info: ModuleInfo<'ctx>) -> Self {
        Self {
            ty: PyType::Module,
            inner: PyValueInner::Module(info),
        }
    }

    /// Create a macro function value (expands at call time)
    pub fn macro_fn(kind: MacroKind) -> Self {
        Self {
            ty: PyType::Macro,
            inner: PyValueInner::Macro(kind),
        }
    }

    /// Create a PyValue from an AST Type and an LLVM value
    pub fn from_ast_type(
        ty: &Type,
        value: BasicValueEnum<'ctx>,
        ptr: Option<PointerValue<'ctx>>,
    ) -> Result<Self, String> {
        Ok(Self::new(value, PyType::from_ast_type(ty)?, ptr))
    }

    /// Get the runtime value (panics if not a runtime type)
    pub fn runtime_value(&self) -> BasicValueEnum<'ctx> {
        match &self.inner {
            PyValueInner::Runtime { value, .. } => *value,
            _ => panic!("Expected runtime value, got {:?}", self.ty),
        }
    }

    /// Get the module info (panics if not a module type)
    pub fn module_info(&self) -> &ModuleInfo<'ctx> {
        match &self.inner {
            PyValueInner::Module(info) => info,
            _ => panic!("Expected module value, got {:?}", self.ty),
        }
    }

    /// For backwards compatibility - get the value field
    /// Panics if not a runtime type
    pub fn value(&self) -> BasicValueEnum<'ctx> {
        self.runtime_value()
    }

    /// Load the current value from memory (for addressable runtime values)
    /// Returns self for non-addressable values or compile-time types
    pub fn load(&self, builder: &Builder<'ctx>, name: &str) -> PyValue<'ctx> {
        match &self.inner {
            PyValueInner::Runtime { value, ptr } => {
                if let Some(p) = ptr {
                    let llvm_type = value.get_type();
                    let loaded = builder.build_load(llvm_type, *p, name).unwrap();
                    Self::new(loaded, self.ty.clone(), Some(*p))
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }

    /// Store a value to this variable (only works for addressable runtime values)
    pub fn store(
        &self,
        builder: &Builder<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        match &self.inner {
            PyValueInner::Runtime { ptr, .. } => {
                if let Some(p) = ptr {
                    builder.build_store(*p, value).unwrap();
                    Ok(())
                } else {
                    Err("Cannot store to a non-addressable value".to_string())
                }
            }
            _ => Err("Cannot store to a compile-time value".to_string()),
        }
    }

    /// Store another PyValue to this variable
    pub fn store_value(
        &self,
        builder: &Builder<'ctx>,
        value: &PyValue<'ctx>,
    ) -> Result<(), String> {
        self.store(builder, value.runtime_value())
    }

    /// Check if two PyValues have the same type
    pub fn same_type(&self, other: &PyValue<'ctx>) -> bool {
        std::mem::discriminant(&self.ty) == std::mem::discriminant(&other.ty)
    }

    // ========================================================================
    // Binary Operations - dispatches to type-specific modules
    // ========================================================================

    /// Perform a binary operation: self op rhs
    pub fn binary_op(
        &self,
        cg: &CgCtx<'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &self.ty {
            PyType::Int => super::int_ops::binary_op(self, cg, op, rhs),
            PyType::Float => super::float_ops::binary_op(self, cg, op, rhs),
            PyType::Bool => super::bool_ops::binary_op(self, cg, op, rhs),
            PyType::Str => super::str_ops::binary_op(self, cg, op, rhs),
            PyType::Bytes => super::bytes_ops::binary_op(self, cg, op, rhs),
            PyType::None => super::none_ops::binary_op(self, cg, op, rhs),
            PyType::List(_) => super::list_ops::binary_op(self, cg, op, rhs),
            PyType::Dict(_, _) => super::dict_ops::binary_op(self, cg, op, rhs),
            PyType::Set(_) => super::set_ops::binary_op(self, cg, op, rhs),
            PyType::Tuple(_) => panic!("Binary operations not supported on tuples"),
            PyType::Function => panic!("Binary operations not supported on functions"),
            PyType::Module => panic!("Binary operations not supported on modules"),
            PyType::Macro => panic!("Binary operations not supported on macros"),
        }
    }

    // ========================================================================
    // Unary Operations - dispatches to type-specific modules
    // ========================================================================

    /// Perform a unary operation on this value
    pub fn unary_op(&self, cg: &CgCtx<'ctx>, op: &UnaryOp) -> Result<PyValue<'ctx>, String> {
        match &self.ty {
            PyType::Int => super::int_ops::unary_op(self, cg, op),
            PyType::Float => super::float_ops::unary_op(self, cg, op),
            PyType::Bool => super::bool_ops::unary_op(self, cg, op),
            PyType::Str => super::str_ops::unary_op(self, cg, op),
            PyType::Bytes => super::bytes_ops::unary_op(self, cg, op),
            PyType::None => super::none_ops::unary_op(self, cg, op),
            PyType::List(_) => super::list_ops::unary_op(self, cg, op),
            PyType::Dict(_, _) => super::dict_ops::unary_op(self, cg, op),
            PyType::Set(_) => super::set_ops::unary_op(self, cg, op),
            PyType::Tuple(_) => panic!("Unary operator {:?} not supported on tuple", op),
            PyType::Function => panic!("Unary operations not supported on functions"),
            PyType::Module => panic!("Unary operations not supported on modules"),
            PyType::Macro => panic!("Unary operations not supported on macros"),
        }
    }

    // ========================================================================
    // Module/Function specific operations
    // ========================================================================

    /// Get a member from a module by name
    pub fn get_member(&self, name: &str) -> Result<PyValue<'ctx>, String> {
        match &self.inner {
            PyValueInner::Module(info) => info
                .members
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Module '{}' has no member '{}'", info.name, name)),
            _ => panic!("get_member called on non-module {:?}", self.ty),
        }
    }

    /// Add a member to a module
    pub fn add_member(&mut self, name: String, value: PyValue<'ctx>) -> Result<(), String> {
        match &mut self.inner {
            PyValueInner::Module(info) => {
                info.members.insert(name, value);
                Ok(())
            }
            _ => panic!("add_member called on non-module {:?}", self.ty),
        }
    }

    /// Get the LLVM FunctionValue for calling (function types only)
    pub fn get_function(&self) -> FunctionInfo<'ctx> {
        match &self.inner {
            PyValueInner::Function(info) => info.clone(),
            _ => panic!("get_function called on non-function {:?}", self.ty),
        }
    }

    /// Get the macro kind (macro types only)
    pub fn get_macro_kind(&self) -> MacroKind {
        match &self.inner {
            PyValueInner::Macro(kind) => *kind,
            _ => panic!("get_macro_kind called on non-macro {:?}", self.ty),
        }
    }
}
