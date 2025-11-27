/// Compilation pipeline module - shared logic for compiling TypePython programs
use crate::ast::parser;
use crate::ast::Program;
use crate::codegen::CodeGen;
use crate::module::ModuleRegistry;
use crate::preprocessor;
use crate::{LangParser, Parser, Rule};
use inkwell::context::Context;
use log::debug;
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

    // Add imported modules to codegen
    for imported_prog in imported_programs {
        codegen.add_imported_module(imported_prog.clone());
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
    let llvm_prefix = std::env::var("LLVM_SYS_211_PREFIX")
        .map_err(|_| "LLVM_SYS_211_PREFIX environment variable is not set".to_string())?;

    let clang = format!("{}/bin/clang", llvm_prefix);

    debug!(
        "Compiling bitcode to LTO object: {} -> {}",
        bitcode_path.display(),
        object_path.display()
    );

    // Use clang -c -flto to create LTO-compatible object files
    // These .o files actually contain bitcode for LTO
    let status = std::process::Command::new(&clang)
        .arg("-c")
        .arg("-flto")
        .arg("-O2")
        .arg("-o")
        .arg(object_path)
        .arg(bitcode_path)
        .status()
        .map_err(|e| format!("Failed to execute clang: {}", e))?;

    if !status.success() {
        return Err("Failed to compile bitcode to LTO object file".to_string());
    }

    debug!(
        "Successfully compiled to LTO object file: {}",
        object_path.display()
    );
    Ok(())
}

/// Compile bitcode to executable using Clang
pub fn compile_to_executable(bitcode_path: &Path, output_path: &Path) -> Result<(), String> {
    let llvm_prefix = std::env::var("LLVM_SYS_211_PREFIX")
        .map_err(|_| "LLVM_SYS_211_PREFIX environment variable is not set".to_string())?;

    let clang = format!("{}/bin/clang", llvm_prefix);

    // Use global builtin library directory to find builtin.ll
    let runtime_path = std::env::var("TYPEPYTHON_RUNTIME")
        .map(PathBuf::from)
        .expect("TYPEPYTHON_RUNTIME environment variable not set")
        .join("runtime.c");

    debug!(
        "Linking with clang: {} + {} -> {}",
        bitcode_path.display(),
        runtime_path.display(),
        output_path.display()
    );

    let mut cmd = std::process::Command::new(&clang);
    cmd.arg("-Wno-override-module").arg(bitcode_path);

    // Link runtime library if it exists
    if runtime_path.exists() {
        debug!("Linking runtime library: {}", runtime_path.display());
        cmd.arg(&runtime_path);
    } else {
        debug!(
            "Warning: Runtime library not found at {}",
            runtime_path.display()
        );
    }

    cmd.arg("-o").arg(output_path).arg("-lm"); // Link math library for pow

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute clang: {}", e))?;

    if !status.success() {
        return Err("Compilation failed".to_string());
    }

    Ok(())
}

/// Compile a file with full module support (resolves and compiles imports)
pub fn compile_file_with_modules<'ctx>(
    path: &Path,
    context: &'ctx Context,
    options: &CompileOptions,
) -> Result<ModuleRegistry<'ctx>, String> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| format!("Error reading {}: {}", path.display(), e))?;

    let _module_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;

    // Create module registry
    let mut registry = ModuleRegistry::new(context);

    // Add the directory of the source file to search paths
    if let Some(parent) = path.parent() {
        registry.add_search_path(parent.to_path_buf());
    }

    // Parse the main file first to get its imports
    debug!("Preprocessing main module");
    let preprocessed = preprocessor::preprocess(&source)?;
    let pairs = LangParser::parse(Rule::program, &preprocessed)
        .map_err(|e| format!("Parse error: {}", e))?;
    let program = parser::build_program(pairs);

    // Compile all imported modules first
    for import in &program.imports {
        registry.compile_module(&import.module_path, options)?;
    }

    // Don't compile the main module here - it will be compiled later with import information
    Ok(registry)
}

/// Link multiple object files together into an executable using LTO
pub fn link_object_files(object_files: &[PathBuf], output_path: &Path) -> Result<(), String> {
    let llvm_prefix = std::env::var("LLVM_SYS_211_PREFIX")
        .map_err(|_| "LLVM_SYS_211_PREFIX environment variable is not set".to_string())?;

    let clang = format!("{}/bin/clang", llvm_prefix);

    debug!("Linking object files with LTO: {:?}", object_files);

    let mut cmd = std::process::Command::new(&clang);
    cmd.arg("-Wno-override-module");

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

/// Full pipeline: compile source file to executable
pub fn compile_and_link(
    source_path: &Path,
    output_path: &Path,
    options: &CompileOptions,
) -> Result<(), String> {
    let context = Context::create();
    let result = compile_file(source_path, &context, options)?;

    // Write bitcode to temporary file
    let bc_path = source_path.with_extension("bc");
    write_bitcode(&result.codegen, &bc_path)?;

    // Compile to executable
    compile_to_executable(&bc_path, output_path)?;

    // Clean up bitcode file
    let _ = std::fs::remove_file(&bc_path);

    Ok(())
}

/// Compile a C file to LLVM bitcode
fn compile_c_to_bitcode(c_file: &Path, output_bc: &Path) -> Result<(), String> {
    let llvm_prefix = std::env::var("LLVM_SYS_211_PREFIX").unwrap();

    let clang = format!("{}/bin/clang", llvm_prefix);

    debug!(
        "Compiling C file {} to {}",
        c_file.display(),
        output_bc.display()
    );

    let output = std::process::Command::new(&clang)
        .args([
            "-c",
            "-emit-llvm",
            "-O2",
            "-o",
            output_bc.to_str().unwrap(),
            c_file.to_str().unwrap(),
        ])
        .output()
        .map_err(|e| format!("Failed to execute clang: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to compile C file to bitcode:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    debug!("Successfully compiled C file to bitcode");
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
    options: &CompileOptions,
) -> Result<(), String> {
    let context = Context::create();
    debug!("Starting compilation pipeline");

    // ============================================================================
    // Step 1: Recursively compile all .py modules with their imports
    // ============================================================================
    let mut module_registry = ModuleRegistry::new(&context);

    // Add source file directory to search paths
    if let Some(parent) = source_path.parent() {
        module_registry.add_search_path(parent.to_path_buf());
    }

    // Read and parse main file to discover imports
    let source = std::fs::read_to_string(source_path)
        .map_err(|e| format!("Error reading {}: {}", source_path.display(), e))?;
    let preprocessed = preprocessor::preprocess(&source)?;
    let pairs = LangParser::parse(Rule::program, &preprocessed)
        .map_err(|e| format!("Parse error in main file: {}", e))?;
    let main_program = parser::build_program(pairs);

    // Compile all imported modules recursively
    for import in &main_program.imports {
        module_registry.compile_module(&import.module_path, options)?;
    }

    // Compile main module with all imports available
    let module_name = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;

    // Collect imported programs for main module
    let mut imported_programs = Vec::new();
    for import in &main_program.imports {
        let import_name = import.module_path.join(".");
        if let Some(compiled_mod) = module_registry.get_module(&import_name) {
            imported_programs.push(compiled_mod.program.clone());
        }
    }

    // Compile main module with imports
    let main_result =
        compile_source_with_imports(&source, module_name, &context, options, &imported_programs)?;

    debug!(
        "Compiled main module and {} imports",
        module_registry.modules().len()
    );

    // ============================================================================
    // Step 2: Compile all modules to .o files
    // ============================================================================
    let temp_dir = std::env::temp_dir().join(format!(
        "typepython_build_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;

    let mut object_files = Vec::new();

    // Compile imported modules to .o files
    for (mod_name, compiled_mod) in module_registry.modules() {
        let bc_path = temp_dir.join(format!("{}.bc", mod_name.replace(".", "_")));

        // Place .o file in __tpycache__ directory next to source file
        let source_dir = compiled_mod
            .path
            .parent()
            .ok_or_else(|| format!("Invalid source path: {}", compiled_mod.path.display()))?;
        let cache_dir = source_dir.join("__tpycache__");
        std::fs::create_dir_all(&cache_dir)
            .map_err(|e| format!("Failed to create __tpycache__ directory: {}", e))?;

        let obj_filename = format!("{}.o", mod_name.replace(".", "_"));
        let obj_path = cache_dir.join(obj_filename);

        compiled_mod
            .codegen
            .get_module()
            .write_bitcode_to_path(&bc_path);
        compile_bitcode_to_lto_object(&bc_path, &obj_path)?;
        object_files.push(obj_path);

        let _ = std::fs::remove_file(&bc_path);
    }

    // Compile main module to .o file
    let main_bc = temp_dir.join(format!("{}.bc", module_name));

    // Place main module .o in __tpycache__ next to source file
    let main_source_dir = source_path
        .parent()
        .ok_or_else(|| "Invalid main source path".to_string())?;
    let main_cache_dir = main_source_dir.join("__tpycache__");
    std::fs::create_dir_all(&main_cache_dir)
        .map_err(|e| format!("Failed to create __tpycache__ directory: {}", e))?;

    let main_obj = main_cache_dir.join(format!("{}.o", module_name));
    write_bitcode(&main_result.codegen, &main_bc)?;
    compile_bitcode_to_lto_object(&main_bc, &main_obj)?;
    object_files.push(main_obj);
    let _ = std::fs::remove_file(&main_bc);

    // ============================================================================
    // Step 3: Compile .c files (builtin and runtime) to .o files
    // ============================================================================

    // Compile builtin module
    let builtin_bc = module_registry.compile_builtin(&temp_dir)?;

    // Place builtin.o in __tpycache__ next to builtin.c
    let builtin_path = module_registry.resolve_module(&[String::from("builtin")])?;
    let builtin_dir = builtin_path
        .parent()
        .ok_or_else(|| "Invalid builtin path".to_string())?;
    let builtin_cache_dir = builtin_dir.join("__tpycache__");
    std::fs::create_dir_all(&builtin_cache_dir)
        .map_err(|e| format!("Failed to create __tpycache__ directory: {}", e))?;

    let builtin_obj = builtin_cache_dir.join("builtin.o");
    compile_bitcode_to_lto_object(&builtin_bc, &builtin_obj)?;
    object_files.push(builtin_obj);
    let _ = std::fs::remove_file(&builtin_bc);

    // Compile runtime library
    let runtime_path = std::env::var("TYPEPYTHON_RUNTIME")
        .map(PathBuf::from)
        .expect("TYPEPYTHON_RUNTIME environment variable not set")
        .join("runtime.c");

    if runtime_path.exists() {
        debug!("Compiling runtime library");
        let runtime_bc = temp_dir.join("runtime.bc");

        // Place runtime.o in __tpycache__ next to runtime.c
        let runtime_dir = runtime_path
            .parent()
            .ok_or_else(|| "Invalid runtime path".to_string())?;
        let runtime_cache_dir = runtime_dir.join("__tpycache__");
        std::fs::create_dir_all(&runtime_cache_dir)
            .map_err(|e| format!("Failed to create __tpycache__ directory: {}", e))?;

        let runtime_obj = runtime_cache_dir.join("runtime.o");

        compile_c_to_bitcode(&runtime_path, &runtime_bc)?;
        compile_bitcode_to_lto_object(&runtime_bc, &runtime_obj)?;
        object_files.push(runtime_obj);

        let _ = std::fs::remove_file(&runtime_bc);
    }

    // ============================================================================
    // Step 4: Link all .o files into executable
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
