// Test suite for invalid TypePython programs that should fail to compile

use std::fs;
use std::path::Path;
use std::process::Command;
use tpy::pipeline::{compile, CompileOptions};

/// Verify that Python3 also considers this code invalid (runtime error)
fn verify_python3_fails(test_path: &str) {
    let output = Command::new("python3")
        .arg(test_path)
        .output()
        .expect("Failed to run python3");

    if output.status.success() {
        panic!(
            "Expected Python3 to fail for {}, but it succeeded.\n\
             This test case is valid Python3 code and should be removed.\n\
             stdout: {}\nstderr: {}",
            test_path,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Compile a test file and verify it fails (doesn't return success)
fn compile_should_fail(test_path: &str) {
    let path = Path::new(test_path);
    let exe_path = path.with_extension("out");

    // First verify this is also invalid in Python3
    verify_python3_fails(test_path);

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

// Cannot subtract Bytes from Float
invalid_test!(
    test_float_sub_bytes,
    "tests/fixtures/invalid/float_sub_bytes.py"
);

// Cannot multiply Float by Bytes
invalid_test!(
    test_float_mul_bytes,
    "tests/fixtures/invalid/float_mul_bytes.py"
);

// Cannot divide Float by Bytes
invalid_test!(
    test_float_div_bytes,
    "tests/fixtures/invalid/float_div_bytes.py"
);

// Cannot floor divide Float by Bytes
invalid_test!(
    test_float_floordiv_bytes,
    "tests/fixtures/invalid/float_floordiv_bytes.py"
);

// Cannot compute Float modulo Bytes
invalid_test!(
    test_float_mod_bytes,
    "tests/fixtures/invalid/float_mod_bytes.py"
);

// Cannot raise Float to Bytes power
invalid_test!(
    test_float_pow_bytes,
    "tests/fixtures/invalid/float_pow_bytes.py"
);

// Cannot bitwise AND Float and Int
invalid_test!(
    test_float_bitand_int,
    "tests/fixtures/invalid/float_bitand_int.py"
);

// Cannot bitwise OR Float and Int
invalid_test!(
    test_float_bitor_int,
    "tests/fixtures/invalid/float_bitor_int.py"
);

// Cannot bitwise XOR Float and Int
invalid_test!(
    test_float_bitxor_int,
    "tests/fixtures/invalid/float_bitxor_int.py"
);

// Cannot left shift Float
invalid_test!(
    test_float_lshift_int,
    "tests/fixtures/invalid/float_lshift_int.py"
);

// Cannot right shift Float
invalid_test!(
    test_float_rshift_int,
    "tests/fixtures/invalid/float_rshift_int.py"
);

// Cannot use bitwise NOT on Float
invalid_test!(test_float_bitnot, "tests/fixtures/invalid/float_bitnot.py");

// ============================================================================
// Int Extended Type Mismatch Error Tests
// ============================================================================

// Cannot raise Int to Bytes power
invalid_test!(
    test_int_pow_bytes,
    "tests/fixtures/invalid/int_pow_bytes.py"
);

// Cannot bitwise AND Int and Bytes
invalid_test!(
    test_int_bitand_bytes,
    "tests/fixtures/invalid/int_bitand_bytes.py"
);

// Cannot bitwise OR Int and Bytes
invalid_test!(
    test_int_bitor_bytes,
    "tests/fixtures/invalid/int_bitor_bytes.py"
);

// Cannot bitwise XOR Int and Bytes
invalid_test!(
    test_int_bitxor_bytes,
    "tests/fixtures/invalid/int_bitxor_bytes.py"
);

// Cannot left shift Int by Bytes
invalid_test!(
    test_int_lshift_bytes,
    "tests/fixtures/invalid/int_lshift_bytes.py"
);

// Cannot right shift Int by Bytes
invalid_test!(
    test_int_rshift_bytes,
    "tests/fixtures/invalid/int_rshift_bytes.py"
);

// Cannot use 'in' with Int and Int
invalid_test!(test_int_in_int, "tests/fixtures/invalid/int_in_int.py");

// Cannot use 'not in' with Int and Int
invalid_test!(
    test_int_notin_int,
    "tests/fixtures/invalid/int_notin_int.py"
);

// ============================================================================
// Bool Extended Type Mismatch Error Tests
// ============================================================================

// Cannot use 'in' with Bool and Int
invalid_test!(test_bool_in_int, "tests/fixtures/invalid/bool_in_int.py");

// Cannot use 'not in' with Bool and Int
invalid_test!(
    test_bool_notin_int,
    "tests/fixtures/invalid/bool_notin_int.py"
);

// ============================================================================
// Bytes Extended Type Mismatch Error Tests
// ============================================================================

// Cannot use 'in' with Bytes and Int
invalid_test!(test_bytes_in_int, "tests/fixtures/invalid/bytes_in_int.py");

// Cannot use 'not in' with Bytes and Int
invalid_test!(
    test_bytes_notin_int,
    "tests/fixtures/invalid/bytes_notin_int.py"
);

// Cannot subtract Bytes from Bytes
invalid_test!(
    test_bytes_sub_bytes,
    "tests/fixtures/invalid/bytes_sub_bytes.py"
);

// Cannot use unary - on Bytes
invalid_test!(test_bytes_neg, "tests/fixtures/invalid/bytes_neg.py");

// ============================================================================
// None Type Mismatch Error Tests
// ============================================================================

// Cannot add None and Int
invalid_test!(test_none_add_int, "tests/fixtures/invalid/none_add_int.py");

// Cannot subtract Int from None
invalid_test!(test_none_sub_int, "tests/fixtures/invalid/none_sub_int.py");

// Cannot multiply None and Int
invalid_test!(test_none_mul_int, "tests/fixtures/invalid/none_mul_int.py");

// Cannot use unary - on None
invalid_test!(test_none_neg, "tests/fixtures/invalid/none_neg.py");

// ============================================================================
// Preprocessor Error Tests
// ============================================================================

// Inconsistent indentation
invalid_test!(
    test_inconsistent_indent,
    "tests/fixtures/invalid/preprocessor/inconsistent_indent.py"
);

// ============================================================================
// List Type Mismatch Error Tests
// ============================================================================

// Cannot add list and Int
invalid_test!(test_list_add_int, "tests/fixtures/invalid/list_add_int.py");

// Cannot multiply list by Bytes
invalid_test!(
    test_list_mul_bytes,
    "tests/fixtures/invalid/list_mul_bytes.py"
);

// Cannot use unary - on list
invalid_test!(test_list_neg, "tests/fixtures/invalid/list_neg.py");

// Cannot subtract list from list
invalid_test!(
    test_list_sub_list,
    "tests/fixtures/invalid/list_sub_list.py"
);

// ============================================================================
// Set Type Mismatch Error Tests
// ============================================================================

// Cannot subtract Int from set
invalid_test!(test_set_sub_int, "tests/fixtures/invalid/set_sub_int.py");

// Cannot use | between set and Int
invalid_test!(
    test_set_bitor_int,
    "tests/fixtures/invalid/set_bitor_int.py"
);

// Cannot use & between set and Int
invalid_test!(
    test_set_bitand_int,
    "tests/fixtures/invalid/set_bitand_int.py"
);

// Cannot use ^ between set and Int
invalid_test!(
    test_set_bitxor_int,
    "tests/fixtures/invalid/set_bitxor_int.py"
);

// Cannot compare set with Int using <
invalid_test!(test_set_lt_int, "tests/fixtures/invalid/set_lt_int.py");

// Cannot use unary - on set
invalid_test!(test_set_neg, "tests/fixtures/invalid/set_neg.py");

// Cannot add set to set
invalid_test!(test_set_add_set, "tests/fixtures/invalid/set_add_set.py");

// ============================================================================
// Dict Type Mismatch Error Tests
// ============================================================================

// Cannot use | between dict and Int
invalid_test!(
    test_dict_bitor_int,
    "tests/fixtures/invalid/dict_bitor_int.py"
);

// Cannot add dict to dict
invalid_test!(
    test_dict_add_dict,
    "tests/fixtures/invalid/dict_add_dict.py"
);

// Cannot use unary - on dict
invalid_test!(test_dict_neg, "tests/fixtures/invalid/dict_neg.py");

// ============================================================================
// Phase 2: Type Error Tests
// ============================================================================

// Call undefined function
invalid_test!(
    test_undefined_function,
    "tests/fixtures/invalid/type_errors/undefined_func.py"
);
