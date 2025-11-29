/// Compilation pipeline module - shared logic for compiling TypePython programs
use crate::codegen::builtins::get_builtin_object_path;
use crate::codegen::CodeGen;
use crate::module::ModuleRegistry;
use inkwell::context::Context;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Options for the compilation pipeline
#[derive(Debug, Default)]
pub struct CompileOptions {
    pub dump_preprocessed: bool,
    pub dump_pest: bool,
    pub dump_ast: bool,
    pub dump_ir: bool,
}

/// Link multiple object files together into an executable using LTO
pub fn link_object_files(object_files: &[PathBuf], output_path: &Path) -> Result<(), String> {
    let clang = crate::module::get_clang_path();

    let mut cmd = std::process::Command::new(clang);

    // Enable LTO (Link-Time Optimization)
    cmd.arg("-flto");
    cmd.arg("-O2");

    // Add all object files
    for obj_file in object_files {
        cmd.arg(obj_file);
    }

    cmd.arg("-o").arg(output_path);

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute clang: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "Linking failed with exit code: {}\nStdout: {}\nStderr: {}",
            output.status.code().unwrap_or(-1),
            stdout,
            stderr
        ));
    }

    Ok(())
}

/// Full pipeline: compile source file and all imports to executable
///
/// This function:
/// 1. Compiles current file to AST and recursively resolves imports to collect all files
/// 2. Creates a function map and compiles each AST to corresponding .o file
/// 3. Collects used builtin modules from all compiled code
/// 4. Links all .o files (including only required builtin modules) into a single executable
pub fn compile(
    source_path: &Path,
    output_path: &Path,
    _options: &CompileOptions,
) -> Result<(), String> {
    let context = Context::create();

    // ============================================================================
    // Step 1: Preprocess all modules - discover and register all modules via BFS
    // ============================================================================

    let module_registry = ModuleRegistry::new(
        &context,
        source_path.parent().unwrap().to_path_buf(),
        source_path,
    )?;

    // ============================================================================
    // Step 2: Compile all modules to LLVM IR and .o files
    // ============================================================================

    // Create a temporary directory for compilation artifacts
    // This directory will be automatically cleaned up when it goes out of scope
    let temp_dir =
        TempDir::new().map_err(|e| format!("Failed to create temporary directory: {}", e))?;
    let cache_dir = temp_dir.path();

    let mut object_files = Vec::new();

    // Track all used builtin modules across all compiled code
    let mut all_used_builtins: HashSet<String> = HashSet::new();

    // Compile each module
    for (module_name, module_value) in module_registry.modules.iter() {
        let program = module_registry
            .programs
            .get(module_name)
            .ok_or_else(|| format!("Program not found for module '{}'", module_name))?;

        // For Python modules, compile to LLVM IR

        // Build global variables map from module's imported modules
        let module_info = module_value.module_info();
        let global_variables: HashMap<String, crate::types::PyValue> = module_info
            .members
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Generate LLVM IR for this module
        let mut codegen = CodeGen::new(&context, module_name);
        codegen.set_global_variables(global_variables);
        codegen.generate(program)?;

        // Collect used builtin modules from this codegen
        all_used_builtins.extend(codegen.used_builtin_modules.iter().cloned());

        // Verify LLVM module
        codegen
            .get_module()
            .verify()
            .map_err(|e| format!("Module '{}' verification failed: {}", module_name, e))?;

        // Write to .o file in the unique cache directory
        let obj_filename = format!("{}.o", module_name);
        let obj_path = cache_dir.join(obj_filename);

        // LLVM bitcode can be directly written to .o files for LTO
        codegen.get_module().write_bitcode_to_path(&obj_path);
        object_files.push(obj_path);

        // Log what we've done
    }

    // ============================================================================
    // Step 3: Add only the builtin modules that are actually used
    // ============================================================================

    for builtin_module in &all_used_builtins {
        let builtin_obj_path = get_builtin_object_path(builtin_module);
        if builtin_obj_path.exists() {
            object_files.push(builtin_obj_path);
        } else {
            return Err(format!(
                "Builtin module '{}' object file not found at {}",
                builtin_module,
                builtin_obj_path.display()
            ));
        }
    }

    // ============================================================================
    // Step 4: Link all .o files into executable
    // ============================================================================

    link_object_files(&object_files, output_path)?;

    Ok(())
}
