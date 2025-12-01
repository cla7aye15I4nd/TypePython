/// Program and function visitor implementation for code generation
use super::super::CodeGen;
use crate::ast::visitor::Visitor;
use crate::ast::*;
use crate::types::{PyType, PyValue};

use inkwell::types::BasicType;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn visit_program_impl(
        &mut self,
        program: &Program,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        // Call visitor method for all imports for coverage
        for import in &program.imports {
            self.visit_import(import)?;
        }

        // External functions are now lazily declared when called

        // Process all class definitions
        for class in &program.classes {
            self.visit_class(class)?;
        }

        // Pre-pass: Create LLVM globals for module-level VarDecl statements
        // This must happen before functions are compiled so they can use global keyword
        for stmt in &program.statements {
            if let Statement::VarDecl { name, var_type, .. } = stmt {
                let py_type = PyType::from_ast_type(var_type)?;
                let llvm_type = py_type.to_llvm(self.cg.ctx);

                // Create a global variable with zero initializer
                let global = self.cg.module.add_global(llvm_type, None, name);
                global.set_initializer(&llvm_type.const_zero());

                // Store in module_vars for lookup by global keyword
                self.module_vars
                    .insert(name.clone(), (global.as_pointer_value(), py_type));
            }
        }

        // First pass: Declare all functions (for mutual recursion support)
        for function in &program.functions {
            self.declare_function(function)?;
        }

        // Second pass: Visit all function declarations to generate bodies
        for function in &program.functions {
            self.visit_function(function)?;
        }

        // Generate a main function that contains all top-level statements
        if !program.statements.is_empty() {
            self.generate_main_function(&program.statements)?;
        }

        Ok(self.cg.ctx.i32_type().const_zero().into())
    }

    pub(crate) fn visit_class_impl(&mut self, class: &Class) -> Result<(), String> {
        // Register the class in our class registry
        self.register_class(class)?;

        // Generate methods first (constructor needs to call __init__)
        for method in &class.methods {
            self.generate_method(class, method)?;
        }

        // Generate constructor (which calls __init__)
        self.generate_class_constructor(class)?;

        Ok(())
    }

    /// Generate the constructor function for a class
    /// The constructor:
    /// 1. Takes arguments matching __init__ (minus self)
    /// 2. Allocates memory for the instance
    /// 3. Calls __init__ with the instance + arguments
    /// 4. Returns the instance
    fn generate_class_constructor(&mut self, class: &Class) -> Result<(), String> {
        let class_name = &class.name;

        // Get field types for struct layout
        let field_types: Vec<PyType> = class
            .fields
            .iter()
            .map(|f| PyType::from_ast_type(&f.field_type))
            .collect::<Result<Vec<_>, _>>()?;

        // Create instance struct type (all fields stored as i64 for uniform access)
        let num_fields = field_types.len();
        let struct_element_type = self.cg.ctx.i64_type();
        let struct_type = self
            .cg
            .ctx
            .struct_type(&vec![struct_element_type.into(); num_fields], false);

        // Find __init__ method to get parameter types
        let init_method = class.methods.iter().find(|m| m.name == "__init__");

        // Constructor parameter types: match __init__ params (skip 'self')
        let init_param_types: Vec<PyType> = if let Some(init) = init_method {
            init.params
                .iter()
                .skip(1) // Skip 'self'
                .map(|p| PyType::from_ast_type(&p.param_type))
                .collect::<Result<Vec<_>, _>>()?
        } else {
            vec![] // No __init__ means no args
        };

        // Constructor function: ClassName_init(init args...) -> ptr
        let constructor_name = format!("__main___{}_init", class_name);

        let param_types: Vec<inkwell::types::BasicMetadataTypeEnum> = init_param_types
            .iter()
            .map(|t| t.to_llvm(self.cg.ctx).into())
            .collect();

        // Return type: pointer to instance
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());
        let fn_type = ptr_type.fn_type(&param_types, false);

        // Create the constructor function
        let function = self
            .cg
            .module
            .add_function(&constructor_name, fn_type, None);
        let entry_bb = self.cg.ctx.append_basic_block(function, "entry");
        self.cg.builder.position_at_end(entry_bb);

        // Allocate memory for the instance
        let instance_ptr = self
            .cg
            .builder
            .build_malloc(struct_type, &format!("{}_instance", class_name))
            .unwrap();

        // Initialize all fields to zero/null
        for i in 0..num_fields {
            let field_ptr = self
                .cg
                .builder
                .build_struct_gep(struct_type, instance_ptr, i as u32, &format!("field_{}", i))
                .unwrap();
            self.cg
                .builder
                .build_store(field_ptr, self.cg.ctx.i64_type().const_zero())
                .unwrap();
        }

        // Call __init__ if it exists
        if init_method.is_some() {
            let init_fn_name = format!("__main___{}___init__", class_name);
            if let Some(init_fn) = self.cg.module.get_function(&init_fn_name) {
                // Build args: self + constructor args
                let mut call_args: Vec<inkwell::values::BasicMetadataValueEnum> =
                    vec![instance_ptr.into()];
                for i in 0..init_param_types.len() {
                    call_args.push(function.get_nth_param(i as u32).unwrap().into());
                }
                self.cg
                    .builder
                    .build_call(init_fn, &call_args, "init_call")
                    .unwrap();
            }
        }

        // Return the instance pointer
        self.cg.builder.build_return(Some(&instance_ptr)).unwrap();

        Ok(())
    }

    /// Convert a BasicValueEnum to i64 for uniform instance field storage
    #[allow(dead_code)]
    fn convert_to_i64(&self, value: BasicValueEnum<'ctx>) -> inkwell::values::IntValue<'ctx> {
        match value {
            BasicValueEnum::IntValue(iv) => {
                if iv.get_type().get_bit_width() == 64 {
                    iv
                } else if iv.get_type().get_bit_width() == 1 {
                    // Bool
                    self.cg
                        .builder
                        .build_int_z_extend(iv, self.cg.ctx.i64_type(), "bool_to_i64")
                        .unwrap()
                } else {
                    self.cg
                        .builder
                        .build_int_s_extend(iv, self.cg.ctx.i64_type(), "int_to_i64")
                        .unwrap()
                }
            }
            BasicValueEnum::FloatValue(fv) => self
                .cg
                .builder
                .build_bit_cast(fv, self.cg.ctx.i64_type(), "float_to_i64")
                .unwrap()
                .into_int_value(),
            BasicValueEnum::PointerValue(pv) => self
                .cg
                .builder
                .build_ptr_to_int(pv, self.cg.ctx.i64_type(), "ptr_to_i64")
                .unwrap(),
            _ => panic!("Unsupported value type for class field"),
        }
    }

    /// Generate a method function for a class
    fn generate_method(&mut self, class: &Class, method: &Method) -> Result<(), String> {
        let class_name = &class.name;
        let method_name = &method.name;

        // Method name: ClassName_method_name
        let fn_name = format!("__main___{}_{}", class_name, method_name);

        // Get return type
        let return_py_type = PyType::from_ast_type(&method.return_type)?;
        let return_llvm_type = return_py_type.to_llvm(self.cg.ctx);

        // Build parameter types - first is always self (ptr), then other params
        let ptr_type = self.cg.ctx.ptr_type(inkwell::AddressSpace::default());
        let mut param_types: Vec<inkwell::types::BasicMetadataTypeEnum> = vec![ptr_type.into()];

        // Skip 'self' parameter and add the rest
        for param in method.params.iter().skip(1) {
            let py_type = PyType::from_ast_type(&param.param_type)?;
            param_types.push(py_type.to_llvm(self.cg.ctx).into());
        }

        // Create function type
        let fn_type = if matches!(return_py_type, PyType::None) {
            self.cg.ctx.void_type().fn_type(&param_types, false)
        } else {
            return_llvm_type.fn_type(&param_types, false)
        };

        // Create or get the function
        let function = self.cg.module.add_function(&fn_name, fn_type, None);

        // Save current function and variables
        let prev_function = self.current_function;
        let prev_variables = std::mem::take(&mut self.variables);

        self.current_function = Some(function);

        // Create entry block
        let entry_bb = self.cg.ctx.append_basic_block(function, "entry");
        self.cg.builder.position_at_end(entry_bb);

        // Set up 'self' parameter
        let self_param = function.get_nth_param(0).unwrap();
        let instance_type = PyType::Instance(crate::types::InstanceType::new(
            class_name.clone(),
            class
                .fields
                .iter()
                .map(|f| Ok((f.name.clone(), PyType::from_ast_type(&f.field_type)?)))
                .collect::<Result<Vec<_>, String>>()?,
        ));
        let self_value = PyValue::new(self_param, instance_type, None);
        self.variables.insert("self".to_string(), self_value);

        // Set up other parameters (skip 'self' which is handled specially)
        for (i, param) in method.params.iter().skip(1).enumerate() {
            let param_value = function.get_nth_param((i + 1) as u32).unwrap();
            let py_type = PyType::from_ast_type(&param.param_type)?;
            let alloca = self.create_entry_block_alloca(method_name, &param.name, &py_type);
            self.cg.builder.build_store(alloca, param_value).unwrap();
            let var = PyValue::new(param_value, py_type, Some(alloca));
            self.variables.insert(param.name.clone(), var);
        }

        // Generate method body
        for stmt in &method.body {
            self.visit_statement(stmt)?;
        }

        // Add default return if needed
        if !self.is_block_terminated() {
            if matches!(return_py_type, PyType::None) {
                self.cg.builder.build_return(None).unwrap();
            } else {
                let zero = return_llvm_type.const_zero();
                self.cg.builder.build_return(Some(&zero)).unwrap();
            }
        }

        // Restore previous state
        self.current_function = prev_function;
        self.variables = prev_variables;

        Ok(())
    }

    pub(crate) fn enter_function_impl(&mut self, func: &Function) -> Result<(), String> {
        // Get the already-declared function
        let mangled_name = self.mangle_function_name(&self.module_name, &func.name);
        let function = self.cg.module.get_function(&mangled_name).ok_or_else(|| {
            format!(
                "Function {} not found (should have been declared in first pass)",
                func.name
            )
        })?;
        self.current_function = Some(function);

        let entry_bb = self.cg.ctx.append_basic_block(function, "entry");
        self.cg.builder.position_at_end(entry_bb);

        // Clear variables for new function scope
        self.variables.clear();
        // Clear global/nonlocal declarations for new function scope
        self.global_vars.clear();
        self.nonlocal_vars.clear();

        // Allocate space for parameters and store them
        for (i, param) in func.params.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let py_type = PyType::from_ast_type(&param.param_type)?;
            let alloca = self.create_entry_block_alloca(&func.name, &param.name, &py_type);
            self.cg.builder.build_store(alloca, param_value).unwrap();
            let var = PyValue::new(param_value, py_type, Some(alloca));
            self.variables.insert(param.name.clone(), var);
        }

        Ok(())
    }

    pub(crate) fn exit_function_impl(&mut self, func: &Function) -> Result<(), String> {
        // Add default return if needed
        if !self.is_block_terminated() {
            match func.return_type {
                Type::None => {
                    self.cg.builder.build_return(None).unwrap();
                }
                Type::Int => {
                    let zero = self.cg.ctx.i64_type().const_zero();
                    self.cg.builder.build_return(Some(&zero)).unwrap();
                }
                Type::Float => {
                    let zero = self.cg.ctx.f64_type().const_zero();
                    self.cg.builder.build_return(Some(&zero)).unwrap();
                }
                Type::Bool => {
                    let zero = self.cg.ctx.bool_type().const_zero();
                    self.cg.builder.build_return(Some(&zero)).unwrap();
                }
                Type::Str
                | Type::Bytes
                | Type::List(_)
                | Type::Dict(_, _)
                | Type::Set(_)
                | Type::Tuple(_)
                | Type::Custom(_)
                | Type::Range => {
                    // Pointer types return null by default
                    let null = self
                        .cg
                        .ctx
                        .ptr_type(inkwell::AddressSpace::default())
                        .const_null();
                    self.cg.builder.build_return(Some(&null)).unwrap();
                }
            }
        }

        Ok(())
    }
}
