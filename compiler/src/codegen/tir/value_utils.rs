use inkwell::values::{BasicValueEnum, PointerValue};

use crate::tir::expr::VarRef;
use crate::tir::stmt::TirLValue;
use crate::tir::TirProgram;

use super::function_gen::FunctionGenContext;

impl<'ctx, 'a> FunctionGenContext<'ctx, 'a> {
    pub(crate) fn load_var(
        &mut self,
        var_ref: &VarRef,
        program: &TirProgram,
    ) -> BasicValueEnum<'ctx> {
        match var_ref {
            VarRef::Local(local_id) => {
                let (ptr, llvm_ty) = self.locals[local_id.index()];
                self.ctx.builder.build_load(llvm_ty, ptr, "local").unwrap()
            }
            VarRef::Param(idx) => self.params[*idx as usize],
            VarRef::Global(mod_id, global_id) => {
                // Look up using module-qualified key
                let module = program.module(*mod_id);
                if let Some(global) = module.globals.get(global_id.index()) {
                    let key = format!("{}::{}", module.name, global.name);
                    if let Some(ptr) = self.ctx.global_variables.get(&key) {
                        // Use the correct type from the global definition
                        let llvm_ty = self.ctx.tir_type_to_llvm(&global.ty, program);
                        return self
                            .ctx
                            .builder
                            .build_load(llvm_ty, *ptr, "global")
                            .unwrap();
                    }
                }
                self.ctx.context.i64_type().const_int(0, false).into()
            }
            VarRef::SelfRef => {
                // Self is the first parameter for methods
                if let Some(func) = self.ctx.current_function {
                    if let Some(param) = func.get_first_param() {
                        return param;
                    }
                }
                self.ctx
                    .context
                    .ptr_type(Default::default())
                    .const_null()
                    .into()
            }
        }
    }

    pub(crate) fn load_var_ptr(
        &mut self,
        var_ref: &VarRef,
        program: &TirProgram,
    ) -> PointerValue<'ctx> {
        match var_ref {
            VarRef::Local(local_id) => self.locals[local_id.index()].0,
            VarRef::Param(_) => {
                // Parameters need to be stored first - this shouldn't happen with TIR
                unreachable!("Cannot get pointer to parameter")
            }
            VarRef::Global(mod_id, global_id) => {
                let module = program.module(*mod_id);
                let global = &module.globals[global_id.index()];
                let key = format!("{}::{}", module.name, global.name);
                self.ctx.global_variables[&key]
            }
            VarRef::SelfRef => {
                unreachable!("Cannot get pointer to self")
            }
        }
    }

    pub(crate) fn codegen_lvalue(
        &mut self,
        lvalue: &TirLValue,
        program: &TirProgram,
    ) -> PointerValue<'ctx> {
        match lvalue {
            TirLValue::Var(var_ref) => self.load_var_ptr(var_ref, program),
            TirLValue::Field {
                object,
                class,
                field,
            } => {
                let obj_val = self.codegen_expr(object, program);
                let class_def = program.class(*class);
                let class_type = self.ctx.class_types[&class_def.qualified_name];
                let obj_ptr = self.value_to_pointer(obj_val);
                self.ctx
                    .builder
                    .build_struct_gep(class_type, obj_ptr, field.index() as u32, "field_ptr")
                    .unwrap()
            }
        }
    }

    pub(crate) fn store_to_lvalue(
        &mut self,
        lvalue: &TirLValue,
        value: BasicValueEnum<'ctx>,
        program: &TirProgram,
    ) {
        let ptr = self.codegen_lvalue(lvalue, program);
        self.ctx.builder.build_store(ptr, value).unwrap();
    }

    /// Convert a value to a pointer, using int_to_ptr if necessary
    /// TIR types only produce IntValue, FloatValue, or PointerValue - other cases are handled
    /// for exhaustiveness but should never occur with valid TIR.
    pub(crate) fn value_to_pointer(&mut self, value: BasicValueEnum<'ctx>) -> PointerValue<'ctx> {
        match value {
            BasicValueEnum::PointerValue(ptr) => ptr,
            BasicValueEnum::IntValue(int) => {
                // Convert i64 to pointer (for values from list_get, etc.)
                self.ctx
                    .builder
                    .build_int_to_ptr(
                        int,
                        self.ctx.context.ptr_type(Default::default()),
                        "int_to_ptr",
                    )
                    .unwrap()
            }
            // TIR should not produce FloatValue where pointer is expected
            BasicValueEnum::FloatValue(_) => {
                self.ctx.context.ptr_type(Default::default()).const_null()
            }
            // TIR does not produce these types - return null for exhaustiveness
            BasicValueEnum::StructValue(_)
            | BasicValueEnum::ArrayValue(_)
            | BasicValueEnum::VectorValue(_)
            | BasicValueEnum::ScalableVectorValue(_) => {
                self.ctx.context.ptr_type(Default::default()).const_null()
            }
        }
    }

    /// Convert a value to i64, using ptr_to_int or zext if necessary
    /// This is needed for list operations which store all values as i64
    /// TIR types only produce IntValue, FloatValue, or PointerValue - other cases are handled
    /// for exhaustiveness but should never occur with valid TIR.
    pub(crate) fn value_to_i64(
        &mut self,
        value: BasicValueEnum<'ctx>,
    ) -> inkwell::values::IntValue<'ctx> {
        match value {
            BasicValueEnum::IntValue(int) => {
                let i64_type = self.ctx.context.i64_type();
                let i8_type = self.ctx.context.i8_type();
                // Check if it's a smaller int type that needs to be extended
                if int.get_type() == i8_type {
                    // Bool stored as i8 - extend to i64
                    self.ctx
                        .builder
                        .build_int_z_extend(int, i64_type, "i8_to_i64")
                        .unwrap()
                } else if int.get_type() == i64_type {
                    int
                } else {
                    // Generic extension for other int types
                    self.ctx
                        .builder
                        .build_int_z_extend(int, i64_type, "int_to_i64")
                        .unwrap()
                }
            }
            BasicValueEnum::PointerValue(ptr) => {
                // Convert pointer to i64 (for storing in lists)
                self.ctx
                    .builder
                    .build_ptr_to_int(ptr, self.ctx.context.i64_type(), "ptr_to_int")
                    .unwrap()
            }
            BasicValueEnum::FloatValue(float) => {
                // Convert float to i64 using bitcast (preserves bit pattern)
                // This is used for storing floats in lists which use i64 for storage
                self.ctx
                    .builder
                    .build_bit_cast(float, self.ctx.context.i64_type(), "float_to_i64")
                    .unwrap()
                    .into_int_value()
            }
            // TIR does not produce these types - return 0 for exhaustiveness
            BasicValueEnum::StructValue(_)
            | BasicValueEnum::ArrayValue(_)
            | BasicValueEnum::VectorValue(_)
            | BasicValueEnum::ScalableVectorValue(_) => {
                self.ctx.context.i64_type().const_int(0, false)
            }
        }
    }

    /// Convert a value to bool (i1 for branching), comparing to zero if necessary
    /// TIR types only produce IntValue, FloatValue, or PointerValue - other cases are handled
    /// for exhaustiveness but should never occur with valid TIR.
    pub(crate) fn convert_to_bool(
        &mut self,
        value: BasicValueEnum<'ctx>,
    ) -> inkwell::values::IntValue<'ctx> {
        match value {
            BasicValueEnum::IntValue(int) => {
                let i8_type = self.ctx.context.i8_type();
                let i64_type = self.ctx.context.i64_type();
                if int.get_type() == i8_type {
                    // Bool stored as i8 - compare to zero to get i1 for branching
                    self.ctx
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            int,
                            i8_type.const_zero(),
                            "to_bool",
                        )
                        .unwrap()
                } else if int.get_type() == i64_type {
                    // Int value - compare to zero
                    self.ctx
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            int,
                            i64_type.const_zero(),
                            "to_bool",
                        )
                        .unwrap()
                } else {
                    // Generic - compare to zero
                    self.ctx
                        .builder
                        .build_int_compare(
                            inkwell::IntPredicate::NE,
                            int,
                            int.get_type().const_zero(),
                            "to_bool",
                        )
                        .unwrap()
                }
            }
            BasicValueEnum::FloatValue(float) => {
                // Float value - compare to zero
                let f64_type = self.ctx.context.f64_type();
                self.ctx
                    .builder
                    .build_float_compare(
                        inkwell::FloatPredicate::ONE,
                        float,
                        f64_type.const_zero(),
                        "to_bool",
                    )
                    .unwrap()
            }
            BasicValueEnum::PointerValue(ptr) => {
                // Pointer value - check if not null
                self.ctx
                    .builder
                    .build_is_not_null(ptr, "ptr_to_bool")
                    .unwrap()
            }
            // TIR does not produce these types - return false for exhaustiveness
            BasicValueEnum::StructValue(_)
            | BasicValueEnum::ArrayValue(_)
            | BasicValueEnum::VectorValue(_)
            | BasicValueEnum::ScalableVectorValue(_) => {
                self.ctx.context.bool_type().const_int(0, false)
            }
        }
    }

    /// Convert a value to f64 (float), converting from int if necessary
    /// TIR types only produce IntValue, FloatValue, or PointerValue - other cases are handled
    /// for exhaustiveness but should never occur with valid TIR.
    pub(crate) fn convert_to_float(
        &mut self,
        value: BasicValueEnum<'ctx>,
    ) -> inkwell::values::FloatValue<'ctx> {
        match value {
            BasicValueEnum::FloatValue(float) => float,
            BasicValueEnum::IntValue(int) => {
                // Convert signed int to float
                let f64_type = self.ctx.context.f64_type();
                self.ctx
                    .builder
                    .build_signed_int_to_float(int, f64_type, "int_to_float")
                    .unwrap()
            }
            // TIR should not produce PointerValue where float is expected
            BasicValueEnum::PointerValue(_) => self.ctx.context.f64_type().const_zero(),
            // TIR does not produce these types - return 0.0 for exhaustiveness
            BasicValueEnum::StructValue(_)
            | BasicValueEnum::ArrayValue(_)
            | BasicValueEnum::VectorValue(_)
            | BasicValueEnum::ScalableVectorValue(_) => self.ctx.context.f64_type().const_zero(),
        }
    }
}
