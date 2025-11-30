/// CLI utilities for the TypePython compiler
use std::path::Path;
use std::process::Command;

/// Result of compile_and_run
pub enum RunResult {
    /// Compilation successful, program ran and returned exit code
    Completed(i32),
    /// Compilation failed with error message
    CompileError(String),
    /// Execution failed with error message
    ExecError(String),
}

/// Compile a TypePython file and optionally run it
/// Returns RunResult indicating success or failure
pub fn compile_and_run(input: &Path, output: Option<&Path>, compile_only: bool) -> RunResult {
    // Verify input file has .py extension
    if input.extension().and_then(|s| s.to_str()) != Some("py") {
        return RunResult::CompileError("Input file must have .py extension".to_string());
    }

    // Derive output path from input if not provided
    let default_output = input.with_extension("");
    let output_path = output.unwrap_or(&default_output);

    if let Err(e) = crate::pipeline::compile(input, output_path) {
        return RunResult::CompileError(e);
    }

    // Run the executable (unless compile-only mode)
    if !compile_only {
        match Command::new(output_path).status() {
            Ok(status) => RunResult::Completed(status.code().unwrap_or(1)),
            Err(e) => RunResult::ExecError(format!("Failed to run executable: {}", e)),
        }
    } else {
        RunResult::Completed(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_invalid_extension() {
        let input = PathBuf::from("test.txt");
        match compile_and_run(&input, None, false) {
            RunResult::CompileError(msg) => {
                assert!(msg.contains("extension"));
            }
            _ => panic!("Expected CompileError"),
        }
    }

    #[test]
    fn test_compile_and_run_success() {
        // Create a temp dir and copy a test file
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.py");

        std::fs::write(
            &test_file,
            r#"def test() -> None:
    print(42)

test()
"#,
        )
        .unwrap();

        let output = temp_dir.path().join("test");
        match compile_and_run(&test_file, Some(&output), false) {
            RunResult::Completed(code) => {
                assert_eq!(code, 0);
            }
            RunResult::CompileError(e) => panic!("Compile error: {}", e),
            RunResult::ExecError(e) => panic!("Exec error: {}", e),
        }
    }

    #[test]
    fn test_compile_only() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.py");

        std::fs::write(
            &test_file,
            r#"def test() -> None:
    print(1)

test()
"#,
        )
        .unwrap();

        let output = temp_dir.path().join("test");
        match compile_and_run(&test_file, Some(&output), true) {
            RunResult::Completed(0) => {
                // Should complete without running
                assert!(output.exists());
            }
            RunResult::CompileError(e) => panic!("Compile error: {}", e),
            _ => panic!("Expected Completed(0)"),
        }
    }

    #[test]
    fn test_compile_error() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("invalid.py");

        // Write invalid Python code that will fail to compile
        std::fs::write(
            &test_file,
            r#"def test() -> int:
    return "not an int"
"#,
        )
        .unwrap();

        match compile_and_run(&test_file, None, false) {
            RunResult::CompileError(_) => {
                // Expected
            }
            RunResult::Completed(_) => panic!("Expected CompileError"),
            RunResult::ExecError(_) => panic!("Expected CompileError"),
        }
    }
}
