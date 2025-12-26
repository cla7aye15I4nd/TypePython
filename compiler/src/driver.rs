use inkwell::context::Context;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use pyo3::Python;

use crate::ast::{AstConverter, Module, ModuleName};
use crate::codegen::generator::Codegen;
use crate::error::{CompilerError, Result};
use crate::python_ast::parse_python;
use crate::tir::lower_to_tir;

/// Target-specific configuration
struct TargetConfig {
    triple: &'static str,
    clang_target: Option<&'static str>,
    runtime_filename: &'static str,
    qemu_command: Option<&'static str>,
    musl_lib_path: &'static str,
}

const X86_64_CONFIG: TargetConfig = TargetConfig {
    triple: "x86_64-unknown-linux-musl",
    clang_target: None,
    runtime_filename: "runtime-x86_64.o",
    qemu_command: None,
    musl_lib_path: runtime::MUSL_X86_64_LIB,
};

const RISCV64_CONFIG: TargetConfig = TargetConfig {
    triple: "riscv64-unknown-linux-musl",
    clang_target: Some("--target=riscv64-linux-musl"),
    runtime_filename: "runtime-riscv64.o",
    qemu_command: Some("qemu-riscv64"),
    musl_lib_path: runtime::MUSL_RISCV64_LIB,
};

/// Target architecture for compilation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Target {
    #[default]
    X86_64,
    RiscV64,
}

impl Target {
    fn config(&self) -> &'static TargetConfig {
        match self {
            Target::X86_64 => &X86_64_CONFIG,
            Target::RiscV64 => &RISCV64_CONFIG,
        }
    }

    pub fn triple(&self) -> &'static str {
        self.config().triple
    }

    pub fn clang_target(&self) -> Option<&'static str> {
        self.config().clang_target
    }

    pub fn runtime_filename(&self) -> &'static str {
        self.config().runtime_filename
    }

    pub fn qemu_command(&self) -> Option<&'static str> {
        self.config().qemu_command
    }

    /// Get the musl library directory (set at compile time by runtime crate)
    pub fn musl_lib_dir(&self) -> PathBuf {
        PathBuf::from(self.config().musl_lib_path)
    }

    fn find_workspace_root() -> Option<PathBuf> {
        // Try CARGO_MANIFEST_DIR first (available during tests)
        if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            let mut path = PathBuf::from(manifest_dir);
            if path.pop() && path.join("Cargo.toml").exists() {
                return Some(path);
            }
        }

        // Walk up from current directory
        let mut current = env::current_dir().ok()?;
        loop {
            let cargo_toml = current.join("Cargo.toml");
            if cargo_toml.exists()
                && fs::read_to_string(&cargo_toml)
                    .map(|s| s.contains("[workspace]"))
                    .unwrap_or(false)
            {
                return Some(current);
            }
            if !current.pop() {
                break;
            }
        }
        None
    }
}

impl FromStr for Target {
    type Err = CompilerError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "x86_64" | "x86-64" | "amd64" => Ok(Target::X86_64),
            "riscv64" | "riscv" | "riscv64gc" => Ok(Target::RiscV64),
            _ => Err(CompilerError::CodegenError(format!(
                "Unknown target '{s}'. Supported: x86_64, riscv64"
            ))),
        }
    }
}

/// Build all modules starting from an entry file (handles cyclic imports)
pub fn build_modules(
    entry_path: &Path,
    entry_dir: &Path,
) -> Result<(HashMap<ModuleName, Module>, ModuleName)> {
    let mut modules = HashMap::new();
    let mut visited = HashSet::new();
    let converter = AstConverter::new(entry_dir);

    let entry_name = parse_module_recursive(entry_path, &converter, &mut modules, &mut visited)?;
    Ok((modules, entry_name))
}

fn parse_module_recursive(
    path: &Path,
    converter: &AstConverter,
    modules: &mut HashMap<ModuleName, Module>,
    visited: &mut HashSet<PathBuf>,
) -> Result<ModuleName> {
    let module_name = ModuleName::new(converter.path_to_module_id(path));
    if visited.contains(path) {
        return Ok(module_name);
    }
    visited.insert(path.to_path_buf());

    let source = fs::read_to_string(path).unwrap();
    let py_ast = parse_python(&source)?;

    let parsed = Python::attach(|py| {
        converter.convert_module(py_ast.bind(py), path.to_path_buf(), module_name.clone())
    })?;

    let dep_paths: Vec<_> = parsed
        .imports
        .iter()
        .map(|i| i.module_path.clone())
        .collect();
    modules.insert(module_name.clone(), parsed);

    for dep_path in dep_paths {
        let _ = parse_module_recursive(&dep_path, converter, modules, visited);
    }

    Ok(module_name)
}

/// Compiler configuration options
#[derive(Default)]
pub struct CompilerOptions {
    pub emit_ast: bool,
    pub emit_llvm: bool,
    pub target: Target,
}

/// Main compiler - orchestrates parsing, type checking, codegen, and linking
pub struct Compiler {
    options: CompilerOptions,
}

impl Compiler {
    pub fn new(options: CompilerOptions) -> Self {
        Self { options }
    }

    /// Compile a Python source file to an executable
    pub fn compile(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        self.with_llvm_module(input_path, |module| {
            self.link_executable(module, output_path)
        })
    }

    /// Compile and run a Python file
    pub fn run(&self, input_path: &Path, args: &[String]) -> Result<()> {
        let temp_exe = env::temp_dir().join("pyc_temp_output");
        self.compile(input_path, &temp_exe)?;
        self.execute(&temp_exe, args)
    }

    fn with_llvm_module<F>(&self, input_path: &Path, f: F) -> Result<()>
    where
        F: for<'ctx> FnOnce(&inkwell::module::Module<'ctx>) -> Result<()>,
    {
        let canonical = self.validate_input(input_path)?;
        let entry_dir = canonical.parent().unwrap();

        let (modules, entry_name) = build_modules(&canonical, entry_dir)?;
        if self.options.emit_ast {
            for module in modules.values() {
                println!("=== Module {} AST ===\n{:#?}", module.id, module);
            }
        }

        let tir_program = lower_to_tir(modules, entry_name)?;
        let context = Context::create();
        let codegen = Codegen::new(&context, self.options.target);
        let llvm_module = codegen.codegen_tir(&tir_program);

        if self.options.emit_llvm {
            println!(
                "=== LLVM IR ===\n{}",
                llvm_module.print_to_string().to_string()
            );
        }

        f(&llvm_module)
    }

    fn validate_input(&self, input_path: &Path) -> Result<PathBuf> {
        let canonical = input_path.canonicalize().map_err(|e| {
            CompilerError::IOError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Input file not found {}: {e}", input_path.display()),
            ))
        })?;

        if !canonical.is_file() {
            return Err(CompilerError::IOError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Input path must be a file, not a directory: {}",
                    input_path.display()
                ),
            )));
        }

        let is_py = canonical
            .extension()
            .and_then(|s| s.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("py"))
            .unwrap_or(false);

        if !is_py {
            return Err(CompilerError::IOError(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "Input file must be a Python (.py) file: {}",
                    input_path.display()
                ),
            )));
        }

        Ok(canonical)
    }

    fn link_executable<'ctx>(
        &self,
        llvm_module: &inkwell::module::Module<'ctx>,
        output_path: &Path,
    ) -> Result<()> {
        let runtime_path = self.find_runtime_library()?;
        let musl_lib = self.options.target.musl_lib_dir();
        let bc_path = output_path.with_extension("bc");

        llvm_module.write_bitcode_to_path(&bc_path);

        let mut cmd = Command::new("clang");
        cmd.args([&bc_path, &runtime_path])
            .arg("-o")
            .arg(output_path)
            .args(["-flto", "-O2", "-static", "-nostdlib"])
            .arg(musl_lib.join("crt1.o"))
            .arg(musl_lib.join("crti.o"))
            .arg(format!("-L{}", musl_lib.display()))
            .arg("-lc")
            .arg(musl_lib.join("crtn.o"));

        if let Some(target_flag) = self.options.target.clang_target() {
            cmd.args([target_flag, "-fuse-ld=lld"]);
            if matches!(self.options.target, Target::RiscV64) {
                cmd.args(["-mabi=lp64d", "--rtlib=libgcc"]);
                cmd.args(["-L/usr/lib/gcc-cross/riscv64-linux-gnu/13", "-lgcc"]);
            }
        }

        let output = cmd.output().map_err(CompilerError::IOError)?;
        let _ = fs::remove_file(&bc_path);

        if !output.status.success() {
            return Err(CompilerError::CodegenError(format!(
                "Linking failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        Ok(())
    }

    fn find_runtime_library(&self) -> Result<PathBuf> {
        let workspace = Target::find_workspace_root().ok_or_else(|| {
            CompilerError::IOError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find workspace root",
            ))
        })?;

        let target_dir = workspace.join("target");
        let filename = self.options.target.runtime_filename();

        // Search in both regular and llvm-cov target directories
        for subdir in ["", "llvm-cov-target"] {
            let base = if subdir.is_empty() {
                target_dir.clone()
            } else {
                target_dir.join(subdir)
            };
            for profile in ["debug", "release"] {
                let build_dir = base.join(profile).join("build");
                if let Ok(entries) = fs::read_dir(&build_dir) {
                    for entry in entries.flatten() {
                        if entry.file_name().to_string_lossy().starts_with("runtime-") {
                            let lib_path = entry.path().join("out").join(filename);
                            if lib_path.exists() {
                                return Ok(lib_path);
                            }
                        }
                    }
                }
            }
        }

        Err(CompilerError::IOError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{filename} not found. Run 'cargo build -p runtime' first."),
        )))
    }

    fn execute(&self, exe_path: &Path, args: &[String]) -> Result<()> {
        let status = match self.options.target.qemu_command() {
            Some(qemu) => Command::new(qemu).arg(exe_path).args(args).status(),
            None => Command::new(exe_path).args(args).status(),
        }
        .map_err(CompilerError::IOError)?;

        if !status.success() {
            return Err(CompilerError::CodegenError(format!(
                "Program exited with status: {status}"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_temp_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
        let path = dir.path().join(name);
        File::create(&path)
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();
        path
    }

    #[test]
    fn test_valid_py_file_uppercase() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = create_temp_file(&temp_dir, "test.PY", "print('hello')");
        let compiler = Compiler::new(CompilerOptions::default());
        assert!(compiler
            .compile(&file_path, &temp_dir.path().join("output"))
            .is_ok());
    }

    #[test]
    fn test_valid_py_file_mixedcase() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = create_temp_file(&temp_dir, "test.Py", "print('hello')");
        let compiler = Compiler::new(CompilerOptions::default());
        assert!(compiler
            .compile(&file_path, &temp_dir.path().join("output"))
            .is_ok());
    }

    #[test]
    fn test_invalid_extension_txt() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = create_temp_file(&temp_dir, "test.txt", "print('hello')");
        let compiler = Compiler::new(CompilerOptions::default());
        let result = compiler.compile(&file_path, &temp_dir.path().join("output"));
        assert!(result.is_err());
        assert!(format!("{:?}", result.unwrap_err()).contains("must be a Python (.py) file"));
    }

    #[test]
    fn test_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.py");
        let compiler = Compiler::new(CompilerOptions::default());
        let result = compiler.compile(&file_path, &temp_dir.path().join("output"));
        assert!(result.is_err());
        let err = format!("{:?}", result.unwrap_err());
        assert!(err.contains("Input file not found"));
        assert!(!err.contains("must be a Python (.py) file"));
    }

    #[test]
    fn test_directory_instead_of_file() {
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("mydir.py");
        fs::create_dir(&subdir).unwrap();
        let compiler = Compiler::new(CompilerOptions::default());
        let result = compiler.compile(&subdir, &temp_dir.path().join("output"));
        assert!(result.is_err());
        assert!(format!("{:?}", result.unwrap_err()).contains("must be a file, not a directory"));
    }

    #[test]
    fn test_symlink_to_py_file() {
        use std::os::unix::fs::symlink;
        let temp_dir = TempDir::new().unwrap();
        let file_path = create_temp_file(&temp_dir, "test.py", "print('hello')");
        let symlink_path = temp_dir.path().join("link.py");
        symlink(&file_path, &symlink_path).unwrap();
        let compiler = Compiler::new(CompilerOptions::default());
        assert!(compiler
            .compile(&symlink_path, &temp_dir.path().join("output"))
            .is_ok());
    }
}
