use crate::ast::BoolOp;
use crate::tir::expr::TirExpr;
use crate::tir::TirProgram;
use inkwell::values::AnyValue;

use super::function_gen::FunctionGenContext;

impl<'ctx, 'a> FunctionGenContext<'ctx, 'a> {
    pub(crate) fn codegen_binop(
        &self,
        lhs: inkwell::values::IntValue<'ctx>,
        op: crate::ast::BinOperator,
        rhs: inkwell::values::IntValue<'ctx>,
    ) -> inkwell::values::IntValue<'ctx> {
        use crate::ast::BinOperator::*;
        match op {
            Add => self.ctx.builder.build_int_add(lhs, rhs, "add").unwrap(),
            Sub => self.ctx.builder.build_int_sub(lhs, rhs, "sub").unwrap(),
            Mult => self.ctx.builder.build_int_mul(lhs, rhs, "mul").unwrap(),
            Div => self
                .ctx
                .builder
                .build_int_signed_div(lhs, rhs, "div")
                .unwrap(),
            FloorDiv => self
                .ctx
                .builder
                .build_int_signed_div(lhs, rhs, "floordiv")
                .unwrap(),
            Mod => self
                .ctx
                .builder
                .build_int_signed_rem(lhs, rhs, "mod")
                .unwrap(),
            LShift => self
                .ctx
                .builder
                .build_left_shift(lhs, rhs, "lshift")
                .unwrap(),
            RShift => self
                .ctx
                .builder
                .build_right_shift(lhs, rhs, true, "rshift")
                .unwrap(),
            BitOr => self.ctx.builder.build_or(lhs, rhs, "bitor").unwrap(),
            BitXor => self.ctx.builder.build_xor(lhs, rhs, "bitxor").unwrap(),
            BitAnd => self.ctx.builder.build_and(lhs, rhs, "bitand").unwrap(),
            Pow => self.codegen_pow(lhs, rhs),
        }
    }

    pub(crate) fn codegen_float_binop(
        &self,
        lhs: inkwell::values::FloatValue<'ctx>,
        op: crate::ast::BinOperator,
        rhs: inkwell::values::FloatValue<'ctx>,
    ) -> inkwell::values::FloatValue<'ctx> {
        use crate::ast::BinOperator::*;
        let f64_type = self.ctx.context.f64_type();
        match op {
            Add => self.ctx.builder.build_float_add(lhs, rhs, "fadd").unwrap(),
            Sub => self.ctx.builder.build_float_sub(lhs, rhs, "fsub").unwrap(),
            Mult => self.ctx.builder.build_float_mul(lhs, rhs, "fmul").unwrap(),
            Div => self.ctx.builder.build_float_div(lhs, rhs, "fdiv").unwrap(),
            FloorDiv => {
                // Floor division: divide then floor
                let div_result = self.ctx.builder.build_float_div(lhs, rhs, "fdiv").unwrap();
                // Call llvm.floor intrinsic
                let floor_fn = inkwell::intrinsics::Intrinsic::find("llvm.floor").unwrap();
                let floor_fn_val = floor_fn
                    .get_declaration(&self.ctx.module, &[f64_type.into()])
                    .unwrap();
                let call = self
                    .ctx
                    .builder
                    .build_call(floor_fn_val, &[div_result.into()], "floor")
                    .unwrap();
                // LLVM guarantees floor returns the same type as input (f64)
                call.as_any_value_enum().into_float_value()
            }
            Mod => self.ctx.builder.build_float_rem(lhs, rhs, "fmod").unwrap(),
            Pow => self.codegen_float_pow(lhs, rhs),
            // Bitwise operations are not valid for floats - TIR type checking prevents this
            // Return 0.0 for exhaustiveness (this code path should never execute)
            LShift | RShift | BitOr | BitXor | BitAnd => f64_type.const_zero(),
        }
    }

    /// Generate code for float exponentiation using llvm.pow intrinsic
    pub(crate) fn codegen_float_pow(
        &self,
        base: inkwell::values::FloatValue<'ctx>,
        exp: inkwell::values::FloatValue<'ctx>,
    ) -> inkwell::values::FloatValue<'ctx> {
        let pow_fn = inkwell::intrinsics::Intrinsic::find("llvm.pow").unwrap();
        let f64_type = self.ctx.context.f64_type();
        let pow_fn_val = pow_fn
            .get_declaration(&self.ctx.module, &[f64_type.into()])
            .unwrap();
        let call = self
            .ctx
            .builder
            .build_call(pow_fn_val, &[base.into(), exp.into()], "fpow")
            .unwrap();
        // LLVM guarantees pow returns the same type as input (f64)
        call.as_any_value_enum().into_float_value()
    }

    /// Generate code for integer exponentiation using exponentiation by squaring
    pub(crate) fn codegen_pow(
        &self,
        base: inkwell::values::IntValue<'ctx>,
        exp: inkwell::values::IntValue<'ctx>,
    ) -> inkwell::values::IntValue<'ctx> {
        let i64_type = self.ctx.context.i64_type();
        let current_fn = self
            .ctx
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        // Create basic blocks for the loop
        let entry_bb = self.ctx.builder.get_insert_block().unwrap();
        let loop_bb = self.ctx.context.append_basic_block(current_fn, "pow_loop");
        let loop_body_bb = self.ctx.context.append_basic_block(current_fn, "pow_body");
        let loop_end_bb = self.ctx.context.append_basic_block(current_fn, "pow_end");

        // Initialize: result = 1, current_base = base, current_exp = exp
        let one = i64_type.const_int(1, false);
        let zero = i64_type.const_int(0, false);
        self.ctx
            .builder
            .build_unconditional_branch(loop_bb)
            .unwrap();

        // Loop header: check if exp > 0
        self.ctx.builder.position_at_end(loop_bb);
        let result_phi = self.ctx.builder.build_phi(i64_type, "result").unwrap();
        let base_phi = self
            .ctx
            .builder
            .build_phi(i64_type, "current_base")
            .unwrap();
        let exp_phi = self.ctx.builder.build_phi(i64_type, "current_exp").unwrap();

        result_phi.add_incoming(&[(&one, entry_bb)]);
        base_phi.add_incoming(&[(&base, entry_bb)]);
        exp_phi.add_incoming(&[(&exp, entry_bb)]);

        let current_result = result_phi.as_basic_value().into_int_value();
        let current_base = base_phi.as_basic_value().into_int_value();
        let current_exp = exp_phi.as_basic_value().into_int_value();

        let exp_gt_zero = self
            .ctx
            .builder
            .build_int_compare(inkwell::IntPredicate::SGT, current_exp, zero, "exp_gt_zero")
            .unwrap();
        self.ctx
            .builder
            .build_conditional_branch(exp_gt_zero, loop_body_bb, loop_end_bb)
            .unwrap();

        // Loop body: if exp & 1, result *= base; base *= base; exp >>= 1
        self.ctx.builder.position_at_end(loop_body_bb);
        let exp_and_one = self
            .ctx
            .builder
            .build_and(current_exp, one, "exp_and_one")
            .unwrap();
        let is_odd = self
            .ctx
            .builder
            .build_int_compare(inkwell::IntPredicate::NE, exp_and_one, zero, "is_odd")
            .unwrap();

        // result = is_odd ? result * base : result
        let result_times_base = self
            .ctx
            .builder
            .build_int_mul(current_result, current_base, "result_times_base")
            .unwrap();
        let new_result = self
            .ctx
            .builder
            .build_select(is_odd, result_times_base, current_result, "new_result")
            .unwrap()
            .into_int_value();

        // base = base * base
        let new_base = self
            .ctx
            .builder
            .build_int_mul(current_base, current_base, "new_base")
            .unwrap();

        // exp = exp >> 1
        let new_exp = self
            .ctx
            .builder
            .build_right_shift(current_exp, one, false, "new_exp")
            .unwrap();

        self.ctx
            .builder
            .build_unconditional_branch(loop_bb)
            .unwrap();

        // Add phi incoming values from loop body
        result_phi.add_incoming(&[(&new_result, loop_body_bb)]);
        base_phi.add_incoming(&[(&new_base, loop_body_bb)]);
        exp_phi.add_incoming(&[(&new_exp, loop_body_bb)]);

        // Loop end: return result
        self.ctx.builder.position_at_end(loop_end_bb);
        current_result
    }

    pub(crate) fn codegen_compare(
        &self,
        lhs: inkwell::values::IntValue<'ctx>,
        op: crate::ast::CompareOp,
        rhs: inkwell::values::IntValue<'ctx>,
    ) -> inkwell::values::IntValue<'ctx> {
        use crate::ast::CompareOp::*;
        use inkwell::IntPredicate::*;
        let predicate = match op {
            Eq => EQ,
            NotEq => NE,
            Lt => SLT,
            LtE => SLE,
            Gt => SGT,
            GtE => SGE,
        };
        // Get i1 result then extend to i8 for consistency with bool representation
        let cmp = self
            .ctx
            .builder
            .build_int_compare(predicate, lhs, rhs, "cmp")
            .unwrap();
        self.ctx
            .builder
            .build_int_z_extend(cmp, self.ctx.context.i8_type(), "cmp_bool")
            .unwrap()
    }

    pub(crate) fn codegen_float_compare(
        &self,
        lhs: inkwell::values::FloatValue<'ctx>,
        op: crate::ast::CompareOp,
        rhs: inkwell::values::FloatValue<'ctx>,
    ) -> inkwell::values::IntValue<'ctx> {
        use crate::ast::CompareOp::*;
        use inkwell::FloatPredicate::*;
        let predicate = match op {
            Eq => OEQ,    // Ordered and equal
            NotEq => ONE, // Ordered and not equal
            Lt => OLT,    // Ordered and less than
            LtE => OLE,   // Ordered and less than or equal
            Gt => OGT,    // Ordered and greater than
            GtE => OGE,   // Ordered and greater than or equal
        };
        // Get i1 result then extend to i8 for consistency with bool representation
        let cmp = self
            .ctx
            .builder
            .build_float_compare(predicate, lhs, rhs, "fcmp")
            .unwrap();
        self.ctx
            .builder
            .build_int_z_extend(cmp, self.ctx.context.i8_type(), "fcmp_bool")
            .unwrap()
    }
    /// Generate code for boolean operators with short-circuit evaluation
    pub(crate) fn codegen_boolop(
        &mut self,
        op: BoolOp,
        values: &[TirExpr],
        program: &TirProgram,
    ) -> inkwell::values::IntValue<'ctx> {
        let func = self.ctx.current_function.unwrap();
        let i8_type = self.ctx.context.i8_type();

        // Allocate result storage (i8 for bool)
        let result_ptr = self
            .ctx
            .builder
            .build_alloca(i8_type, "boolop_result")
            .unwrap();

        // Initialize based on operator
        let init_val = match op {
            BoolOp::And => i8_type.const_int(1, false), // true
            BoolOp::Or => i8_type.const_int(0, false),  // false
        };
        self.ctx.builder.build_store(result_ptr, init_val).unwrap();

        let end_bb = self.ctx.context.append_basic_block(func, "boolop.end");

        for (i, val) in values.iter().enumerate() {
            let val_expr = self.codegen_expr(val, program);
            let bool_val = self.convert_to_bool(val_expr); // Returns i1

            if i == values.len() - 1 {
                // Last value - extend to i8, store and jump to end
                let i8_val = self
                    .ctx
                    .builder
                    .build_int_z_extend(bool_val, i8_type, "bool_to_i8")
                    .unwrap();
                self.ctx.builder.build_store(result_ptr, i8_val).unwrap();
                self.ctx.builder.build_unconditional_branch(end_bb).unwrap();
            } else {
                let next_bb = self.ctx.context.append_basic_block(func, "boolop.next");
                let short_bb = self.ctx.context.append_basic_block(func, "boolop.short");

                match op {
                    BoolOp::And => {
                        // Short-circuit: if false, result is false
                        self.ctx
                            .builder
                            .build_conditional_branch(bool_val, next_bb, short_bb)
                            .unwrap();
                    }
                    BoolOp::Or => {
                        // Short-circuit: if true, result is true
                        self.ctx
                            .builder
                            .build_conditional_branch(bool_val, short_bb, next_bb)
                            .unwrap();
                    }
                }

                // Short-circuit block
                self.ctx.builder.position_at_end(short_bb);
                let short_val = match op {
                    BoolOp::And => i8_type.const_int(0, false), // false
                    BoolOp::Or => i8_type.const_int(1, false),  // true
                };
                self.ctx.builder.build_store(result_ptr, short_val).unwrap();
                self.ctx.builder.build_unconditional_branch(end_bb).unwrap();

                self.ctx.builder.position_at_end(next_bb);
            }
        }

        self.ctx.builder.position_at_end(end_bb);
        self.ctx
            .builder
            .build_load(i8_type, result_ptr, "boolop.val")
            .unwrap()
            .into_int_value()
    }
}
