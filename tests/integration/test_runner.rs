/// Unified test runner for TypePython compilation and execution tests
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;
use tpy::pipeline::{compile, CompileOptions};

/// Compile and run a test, comparing output with expected results
pub fn compile_and_run_test(test_path: &str) -> Result<()> {
    let path = Path::new(test_path);

    // Determine output executable path
    let exe_path = path.with_extension("out");

    // Clean up previous artifacts
    let _ = fs::remove_file(&exe_path);
    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir_all(parent.join("__tpycache__"));
    }

    // Compile the program
    compile(path, &exe_path, &CompileOptions::default())
        .map_err(|e| anyhow::anyhow!("Compilation failed for {}: {}", path.display(), e))?;

    // Run the compiled executable
    let actual_output = run_executable(&exe_path, path)?;

    // Get or generate expected output
    let expected_output = get_or_generate_expected_output(path)?;

    // Compare outputs
    assert_eq!(
        actual_output.trim(),
        expected_output.trim(),
        "Output mismatch for {}",
        path.display()
    );

    // Cleanup
    let _ = fs::remove_file(&exe_path);

    Ok(())
}

/// Run an executable and return its stdout
fn run_executable(exe_path: &Path, source_path: &Path) -> Result<String> {
    let output = Command::new(exe_path)
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute {}: {}", exe_path.display(), e))?;

    if !output.status.success() {
        anyhow::bail!(
            "Execution of {} failed with exit code: {}",
            source_path.display(),
            output.status.code().unwrap_or(-1)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Get expected output from .txt file or generate it using Python 3
fn get_or_generate_expected_output(path: &Path) -> Result<String> {
    let output_path = path.with_extension("txt");

    if output_path.exists() {
        Ok(fs::read_to_string(&output_path)?)
    } else {
        // Generate expected output using Python 3
        let output = Command::new("python3")
            .arg(path)
            .current_dir(path.parent().unwrap_or(Path::new(".")))
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to run python3: {}", e))?;

        if !output.status.success() {
            anyhow::bail!(
                "Python3 execution failed for {}: {}",
                path.display(),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let expected = String::from_utf8_lossy(&output.stdout).into_owned();
        fs::write(&output_path, &expected)?;
        Ok(expected)
    }
}
