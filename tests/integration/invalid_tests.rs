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
