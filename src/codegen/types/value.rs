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
    Tuple(Box<PyType>),
    Function,
    Module,
    Macro,
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
            Type::List(elem) => Ok(PyType::List(Box::new(PyType::from_ast_type(elem)?))),
            Type::Dict(k, v) => Ok(PyType::Dict(
                Box::new(PyType::from_ast_type(k)?),
                Box::new(PyType::from_ast_type(v)?),
            )),
            Type::Set(elem) => Ok(PyType::Set(Box::new(PyType::from_ast_type(elem)?))),
            Type::Tuple(_) => Err("Tuple type not yet implemented".to_string()),
            Type::Custom(name) => Err(format!("Custom type '{}' not yet implemented", name)),
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
            PyType::List(_) | PyType::Dict(_, _) | PyType::Set(_) | PyType::Tuple(_) => {
                ctx.ptr_type(inkwell::AddressSpace::default()).into()
            }
            PyType::Function | PyType::Module | PyType::Macro => ctx.i64_type().into(),
        }
    }
}

// ============================================================================
// FunctionInfo, ModuleInfo, MacroKind
// ============================================================================

/// Function metadata for compile-time function references
#[derive(Clone, Debug)]
pub struct FunctionInfo<'ctx> {
    pub mangled_name: String,
    pub function: FunctionValue<'ctx>,
    pub param_types: Vec<PyType>,
    pub return_type: PyType,
    pub bound_args: Vec<BasicValueEnum<'ctx>>,
}

impl<'ctx> FunctionInfo<'ctx> {
    pub fn from_ast(
        context: &'ctx Context,
        mangled_name: &str,
        func: &crate::ast::Function,
    ) -> Self {
        use inkwell::types::{BasicMetadataTypeEnum, BasicType};

        let placeholder_module = context.create_module("__placeholder__");

        let param_types: Vec<PyType> = func
            .params
            .iter()
            .filter_map(|p| PyType::from_ast_type(&p.param_type).ok())
            .collect();

        let return_type = PyType::from_ast_type(&func.return_type).unwrap_or(PyType::None);

        let llvm_param_types: Vec<BasicMetadataTypeEnum> = param_types
            .iter()
            .map(|p| p.to_llvm(context).into())
            .collect();

        let fn_type = match return_type {
            PyType::None => context.void_type().fn_type(&llvm_param_types, false),
            _ => {
                let ret_type = return_type.to_llvm(context);
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

    pub fn declare_in_module(
        &self,
        context: &'ctx Context,
        module: &Module<'ctx>,
    ) -> FunctionValue<'ctx> {
        use inkwell::types::{BasicMetadataTypeEnum, BasicType};

        if let Some(f) = module.get_function(&self.mangled_name) {
            return f;
        }

        let llvm_param_types: Vec<BasicMetadataTypeEnum> = self
            .param_types
            .iter()
            .map(|t| t.to_llvm(context).into())
            .collect();

        let fn_type = match &self.return_type {
            PyType::None => context.void_type().fn_type(&llvm_param_types, false),
            _ => {
                let ret_type = self.return_type.to_llvm(context);
                ret_type.fn_type(&llvm_param_types, false)
            }
        };

        module.add_function(&self.mangled_name, fn_type, None)
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
    Tuple(PtrStorage<'ctx>, Box<PyType>),
    // Compile-time types
    Function(FunctionInfo<'ctx>),
    Module(ModuleInfo<'ctx>),
    Macro(MacroKind),
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

    pub fn function(info: FunctionInfo<'ctx>) -> Self {
        PyValue::Function(info)
    }

    pub fn module(info: ModuleInfo<'ctx>) -> Self {
        PyValue::Module(info)
    }

    pub fn macro_fn(kind: MacroKind) -> Self {
        PyValue::Macro(kind)
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
            (PyType::Function, _) | (PyType::Module, _) | (PyType::Macro, _) => {
                panic!("Use specific constructors for Function/Module/Macro")
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
            PyValue::Function(_) => PyType::Function,
            PyValue::Module(_) => PyType::Module,
            PyValue::Macro(_) => PyType::Macro,
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
            PyValue::Function(f) => f.function.as_global_value().as_pointer_value().into(),
            PyValue::Module(_) => panic!("Module has no LLVM value"),
            PyValue::Macro(_) => panic!("Macro has no LLVM value"),
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
            PyValue::Function(info) => info.clone(),
            _ => panic!("get_function called on non-function"),
        }
    }

    pub fn get_macro_kind(&self) -> MacroKind {
        match self {
            PyValue::Macro(kind) => *kind,
            _ => panic!("get_macro_kind called on non-macro"),
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
            PyValue::Function(_) => Err("Binary operations not supported on functions".to_string()),
            PyValue::Module(_) => Err("Binary operations not supported on modules".to_string()),
            PyValue::Macro(_) => Err("Binary operations not supported on macros".to_string()),
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
            PyValue::Function(_) => Err("Unary operations not supported on functions".to_string()),
            PyValue::Module(_) => Err("Unary operations not supported on modules".to_string()),
            PyValue::Macro(_) => Err("Unary operations not supported on macros".to_string()),
        }
    }

    // ========================================================================
    // Module Operations
    // ========================================================================

    pub fn get_member(&self, name: &str) -> Result<PyValue<'ctx>, String> {
        match self {
            PyValue::Module(info) => info
                .members
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Module '{}' has no member '{}'", info.name, name)),
            _ => Err(format!("get_member called on non-module: {:?}", self.ty())),
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
            PyValue::Function(_) | PyValue::Module(_) | PyValue::Macro(_) => {
                self.ctx.bool_type().const_int(1, false)
            }
        }
    }
}
