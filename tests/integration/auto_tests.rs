//! Auto-generated test module
//!
//! Tests are automatically generated from .py files in tests/fixtures/
//! Edit build.rs to modify test generation logic

// Include generated valid tests
include!(concat!(env!("OUT_DIR"), "/auto_valid_tests.rs"));

// Include generated invalid tests
include!(concat!(env!("OUT_DIR"), "/auto_invalid_tests.rs"));
