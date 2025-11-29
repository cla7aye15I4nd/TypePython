/// CLI tests for main.rs to improve coverage
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper to get the path to the compiled tpy binary
fn get_tpy_binary() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mut path = PathBuf::from(manifest_dir);
    path.push("target");
    path.push("debug");
    path.push("tpy");
    path
}

/// Helper to create a temporary Python file
fn create_temp_py_file(content: &str, name: &str, dir: &TempDir) -> PathBuf {
    let file_path = dir.path().join(name);
    fs::write(&file_path, content).unwrap();
    file_path
}

#[test]
fn test_cli_invalid_file_extension() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_py_file("print(42)", "test.txt", &temp_dir);

    let output = Command::new(get_tpy_binary())
        .arg(&file_path)
        .output()
        .expect("Failed to execute tpy");

    assert!(!output.status.success());
    assert_eq!(output.status.code().unwrap(), 1);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(".py") || stderr.contains("extension"));
}

// Note: dump-pp, dump-pest, dump-ast, and dump-ir flags are defined in the CLI
// but not yet fully implemented in the pipeline, so we skip testing them for now

#[test]
fn test_cli_compile_only() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_py_file("print(42)", "test.py", &temp_dir);

    let output = Command::new(get_tpy_binary())
        .arg("--compile-only")
        .arg(&file_path)
        .output()
        .expect("Failed to execute tpy");

    assert!(output.status.success());

    // The executable should be created (without extension)
    let exe_path = file_path.with_extension("");
    assert!(exe_path.exists(), "Executable should be created");

    // But the output should be empty (not executed)
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "");
}

#[test]
fn test_cli_compile_only_short_flag() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_py_file("print(42)", "test.py", &temp_dir);

    let output = Command::new(get_tpy_binary())
        .arg("-c")
        .arg(&file_path)
        .output()
        .expect("Failed to execute tpy");

    assert!(output.status.success());

    let exe_path = file_path.with_extension("");
    assert!(
        exe_path.exists(),
        "Executable should be created with -c flag"
    );
}

#[test]
fn test_cli_custom_output_path() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_py_file("print(42)", "test.py", &temp_dir);
    let custom_output = temp_dir.path().join("my_program");

    let output = Command::new(get_tpy_binary())
        .arg("-o")
        .arg(&custom_output)
        .arg("-c")
        .arg(&file_path)
        .output()
        .expect("Failed to execute tpy");

    assert!(output.status.success());
    assert!(
        custom_output.exists(),
        "Custom output executable should be created"
    );
}

#[test]
fn test_cli_custom_output_path_long_flag() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_py_file("print(42)", "test.py", &temp_dir);
    let custom_output = temp_dir.path().join("my_program2");

    let output = Command::new(get_tpy_binary())
        .arg("--output")
        .arg(&custom_output)
        .arg("-c")
        .arg(&file_path)
        .output()
        .expect("Failed to execute tpy");

    assert!(output.status.success());
    assert!(
        custom_output.exists(),
        "Custom output executable should be created"
    );
}

#[test]
fn test_cli_compilation_error() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_py_file("undefined_variable", "test.py", &temp_dir);

    let output = Command::new(get_tpy_binary())
        .arg(&file_path)
        .output()
        .expect("Failed to execute tpy");

    assert!(!output.status.success());
    assert_eq!(output.status.code().unwrap(), 1);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.is_empty(), "Should have error message");
}

#[test]
fn test_cli_successful_compile_and_run() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = create_temp_py_file("print(42)", "test.py", &temp_dir);

    let output = Command::new(get_tpy_binary())
        .arg(&file_path)
        .output()
        .expect("Failed to execute tpy");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "42");
}
