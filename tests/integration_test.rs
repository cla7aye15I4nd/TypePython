use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use similar::{ChangeTag, TextDiff};
use std::path::PathBuf;
use tempfile::TempDir;

fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test")
}

// ============================================================================
// pyrun tests
// ============================================================================

#[test]
fn test_pyrun_no_args() {
    cargo_bin_cmd!("pyrun")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_pyrun_help() {
    cargo_bin_cmd!("pyrun")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Run Python files"));
}

#[test]
fn test_pyrun_nonexistent_file() {
    cargo_bin_cmd!("pyrun")
        .arg("nonexistent.py")
        .assert()
        .failure();
}

#[test]
fn test_pyrun_invalid_target() {
    let simple_py = test_dir().join("exceptions/simple.py");

    cargo_bin_cmd!("pyrun")
        .args([simple_py.to_str().unwrap(), "--target", "invalid_arch"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown target"));
}

#[test]
fn test_pyrun_unknown_flag() {
    let simple_py = test_dir().join("exceptions/simple.py");

    cargo_bin_cmd!("pyrun")
        .args([simple_py.to_str().unwrap(), "--unknown-flag"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}

#[test]
fn test_pyrun_missing_value_for_target() {
    let simple_py = test_dir().join("exceptions/simple.py");

    cargo_bin_cmd!("pyrun")
        .args([simple_py.to_str().unwrap(), "--target"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("a value is required"));
}

#[test]
fn test_pyrun_main_program() {
    let main_py = test_dir().join("main.py");

    if !main_py.exists() {
        panic!("main.py not found at {}", main_py.display());
    }

    // Run with Python interpreter
    let python_output = std::process::Command::new("python3")
        .arg(&main_py)
        .output()
        .expect("Failed to execute python3");
    assert!(
        python_output.status.success(),
        "Python3 failed to execute main.py"
    );
    let python_output = String::from_utf8_lossy(&python_output.stdout).to_string();

    // Run with pyrun
    let compiler_output = cargo_bin_cmd!("pyrun")
        .arg(&main_py)
        .output()
        .expect("Failed to run pyrun");
    assert!(
        compiler_output.status.success(),
        "pyrun failed to execute main.py"
    );
    let compiler_output = String::from_utf8_lossy(&compiler_output.stdout).to_string();

    if python_output != compiler_output {
        // Use unified diff for better readability
        let diff = TextDiff::from_lines(&python_output, &compiler_output);

        eprintln!("\n========== OUTPUT MISMATCH ==========");
        eprintln!(
            "Expected {} lines, got {} lines\n",
            python_output.lines().count(),
            compiler_output.lines().count()
        );

        // Show context around differences (not all 35k lines)
        let mut shown_lines = 0;
        const MAX_CONTEXT: usize = 50; // Show first 50 lines of diff

        for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
            if shown_lines >= MAX_CONTEXT {
                eprintln!(
                    "\n... (diff truncated, showing first {} lines of changes) ...\n",
                    MAX_CONTEXT
                );
                break;
            }

            if idx > 0 {
                eprintln!("...");
            }

            for op in group {
                for change in diff.iter_inline_changes(op) {
                    if shown_lines >= MAX_CONTEXT {
                        break;
                    }

                    let (sign, style) = match change.tag() {
                        ChangeTag::Delete => ("-", "\x1b[31m"), // Red
                        ChangeTag::Insert => ("+", "\x1b[32m"), // Green
                        ChangeTag::Equal => (" ", "\x1b[0m"),   // Normal
                    };

                    eprint!(
                        "{}{}{:4} | {}",
                        style,
                        sign,
                        change.old_index().map(|i| i + 1).unwrap_or(0),
                        change
                    );
                    eprintln!("\x1b[0m");

                    if change.tag() != ChangeTag::Equal {
                        shown_lines += 1;
                    }
                }
            }
        }

        eprintln!("=====================================\n");
        panic!("Output mismatch between Python and compiler");
    }

    cargo_bin_cmd!("pyrun")
        .arg(&main_py)
        .arg("--emit-ast")
        .arg("--emit-llvm")
        .output()
        .expect("Failed to run pyrun");
}

#[test]
fn test_pyrun_riscv64() {
    // Skip if QEMU is not available
    if std::process::Command::new("qemu-riscv64")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("Skipping RISC-V test: qemu-riscv64 not available");
        return;
    }

    let simple_py = test_dir().join("exceptions/simple.py");

    if !simple_py.exists() {
        panic!("simple.py not found at {}", simple_py.display());
    }

    // Run with pyrun targeting RISC-V
    let output = cargo_bin_cmd!("pyrun")
        .args([simple_py.to_str().unwrap(), "--target", "riscv64"])
        .output()
        .expect("Failed to run pyrun with riscv64 target");

    assert!(
        output.status.success(),
        "pyrun --target riscv64 failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1") && stdout.contains("2"));
}

// ============================================================================
// pycc tests
// ============================================================================

#[test]
fn test_pycc_no_args() {
    cargo_bin_cmd!("pycc")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_pycc_nonexistent_file() {
    cargo_bin_cmd!("pycc")
        .arg("nonexistent.py")
        .assert()
        .failure();
}

#[test]
fn test_pycc_missing_output_flag() {
    let simple_py = test_dir().join("exceptions/simple.py");

    cargo_bin_cmd!("pycc")
        .arg(simple_py.to_str().unwrap())
        .assert()
        .failure()
        .stderr(predicate::str::contains("--output"));
}

#[test]
fn test_pycc_invalid_target() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output");
    let simple_py = test_dir().join("exceptions/simple.py");

    cargo_bin_cmd!("pycc")
        .args([
            simple_py.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--target",
            "invalid_arch",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown target"));
}

#[test]
fn test_pycc_unknown_flag() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output");
    let simple_py = test_dir().join("exceptions/simple.py");

    cargo_bin_cmd!("pycc")
        .args([
            simple_py.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--unknown-flag",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument"));
}

#[test]
fn test_pycc_missing_value_for_output() {
    let simple_py = test_dir().join("exceptions/simple.py");

    cargo_bin_cmd!("pycc")
        .args([simple_py.to_str().unwrap(), "-o"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("a value is required"));
}

#[test]
fn test_pycc_missing_value_for_target() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output");
    let simple_py = test_dir().join("exceptions/simple.py");

    cargo_bin_cmd!("pycc")
        .args([
            simple_py.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--target",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("a value is required"));
}

#[test]
fn test_pycc_compile_x86_64() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("simple");

    cargo_bin_cmd!("pycc")
        .args([
            test_dir().join("exceptions/simple.py").to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--target",
            "x86_64",
        ])
        .assert()
        .success();

    assert!(output_path.exists());

    // Run the compiled executable
    let output = std::process::Command::new(&output_path)
        .output()
        .expect("Failed to run compiled executable");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1") && stdout.contains("2"));
}

#[test]
fn test_pycc_compile_riscv64() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("simple_riscv");

    cargo_bin_cmd!("pycc")
        .args([
            test_dir().join("exceptions/simple.py").to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--target",
            "riscv64",
        ])
        .assert()
        .success();

    assert!(output_path.exists());

    // Run with QEMU if available
    if std::process::Command::new("qemu-riscv64")
        .arg("--version")
        .output()
        .is_ok()
    {
        let output = std::process::Command::new("qemu-riscv64")
            .arg(&output_path)
            .output()
            .expect("Failed to run with QEMU");
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("1") && stdout.contains("2"));
    }
}

// ============================================================================
// Static linking tests
// ============================================================================

#[test]
fn test_pycc_static_binary() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("main_static");

    // Compile main.py to a binary
    cargo_bin_cmd!("pycc")
        .args([
            test_dir().join("main.py").to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "--target",
            "x86_64",
        ])
        .assert()
        .success();

    assert!(output_path.exists(), "Compiled binary should exist");

    // Check with 'file' command that it's statically linked
    let file_output = std::process::Command::new("file")
        .arg(&output_path)
        .output()
        .expect("Failed to run file command");
    let file_str = String::from_utf8_lossy(&file_output.stdout);
    assert!(
        file_str.contains("statically linked"),
        "Binary should be statically linked. file output: {}",
        file_str
    );

    // Check with 'ldd' command that there are no dynamic dependencies
    let ldd_output = std::process::Command::new("ldd")
        .arg(&output_path)
        .output()
        .expect("Failed to run ldd command");
    let ldd_str = String::from_utf8_lossy(&ldd_output.stdout);
    let ldd_stderr = String::from_utf8_lossy(&ldd_output.stderr);

    // ldd returns non-zero for static binaries and outputs "not a dynamic executable"
    assert!(
        ldd_str.contains("not a dynamic executable")
            || ldd_stderr.contains("not a dynamic executable"),
        "Binary should have no dynamic dependencies. ldd output: {} stderr: {}",
        ldd_str,
        ldd_stderr
    );

    // Also verify the binary runs correctly
    let run_output = std::process::Command::new(&output_path)
        .output()
        .expect("Failed to run compiled binary");
    assert!(
        run_output.status.success(),
        "Static binary should execute successfully"
    );
}

// ============================================================================
// Invalid file tests
// ============================================================================

#[test]
fn test_invalid_files() {
    let invalid_dir = test_dir().join("invalid");

    if !invalid_dir.exists() {
        panic!("invalid directory not found at {}", invalid_dir.display());
    }

    let entries = std::fs::read_dir(&invalid_dir).expect("Failed to read invalid directory");

    let mut test_count = 0;
    let mut failed_to_error = Vec::new();

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("py") {
            test_count += 1;
            let file_name = path.file_name().unwrap().to_str().unwrap();

            let temp_dir = TempDir::new().unwrap();
            let output_path = temp_dir.path().join("output");

            let result = cargo_bin_cmd!("pycc")
                .args([path.to_str().unwrap(), "-o", output_path.to_str().unwrap()])
                .output()
                .expect("Failed to run pycc");

            if result.status.success() {
                failed_to_error.push(file_name.to_string());
            }
        }
    }

    if test_count == 0 {
        panic!("No .py files found in invalid directory");
    }

    if !failed_to_error.is_empty() {
        panic!(
            "The following files should have failed compilation but didn't: {:?}",
            failed_to_error
        );
    }
}
