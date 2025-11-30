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
    // Scalar types with specific LLVM types
    Int(IntValue<'ctx>, Option<PointerValue<'ctx>>),
    Float(FloatValue<'ctx>, Option<PointerValue<'ctx>>),
    Bool(IntValue<'ctx>, Option<PointerValue<'ctx>>), // i1
    Str(PointerValue<'ctx>, Option<PointerValue<'ctx>>),
    Bytes(PointerValue<'ctx>, Option<PointerValue<'ctx>>),
    None(IntValue<'ctx>, Option<PointerValue<'ctx>>), // i32
    // Container types with element type info
    List(PointerValue<'ctx>, Option<PointerValue<'ctx>>, Box<PyType>),
    Dict(
        PointerValue<'ctx>,
        Option<PointerValue<'ctx>>,
        Box<PyType>,
        Box<PyType>,
    ),
    Set(PointerValue<'ctx>, Option<PointerValue<'ctx>>, Box<PyType>),
    Tuple(PointerValue<'ctx>, Option<PointerValue<'ctx>>, Box<PyType>),
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
        PyValue::Int(value, None)
    }

    pub fn float(value: FloatValue<'ctx>) -> Self {
        PyValue::Float(value, None)
    }

    pub fn bool(value: IntValue<'ctx>) -> Self {
        PyValue::Bool(value, None)
    }

    pub fn new_str(value: PointerValue<'ctx>) -> Self {
        PyValue::Str(value, None)
    }

    pub fn bytes(value: PointerValue<'ctx>) -> Self {
        PyValue::Bytes(value, None)
    }

    pub fn none(value: IntValue<'ctx>) -> Self {
        PyValue::None(value, None)
    }

    pub fn list(value: PointerValue<'ctx>, elem_type: PyType) -> Self {
        PyValue::List(value, None, Box::new(elem_type))
    }

    pub fn dict(value: PointerValue<'ctx>, key_type: PyType, val_type: PyType) -> Self {
        PyValue::Dict(value, None, Box::new(key_type), Box::new(val_type))
    }

    pub fn set(value: PointerValue<'ctx>, elem_type: PyType) -> Self {
        PyValue::Set(value, None, Box::new(elem_type))
    }

    pub fn tuple(value: PointerValue<'ctx>, elem_type: PyType) -> Self {
        PyValue::Tuple(value, None, Box::new(elem_type))
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
        match ty {
            PyType::Int => PyValue::Int(value.into_int_value(), ptr),
            PyType::Float => PyValue::Float(value.into_float_value(), ptr),
            PyType::Bool => PyValue::Bool(value.into_int_value(), ptr),
            PyType::Str => PyValue::Str(value.into_pointer_value(), ptr),
            PyType::Bytes => PyValue::Bytes(value.into_pointer_value(), ptr),
            PyType::None => PyValue::None(value.into_int_value(), ptr),
            PyType::List(elem) => PyValue::List(value.into_pointer_value(), ptr, elem),
            PyType::Dict(k, v) => PyValue::Dict(value.into_pointer_value(), ptr, k, v),
            PyType::Set(elem) => PyValue::Set(value.into_pointer_value(), ptr, elem),
            PyType::Tuple(elem) => PyValue::Tuple(value.into_pointer_value(), ptr, elem),
            PyType::Function | PyType::Module | PyType::Macro => {
                panic!("Use specific constructors for Function/Module/Macro")
            }
        }
    }

    /// Create from AST Type
    pub fn from_ast_type(
        ty: &Type,
        value: BasicValueEnum<'ctx>,
        ptr: Option<PointerValue<'ctx>>,
    ) -> Result<Self, String> {
        let pytype = PyType::from_ast_type(ty)?;
        Ok(Self::new(value, pytype, ptr))
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Get the type tag
    pub fn ty(&self) -> PyType {
        match self {
            PyValue::Int(_, _) => PyType::Int,
            PyValue::Float(_, _) => PyType::Float,
            PyValue::Bool(_, _) => PyType::Bool,
            PyValue::Str(_, _) => PyType::Str,
            PyValue::Bytes(_, _) => PyType::Bytes,
            PyValue::None(_, _) => PyType::None,
            PyValue::List(_, _, elem) => PyType::List(elem.clone()),
            PyValue::Dict(_, _, k, v) => PyType::Dict(k.clone(), v.clone()),
            PyValue::Set(_, _, elem) => PyType::Set(elem.clone()),
            PyValue::Tuple(_, _, elem) => PyType::Tuple(elem.clone()),
            PyValue::Function(_) => PyType::Function,
            PyValue::Module(_) => PyType::Module,
            PyValue::Macro(_) => PyType::Macro,
        }
    }

    /// Get the LLVM value as BasicValueEnum (for compatibility)
    pub fn value(&self) -> BasicValueEnum<'ctx> {
        match self {
            PyValue::Int(v, _) => (*v).into(),
            PyValue::Float(v, _) => (*v).into(),
            PyValue::Bool(v, _) => (*v).into(),
            PyValue::Str(v, _) => (*v).into(),
            PyValue::Bytes(v, _) => (*v).into(),
            PyValue::None(v, _) => (*v).into(),
            PyValue::List(v, _, _) => (*v).into(),
            PyValue::Dict(v, _, _, _) => (*v).into(),
            PyValue::Set(v, _, _) => (*v).into(),
            PyValue::Tuple(v, _, _) => (*v).into(),
            PyValue::Function(f) => f.function.as_global_value().as_pointer_value().into(),
            PyValue::Module(_) => panic!("Module has no LLVM value"),
            PyValue::Macro(_) => panic!("Macro has no LLVM value"),
        }
    }

    /// Get the pointer (for addressable values)
    pub fn ptr(&self) -> Option<PointerValue<'ctx>> {
        match self {
            PyValue::Int(_, p) | PyValue::Float(_, p) | PyValue::Bool(_, p) => *p,
            PyValue::Str(_, p) | PyValue::Bytes(_, p) | PyValue::None(_, p) => *p,
            PyValue::List(_, p, _) | PyValue::Set(_, p, _) | PyValue::Tuple(_, p, _) => *p,
            PyValue::Dict(_, p, _, _) => *p,
            _ => None,
        }
    }

    /// Get IntValue (for Int, Bool, None)
    pub fn int_value(&self) -> IntValue<'ctx> {
        match self {
            PyValue::Int(v, _) | PyValue::Bool(v, _) | PyValue::None(v, _) => *v,
            _ => panic!("int_value called on non-int type: {:?}", self.ty()),
        }
    }

    /// Get FloatValue (for Float)
    pub fn float_value(&self) -> FloatValue<'ctx> {
        match self {
            PyValue::Float(v, _) => *v,
            _ => panic!("float_value called on non-float type"),
        }
    }

    /// Get PointerValue (for Str, Bytes, containers)
    pub fn ptr_value(&self) -> PointerValue<'ctx> {
        match self {
            PyValue::Str(v, _) | PyValue::Bytes(v, _) => *v,
            PyValue::List(v, _, _) | PyValue::Set(v, _, _) | PyValue::Tuple(v, _, _) => *v,
            PyValue::Dict(v, _, _, _) => *v,
            _ => panic!("ptr_value called on non-pointer type: {:?}", self.ty()),
        }
    }

    // For backwards compatibility
    pub fn runtime_value(&self) -> BasicValueEnum<'ctx> {
        self.value()
    }

    /// Get element type for containers
    pub fn elem_type(&self) -> &PyType {
        match self {
            PyValue::List(_, _, elem) | PyValue::Set(_, _, elem) | PyValue::Tuple(_, _, elem) => {
                elem
            }
            _ => panic!("elem_type called on non-container"),
        }
    }

    /// Get key type for Dict
    pub fn key_type(&self) -> &PyType {
        match self {
            PyValue::Dict(_, _, k, _) => k,
            _ => panic!("key_type called on non-dict"),
        }
    }

    /// Get value type for Dict
    pub fn val_type(&self) -> &PyType {
        match self {
            PyValue::Dict(_, _, _, v) => v,
            _ => panic!("val_type called on non-dict"),
        }
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

    /// Load the current value from memory
    pub fn load(&self, builder: &Builder<'ctx>, name: &str) -> PyValue<'ctx> {
        let ptr = match self.ptr() {
            Some(p) => p,
            None => return self.clone(),
        };

        let loaded = builder
            .build_load(self.value().get_type(), ptr, name)
            .unwrap();

        match self {
            PyValue::Int(_, _) => PyValue::Int(loaded.into_int_value(), Some(ptr)),
            PyValue::Float(_, _) => PyValue::Float(loaded.into_float_value(), Some(ptr)),
            PyValue::Bool(_, _) => PyValue::Bool(loaded.into_int_value(), Some(ptr)),
            PyValue::Str(_, _) => PyValue::Str(loaded.into_pointer_value(), Some(ptr)),
            PyValue::Bytes(_, _) => PyValue::Bytes(loaded.into_pointer_value(), Some(ptr)),
            PyValue::None(_, _) => PyValue::None(loaded.into_int_value(), Some(ptr)),
            PyValue::List(_, _, elem) => {
                PyValue::List(loaded.into_pointer_value(), Some(ptr), elem.clone())
            }
            PyValue::Dict(_, _, k, v) => {
                PyValue::Dict(loaded.into_pointer_value(), Some(ptr), k.clone(), v.clone())
            }
            PyValue::Set(_, _, elem) => {
                PyValue::Set(loaded.into_pointer_value(), Some(ptr), elem.clone())
            }
            PyValue::Tuple(_, _, elem) => {
                PyValue::Tuple(loaded.into_pointer_value(), Some(ptr), elem.clone())
            }
            _ => self.clone(),
        }
    }

    /// Store a value to this variable
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
            PyValue::Int(_, _) => super::int_ops::binary_op(self, cg, op, rhs),
            PyValue::Float(_, _) => super::float_ops::binary_op(self, cg, op, rhs),
            PyValue::Bool(_, _) => super::bool_ops::binary_op(self, cg, op, rhs),
            PyValue::Str(_, _) => super::str_ops::binary_op(self, cg, op, rhs),
            PyValue::Bytes(_, _) => super::bytes_ops::binary_op(self, cg, op, rhs),
            PyValue::None(_, _) => super::none_ops::binary_op(self, cg, op, rhs),
            PyValue::List(_, _, _) => super::list_ops::binary_op(self, cg, op, rhs),
            PyValue::Dict(_, _, _, _) => super::dict_ops::binary_op(self, cg, op, rhs),
            PyValue::Set(_, _, _) => super::set_ops::binary_op(self, cg, op, rhs),
            PyValue::Tuple(_, _, _) => Err("Binary operations not supported on tuples".to_string()),
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
            PyValue::Int(_, _) => super::int_ops::unary_op(self, cg, op),
            PyValue::Float(_, _) => super::float_ops::unary_op(self, cg, op),
            PyValue::Bool(_, _) => super::bool_ops::unary_op(self, cg, op),
            PyValue::Str(_, _) => super::str_ops::unary_op(self, cg, op),
            PyValue::Bytes(_, _) => super::bytes_ops::unary_op(self, cg, op),
            PyValue::None(_, _) => super::none_ops::unary_op(self, cg, op),
            PyValue::List(_, _, _) => super::list_ops::unary_op(self, cg, op),
            PyValue::Dict(_, _, _, _) => super::dict_ops::unary_op(self, cg, op),
            PyValue::Set(_, _, _) => super::set_ops::unary_op(self, cg, op),
            PyValue::Tuple(_, _, _) => {
                Err(format!("Unary operator {:?} not supported on tuple", op))
            }
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
    pub fn value_to_bool(&self, val: &PyValue<'ctx>) -> IntValue<'ctx> {
        match val {
            PyValue::Bool(v, _) => *v,
            PyValue::Int(v, _) => {
                let zero = v.get_type().const_zero();
                self.builder
                    .build_int_compare(inkwell::IntPredicate::NE, *v, zero, "to_bool")
                    .unwrap()
            }
            PyValue::Float(v, _) => {
                let zero = self.ctx.f64_type().const_zero();
                self.builder
                    .build_float_compare(inkwell::FloatPredicate::ONE, *v, zero, "to_bool")
                    .unwrap()
            }
            PyValue::None(_, _) => self.ctx.bool_type().const_zero(),
            PyValue::Str(v, _) => {
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
            PyValue::Bytes(v, _) => {
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
            PyValue::List(v, _, _) => {
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
            PyValue::Dict(v, _, _, _) => {
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
            PyValue::Set(v, _, _) => {
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
            PyValue::Tuple(_, _, _) => self.ctx.bool_type().const_int(1, false),
            PyValue::Function(_) | PyValue::Module(_) | PyValue::Macro(_) => {
                self.ctx.bool_type().const_int(1, false)
            }
        }
    }
}
