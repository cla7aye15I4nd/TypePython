/// Module resolution and management for TypePython
use inkwell::context::Context;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

use crate::ast::Program;
use crate::Parser;

/// Get the clang executable path (hardcoded at compile time from build.rs)
pub fn get_clang_path() -> String {
    format!("{}/bin/clang", env!("TYPEPYTHON_LLVM_PREFIX"))
}

/// Module registry manages all compiled modules
pub struct ModuleRegistry<'ctx> {
    /// LLVM context (shared across all modules)
    #[allow(dead_code)]
    context: &'ctx Context,
    /// Root paths for all module
    root: PathBuf,
    /// Module values: module_name -> PyValue (Module type with ModuleInfo)
    pub modules: HashMap<String, crate::types::PyValue<'ctx>>,
    /// AST programs: module_name -> Program (for codegen to look up function definitions)
    pub programs: HashMap<String, Program>,
}

impl<'ctx> ModuleRegistry<'ctx> {
    /// Create a new module registry and preprocess all modules starting from entry_path
    pub fn new(context: &'ctx Context, root: PathBuf, entry_path: &Path) -> Result<Self, String> {
        let mut registry = ModuleRegistry {
            context,
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
    ///
    /// Note: C builtin modules are no longer automatically loaded here.
    /// They are compiled at build time and linked selectively based on usage.
    pub fn preprocess_modules(&mut self, entry_path: &Path) -> Result<(), String> {
        use crate::types::{ModuleInfo, PyValue};
        use crate::{LangParser, Rule};

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

            // Read source file
            let current_source = std::fs::read_to_string(&current_path)
                .map_err(|e| format!("Error reading {}: {}", current_path.display(), e))?;

            // Parse and build AST
            let preprocessed = crate::preprocessor::preprocess(&current_source)?;
            let pairs = LangParser::parse(Rule::program, &preprocessed)
                .map_err(|e| format!("Parse error in {}: {}", current_path.display(), e))?;
            let program = crate::ast::parser::build_program(pairs);

            // Collect imports for second pass
            let mut imports = Vec::new();
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

                imports.push((local_name, import_module_name));
                queue.push_back(import_path);
            }

            // Store program and create module (members will be populated in second pass)
            self.programs.insert(module_name.clone(), program);
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
            if let Some(program) = self.programs.get(module_name) {
                let functions_to_add: Vec<_> = program
                    .functions
                    .iter()
                    .map(|func| {
                        let mangled_name = Self::mangle_function_name(module_name, &func.name);
                        let func_info = crate::types::FunctionInfo::from_ast(&mangled_name, func);
                        (func.name.clone(), PyValue::function(func_info))
                    })
                    .collect();

                if let Some(module) = self.modules.get_mut(module_name) {
                    for (func_name, func_value) in functions_to_add {
                        module.add_member(func_name, func_value).ok();
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
