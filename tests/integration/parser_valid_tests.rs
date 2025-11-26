use pest::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use tpy::{LangParser, Rule};

/// Get all .py files in a directory
fn get_test_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("py") {
                files.push(path);
            }
        }
    }

    files.sort();
    files
}

/// Test that all valid fixtures parse successfully
#[test]
fn test_all_valid_fixtures() {
    let valid_dir = Path::new("tests/fixtures/valid");
    let test_files = get_test_files(valid_dir);

    assert!(
        !test_files.is_empty(),
        "No test files found in {}",
        valid_dir.display()
    );

    let mut failures = Vec::new();

    for file_path in &test_files {
        let source = fs::read_to_string(file_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", file_path.display(), e));

        match LangParser::parse(Rule::program, &source) {
            Ok(_) => {
                println!("✓ {}", file_path.file_name().unwrap().to_string_lossy());
            }
            Err(e) => {
                failures.push(format!("{}: {}", file_path.display(), e));
                eprintln!("✗ {}", file_path.file_name().unwrap().to_string_lossy());
            }
        }
    }

    if !failures.is_empty() {
        panic!(
            "\n{} test(s) failed:\n{}",
            failures.len(),
            failures.join("\n\n")
        );
    }

    println!("\nAll {} test(s) passed!", test_files.len());
}

/// Individual tests for each fixture file for better granularity
#[test]
fn test_simple() {
    parse_and_validate("tests/fixtures/valid/simple.py");
}

#[test]
fn test_all_types() {
    parse_and_validate("tests/fixtures/valid/all_types.py");
}

#[test]
fn test_control_flow() {
    parse_and_validate("tests/fixtures/valid/control_flow.py");
}

#[test]
fn test_expressions() {
    parse_and_validate("tests/fixtures/valid/expressions.py");
}

#[test]
fn test_factorial() {
    parse_and_validate("tests/fixtures/valid/factorial.py");
}

#[test]
fn test_fibonacci() {
    parse_and_validate("tests/fixtures/valid/fibonacci.py");
}

#[test]
fn test_nested_functions() {
    parse_and_validate("tests/fixtures/valid/nested_functions.py");
}

/// Helper function to parse a file and validate it
fn parse_and_validate(path: &str) {
    let source = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    match LangParser::parse(Rule::program, &source) {
        Ok(pairs) => {
            // Verify that we got a program node
            let mut has_program = false;
            for pair in pairs {
                if pair.as_rule() == Rule::program {
                    has_program = true;
                    break;
                }
            }
            assert!(has_program, "Expected a program node in parse tree");
        }
        Err(e) => {
            panic!("Parse error in {}:\n{}", path, e);
        }
    }
}
