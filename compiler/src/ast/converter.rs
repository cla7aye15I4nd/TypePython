//! Python AST to internal AST conversion

use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyList, PyListMethods, PyTypeMethods};

use crate::ast::types::*;
use crate::error::{CompilerError, Result};

// ============================================================================
// Import Types
// ============================================================================

/// An imported name with optional alias (e.g., `x` or `y as z`)
#[derive(Debug, Clone)]
pub struct ImportAlias {
    /// The name being imported ("*" for star imports)
    pub name: String,
    /// Optional alias (the `as` name)
    pub alias: Option<String>,
}

/// Unique identifier for a module (dotted path like "pkg.submodule")
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ModuleName(pub String);

impl ModuleName {
    pub fn new(name: impl Into<String>) -> Self {
        ModuleName(name.into())
    }
}

impl std::fmt::Display for ModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Information about an import statement
#[derive(Debug, Clone)]
pub struct ImportInfo {
    /// The original module name as written in the import statement
    pub source_name: String,
    /// The resolved module ID (global identifier based on file path)
    pub module_id: ModuleName,
    /// The resolved module file path
    pub module_path: std::path::PathBuf,
    /// What is being imported
    pub kind: ImportKind,
}

/// What kind of import this is
#[derive(Debug, Clone)]
pub enum ImportKind {
    /// `import x` - imports the module itself
    Module { alias: Option<String> },
    /// `from x import a, b` - imports specific names
    Names(Vec<ImportAlias>),
    /// `from x import *` - imports all public names
    Star,
}

// ============================================================================
// AST Converter
// ============================================================================

/// Converter from Python AST to internal AST
pub struct AstConverter {
    /// Search paths for module resolution (in order of priority)
    search_paths: Vec<std::path::PathBuf>,
}

impl AstConverter {
    /// Create a new converter with the entry file's directory as the primary search path
    pub fn new(entry_dir: &std::path::Path) -> Self {
        let search_paths = vec![entry_dir.to_path_buf()];
        AstConverter { search_paths }
    }

    // Module(stmt* body, type_ignore* type_ignores)
    pub fn convert_module(
        &self,
        py_ast: &Bound<'_, PyAny>,
        path: std::path::PathBuf,
        id: ModuleName,
    ) -> Result<Module> {
        Python::attach(|_py| {
            // Get the body field of the Module
            let py_stmts = self.get_list_attr(py_ast, "body");

            let mut stmts = Vec::new();
            let mut imports = Vec::new();

            // Process all statements, converting imports and other code
            for py_stmt in py_stmts.iter() {
                let class_name = py_stmt.get_type().name().unwrap();
                match class_name.to_string().as_str() {
                    "Import" => {
                        // Convert import statements
                        self.convert_import(&py_stmt, &path, &mut imports)?;
                    }
                    "ImportFrom" => {
                        // Convert import-from statements
                        self.convert_import_from(&py_stmt, &path, &mut imports)?;
                    }
                    _ => {
                        // Convert regular statements
                        stmts.push(self.convert_stmt(&py_stmt)?);
                    }
                }
            }

            let module = Module {
                id,
                path,
                imports,
                body: stmts,
            };

            Ok(module)
        })
    }

    // Import(alias* names)
    fn convert_import(
        &self,
        py_stmt: &Bound<'_, PyAny>,
        current_path: &std::path::Path,
        imports: &mut Vec<ImportInfo>,
    ) -> Result<()> {
        let py_names = self.get_list_attr(py_stmt, "names");

        for py_alias in py_names.iter() {
            let alias = self.convert_import_alias(&py_alias);

            // Resolve the module path (level 0 = absolute import)
            let module_path = self.resolve(&alias.name, current_path, 0)?;
            let module_id = ModuleName::new(self.path_to_module_id(&module_path));

            imports.push(ImportInfo {
                source_name: alias.name.clone(),
                module_id,
                module_path,
                kind: ImportKind::Module { alias: alias.alias },
            });
        }

        Ok(())
    }

    // ImportFrom(identifier? module, alias* names, int? level)
    fn convert_import_from(
        &self,
        py_stmt: &Bound<'_, PyAny>,
        current_path: &std::path::Path,
        imports: &mut Vec<ImportInfo>,
    ) -> Result<()> {
        let module = py_stmt
            .getattr("module")
            .ok()
            .and_then(|m| m.extract::<String>().ok());

        let level = py_stmt
            .getattr("level")
            .and_then(|l| l.extract::<usize>())
            .unwrap_or(0);

        let py_names = self.get_list_attr(py_stmt, "names");

        let mut names = Vec::new();
        for py_alias in py_names.iter() {
            names.push(self.convert_import_alias(&py_alias));
        }

        let module_name = module.as_deref().unwrap_or("");

        // Special case: from . import x, y (module_name is empty, level > 0)
        // Each imported name is treated as a separate module from the current directory
        if module_name.is_empty() && level > 0 {
            for alias in &names {
                // Star imports not supported - packages are just directories with no namespace
                if alias.name == "*" {
                    return Err(CompilerError::RelativeImportError(
                        "Cannot use 'from . import *' - packages have no namespace to import from"
                            .to_string(),
                    ));
                }

                // Each name is actually a module to import
                let relative_module_path = self.resolve(&alias.name, current_path, level)?;
                let relative_module_id =
                    ModuleName::new(self.path_to_module_id(&relative_module_path));

                // Use the provided alias, or default to the imported name itself
                // For "from . import helper", alias.name is "helper" and alias.alias is None
                // We want to bind it as "helper", not as the full module path
                let local_name = alias.alias.clone().or_else(|| Some(alias.name.clone()));

                imports.push(ImportInfo {
                    source_name: alias.name.clone(),
                    module_id: relative_module_id,
                    module_path: relative_module_path,
                    kind: ImportKind::Module { alias: local_name },
                });
            }
            return Ok(());
        }

        // Resolve the module path
        let module_path = self.resolve(module_name, current_path, level)?;
        let module_id = ModuleName::new(self.path_to_module_id(&module_path));

        imports.push(ImportInfo {
            source_name: module_name.to_string(),
            module_id,
            module_path,
            kind: if names.len() == 1 && names[0].name == "*" {
                ImportKind::Star
            } else {
                ImportKind::Names(names)
            },
        });

        Ok(())
    }

    // alias = (identifier name, identifier? asname)
    fn convert_import_alias(&self, node: &Bound<'_, pyo3::PyAny>) -> ImportAlias {
        let name = self.get_string_attr(node, "name");
        let asname = node.getattr("asname").unwrap();
        let alias = if asname.is_none() {
            None
        } else {
            Some(asname.extract::<String>().unwrap())
        };
        ImportAlias { name, alias }
    }

    // stmt = FunctionDef | ClassDef | Return | If | While | Assign | AnnAssign | AugAssign | Expr
    /// Convert a Python statement node
    /// Returns None for import statements (which are handled separately)
    fn convert_stmt(&self, py_stmt: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let class_name = py_stmt.get_type().name().unwrap();

            match class_name.to_string().as_str() {
                "FunctionDef" => self.convert_function_def(py_stmt),
                "ClassDef" => self.convert_class_def(py_stmt),
                "Return" => self.convert_return(py_stmt),
                "If" => self.convert_if(py_stmt),
                "While" => self.convert_while(py_stmt),
                "For" => self.convert_for(py_stmt),
                "Assign" => self.convert_assign(py_stmt),
                "AnnAssign" => self.convert_ann_assign(py_stmt),
                "AugAssign" => self.convert_aug_assign(py_stmt),
                "Expr" => self.convert_expr_stmt(py_stmt),
                "Try" => self.convert_try(py_stmt),
                "Raise" => self.convert_raise(py_stmt),
                _ => Err(CompilerError::UnsupportedFeature(format!(
                    "Unsupported statement type: {}",
                    class_name
                ))),
            }
        })
    }

    // FunctionDef(identifier name, arguments args, stmt* body, expr* decorator_list,
    //             expr? returns, string? type_comment, type_param* type_params)
    // arguments = (arg* posonlyargs, arg* args, arg? vararg, arg* kwonlyargs,
    //              expr* kw_defaults, arg? kwarg, expr* defaults)
    // arg = (identifier arg, expr? annotation, string? type_comment)
    fn convert_function_def(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let name = self.get_string_attr(node, "name");
            let py_args = node.getattr("args").unwrap();
            let py_args_list = self.get_list_attr(&py_args, "args");

            let mut args = Vec::new();
            for py_arg in py_args_list.iter() {
                let arg_name = self.get_string_attr(&py_arg, "arg");
                let annotation = self.get_optional_type_annotation(&py_arg, "annotation");
                args.push(Arg {
                    name: arg_name,
                    annotation,
                });
            }

            // Get return type annotation
            let return_type = self.get_optional_type_annotation(node, "returns");

            // Get function body
            let body = self.convert_stmt_list(node, "body")?;

            Ok(Stmt::FunctionDef {
                name,
                args,
                return_type,
                body,
            })
        })
    }

    // ClassDef(identifier name, expr* bases, keyword* keywords, stmt* body,
    //          expr* decorator_list, type_param* type_params)
    fn convert_class_def(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let name = self.get_string_attr(node, "name");
            let py_bases_list = self.get_list_attr(node, "bases");

            // Only single inheritance is supported
            let base = if py_bases_list.len() > 1 {
                return Err(CompilerError::UnsupportedFeature(
                    "Multiple inheritance is not supported".to_string(),
                ));
            } else if py_bases_list.len() == 1 {
                Some(self.get_name_id(&py_bases_list.get_item(0).unwrap()))
            } else {
                None
            };

            let py_body_list = self.get_list_attr(node, "body");

            let mut class_body = Vec::new();
            for py_item in py_body_list.iter() {
                let item_class = py_item.get_type().name().unwrap();

                match item_class.to_string().as_str() {
                    // AnnAssign(expr target, expr annotation, expr? value, int simple)
                    "AnnAssign" => {
                        // Field definition (e.g., x: int)
                        // Check that there's no value (pure annotation)
                        let py_value = py_item.getattr("value").unwrap();
                        if !py_value.is_none() {
                            return Err(CompilerError::UnsupportedFeature(
                                "Field initialization in class body not supported (use __init__)"
                                    .to_string(),
                            ));
                        }

                        let target = py_item.getattr("target").unwrap();
                        let field_name = self.get_name_id(&target);
                        let annotation = py_item.getattr("annotation").unwrap();
                        let field_type = self.get_type_annotation(&annotation)?;

                        class_body.push(ClassBodyItem::FieldDef {
                            name: field_name,
                            annotation: field_type,
                        });
                    }
                    // FunctionDef(identifier name, arguments args, stmt* body, ...)
                    "FunctionDef" => {
                        let method_name = self.get_string_attr(&py_item, "name");
                        let py_args = py_item.getattr("args").unwrap();
                        let py_args_list = self.get_list_attr(&py_args, "args");

                        let mut args = Vec::new();
                        for py_arg in py_args_list.iter() {
                            let arg_name = self.get_string_attr(&py_arg, "arg");
                            let annotation =
                                self.get_optional_type_annotation(&py_arg, "annotation");
                            args.push(Arg {
                                name: arg_name,
                                annotation,
                            });
                        }

                        let return_type = self.get_optional_type_annotation(&py_item, "returns");
                        let method_body = self.convert_stmt_list(&py_item, "body")?;

                        class_body.push(ClassBodyItem::MethodDef {
                            name: method_name,
                            args,
                            return_type,
                            body: method_body,
                        });
                    }
                    _ => {
                        return Err(CompilerError::UnsupportedFeature(format!(
                            "Unsupported class body item: {}",
                            item_class
                        )));
                    }
                }
            }

            Ok(Stmt::ClassDef {
                name,
                base,
                body: class_body,
            })
        })
    }

    // Return(expr? value)
    fn convert_return(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let py_value = node.getattr("value").unwrap();
            let value = if py_value.is_none() {
                None
            } else {
                Some(self.convert_expr(&py_value)?)
            };
            Ok(Stmt::Return { value })
        })
    }

    // If(expr test, stmt* body, stmt* orelse)
    fn convert_if(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let test = self.convert_expr(&node.getattr("test").unwrap())?;
            let body = self.convert_stmt_list(node, "body")?;
            let orelse = self.convert_stmt_list(node, "orelse")?;
            Ok(Stmt::If { test, body, orelse })
        })
    }

    // While(expr test, stmt* body, stmt* orelse)
    fn convert_while(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let test = self.convert_expr(&node.getattr("test").unwrap())?;
            let body = self.convert_stmt_list(node, "body")?;
            Ok(Stmt::While { test, body })
        })
    }

    // For(expr target, expr iter, stmt* body, stmt* orelse, string? type_comment)
    fn convert_for(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let py_target = node.getattr("target").unwrap();
            // For now, only support simple Name targets
            let target_class = py_target.get_type().name().unwrap();
            if target_class != "Name" {
                return Err(CompilerError::UnsupportedFeature(
                    "For loop target must be a simple variable name".to_string(),
                ));
            }
            let target = self.get_string_attr(&py_target, "id");
            let iter = self.convert_expr(&node.getattr("iter").unwrap())?;
            let body = self.convert_stmt_list(node, "body")?;
            Ok(Stmt::For { target, iter, body })
        })
    }

    // Try(stmt* body, excepthandler* handlers, stmt* orelse, stmt* finalbody)
    fn convert_try(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let body = self.convert_stmt_list(node, "body")?;
            let handlers = self.convert_except_handlers(node)?;
            let orelse = self.convert_stmt_list(node, "orelse")?;
            let finalbody = self.convert_stmt_list(node, "finalbody")?;

            Ok(Stmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            })
        })
    }

    // ExceptHandler(expr? type, identifier? name, stmt* body)
    fn convert_except_handlers(&self, node: &Bound<'_, PyAny>) -> Result<Vec<ExceptHandler>> {
        Python::attach(|_py| {
            let py_handlers = self.get_list_attr(node, "handlers");
            let mut handlers = Vec::new();

            for py_handler in py_handlers.iter() {
                let py_type = py_handler.getattr("type").unwrap();
                let exc_type = if py_type.is_none() {
                    None
                } else {
                    // Exception type should be a Name node
                    Some(self.get_name_id(&py_type))
                };

                let py_name = py_handler.getattr("name").unwrap();
                let name = if py_name.is_none() {
                    None
                } else {
                    Some(py_name.extract::<String>().unwrap())
                };

                let handler_body = self.convert_stmt_list(&py_handler, "body")?;

                handlers.push(ExceptHandler {
                    exc_type,
                    name,
                    body: handler_body,
                });
            }

            Ok(handlers)
        })
    }

    // Raise(expr? exc, expr? cause)
    fn convert_raise(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let py_exc = node.getattr("exc").unwrap();
            let exc = if py_exc.is_none() {
                None
            } else {
                Some(self.convert_expr(&py_exc)?)
            };

            // Note: 'cause' (raise X from Y) is not supported
            Ok(Stmt::Raise { exc })
        })
    }

    // Assign(expr* targets, expr value, string? type_comment)
    fn convert_assign(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let targets_list = self.get_list_attr(node, "targets");

            if targets_list.len() != 1 {
                return Err(CompilerError::UnsupportedFeature(
                    "Multiple assignment targets not supported".to_string(),
                ));
            }

            let target = targets_list.get_item(0).unwrap();
            let target_expr = self.convert_expr(&target)?;
            let value = self.convert_expr(&node.getattr("value").unwrap())?;

            Ok(Stmt::Assign {
                target: target_expr,
                value,
                type_annotation: None,
            })
        })
    }

    // AnnAssign(expr target, expr annotation, expr? value, int simple)
    fn convert_ann_assign(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let target = node.getattr("target").unwrap();
            let target_expr = self.convert_expr(&target)?;
            let annotation = self.get_type_annotation(&node.getattr("annotation").unwrap())?;

            let py_value = node.getattr("value").unwrap();
            if py_value.is_none() {
                return Err(CompilerError::UnsupportedFeature(
                    "AnnAssign without value not supported".to_string(),
                ));
            }
            let value = self.convert_expr(&py_value)?;

            Ok(Stmt::Assign {
                target: target_expr,
                value,
                type_annotation: Some(annotation),
            })
        })
    }

    // AugAssign(expr target, operator op, expr value)
    fn convert_aug_assign(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let target = node.getattr("target").unwrap();
            let target_name = self.get_name_id(&target);
            let op = self.convert_bin_operator(&node.getattr("op").unwrap())?;
            let value = self.convert_expr(&node.getattr("value").unwrap())?;

            Ok(Stmt::AugAssign {
                target: target_name,
                op,
                value,
            })
        })
    }

    // Expr(expr value)
    fn convert_expr_stmt(&self, node: &Bound<'_, PyAny>) -> Result<Stmt> {
        Python::attach(|_py| {
            let value = self.convert_expr(&node.getattr("value").unwrap())?;
            Ok(Stmt::Expr { value })
        })
    }

    // expr = Constant | Name | BinOp | Compare | BoolOp | UnaryOp | Call | List | Subscript | Attribute
    fn convert_expr(&self, py_expr: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let class_name = py_expr.get_type().name().unwrap();

            match class_name.to_string().as_str() {
                "Constant" => self.convert_constant(py_expr),
                "Name" => self.convert_name(py_expr),
                "BinOp" => self.convert_binop(py_expr),
                "Compare" => self.convert_compare(py_expr),
                "BoolOp" => self.convert_boolop(py_expr),
                "UnaryOp" => self.convert_unaryop(py_expr),
                "Call" => self.convert_call(py_expr),
                "List" => self.convert_list(py_expr),
                "Subscript" => self.convert_subscript(py_expr),
                "Attribute" => self.convert_attribute(py_expr),
                _ => Err(CompilerError::UnsupportedFeature(format!(
                    "Unsupported expression type: {}",
                    class_name
                ))),
            }
        })
    }

    // Constant(constant value, string? kind)
    fn convert_constant(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let value = node.getattr("value").unwrap();

            // Check bool BEFORE int because Python's bool is a subclass of int
            if let Ok(bool_val) = value.extract::<bool>() {
                Ok(Expr::Constant(Constant::Bool(bool_val)))
            } else if let Ok(int_val) = value.extract::<i64>() {
                Ok(Expr::Constant(Constant::Int(int_val)))
            } else if let Ok(float_val) = value.extract::<f64>() {
                Ok(Expr::Constant(Constant::Float(float_val)))
            } else if let Ok(str_val) = value.extract::<String>() {
                Ok(Expr::Constant(Constant::Str(str_val)))
            } else if let Ok(bytes_val) = value.extract::<Vec<u8>>() {
                Ok(Expr::Constant(Constant::Bytes(bytes_val)))
            } else {
                Err(CompilerError::UnsupportedFeature(
                    "Only integer, float, string, boolean, and bytes constants are supported"
                        .to_string(),
                ))
            }
        })
    }

    // Name(identifier id, expr_context ctx)
    fn convert_name(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        let id = self.get_string_attr(node, "id");
        Ok(Expr::Name(id))
    }

    // BinOp(expr left, operator op, expr right)
    fn convert_binop(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let left = self.convert_expr(&node.getattr("left").unwrap())?;
            let op = self.convert_bin_operator(&node.getattr("op").unwrap())?;
            let right = self.convert_expr(&node.getattr("right").unwrap())?;

            Ok(Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        })
    }

    // Compare(expr left, cmpop* ops, expr* comparators)
    fn convert_compare(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let left = self.convert_expr(&node.getattr("left").unwrap())?;
            let ops_pylist = self.get_list_attr(node, "ops");
            let comparators_pylist = self.get_list_attr(node, "comparators");

            let mut ops = Vec::new();
            for py_op in ops_pylist.iter() {
                ops.push(self.convert_compare_op(&py_op)?);
            }

            let mut comparators = Vec::new();
            for py_comp in comparators_pylist.iter() {
                comparators.push(self.convert_expr(&py_comp)?);
            }

            Ok(Expr::Compare {
                left: Box::new(left),
                ops,
                comparators,
            })
        })
    }

    // BoolOp(boolop op, expr* values)
    fn convert_boolop(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let py_op = node.getattr("op").unwrap();
            let op_name = py_op.get_type().name().unwrap();
            let op = match op_name.to_string().as_str() {
                "And" => BoolOp::And,
                "Or" => BoolOp::Or,
                _ => {
                    return Err(CompilerError::UnsupportedFeature(format!(
                        "Unsupported boolean operator: {}",
                        op_name
                    )))
                }
            };

            let values_pylist = self.get_list_attr(node, "values");
            let mut values = Vec::new();
            for py_val in values_pylist.iter() {
                values.push(self.convert_expr(&py_val)?);
            }

            Ok(Expr::BoolOp { op, values })
        })
    }

    // UnaryOp(unaryop op, expr operand)
    fn convert_unaryop(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let py_op = node.getattr("op").unwrap();
            let op_name = py_op.get_type().name().unwrap();
            let op = match op_name.to_string().as_str() {
                "Not" => UnaryOp::Not,
                "USub" => UnaryOp::USub,
                _ => {
                    return Err(CompilerError::UnsupportedFeature(format!(
                        "Unsupported unary operator: {}",
                        op_name
                    )))
                }
            };

            let operand = self.convert_expr(&node.getattr("operand").unwrap())?;
            Ok(Expr::UnaryOp {
                op,
                operand: Box::new(operand),
            })
        })
    }

    // Call(expr func, expr* args, keyword* keywords)
    fn convert_call(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let func = self.convert_expr(&node.getattr("func").unwrap())?;
            let args_pylist = self.get_list_attr(node, "args");

            let mut args = Vec::new();
            for py_arg in args_pylist.iter() {
                args.push(self.convert_expr(&py_arg)?);
            }

            Ok(Expr::Call {
                func: Box::new(func),
                args,
            })
        })
    }

    // List(expr* elts, expr_context ctx)
    fn convert_list(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let elts_pylist = self.get_list_attr(node, "elts");

            let mut elts = Vec::new();
            for py_elt in elts_pylist.iter() {
                elts.push(self.convert_expr(&py_elt)?);
            }

            Ok(Expr::List { elts })
        })
    }

    // Subscript(expr value, expr slice, expr_context ctx)
    fn convert_subscript(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let value = self.convert_expr(&node.getattr("value").unwrap())?;
            let index = self.convert_expr(&node.getattr("slice").unwrap())?;

            Ok(Expr::Subscript {
                value: Box::new(value),
                index: Box::new(index),
            })
        })
    }

    // Attribute(expr value, identifier attr, expr_context ctx)
    fn convert_attribute(&self, node: &Bound<'_, PyAny>) -> Result<Expr> {
        Python::attach(|_py| {
            let value = self.convert_expr(&node.getattr("value").unwrap())?;
            let attr = self.get_string_attr(node, "attr");

            Ok(Expr::Attribute {
                value: Box::new(value),
                attr,
            })
        })
    }

    // operator = Add | Sub | Mult | MatMult | Div | Mod | Pow | LShift
    //          | RShift | BitOr | BitXor | BitAnd | FloorDiv
    fn convert_bin_operator(&self, py_op: &Bound<'_, PyAny>) -> Result<BinOperator> {
        Python::attach(|_py| {
            let class_name = py_op.get_type().name().unwrap();

            match class_name.to_string().as_str() {
                "Add" => Ok(BinOperator::Add),
                "Sub" => Ok(BinOperator::Sub),
                "Mult" => Ok(BinOperator::Mult),
                "Div" => Ok(BinOperator::Div),
                "FloorDiv" => Ok(BinOperator::FloorDiv),
                "Mod" => Ok(BinOperator::Mod),
                "Pow" => Ok(BinOperator::Pow),
                "LShift" => Ok(BinOperator::LShift),
                "RShift" => Ok(BinOperator::RShift),
                "BitOr" => Ok(BinOperator::BitOr),
                "BitXor" => Ok(BinOperator::BitXor),
                "BitAnd" => Ok(BinOperator::BitAnd),
                _ => Err(CompilerError::UnsupportedFeature(format!(
                    "Unsupported binary operator: {}",
                    class_name
                ))),
            }
        })
    }

    // cmpop = Eq | NotEq | Lt | LtE | Gt | GtE | Is | IsNot | In | NotIn
    fn convert_compare_op(&self, py_op: &Bound<'_, PyAny>) -> Result<CompareOp> {
        Python::attach(|_py| {
            let class_name = py_op.get_type().name().unwrap();

            match class_name.to_string().as_str() {
                "Eq" => Ok(CompareOp::Eq),
                "NotEq" => Ok(CompareOp::NotEq),
                "Lt" => Ok(CompareOp::Lt),
                "LtE" => Ok(CompareOp::LtE),
                "Gt" => Ok(CompareOp::Gt),
                "GtE" => Ok(CompareOp::GtE),
                _ => Err(CompilerError::UnsupportedFeature(format!(
                    "Unsupported comparison operator: {}",
                    class_name
                ))),
            }
        })
    }

    // Type annotations use Name or Subscript (e.g., int, list[int])
    fn get_type_annotation(&self, py_annot: &Bound<'_, PyAny>) -> Result<TypeAnnotation> {
        Python::attach(|_py| {
            let class_name = py_annot.get_type().name().unwrap();

            match class_name.to_string().as_str() {
                "Name" => {
                    let id = self.get_string_attr(py_annot, "id");
                    match id.as_str() {
                        "int" => Ok(TypeAnnotation::Int),
                        "float" => Ok(TypeAnnotation::Float),
                        "str" => Ok(TypeAnnotation::Str),
                        "bool" => Ok(TypeAnnotation::Bool),
                        "bytes" => Ok(TypeAnnotation::Bytes),
                        "bytearray" => Ok(TypeAnnotation::ByteArray),
                        class_name => Ok(TypeAnnotation::ClassName(class_name.to_string())),
                    }
                }
                "Subscript" => {
                    let slice = py_annot.getattr("slice").unwrap();
                    let inner_type = self.get_type_annotation(&slice)?;
                    Ok(TypeAnnotation::List(Box::new(inner_type)))
                }
                // Handle string annotations (forward references) like "ClassName"
                "Constant" => {
                    let value = py_annot.getattr("value").unwrap();
                    if let Ok(s) = value.extract::<String>() {
                        // Treat string annotation the same as a bare name
                        match s.as_str() {
                            "int" => Ok(TypeAnnotation::Int),
                            "float" => Ok(TypeAnnotation::Float),
                            "str" => Ok(TypeAnnotation::Str),
                            "bool" => Ok(TypeAnnotation::Bool),
                            "bytes" => Ok(TypeAnnotation::Bytes),
                            "bytearray" => Ok(TypeAnnotation::ByteArray),
                            class_name => Ok(TypeAnnotation::ClassName(class_name.to_string())),
                        }
                    } else {
                        Err(CompilerError::UnsupportedFeature(
                            "String annotation must be a string literal".to_string(),
                        ))
                    }
                }
                _ => Err(CompilerError::UnsupportedFeature(format!(
                    "Unsupported annotation type: {}",
                    class_name
                ))),
            }
        })
    }

    /// Get optional type annotation (returns None if attribute is None or missing)
    fn get_optional_type_annotation(
        &self,
        node: &Bound<'_, PyAny>,
        attr: &str,
    ) -> Option<TypeAnnotation> {
        Python::attach(|_| match node.getattr(attr) {
            Ok(py_annot) => {
                if py_annot.is_none() {
                    None
                } else {
                    // Check if this is a Constant with value None (for -> None return type)
                    if let Ok(value) = py_annot.getattr("value") {
                        if value.is_none() {
                            return None;
                        }
                    }

                    Some(self.get_type_annotation(&py_annot).unwrap())
                }
            }
            Err(_) => None,
        })
    }

    // Helper: Get string attribute (only call for required fields)
    fn get_string_attr(&self, node: &Bound<'_, PyAny>, attr: &str) -> String {
        node.getattr(attr).unwrap().extract::<String>().unwrap()
    }

    // Helper: Get list attribute (only call for required fields)
    fn get_list_attr<'py>(&self, node: &Bound<'py, PyAny>, attr: &str) -> Bound<'py, PyList> {
        node.getattr(attr).unwrap().cast_into::<PyList>().unwrap()
    }

    // Helper: Get name id from Name node
    fn get_name_id(&self, node: &Bound<'_, PyAny>) -> String {
        self.get_string_attr(node, "id")
    }

    // Helper: Convert a list of statements
    fn convert_stmt_list(&self, node: &Bound<'_, PyAny>, attr: &str) -> Result<Vec<Stmt>> {
        let py_list = self.get_list_attr(node, attr);
        let mut stmts = Vec::new();
        for py_stmt in py_list.iter() {
            stmts.push(self.convert_stmt(&py_stmt)?);
        }
        Ok(stmts)
    }

    // ============================================================================
    // Module Resolution
    // ============================================================================

    /// Resolve a module name to a file path
    ///
    /// # Arguments
    /// * `module_name` - Dotted module name (e.g., "mypackage.submodule")
    /// * `from_file` - The file that is importing (for relative imports)
    /// * `level` - Relative import level (0 = absolute, 1 = from ., 2 = from .., etc.)
    ///
    /// # Returns
    /// The resolved file path, or an error if not found
    pub fn resolve(
        &self,
        module_name: &str,
        from_file: &std::path::Path,
        level: usize,
    ) -> Result<std::path::PathBuf> {
        if level > 0 {
            // Relative import
            self.resolve_relative(module_name, from_file, level)
        } else {
            // Absolute import
            self.resolve_absolute(module_name)
        }
    }

    /// Resolve an absolute import (level = 0)
    fn resolve_absolute(&self, module_name: &str) -> Result<std::path::PathBuf> {
        let parts: Vec<&str> = module_name.split('.').collect();

        for search_path in &self.search_paths {
            if let Some(resolved) = self.try_resolve_in_dir(search_path, &parts) {
                return Ok(resolved);
            }
        }

        Err(CompilerError::ModuleNotFound(module_name.to_string()))
    }

    /// Resolve a relative import (level > 0)
    fn resolve_relative(
        &self,
        module_name: &str,
        from_file: &std::path::Path,
        level: usize,
    ) -> Result<std::path::PathBuf> {
        use std::path::Path;
        // Start from the directory containing the importing file
        let mut base_dir = from_file.parent().unwrap_or(Path::new(".")).to_path_buf();

        // Go up `level - 1` directories (level 1 = same package, level 2 = parent package, etc.)
        for _ in 0..(level - 1) {
            base_dir = base_dir
                .parent()
                .ok_or_else(|| {
                    CompilerError::RelativeImportError(format!(
                        "Cannot go {} levels up from {:?}",
                        level, from_file
                    ))
                })?
                .to_path_buf();
        }

        // Empty module_name (e.g., `from . import x`) is handled in convert_import_from()
        // where each imported name is resolved as a separate module
        if module_name.is_empty() {
            return Err(CompilerError::RelativeImportError(
                "Empty module name should be handled in convert_import_from".to_string(),
            ));
        }

        let parts: Vec<&str> = module_name.split('.').collect();
        if let Some(resolved) = self.try_resolve_in_dir(&base_dir, &parts) {
            return Ok(resolved);
        }

        Err(CompilerError::ModuleNotFound(format!(
            "{} (relative import from {:?})",
            module_name, from_file
        )))
    }

    /// Try to resolve module parts in a directory
    /// Returns the path if found, None otherwise
    fn try_resolve_in_dir(
        &self,
        dir: &std::path::Path,
        parts: &[&str],
    ) -> Option<std::path::PathBuf> {
        if parts.is_empty() {
            return None;
        }

        let mut current = dir.to_path_buf();

        // Navigate to the correct subdirectory for all but the last part
        for &part in &parts[..parts.len() - 1] {
            current = current.join(part);
            if !current.is_dir() {
                return None;
            }
            // Any directory is treated as a package (directories are packages, modules are .py files)
        }

        let last_part = parts[parts.len() - 1];

        // Try as a module file: dir/module.py
        let module_file = current.join(format!("{}.py", last_part));
        if module_file.exists() {
            return Some(module_file);
        }

        // Packages are just directories - cannot import a directory directly
        None
    }

    /// Convert a file path back to a module ID
    pub fn path_to_module_id(&self, path: &std::path::Path) -> String {
        // Canonicalize input path if possible
        let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

        // Find which search path this file is under
        for search_path in &self.search_paths {
            // Canonicalize search path for comparison
            if let Ok(canonical_search) = search_path.canonicalize() {
                if let Ok(relative) = path.strip_prefix(&canonical_search) {
                    return self.relative_path_to_module_id(relative);
                }
            }
        }

        // Fallback: use the file stem
        path.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Convert a relative path to a module ID
    fn relative_path_to_module_id(&self, relative: &std::path::Path) -> String {
        let mut parts = Vec::new();

        for component in relative.components() {
            if let std::path::Component::Normal(s) = component {
                let s = s.to_string_lossy();
                // Remove .py extension from module files
                let name = s.strip_suffix(".py").unwrap_or(&s);
                parts.push(name.to_string());
            }
        }

        parts.join(".")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::python_ast::parser::parse_python;

    #[test]
    fn test_convert_simple_assign() {
        let source = "x: int = 5";
        let py_ast = parse_python(source).unwrap();

        let temp_dir = std::env::temp_dir();
        let converter = AstConverter::new(&temp_dir);
        Python::attach(|py| {
            let result = converter.convert_module(
                py_ast.bind(py),
                std::path::PathBuf::from("test.py"),
                ModuleName::new("test"),
            );
            assert!(result.is_ok());
            let module = result.unwrap();
            assert_eq!(module.body.len(), 1);
        });
    }

    #[test]
    fn test_convert_function() {
        let source = r#"
def add(a: int, b: int) -> int:
    return a + b
"#;
        let py_ast = parse_python(source).unwrap();

        let temp_dir = std::env::temp_dir();
        let converter = AstConverter::new(&temp_dir);
        Python::attach(|py| {
            let result = converter.convert_module(
                py_ast.bind(py),
                std::path::PathBuf::from("test.py"),
                ModuleName::new("test"),
            );
            assert!(result.is_ok());
            let module = result.unwrap();
            assert_eq!(module.body.len(), 1);
        });
    }
}
