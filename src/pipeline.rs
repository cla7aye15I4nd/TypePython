/// Compilation pipeline module - shared logic for compiling TypePython programs
use crate::codegen::builtins::get_runtime_object_path;
use crate::codegen::CodeGen;
use crate::module::ModuleRegistry;
use inkwell::context::Context;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

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

    // Link against math library for functions like round(), pow(), etc.
    cmd.arg("-lm");

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run linker: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Linking failed: {}", stderr))
    }
}

/// Full pipeline: compile source file and all imports to executable
///
/// This function:
/// 1. Compiles current file to AST and recursively resolves imports to collect all files
/// 2. Creates a function map and compiles each AST to corresponding .o file
/// 3. Links all .o files with the unified runtime.o into executable
pub fn compile(source_path: &Path, output_path: &Path) -> Result<(), String> {
    let context = Context::create();

    // Step 1: Preprocess all modules - discover and register all modules via BFS
    let module_registry = ModuleRegistry::new(
        &context,
        source_path.parent().unwrap().to_path_buf(),
        source_path,
    )?;

    // Step 2: Compile all modules to LLVM IR and .o files
    let temp_dir =
        TempDir::new().map_err(|e| format!("Failed to create temporary directory: {}", e))?;
    let cache_dir = temp_dir.path();

    let mut object_files = Vec::new();

    for (module_name, module_value) in module_registry.modules.iter() {
        let module = module_registry
            .programs
            .get(module_name)
            .ok_or_else(|| format!("Program not found for module '{}'", module_name))?;

        // Convert Python AST Module to legacy Program
        let program = crate::ast::Program::from_module(module)?;

        let module_info = module_value.module_info();
        let global_variables: HashMap<String, crate::types::PyValue> = module_info
            .members
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let mut codegen = CodeGen::new(&context, module_name);
        codegen.set_global_variables(global_variables);
        codegen.generate(&program)?;

        codegen
            .get_module()
            .verify()
            .map_err(|e| format!("Module '{}' verification failed: {}", module_name, e))?;

        let obj_filename = format!("{}.o", module_name);
        let obj_path = cache_dir.join(obj_filename);
        codegen.get_module().write_bitcode_to_path(&obj_path);
        object_files.push(obj_path);
    }

    // Step 3: Add the unified runtime.o (LTO will eliminate unused functions)
    let runtime_obj = get_runtime_object_path();
    if !runtime_obj.exists() {
        return Err(format!(
            "Runtime object file not found at {}",
            runtime_obj.display()
        ));
    }
    object_files.push(runtime_obj);

    // Step 4: Link all .o files into executable
    link_object_files(&object_files, output_path)?;

    Ok(())
}
