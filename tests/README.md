# TypePython Tests

This directory contains the test suite for the TypePython compiler.

## Directory Structure

```
tests/
├── README.md                 # This file
├── integration_tests.rs      # Entry point for integration tests
├── integration/              # Integration test modules
│   ├── mod.rs                # Module declaration
│   └── parser_valid_tests.rs # Parser tests for valid fixtures
└── fixtures/                 # Test fixture files
    └── valid/                # Valid TypePython programs that should parse
        ├── simple.py
        ├── all_types.py
        ├── control_flow.py
        ├── expressions.py
        ├── factorial.py
        ├── fibonacci.py
        └── nested_functions.py
```

## Running Tests

### Run all tests
```bash
cargo test
```

### Run only integration tests
```bash
cargo test --test integration_tests
```

### Run a specific test
```bash
cargo test test_simple
```

### Run tests with output
```bash
cargo test -- --nocapture
```

## Test Structure

### Parser Valid Tests
Located in `integration/parser_valid_tests.rs`, these tests:

1. **`test_all_valid_fixtures()`** - Automatically discovers and tests all `.py` files in `fixtures/valid/`
2. **Individual tests** - Each fixture file has its own dedicated test for granular failure reporting:
   - `test_simple()`
   - `test_all_types()`
   - `test_control_flow()`
   - `test_expressions()`
   - `test_factorial()`
   - `test_fibonacci()`
   - `test_nested_functions()`

## Adding New Tests

### Adding a new valid fixture
1. Create a new `.py` file in `fixtures/valid/`
2. Add a new individual test in `integration/parser_valid_tests.rs`:
   ```rust
   #[test]
   fn test_my_new_feature() {
       parse_and_validate("tests/fixtures/valid/my_new_feature.py");
   }
   ```
3. The file will automatically be picked up by `test_all_valid_fixtures()`

### Adding invalid fixtures (future)
Create a `fixtures/invalid/` directory and corresponding test module to verify parse errors are caught correctly.
