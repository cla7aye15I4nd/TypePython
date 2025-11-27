use inkwell::context::Context;
use std::fs;
use std::path::Path;
use std::process::Command;
use tpy::pipeline::{compile_file, compile_to_executable, CompileOptions};

macro_rules! valid_test {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            test_valid($path);
        }
    };
    ($name:ident, $path:expr, ignore) => {
        #[test]
        #[ignore]
        fn $name() {
            test_valid($path);
        }
    };
}

// Basic tests - these should all pass
valid_test!(
    test_hello_world,
    "tests/fixtures/valid/basic/hello_world.py"
);
valid_test!(test_simple, "tests/fixtures/valid/basic/simple.py");
valid_test!(test_basic_math, "tests/fixtures/valid/basic/basic_math.py");
valid_test!(
    test_bool_operations,
    "tests/fixtures/valid/basic/bool_operations.py"
);
valid_test!(
    test_string_basics,
    "tests/fixtures/valid/basic/string_basics.py"
);
valid_test!(
    test_float_operations,
    "tests/fixtures/valid/basic/float_operations.py"
);
valid_test!(
    test_simple_function,
    "tests/fixtures/valid/basic/simple_function.py"
);
valid_test!(
    test_function_with_return,
    "tests/fixtures/valid/basic/function_with_return.py"
);
valid_test!(test_if_else, "tests/fixtures/valid/basic/if_else.py");
valid_test!(test_while_loop, "tests/fixtures/valid/basic/while_loop.py");
valid_test!(
    test_sum_numbers,
    "tests/fixtures/valid/basic/sum_numbers.py"
);
valid_test!(test_factorial, "tests/fixtures/valid/basic/factorial.py");
valid_test!(test_fibonacci, "tests/fixtures/valid/basic/fibonacci.py");
valid_test!(
    test_expressions,
    "tests/fixtures/valid/basic/expressions.py"
);
valid_test!(test_all_types, "tests/fixtures/valid/basic/all_types.py");

// Advanced tests - these may require more features
valid_test!(
    test_control_flow,
    "tests/fixtures/valid/advanced/control_flow.py",
    ignore
);
valid_test!(
    test_nested_functions,
    "tests/fixtures/valid/advanced/nested_functions.py",
    ignore
);

fn test_valid(path: &str) {
    let path = Path::new(path);
    let context = Context::create();

    let result = compile_file(path, &context, &CompileOptions::default())
        .unwrap_or_else(|e| panic!("Compilation failed for {}: {}", path.display(), e));

    let ll_path = path.with_extension("ll");
    result
        .codegen
        .get_module()
        .print_to_file(&ll_path)
        .expect("Failed to write LLVM IR");

    let exe_path = path.with_extension("out");
    compile_to_executable(&ll_path, &exe_path)
        .unwrap_or_else(|e| panic!("Failed to compile executable: {}", e));

    let actual_output = run_executable(&exe_path, path);
    let expected_output = get_or_generate_expected_output(path);

    assert_eq!(
        expected_output.trim(),
        actual_output.trim(),
        "Output mismatch for {}",
        path.display()
    );
}

fn run_executable(exe_path: &Path, source_path: &Path) -> String {
    let output = Command::new(exe_path).output().expect("Failed to execute");
    if !output.status.success() {
        panic!(
            "Execution of {} failed with status: {}",
            source_path.display(),
            output.status
        );
    }
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn get_or_generate_expected_output(path: &Path) -> String {
    let output_path = path.with_extension("txt");
    if output_path.exists() {
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
    }
}
