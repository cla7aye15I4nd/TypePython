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

/// Full pipeline with module support: compile source file and all imports to executable
pub fn compile_and_link_with_modules(
    source_path: &Path,
    output_path: &Path,
    options: &CompileOptions,
) -> Result<(), String> {
    let context = Context::create();

    debug!("Starting module-aware compilation");

    // Compile main file and get module registry
    let registry = compile_file_with_modules(source_path, &context, options)?;

    debug!(
        "Module registry has {} compiled modules",
        registry.modules().len()
    );

    // Compile the main module with imports
    let module_name = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;

    // Read and parse main file to get imports
    let source = std::fs::read_to_string(source_path)
        .map_err(|e| format!("Error reading {}: {}", source_path.display(), e))?;
    let preprocessed = preprocessor::preprocess(&source)?;
    let pairs = LangParser::parse(Rule::program, &preprocessed)
        .map_err(|e| format!("Parse error: {}", e))?;
    let main_program = parser::build_program(pairs);

    debug!("Main program has {} imports", main_program.imports.len());
    for imp in &main_program.imports {
        debug!("  Import: {}", imp.module_path.join("."));
    }

    // Collect programs from imported modules
    let mut imported_programs = Vec::new();
    for import in &main_program.imports {
        let import_name = import.module_path.join(".");
        debug!("Looking for imported module: {}", import_name);
        if let Some(compiled_mod) = registry.get_module(&import_name) {
            debug!(
                "Found module {} with {} functions",
                import_name,
                compiled_mod.program.functions.len()
            );
            imported_programs.push(compiled_mod.program.clone());
        } else {
            debug!("Module {} not found in registry", import_name);
        }
    }
    debug!(
        "Collected {} imported programs for main module",
        imported_programs.len()
    );

    // Compile main module with imports
    let main_result =
        compile_source_with_imports(&source, module_name, &context, options, &imported_programs)?;

    // Create build directory for object files (in current directory)
    let build_dir = PathBuf::from("build");
    std::fs::create_dir_all(&build_dir)
        .map_err(|e| format!("Failed to create build directory: {}", e))?;

    debug!("Using build directory: {}", build_dir.display());

    // Create temporary directory for intermediate bitcode files
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

    // Compile all imported modules to .o files
    for (mod_name, compiled_mod) in registry.modules() {
        let bc_path = temp_dir.join(format!("{}.bc", mod_name.replace(".", "_")));
        let obj_path = build_dir.join(format!("{}.o", mod_name.replace(".", "_")));

        // Write bitcode
        compiled_mod
            .codegen
            .get_module()
            .write_bitcode_to_path(&bc_path);
        debug!(
            "Wrote bitcode for module '{}': {}",
            mod_name,
            bc_path.display()
        );

        // Compile to LTO object file
        compile_bitcode_to_lto_object(&bc_path, &obj_path)?;
        object_files.push(obj_path);

        // Clean up temporary bitcode
        let _ = std::fs::remove_file(&bc_path);
    }

    // Compile main module to .o file
    let main_bc = temp_dir.join(format!("{}.bc", module_name));
    let main_obj = build_dir.join(format!("{}.o", module_name));
    write_bitcode(&main_result.codegen, &main_bc)?;
    debug!("Wrote main module bitcode: {}", main_bc.display());
    compile_bitcode_to_lto_object(&main_bc, &main_obj)?;
    object_files.push(main_obj);
    let _ = std::fs::remove_file(&main_bc);

    // Compile builtin module to .o file
    let builtin_bc = registry.compile_builtin(&temp_dir)?;
    let builtin_obj = build_dir.join("builtin.o");
    compile_bitcode_to_lto_object(&builtin_bc, &builtin_obj)?;
    object_files.push(builtin_obj);
    let _ = std::fs::remove_file(&builtin_bc);

    // Link all object files with LTO
    link_object_files(&object_files, output_path)?;

    debug!(
        "Build complete! Object files preserved in {}",
        build_dir.display()
    );

    Ok(())
}
