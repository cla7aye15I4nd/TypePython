/// Compilation pipeline module - shared logic for compiling TypePython programs
use crate::codegen::CodeGen;
use crate::module::ModuleRegistry;
use inkwell::context::Context;
use log::debug;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

    debug!("Linking object files with LTO: {:?}", object_files);

    let mut cmd = std::process::Command::new(clang);

    // Enable LTO (Link-Time Optimization)
    cmd.arg("-flto");
    cmd.arg("-O2");

    // Add all object files
    for obj_file in object_files {
        cmd.arg(obj_file);
    }

    cmd.arg("-o").arg(output_path).arg("-lm");

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute clang: {}", e))?;

    if !status.success() {
        return Err("Linking failed".to_string());
    }

    debug!("Successfully linked with LTO to {}", output_path.display());

    Ok(())
}

/// Full pipeline: compile source file and all imports to executable
///
/// This function:
/// 1. Compiles current file to AST and recursively resolves imports to collect all files
/// 2. Creates a function map and compiles each AST to corresponding .o file
/// 3. Handles .c files specially (compiled directly to .o without AST)
/// 4. Links all .o files into a single executable
pub fn compile(
    source_path: &Path,
    output_path: &Path,
    _options: &CompileOptions,
) -> Result<(), String> {
    let context = Context::create();
    debug!("Starting compilation pipeline");

    // ============================================================================
    // Step 1: Preprocess all modules - discover and register all modules via BFS
    // ============================================================================

    let module_registry = ModuleRegistry::new(
        &context,
        source_path.parent().unwrap().to_path_buf(),
        source_path,
    )?;

    debug!(
        "Discovered and preprocessed {} modules",
        module_registry.module_data.len()
    );

    // ============================================================================
    // Step 2: Compile all modules to LLVM IR and .o files
    // ============================================================================
    let mut object_files = Vec::new();

    // Compile each module
    for (module_path, (module_name, program, imported_symbols)) in &module_registry.module_data {
        // Check if it's a C file
        if module_path.extension().and_then(|s| s.to_str()) == Some("c") {
            // Compile C file to .o
            let source_dir = module_path
                .parent()
                .ok_or_else(|| format!("Invalid source path: {}", module_path.display()))?;
            let cache_dir = source_dir.join("__tpycache__");
            std::fs::create_dir_all(&cache_dir)
                .map_err(|e| format!("Failed to create __tpycache__ directory: {}", e))?;

            let obj_path = module_registry.compile_c_module(module_path, &cache_dir)?;
            object_files.push(obj_path);
            continue;
        }

        // For Python modules, compile to LLVM IR
        debug!("Compiling module '{}' to LLVM IR", module_name);

        // Collect imported programs with their module names
        let mut imported_modules_data = Vec::new();
        for imported_symbol in imported_symbols.values() {
            if let crate::module::ImportedSymbol::Module(real_module_name) = imported_symbol {
                // Find the imported module's program
                for (other_module_name, other_program, _) in module_registry.module_data.values() {
                    if other_module_name == real_module_name {
                        imported_modules_data
                            .push((real_module_name.clone(), other_program.clone()));
                        break;
                    }
                }
            }
        }

        // Generate LLVM IR for this module
        let mut codegen = CodeGen::new(&context, module_name);

        // Set imported symbols for name mangling
        let imported_symbols_map: HashMap<String, String> = imported_symbols
            .iter()
            .filter_map(|(local_name, symbol)| {
                if let crate::module::ImportedSymbol::Module(real_name) = symbol {
                    Some((local_name.clone(), real_name.clone()))
                } else {
                    None
                }
            })
            .collect();
        codegen.set_imported_symbols(imported_symbols_map);

        for (mod_name, imported_prog) in imported_modules_data {
            codegen.add_imported_module(mod_name, imported_prog);
        }
        codegen.generate(program)?;

        // Verify LLVM module
        codegen
            .get_module()
            .verify()
            .map_err(|e| format!("Module '{}' verification failed: {}", module_name, e))?;

        // Write directly to .o file (bitcode format)
        let source_dir = module_path
            .parent()
            .ok_or_else(|| format!("Invalid source path: {}", module_path.display()))?;
        let cache_dir = source_dir.join("__tpycache__");
        std::fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create __tpycache__ directory: {}", e))?;

        let obj_filename = format!("{}.o", module_name);
        let obj_path = cache_dir.join(obj_filename);

        // LLVM bitcode can be directly written to .o files for LTO
        codegen.get_module().write_bitcode_to_path(&obj_path);
        object_files.push(obj_path);
    }

    // ============================================================================
    // Step 3: Link all .o files into executable
    // ============================================================================
    debug!("Linking {} object files", object_files.len());
    link_object_files(&object_files, output_path)?;

    debug!(
        "Compilation complete! Executable: {}",
        output_path.display()
    );
    debug!("Object files cached in __tpycache__ directories");

    Ok(())
}
