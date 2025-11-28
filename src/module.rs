/// Module resolution and management for TypePython
use inkwell::context::Context;
use log::debug;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

use crate::ast::Program;
use crate::Parser;

/// Represents an imported symbol (module or function)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportedSymbol {
    /// An imported module with its real module name
    Module(String),
    /// An imported function with its real function name - for future use
    Function(String),
}

/// Get the clang executable path
pub fn get_clang_path() -> String {
    let llvm_prefix =
        std::env::var("LLVM_SYS_211_PREFIX").unwrap_or_else(|_| "/usr/lib/llvm-21".to_string());
    format!("{}/bin/clang", llvm_prefix)
}

/// Get the builtin library directory path (for user-provided Python builtins)
/// Note: C builtin modules are now compiled at build time and linked selectively
pub fn get_builtin_library_dir() -> Option<PathBuf> {
    std::env::var("TYPEPYTHON_RUNTIME").map(PathBuf::from).ok()
}

/// Type alias for preprocessed module data: (module_name, program, imported_symbols)
type PreprocessedModule = (String, Program, HashMap<String, ImportedSymbol>);

/// Module registry manages all compiled modules
pub struct ModuleRegistry<'ctx> {
    /// LLVM context (shared across all modules)
    _context: &'ctx Context,
    /// Root paths for all module
    root: PathBuf,
    /// Preprocessed module data: map of file paths to (module_name, program, imported_symbols)
    pub module_data: HashMap<PathBuf, PreprocessedModule>,
}

impl<'ctx> ModuleRegistry<'ctx> {
    /// Create a new module registry and preprocess all modules starting from entry_path
    pub fn new(context: &'ctx Context, root: PathBuf, entry_path: &Path) -> Result<Self, String> {
        let mut registry = ModuleRegistry {
            _context: context,
            root,
            module_data: HashMap::new(),
        };

        // Automatically preprocess all modules
        registry.module_data = registry.preprocess_modules(entry_path)?;

        Ok(registry)
    }

    /// Generate a module name from a file path
    /// For example:
    /// - /path/to/project/math/helpers.py -> "math.helpers"
    /// - /usr/lib/typepython/builtins/list.py -> "<builtin>.builtins.list"
    fn generate_module_name(&self, path: &Path) -> Result<String, String> {
        // Check if it's a builtin module (if TYPEPYTHON_RUNTIME is set)
        if let Some(builtin_dir) = get_builtin_library_dir() {
            if let Ok(rel_path) = path.strip_prefix(&builtin_dir) {
                let module_path = rel_path
                    .with_extension("")
                    .to_string_lossy()
                    .replace("/", ".");
                return Ok(format!("<builtin>.{}", module_path));
            }
        }

        // Otherwise, it's a user module relative to root
        let rel_path = path.strip_prefix(&self.root).map_err(|_| {
            format!(
                "Path {} is not relative to root {}",
                path.display(),
                self.root.display()
            )
        })?;

        let module_path = rel_path
            .with_extension("")
            .to_string_lossy()
            .replace("/", ".");

        Ok(module_path)
    }

    /// Preprocess all modules using BFS starting from entry file
    /// Discovers all imported modules, generates module names, and builds symbol maps
    /// Returns a map of file paths to their (module_name, program, imported_symbols)
    ///
    /// Note: C builtin modules are no longer automatically loaded here.
    /// They are compiled at build time and linked selectively based on usage.
    pub fn preprocess_modules(
        &self,
        entry_path: &Path,
    ) -> Result<HashMap<PathBuf, PreprocessedModule>, String> {
        use crate::{LangParser, Rule};

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut module_data = HashMap::new();

        // Add entry module to queue
        queue.push_back(entry_path.to_path_buf());

        // Optionally add Python builtin modules from TYPEPYTHON_RUNTIME if set
        // Note: C builtin modules are now compiled at build time and linked selectively
        if let Some(builtin_dir) = get_builtin_library_dir() {
            if builtin_dir.exists() {
                if let Ok(entries) = std::fs::read_dir(&builtin_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() {
                            // Only add .py files from builtin directory (not .c)
                            if let Some(ext) = path.extension() {
                                if ext == "py" {
                                    debug!("Auto-adding builtin Python module: {}", path.display());
                                    queue.push_back(path);
                                }
                            }
                        }
                    }
                }
            }
        }

        // BFS through all modules
        while let Some(current_path) = queue.pop_front() {
            // Skip if already visited
            let path_key = current_path.to_string_lossy().to_string();
            if visited.contains(&path_key) {
                continue;
            }
            visited.insert(path_key);

            // Generate module name for this module
            let module_name = self.generate_module_name(&current_path)?;

            // Handle C files separately - they don't have imports but should be registered
            if current_path.extension().and_then(|s| s.to_str()) == Some("c") {
                debug!(
                    "Preprocessing C module '{}' at {}",
                    module_name,
                    current_path.display()
                );
                // C files have no AST program or imports, but we still register them
                // Use empty program and symbol map as placeholders (will be compiled directly to .o)
                let empty_program = Program {
                    imports: vec![],
                    classes: vec![],
                    functions: vec![],
                    statements: vec![],
                };
                module_data.insert(
                    current_path.clone(),
                    (module_name, empty_program, HashMap::new()),
                );
                continue;
            }

            // Read source file
            let current_source = std::fs::read_to_string(&current_path)
                .map_err(|e| format!("Error reading {}: {}", current_path.display(), e))?;

            // Parse and build AST
            let preprocessed = crate::preprocessor::preprocess(&current_source)?;
            let pairs = LangParser::parse(Rule::program, &preprocessed)
                .map_err(|e| format!("Parse error in {}: {}", current_path.display(), e))?;
            let program = crate::ast::parser::build_program(pairs);

            debug!(
                "Preprocessing module '{}' at {}",
                module_name,
                current_path.display()
            );

            // Build symbol map for this module's imports
            let mut imported_symbols = HashMap::new();

            // Add this module's imports to the queue
            for import in &program.imports {
                // Resolve import path
                let import_path =
                    self.resolve_module(&import.module_path, current_path.parent().unwrap())?;

                // Generate the real module name for the imported module
                let import_module_name = self.generate_module_name(&import_path)?;

                // For now, use the last component as local name
                // TODO: Support "import foo as bar" syntax
                let local_name = import
                    .module_path
                    .last()
                    .ok_or_else(|| "Empty import path".to_string())?
                    .clone();

                // Add to symbol map (local_name -> real module name)
                imported_symbols.insert(local_name, ImportedSymbol::Module(import_module_name));

                queue.push_back(import_path);
            }

            debug!(
                "Module '{}' imports {} symbols",
                module_name,
                imported_symbols.len()
            );

            // Store module data
            module_data.insert(
                current_path.clone(),
                (module_name, program, imported_symbols),
            );
        }

        debug!("Preprocessed {} modules total", module_data.len());
        Ok(module_data)
    }

    /// Resolve a module path to a file path
    /// For example: ["math", "helpers"] -> "math/helpers.py" or "./math/helpers.py"
    /// Supports .py files (C builtins are linked at build time, not imported)
    pub fn resolve_module(
        &self,
        module_path: &[String],
        current_path: &Path,
    ) -> Result<PathBuf, String> {
        // Try different file extensions (only .py, C builtins are linked separately)
        let extensions = ["py"];
        let module_base = module_path.join("/");

        // Build search paths list dynamically
        let mut search_paths: Vec<(&str, PathBuf, PathBuf)> = Vec::new();

        // Add builtin directory if TYPEPYTHON_RUNTIME is set
        if let Some(builtin_dir) = get_builtin_library_dir() {
            search_paths.push(("<builtin>", builtin_dir.clone(), builtin_dir));
        }

        // Add current directory
        search_paths.push(("", current_path.to_path_buf(), self.root.clone()));

        for (name, dir, root) in search_paths {
            for ext in &extensions {
                let candidate = dir.join(format!("{}.{}", module_base, ext));

                // compare the relative module name for
                if let Ok(rel_path) = candidate.strip_prefix(&root) {
                    let module_name = format!(
                        "{}.{}",
                        name,
                        &rel_path
                            .with_extension("")
                            .to_string_lossy()
                            .replace("/", ".")
                    );
                    if candidate.exists() {
                        debug!(
                            "Resolved module '{}' to {}",
                            module_name,
                            candidate.display()
                        );

                        return Ok(candidate);
                    }
                }
            }
        }

        Err(format!(
            "Module '{}' not found in builtin or current directory",
            module_path.join(".")
        ))
    }
}
