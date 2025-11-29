//! PyValue - Python value with type information
//!
//! This module provides the core PyValue struct that combines
//! an LLVM value with its Python type information.

use crate::ast::{BinaryOp, Type, UnaryOp};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

use super::traits::{BitNegatable, LogicalNegatable, Negatable, Printable, ToBool, UnaryPlusable};

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

/// Python type enum - represents the type without an LLVM value
#[derive(Clone, Debug, PartialEq)]
pub enum PyType {
    Int,
    Float,
    Bool,
    Bytes,
    None,
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
            Type::List(_) => Err("List type not yet implemented".to_string()),
            Type::Dict(_, _) => Err("Dict type not yet implemented".to_string()),
            Type::Set(_) => Err("Set type not yet implemented".to_string()),
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
        }
    }
}

/// A Python value paired with its type information.
/// This combines an LLVM IR value with its Python-level type.
///
/// Values can optionally have an address (pointer to stack allocation).
/// - With address: lvalue that can be loaded/stored (variables)
/// - Without address: rvalue/temporary (literals, expression results)
#[derive(Clone)]
pub struct PyValue<'ctx> {
    pub value: BasicValueEnum<'ctx>,
    pub ty: PyType,
    /// Optional pointer for addressable values (variables)
    ptr: Option<PointerValue<'ctx>>,
    /// LLVM type for loading (needed when ptr is Some)
    llvm_type: Option<BasicTypeEnum<'ctx>>,
}

impl<'ctx> PyValue<'ctx> {
    /// Create a new Python value (rvalue - no address)
    pub fn new(value: BasicValueEnum<'ctx>, ty: PyType) -> Self {
        Self {
            value,
            ty,
            ptr: None,
            llvm_type: None,
        }
    }

    /// Create a new addressable Python value (lvalue - has address)
    pub fn new_addressable(
        value: BasicValueEnum<'ctx>,
        ty: PyType,
        ptr: PointerValue<'ctx>,
        llvm_type: BasicTypeEnum<'ctx>,
    ) -> Self {
        Self {
            value,
            ty,
            ptr: Some(ptr),
            llvm_type: Some(llvm_type),
        }
    }

    /// Create a Python int value
    pub fn int(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Int)
    }

    /// Create a Python float value
    pub fn float(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Float)
    }

    /// Create a Python bool value
    pub fn bool(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Bool)
    }

    /// Create a Python bytes value
    pub fn bytes(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::Bytes)
    }

    /// Create a Python none value
    pub fn none(value: BasicValueEnum<'ctx>) -> Self {
        Self::new(value, PyType::None)
    }

    /// Create a PyValue from an AST Type and an LLVM value
    pub fn from_ast_type(ty: &Type, value: BasicValueEnum<'ctx>) -> Result<Self, String> {
        Ok(Self::new(value, PyType::from_ast_type(ty)?))
    }

    /// Create an addressable PyValue from an AST Type
    pub fn from_ast_type_addressable(
        ty: &Type,
        value: BasicValueEnum<'ctx>,
        ptr: PointerValue<'ctx>,
        llvm_type: BasicTypeEnum<'ctx>,
    ) -> Result<Self, String> {
        Ok(Self::new_addressable(
            value,
            PyType::from_ast_type(ty)?,
            ptr,
            llvm_type,
        ))
    }

    /// Check if this value has an address (is an lvalue)
    pub fn has_address(&self) -> bool {
        self.ptr.is_some()
    }

    /// Get the pointer if this value has an address
    pub fn ptr(&self) -> Option<PointerValue<'ctx>> {
        self.ptr
    }

    /// Load the current value from memory (for addressable values)
    /// Returns self.value for non-addressable values
    pub fn load(&self, builder: &Builder<'ctx>, name: &str) -> PyValue<'ctx> {
        if let (Some(ptr), Some(llvm_type)) = (self.ptr, self.llvm_type) {
            let value = builder.build_load(llvm_type, ptr, name).unwrap();
            Self::new_addressable(value, self.ty.clone(), ptr, llvm_type)
        } else {
            self.clone()
        }
    }

    /// Store a value to this variable (only works for addressable values)
    pub fn store(
        &self,
        builder: &Builder<'ctx>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        if let Some(ptr) = self.ptr {
            builder.build_store(ptr, value).unwrap();
            Ok(())
        } else {
            Err("Cannot store to a non-addressable value".to_string())
        }
    }

    /// Store another PyValue to this variable
    pub fn store_value(
        &self,
        builder: &Builder<'ctx>,
        value: &PyValue<'ctx>,
    ) -> Result<(), String> {
        self.store(builder, value.value)
    }

    /// Check if two PyValues have the same type
    pub fn same_type(&self, other: &PyValue<'ctx>) -> bool {
        std::mem::discriminant(&self.ty) == std::mem::discriminant(&other.ty)
    }

    /// Perform a binary operation: self op rhs
    /// Dispatches to the appropriate type's dispatch function
    pub fn binary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &self.ty {
            PyType::Int => super::int::int_binary_op(cg, op, self.value, rhs),
            PyType::Float => super::float::float_binary_op(cg, op, self.value, rhs),
            PyType::Bool => super::bool::bool_binary_op(cg, op, self.value, rhs),
            PyType::Bytes => super::bytes::bytes_binary_op(cg, op, self.value, rhs),
            PyType::None => super::none::none_binary_op(cg, op, self.value, rhs),
        }
    }

    /// Perform a unary operation on this value
    /// Dispatches to the appropriate type wrapper's trait implementation
    pub fn unary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match &self.ty {
            PyType::Int => {
                let wrapper = super::int::PyInt::from_basic(self.value);
                match op {
                    UnaryOp::Neg => wrapper.neg(cg),
                    UnaryOp::Pos => wrapper.pos(cg),
                    UnaryOp::Not => wrapper.logical_not(cg),
                    UnaryOp::BitNot => wrapper.bitnot(cg),
                }
            }
            PyType::Float => {
                let wrapper = super::float::PyFloat::from_basic(self.value);
                match op {
                    UnaryOp::Neg => wrapper.neg(cg),
                    UnaryOp::Pos => wrapper.pos(cg),
                    UnaryOp::Not | UnaryOp::BitNot => {
                        Err(format!("Operator {:?} not supported on floats", op))
                    }
                }
            }
            PyType::Bool => {
                let wrapper = super::bool::PyBool::from_basic(self.value);
                match op {
                    UnaryOp::Not => wrapper.logical_not(cg),
                    _ => Err(format!("Operator {:?} not supported on bools", op)),
                }
            }
            PyType::Bytes => Err(format!("Unary operator {:?} not supported on bytes", op)),
            PyType::None => Err(format!("Unary operator {:?} not supported on None", op)),
        }
    }

    /// Get the print function name for this value's type
    /// Dispatches to the appropriate type wrapper's Printable trait implementation
    pub fn print_function_name(&self) -> &'static str {
        match &self.ty {
            PyType::Int => super::int::PyInt::from_basic(self.value).print_function_name(),
            PyType::Float => super::float::PyFloat::from_basic(self.value).print_function_name(),
            PyType::Bool => super::bool::PyBool::from_basic(self.value).print_function_name(),
            PyType::Bytes => super::bytes::PyBytes::from_basic(self.value).print_function_name(),
            PyType::None => super::none::PyNone::from_basic(self.value).print_function_name(),
        }
    }

    /// Generate a print call for this value
    /// Dispatches to the appropriate type wrapper's Printable trait implementation
    pub fn print(
        &self,
        builder: &Builder<'ctx>,
        print_fn: FunctionValue<'ctx>,
    ) -> Result<(), String> {
        match &self.ty {
            PyType::Int => {
                let wrapper = super::int::PyInt::from_basic(self.value);
                wrapper.print(builder, print_fn)
            }
            PyType::Float => {
                let wrapper = super::float::PyFloat::from_basic(self.value);
                wrapper.print(builder, print_fn)
            }
            PyType::Bool => {
                let wrapper = super::bool::PyBool::from_basic(self.value);
                wrapper.print(builder, print_fn)
            }
            PyType::Bytes => {
                let wrapper = super::bytes::PyBytes::from_basic(self.value);
                wrapper.print(builder, print_fn)
            }
            PyType::None => {
                let wrapper = super::none::PyNone::from_basic(self.value);
                wrapper.print(builder, print_fn)
            }
        }
    }

    /// Convert this value to a boolean (truthiness test)
    /// Dispatches to the appropriate type wrapper's ToBool trait implementation
    pub fn to_bool<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        match &self.ty {
            PyType::Int => {
                let wrapper = super::int::PyInt::from_basic(self.value);
                wrapper.to_bool(cg)
            }
            PyType::Float => {
                let wrapper = super::float::PyFloat::from_basic(self.value);
                wrapper.to_bool(cg)
            }
            PyType::Bool => {
                let wrapper = super::bool::PyBool::from_basic(self.value);
                wrapper.to_bool(cg)
            }
            PyType::Bytes => {
                let wrapper = super::bytes::PyBytes::from_basic(self.value);
                wrapper.to_bool(cg)
            }
            PyType::None => {
                let wrapper = super::none::PyNone::from_basic(self.value);
                wrapper.to_bool(cg)
            }
        }
    }
}

impl std::fmt::Debug for PyValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_address() {
            write!(f, "PyValue::{}(lvalue)", self.ty.type_name())
        } else {
            write!(f, "PyValue::{}", self.ty.type_name())
        }
    }
}
