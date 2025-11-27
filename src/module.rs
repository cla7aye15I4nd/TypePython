/// Module resolution and management for TypePython
use inkwell::context::Context;
use log::debug;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::ast::Program;
use crate::codegen::CodeGen;
use crate::pipeline::CompileOptions;
use crate::Parser;

/// Global builtin library directory path
/// This is initialized once on first access and reused throughout the program
static BUILTIN_LIB_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Get the builtin library directory path
/// Automatically finds and caches the correct path on first call
pub fn builtin_lib_dir() -> &'static PathBuf {
    BUILTIN_LIB_DIR.get_or_init(find_builtin_lib_dir)
}

/// Find the builtin library directory by checking environment variable or searching
fn find_builtin_lib_dir() -> PathBuf {
    // First, check if TYPEPYTHON_RUNTIME environment variable is set
    if let Ok(runtime_dir) = std::env::var("TYPEPYTHON_RUNTIME") {
        let path = PathBuf::from(runtime_dir);
        if path.join("builtin.c").exists() {
            debug!(
                "Using builtin library directory from TYPEPYTHON_RUNTIME: {}",
                path.display()
            );
            return path;
        } else {
            debug!(
                "Warning: TYPEPYTHON_RUNTIME is set but builtin.c not found at: {}",
                path.display()
            );
        }
    }

    // Fallback: search multiple candidate locations
    let candidates = [
        PathBuf::from("src/runtime"),
        PathBuf::from("./src/runtime"),
        PathBuf::from("../src/runtime"),
        std::env::current_dir()
            .ok()
            .map(|p| p.join("src/runtime"))
            .unwrap_or_default(),
        std::env::current_exe()
            .ok()
            .and_then(|exe| {
                exe.parent()
                    .and_then(|p| p.parent())
                    .map(|p| p.join("src/runtime"))
            })
            .unwrap_or_default(),
    ];

    // Find the first directory that contains builtin.c
    for candidate in &candidates {
        let builtin_c = candidate.join("builtin.c");
        if builtin_c.exists() {
            debug!(
                "Found builtin library directory at: {}",
                candidate.display()
            );
            return candidate.clone();
        }
    }

    // Fallback to default location
    debug!("Warning: Builtin library directory not found, using default: src/runtime");
    PathBuf::from("src/runtime")
}

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
        ModuleRegistry {
            modules: HashMap::new(),
            context,
            search_paths: vec![],
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
        // Try different file extensions
        let extensions = ["py", "c"];
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

    /// Get the path to the builtin.c file
    /// The builtin module is always compiled and included automatically
    pub fn get_builtin_path(&self) -> PathBuf {
        builtin_lib_dir().join("builtin.c")
    }

    /// Compile a C module to bitcode and object file
    /// Returns the path to the generated object file
    pub fn compile_c_module(
        &self,
        c_path: &Path,
        output_dir: &Path,
        cache_dir: &Path,
    ) -> Result<PathBuf, String> {
        let module_name = c_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("Invalid C file name: {}", c_path.display()))?;

        let bc_path = output_dir.join(format!("{}.bc", module_name));
        let obj_path = cache_dir.join(format!("{}.o", module_name));

        // Compile C to bitcode
        self.compile_c_file(c_path, &bc_path)?;

        // Compile bitcode to LTO object
        crate::pipeline::compile_bitcode_to_lto_object(&bc_path, &obj_path)?;

        // Clean up temporary bitcode file
        let _ = std::fs::remove_file(&bc_path);

        debug!(
            "Compiled C module {} to {}",
            module_name,
            obj_path.display()
        );
        Ok(obj_path)
    }
}
