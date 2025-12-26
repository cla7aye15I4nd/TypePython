use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{AnyValue, BasicValueEnum};

use crate::codegen::context::CodegenContext;
use crate::tir::decls::{TirClass, TirFunction};
use crate::tir::{TirModule, TirProgram, TirType};

/// Helper to extract BasicValueEnum from a call site
pub(crate) fn call_result_to_basic_value<'ctx>(
    call_site: inkwell::values::CallSiteValue<'ctx>,
    default: BasicValueEnum<'ctx>,
) -> BasicValueEnum<'ctx> {
    let any_value = call_site.as_any_value_enum();
    match any_value {
        inkwell::values::AnyValueEnum::IntValue(v) => v.into(),
        inkwell::values::AnyValueEnum::PointerValue(v) => v.into(),
        inkwell::values::AnyValueEnum::FloatValue(v) => v.into(),
        _ => default,
    }
}

impl<'ctx> CodegenContext<'ctx> {
    pub(crate) fn declare_tir_module_globals(&mut self, module: &TirModule, program: &TirProgram) {
        for global in &module.globals {
            let llvm_ty = self.tir_type_to_llvm(&global.ty, program);
            let global_name = format!("{}_{}", module.name.replace('.', "_"), global.name);
            let global_var = self.module.add_global(llvm_ty, None, &global_name);
            global_var.set_initializer(&llvm_ty.const_zero());
            // Use module-qualified name as key to avoid collisions
            let key = format!("{}::{}", module.name, global.name);
            self.global_variables
                .insert(key, global_var.as_pointer_value());
        }
    }

    pub(crate) fn declare_tir_class(&mut self, class: &TirClass, program: &TirProgram) {
        // Create the struct type with all fields (inherited first, then own)
        let field_types: Vec<BasicTypeEnum<'ctx>> = class
            .all_fields()
            .map(|(_, ty)| self.tir_type_to_llvm(ty, program))
            .collect();

        let struct_type = self.context.opaque_struct_type(&class.qualified_name);
        struct_type.set_body(&field_types, false);

        // Store by qualified name
        self.class_types
            .insert(class.qualified_name.clone(), struct_type);
    }

    pub(crate) fn declare_tir_function(&mut self, func: &TirFunction, program: &TirProgram) {
        // Skip runtime functions - they're already declared by the runtime
        if func.runtime_name.is_some() {
            return;
        }

        // Build parameter types
        let mut param_types: Vec<BasicTypeEnum<'ctx>> = Vec::new();

        // If it's a method, add self parameter
        if func.class.is_some() {
            param_types.push(self.context.ptr_type(Default::default()).into());
        }

        // Add regular parameters
        for (_, ty) in &func.params {
            param_types.push(self.tir_type_to_llvm(ty, program));
        }

        // Build return type
        let return_type = self.tir_type_to_llvm(&func.return_type, program);

        // Create the function type
        let fn_type = if func.return_type == TirType::Void {
            self.context.void_type().fn_type(
                &param_types.iter().map(|t| (*t).into()).collect::<Vec<_>>(),
                false,
            )
        } else {
            return_type.fn_type(
                &param_types.iter().map(|t| (*t).into()).collect::<Vec<_>>(),
                false,
            )
        };

        // Create the LLVM function
        let llvm_name = format!("__pyc_{}", func.qualified_name.replace('.', "_"));
        let fn_value = self.module.add_function(&llvm_name, fn_type, None);

        // Store it
        self.functions.insert(func.qualified_name.clone(), fn_value);
        self.functions.insert(func.name.clone(), fn_value);
    }
    pub(crate) fn tir_type_to_llvm(
        &self,
        ty: &TirType,
        _program: &TirProgram,
    ) -> BasicTypeEnum<'ctx> {
        match ty {
            TirType::Int => self.context.i64_type().into(),
            TirType::Float => self.context.f64_type().into(), // Use f64 for double precision
            TirType::Bool => self.context.i8_type().into(),   // Use i8 for C ABI compatibility
            TirType::Void => self.context.i64_type().into(),  // Use i64 as placeholder for void
            TirType::Class(_class_id) => {
                // All classes (including Bytes, Str) are represented as pointers
                self.context.ptr_type(Default::default()).into()
            } // NOTE: TirType no longer has TypeVar variant - type inference is enforced
                                                               // at compile-time by the two-layer TIR architecture (TirTypeUnresolved -> TirType)
        }
    }
}
