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
    ) -> Result<Option<PyValue<'ctx>>, String> {
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
                Ok(Some(self.extract_int_call_result(call)?))
            }
            PyType::Float => {
                let abs_fn = self.get_or_declare_c_builtin("abs_float");
                let call = self
                    .builder
                    .build_call(abs_fn, &[val.value().into()], "abs")
                    .unwrap();
                Ok(Some(self.extract_float_call_result(call)?))
            }
            _ => Err(format!("abs() not supported for type {:?}", val.ty)),
        }
    }

    /// Generate round() call
    pub(super) fn generate_round_call(
        &mut self,
        args: &[Expression],
    ) -> Result<Option<PyValue<'ctx>>, String> {
        if args.is_empty() || args.len() > 2 {
            return Err("round() takes 1 or 2 arguments".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;

        if args.len() == 1 {
            // round(x) - round to nearest integer
            match val.ty {
                PyType::Int => Ok(Some(val)), // int is already rounded
                PyType::Float => {
                    let round_fn = self.get_or_declare_c_builtin("round_float");
                    let call = self
                        .builder
                        .build_call(round_fn, &[val.value().into()], "round")
                        .unwrap();
                    Ok(Some(self.extract_int_call_result(call)?))
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
                    Ok(Some(self.extract_float_call_result(call)?))
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
                    Ok(Some(self.extract_int_call_result(call)?))
                }
                _ => Err(format!("round() not supported for type {:?}", val.ty)),
            }
        }
    }

    /// Generate min() call
    pub(super) fn generate_min_call(
        &mut self,
        args: &[Expression],
    ) -> Result<Option<PyValue<'ctx>>, String> {
        if args.len() < 2 {
            return Err("min() requires at least 2 arguments".to_string());
        }

        let mut result = self.evaluate_expression(&args[0])?;
        for arg in &args[1..] {
            let val = self.evaluate_expression(arg)?;
            result = self.generate_minmax_select(&result, &val, true)?;
        }
        Ok(Some(result))
    }

    /// Generate max() call
    pub(super) fn generate_max_call(
        &mut self,
        args: &[Expression],
    ) -> Result<Option<PyValue<'ctx>>, String> {
        if args.len() < 2 {
            return Err("max() requires at least 2 arguments".to_string());
        }

        let mut result = self.evaluate_expression(&args[0])?;
        for arg in &args[1..] {
            let val = self.evaluate_expression(arg)?;
            result = self.generate_minmax_select(&result, &val, false)?;
        }
        Ok(Some(result))
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
    ) -> Result<Option<PyValue<'ctx>>, String> {
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
            Ok(Some(self.extract_int_call_result(call)?))
        } else if base.ty == PyType::Int && exp.ty == PyType::Int {
            // pow(int, int) - use integer power, returns int
            let pow_fn = self.get_or_declare_c_builtin("pow_int");
            let call = self
                .builder
                .build_call(pow_fn, &[base.value().into(), exp.value().into()], "pow")
                .unwrap();
            Ok(Some(self.extract_int_call_result(call)?))
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
            Ok(Some(self.extract_float_call_result(call)?))
        }
    }

    /// Generate len() call
    pub(super) fn generate_len_call(
        &mut self,
        args: &[Expression],
    ) -> Result<Option<PyValue<'ctx>>, String> {
        if args.len() != 1 {
            return Err("len() takes exactly 1 argument".to_string());
        }

        let val = self.evaluate_expression(&args[0])?;
        match val.ty {
            PyType::Bytes => {
                let len_fn = self.get_or_declare_c_builtin("bytes_len");
                let call = self
                    .builder
                    .build_call(len_fn, &[val.value().into()], "len")
                    .unwrap();
                Ok(Some(self.extract_int_call_result(call)?))
            }
            _ => Err(format!("len() not supported for type {:?}", val.ty)),
        }
    }
}
