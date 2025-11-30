// Include the auto-generated builtin table from build.rs
include!(concat!(env!("OUT_DIR"), "/builtins_generated.rs"));

/// Get the path to the builtin build directory where .o files are stored
pub fn get_builtin_build_dir() -> std::path::PathBuf {
    // This is set by build.rs
    let dir = env!("TYPEPYTHON_BUILTIN_BUILD_DIR");
    std::path::PathBuf::from(dir)
}

/// Get the path to a specific builtin module's object file
pub fn get_builtin_object_path(module_name: &str) -> std::path::PathBuf {
    get_builtin_build_dir().join(format!("{}.o", module_name))
}

impl BuiltinFunction {
    /// Convert a parameter type to an LLVM type
    pub fn param_type_to_llvm<'ctx>(
        param_type: &BuiltinType,
        context: &'ctx inkwell::context::Context,
    ) -> inkwell::types::BasicMetadataTypeEnum<'ctx> {
        match param_type {
            BuiltinType::I64 => context.i64_type().into(),
            BuiltinType::I32 => context.i32_type().into(),
            BuiltinType::I8 => context.i8_type().into(),
            BuiltinType::F64 => context.f64_type().into(),
            BuiltinType::Bool => context.bool_type().into(),
            BuiltinType::Ptr => context.ptr_type(inkwell::AddressSpace::default()).into(),
            _ => panic!("Unsupported parameter type: {:?}", param_type),
        }
    }

    /// Create an LLVM function type from this builtin function's signature
    pub fn to_llvm_fn_type<'ctx>(
        &self,
        context: &'ctx inkwell::context::Context,
    ) -> inkwell::types::FunctionType<'ctx> {
        let param_types: Vec<inkwell::types::BasicMetadataTypeEnum> = self
            .params
            .iter()
            .map(|p| Self::param_type_to_llvm(p, context))
            .collect();

        match self.return_type {
            BuiltinType::Void => context.void_type().fn_type(&param_types, false),
            BuiltinType::I64 => context.i64_type().fn_type(&param_types, false),
            BuiltinType::I32 => context.i32_type().fn_type(&param_types, false),
            BuiltinType::I8 => context.i8_type().fn_type(&param_types, false),
            BuiltinType::F64 => context.f64_type().fn_type(&param_types, false),
            BuiltinType::Ptr => context
                .ptr_type(inkwell::AddressSpace::default())
                .fn_type(&param_types, false),
            _ => panic!("Unsupported return type: {:?}", self.return_type),
        }
    }
}
