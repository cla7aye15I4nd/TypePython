# Stress tests module
from stresstest.deep_nesting import test_three_level_nesting, test_three_level_assign, test_three_level_method
from stresstest.deep_nesting import test_class_with_list_field, test_list_in_class_access, test_list_in_class_modify
from stresstest.deep_nesting import test_list_of_nested_class, test_list_of_nested_modify
from stresstest.deep_nesting import test_four_level_access, test_four_level_modify
from stresstest.deep_nesting import test_list_in_list_in_class, test_list_in_list_modify
from stresstest.deep_nesting import test_complex_expression, test_nested_loop_access, test_modify_in_loop
from stresstest.stress_test import run_all_stress_tests

def test() -> int:
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

    # Stress tests - large scale random testing
    print(run_all_stress_tests())        # 26 (number of stress tests passed)

    return 0
