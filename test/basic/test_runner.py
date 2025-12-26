# Basic tests module
from algorithm.factorial import factorial
from algorithm.fibonacci import fibonacci
from basic.control_flow.while_sum import sum_to_n
from basic.hello import hello
from basic.collections.list_ops import get_first, get_third
from basic.classes.class_simple import Point
from basic.primitives.operators import test_add, test_sub, test_mult, test_mod
from basic.primitives.operators import test_eq, test_neq, test_lt, test_lte, test_gt, test_gte
from basic.primitives.aug_assign import test_add_assign, test_sub_assign, test_mult_assign, test_mod_assign, test_compound_aug
from basic.collections.list_advanced import list_len, list_sum, create_and_access, nested_access
from basic.control_flow.edge_cases import expr_stmt, nested_if, count_to_limit, in_range, chained_compare
from basic.classes.complex_types import test_class_in_class, test_chained_assign, test_nested_method
from basic.classes.complex_types import test_multiple_chained, test_list_set, test_list_of_class
from basic.classes.complex_types import test_list_element_modify, test_deep_nesting
from basic.classes.string_repr import test_str_only, test_repr_only, test_both_str_and_repr
from basic.classes.string_repr import test_str_with_internal_print, test_repr_with_internal_print
from basic.classes.string_repr import test_nested_with_str, test_multiple_instances, test_str_in_expression
from datastructure.hashmap import test_hashmap_basic, test_hashmap_update, test_hashmap_contains
from datastructure.hashset import test_hashset_basic, test_hashset_contains
from datastructure.bst import test_bst_insert, test_bst_contains
from datastructure.heap import test_heap_basic, test_heap_pop, test_heap_sort
from stresstest.deep_nesting import test_three_level_nesting, test_three_level_assign, test_three_level_method
from stresstest.deep_nesting import test_class_with_list_field, test_list_in_class_access, test_list_in_class_modify
from stresstest.deep_nesting import test_list_of_nested_class, test_list_of_nested_modify
from stresstest.deep_nesting import test_four_level_access, test_four_level_modify
from stresstest.deep_nesting import test_list_in_list_in_class, test_list_in_list_modify
from stresstest.deep_nesting import test_complex_expression, test_nested_loop_access, test_modify_in_loop
from basic.primitives.bool import test_bool_true, test_bool_false, test_bool_var_true, test_bool_var_false
from basic.primitives.bool import test_compare_to_bool, test_and_tt, test_and_tf, test_and_ff
from basic.primitives.bool import test_or_tt, test_or_tf, test_or_ff, test_not_true, test_not_false
from basic.primitives.bool import test_and_vars, test_or_vars, test_not_var, test_chained_compare_bool
from basic.primitives.bool_params import test_bool_param, test_bool_identity, test_bool_and_func
from basic.primitives.binops import test_bitand, test_bitor, test_bitxor, test_lshift, test_rshift, test_pow
from basic.primitives.coverage_extras import test_unary_minus, test_division, test_print_newline, test_print_bool, test_both_branches_return, test_bool_to_int
from basic.primitives.strings import get_greeting, get_empty, concat_strings
from stresstest.stress_test import run_all_stress_tests
from basic.primitives.bytes_test import test_bytes_literal, test_bytes_index, test_bytes_len, test_bytes_empty_len
from basic.primitives.bytearray_test import test_bytearray_create, test_bytearray_len, test_bytearray_empty
from basic.primitives.bytearray_test import test_bytearray_append, test_bytearray_append_value, test_bytearray_set
from basic.primitives.magic_methods_test import test_list_int_len, test_list_str_len
from basic.primitives.magic_methods_test import test_str_len, test_str_empty_len
from basic.primitives.magic_methods_test import test_list_getitem_first, test_list_getitem_middle, test_list_getitem_last
from basic.primitives.magic_methods_test import test_bytes_getitem_first, test_bytes_getitem_last
from basic.primitives.magic_methods_test import test_bytearray_getitem_first, test_bytearray_getitem_last
from basic.primitives.magic_methods_test import test_list_setitem_first, test_list_setitem_middle, test_list_setitem_last
from basic.primitives.magic_methods_test import test_bytearray_setitem_first, test_bytearray_setitem_middle, test_bytearray_setitem_last
from basic.primitives.magic_methods_test import test_list_len_after_setitem, test_list_sum_after_setitem, test_bytearray_sum_after_setitem
from basic.primitives.magic_methods_test import test_custom_len, test_custom_getitem_first, test_custom_getitem_last
from basic.primitives.magic_methods_test import test_custom_setitem, test_custom_setitem_and_sum
from basic.primitives.magic_methods_test import test_custom_str_len, test_custom_str_getitem, test_custom_str_setitem
from basic.iterators.iterator_tests import test_for_range_one_arg, test_for_range_two_args, test_for_range_three_args
from basic.iterators.iterator_tests import test_for_list_basic, test_for_list_modify
from basic.iterators.iterator_tests import test_for_nested_range, test_for_nested_list, test_for_nested_mixed, test_for_triple_nested
from basic.iterators.iter_next_tests import test_iter_next_basic, test_iter_next_all_elements, test_iter_range
from basic.primitives.bytearray_empty import test_bytearray_empty_constructor, test_bytearray_empty_then_append
from basic.primitives.float_test import test_float_literal, test_float_add, test_float_sub, test_float_mult
from basic.primitives.float_test import test_float_div, test_int_div_returns_float, test_mixed_add, test_float_neg
from basic.primitives.float_test import test_float_gt, test_float_lt, test_float_eq
from basic.primitives.float_test import test_float_truthy, test_float_falsy
from basic.primitives.float_test import test_float_floordiv, test_float_mod, test_float_pow, test_print_float

def test() -> int:
    # Basic function tests
    print(factorial(5))      # 120
    print(fibonacci(10))     # 55
    print(sum_to_n(10))      # 55
    print(hello())           # 42

    # List operations
    numbers: list[int] = [1, 2, 3, 4, 5]
    print(get_first(numbers))  # 1
    print(get_third(numbers))  # 3

    # Class test
    p: Point = Point(10, 20)
    print(p.get_x())    # 10
    print(p.get_sum())  # 30

    # Binary operators
    print(test_add(10, 5))   # 15
    print(test_sub(10, 3))   # 7
    print(test_mult(4, 5))   # 20
    print(test_mod(17, 5))   # 2

    # Comparison operators
    print(test_eq(5, 5))     # 1
    print(test_eq(5, 3))     # 0
    print(test_neq(5, 3))    # 1
    print(test_neq(5, 5))    # 0
    print(test_lt(3, 5))     # 1
    print(test_lt(5, 3))     # 0
    print(test_lte(5, 5))    # 1
    print(test_lte(3, 5))    # 1
    print(test_gt(5, 3))     # 1
    print(test_gt(3, 5))     # 0
    print(test_gte(5, 5))    # 1
    print(test_gte(5, 3))    # 1

    # Augmented assignments
    print(test_add_assign())   # 15
    print(test_sub_assign())   # 7
    print(test_mult_assign())  # 20
    print(test_mod_assign())   # 1
    print(test_compound_aug()) # 25

    # Advanced list operations
    nums: list[int] = [1, 2, 3, 4, 5]
    print(list_len(nums))        # 5
    print(list_sum(nums))        # 15
    print(create_and_access())   # 60
    print(nested_access(nums, 2)) # 3

    # Edge case tests
    print(expr_stmt())           # 5
    print(nested_if(25))         # 3
    print(nested_if(15))         # 2
    print(nested_if(7))          # 1
    print(nested_if(3))          # 0
    print(count_to_limit(5))     # 5
    print(in_range(5, 1, 10))    # 1
    print(in_range(15, 1, 10))   # 0
    print(chained_compare(1, 5, 10))  # 1 (1 < 5 < 10)
    print(chained_compare(5, 5, 10))  # 0 (5 < 5 is false)
    print(chained_compare(1, 10, 5))  # 0 (10 < 5 is false)

    # Complex types tests
    print(test_class_in_class())     # 5 (r.corner.x where corner is Point(5, 10))
    print(test_chained_assign())     # 42 (r.corner.x = 42)
    print(test_nested_method())      # 10 (r.corner.get_sum() where corner is Point(3, 7))
    print(test_multiple_chained())   # 30 (r.corner.x=10, r.corner.y=20, sum)
    print(test_list_set())           # 99 (points[0] = Point(99, 99))
    print(test_list_of_class())      # 50 (points[0].x + points[1].y = 10 + 40)
    print(test_list_element_modify()) # 100 (points[0].x = 100)
    print(test_deep_nesting())       # 4 (original=1, modified=1+2=3, sum=4)

    # __str__ and __repr__ tests
    print(test_str_only())           # 1 (prints "Point(x, y)")
    print(test_repr_only())          # 1 (prints "Vector[x, y]")
    print(test_both_str_and_repr())  # 1 (prints "Rectangle(w x h)")
    print(test_str_with_internal_print())    # 1 (prints messages inside and outside)
    print(test_repr_with_internal_print())   # 1 (prints messages inside and outside)
    print(test_nested_with_str())    # 1 (prints nested class)
    print(test_multiple_instances()) # 2 (prints two Point instances)
    print(test_str_in_expression())  # 1 (prints Point and field value)

    # Data structure tests - HashMap
    print(test_hashmap_basic())      # 200
    print(test_hashmap_update())     # 999
    print(test_hashmap_contains())   # 2

    # Data structure tests - HashSet
    print(test_hashset_basic())      # 3
    print(test_hashset_contains())   # 2

    # Data structure tests - Binary Search Tree
    print(test_bst_insert())         # 5
    print(test_bst_contains())       # 2

    # Data structure tests - MinHeap
    print(test_heap_basic())         # 1
    print(test_heap_pop())           # 4
    print(test_heap_sort())          # 6

    # Deep nesting tests - 3-level class nesting (Container->Box->Item)
    print(test_three_level_nesting())    # 42
    print(test_three_level_assign())     # 99
    print(test_three_level_method())     # 77

    # Deep nesting tests - class with list field
    print(test_class_with_list_field())  # 60
    print(test_list_in_class_access())   # 15
    print(test_list_in_class_modify())   # 200

    # Deep nesting tests - list of nested classes (list[Box] where Box->Item)
    print(test_list_of_nested_class())   # 22
    print(test_list_of_nested_modify())  # 999

    # Deep nesting tests - 4-level access (Storage->containers[i]->box->item->value)
    print(test_four_level_access())      # 222
    print(test_four_level_modify())      # 888

    # Deep nesting tests - list in list in class (Company->inventories[i]->items[j]->value)
    print(test_list_in_list_in_class())  # 40
    print(test_list_in_list_modify())    # 777

    # Deep nesting tests - complex expressions and loops
    print(test_complex_expression())     # 60
    print(test_nested_loop_access())     # 60
    print(test_modify_in_loop())         # 60

    # Bool type tests
    print(test_bool_true())              # 1
    print(test_bool_false())             # 1
    print(test_bool_var_true())          # 1
    print(test_bool_var_false())         # 1
    print(test_compare_to_bool())        # 1
    print(test_and_tt())                 # 1
    print(test_and_tf())                 # 1
    print(test_and_ff())                 # 1
    print(test_or_tt())                  # 1
    print(test_or_tf())                  # 1
    print(test_or_ff())                  # 1
    print(test_not_true())               # 1
    print(test_not_false())              # 1
    print(test_and_vars())               # 1
    print(test_or_vars())                # 1
    print(test_not_var())                # 1
    print(test_chained_compare_bool())   # 1

    # Bool parameter tests
    print(test_bool_param())             # 1
    print(test_bool_identity())          # 1
    print(test_bool_and_func())          # 1

    # Bitwise and power operators
    print(test_bitand(12, 10))           # 8 (0b1100 & 0b1010 = 0b1000)
    print(test_bitor(12, 10))            # 14 (0b1100 | 0b1010 = 0b1110)
    print(test_bitxor(12, 10))           # 6 (0b1100 ^ 0b1010 = 0b0110)
    print(test_lshift(1, 4))             # 16 (1 << 4)
    print(test_rshift(16, 2))            # 4 (16 >> 2)
    print(test_pow(2, 10))               # 1024 (2 ** 10)
    print(test_pow(3, 4))                # 81 (3 ** 4)

    # Coverage extras - unary minus, division, print edge cases
    print(test_unary_minus())            # -5
    print(test_division())               # 5
    print(test_print_newline())          # prints newline, returns 1
    print(test_print_bool())             # prints True, returns 1
    print(test_both_branches_return())   # 1
    print(test_bool_to_int())            # 1

    # String tests
    print(get_greeting())                # Hello, World!
    print(get_empty())                   # (empty line)
    concat_strings("foo", "bar")         # prints foo and bar

    # Stress tests - large scale random testing
    print(run_all_stress_tests())        # 26 (number of stress tests passed)

    # Bytes type tests
    print(test_bytes_literal())          # 104 (ASCII 'h')
    print(test_bytes_index())            # 98 (ASCII 'b')
    print(test_bytes_len())              # 5
    print(test_bytes_empty_len())        # 0

    # ByteArray type tests
    print(test_bytearray_create())       # 104 (ASCII 'h')
    print(test_bytearray_len())          # 5
    print(test_bytearray_empty())        # 0
    print(test_bytearray_append())       # 3
    print(test_bytearray_append_value()) # 99
    print(test_bytearray_set())          # 65 (ASCII 'A')

    # Magic methods tests - __len__
    print(test_list_int_len())           # 5
    print(test_list_str_len())           # 3
    print(test_str_len())                # 5
    print(test_str_empty_len())          # 0

    # Magic methods tests - __getitem__
    print(test_list_getitem_first())     # 100
    print(test_list_getitem_middle())    # 30
    print(test_list_getitem_last())      # 25
    print(test_bytes_getitem_first())    # 104 (ASCII 'h')
    print(test_bytes_getitem_last())     # 111 (ASCII 'o')
    print(test_bytearray_getitem_first()) # 119 (ASCII 'w')
    print(test_bytearray_getitem_last()) # 100 (ASCII 'd')

    # Magic methods tests - __setitem__
    print(test_list_setitem_first())     # 100
    print(test_list_setitem_middle())    # 999
    print(test_list_setitem_last())      # 42
    print(test_bytearray_setitem_first()) # 65
    print(test_bytearray_setitem_middle()) # 88
    print(test_bytearray_setitem_last()) # 90

    # Magic methods tests - combined
    print(test_list_len_after_setitem()) # 5
    print(test_list_sum_after_setitem()) # 90
    print(test_bytearray_sum_after_setitem()) # 30

    # Magic methods tests - custom class
    print(test_custom_len())             # 3
    print(test_custom_getitem_first())   # 100
    print(test_custom_getitem_last())    # 15
    print(test_custom_setitem())         # 999
    print(test_custom_setitem_and_sum()) # 420
    print(test_custom_str_len())         # 2
    print(test_custom_str_getitem())     # 5
    print(test_custom_str_setitem())     # 5

    # Iterator tests - for loops with range
    print(test_for_range_one_arg())      # 1
    print(test_for_range_two_args())     # 1
    print(test_for_range_three_args())   # 1

    # Iterator tests - for loops with list
    print(test_for_list_basic())         # 1
    print(test_for_list_modify())        # 1

    # Iterator tests - nested for loops
    print(test_for_nested_range())       # 1
    print(test_for_nested_list())        # 1
    print(test_for_nested_mixed())       # 1
    print(test_for_triple_nested())      # 1

    # iter() and next() builtin tests
    print(test_iter_next_basic())        # 30
    print(test_iter_next_all_elements()) # 15
    print(test_iter_range())             # 1

    # Empty bytearray constructor tests
    print(test_bytearray_empty_constructor()) # 0
    print(test_bytearray_empty_then_append()) # 2

    # Float type tests
    print(test_float_literal())              # 1
    print(test_float_add())                  # 1
    print(test_float_sub())                  # 1
    print(test_float_mult())                 # 1
    print(test_float_div())                  # 1
    print(test_int_div_returns_float())      # 1
    print(test_mixed_add())                  # 1
    print(test_float_neg())                  # 1
    print(test_float_gt())                   # 1
    print(test_float_lt())                   # 1
    print(test_float_eq())                   # 1
    print(test_float_truthy())               # 1
    print(test_float_falsy())                # 1
    print(test_float_floordiv())             # 1
    print(test_float_mod())                  # 1
    print(test_float_pow())                  # 1
    print(test_print_float())                # prints 3.14, returns 1

    return 0
