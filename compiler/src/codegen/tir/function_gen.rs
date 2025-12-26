use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, PointerValue};

use crate::codegen::context::CodegenContext;
use crate::tir::decls::TirFunction;
use crate::tir::{TirModule, TirProgram, TirType};

pub(crate) struct FunctionGenContext<'ctx, 'a> {
    /// The codegen context
    pub(crate) ctx: &'a mut CodegenContext<'ctx>,

    /// Local variables indexed by LocalId (pointer and its LLVM type)
    pub(crate) locals: Vec<(PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,

    /// Parameters as values (not pointers)
    pub(crate) params: Vec<BasicValueEnum<'ctx>>,
}

impl<'ctx> CodegenContext<'ctx> {
    pub(crate) fn codegen_tir_function(&mut self, func: &TirFunction, program: &TirProgram) {
        // Skip runtime functions - they have no body to codegen
        if func.runtime_name.is_some() {
            return;
        }

        let fn_value = self.functions[&func.qualified_name];

        // Create entry block
        let entry = self.context.append_basic_block(fn_value, "entry");
        self.builder.position_at_end(entry);

        self.current_function = Some(fn_value);

        // Set up current class context if this is a method
        if let Some(class_id) = func.class {
            let class = program.class(class_id);
            self.current_class = Some(class.qualified_name.clone());
        }

        // Allocate local variables
        let mut locals: Vec<(PointerValue<'ctx>, BasicTypeEnum<'ctx>)> = Vec::new();
        for (name, ty) in &func.locals {
            let llvm_ty = self.tir_type_to_llvm(ty, program);
            let ptr = self.builder.build_alloca(llvm_ty, name).unwrap();
            locals.push((ptr, llvm_ty));
        }

        // Collect parameters
        let mut params: Vec<BasicValueEnum<'ctx>> = Vec::new();
        for (i, (_, _)) in func.params.iter().enumerate() {
            let param_offset = if func.class.is_some() { 1 } else { 0 };
            let param_value = fn_value.get_nth_param((i + param_offset) as u32).unwrap();
            params.push(param_value);
        }

        // Generate body
        let mut fn_ctx = FunctionGenContext {
            ctx: self,
            locals,
            params,
        };

        for stmt in &func.body {
            fn_ctx.codegen_stmt(stmt, program);
        }

        // Only void functions need implicit return terminators.
        // Non-void functions must have explicit returns on all paths (validated during TIR lowering).
        if func.return_type == TirType::Void {
            self.add_missing_terminators();
        }

        self.current_function = None;
        self.current_class = None;
    }

    pub(crate) fn generate_tir_module_init(&mut self, module: &TirModule, program: &TirProgram) {
        let void_type = self.context.void_type();
        let fn_type = void_type.fn_type(&[], false);
        let init_name = format!("__pyc_init_{}", module.name.replace('.', "_"));
        let function = self.module.add_function(&init_name, fn_type, None);

        self.current_function = Some(function);

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // Globals are already declared in declare_tir_module_globals

        // Allocate local variables for module init (e.g., for loop temporaries)
        let mut locals: Vec<(PointerValue<'ctx>, BasicTypeEnum<'ctx>)> = Vec::new();
        for (name, ty) in &module.init_locals {
            let llvm_ty = self.tir_type_to_llvm(ty, program);
            let ptr = self.builder.build_alloca(llvm_ty, name).unwrap();
            locals.push((ptr, llvm_ty));
        }

        let mut fn_ctx = FunctionGenContext {
            ctx: self,
            locals,
            params: Vec::new(),
        };

        for stmt in &module.init_body {
            fn_ctx.codegen_stmt(stmt, program);
        }

        // Only add return if the current block doesn't already have a terminator
        // (e.g., if the last statement was a raise/unreachable)
        if let Some(current_block) = self.builder.get_insert_block() {
            if current_block.get_terminator().is_none() {
                self.builder.build_return(None).unwrap();
            }
        }
        self.current_function = None;
    }

    pub(crate) fn generate_tir_main(&mut self, program: &TirProgram) {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // Call all module init functions in order (they are already sorted by dependency)
        // This ensures globals are initialized before any function tries to use them
        for module in &program.modules {
            let init_name = format!("__pyc_init_{}", module.name.replace('.', "_"));
            if let Some(module_init) = self.module.get_function(&init_name) {
                self.builder.build_call(module_init, &[], "").unwrap();
            }
        }

        // Return 0
        let zero = i32_type.const_int(0, false);
        self.builder.build_return(Some(&zero)).unwrap();
    }

    /// Add implicit return terminators to basic blocks that don't have one.
    /// This is only valid for void functions - non-void functions must have
    /// explicit returns on all paths (validated during TIR lowering).
    pub(crate) fn add_missing_terminators(&mut self) {
        if let Some(func) = self.current_function {
            // Collect blocks that need terminators
            let mut blocks_needing_terminator = Vec::new();
            let mut block = func.get_first_basic_block();
            while let Some(bb) = block {
                if bb.get_terminator().is_none() {
                    blocks_needing_terminator.push(bb);
                }
                block = bb.get_next_basic_block();
            }

            // Add implicit void returns
            for bb in blocks_needing_terminator {
                self.builder.position_at_end(bb);
                self.builder.build_return(None).unwrap();
            }
        }
    }
}
