# Inheritance tests module
from inheritance.simple_inherit import Dog as SimpleDog
from inheritance.method_inherit import Child as MethodChild
from inheritance.multilevel import C as MultiC
from inheritance.magic_methods_inherit import test_inherit_len, test_inherit_getitem_first
from inheritance.magic_methods_inherit import test_inherit_getitem_last, test_inherit_setitem
from inheritance.magic_methods_inherit import test_inherit_setitem_sum, test_inherit_own_field
from inheritance.magic_methods_inherit import test_inherit_combined
from inheritance.magic_methods_inherit import test_override_getitem, test_override_getitem_all
from inheritance.magic_methods_inherit import test_override_inherit_len, test_override_inherit_setitem
from inheritance.magic_methods_inherit import test_override_setitem_then_get
from inheritance.complex_inherit import test_multilevel_len, test_multilevel_fields
from inheritance.complex_inherit import test_multilevel_override_chain
from inheritance.complex_inherit import test_nested_inheritance, test_nested_chained_access
from inheritance.complex_inherit import test_list_of_derived
from inheritance.complex_inherit import test_derived_uses_parent_method
from inheritance.complex_inherit import test_modify_inherited_field
from inheritance.super_method_call import test_super_method_call, test_super_paramless_method, test_super_preserves_self


def test() -> int:
    # Simple inheritance tests
    print("=== Simple Inheritance Tests ===")
    dog: SimpleDog = SimpleDog(3, 4)
    print(dog.age)    # 3 - inherited field
    print(dog.legs)   # 4 - own field
    print(dog.speak())  # 1 - overridden method

    # Method inheritance tests
    print("=== Method Inheritance Tests ===")
    child: MethodChild = MethodChild(10, 5)
    print(child.get_value())  # 10 - inherited method
    print(child.double())     # 20 - inherited method
    print(child.extra)        # 5 - own field

    # Multi-level inheritance tests
    print("=== Multi-level Inheritance Tests ===")
    mc: MultiC = MultiC(1, 2, 3)
    print(mc.get_a())    # 1 - from grandparent
    print(mc.get_b())    # 2 - from parent
    print(mc.get_c())    # 3 - own method
    print(mc.get_sum())  # 6 - accesses all fields

    # Magic methods inheritance tests
    print("=== Magic Methods Inheritance Tests ===")
    print(test_inherit_len())           # 3
    print(test_inherit_getitem_first()) # 100
    print(test_inherit_getitem_last())  # 300
    print(test_inherit_setitem())       # 999
    print(test_inherit_setitem_sum())   # 150
    print(test_inherit_own_field())     # 42
    print(test_inherit_combined())      # 224

    # Magic methods override tests
    print("=== Magic Methods Override Tests ===")
    print(test_override_getitem())      # 20
    print(test_override_getitem_all())  # 90
    print(test_override_inherit_len())  # 3
    print(test_override_inherit_setitem()) # 200
    print(test_override_setitem_then_get()) # 300

    # Complex inheritance tests
    print("=== Complex Inheritance Tests ===")
    print(test_multilevel_len())           # 42
    print(test_multilevel_fields())        # 100
    print(test_multilevel_override_chain()) # 5
    print(test_nested_inheritance())       # 60
    print(test_nested_chained_access())    # 30
    print(test_list_of_derived())          # 66
    print(test_derived_uses_parent_method()) # 50
    print(test_modify_inherited_field())   # 305

    # Super method call tests
    print("=== Super Method Call Tests ===")
    print(test_super_method_call())        # 45
    print(test_super_paramless_method())   # 21
    print(test_super_preserves_self())     # 20

    return 0
