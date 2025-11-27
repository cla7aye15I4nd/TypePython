/// Module resolution and management for TypePython
use inkwell::context::Context;
use log::debug;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::ast::Program;
use crate::codegen::CodeGen;
use crate::pipeline::CompileOptions;
use crate::Parser;

/// Represents a compiled module with its LLVM bitcode
pub struct CompiledModule<'ctx> {
    pub name: String,
    pub path: PathBuf,
    pub program: Program,
    pub codegen: CodeGen<'ctx>,
}

/// Module registry manages all compiled modules
pub struct ModuleRegistry<'ctx> {
    /// Map from module name to compiled module
    modules: HashMap<String, CompiledModule<'ctx>>,
    /// LLVM context (shared across all modules)
    context: &'ctx Context,
    /// Base search paths for module resolution
    search_paths: Vec<PathBuf>,
}

impl<'ctx> ModuleRegistry<'ctx> {
    /// Create a new module registry
    pub fn new(context: &'ctx Context) -> Self {
        let mut search_paths = vec![
            PathBuf::from("."),   // Current directory
            PathBuf::from("src"), // src directory
        ];

        // Add current working directory
        if let Ok(cwd) = std::env::current_dir() {
            search_paths.push(cwd);
        }

        ModuleRegistry {
            modules: HashMap::new(),
            context,
            search_paths,
        }
    }

    /// Add a search path for module resolution
    pub fn add_search_path(&mut self, path: PathBuf) {
        if !self.search_paths.contains(&path) {
            self.search_paths.push(path);
        }
    }

    /// Resolve a module path to a file path
    /// For example: ["math", "helpers"] -> "math/helpers.py" or "./math/helpers.py"
    /// Supports .py, .tpy, and .c files
    pub fn resolve_module(&self, module_path: &[String]) -> Result<PathBuf, String> {
        // Special case: "builtin" module - look for builtin.c in runtime directory
        if module_path.len() == 1 && module_path[0] == "builtin" {
            // Try to find builtin.c in src/runtime
            let builtin_candidates = [
                PathBuf::from("src/runtime/builtin.c"),
                PathBuf::from("../src/runtime/builtin.c"),
                PathBuf::from("../../src/runtime/builtin.c"),
                PathBuf::from("../../../src/runtime/builtin.c"),
                PathBuf::from("../../../../src/runtime/builtin.c"),
                PathBuf::from("/workspaces/TypePython/src/runtime/builtin.c"),
                std::env::current_dir()
                    .ok()
                    .map(|p| p.join("src/runtime/builtin.c"))
                    .unwrap_or_default(),
                std::env::current_exe()
                    .ok()
                    .and_then(|p| {
                        p.parent()
                            .and_then(|p| p.parent())
                            .map(|p| p.join("src/runtime/builtin.c"))
                    })
                    .unwrap_or_default(),
            ];

            for candidate in &builtin_candidates {
                if candidate.exists() {
                    debug!("Found builtin module at: {}", candidate.display());
                    return Ok(candidate.clone());
                }
            }

            return Err("Builtin module (src/runtime/builtin.c) not found".to_string());
        }

        // Try different file extensions
        let extensions = ["py", "tpy", "c"];
        let module_base = module_path.join("/");

        // Search in all search paths
        for base_path in &self.search_paths {
            for ext in &extensions {
                let module_file = format!("{}.{}", module_base, ext);
                let full_path = base_path.join(&module_file);
                debug!("Checking for module at: {}", full_path.display());

                if full_path.exists() {
                    debug!("Found module at: {}", full_path.display());
                    return Ok(full_path);
                }
            }
        }

        Err(format!(
            "Module '{}' not found in search paths: {:?}",
            module_path.join("."),
            self.search_paths
        ))
    }

    /// Compile a C file to LLVM bitcode
    fn compile_c_file(&self, c_file: &Path, output_bc: &Path) -> Result<(), String> {
        let llvm_prefix =
            std::env::var("LLVM_SYS_211_PREFIX").unwrap_or_else(|_| "/usr/lib/llvm-21".to_string());

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

    /// Compile and register a module
    pub fn compile_module(
        &mut self,
        module_path: &[String],
        options: &CompileOptions,
    ) -> Result<(), String> {
        let module_name = module_path.join(".");

        // Check if already compiled
        if self.modules.contains_key(&module_name) {
            debug!("Module '{}' already compiled", module_name);
            return Ok(());
        }

        debug!("Compiling module: {}", module_name);

        // Resolve the module file path
        let file_path = self.resolve_module(module_path)?;

        // Check if it's a C file
        if file_path.extension().and_then(|s| s.to_str()) == Some("c") {
            debug!(
                "Module '{}' is a C file, will be compiled separately",
                module_name
            );
            // C files don't have imports and are handled at link time
            // We don't store them in the modules map since they're compiled directly to .bc
            return Ok(());
        }

        // Read the source code (for .py and .tpy files)
        let source = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Error reading {}: {}", file_path.display(), e))?;

        // Parse to get imports first
        let preprocessed = crate::preprocessor::preprocess(&source)?;
        let pairs = crate::LangParser::parse(crate::Rule::program, &preprocessed)
            .map_err(|e| format!("Parse error: {}", e))?;
        let program = crate::ast::parser::build_program(pairs);

        // Recursively compile imported modules first
        for import in &program.imports {
            self.compile_module(&import.module_path, options)?;
        }

        // Collect programs from imported modules
        let mut imported_programs = Vec::new();
        for import in &program.imports {
            let import_name = import.module_path.join(".");
            if let Some(compiled_mod) = self.modules.get(&import_name) {
                imported_programs.push(compiled_mod.program.clone());
            }
        }

        // Compile the module with imports
        let result = crate::pipeline::compile_source_with_imports(
            &source,
            &module_name,
            self.context,
            options,
            &imported_programs,
        )?;

        // Store the compiled module
        let compiled_module = CompiledModule {
            name: module_name.clone(),
            path: file_path,
            program: result.program,
            codegen: result.codegen,
        };

        self.modules.insert(module_name, compiled_module);

        Ok(())
    }

    /// Get a compiled module by name
    pub fn get_module(&self, name: &str) -> Option<&CompiledModule<'ctx>> {
        self.modules.get(name)
    }

    /// Get all compiled modules
    pub fn modules(&self) -> &HashMap<String, CompiledModule<'ctx>> {
        &self.modules
    }

    /// Write all modules to bitcode files
    pub fn write_all_bitcode(&self, output_dir: &Path) -> Result<Vec<PathBuf>, String> {
        let mut bitcode_files = Vec::new();

        for (name, module) in &self.modules {
            let bc_filename = format!("{}.bc", name.replace(".", "_"));
            let bc_path = output_dir.join(bc_filename);

            module.codegen.get_module().write_bitcode_to_path(&bc_path);
            debug!(
                "Wrote bitcode for module '{}' to {}",
                name,
                bc_path.display()
            );

            bitcode_files.push(bc_path);
        }

        Ok(bitcode_files)
    }

    /// Compile the builtin module and return the path to its bitcode
    /// The builtin module is always compiled and included automatically
    pub fn compile_builtin(&self, output_dir: &Path) -> Result<PathBuf, String> {
        // Find builtin.c
        let builtin_path = self.resolve_module(&[String::from("builtin")])?;

        // Compile to bitcode
        let builtin_bc = output_dir.join("builtin.bc");
        self.compile_c_file(&builtin_path, &builtin_bc)?;

        debug!("Compiled builtin module to {}", builtin_bc.display());
        Ok(builtin_bc)
    }
}
