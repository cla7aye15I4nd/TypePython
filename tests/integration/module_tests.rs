use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn compile_and_run_module_test(test_dir: &str) -> Result<String> {
    let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("valid")
        .join("modules")
        .join(test_dir);

    let main_file = fixtures_path.join("main.py");
    let output_exe = fixtures_path.join("test_output");

    // Clean up any previous build
    let _ = std::fs::remove_file(&output_exe);
    let build_dir = fixtures_path.join("build");
    let _ = std::fs::remove_dir_all(&build_dir);

    // Compile the program
    let tpy_bin = PathBuf::from(env!("CARGO_BIN_EXE_tpy"));

    let compile_output = Command::new(&tpy_bin)
        .arg(&main_file)
        .arg("-c")
        .arg("-o")
        .arg(&output_exe)
        .current_dir(&fixtures_path)
        .output()?;

    if !compile_output.status.success() {
        let stderr = String::from_utf8_lossy(&compile_output.stderr);
        let stdout = String::from_utf8_lossy(&compile_output.stdout);
        anyhow::bail!(
            "Compilation failed for {}: stdout: {}, stderr: {}",
            test_dir,
            stdout,
            stderr
        );
    }

    // Verify object files were created in build directory
    assert!(
        build_dir.exists(),
        "Build directory should exist for {}",
        test_dir
    );

    let object_files: Vec<_> = std::fs::read_dir(&build_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("o"))
        .collect();

    assert!(
        !object_files.is_empty(),
        "Should have created .o files in build directory for {}",
        test_dir
    );

    // Run the compiled executable
    let run_output = Command::new(&output_exe)
        .current_dir(&fixtures_path)
        .output()?;

    if !run_output.status.success() {
        anyhow::bail!(
            "Execution failed for {}: exit code {}",
            test_dir,
            run_output.status.code().unwrap_or(-1)
        );
    }

    let actual_output = String::from_utf8_lossy(&run_output.stdout).to_string();

    // Clean up
    let _ = std::fs::remove_file(&output_exe);

    Ok(actual_output)
}

fn get_or_generate_expected_output(test_dir: &str) -> Result<String> {
    let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("valid")
        .join("modules")
        .join(test_dir);

    let main_file = fixtures_path.join("main.py");
    let output_path = main_file.with_extension("txt");

    if output_path.exists() {
        Ok(fs::read_to_string(&output_path)?)
    } else {
        // Use main_expected.py if it exists (Python-compatible reference implementation)
        let expected_file = fixtures_path.join("main.py");
        let python_file = if expected_file.exists() {
            expected_file
        } else {
            main_file.clone()
        };

        let output = String::from_utf8_lossy(
            &Command::new("python3")
                .arg(&python_file)
                .current_dir(&fixtures_path)
                .output()?
                .stdout,
        )
        .into_owned();
        fs::write(&output_path, &output)?;
        Ok(output)
    }
}

#[test]
fn test_simple_import() -> Result<()> {
    let actual_output = compile_and_run_module_test("simple_import")?;
    let expected_output = get_or_generate_expected_output("simple_import")?;

    assert_eq!(
        actual_output.trim(),
        expected_output.trim(),
        "Output mismatch for simple_import"
    );

    Ok(())
}

#[test]
fn test_multiple_imports() -> Result<()> {
    let actual_output = compile_and_run_module_test("multiple_imports")?;
    let expected_output = get_or_generate_expected_output("multiple_imports")?;

    assert_eq!(
        actual_output.trim(),
        expected_output.trim(),
        "Output mismatch for multiple_imports"
    );

    Ok(())
}
