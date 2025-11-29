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
/// This reduces parameter passing in codegen operations.
pub struct CgCtx<'a, 'ctx> {
    pub ctx: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
}

impl<'a, 'ctx> CgCtx<'a, 'ctx> {
    pub fn new(ctx: &'ctx Context, builder: &'a Builder<'ctx>, module: &'a Module<'ctx>) -> Self {
        Self {
            ctx,
            builder,
            module,
        }
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
            .map(|p| Self::pytype_to_llvm(context, p).into())
            .collect();

        // Build function type
        let fn_type = match return_type {
            PyType::None => context.void_type().fn_type(&llvm_param_types, false),
            _ => {
                let ret_type = Self::pytype_to_llvm(context, &return_type);
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
            .map(|t| Self::pytype_to_llvm(context, t).into())
            .collect();

        // Build function type
        let fn_type = match &self.return_type {
            PyType::None => context.void_type().fn_type(&llvm_param_types, false),
            _ => {
                let ret_type = Self::pytype_to_llvm(context, &self.return_type);
                ret_type.fn_type(&llvm_param_types, false)
            }
        };

        module.add_function(&self.mangled_name, fn_type, None)
    }

    /// Convert PyType to LLVM BasicTypeEnum
    fn pytype_to_llvm(
        context: &'ctx inkwell::context::Context,
        ty: &PyType,
    ) -> inkwell::types::BasicTypeEnum<'ctx> {
        match ty {
            PyType::Int => context.i64_type().into(),
            PyType::Float => context.f64_type().into(),
            PyType::Bool => context.bool_type().into(),
            PyType::Bytes => context.ptr_type(inkwell::AddressSpace::default()).into(),
            PyType::None => context.i32_type().into(), // void represented as i32 for now
            // Container types are all pointers to heap-allocated structs
            PyType::List(_) => context.ptr_type(inkwell::AddressSpace::default()).into(),
            PyType::Dict(_, _) => context.ptr_type(inkwell::AddressSpace::default()).into(),
            PyType::Set(_) => context.ptr_type(inkwell::AddressSpace::default()).into(),
            _ => context.i64_type().into(), // fallback for Function/Module
        }
    }
}

/// Module metadata containing its members (functions and submodules)
#[derive(Clone, Debug)]
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
}

/// Python type enum - represents the type without an LLVM value
#[derive(Clone, Debug, PartialEq)]
pub enum PyType {
    // Runtime types (have BasicValueEnum)
    Int,
    Float,
    Bool,
    Bytes,
    None,
    // Container types (heap-allocated, pointer values)
    List(Box<PyType>),              // Element type
    Dict(Box<PyType>, Box<PyType>), // Key type, Value type
    Set(Box<PyType>),               // Element type
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
            Type::None => Ok(PyType::None),
            Type::Str => Err("Str type not yet implemented (use Bytes)".to_string()),
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

    /// Get a debug representation of the type
    pub fn type_name(&self) -> &'static str {
        match self {
            PyType::Int => "Int",
            PyType::Float => "Float",
            PyType::Bool => "Bool",
            PyType::Bytes => "Bytes",
            PyType::None => "None",
            PyType::List(_) => "List",
            PyType::Dict(_, _) => "Dict",
            PyType::Set(_) => "Set",
            PyType::Function => "Function",
            PyType::Module => "Module",
            PyType::Macro => "Macro",
        }
    }

    /// Check if this is a runtime type (has BasicValueEnum)
    pub fn is_runtime(&self) -> bool {
        matches!(
            self,
            PyType::Int
                | PyType::Float
                | PyType::Bool
                | PyType::Bytes
                | PyType::None
                | PyType::List(_)
                | PyType::Dict(_, _)
                | PyType::Set(_)
        )
    }

    /// Check if this is a compile-time type (Module, Function, or Macro)
    pub fn is_compile_time(&self) -> bool {
        matches!(self, PyType::Function | PyType::Module | PyType::Macro)
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

    /// Check if this value has an address (is an lvalue) - only for runtime types
    pub fn has_address(&self) -> bool {
        match &self.inner {
            PyValueInner::Runtime { ptr, .. } => ptr.is_some(),
            _ => false,
        }
    }

    /// Get the pointer if this value has an address (runtime types only)
    pub fn ptr(&self) -> Option<PointerValue<'ctx>> {
        match &self.inner {
            PyValueInner::Runtime { ptr, .. } => *ptr,
            _ => None,
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
    pub fn binary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &self.ty {
            PyType::Int => super::int_ops::binary_op(self, cg, op, rhs),
            PyType::Float => super::float_ops::binary_op(self, cg, op, rhs),
            PyType::Bool => super::bool_ops::binary_op(self, cg, op, rhs),
            PyType::Bytes => super::bytes_ops::binary_op(self, cg, op, rhs),
            PyType::None => super::none_ops::binary_op(self, cg, op, rhs),
            PyType::List(_) => super::list_ops::binary_op(self, cg, op, rhs),
            PyType::Dict(_, _) => super::dict_ops::binary_op(self, cg, op, rhs),
            PyType::Set(_) => super::set_ops::binary_op(self, cg, op, rhs),
            PyType::Function => Err("Binary operations not supported on functions".to_string()),
            PyType::Module => Err("Binary operations not supported on modules".to_string()),
            PyType::Macro => Err("Binary operations not supported on macros".to_string()),
        }
    }

    // ========================================================================
    // Unary Operations - dispatches to type-specific modules
    // ========================================================================

    /// Perform a unary operation on this value
    pub fn unary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match &self.ty {
            PyType::Int => super::int_ops::unary_op(self, cg, op),
            PyType::Float => super::float_ops::unary_op(self, cg, op),
            PyType::Bool => super::bool_ops::unary_op(self, cg, op),
            PyType::Bytes => super::bytes_ops::unary_op(self, cg, op),
            PyType::None => super::none_ops::unary_op(self, cg, op),
            PyType::List(_) => super::list_ops::unary_op(self, cg, op),
            PyType::Dict(_, _) => super::dict_ops::unary_op(self, cg, op),
            PyType::Set(_) => super::set_ops::unary_op(self, cg, op),
            PyType::Function => Err("Unary operations not supported on functions".to_string()),
            PyType::Module => Err("Unary operations not supported on modules".to_string()),
            PyType::Macro => Err("Unary operations not supported on macros".to_string()),
        }
    }

    // ========================================================================
    // Print Operations
    // ========================================================================

    /// Get the print function name for this value's type
    pub fn print_function_name(&self) -> &'static str {
        match &self.ty {
            PyType::Int => "print_int",
            PyType::Float => "print_float",
            PyType::Bool => "print_bool",
            PyType::Bytes => "print_bytes",
            PyType::None => "print_none",
            PyType::List(_) => "print_list",
            PyType::Dict(_, _) => "print_dict",
            PyType::Set(_) => "print_set",
            PyType::Function => "print_function",
            PyType::Module => "print_module",
            PyType::Macro => "print_macro",
        }
    }

    /// Generate a print call for this value
    pub fn print(
        &self,
        builder: &Builder<'ctx>,
        print_fn: FunctionValue<'ctx>,
    ) -> Result<(), String> {
        match &self.ty {
            PyType::Int => {
                builder
                    .build_call(print_fn, &[self.runtime_value().into()], "print_int")
                    .unwrap();
            }
            PyType::Float => {
                builder
                    .build_call(print_fn, &[self.runtime_value().into()], "print_float")
                    .unwrap();
            }
            PyType::Bool => {
                builder
                    .build_call(print_fn, &[self.runtime_value().into()], "print_bool")
                    .unwrap();
            }
            PyType::Bytes => {
                builder
                    .build_call(print_fn, &[self.runtime_value().into()], "print_bytes")
                    .unwrap();
            }
            PyType::None => {
                builder.build_call(print_fn, &[], "print_none").unwrap();
            }
            PyType::List(_) => {
                builder
                    .build_call(print_fn, &[self.runtime_value().into()], "print_list")
                    .unwrap();
            }
            PyType::Dict(_, _) => {
                builder
                    .build_call(print_fn, &[self.runtime_value().into()], "print_dict")
                    .unwrap();
            }
            PyType::Set(_) => {
                builder
                    .build_call(print_fn, &[self.runtime_value().into()], "print_set")
                    .unwrap();
            }
            PyType::Function => {
                return Err("Cannot print function".to_string());
            }
            PyType::Module => {
                return Err("Cannot print module".to_string());
            }
            PyType::Macro => {
                return Err("Cannot print macro".to_string());
            }
        }
        Ok(())
    }

    // ========================================================================
    // Type Conversion
    // ========================================================================

    /// Convert this value to a boolean (truthiness test)
    pub fn to_bool<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        use inkwell::{FloatPredicate, IntPredicate};

        match &self.ty {
            PyType::Int => {
                let int_val = self.runtime_value().into_int_value();
                let zero = cg.ctx.i64_type().const_zero();
                Ok(cg
                    .builder
                    .build_int_compare(IntPredicate::NE, int_val, zero, "int_to_bool")
                    .unwrap()
                    .into())
            }
            PyType::Float => {
                let float_val = self.runtime_value().into_float_value();
                let zero = cg.ctx.f64_type().const_zero();
                Ok(cg
                    .builder
                    .build_float_compare(FloatPredicate::ONE, float_val, zero, "float_to_bool")
                    .unwrap()
                    .into())
            }
            PyType::Bool => Ok(self.runtime_value()),
            PyType::Bytes => {
                let ptr_val = self.runtime_value().into_pointer_value();
                let strlen_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "strlen_bytes");
                let call_site = cg
                    .builder
                    .build_call(strlen_fn, &[ptr_val.into()], "strlen")
                    .unwrap();
                let len = super::extract_int_result(call_site, "strlen_bytes")?;
                let zero = cg.ctx.i64_type().const_zero();
                Ok(cg
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        len.into_int_value(),
                        zero,
                        "bytes_to_bool",
                    )
                    .unwrap()
                    .into())
            }
            PyType::None => {
                // None is always falsy
                Ok(cg.ctx.bool_type().const_zero().into())
            }
            PyType::List(_) => {
                // Empty list is falsy, non-empty is truthy
                let ptr_val = self.runtime_value().into_pointer_value();
                let len_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "list_len");
                let call_site = cg
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "list_len")
                    .unwrap();
                let len = super::extract_int_result(call_site, "list_len")?;
                let zero = cg.ctx.i64_type().const_zero();
                Ok(cg
                    .builder
                    .build_int_compare(IntPredicate::NE, len.into_int_value(), zero, "list_to_bool")
                    .unwrap()
                    .into())
            }
            PyType::Dict(_, _) => {
                // Empty dict is falsy, non-empty is truthy
                let ptr_val = self.runtime_value().into_pointer_value();
                let len_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "dict_len");
                let call_site = cg
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "dict_len")
                    .unwrap();
                let len = super::extract_int_result(call_site, "dict_len")?;
                let zero = cg.ctx.i64_type().const_zero();
                Ok(cg
                    .builder
                    .build_int_compare(IntPredicate::NE, len.into_int_value(), zero, "dict_to_bool")
                    .unwrap()
                    .into())
            }
            PyType::Set(_) => {
                // Empty set is falsy, non-empty is truthy
                let ptr_val = self.runtime_value().into_pointer_value();
                let len_fn = super::get_or_declare_builtin(cg.module, cg.ctx, "set_len");
                let call_site = cg
                    .builder
                    .build_call(len_fn, &[ptr_val.into()], "set_len")
                    .unwrap();
                let len = super::extract_int_result(call_site, "set_len")?;
                let zero = cg.ctx.i64_type().const_zero();
                Ok(cg
                    .builder
                    .build_int_compare(IntPredicate::NE, len.into_int_value(), zero, "set_to_bool")
                    .unwrap()
                    .into())
            }
            PyType::Function => {
                // Functions are always truthy
                Ok(cg.ctx.bool_type().const_int(1, false).into())
            }
            PyType::Module => {
                // Modules are always truthy
                Ok(cg.ctx.bool_type().const_int(1, false).into())
            }
            PyType::Macro => {
                // Macros are always truthy
                Ok(cg.ctx.bool_type().const_int(1, false).into())
            }
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
            _ => Err(format!("Cannot get member from {:?}", self.ty)),
        }
    }

    /// Add a member to a module
    pub fn add_member(&mut self, name: String, value: PyValue<'ctx>) -> Result<(), String> {
        match &mut self.inner {
            PyValueInner::Module(info) => {
                info.members.insert(name, value);
                Ok(())
            }
            _ => Err(format!("Cannot add member to {:?}", self.ty)),
        }
    }

    /// Get the LLVM FunctionValue for calling (function types only)
    pub fn get_function(&self) -> Result<FunctionInfo<'ctx>, String> {
        match &self.inner {
            PyValueInner::Function(info) => Ok(info.clone()),
            _ => Err(format!("Expected function, got {:?}", self.ty)),
        }
    }

    /// Get the macro kind (macro types only)
    pub fn get_macro_kind(&self) -> Result<MacroKind, String> {
        match &self.inner {
            PyValueInner::Macro(kind) => Ok(*kind),
            _ => Err(format!("Expected macro, got {:?}", self.ty)),
        }
    }
}

impl std::fmt::Debug for PyValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            PyValueInner::Runtime { ptr, .. } => {
                if ptr.is_some() {
                    write!(f, "PyValue::{}(lvalue)", self.ty.type_name())
                } else {
                    write!(f, "PyValue::{}", self.ty.type_name())
                }
            }
            PyValueInner::Function(info) => {
                write!(f, "PyValue::Function({})", info.mangled_name)
            }
            PyValueInner::Module(info) => {
                write!(f, "PyValue::Module({})", info.name)
            }
            PyValueInner::Macro(kind) => {
                write!(f, "PyValue::Macro({:?})", kind)
            }
        }
    }
}
