// Test suite for invalid TypePython programs that should fail to compile

use std::fs;
use std::path::Path;
use tpy::pipeline::{compile, CompileOptions};

/// Compile a test file and verify it fails (doesn't return success)
fn compile_should_fail(test_path: &str) {
    let path = Path::new(test_path);
    let exe_path = path.with_extension("out");

    // Clean up previous artifacts
    let _ = fs::remove_file(&exe_path);

    // Try to compile - should fail
    let result = compile(path, &exe_path, &CompileOptions::default());

    // Verify we got an error
    if result.is_ok() {
        // Cleanup if it somehow succeeded
        let _ = fs::remove_file(&exe_path);
        panic!(
            "Expected compilation to fail for {}, but it succeeded",
            path.display()
        );
    }
}

macro_rules! invalid_test {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            compile_should_fail($path);
        }
    };
}

// ============================================================================
// Bytes Error Tests
// ============================================================================

// bytes has no method 'nonexistent'
invalid_test!(
    test_bytes_invalid_method,
    "tests/fixtures/invalid/bytes_invalid_method.py"
);

// list has no method 'nonexistent'
invalid_test!(
    test_list_invalid_method,
    "tests/fixtures/invalid/list_invalid_method.py"
);

// dict has no method 'nonexistent'
invalid_test!(
    test_dict_invalid_method,
    "tests/fixtures/invalid/dict_invalid_method.py"
);

// ============================================================================
// Attribute Access Error Tests
// ============================================================================

// Attribute access not supported for type Int
invalid_test!(
    test_int_attribute,
    "tests/fixtures/invalid/int_attribute.py"
);

// ============================================================================
// Subscript Error Tests
// ============================================================================

// Subscript operation not supported for type Int
invalid_test!(
    test_int_subscript,
    "tests/fixtures/invalid/int_subscript.py"
);

// Slice operation not supported for type Int
invalid_test!(test_int_slice, "tests/fixtures/invalid/int_slice.py");

// ============================================================================
// Variable Error Tests
// ============================================================================

// Variable undefined_var not found
invalid_test!(
    test_undefined_variable,
    "tests/fixtures/invalid/undefined_variable.py"
);

// ============================================================================
// Int Type Mismatch Error Tests
// ============================================================================

// Cannot add Int and Bytes
invalid_test!(
    test_int_add_bytes,
    "tests/fixtures/invalid/int_add_bytes.py"
);

// Cannot subtract Bytes from Int
invalid_test!(
    test_int_sub_bytes,
    "tests/fixtures/invalid/int_sub_bytes.py"
);

// Cannot divide Int by Bytes
invalid_test!(
    test_int_div_bytes,
    "tests/fixtures/invalid/int_div_bytes.py"
);

// Cannot floor divide Int by Bytes
invalid_test!(
    test_int_floordiv_bytes,
    "tests/fixtures/invalid/int_floordiv_bytes.py"
);

// Cannot compute Int modulo Bytes
invalid_test!(
    test_int_mod_bytes,
    "tests/fixtures/invalid/int_mod_bytes.py"
);

// ============================================================================
// Bytes Type Mismatch Error Tests
// ============================================================================

// Cannot concatenate Bytes and Int
invalid_test!(
    test_bytes_add_int,
    "tests/fixtures/invalid/bytes_add_int.py"
);

// Cannot multiply Bytes by Bytes
invalid_test!(
    test_bytes_mul_bytes,
    "tests/fixtures/invalid/bytes_mul_bytes.py"
);

// ============================================================================
// Builtin Function Error Tests
// ============================================================================

// abs() takes exactly 1 argument
invalid_test!(test_abs_no_args, "tests/fixtures/invalid/abs_no_args.py");

// abs() not supported for type Bytes
invalid_test!(
    test_abs_wrong_type,
    "tests/fixtures/invalid/abs_wrong_type.py"
);

// round() takes 1 or 2 arguments
invalid_test!(
    test_round_no_args,
    "tests/fixtures/invalid/round_no_args.py"
);

// round() not supported for type Bytes
invalid_test!(
    test_round_wrong_type,
    "tests/fixtures/invalid/round_wrong_type.py"
);

// round() ndigits must be an integer
invalid_test!(
    test_round_ndigits_not_int,
    "tests/fixtures/invalid/round_ndigits_not_int.py"
);

// min() requires at least 2 arguments
invalid_test!(test_min_one_arg, "tests/fixtures/invalid/min_one_arg.py");

// max() requires at least 2 arguments
invalid_test!(test_max_one_arg, "tests/fixtures/invalid/max_one_arg.py");

// pow() takes 2 or 3 arguments
invalid_test!(test_pow_one_arg, "tests/fixtures/invalid/pow_one_arg.py");

// pow() with 3 arguments requires all int arguments
invalid_test!(
    test_pow_mod_not_int,
    "tests/fixtures/invalid/pow_mod_not_int.py"
);

// len() takes exactly 1 argument
invalid_test!(test_len_no_args, "tests/fixtures/invalid/len_no_args.py");

// len() not supported for type Int
invalid_test!(
    test_len_wrong_type,
    "tests/fixtures/invalid/len_wrong_type.py"
);

// ============================================================================
// Bool Type Mismatch Error Tests
// ============================================================================

// Cannot bitwise AND Bool and Bytes
invalid_test!(
    test_bool_bitand_bytes,
    "tests/fixtures/invalid/bool_bitand_bytes.py"
);

// Cannot bitwise OR Bool and Bytes
invalid_test!(
    test_bool_bitor_bytes,
    "tests/fixtures/invalid/bool_bitor_bytes.py"
);

// Cannot bitwise XOR Bool and Bytes
invalid_test!(
    test_bool_bitxor_bytes,
    "tests/fixtures/invalid/bool_bitxor_bytes.py"
);

// ============================================================================
// Float Type Mismatch Error Tests
// ============================================================================

// Cannot add Float and Bytes
invalid_test!(
    test_float_add_bytes,
    "tests/fixtures/invalid/float_add_bytes.py"
);

// ============================================================================
// Preprocessor Error Tests
// ============================================================================

// Inconsistent indentation
invalid_test!(
    test_inconsistent_indent,
    "tests/fixtures/invalid/preprocessor/inconsistent_indent.py"
);

// ============================================================================
// Phase 2: Type Error Tests
// ============================================================================

// Call undefined function
invalid_test!(
    test_undefined_function,
    "tests/fixtures/invalid/type_errors/undefined_func.py"
);
