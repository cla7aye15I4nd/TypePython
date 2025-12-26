use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("{}", format_multiple_errors(.0))]
    Multiple(Vec<CompilerError>),

    #[error("Type error at line {line}: {message}")]
    TypeError { line: usize, message: String },

    #[error("Type error: {0}")]
    TypeErrorSimple(String),

    #[error("Type inference error: {0}")]
    TypeInferenceError(String),

    #[error("Undefined variable: '{0}'")]
    UndefinedVariable(String),

    #[error("Undefined function: '{0}'")]
    UndefinedFunction(String),

    #[error("Variable '{0}' already defined in this scope")]
    DuplicateVariable(String),

    #[error("Missing type annotation for variable '{0}'")]
    MissingTypeAnnotation(String),

    #[error("Missing return type annotation for function '{0}'")]
    MissingReturnType(String),

    #[error("Return type mismatch in function '{func}': expected {expected}, found {found}")]
    ReturnTypeMismatch {
        func: String,
        expected: String,
        found: String,
    },

    #[error("Argument count mismatch for function '{func}': expected {expected}, found {found}")]
    ArgumentCountMismatch {
        func: String,
        expected: usize,
        found: usize,
    },

    #[error("Argument type mismatch for function '{func}' at position {position}: expected {expected}, found {found}")]
    ArgumentTypeMismatch {
        func: String,
        position: usize,
        expected: String,
        found: String,
    },

    #[error("LLVM error: {0}")]
    LLVMError(String),

    #[error("Codegen error: {0}")]
    CodegenError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Python error: {0}")]
    PythonError(#[from] pyo3::PyErr),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("Invalid AST node: {0}")]
    InvalidASTNode(String),

    #[error("Module not found: '{0}'")]
    ModuleNotFound(String),

    #[error("Cannot import '{name}' from module '{module}'")]
    ImportNameNotFound { module: String, name: String },

    #[error("Relative import outside package: '{0}'")]
    RelativeImportError(String),

    #[error("Circular import detected: {0}")]
    CircularImport(String),
}

pub type Result<T> = std::result::Result<T, CompilerError>;

/// Format multiple errors for display.
fn format_multiple_errors(errors: &[CompilerError]) -> String {
    if errors.is_empty() {
        return "No errors".to_string();
    }
    if errors.len() == 1 {
        return errors[0].to_string();
    }
    let mut result = format!("{} errors:\n", errors.len());
    for (i, err) in errors.iter().enumerate() {
        result.push_str(&format!("  {}. {}\n", i + 1, err));
    }
    result
}

/// Collector for accumulating multiple errors during compilation.
/// Allows the compiler to continue after recoverable errors and report all at once.
#[derive(Default)]
pub struct ErrorCollector {
    errors: Vec<CompilerError>,
}

impl ErrorCollector {
    /// Create a new empty error collector.
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Add an error to the collection.
    pub fn push(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    /// Check if any errors have been collected.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get the number of collected errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Check if the collector is empty.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Convert to a Result, returning Ok(()) if no errors, or Err with all errors.
    pub fn into_result(self) -> Result<()> {
        if self.errors.is_empty() {
            Ok(())
        } else if self.errors.len() == 1 {
            Err(self.errors.into_iter().next().unwrap())
        } else {
            Err(CompilerError::Multiple(self.errors))
        }
    }

    /// Convert to a Result with a value, returning Ok(value) if no errors.
    pub fn into_result_with<T>(self, value: T) -> Result<T> {
        if self.errors.is_empty() {
            Ok(value)
        } else if self.errors.len() == 1 {
            Err(self.errors.into_iter().next().unwrap())
        } else {
            Err(CompilerError::Multiple(self.errors))
        }
    }

    /// Try an operation, collecting any error instead of failing immediately.
    /// Returns true if the operation succeeded.
    pub fn try_collect<T>(&mut self, result: Result<T>) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(e) => {
                self.push(e);
                None
            }
        }
    }

    /// Get a reference to the collected errors.
    pub fn errors(&self) -> &[CompilerError] {
        &self.errors
    }
}
