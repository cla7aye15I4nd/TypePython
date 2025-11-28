// Unified test suite for all valid TypePython programs
// Combines basic, advanced, and module tests

use crate::integration::test_runner::compile_and_run_test;

macro_rules! test_case {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            compile_and_run_test($path).unwrap();
        }
    };
    ($name:ident, $path:expr, ignore) => {
        #[test]
        #[ignore]
        fn $name() {
            compile_and_run_test($path).unwrap();
        }
    };
}

// ============================================================================
// Basic Tests
// ============================================================================

test_case!(
    test_hello_world,
    "tests/fixtures/valid/basic/hello_world.py"
);
test_case!(test_simple, "tests/fixtures/valid/basic/simple.py");
test_case!(test_basic_math, "tests/fixtures/valid/basic/basic_math.py");
test_case!(
    test_bool_operations,
    "tests/fixtures/valid/basic/bool_operations.py"
);
test_case!(
    test_string_basics,
    "tests/fixtures/valid/basic/string_basics.py"
);
test_case!(
    test_float_operations,
    "tests/fixtures/valid/basic/float_operations.py"
);
test_case!(
    test_simple_function,
    "tests/fixtures/valid/basic/simple_function.py"
);
test_case!(
    test_function_with_return,
    "tests/fixtures/valid/basic/function_with_return.py"
);
test_case!(test_if_else, "tests/fixtures/valid/basic/if_else.py");
test_case!(test_while_loop, "tests/fixtures/valid/basic/while_loop.py");
test_case!(
    test_sum_numbers,
    "tests/fixtures/valid/basic/sum_numbers.py"
);
test_case!(test_factorial, "tests/fixtures/valid/basic/factorial.py");
test_case!(test_fibonacci, "tests/fixtures/valid/basic/fibonacci.py");
test_case!(
    test_expressions,
    "tests/fixtures/valid/basic/expressions.py"
);
test_case!(test_all_types, "tests/fixtures/valid/basic/all_types.py");

// ============================================================================
// Advanced Tests
// ============================================================================

test_case!(
    test_control_flow,
    "tests/fixtures/valid/advanced/control_flow.py",
    ignore
);
test_case!(
    test_nested_functions,
    "tests/fixtures/valid/advanced/nested_functions.py",
    ignore
);

// ============================================================================
// Module Tests
// ============================================================================

test_case!(
    test_simple_import,
    "tests/fixtures/valid/modules/simple_import/main.py"
);
test_case!(
    test_multiple_imports,
    "tests/fixtures/valid/modules/multiple_imports/main.py"
);
test_case!(
    test_deep_import,
    "tests/fixtures/valid/modules/deep_import/main.py"
);
