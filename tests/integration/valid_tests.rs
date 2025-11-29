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
    test_bool_int_ops,
    "tests/fixtures/valid/basic/bool_int_ops.py"
);
test_case!(
    test_int_float_ops,
    "tests/fixtures/valid/basic/int_float_ops.py"
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
    test_break_statement,
    "tests/fixtures/valid/basic/break_statement.py"
);
test_case!(
    test_continue_statement,
    "tests/fixtures/valid/basic/continue_statement.py"
);
test_case!(
    test_power_operator,
    "tests/fixtures/valid/basic/power_operator.py"
);
test_case!(
    test_floor_division,
    "tests/fixtures/valid/basic/floor_division.py"
);
test_case!(
    test_identity_operators,
    "tests/fixtures/valid/basic/identity_operators.py"
);
test_case!(
    test_string_concat_compare,
    "tests/fixtures/valid/basic/string_concat_compare.py"
);
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
test_case!(
    test_bitwise_operators,
    "tests/fixtures/valid/basic/bitwise_operators.py"
);
test_case!(
    test_modulo_operator,
    "tests/fixtures/valid/basic/modulo_operator.py"
);
test_case!(
    test_unary_operators,
    "tests/fixtures/valid/basic/unary_operators.py"
);
test_case!(
    test_augmented_assignment,
    "tests/fixtures/valid/basic/augmented_assignment.py"
);
test_case!(
    test_division_types,
    "tests/fixtures/valid/basic/division_types.py"
);
test_case!(
    test_float_comparisons,
    "tests/fixtures/valid/basic/float_comparisons.py"
);
test_case!(
    test_boolean_bitwise,
    "tests/fixtures/valid/basic/boolean_bitwise.py"
);
test_case!(
    test_none_operations,
    "tests/fixtures/valid/basic/none_operations.py"
);
test_case!(
    test_type_coercion,
    "tests/fixtures/valid/basic/type_coercion.py"
);
test_case!(
    test_int_true_division,
    "tests/fixtures/valid/basic/int_true_division.py"
);
test_case!(
    test_bool_identity_ops,
    "tests/fixtures/valid/basic/bool_identity_ops.py"
);
test_case!(
    test_bool_logical_ops,
    "tests/fixtures/valid/basic/bool_logical_ops.py"
);
test_case!(
    test_bool_mixed_ops,
    "tests/fixtures/valid/basic/bool_mixed_ops.py"
);
test_case!(
    test_int_unary_not,
    "tests/fixtures/valid/basic/int_unary_not.py"
);
test_case!(test_print_none, "tests/fixtures/valid/basic/print_none.py");
test_case!(test_elif_test, "tests/fixtures/valid/basic/elif_test.py");
test_case!(
    test_numeric_literals,
    "tests/fixtures/valid/basic/numeric_literals.py"
);
test_case!(
    test_pass_statement,
    "tests/fixtures/valid/basic/pass_statement.py"
);
test_case!(
    test_tab_indentation,
    "tests/fixtures/valid/basic/tab_indentation.py"
);
test_case!(
    test_int_float_bool_conversion,
    "tests/fixtures/valid/basic/int_float_bool_conversion.py"
);
test_case!(
    test_escape_sequences,
    "tests/fixtures/valid/basic/escape_sequences.py"
);

// ============================================================================
// Advanced Tests - Arithmetic
// ============================================================================

test_case!(
    test_complex_arithmetic,
    "tests/fixtures/valid/advanced/arithmetic/complex_arithmetic.py"
);
test_case!(
    test_division_operations,
    "tests/fixtures/valid/advanced/arithmetic/division_operations.py"
);
test_case!(
    test_negative_numbers,
    "tests/fixtures/valid/advanced/arithmetic/negative_numbers.py"
);
test_case!(
    test_large_numbers,
    "tests/fixtures/valid/advanced/arithmetic/large_numbers.py"
);
test_case!(
    test_bitwise_simulation,
    "tests/fixtures/valid/advanced/arithmetic/bitwise_simulation.py"
);

// ============================================================================
// Advanced Tests - Control Flow
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
test_case!(
    test_nested_conditionals,
    "tests/fixtures/valid/advanced/control_flow_complex/nested_conditionals.py"
);
test_case!(
    test_nested_loops,
    "tests/fixtures/valid/advanced/control_flow_complex/nested_loops.py"
);
test_case!(
    test_loop_with_conditionals,
    "tests/fixtures/valid/advanced/control_flow_complex/loop_with_conditionals.py"
);
test_case!(
    test_early_termination,
    "tests/fixtures/valid/advanced/control_flow_complex/early_termination.py"
);
test_case!(
    test_switch_simulation,
    "tests/fixtures/valid/advanced/control_flow_complex/switch_simulation.py"
);
test_case!(
    test_guard_clauses,
    "tests/fixtures/valid/advanced/control_flow_complex/guard_clauses.py"
);

// ============================================================================
// Advanced Tests - Functions
// ============================================================================

test_case!(
    test_multiple_returns,
    "tests/fixtures/valid/advanced/functions/multiple_returns.py"
);
test_case!(
    test_function_chains,
    "tests/fixtures/valid/advanced/functions/function_chains.py"
);
test_case!(
    test_parameter_patterns,
    "tests/fixtures/valid/advanced/functions/parameter_patterns.py"
);
test_case!(
    test_void_functions,
    "tests/fixtures/valid/advanced/functions/void_functions.py",
    ignore
);
test_case!(
    test_default_behavior,
    "tests/fixtures/valid/advanced/functions/default_behavior.py"
);
test_case!(
    test_callback_pattern,
    "tests/fixtures/valid/advanced/functions/callback_pattern.py"
);

// ============================================================================
// Advanced Tests - Type System
// ============================================================================

test_case!(
    test_float_precision,
    "tests/fixtures/valid/advanced/types/float_precision.py"
);
test_case!(
    test_boolean_logic,
    "tests/fixtures/valid/advanced/types/boolean_logic.py"
);
test_case!(
    test_type_mixing,
    "tests/fixtures/valid/advanced/types/type_mixing.py"
);
test_case!(
    test_type_comparisons,
    "tests/fixtures/valid/advanced/types/type_comparisons.py"
);
test_case!(
    test_type_inference,
    "tests/fixtures/valid/advanced/types/type_inference_test.py"
);

// ============================================================================
// Advanced Tests - Strings
// ============================================================================

test_case!(
    test_string_operations,
    "tests/fixtures/valid/advanced/strings/string_operations.py"
);
test_case!(
    test_string_conditionals,
    "tests/fixtures/valid/advanced/strings/string_conditionals.py"
);
test_case!(
    test_string_loops,
    "tests/fixtures/valid/advanced/strings/string_loops.py"
);
test_case!(
    test_mixed_output,
    "tests/fixtures/valid/advanced/strings/mixed_output.py"
);

// ============================================================================
// Advanced Tests - Recursion
// ============================================================================

test_case!(
    test_deep_recursion,
    "tests/fixtures/valid/advanced/recursion/deep_recursion.py"
);
test_case!(
    test_mutual_recursion,
    "tests/fixtures/valid/advanced/recursion/mutual_recursion.py"
);
test_case!(
    test_gcd_lcm,
    "tests/fixtures/valid/advanced/recursion/gcd_lcm.py"
);
test_case!(
    test_ackermann,
    "tests/fixtures/valid/advanced/recursion/ackermann.py"
);
test_case!(
    test_tower_of_hanoi,
    "tests/fixtures/valid/advanced/recursion/tower_of_hanoi.py"
);
test_case!(
    test_recursive_fibonacci,
    "tests/fixtures/valid/advanced/recursion/recursive_fibonacci.py"
);

// ============================================================================
// Advanced Tests - Algorithms
// ============================================================================

test_case!(
    test_sorting,
    "tests/fixtures/valid/advanced/algorithms/sorting.py"
);
test_case!(
    test_prime_sieve,
    "tests/fixtures/valid/advanced/algorithms/prime_sieve.py"
);
test_case!(
    test_binary_search,
    "tests/fixtures/valid/advanced/algorithms/binary_search.py"
);
test_case!(
    test_matrix_operations,
    "tests/fixtures/valid/advanced/algorithms/matrix_operations.py"
);
test_case!(
    test_dynamic_programming,
    "tests/fixtures/valid/advanced/algorithms/dynamic_programming.py"
);
test_case!(
    test_number_theory,
    "tests/fixtures/valid/advanced/algorithms/number_theory.py"
);
test_case!(
    test_combinatorics,
    "tests/fixtures/valid/advanced/algorithms/combinatorics.py"
);
test_case!(
    test_sequence_generation,
    "tests/fixtures/valid/advanced/algorithms/sequence_generation.py"
);

// ============================================================================
// Advanced Tests - Edge Cases
// ============================================================================

test_case!(
    test_zero_handling,
    "tests/fixtures/valid/advanced/edge_cases/zero_handling.py"
);
test_case!(
    test_boundary_values,
    "tests/fixtures/valid/advanced/edge_cases/boundary_values.py"
);
test_case!(
    test_nested_expressions,
    "tests/fixtures/valid/advanced/edge_cases/nested_expressions.py"
);
test_case!(
    test_empty_functions,
    "tests/fixtures/valid/advanced/edge_cases/empty_functions.py"
);

// ============================================================================
// Advanced Tests - Scope
// ============================================================================

test_case!(
    test_variable_shadowing,
    "tests/fixtures/valid/advanced/scope/variable_shadowing.py"
);

// ============================================================================
// Advanced Tests - Operators
// ============================================================================

test_case!(
    test_precedence_complex,
    "tests/fixtures/valid/advanced/operators/precedence_complex.py"
);
test_case!(
    test_bitwise_complex,
    "tests/fixtures/valid/advanced/operators/bitwise_complex.py"
);
test_case!(
    test_comparison_chains,
    "tests/fixtures/valid/advanced/operators/comparison_chains.py"
);
test_case!(
    test_arithmetic_edge_cases,
    "tests/fixtures/valid/advanced/operators/arithmetic_edge_cases.py"
);
test_case!(
    test_logical_short_circuit,
    "tests/fixtures/valid/advanced/operators/logical_short_circuit.py"
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
test_case!(
    test_complex_imports,
    "tests/fixtures/valid/modules/complex_imports/main.py"
);
test_case!(
    test_chained_imports,
    "tests/fixtures/valid/modules/chained_imports/main.py"
);
test_case!(
    test_namespace_test,
    "tests/fixtures/valid/modules/namespace_test/main.py"
);

// ============================================================================
// Builtin Functions Tests
// ============================================================================

test_case!(
    test_math_builtins,
    "tests/fixtures/valid/builtins/math_builtins.py"
);
test_case!(
    test_bytes_operations,
    "tests/fixtures/valid/builtins/bytes_operations.py"
);
test_case!(
    test_math_operations,
    "tests/fixtures/valid/builtins/math_operations.py"
);
test_case!(
    test_bytes_methods,
    "tests/fixtures/valid/builtins/bytes_methods.py"
);
test_case!(
    test_bytes_case,
    "tests/fixtures/valid/builtins/bytes_case.py"
);
test_case!(
    test_bytes_predicates,
    "tests/fixtures/valid/builtins/bytes_predicates.py"
);
test_case!(
    test_bytes_padding,
    "tests/fixtures/valid/builtins/bytes_padding.py"
);
test_case!(
    test_math_extra,
    "tests/fixtures/valid/builtins/math_extra.py"
);

// ============================================================================
// Container Types Tests
// ============================================================================

test_case!(test_list_basic, "tests/fixtures/valid/list/list_basic.py");
test_case!(
    test_list_methods,
    "tests/fixtures/valid/list/list_methods.py"
);
test_case!(
    test_list_slicing,
    "tests/fixtures/valid/list/list_slicing.py"
);
test_case!(
    test_list_operations,
    "tests/fixtures/valid/list/list_operations.py"
);
test_case!(test_list_sort, "tests/fixtures/valid/list/list_sort.py");
test_case!(
    test_list_builtin,
    "tests/fixtures/valid/list/list_builtin.py"
);
test_case!(test_list_remove, "tests/fixtures/valid/list/list_remove.py");
test_case!(test_list_extend, "tests/fixtures/valid/list/list_extend.py");
test_case!(test_dict_basic, "tests/fixtures/valid/dict/dict_basic.py");
test_case!(
    test_dict_methods,
    "tests/fixtures/valid/dict/dict_methods.py"
);
test_case!(
    test_dict_operations,
    "tests/fixtures/valid/dict/dict_operations.py"
);
test_case!(
    test_dict_setdefault,
    "tests/fixtures/valid/dict/dict_setdefault.py"
);
test_case!(
    test_dict_builtin,
    "tests/fixtures/valid/dict/dict_builtin.py"
);
test_case!(test_dict_update, "tests/fixtures/valid/dict/dict_update.py");
test_case!(test_dict_pop, "tests/fixtures/valid/dict/dict_pop.py");
test_case!(
    test_dict_len_and_in,
    "tests/fixtures/valid/dict/dict_len_and_in.py"
);
test_case!(test_set_basic, "tests/fixtures/valid/set/set_basic.py");
test_case!(
    test_set_operations,
    "tests/fixtures/valid/set/set_operations.py"
);
test_case!(
    test_set_update_methods,
    "tests/fixtures/valid/set/set_update_methods.py"
);
test_case!(test_set_discard, "tests/fixtures/valid/set/set_discard.py");
test_case!(
    test_set_comparisons,
    "tests/fixtures/valid/set/set_comparisons.py"
);
test_case!(
    test_set_all_methods,
    "tests/fixtures/valid/set/set_all_methods.py"
);

// Pressure tests (10^7 elements)
test_case!(
    test_list_pressure,
    "tests/fixtures/valid/list/list_pressure.py"
);
test_case!(
    test_dict_pressure,
    "tests/fixtures/valid/dict/dict_pressure.py"
);
test_case!(
    test_set_pressure,
    "tests/fixtures/valid/set/set_pressure.py"
);

// New Coverage Tests
// ============================================================================

// Number literal formats
test_case!(
    test_number_literals,
    "tests/fixtures/valid/basic/number_literals.py"
);

// Int/Float mixed operations
test_case!(
    test_int_float_mixed_ops,
    "tests/fixtures/valid/advanced/operators/int_float_mixed_ops.py"
);

// Bitwise operations
test_case!(
    test_int_bitwise_ops,
    "tests/fixtures/valid/advanced/operators/int_bitwise_ops.py"
);

// Mixed type comparisons
test_case!(
    test_int_comparison_mixed,
    "tests/fixtures/valid/advanced/operators/int_comparison_mixed.py"
);

// Unary operations
test_case!(
    test_int_unary_ops,
    "tests/fixtures/valid/advanced/operators/int_unary_ops.py"
);

// Escape sequences (comprehensive)
test_case!(
    test_escape_sequences_comprehensive,
    "tests/fixtures/valid/advanced/strings/escape_sequences.py"
);

// ============================================================================
// Phase 2 Coverage Tests
// ============================================================================

// Int operations comprehensive
test_case!(
    test_int_shift_operations,
    "tests/fixtures/valid/advanced/operators/int_shift_operations.py"
);
test_case!(
    test_int_modulo_negative,
    "tests/fixtures/valid/advanced/operators/int_modulo_negative.py"
);

// Number formats
test_case!(
    test_binary_literals,
    "tests/fixtures/valid/advanced/number_formats/binary_literals.py"
);
test_case!(
    test_hex_literals,
    "tests/fixtures/valid/advanced/number_formats/hex_literals.py"
);

// Functions
test_case!(
    test_functions_no_params,
    "tests/fixtures/valid/advanced/functions/no_params.py"
);
test_case!(
    test_functions_multi_params,
    "tests/fixtures/valid/advanced/functions/multi_params.py"
);

// Expressions
test_case!(
    test_expressions_complex_nested,
    "tests/fixtures/valid/advanced/expressions/complex_nested.py"
);
test_case!(
    test_expressions_all_operators,
    "tests/fixtures/valid/advanced/expressions/all_operators.py"
);

// Control flow
test_case!(
    test_control_flow_nested_ifs,
    "tests/fixtures/valid/advanced/control_flow/nested_ifs.py"
);
