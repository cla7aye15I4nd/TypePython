/// Compilation pipeline module - shared logic for compiling TypePython programs
use crate::ast::parser;
use crate::ast::Program;
use crate::codegen::CodeGen;
use crate::module::ModuleRegistry;
use crate::preprocessor;
use crate::{LangParser, Parser, Rule};
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

/// Result of compilation
pub struct CompileResult<'ctx> {
    pub program: Program,
    pub codegen: CodeGen<'ctx>,
}

/// Complete compilation pipeline from source to LLVM module
pub fn compile_source<'ctx>(
    source: &str,
    module_name: &str,
    context: &'ctx Context,
    options: &CompileOptions,
) -> Result<CompileResult<'ctx>, String> {
    compile_source_with_imports(source, module_name, context, options, &[])
}

/// Complete compilation pipeline from source to LLVM module with imported modules
pub fn compile_source_with_imports<'ctx>(
    source: &str,
    module_name: &str,
    context: &'ctx Context,
    options: &CompileOptions,
    imported_programs: &[Program],
) -> Result<CompileResult<'ctx>, String> {
    // Step 1: Preprocess - convert indentation to explicit tokens
    debug!("Preprocessing source");
    let preprocessed = preprocessor::preprocess(source)?;

    if options.dump_preprocessed {
        let dump_path = format!("{}_preprocessed.txt", module_name);
        std::fs::write(&dump_path, &preprocessed)
            .map_err(|e| format!("Failed to dump preprocessed source: {}", e))?;
        debug!("Dumped preprocessed source to {}", dump_path);
    }

    // Step 2: Parse with Pest
    debug!("Parsing with PEST");
    let pairs = LangParser::parse(Rule::program, &preprocessed)
        .map_err(|e| format!("Parse error: {}", e))?;

    if options.dump_pest {
        let dump_path = format!("{}_pest.txt", module_name);
        let pest_output = pairs
            .clone()
            .map(|pair| format!("{:#?}", pair))
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(&dump_path, &pest_output)
            .map_err(|e| format!("Failed to dump PEST parse tree: {}", e))?;
        debug!("Dumped PEST parse tree to {}", dump_path);
    }

    // Step 3: Build AST
    debug!("Building AST");
    let program = parser::build_program(pairs);

    if options.dump_ast {
        let dump_path = format!("{}_ast.txt", module_name);
        let ast_output = format!("{:#?}", program);
        std::fs::write(&dump_path, &ast_output)
            .map_err(|e| format!("Failed to dump AST: {}", e))?;
        debug!("Dumped AST to {}", dump_path);
    }

    // Step 4: Generate LLVM IR
    debug!("Generating LLVM IR");
    let mut codegen = CodeGen::new(context, module_name);

    // Add imported modules to codegen (old API without module names)
    for imported_prog in imported_programs {
        codegen.add_imported_module("unknown".to_string(), imported_prog.clone());
    }

    codegen.generate(&program)?;

    if options.dump_ir {
        let dump_path = format!("{}_ir.ll", module_name);
        let ir_output = codegen.get_module().print_to_string().to_string();
        std::fs::write(&dump_path, &ir_output)
            .map_err(|e| format!("Failed to dump LLVM IR: {}", e))?;
        debug!("Dumped LLVM IR to {}", dump_path);
    }

    // Step 5: Verify LLVM module
    debug!("Verifying LLVM module");
    codegen
        .get_module()
        .verify()
        .map_err(|e| format!("Module verification failed: {}", e))?;

    Ok(CompileResult { program, codegen })
}

/// Compile a file to LLVM bitcode
pub fn compile_file<'ctx>(
    path: &Path,
    context: &'ctx Context,
    options: &CompileOptions,
) -> Result<CompileResult<'ctx>, String> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| format!("Error reading {}: {}", path.display(), e))?;

    let module_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;

    compile_source(&source, module_name, context, options)
}

/// Write LLVM bitcode to file
pub fn write_bitcode(codegen: &CodeGen, output_path: &Path) -> Result<(), String> {
    codegen.get_module().write_bitcode_to_path(output_path);
    Ok(())
}

/// Compile LLVM bitcode to LTO object file (.o containing bitcode)
/// These .o files contain LLVM bitcode for link-time optimization
pub fn compile_bitcode_to_lto_object(
    bitcode_path: &Path,
    object_path: &Path,
) -> Result<(), String> {
    ModuleRegistry::compile_bitcode_to_lto_object(bitcode_path, object_path)
}

/// Compile bitcode to executable using Clang
pub fn compile_to_executable(bitcode_path: &Path, output_path: &Path) -> Result<(), String> {
    let clang = crate::module::get_clang_path();

    debug!(
        "Linking with clang: {} -> {}",
        bitcode_path.display(),
        output_path.display()
    );

    let mut cmd = std::process::Command::new(clang);
    cmd.arg("-Wno-override-module").arg(bitcode_path);

    cmd.arg("-o").arg(output_path).arg("-lm"); // Link math library for pow

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute clang: {}", e))?;

    if !status.success() {
        return Err("Compilation failed".to_string());
    }

    Ok(())
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

    let module_registry =
        ModuleRegistry::new(&context, source_path.parent().unwrap().to_path_buf());

    // Use BFS to discover all modules, generate their names, and build symbol maps
    let module_data = module_registry.preprocess_modules(source_path)?;

    debug!("Discovered and preprocessed {} modules", module_data.len());

    // ============================================================================
    // Step 2: Compile all modules to LLVM IR and .o files
    // ============================================================================
    let temp_dir = std::env::temp_dir().join(format!("tpy_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;

    let mut object_files = Vec::new();

    // Compile each module
    for (module_path, (module_name, program, imported_symbols)) in &module_data {
        // Check if it's a C file
        if module_path.extension().and_then(|s| s.to_str()) == Some("c") {
            // Compile C file to .o
            let source_dir = module_path
                .parent()
                .ok_or_else(|| format!("Invalid source path: {}", module_path.display()))?;
            let cache_dir = source_dir.join("__tpycache__");
            std::fs::create_dir_all(&cache_dir)
                .map_err(|e| format!("Failed to create __tpycache__ directory: {}", e))?;

            let obj_path = module_registry.compile_c_module(module_path, &temp_dir, &cache_dir)?;
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
                for (other_module_name, other_program, _) in module_data.values() {
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

    // Clean up temp directory
    let _ = std::fs::remove_dir_all(&temp_dir);

    Ok(())
}
