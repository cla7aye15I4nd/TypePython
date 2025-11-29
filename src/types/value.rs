//! PyValue - Unified Python value type with all operations
//!
//! This module provides the core PyValue struct that combines an LLVM value
//! with its Python type information and implements all operations directly.

use crate::ast::{BinaryOp, Type, UnaryOp};
use crate::codegen::builtins::BUILTIN_TABLE;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::{FloatPredicate, IntPredicate};

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
}

impl<'ctx> PyValue<'ctx> {
    /// Create a new Python value
    /// - Without ptr: rvalue (literals, expression results)
    /// - With ptr: lvalue (variables that can be stored to)
    pub fn new(value: BasicValueEnum<'ctx>, ty: PyType, ptr: Option<PointerValue<'ctx>>) -> Self {
        Self { value, ty, ptr }
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

    /// Create a PyValue from an AST Type and an LLVM value
    pub fn from_ast_type(
        ty: &Type,
        value: BasicValueEnum<'ctx>,
        ptr: Option<PointerValue<'ctx>>,
    ) -> Result<Self, String> {
        Ok(Self::new(value, PyType::from_ast_type(ty)?, ptr))
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
        if let Some(ptr) = self.ptr {
            let llvm_type = self.value.get_type();
            let value = builder.build_load(llvm_type, ptr, name).unwrap();
            Self::new(value, self.ty.clone(), Some(ptr))
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

    // ========================================================================
    // Binary Operations
    // ========================================================================

    /// Perform a binary operation: self op rhs
    pub fn binary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        match &self.ty {
            PyType::Int => self.int_binary_op(cg, op, rhs),
            PyType::Float => self.float_binary_op(cg, op, rhs),
            PyType::Bool => self.bool_binary_op(cg, op, rhs),
            PyType::Bytes => self.bytes_binary_op(cg, op, rhs),
            PyType::None => self.none_binary_op(cg, op, rhs),
        }
    }

    /// Binary operations for Int type
    fn int_binary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let lhs_int = self.value.into_int_value();

        match op {
            // Arithmetic
            BinaryOp::Add => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_int_add(lhs_int, rhs.value.into_int_value(), "add")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot add Int and {:?}", rhs.ty)),
            },
            BinaryOp::Sub => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_int_sub(lhs_int, rhs.value.into_int_value(), "sub")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot subtract {:?} from Int", rhs.ty)),
            },
            BinaryOp::Mul => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_int_mul(lhs_int, rhs.value.into_int_value(), "mul")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot multiply Int and {:?}", rhs.ty)),
            },
            BinaryOp::Div => {
                // Python 3 semantics: int / int -> float
                let lhs_float = cg
                    .builder
                    .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "lhs_itof")
                    .unwrap();
                match &rhs.ty {
                    PyType::Int => {
                        let rhs_float = cg
                            .builder
                            .build_signed_int_to_float(
                                rhs.value.into_int_value(),
                                cg.ctx.f64_type(),
                                "rhs_itof",
                            )
                            .unwrap();
                        let result = cg
                            .builder
                            .build_float_div(lhs_float, rhs_float, "fdiv")
                            .unwrap();
                        Ok(PyValue::float(result.into()))
                    }
                    PyType::Float => PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs),
                    _ => Err(format!("Cannot divide Int by {:?}", rhs.ty)),
                }
            }
            BinaryOp::FloorDiv => match &rhs.ty {
                PyType::Int => {
                    let floordiv_fn = get_or_declare_builtin(cg.module, cg.ctx, "floordiv_int");
                    let call_site = cg
                        .builder
                        .build_call(floordiv_fn, &[lhs_int.into(), rhs.value.into()], "floordiv")
                        .unwrap();
                    Ok(PyValue::int(extract_int_result(call_site, "floordiv_int")?))
                }
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot floor divide Int by {:?}", rhs.ty)),
            },
            BinaryOp::Mod => match &rhs.ty {
                PyType::Int => {
                    let mod_fn = get_or_declare_builtin(cg.module, cg.ctx, "mod_int");
                    let call_site = cg
                        .builder
                        .build_call(mod_fn, &[lhs_int.into(), rhs.value.into()], "mod")
                        .unwrap();
                    Ok(PyValue::int(extract_int_result(call_site, "mod_int")?))
                }
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot compute Int modulo {:?}", rhs.ty)),
            },
            BinaryOp::Pow => match &rhs.ty {
                PyType::Int => {
                    let pow_fn = get_or_declare_builtin(cg.module, cg.ctx, "pow_int");
                    let call_site = cg
                        .builder
                        .build_call(pow_fn, &[lhs_int.into(), rhs.value.into()], "ipow")
                        .unwrap();
                    Ok(PyValue::int(extract_int_result(call_site, "pow_int")?))
                }
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot raise Int to power {:?}", rhs.ty)),
            },

            // Bitwise
            BinaryOp::BitAnd => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_and(lhs_int, rhs.value.into_int_value(), "bitand")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                _ => Err(format!("Cannot bitwise AND Int and {:?}", rhs.ty)),
            },
            BinaryOp::BitOr => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_or(lhs_int, rhs.value.into_int_value(), "bitor")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                _ => Err(format!("Cannot bitwise OR Int and {:?}", rhs.ty)),
            },
            BinaryOp::BitXor => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_xor(lhs_int, rhs.value.into_int_value(), "bitxor")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                _ => Err(format!("Cannot bitwise XOR Int and {:?}", rhs.ty)),
            },
            BinaryOp::LShift => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_left_shift(lhs_int, rhs.value.into_int_value(), "lshift")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                _ => Err(format!("Cannot left shift Int by {:?}", rhs.ty)),
            },
            BinaryOp::RShift => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_right_shift(lhs_int, rhs.value.into_int_value(), true, "rshift")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                _ => Err(format!("Cannot right shift Int by {:?}", rhs.ty)),
            },

            // Comparison
            BinaryOp::Eq => match &rhs.ty {
                PyType::Int => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::EQ,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "eq",
                        )
                        .unwrap()
                        .into(),
                )),
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
            },
            BinaryOp::Ne => match &rhs.ty {
                PyType::Int => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::NE,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "ne",
                        )
                        .unwrap()
                        .into(),
                )),
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
            },
            BinaryOp::Lt => match &rhs.ty {
                PyType::Int => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::SLT,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "lt",
                        )
                        .unwrap()
                        .into(),
                )),
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
            },
            BinaryOp::Le => match &rhs.ty {
                PyType::Int => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::SLE,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "le",
                        )
                        .unwrap()
                        .into(),
                )),
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
            },
            BinaryOp::Gt => match &rhs.ty {
                PyType::Int => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::SGT,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "gt",
                        )
                        .unwrap()
                        .into(),
                )),
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
            },
            BinaryOp::Ge => match &rhs.ty {
                PyType::Int => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::SGE,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "ge",
                        )
                        .unwrap()
                        .into(),
                )),
                PyType::Float => {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs)
                }
                _ => Err(format!("Cannot compare Int with {:?}", rhs.ty)),
            },
            BinaryOp::Is => match &rhs.ty {
                PyType::Int => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::EQ,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "is",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot use 'is' between Int and {:?}", rhs.ty)),
            },
            BinaryOp::IsNot => match &rhs.ty {
                PyType::Int => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::NE,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "isnot",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot use 'is not' between Int and {:?}", rhs.ty)),
            },

            // Logical
            BinaryOp::And => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_and(lhs_int, rhs.value.into_int_value(), "and")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                _ => Err(format!("Cannot logical AND Int and {:?}", rhs.ty)),
            },
            BinaryOp::Or => match &rhs.ty {
                PyType::Int => {
                    let result = cg
                        .builder
                        .build_or(lhs_int, rhs.value.into_int_value(), "or")
                        .unwrap();
                    Ok(PyValue::int(result.into()))
                }
                _ => Err(format!("Cannot logical OR Int and {:?}", rhs.ty)),
            },

            BinaryOp::In | BinaryOp::NotIn => Err(format!(
                "Membership operator {:?} requires container support",
                op
            )),
        }
    }

    /// Binary operations for Float type
    fn float_binary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let lhs_float = self.value.into_float_value();

        // Helper to coerce rhs to float
        let coerce_rhs =
            |rhs: &PyValue<'ctx>| -> Result<inkwell::values::FloatValue<'ctx>, String> {
                match &rhs.ty {
                    PyType::Float => Ok(rhs.value.into_float_value()),
                    PyType::Int => Ok(cg
                        .builder
                        .build_signed_int_to_float(
                            rhs.value.into_int_value(),
                            cg.ctx.f64_type(),
                            "itof",
                        )
                        .unwrap()),
                    _ => Err(format!("Cannot coerce {:?} to float", rhs.ty)),
                }
            };

        match op {
            // Arithmetic
            BinaryOp::Add => {
                let rhs_float = coerce_rhs(rhs)?;
                let result = cg
                    .builder
                    .build_float_add(lhs_float, rhs_float, "fadd")
                    .unwrap();
                Ok(PyValue::float(result.into()))
            }
            BinaryOp::Sub => {
                let rhs_float = coerce_rhs(rhs)?;
                let result = cg
                    .builder
                    .build_float_sub(lhs_float, rhs_float, "fsub")
                    .unwrap();
                Ok(PyValue::float(result.into()))
            }
            BinaryOp::Mul => {
                let rhs_float = coerce_rhs(rhs)?;
                let result = cg
                    .builder
                    .build_float_mul(lhs_float, rhs_float, "fmul")
                    .unwrap();
                Ok(PyValue::float(result.into()))
            }
            BinaryOp::Div => {
                let rhs_float = coerce_rhs(rhs)?;
                let result = cg
                    .builder
                    .build_float_div(lhs_float, rhs_float, "fdiv")
                    .unwrap();
                Ok(PyValue::float(result.into()))
            }
            BinaryOp::FloorDiv => {
                let rhs_float = coerce_rhs(rhs)?;
                let div_result = cg
                    .builder
                    .build_float_div(lhs_float, rhs_float, "fdiv")
                    .unwrap();
                let floor_fn = get_or_declare_builtin(cg.module, cg.ctx, "floor_float");
                let call_site = cg
                    .builder
                    .build_call(floor_fn, &[div_result.into()], "floor")
                    .unwrap();
                Ok(PyValue::float(extract_float_result(
                    call_site,
                    "floor_float",
                )?))
            }
            BinaryOp::Mod => {
                let rhs_float = coerce_rhs(rhs)?;
                let fmod_fn = get_or_declare_builtin(cg.module, cg.ctx, "mod_float");
                let call_site = cg
                    .builder
                    .build_call(fmod_fn, &[lhs_float.into(), rhs_float.into()], "fmod")
                    .unwrap();
                Ok(PyValue::float(extract_float_result(
                    call_site,
                    "mod_float",
                )?))
            }
            BinaryOp::Pow => {
                let rhs_float = coerce_rhs(rhs)?;
                let pow_fn = get_or_declare_builtin(cg.module, cg.ctx, "pow_float");
                let call_site = cg
                    .builder
                    .build_call(pow_fn, &[lhs_float.into(), rhs_float.into()], "fpow")
                    .unwrap();
                Ok(PyValue::float(extract_float_result(
                    call_site,
                    "pow_float",
                )?))
            }

            // Comparison
            BinaryOp::Eq => {
                let rhs_float = coerce_rhs(rhs)?;
                Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "feq")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Ne => {
                let rhs_float = coerce_rhs(rhs)?;
                Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "fne")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Lt => {
                let rhs_float = coerce_rhs(rhs)?;
                Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OLT, lhs_float, rhs_float, "flt")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Le => {
                let rhs_float = coerce_rhs(rhs)?;
                Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OLE, lhs_float, rhs_float, "fle")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Gt => {
                let rhs_float = coerce_rhs(rhs)?;
                Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OGT, lhs_float, rhs_float, "fgt")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Ge => {
                let rhs_float = coerce_rhs(rhs)?;
                Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OGE, lhs_float, rhs_float, "fge")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::Is => {
                let rhs_float = coerce_rhs(rhs)?;
                Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::OEQ, lhs_float, rhs_float, "is")
                        .unwrap()
                        .into(),
                ))
            }
            BinaryOp::IsNot => {
                let rhs_float = coerce_rhs(rhs)?;
                Ok(PyValue::bool(
                    cg.builder
                        .build_float_compare(FloatPredicate::ONE, lhs_float, rhs_float, "isnot")
                        .unwrap()
                        .into(),
                ))
            }

            // Not supported on floats
            BinaryOp::And | BinaryOp::Or => {
                Err("Logical operators not supported on floats".to_string())
            }
            BinaryOp::BitOr
            | BinaryOp::BitXor
            | BinaryOp::BitAnd
            | BinaryOp::LShift
            | BinaryOp::RShift => Err(format!("Bitwise operator {:?} not supported on floats", op)),
            BinaryOp::In | BinaryOp::NotIn => Err(format!(
                "Membership operator {:?} requires container support",
                op
            )),
        }
    }

    /// Binary operations for Bool type
    fn bool_binary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let lhs_bool = self.value.into_int_value();

        match op {
            // For arithmetic, coerce to int first
            BinaryOp::Add
            | BinaryOp::Sub
            | BinaryOp::Mul
            | BinaryOp::Div
            | BinaryOp::FloorDiv
            | BinaryOp::Mod
            | BinaryOp::Pow
            | BinaryOp::LShift
            | BinaryOp::RShift => {
                let lhs_int = cg
                    .builder
                    .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                    .unwrap();

                // If rhs is Float, coerce to float
                if let PyType::Float = &rhs.ty {
                    let lhs_float = cg
                        .builder
                        .build_signed_int_to_float(lhs_int, cg.ctx.f64_type(), "itof")
                        .unwrap();
                    return PyValue::float(lhs_float.into()).float_binary_op(cg, op, rhs);
                }

                PyValue::int(lhs_int.into()).int_binary_op(cg, op, rhs)
            }

            // Bitwise (bool-specific)
            BinaryOp::BitAnd => match &rhs.ty {
                PyType::Bool => {
                    let result = cg
                        .builder
                        .build_and(lhs_bool, rhs.value.into_int_value(), "bitand")
                        .unwrap();
                    Ok(PyValue::bool(result.into()))
                }
                _ => Err(format!("Cannot bitwise AND Bool and {:?}", rhs.ty)),
            },
            BinaryOp::BitOr => match &rhs.ty {
                PyType::Bool => {
                    let result = cg
                        .builder
                        .build_or(lhs_bool, rhs.value.into_int_value(), "bitor")
                        .unwrap();
                    Ok(PyValue::bool(result.into()))
                }
                _ => Err(format!("Cannot bitwise OR Bool and {:?}", rhs.ty)),
            },
            BinaryOp::BitXor => match &rhs.ty {
                PyType::Bool => {
                    let result = cg
                        .builder
                        .build_xor(lhs_bool, rhs.value.into_int_value(), "bitxor")
                        .unwrap();
                    Ok(PyValue::bool(result.into()))
                }
                _ => Err(format!("Cannot bitwise XOR Bool and {:?}", rhs.ty)),
            },

            // Comparison
            BinaryOp::Eq => match &rhs.ty {
                PyType::Bool => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::EQ,
                            lhs_bool,
                            rhs.value.into_int_value(),
                            "eq",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot compare Bool with {:?}", rhs.ty)),
            },
            BinaryOp::Ne => match &rhs.ty {
                PyType::Bool => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::NE,
                            lhs_bool,
                            rhs.value.into_int_value(),
                            "ne",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot compare Bool with {:?}", rhs.ty)),
            },
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                // Coerce to int for ordering comparisons
                let lhs_int = cg
                    .builder
                    .build_int_z_extend(lhs_bool, cg.ctx.i64_type(), "btoi")
                    .unwrap();
                PyValue::int(lhs_int.into()).int_binary_op(cg, op, rhs)
            }
            BinaryOp::Is => match &rhs.ty {
                PyType::Bool => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::EQ,
                            lhs_bool,
                            rhs.value.into_int_value(),
                            "is",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot use 'is' between Bool and {:?}", rhs.ty)),
            },
            BinaryOp::IsNot => match &rhs.ty {
                PyType::Bool => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::NE,
                            lhs_bool,
                            rhs.value.into_int_value(),
                            "isnot",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot use 'is not' between Bool and {:?}", rhs.ty)),
            },

            // Logical
            BinaryOp::And => match &rhs.ty {
                PyType::Bool => {
                    let result = cg
                        .builder
                        .build_and(lhs_bool, rhs.value.into_int_value(), "and")
                        .unwrap();
                    Ok(PyValue::bool(result.into()))
                }
                _ => Err(format!("Cannot logical AND Bool and {:?}", rhs.ty)),
            },
            BinaryOp::Or => match &rhs.ty {
                PyType::Bool => {
                    let result = cg
                        .builder
                        .build_or(lhs_bool, rhs.value.into_int_value(), "or")
                        .unwrap();
                    Ok(PyValue::bool(result.into()))
                }
                _ => Err(format!("Cannot logical OR Bool and {:?}", rhs.ty)),
            },

            BinaryOp::In | BinaryOp::NotIn => Err(format!(
                "Membership operator {:?} not supported on Bool",
                op
            )),
        }
    }

    /// Binary operations for Bytes type
    fn bytes_binary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let lhs_ptr = self.value.into_pointer_value();

        match op {
            // Concatenation
            BinaryOp::Add => match &rhs.ty {
                PyType::Bytes => {
                    let strcat_fn = get_or_declare_builtin(cg.module, cg.ctx, "strcat_bytes");
                    let call_site = cg
                        .builder
                        .build_call(strcat_fn, &[lhs_ptr.into(), rhs.value.into()], "bytescat")
                        .unwrap();
                    Ok(PyValue::bytes(extract_ptr_result(
                        call_site,
                        "strcat_bytes",
                    )?))
                }
                _ => Err(format!("Cannot concatenate Bytes and {:?}", rhs.ty)),
            },

            // Repetition
            BinaryOp::Mul => match &rhs.ty {
                PyType::Int => {
                    let repeat_fn = get_or_declare_builtin(cg.module, cg.ctx, "strrepeat_bytes");
                    let call_site = cg
                        .builder
                        .build_call(
                            repeat_fn,
                            &[lhs_ptr.into(), rhs.value.into()],
                            "bytes_repeat",
                        )
                        .unwrap();
                    Ok(PyValue::bytes(extract_ptr_result(
                        call_site,
                        "strrepeat_bytes",
                    )?))
                }
                _ => Err(format!("Cannot multiply Bytes by {:?}", rhs.ty)),
            },

            // Comparison
            BinaryOp::Eq => match &rhs.ty {
                PyType::Bytes => {
                    let strcmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "strcmp_bytes");
                    let call_site = cg
                        .builder
                        .build_call(strcmp_fn, &[lhs_ptr.into(), rhs.value.into()], "bytescmp")
                        .unwrap();
                    let result = extract_int_result(call_site, "strcmp_bytes")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
            },
            BinaryOp::Ne => match &rhs.ty {
                PyType::Bytes => {
                    let strcmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "strcmp_bytes");
                    let call_site = cg
                        .builder
                        .build_call(strcmp_fn, &[lhs_ptr.into(), rhs.value.into()], "bytescmp")
                        .unwrap();
                    let result = extract_int_result(call_site, "strcmp_bytes")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    let negated = cg.builder.build_not(bool_val, "ne").unwrap();
                    Ok(PyValue::bool(negated.into()))
                }
                _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
            },
            BinaryOp::Lt => match &rhs.ty {
                PyType::Bytes => {
                    let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_lt");
                    let call_site = cg
                        .builder
                        .build_call(cmp_fn, &[lhs_ptr.into(), rhs.value.into()], "bytes_lt")
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_lt")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
            },
            BinaryOp::Le => match &rhs.ty {
                PyType::Bytes => {
                    let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_le");
                    let call_site = cg
                        .builder
                        .build_call(cmp_fn, &[lhs_ptr.into(), rhs.value.into()], "bytes_le")
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_le")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
            },
            BinaryOp::Gt => match &rhs.ty {
                PyType::Bytes => {
                    let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_gt");
                    let call_site = cg
                        .builder
                        .build_call(cmp_fn, &[lhs_ptr.into(), rhs.value.into()], "bytes_gt")
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_gt")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
            },
            BinaryOp::Ge => match &rhs.ty {
                PyType::Bytes => {
                    let cmp_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_ge");
                    let call_site = cg
                        .builder
                        .build_call(cmp_fn, &[lhs_ptr.into(), rhs.value.into()], "bytes_ge")
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_ge")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                _ => Err(format!("Cannot compare Bytes with {:?}", rhs.ty)),
            },

            // Membership
            BinaryOp::In => match &rhs.ty {
                PyType::Bytes => {
                    let rhs_ptr = rhs.value.into_pointer_value();
                    let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_contains");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs_ptr.into(), lhs_ptr.into()],
                            "bytes_contains",
                        )
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_contains")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    Ok(PyValue::bool(bool_val.into()))
                }
                _ => Err(format!("Cannot use 'in' with Bytes and {:?}", rhs.ty)),
            },
            BinaryOp::NotIn => match &rhs.ty {
                PyType::Bytes => {
                    let rhs_ptr = rhs.value.into_pointer_value();
                    let contains_fn = get_or_declare_builtin(cg.module, cg.ctx, "bytes_contains");
                    let call_site = cg
                        .builder
                        .build_call(
                            contains_fn,
                            &[rhs_ptr.into(), lhs_ptr.into()],
                            "bytes_contains",
                        )
                        .unwrap();
                    let result = extract_int_result(call_site, "bytes_contains")?;
                    let bool_val = cg
                        .builder
                        .build_int_truncate(result.into_int_value(), cg.ctx.bool_type(), "to_bool")
                        .unwrap();
                    let negated = cg.builder.build_not(bool_val, "not_in").unwrap();
                    Ok(PyValue::bool(negated.into()))
                }
                _ => Err(format!("Cannot use 'not in' with Bytes and {:?}", rhs.ty)),
            },

            _ => Err(format!("Operator {:?} not supported for bytes type", op)),
        }
    }

    /// Binary operations for None type
    fn none_binary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &BinaryOp,
        rhs: &PyValue<'ctx>,
    ) -> Result<PyValue<'ctx>, String> {
        let lhs_int = self.value.into_int_value();

        match op {
            BinaryOp::Eq => match &rhs.ty {
                PyType::None => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::EQ,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "eq",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot compare None with {:?}", rhs.ty)),
            },
            BinaryOp::Ne => match &rhs.ty {
                PyType::None => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::NE,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "ne",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot compare None with {:?}", rhs.ty)),
            },
            BinaryOp::Is => match &rhs.ty {
                PyType::None => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::EQ,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "is_none",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot use 'is' between None and {:?}", rhs.ty)),
            },
            BinaryOp::IsNot => match &rhs.ty {
                PyType::None => Ok(PyValue::bool(
                    cg.builder
                        .build_int_compare(
                            IntPredicate::NE,
                            lhs_int,
                            rhs.value.into_int_value(),
                            "isnot_none",
                        )
                        .unwrap()
                        .into(),
                )),
                _ => Err(format!("Cannot use 'is not' between None and {:?}", rhs.ty)),
            },
            _ => Err(format!("Operator {:?} not supported on None", op)),
        }
    }

    // ========================================================================
    // Unary Operations
    // ========================================================================

    /// Perform a unary operation on this value
    pub fn unary_op<'a>(
        &self,
        cg: &CgCtx<'a, 'ctx>,
        op: &UnaryOp,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        match &self.ty {
            PyType::Int => {
                let int_val = self.value.into_int_value();
                match op {
                    UnaryOp::Neg => Ok(cg.builder.build_int_neg(int_val, "neg").unwrap().into()),
                    UnaryOp::Pos => Ok(self.value),
                    UnaryOp::Not => Ok(cg.builder.build_not(int_val, "not").unwrap().into()),
                    UnaryOp::BitNot => Ok(cg.builder.build_not(int_val, "bitnot").unwrap().into()),
                }
            }
            PyType::Float => {
                let float_val = self.value.into_float_value();
                match op {
                    UnaryOp::Neg => Ok(cg
                        .builder
                        .build_float_neg(float_val, "fneg")
                        .unwrap()
                        .into()),
                    UnaryOp::Pos => Ok(self.value),
                    UnaryOp::Not | UnaryOp::BitNot => {
                        Err(format!("Operator {:?} not supported on floats", op))
                    }
                }
            }
            PyType::Bool => {
                let bool_val = self.value.into_int_value();
                match op {
                    UnaryOp::Not => Ok(cg.builder.build_not(bool_val, "not").unwrap().into()),
                    _ => Err(format!("Operator {:?} not supported on bools", op)),
                }
            }
            PyType::Bytes => Err(format!("Unary operator {:?} not supported on bytes", op)),
            PyType::None => Err(format!("Unary operator {:?} not supported on None", op)),
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
                    .build_call(print_fn, &[self.value.into()], "print_int")
                    .unwrap();
            }
            PyType::Float => {
                builder
                    .build_call(print_fn, &[self.value.into()], "print_float")
                    .unwrap();
            }
            PyType::Bool => {
                builder
                    .build_call(print_fn, &[self.value.into()], "print_bool")
                    .unwrap();
            }
            PyType::Bytes => {
                builder
                    .build_call(print_fn, &[self.value.into()], "print_bytes")
                    .unwrap();
            }
            PyType::None => {
                builder.build_call(print_fn, &[], "print_none").unwrap();
            }
        }
        Ok(())
    }

    // ========================================================================
    // Type Conversion
    // ========================================================================

    /// Convert this value to a boolean (truthiness test)
    pub fn to_bool<'a>(&self, cg: &CgCtx<'a, 'ctx>) -> Result<BasicValueEnum<'ctx>, String> {
        match &self.ty {
            PyType::Int => {
                let int_val = self.value.into_int_value();
                let zero = cg.ctx.i64_type().const_zero();
                Ok(cg
                    .builder
                    .build_int_compare(IntPredicate::NE, int_val, zero, "int_to_bool")
                    .unwrap()
                    .into())
            }
            PyType::Float => {
                let float_val = self.value.into_float_value();
                let zero = cg.ctx.f64_type().const_zero();
                Ok(cg
                    .builder
                    .build_float_compare(FloatPredicate::ONE, float_val, zero, "float_to_bool")
                    .unwrap()
                    .into())
            }
            PyType::Bool => Ok(self.value),
            PyType::Bytes => {
                let ptr_val = self.value.into_pointer_value();
                let strlen_fn = get_or_declare_builtin(cg.module, cg.ctx, "strlen_bytes");
                let call_site = cg
                    .builder
                    .build_call(strlen_fn, &[ptr_val.into()], "strlen")
                    .unwrap();
                let len = extract_int_result(call_site, "strlen_bytes")?;
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

// ============================================================================
// Helper Functions
// ============================================================================

fn get_or_declare_builtin<'ctx>(
    module: &Module<'ctx>,
    ctx: &'ctx Context,
    name: &str,
) -> FunctionValue<'ctx> {
    let builtin = BUILTIN_TABLE
        .get(name)
        .unwrap_or_else(|| panic!("Unknown builtin function: {}", name));

    if let Some(func) = module.get_function(builtin.symbol) {
        return func;
    }

    let fn_type = builtin.to_llvm_fn_type(ctx);
    module.add_function(builtin.symbol, fn_type, None)
}

fn extract_int_result<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> Result<BasicValueEnum<'ctx>, String> {
    use inkwell::values::AnyValue;
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::IntValue(iv) = any_val {
        Ok(iv.into())
    } else {
        Err(format!("{} did not return an int value", fn_name))
    }
}

fn extract_float_result<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> Result<BasicValueEnum<'ctx>, String> {
    use inkwell::values::AnyValue;
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::FloatValue(fv) = any_val {
        Ok(fv.into())
    } else {
        Err(format!("{} did not return a float value", fn_name))
    }
}

fn extract_ptr_result<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    fn_name: &str,
) -> Result<BasicValueEnum<'ctx>, String> {
    use inkwell::values::AnyValue;
    let any_val = call_site.as_any_value_enum();
    if let inkwell::values::AnyValueEnum::PointerValue(pv) = any_val {
        Ok(pv.into())
    } else {
        Err(format!("{} did not return a pointer value", fn_name))
    }
}
