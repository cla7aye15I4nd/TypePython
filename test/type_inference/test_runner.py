#!/usr/bin/env python3
"""
Test runner for type inference tests.

This module tests the type inference system's ability to infer types for
empty containers (lists, dicts, sets) based on usage context.

Test Categories:
1. Basic empty containers (empty_list.py, empty_dict.py, empty_set.py)
2. Nested structures (nested_structures.py)
3. Cross-variable type flow (cross_variable_flow.py)
4. Loop-based inference (loop_inference.py)
5. Conditional branch inference (conditional_inference.py)
6. Complex mixed scenarios (complex_scenarios.py)
"""

# Empty list tests
from empty_list import (
    test_empty_list_append,
    test_empty_list_setitem,
    test_empty_list_in_loop,
    test_empty_list_len,
    test_empty_list_multiple_appends,
    test_nested_list_access,
)

# Nested structure tests
from nested_structures import (
    test_nested_list_basic,
    test_nested_list_multiple_inner,
    test_triple_nested,
)

# Cross-variable flow tests
from cross_variable_flow import (
    test_variable_to_variable_list,
    test_chain_of_assignments,
    test_bidirectional_flow,
)

# Loop inference tests
from loop_inference import (
    test_build_list_in_loop,
    test_nested_loop_matrix,
    test_accumulate_from_iteration,
)

# Conditional inference tests
from conditional_inference import (
    test_both_branches_same_type,
    test_nested_conditionals,
    test_conditional_with_different_operations,
)

# Complex scenario tests
from complex_scenarios import (
    test_data_transformation_pipeline,
    test_histogram_building,
)

# Empty dict tests (will work once dict support is implemented)
# from empty_dict import (
#     test_empty_dict_setitem,
#     test_empty_dict_str_keys,
#     test_empty_dict_update,
#     test_empty_dict_len,
#     test_empty_dict_iteration,
#     test_dict_literal_non_empty,
#     test_dict_literal_iteration,
# )

# Empty set tests (will work once set support is implemented)
# from empty_set import (
#     test_empty_set_add,
#     test_empty_set_contains,
#     test_empty_set_remove,
#     test_empty_set_iteration,
#     test_set_literal_non_empty,
#     test_set_literal_duplicates,
#     test_set_add_duplicates,
#     test_set_membership_loop,
# )


def run_test(name: str, test_func, expected: int) -> int:
    """Helper to run a single test and report result"""
    result = test_func()
    if result == expected:
        print("✓", name, "PASSED")
        return 1  # Success count
    else:
        print("✗", name, "FAILED: expected", expected, "got", result)
        return 0  # Failure


def main() -> int:
    """Run all type inference tests"""
    print("=" * 70)
    print("TYPE INFERENCE COMPREHENSIVE TEST SUITE")
    print("=" * 70)

    tests_passed: int = 0
    tests_failed: int = 0

    # Category 1: Empty List Tests
    print("\n--- Category 1: Basic Empty List Tests ---")
    tests_passed = tests_passed + run_test("test_empty_list_append", test_empty_list_append, 30)
    tests_passed = tests_passed + run_test("test_empty_list_setitem", test_empty_list_setitem, 100)
    tests_passed = tests_passed + run_test("test_empty_list_in_loop", test_empty_list_in_loop, 100)
    tests_passed = tests_passed + run_test("test_empty_list_len", test_empty_list_len, 1)
    tests_passed = tests_passed + run_test("test_empty_list_multiple_appends", test_empty_list_multiple_appends, 60)
    tests_passed = tests_passed + run_test("test_nested_list_access", test_nested_list_access, 6)

    # Category 2: Nested Structures
    print("\n--- Category 2: Nested Structure Tests ---")
    tests_passed = tests_passed + run_test("test_nested_list_basic", test_nested_list_basic, 20)
    tests_passed = tests_passed + run_test("test_nested_list_multiple_inner", test_nested_list_multiple_inner, 10)
    tests_passed = tests_passed + run_test("test_triple_nested", test_triple_nested, 42)

    # Category 3: Cross-Variable Flow
    print("\n--- Category 3: Cross-Variable Type Flow Tests ---")
    tests_passed = tests_passed + run_test("test_variable_to_variable_list", test_variable_to_variable_list, 30)
    tests_passed = tests_passed + run_test("test_chain_of_assignments", test_chain_of_assignments, 100)
    tests_passed = tests_passed + run_test("test_bidirectional_flow", test_bidirectional_flow, 15)

    # Category 4: Loop-Based Inference
    print("\n--- Category 4: Loop-Based Inference Tests ---")
    tests_passed = tests_passed + run_test("test_build_list_in_loop", test_build_list_in_loop, 25)
    tests_passed = tests_passed + run_test("test_nested_loop_matrix", test_nested_loop_matrix, 5)
    tests_passed = tests_passed + run_test("test_accumulate_from_iteration", test_accumulate_from_iteration, 60)

    # Category 5: Conditional Inference
    print("\n--- Category 5: Conditional Branch Inference Tests ---")
    tests_passed = tests_passed + run_test("test_both_branches_same_type", test_both_branches_same_type, 10)
    tests_passed = tests_passed + run_test("test_nested_conditionals", test_nested_conditionals, 200)
    tests_passed = tests_passed + run_test("test_conditional_with_different_operations", test_conditional_with_different_operations, 3)

    # Category 6: Complex Scenarios
    print("\n--- Category 6: Complex Mixed Scenarios ---")
    tests_passed = tests_passed + run_test("test_data_transformation_pipeline", test_data_transformation_pipeline, 6)
    tests_passed = tests_passed + run_test("test_histogram_building", test_histogram_building, 3)

    # Calculate total
    total_tests: int = 21
    tests_failed = total_tests - tests_passed

    # Summary
    print("\n" + "=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print("Tests passed:", tests_passed, "/", total_tests)
    print("Tests failed:", tests_failed, "/", total_tests)

    if tests_failed == 0:
        print("\n🎉 All", total_tests, "tests PASSED!")
        return 0
    else:
        print("\n❌", tests_failed, "tests FAILED")
        return 1


# Run main if this is the entry point
if __name__ == "__main__":
    exit(main())
