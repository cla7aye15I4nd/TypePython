/// Module resolution and management for TypePython
use inkwell::context::Context;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ast::{self, Expr, Module, Stmt};

/// Get the clang executable path (hardcoded at compile time from build.rs)
pub fn get_clang_path() -> String {
    format!("{}/bin/clang", env!("TYPEPYTHON_LLVM_PREFIX"))
}

/// Parse a Python file using the ast_to_json.py script
pub fn parse_python_file(path: &Path) -> Result<Module, String> {
    // Try to find the script relative to the executable or use a fallback
    let script_path = find_ast_script()?;

    let output = Command::new("python3")
        .arg(&script_path)
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run Python AST parser: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "Python AST parser failed for {}: {}\n{}",
            path.display(),
            stderr,
            stdout
        ));
    }

    let json = String::from_utf8_lossy(&output.stdout);
    ast::parse_json(&json)
}

/// Find the ast_to_json.py script
fn find_ast_script() -> Result<PathBuf, String> {
    // Try multiple locations
    let candidates = [
        // Relative to current working directory
        PathBuf::from("scripts/ast_to_json.py"),
        // Relative to executable
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("../scripts/ast_to_json.py")))
            .unwrap_or_default(),
        // Relative to manifest dir (for development)
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts/ast_to_json.py"),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    Err(format!(
        "Could not find ast_to_json.py script. Searched: {:?}",
        candidates
    ))
}

/// Module registry manages all compiled modules
pub struct ModuleRegistry<'ctx> {
    /// Root paths for all module
    root: PathBuf,
    /// Module values: module_name -> PyValue (Module type with ModuleInfo)
    pub modules: HashMap<String, crate::types::PyValue<'ctx>>,
    /// AST modules: module_name -> Module (for codegen to look up function definitions)
    pub programs: HashMap<String, Module>,
}

impl<'ctx> ModuleRegistry<'ctx> {
    /// Create a new module registry and preprocess all modules starting from entry_path
    pub fn new(_context: &'ctx Context, root: PathBuf, entry_path: &Path) -> Result<Self, String> {
        let mut registry = ModuleRegistry {
            root,
            modules: HashMap::new(),
            programs: HashMap::new(),
        };

        // Automatically preprocess all modules
        registry.preprocess_modules(entry_path)?;

        Ok(registry)
    }

    /// Generate a module name from a file path
    /// For example:
    /// - /path/to/project/math/helpers.py -> "math.helpers"
    fn generate_module_name(&self, path: &Path) -> Result<String, String> {
        // User module relative to root
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
    /// Discovers all imported modules, generates module names, and builds PyValue modules
    pub fn preprocess_modules(&mut self, entry_path: &Path) -> Result<(), String> {
        use crate::types::{ModuleInfo, PyValue};

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        // Temporary storage for import relationships: module_name -> Vec<(local_name, imported_module_name)>
        let mut import_map: HashMap<String, Vec<(String, String)>> = HashMap::new();

        // Add entry module to queue
        queue.push_back(entry_path.to_path_buf());

        // BFS through all modules - first pass: collect all modules and their programs
        while let Some(current_path) = queue.pop_front() {
            // Skip if already visited
            let path_key = current_path.to_string_lossy().to_string();
            if visited.contains(&path_key) {
                continue;
            }
            visited.insert(path_key);

            // Generate module name for this module
            let module_name = self.generate_module_name(&current_path)?;

            // Parse Python file to AST
            let module = parse_python_file(&current_path)?;

            // Extract imports from module body
            let mut imports = Vec::new();
            for stmt in &module.body {
                if let Stmt::Import(import_stmt) = stmt {
                    for alias in &import_stmt.names {
                        // Parse module path from name (e.g., "math.helpers" -> ["math", "helpers"])
                        let module_path: Vec<String> =
                            alias.name.split('.').map(String::from).collect();

                        // Resolve import path - error if module not found
                        let import_path =
                            self.resolve_module(&module_path, current_path.parent().unwrap())?;

                        // Generate the real module name for the imported module
                        let import_module_name = self.generate_module_name(&import_path)?;

                        // Use asname if provided, otherwise use the last component
                        let local_name = alias
                            .asname
                            .clone()
                            .unwrap_or_else(|| module_path.last().unwrap().clone());

                        imports.push((local_name, import_module_name));
                        queue.push_back(import_path);
                    }
                }
            }

            // Store program and create module (members will be populated in second pass)
            self.programs.insert(module_name.clone(), module);
            self.modules.insert(
                module_name.clone(),
                PyValue::module(ModuleInfo {
                    name: module_name.clone(),
                    members: HashMap::new(),
                }),
            );
            import_map.insert(module_name, imports);
        }

        // Second pass: Add functions to each module's members FIRST
        // This must happen before import resolution so imported modules have their functions
        for module_name in import_map.keys() {
            if let Some(module) = self.programs.get(module_name) {
                let functions_to_add: Vec<_> = extract_functions(module)
                    .iter()
                    .map(|func| {
                        let mangled_name = Self::mangle_function_name(module_name, &func.name);
                        let func_info =
                            crate::types::FunctionInfo::from_function_def(&mangled_name, func);
                        (func.name.clone(), PyValue::function(func_info))
                    })
                    .collect();

                if let Some(module_val) = self.modules.get_mut(module_name) {
                    for (func_name, func_value) in functions_to_add {
                        module_val.add_member(func_name, func_value).ok();
                    }
                }
            }
        }

        // Third pass: Add imported modules to each module's members
        // Now all modules have their functions populated
        for (module_name, imports) in import_map {
            for (local_name, imported_module_name) in imports {
                if let Some(imported_module) = self.modules.get(&imported_module_name).cloned() {
                    if let Some(module) = self.modules.get_mut(&module_name) {
                        module.add_member(local_name, imported_module).ok();
                    }
                }
            }
        }

        Ok(())
    }

    /// Mangle function name with module name
    fn mangle_function_name(module_name: &str, function_name: &str) -> String {
        let clean_module = module_name
            .replace(".", "_")
            .replace("<", "")
            .replace(">", "");
        format!("{}_{}", clean_module, function_name)
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

        // Search in current directory only
        for ext in &extensions {
            let candidate = current_path.join(format!("{}.{}", module_base, ext));

            if candidate.exists() {
                return Ok(candidate);
            }
        }

        Err(format!(
            "Module '{}' not found in current directory",
            module_path.join(".")
        ))
    }
}

// ============================================================================
// Helper functions for extracting info from Python AST
// ============================================================================

/// Extract all function definitions from a module
pub fn extract_functions(module: &Module) -> Vec<&ast::FunctionDef> {
    module
        .body
        .iter()
        .filter_map(|stmt| {
            if let Stmt::FunctionDef(func) = stmt {
                Some(func)
            } else {
                None
            }
        })
        .collect()
}

/// Extract all class definitions from a module
pub fn extract_classes(module: &Module) -> Vec<&ast::ClassDef> {
    module
        .body
        .iter()
        .filter_map(|stmt| {
            if let Stmt::ClassDef(class) = stmt {
                Some(class)
            } else {
                None
            }
        })
        .collect()
}

/// Extract all import statements from a module
pub fn extract_imports(module: &Module) -> Vec<&ast::PyImport> {
    module
        .body
        .iter()
        .filter_map(|stmt| {
            if let Stmt::Import(import) = stmt {
                Some(import)
            } else {
                None
            }
        })
        .collect()
}

/// Extract top-level statements (excluding functions, classes, and imports)
pub fn extract_statements(module: &Module) -> Vec<&Stmt> {
    module
        .body
        .iter()
        .filter(|stmt| {
            !matches!(
                stmt,
                Stmt::FunctionDef(_) | Stmt::ClassDef(_) | Stmt::Import(_) | Stmt::ImportFrom(_)
            )
        })
        .collect()
}

/// Get type annotation as string from an expression (for type hints)
pub fn type_annotation_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Name(name) => name.id.clone(),
        Expr::Subscript(sub) => {
            let base = type_annotation_to_string(&sub.value);
            let slice = type_annotation_to_string(&sub.slice);
            format!("{}[{}]", base, slice)
        }
        Expr::Tuple(tuple) => {
            let types: Vec<String> = tuple.elts.iter().map(type_annotation_to_string).collect();
            types.join(", ")
        }
        Expr::Constant(c) => {
            if let ast::ConstantValue::None = &c.value {
                "None".to_string()
            } else if let Some(s) = c.value.as_str() {
                // Forward reference
                s.to_string()
            } else {
                "Any".to_string()
            }
        }
        _ => "Any".to_string(),
    }
}
