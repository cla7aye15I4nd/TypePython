//! Math builtin functions: abs, min, max, pow, round, len, print

use crate::ast::Expression;
use crate::codegen::CodeGen;
use crate::types::{PyType, PyValue};
use inkwell::FloatPredicate;
use inkwell::IntPredicate;

impl<'ctx> CodeGen<'ctx> {
    /// Generate abs() call with type dispatch
    pub(super) fn generate_abs_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("abs() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match val.ty {
            PyType::Int => {
                let abs_fn = self.get_or_declare_c_builtin("abs_int");
                let call = self
                    .builder
                    .build_call(abs_fn, &[val.value().into()], "abs")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            PyType::Float => {
                let abs_fn = self.get_or_declare_c_builtin("abs_float");
                let call = self
                    .builder
                    .build_call(abs_fn, &[val.value().into()], "abs")
                    .unwrap();
                Ok(self.extract_float_call_result(call)?)
            }
            _ => Err(format!("abs() not supported for type {:?}", val.ty)),
        }
    }

    /// Generate round() call
    pub(super) fn generate_round_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() || args.len() > 2 {
            return Err("round() takes 1 or 2 arguments".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;

        if args.len() == 1 {
            // round(x) - round to nearest integer
            match val.ty {
                PyType::Int => Ok(val), // int is already rounded
                PyType::Float => {
                    let round_fn = self.get_or_declare_c_builtin("round_float");
                    let call = self
                        .builder
                        .build_call(round_fn, &[val.value().into()], "round")
                        .unwrap();
                    Ok(self.extract_int_call_result(call))
                }
                _ => Err(format!("round() not supported for type {:?}", val.ty)),
            }
        } else {
            // round(x, ndigits)
            let ndigits = self.evaluate_expression(&args[1])?;
            if ndigits.ty != PyType::Int {
                return Err("round() ndigits must be an integer".to_string());
            }

            match val.ty {
                PyType::Float => {
                    let round_fn = self.get_or_declare_c_builtin("round_float_ndigits");
                    let call = self
                        .builder
                        .build_call(
                            round_fn,
                            &[val.value().into(), ndigits.value().into()],
                            "round",
                        )
                        .unwrap();
                    Ok(self.extract_float_call_result(call)?)
                }
                PyType::Int => {
                    // For integers with ndigits, use round_int_ndigits which returns int
                    let round_fn = self.get_or_declare_c_builtin("round_int_ndigits");
                    let call = self
                        .builder
                        .build_call(
                            round_fn,
                            &[val.value().into(), ndigits.value().into()],
                            "round",
                        )
                        .unwrap();
                    Ok(self.extract_int_call_result(call))
                }
                _ => Err(format!("round() not supported for type {:?}", val.ty)),
            }
        }
    }

    /// Generate min() call
    pub(super) fn generate_min_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() {
            return Err("min() requires at least 1 argument".to_string());
        }

        // Single argument case: min of an iterable
        if args.len() == 1 {
            let val = self.evaluate_expression(&args[0])?;
            return self.generate_iterable_minmax(&val, true);
        }

        // Multiple arguments: min of all values
        let mut result = self.evaluate_expression(&args[0])?;
        for arg in &args[1..] {
            let val = self.evaluate_expression(arg)?;
            result = self.generate_minmax_select(&result, &val, true)?;
        }
        Ok(result)
    }

    /// Generate max() call
    pub(super) fn generate_max_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() {
            return Err("max() requires at least 1 argument".to_string());
        }

        // Single argument case: max of an iterable
        if args.len() == 1 {
            let val = self.evaluate_expression(&args[0])?;
            return self.generate_iterable_minmax(&val, false);
        }

        // Multiple arguments: max of all values
        let mut result = self.evaluate_expression(&args[0])?;
        for arg in &args[1..] {
            let val = self.evaluate_expression(arg)?;
            result = self.generate_minmax_select(&result, &val, false)?;
        }
        Ok(result)
    }

    /// Generate min/max for an iterable (single argument to min/max)
    fn generate_iterable_minmax(
        &mut self,
        val: &PyValue<'ctx>,
        is_min: bool,
    ) -> Result<PyValue<'ctx>, String> {
        let func_name = if is_min { "min" } else { "max" };
        match &val.ty {
            PyType::List(elem_ty) => {
                let builtin = if is_min { "list_min" } else { "list_max" };
                let minmax_fn = self.get_or_declare_c_builtin(builtin);
                let call = self
                    .builder
                    .build_call(minmax_fn, &[val.value().into()], func_name)
                    .unwrap();
                // Return type matches element type
                match elem_ty.as_ref() {
                    PyType::Int => Ok(self.extract_int_call_result(call)),
                    _ => Err(format!(
                        "{}() on list only supported for int elements",
                        func_name
                    )),
                }
            }
            PyType::Set(elem_ty) => {
                let builtin = if is_min { "set_min" } else { "set_max" };
                let minmax_fn = self.get_or_declare_c_builtin(builtin);
                let call = self
                    .builder
                    .build_call(minmax_fn, &[val.value().into()], func_name)
                    .unwrap();
                // Return type matches element type
                match elem_ty.as_ref() {
                    PyType::Int => Ok(self.extract_int_call_result(call)),
                    _ => Err(format!(
                        "{}() on set only supported for int elements",
                        func_name
                    )),
                }
            }
            PyType::Dict(key_ty, _) => {
                let builtin = if is_min { "dict_min" } else { "dict_max" };
                let minmax_fn = self.get_or_declare_c_builtin(builtin);
                let call = self
                    .builder
                    .build_call(minmax_fn, &[val.value().into()], func_name)
                    .unwrap();
                // Return type matches key type
                match key_ty.as_ref() {
                    PyType::Int => Ok(self.extract_int_call_result(call)),
                    _ => Err(format!(
                        "{}() on dict only supported for int keys",
                        func_name
                    )),
                }
            }
            _ => Err(format!(
                "{}() with single argument requires an iterable (list, set, or dict)",
                func_name
            )),
        }
    }

    /// Generate min/max selection between two values
    pub(crate) fn generate_minmax_select(
        &mut self,
        a: &PyValue<'ctx>,
        b: &PyValue<'ctx>,
        is_min: bool,
    ) -> Result<PyValue<'ctx>, String> {
        let same_type = a.same_type(b);

        if same_type {
            match a.ty {
                PyType::Int => {
                    let a_val = a.value().into_int_value();
                    let b_val = b.value().into_int_value();
                    let pred = if is_min {
                        IntPredicate::SLT
                    } else {
                        IntPredicate::SGT
                    };
                    let cmp = self
                        .builder
                        .build_int_compare(pred, a_val, b_val, "cmp")
                        .unwrap();
                    let result = self
                        .builder
                        .build_select(cmp, a_val, b_val, "minmax")
                        .unwrap();
                    Ok(PyValue::int(result.into_int_value().into()))
                }
                PyType::Float => {
                    let a_val = a.value().into_float_value();
                    let b_val = b.value().into_float_value();
                    let pred = if is_min {
                        FloatPredicate::OLT
                    } else {
                        FloatPredicate::OGT
                    };
                    let cmp = self
                        .builder
                        .build_float_compare(pred, a_val, b_val, "cmp")
                        .unwrap();
                    let result = self
                        .builder
                        .build_select(cmp, a_val, b_val, "minmax")
                        .unwrap();
                    Ok(PyValue::float(result.into_float_value().into()))
                }
                PyType::Bytes => {
                    // Compare bytes by length
                    let a_len = self.bytes_len(a.value())?.value().into_int_value();
                    let b_len = self.bytes_len(b.value())?.value().into_int_value();
                    let pred = if is_min {
                        IntPredicate::SLT
                    } else {
                        IntPredicate::SGT
                    };
                    let cmp = self
                        .builder
                        .build_int_compare(pred, a_len, b_len, "cmp")
                        .unwrap();
                    let result = self
                        .builder
                        .build_select(cmp, a.value(), b.value(), "minmax")
                        .unwrap();
                    Ok(PyValue::bytes(result.into_pointer_value().into()))
                }
                PyType::List(_) => {
                    // Compare lists lexicographically
                    let cmp_fn = self.get_or_declare_c_builtin("list_cmp");
                    let cmp_call = self
                        .builder
                        .build_call(cmp_fn, &[a.value().into(), b.value().into()], "list_cmp")
                        .unwrap();
                    let cmp_result = self.extract_int_call_result(cmp_call);
                    // For min: select a if cmp < 0 (a < b), else b
                    // For max: select a if cmp > 0 (a > b), else b
                    let pred = if is_min {
                        IntPredicate::SLT
                    } else {
                        IntPredicate::SGT
                    };
                    let zero = self.context.i64_type().const_zero();
                    let cmp = self
                        .builder
                        .build_int_compare(pred, cmp_result.value().into_int_value(), zero, "cmp")
                        .unwrap();
                    let result = self
                        .builder
                        .build_select(cmp, a.value(), b.value(), "minmax")
                        .unwrap();
                    Ok(PyValue::new(
                        result.into_pointer_value().into(),
                        a.ty.clone(),
                        None,
                    ))
                }
                PyType::Dict(_, _) => {
                    // Python doesn't support comparison between dicts
                    Err("'<' not supported between instances of 'dict' and 'dict'".to_string())
                }
                PyType::Set(_) => {
                    // Compare sets using proper subset/superset
                    let cmp_fn = self.get_or_declare_c_builtin("set_cmp");
                    let cmp_call = self
                        .builder
                        .build_call(cmp_fn, &[a.value().into(), b.value().into()], "set_cmp")
                        .unwrap();
                    let cmp_result = self.extract_int_call_result(cmp_call);
                    // For min: select b if cmp > 0 (a > b), else a
                    // For max: select b if cmp < 0 (a < b), else a
                    // This ensures incomparable sets (cmp == 0) return first argument
                    let pred = if is_min {
                        IntPredicate::SGT
                    } else {
                        IntPredicate::SLT
                    };
                    let zero = self.context.i64_type().const_zero();
                    let select_b = self
                        .builder
                        .build_int_compare(pred, cmp_result.value().into_int_value(), zero, "cmp")
                        .unwrap();
                    let result = self
                        .builder
                        .build_select(select_b, b.value(), a.value(), "minmax")
                        .unwrap();
                    Ok(PyValue::new(
                        result.into_pointer_value().into(),
                        a.ty.clone(),
                        None,
                    ))
                }
                _ => Err(format!("min/max not supported for type {:?}", a.ty)),
            }
        } else {
            // Mixed types - convert to float
            let a_float = self.coerce_to_float(a)?;
            let b_float = self.coerce_to_float(b)?;
            let pred = if is_min {
                FloatPredicate::OLT
            } else {
                FloatPredicate::OGT
            };
            let cmp = self
                .builder
                .build_float_compare(pred, a_float, b_float, "cmp")
                .unwrap();
            let result = self
                .builder
                .build_select(cmp, a_float, b_float, "minmax")
                .unwrap();
            Ok(PyValue::float(result.into_float_value().into()))
        }
    }

    /// Coerce a value to float
    fn coerce_to_float(
        &mut self,
        val: &PyValue<'ctx>,
    ) -> Result<inkwell::values::FloatValue<'ctx>, String> {
        match val.ty {
            PyType::Float => Ok(val.value().into_float_value()),
            PyType::Int => Ok(self
                .builder
                .build_signed_int_to_float(
                    val.value().into_int_value(),
                    self.context.f64_type(),
                    "int_to_float",
                )
                .unwrap()),
            _ => Err(format!("Cannot coerce {:?} to float", val.ty)),
        }
    }

    /// Generate pow() call
    pub(super) fn generate_pow_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() < 2 || args.len() > 3 {
            return Err("pow() takes 2 or 3 arguments".to_string());
        }

        let base = self.evaluate_expression(&args[0])?;
        let exp = self.evaluate_expression(&args[1])?;

        if args.len() == 3 {
            // pow(base, exp, mod) - modular exponentiation
            let modulo = self.evaluate_expression(&args[2])?;
            if base.ty != PyType::Int || exp.ty != PyType::Int || modulo.ty != PyType::Int {
                return Err("pow() with 3 arguments requires all int arguments".to_string());
            }
            let pow_mod_fn = self.get_or_declare_c_builtin("pow_int_mod");
            let call = self
                .builder
                .build_call(
                    pow_mod_fn,
                    &[
                        base.value().into(),
                        exp.value().into(),
                        modulo.value().into(),
                    ],
                    "pow",
                )
                .unwrap();
            Ok(self.extract_int_call_result(call))
        } else if base.ty == PyType::Int && exp.ty == PyType::Int {
            // pow(int, int) - use integer power, returns int
            let pow_fn = self.get_or_declare_c_builtin("pow_int");
            let call = self
                .builder
                .build_call(pow_fn, &[base.value().into(), exp.value().into()], "pow")
                .unwrap();
            Ok(self.extract_int_call_result(call))
        } else {
            // pow(base, exp) - use floating point pow
            let base_float = self.coerce_to_float(&base)?;
            let exp_float = self.coerce_to_float(&exp)?;

            let pow_intrinsic = inkwell::intrinsics::Intrinsic::find("llvm.pow.f64")
                .ok_or("Failed to find llvm.pow.f64 intrinsic")?;
            let pow_fn = pow_intrinsic
                .get_declaration(&self.module, &[self.context.f64_type().into()])
                .ok_or("Failed to get pow declaration")?;

            let call = self
                .builder
                .build_call(pow_fn, &[base_float.into(), exp_float.into()], "pow")
                .unwrap();
            Ok(self.extract_float_call_result(call)?)
        }
    }

    /// Generate len() call
    pub(super) fn generate_len_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("len() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match &val.ty {
            PyType::Bytes => {
                let len_fn = self.get_or_declare_c_builtin("bytes_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "len")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            PyType::List(_) => {
                let len_fn = self.get_or_declare_c_builtin("list_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "len")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            PyType::Dict(_, _) => {
                let len_fn = self.get_or_declare_c_builtin("dict_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "len")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            PyType::Set(_) => {
                let len_fn = self.get_or_declare_c_builtin("set_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "len")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            PyType::Str => {
                let len_fn = self.get_or_declare_c_builtin("str_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "len")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            _ => Err(format!("len() not supported for type {:?}", val.ty)),
        }
    }

    // ========================================================================
    // Type conversion builtins: int(), float(), bool(), str()
    // ========================================================================

    /// Generate int() call - convert to integer
    pub(super) fn generate_int_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("int() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match val.ty {
            PyType::Int => Ok(val),
            PyType::Float => {
                let result = self
                    .builder
                    .build_float_to_signed_int(
                        val.value().into_float_value(),
                        self.context.i64_type(),
                        "ftoi",
                    )
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            PyType::Bool => {
                let result = self
                    .builder
                    .build_int_z_extend(
                        val.value().into_int_value(),
                        self.context.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                Ok(PyValue::int(result.into()))
            }
            _ => Err(format!("int() not supported for type {:?}", val.ty)),
        }
    }

    /// Generate float() call - convert to float
    pub(super) fn generate_float_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("float() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match val.ty {
            PyType::Float => Ok(val),
            PyType::Int => {
                let result = self
                    .builder
                    .build_signed_int_to_float(
                        val.value().into_int_value(),
                        self.context.f64_type(),
                        "itof",
                    )
                    .unwrap();
                Ok(PyValue::float(result.into()))
            }
            PyType::Bool => {
                let int_val = self
                    .builder
                    .build_int_z_extend(
                        val.value().into_int_value(),
                        self.context.i64_type(),
                        "btoi",
                    )
                    .unwrap();
                let result = self
                    .builder
                    .build_signed_int_to_float(int_val, self.context.f64_type(), "itof")
                    .unwrap();
                Ok(PyValue::float(result.into()))
            }
            _ => Err(format!("float() not supported for type {:?}", val.ty)),
        }
    }

    /// Generate bool() call - convert to boolean
    pub(super) fn generate_bool_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("bool() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match &val.ty {
            PyType::Bool => Ok(val),
            PyType::Int => {
                let zero = self.context.i64_type().const_zero();
                let result = self
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        val.value().into_int_value(),
                        zero,
                        "itob",
                    )
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            PyType::Float => {
                let zero = self.context.f64_type().const_zero();
                let result = self
                    .builder
                    .build_float_compare(
                        FloatPredicate::ONE,
                        val.value().into_float_value(),
                        zero,
                        "ftob",
                    )
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            PyType::Str => {
                let len_fn = self.get_or_declare_c_builtin("str_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "str_len")
                    .unwrap();
                let len = self.extract_int_call_result(call);
                let zero = self.context.i64_type().const_zero();
                let result = self
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        len.value().into_int_value(),
                        zero,
                        "stob",
                    )
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            PyType::Bytes => {
                let len_fn = self.get_or_declare_c_builtin("bytes_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "bytes_len")
                    .unwrap();
                let len = self.extract_int_call_result(call);
                let zero = self.context.i64_type().const_zero();
                let result = self
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        len.value().into_int_value(),
                        zero,
                        "btob",
                    )
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            PyType::List(_) => {
                let len_fn = self.get_or_declare_c_builtin("list_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "list_len")
                    .unwrap();
                let len = self.extract_int_call_result(call);
                let zero = self.context.i64_type().const_zero();
                let result = self
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        len.value().into_int_value(),
                        zero,
                        "ltob",
                    )
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            PyType::Dict(_, _) => {
                let len_fn = self.get_or_declare_c_builtin("dict_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "dict_len")
                    .unwrap();
                let len = self.extract_int_call_result(call);
                let zero = self.context.i64_type().const_zero();
                let result = self
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        len.value().into_int_value(),
                        zero,
                        "dtob",
                    )
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            PyType::Set(_) => {
                let len_fn = self.get_or_declare_c_builtin("set_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "set_len")
                    .unwrap();
                let len = self.extract_int_call_result(call);
                let zero = self.context.i64_type().const_zero();
                let result = self
                    .builder
                    .build_int_compare(
                        IntPredicate::NE,
                        len.value().into_int_value(),
                        zero,
                        "stob",
                    )
                    .unwrap();
                Ok(PyValue::bool(result.into()))
            }
            PyType::None => {
                // None is always falsy
                let result = self.context.bool_type().const_zero();
                Ok(PyValue::bool(result.into()))
            }
            _ => Err(format!("bool() not supported for type {:?}", val.ty)),
        }
    }

    /// Generate str() call - convert to string
    pub(super) fn generate_str_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("str() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match &val.ty {
            PyType::Str => Ok(val),
            PyType::Int => {
                let str_fn = self.get_or_declare_c_builtin("int_to_str");
                let call = self
                    .builder
                    .build_call(str_fn, &[val.value().into()], "int_to_str")
                    .unwrap();
                Ok(self.extract_str_call_result(call))
            }
            PyType::Float => {
                let str_fn = self.get_or_declare_c_builtin("float_to_str");
                let call = self
                    .builder
                    .build_call(str_fn, &[val.value().into()], "float_to_str")
                    .unwrap();
                Ok(self.extract_str_call_result(call))
            }
            PyType::Bool => {
                let str_fn = self.get_or_declare_c_builtin("bool_to_str");
                let call = self
                    .builder
                    .build_call(str_fn, &[val.value().into()], "bool_to_str")
                    .unwrap();
                Ok(self.extract_str_call_result(call))
            }
            _ => Err(format!("str() not supported for type {:?}", val.ty)),
        }
    }

    // ========================================================================
    // String representation builtins: bin(), hex(), oct(), chr(), ord(), ascii()
    // ========================================================================

    /// Generate bin() call - convert int to binary string
    pub(super) fn generate_bin_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("bin() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        let int_val = match val.ty {
            PyType::Int => val.value().into_int_value(),
            PyType::Bool => self
                .builder
                .build_int_z_extend(
                    val.value().into_int_value(),
                    self.context.i64_type(),
                    "btoi",
                )
                .unwrap(),
            _ => return Err(format!("bin() not supported for type {:?}", val.ty)),
        };

        let bin_fn = self.get_or_declare_c_builtin("int_to_bin");
        let call = self
            .builder
            .build_call(bin_fn, &[int_val.into()], "bin")
            .unwrap();
        Ok(self.extract_str_call_result(call))
    }

    /// Generate hex() call - convert int to hexadecimal string
    pub(super) fn generate_hex_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("hex() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        let int_val = match val.ty {
            PyType::Int => val.value().into_int_value(),
            PyType::Bool => self
                .builder
                .build_int_z_extend(
                    val.value().into_int_value(),
                    self.context.i64_type(),
                    "btoi",
                )
                .unwrap(),
            _ => return Err(format!("hex() not supported for type {:?}", val.ty)),
        };

        let hex_fn = self.get_or_declare_c_builtin("int_to_hex");
        let call = self
            .builder
            .build_call(hex_fn, &[int_val.into()], "hex")
            .unwrap();
        Ok(self.extract_str_call_result(call))
    }

    /// Generate oct() call - convert int to octal string
    pub(super) fn generate_oct_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("oct() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        let int_val = match val.ty {
            PyType::Int => val.value().into_int_value(),
            PyType::Bool => self
                .builder
                .build_int_z_extend(
                    val.value().into_int_value(),
                    self.context.i64_type(),
                    "btoi",
                )
                .unwrap(),
            _ => return Err(format!("oct() not supported for type {:?}", val.ty)),
        };

        let oct_fn = self.get_or_declare_c_builtin("int_to_oct");
        let call = self
            .builder
            .build_call(oct_fn, &[int_val.into()], "oct")
            .unwrap();
        Ok(self.extract_str_call_result(call))
    }

    /// Generate chr() call - convert int to single character string
    pub(super) fn generate_chr_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("chr() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        let int_val = match val.ty {
            PyType::Int => val.value().into_int_value(),
            PyType::Bool => self
                .builder
                .build_int_z_extend(
                    val.value().into_int_value(),
                    self.context.i64_type(),
                    "btoi",
                )
                .unwrap(),
            _ => return Err(format!("chr() not supported for type {:?}", val.ty)),
        };

        let chr_fn = self.get_or_declare_c_builtin("int_to_chr");
        let call = self
            .builder
            .build_call(chr_fn, &[int_val.into()], "chr")
            .unwrap();
        Ok(self.extract_str_call_result(call))
    }

    /// Generate ord() call - convert single character string to int
    pub(super) fn generate_ord_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("ord() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match &val.ty {
            PyType::Str => {
                let ord_fn = self.get_or_declare_c_builtin("str_ord");
                let call = self
                    .builder
                    .build_call(ord_fn, &[val.value().into()], "ord")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            _ => Err(format!("ord() not supported for type {:?}", val.ty)),
        }
    }

    /// Generate ascii() call - return ASCII representation of object
    pub(super) fn generate_ascii_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("ascii() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match &val.ty {
            PyType::Int => {
                let ascii_fn = self.get_or_declare_c_builtin("int_to_ascii");
                let call = self
                    .builder
                    .build_call(ascii_fn, &[val.value().into()], "ascii")
                    .unwrap();
                Ok(self.extract_str_call_result(call))
            }
            PyType::Float => {
                let ascii_fn = self.get_or_declare_c_builtin("float_to_ascii");
                let call = self
                    .builder
                    .build_call(ascii_fn, &[val.value().into()], "ascii")
                    .unwrap();
                Ok(self.extract_str_call_result(call))
            }
            PyType::Bool => {
                let ascii_fn = self.get_or_declare_c_builtin("bool_to_ascii");
                let call = self
                    .builder
                    .build_call(ascii_fn, &[val.value().into()], "ascii")
                    .unwrap();
                Ok(self.extract_str_call_result(call))
            }
            PyType::Str => {
                let ascii_fn = self.get_or_declare_c_builtin("str_to_ascii");
                let call = self
                    .builder
                    .build_call(ascii_fn, &[val.value().into()], "ascii")
                    .unwrap();
                Ok(self.extract_str_call_result(call))
            }
            PyType::Bytes => {
                let ascii_fn = self.get_or_declare_c_builtin("bytes_to_ascii");
                let call = self
                    .builder
                    .build_call(ascii_fn, &[val.value().into()], "ascii")
                    .unwrap();
                Ok(self.extract_str_call_result(call))
            }
            PyType::None => {
                let ascii_fn = self.get_or_declare_c_builtin("none_to_ascii");
                let call = self.builder.build_call(ascii_fn, &[], "ascii").unwrap();
                Ok(self.extract_str_call_result(call))
            }
            _ => Err(format!("ascii() not supported for type {:?}", val.ty)),
        }
    }

    // ========================================================================
    // Sequence builtins: sum(), sorted(), reversed()
    // ========================================================================

    /// Generate sum() call - sum elements of an iterable
    pub(super) fn generate_sum_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.is_empty() || args.len() > 2 {
            return Err("sum() takes 1 or 2 arguments".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;

        // Get start value (default 0)
        let start = if args.len() == 2 {
            let start_val = self.evaluate_expression(&args[1])?;
            if start_val.ty != PyType::Int {
                return Err("sum() start must be an integer".to_string());
            }
            start_val.value().into_int_value()
        } else {
            self.context.i64_type().const_zero()
        };

        match &val.ty {
            PyType::List(_) => {
                let sum_fn = self.get_or_declare_c_builtin("list_sum");
                let call = self
                    .builder
                    .build_call(sum_fn, &[val.value().into(), start.into()], "sum")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            PyType::Set(_) => {
                let sum_fn = self.get_or_declare_c_builtin("set_sum");
                let call = self
                    .builder
                    .build_call(sum_fn, &[val.value().into(), start.into()], "sum")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            PyType::Bytes => {
                let sum_fn = self.get_or_declare_c_builtin("bytes_sum");
                let call = self
                    .builder
                    .build_call(sum_fn, &[val.value().into(), start.into()], "sum")
                    .unwrap();
                Ok(self.extract_int_call_result(call))
            }
            _ => Err(format!("sum() not supported for type {:?}", val.ty)),
        }
    }

    /// Generate sorted() call - return sorted list
    pub(super) fn generate_sorted_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("sorted() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match &val.ty {
            PyType::List(elem_ty) => {
                let sorted_fn = self.get_or_declare_c_builtin("list_sorted");
                let call = self
                    .builder
                    .build_call(sorted_fn, &[val.value().into()], "sorted")
                    .unwrap();
                let result = self.extract_ptr_call_result(call);
                Ok(PyValue::new(
                    result.value(),
                    PyType::List(elem_ty.clone()),
                    None,
                ))
            }
            PyType::Set(elem_ty) => {
                let sorted_fn = self.get_or_declare_c_builtin("set_sorted");
                let call = self
                    .builder
                    .build_call(sorted_fn, &[val.value().into()], "sorted")
                    .unwrap();
                let result = self.extract_ptr_call_result(call);
                Ok(PyValue::new(
                    result.value(),
                    PyType::List(elem_ty.clone()),
                    None,
                ))
            }
            PyType::Str => {
                let sorted_fn = self.get_or_declare_c_builtin("str_sorted");
                let call = self
                    .builder
                    .build_call(sorted_fn, &[val.value().into()], "sorted")
                    .unwrap();
                let result = self.extract_ptr_call_result(call);
                Ok(PyValue::new(
                    result.value(),
                    PyType::List(Box::new(PyType::Str)),
                    None,
                ))
            }
            _ => Err(format!("sorted() not supported for type {:?}", val.ty)),
        }
    }

    /// Generate reversed() call - return reversed iterator/list
    pub(super) fn generate_reversed_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 1 {
            return Err("reversed() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match &val.ty {
            PyType::List(elem_ty) => {
                let reversed_fn = self.get_or_declare_c_builtin("list_reversed");
                let call = self
                    .builder
                    .build_call(reversed_fn, &[val.value().into()], "reversed")
                    .unwrap();
                let result = self.extract_ptr_call_result(call);
                Ok(PyValue::new(
                    result.value(),
                    PyType::List(elem_ty.clone()),
                    None,
                ))
            }
            PyType::Str => {
                let reversed_fn = self.get_or_declare_c_builtin("str_reversed");
                let call = self
                    .builder
                    .build_call(reversed_fn, &[val.value().into()], "reversed")
                    .unwrap();
                Ok(self.extract_str_call_result(call))
            }
            _ => Err(format!("reversed() not supported for type {:?}", val.ty)),
        }
    }

    // ========================================================================
    // Math builtins: divmod()
    // ========================================================================

    /// Generate divmod() call - return [quotient, remainder] list
    pub(super) fn generate_divmod_call(
        &mut self,
        args: &[Expression],
    ) -> Result<PyValue<'ctx>, String> {
        if args.len() != 2 {
            return Err("divmod() takes exactly 2 arguments".to_string());
        }

        let a = self.evaluate_expression(&args[0])?;
        let b = self.evaluate_expression(&args[1])?;

        // Both must be numeric (int or float)
        match (&a.ty, &b.ty) {
            (PyType::Int, PyType::Int) => {
                let divmod_fn = self.get_or_declare_c_builtin("divmod_int");
                let call = self
                    .builder
                    .build_call(
                        divmod_fn,
                        &[a.value().into(), b.value().into()],
                        "divmod",
                    )
                    .unwrap();
                // Returns a list [quotient, remainder] as a pointer
                let result = self.extract_ptr_call_result(call);
                Ok(PyValue::new(
                    result.value(),
                    PyType::List(Box::new(PyType::Int)),
                    None,
                ))
            }
            (PyType::Int, PyType::Bool) | (PyType::Bool, PyType::Int) | (PyType::Bool, PyType::Bool) => {
                // Convert bools to ints
                let a_int = match a.ty {
                    PyType::Bool => self
                        .builder
                        .build_int_z_extend(
                            a.value().into_int_value(),
                            self.context.i64_type(),
                            "btoi",
                        )
                        .unwrap(),
                    _ => a.value().into_int_value(),
                };
                let b_int = match b.ty {
                    PyType::Bool => self
                        .builder
                        .build_int_z_extend(
                            b.value().into_int_value(),
                            self.context.i64_type(),
                            "btoi",
                        )
                        .unwrap(),
                    _ => b.value().into_int_value(),
                };
                let divmod_fn = self.get_or_declare_c_builtin("divmod_int");
                let call = self
                    .builder
                    .build_call(divmod_fn, &[a_int.into(), b_int.into()], "divmod")
                    .unwrap();
                let result = self.extract_ptr_call_result(call);
                Ok(PyValue::new(
                    result.value(),
                    PyType::List(Box::new(PyType::Int)),
                    None,
                ))
            }
            (PyType::Float, _) | (_, PyType::Float) => {
                // Convert to floats
                let a_float = self.coerce_to_float(&a)?;
                let b_float = self.coerce_to_float(&b)?;
                let divmod_fn = self.get_or_declare_c_builtin("divmod_float");
                let call = self
                    .builder
                    .build_call(divmod_fn, &[a_float.into(), b_float.into()], "divmod")
                    .unwrap();
                let result = self.extract_ptr_call_result(call);
                Ok(PyValue::new(
                    result.value(),
                    PyType::List(Box::new(PyType::Float)),
                    None,
                ))
            }
            _ => Err(format!(
                "divmod() not supported for types {:?} and {:?}",
                a.ty, b.ty
            )),
        }
    }
}
