use inkwell::context::Context;
use pest::Parser;
use std::fs;
use std::process::Command;
use tpy::{codegen::CodeGen, pest_to_ast, preprocessor, LangParser, Rule};

/// Individual tests for each fixture file for better granularity
#[test]
fn test_simple() {
    test_valid("tests/fixtures/valid/simple.py");
}

#[test]
fn test_all_types() {
    test_valid("tests/fixtures/valid/all_types.py");
}

#[test]
#[ignore]
fn test_control_flow() {
    test_valid("tests/fixtures/valid/control_flow.py");
}

#[test]
fn test_expressions() {
    test_valid("tests/fixtures/valid/expressions.py");
}

#[test]
fn test_factorial() {
    test_valid("tests/fixtures/valid/factorial.py");
}

#[test]
fn test_fibonacci() {
    test_valid("tests/fixtures/valid/fibonacci.py");
}

#[test]
#[ignore]
fn test_nested_functions() {
    test_valid("tests/fixtures/valid/nested_functions.py");
}

/// Helper function to parse a file, convert to AST, and generate IR
fn test_valid(path: &str) {
    let source =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    let preprocessed = preprocessor::preprocess(&source).expect("Preprocessing failed");

    // Step 1: Parse with Pest
    let pairs = LangParser::parse(Rule::program, &preprocessed).expect("Parsing failed");

    // Step 2: Convert Pest AST to our AST
    let program = pest_to_ast::build_program(pairs);

    // Step 3: Generate LLVM IR
    let context = Context::create();
    let module_name = std::path::Path::new(path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let mut codegen = CodeGen::new(&context, module_name);

    if let Err(e) = codegen.generate(&program) {
        panic!("IR generation error in {}:\n{}", path, e);
    }

    // Step 4: Verify the generated module
    if let Err(e) = codegen.get_module().verify() {
        panic!("IR verification failed in {}:\n{}", path, e);
    }

    // Step 5: Write bitcode to file
    let ll_path = std::path::Path::new(path).with_extension("ll");
    codegen
        .get_module()
        .print_to_file(&ll_path)
        .expect("Failed to write LLVM IR");

    let exe_path = std::path::Path::new(path).with_extension("out");
    match std::env::var("LLVM_SYS_211_PREFIX") {
        Err(_) => {
            eprintln!("Error: LLVM_SYS_211_PREFIX environment variable is not set.");
            std::process::exit(1);
        }
        Ok(prefix) => {
            let llc = format!("{}/bin/clang", prefix);

            Command::new(&llc)
                .arg(&ll_path)
                .arg("-o")
                .arg(&exe_path)
                .arg("-Wno-override-module")
                .status()
                .expect("Failed to invoke clang");
        }
    }

    // Step 6: Run the executable
    let output_path = std::path::Path::new(path).with_extension("txt");
    let output = Command::new(&exe_path).output().expect("Failed to execute");
    if !output.status.success() {
        panic!(
            "Execution of {} failed with status: {}",
            path, output.status
        );
    }

    let expected_output = if output_path.exists() {
        fs::read_to_string(&output_path).expect("Failed to read expected output file")
    } else {
        let output = String::from_utf8_lossy(
            &Command::new("python3")
                .arg(path)
                .output()
                .expect("Failed to execute reference Python interpreter")
                .stdout,
        )
        .into_owned();
        fs::write(&output_path, &output).expect("Failed to write expected output file");
        output
    };

    let actual_output = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        expected_output.trim(),
        actual_output.trim(),
        "Output mismatch for {}",
        path
    );
}
