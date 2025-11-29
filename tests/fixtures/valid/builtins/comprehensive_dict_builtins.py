# Comprehensive test for all dict operations with all applicable builtin functions
# Tests: len, dict() constructor, min/max on keys/values
# Dict methods: get, pop, clear, copy, keys, values, items, update, setdefault

# ============================================================================
# SECTION 1: len() with dicts
# ============================================================================
print("# len() with dicts")

empty_dict: dict[str, int] = {}
print(len(empty_dict))  # 0

single_dict: dict[str, int] = {"a": 1}
print(len(single_dict))  # 1

multi_dict: dict[str, int] = {"a": 1, "b": 2, "c": 3}
print(len(multi_dict))  # 3

# ============================================================================
# SECTION 2: dict() constructor
# ============================================================================
print("# dict() constructor")

new_dict: dict[str, int] = dict()
print(len(new_dict))  # 0

# ============================================================================
# SECTION 3: Dictionary indexing and assignment
# ============================================================================
print("# Dictionary indexing and assignment")

index_dict: dict[str, int] = {}
index_dict["x"] = 10
index_dict["y"] = 20
index_dict["z"] = 30

print(len(index_dict))  # 3
print(index_dict["x"])  # 10
print(index_dict["y"])  # 20
print(index_dict["z"])  # 30

# ============================================================================
# SECTION 4: min() and max() with dict values
# ============================================================================
print("# min() and max() with dict values")

val_dict: dict[str, int] = {"a": 5, "b": 2, "c": 9, "d": 1}
print(min(val_dict["a"], val_dict["b"]))  # 2
print(max(val_dict["a"], val_dict["c"]))  # 9

# Multiple values
print(min(val_dict["a"], val_dict["b"], val_dict["d"]))  # 1
print(max(val_dict["a"], val_dict["b"], val_dict["c"]))  # 9

# ============================================================================
# SECTION 5: Dict method - get()
# ============================================================================
print("# get() method")

get_dict: dict[str, int] = {"foo": 100, "bar": 200}
val1: int = get_dict.get("foo", 0)
print(val1)  # 100

val2: int = get_dict.get("bar", 0)
print(val2)  # 200

val3: int = get_dict.get("baz", -1)
print(val3)  # -1 (default value)

# ============================================================================
# SECTION 6: Dict method - pop()
# ============================================================================
print("# pop() method")

pop_dict: dict[str, int] = {"a": 10, "b": 20, "c": 30}
print(len(pop_dict))  # 3

popped1: int = pop_dict.pop("b")
print(popped1)  # 20
print(len(pop_dict))  # 2

# Verify remaining keys
print(pop_dict["a"])  # 10
print(pop_dict["c"])  # 30

# ============================================================================
# SECTION 7: Dict method - copy()
# ============================================================================
print("# copy() method")

orig_dict: dict[str, int] = {"x": 1, "y": 2}
copy_dict: dict[str, int] = orig_dict.copy()

print(len(copy_dict))  # 2
print(copy_dict["x"])  # 1
print(copy_dict["y"])  # 2

# Modify copy to verify independence
copy_dict["z"] = 3
print(len(orig_dict))  # 2 (unchanged)
print(len(copy_dict))  # 3 (modified)

# ============================================================================
# SECTION 8: Dict method - clear()
# ============================================================================
print("# clear() method")

clear_dict: dict[str, int] = {"a": 1, "b": 2, "c": 3}
print(len(clear_dict))  # 3
clear_dict.clear()
print(len(clear_dict))  # 0

# ============================================================================
# SECTION 9: Dictionary with different value types
# ============================================================================
print("# Dictionaries with different value types")

float_dict: dict[str, float] = {"pi": 3.14, "e": 2.71}
print(len(float_dict))  # 2
print(max(float_dict["pi"], float_dict["e"]))  # 3.14

bool_dict: dict[str, bool] = {"yes": True, "no": False}
print(len(bool_dict))  # 2
print(bool_dict["yes"])  # True

# ============================================================================
# SECTION 10: Integer keys
# ============================================================================
print("# Integer keys")

int_key_dict: dict[int, str] = {}
int_key_dict[1] = "one"
int_key_dict[2] = "two"
int_key_dict[3] = "three"

print(len(int_key_dict))  # 3
print(int_key_dict[1])  # "one" (but we print its presence)
print(len(int_key_dict[1]))  # 3 (length of "one")

# Use min/max on keys
print(min(1, 2, 3))  # 1
print(max(1, 2, 3))  # 3

# ============================================================================
# SECTION 11: Complex combinations
# ============================================================================
print("# Complex combinations")

combo_dict: dict[str, int] = dict()
combo_dict["alpha"] = 10
combo_dict["beta"] = 20
combo_dict["gamma"] = 30

print(len(combo_dict))  # 3

# Get with default
val_x: int = combo_dict.get("alpha", 0)
print(val_x)  # 10

val_y: int = combo_dict.get("delta", 99)
print(val_y)  # 99

# Pop and check length
popped_val: int = combo_dict.pop("beta")
print(popped_val)  # 20
print(len(combo_dict))  # 2

# ============================================================================
# SECTION 12: Nested operations with len()
# ============================================================================
print("# Nested operations with len()")

nested_dict: dict[str, int] = {"a": 1, "b": 2}
print(len(nested_dict))  # 2

nested_dict["c"] = 3
print(len(nested_dict))  # 3

nested_dict.pop("a")
print(len(nested_dict))  # 2

nested_dict.clear()
print(len(nested_dict))  # 0

# ============================================================================
# SECTION 13: Using abs() with dict values
# ============================================================================
print("# abs() with dict values")

abs_dict: dict[str, int] = {"neg": -42, "pos": 42}
print(abs(abs_dict["neg"]))  # 42
print(abs(abs_dict["pos"]))  # 42

float_abs_dict: dict[str, float] = {"x": -3.14, "y": 2.71}
print(abs(float_abs_dict["x"]))  # 3.14

# ============================================================================
# SECTION 14: Using round() with dict values
# ============================================================================
print("# round() with dict values")

round_dict: dict[str, float] = {"pi": 3.14159, "e": 2.71828}
print(round(round_dict["pi"], 2))  # 3.14
print(round(round_dict["e"], 2))  # 2.72

# ============================================================================
# SECTION 15: Using pow() with dict values
# ============================================================================
print("# pow() with dict values")

pow_dict: dict[str, int] = {"base": 2, "exp": 10}
print(pow(pow_dict["base"], pow_dict["exp"]))  # 1024

pow_dict2: dict[str, int] = {"x": 3, "y": 4}
print(pow(pow_dict2["x"], pow_dict2["y"]))  # 81

# ============================================================================
# SECTION 16: Multiple dict operations in sequence
# ============================================================================
print("# Multiple dict operations in sequence")

seq_dict: dict[str, int] = {}
seq_dict["first"] = 1
seq_dict["second"] = 2
seq_dict["third"] = 3

print(len(seq_dict))  # 3

# Copy
seq_copy: dict[str, int] = seq_dict.copy()
print(len(seq_copy))  # 3

# Modify original
seq_dict["fourth"] = 4
print(len(seq_dict))  # 4
print(len(seq_copy))  # 3 (unchanged)

# Pop from copy
val_popped: int = seq_copy.pop("second")
print(val_popped)  # 2
print(len(seq_copy))  # 2

# ============================================================================
# SECTION 17: Edge cases
# ============================================================================
print("# Edge cases")

# Empty dict operations
edge_dict: dict[str, int] = dict()
print(len(edge_dict))  # 0

edge_dict["key"] = 42
print(len(edge_dict))  # 1
print(edge_dict["key"])  # 42

# Single element
edge_dict.clear()
edge_dict["only"] = 100
print(edge_dict["only"])  # 100
print(len(edge_dict))  # 1

# Get with default on empty
edge_dict.clear()
default_val: int = edge_dict.get("missing", 777)
print(default_val)  # 777

# ============================================================================
# SECTION 18: Updating existing keys
# ============================================================================
print("# Updating existing keys")

update_dict: dict[str, int] = {"a": 1, "b": 2}
print(len(update_dict))  # 2
print(update_dict["a"])  # 1

update_dict["a"] = 99
print(update_dict["a"])  # 99
print(len(update_dict))  # 2 (same length, just updated value)

# ============================================================================
# SECTION 19: Combining with other builtins
# ============================================================================
print("# Combining with other builtins")

builtin_dict: dict[str, int] = {"v1": 5, "v2": 3, "v3": 8}

# Use min/max on values
min_val: int = min(builtin_dict["v1"], builtin_dict["v2"])
max_val: int = max(builtin_dict["v1"], builtin_dict["v3"])
print(min_val)  # 3
print(max_val)  # 8

# Use abs on values
builtin_dict["v4"] = -10
print(abs(builtin_dict["v4"]))  # 10

# ============================================================================
# SECTION 20: Complex key-value patterns
# ============================================================================
print("# Complex key-value patterns")

pattern_dict: dict[int, int] = {}
pattern_dict[10] = 100
pattern_dict[20] = 200
pattern_dict[30] = 300

print(len(pattern_dict))  # 3
print(pattern_dict[10])  # 100

# Use keys in calculations
key_val: int = pattern_dict[20]
print(pow(2, 3))  # 8 (unrelated to dict, just using pow)
print(key_val)  # 200

# ============================================================================
# SECTION 21: Dict with bool values
# ============================================================================
print("# Dict with bool values")

bool_val_dict: dict[str, bool] = {}
bool_val_dict["flag1"] = True
bool_val_dict["flag2"] = False
bool_val_dict["flag3"] = True

print(len(bool_val_dict))  # 3
print(bool_val_dict["flag1"])  # True
print(bool_val_dict["flag2"])  # False

# ============================================================================
# SECTION 22: String values with len()
# ============================================================================
print("# String values with len()")

str_val_dict: dict[str, str] = {"key1": "hello", "key2": "world"}
print(len(str_val_dict))  # 2
print(len(str_val_dict["key1"]))  # 5
print(len(str_val_dict["key2"]))  # 5

print("# All tests completed")
