/// Module resolution and management for TypePython
use inkwell::context::Context;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyAnyMethods, PyDict, PyList};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};

use crate::ast::{self, Expr, Module, Stmt};

/// Get the clang executable path (hardcoded at compile time from build.rs)
pub fn get_clang_path() -> String {
    format!("{}/bin/clang", env!("TYPEPYTHON_LLVM_PREFIX"))
}

/// Parse a Python file using PyO3 to call Python's ast module directly
pub fn parse_python_file(path: &Path) -> Result<Module, String> {
    // Read the source file
    let source = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;
    let path_str = path.to_string_lossy().to_string();

    let json = Python::with_gil(|py| -> PyResult<String> {
        // Import Python's ast and json modules directly
        let ast_mod = py.import("ast")?;
        let json_mod = py.import("json")?;

        // Parse the source code: ast.parse(source, filename=path)
        let tree = ast_mod.call_method(
            "parse",
            (&source,),
            Some(&[("filename", &path_str)].into_py_dict(py)?),
        )?;

        // Convert AST to JSON-serializable dict
        let dict = ast_node_to_dict(py, &tree)?;

        // Serialize to JSON string
        let json_str = json_mod.call_method1("dumps", (dict,))?;
        json_str.extract::<String>()
    })
    .map_err(|e| format!("Python AST parser failed for {}: {}", path.display(), e))?;

    ast::parse_json(&json)
}

/// Recursively convert a Python AST node to a dictionary
fn ast_node_to_dict<'py>(
    py: Python<'py>,
    node: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyDict>> {
    let ast_mod = py.import("ast")?;
    let dict = PyDict::new(py);

    // Get the node type name
    let type_name: String = node.get_type().name()?.extract()?;
    dict.set_item("_type", &type_name)?;

    // Iterate over fields using ast.iter_fields() - returns a generator, convert to list
    let iter_fields = ast_mod.getattr("iter_fields")?;
    let fields_gen = iter_fields.call1((node,))?;
    let fields: Bound<'py, PyList> = py
        .get_type::<PyList>()
        .call1((fields_gen,))?
        .downcast_into()?;

    for item in fields.iter() {
        let tuple: (String, Bound<'py, PyAny>) = item.extract()?;
        let (field_name, value) = tuple;
        let serialized = serialize_value(py, &value)?;
        dict.set_item(field_name, serialized)?;
    }

    // Add location info if present
    if let Ok(lineno) = node.getattr("lineno") {
        if !lineno.is_none() {
            dict.set_item("lineno", lineno)?;
        }
    }
    if let Ok(col_offset) = node.getattr("col_offset") {
        if !col_offset.is_none() {
            dict.set_item("col_offset", col_offset)?;
        }
    }
    if let Ok(end_lineno) = node.getattr("end_lineno") {
        if !end_lineno.is_none() {
            dict.set_item("end_lineno", end_lineno)?;
        }
    }
    if let Ok(end_col_offset) = node.getattr("end_col_offset") {
        if !end_col_offset.is_none() {
            dict.set_item("end_col_offset", end_col_offset)?;
        }
    }

    Ok(dict)
}

/// Serialize a Python value for JSON conversion
fn serialize_value<'py>(py: Python<'py>, value: &Bound<'py, PyAny>) -> PyResult<PyObject> {
    let ast_mod = py.import("ast")?;
    let ast_class = ast_mod.getattr("AST")?;

    // Check if it's an AST node
    if value.is_instance(&ast_class)? {
        return Ok(ast_node_to_dict(py, value)?.into_any().unbind());
    }

    // Check if it's a list
    if let Ok(list) = value.downcast::<PyList>() {
        let py_list = PyList::empty(py);
        for item in list.iter() {
            py_list.append(serialize_value(py, &item)?)?;
        }
        return Ok(py_list.into_any().unbind());
    }

    // Check if it's bytes - convert to list of integers for JSON serialization
    if value.get_type().name()? == "bytes" {
        let bytes: Vec<u8> = value.extract()?;
        let int_list: Vec<i64> = bytes.iter().map(|&b| b as i64).collect();
        let dict = PyDict::new(py);
        dict.set_item("_bytes", int_list)?;
        return Ok(dict.into_any().unbind());
    }

    // Check if it's a complex number
    if value.get_type().name()? == "complex" {
        let real: f64 = value.getattr("real")?.extract()?;
        let imag: f64 = value.getattr("imag")?.extract()?;
        let complex_dict = PyDict::new(py);
        complex_dict.set_item("real", real)?;
        complex_dict.set_item("imag", imag)?;
        let dict = PyDict::new(py);
        dict.set_item("_complex", complex_dict)?;
        return Ok(dict.into_any().unbind());
    }

    // Check for ellipsis
    if value.is(&py.Ellipsis()) {
        let dict = PyDict::new(py);
        dict.set_item("_ellipsis", true)?;
        return Ok(dict.into_any().unbind());
    }

    // For None, bool, int, float, str - return as-is (JSON-compatible)
    Ok(value.clone().unbind())
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
