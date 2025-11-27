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

/// Compile bitcode to executable using Clang
pub fn compile_to_executable(bitcode_path: &Path, output_path: &Path) -> Result<(), String> {
    let llvm_prefix = std::env::var("LLVM_SYS_211_PREFIX")
        .map_err(|_| "LLVM_SYS_211_PREFIX environment variable is not set".to_string())?;

    let clang = format!("{}/bin/clang", llvm_prefix);

    // Find the runtime library - try multiple locations
    let runtime_candidates = [
        std::path::PathBuf::from("src/runtime/builtin.ll"),
        std::path::PathBuf::from("../src/runtime/builtin.ll"),
        std::env::current_dir()
            .ok()
            .map(|p| p.join("src/runtime/builtin.ll"))
            .unwrap_or_default(),
        // Also try the old location for backwards compatibility
        std::path::PathBuf::from("runtime/builtin.ll"),
    ];

    let runtime_path = runtime_candidates
        .iter()
        .find(|p| p.exists())
        .cloned()
        .unwrap_or_else(|| std::path::PathBuf::from("src/runtime/builtin.ll"));

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

    let module_name = path
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

    // Now compile the main module
    let _result = compile_source(&source, module_name, context, options)?;

    Ok(registry)
}

/// Link multiple bitcode files together into an executable
pub fn link_bitcode_files(bitcode_files: &[PathBuf], output_path: &Path) -> Result<(), String> {
    let llvm_prefix = std::env::var("LLVM_SYS_211_PREFIX")
        .map_err(|_| "LLVM_SYS_211_PREFIX environment variable is not set".to_string())?;

    let clang = format!("{}/bin/clang", llvm_prefix);

    debug!("Linking bitcode files: {:?}", bitcode_files);

    let mut cmd = std::process::Command::new(&clang);
    cmd.arg("-Wno-override-module");

    // Add all bitcode files (builtin should already be included by the caller)
    for bc_file in bitcode_files {
        cmd.arg(bc_file);
    }

    cmd.arg("-o").arg(output_path).arg("-lm");

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to execute clang: {}", e))?;

    if !status.success() {
        return Err("Linking failed".to_string());
    }

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

    // Compile main file and get module registry
    let registry = compile_file_with_modules(source_path, &context, options)?;

    // Compile the main module and add to registry
    let module_name = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;

    let main_result = compile_file(source_path, &context, options)?;

    // Create unique temporary directory for bitcode files
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

    // Write all module bitcode files
    let mut bitcode_files = registry.write_all_bitcode(&temp_dir)?;

    // Write main module bitcode
    let main_bc = temp_dir.join(format!("{}.bc", module_name));
    write_bitcode(&main_result.codegen, &main_bc)?;
    bitcode_files.push(main_bc);

    // Always compile and include builtin module
    let builtin_bc = registry.compile_builtin(&temp_dir)?;
    bitcode_files.push(builtin_bc);

    // Link all bitcode files
    link_bitcode_files(&bitcode_files, output_path)?;

    // Clean up temporary files
    for bc_file in &bitcode_files {
        let _ = std::fs::remove_file(bc_file);
    }

    Ok(())
}
