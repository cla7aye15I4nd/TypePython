use inkwell::context::Context;
use pest::Parser;
use std::fs;
use tpy::{codegen::CodeGen, pest_to_ast, LangParser, Rule};

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
#[ignore]
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
#[ignore]
fn test_nested_functions() {
    parse_and_validate("tests/fixtures/valid/nested_functions.py");
}

/// Helper function to parse a file, convert to AST, and generate IR
fn parse_and_validate(path: &str) {
    let source =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e));

    // Step 1: Parse with Pest
    let pairs = match LangParser::parse(Rule::program, &source) {
        Ok(pairs) => pairs,
        Err(e) => {
            panic!("Parse error in {}:\n{}", path, e);
        }
    };

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

    println!("✓ Successfully parsed, generated AST and IR for {}", path);
}
