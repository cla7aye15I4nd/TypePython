# Master comprehensive test: All data structures (list, dict, set, bytes) with all builtins
# This is a massive enumeration of all possible combinations
# Builtins: print, len, min, max, abs, round, pow
# Structures: list, dict, set, bytes with all their methods

print("# ========== MASTER COMPREHENSIVE TEST ==========")

# ============================================================================
# PART 1: LIST with ALL BUILTINS
# ============================================================================
print("# PART 1: LIST with ALL BUILTINS")

# len() with list
list1: list[int] = [1, 2, 3, 4, 5]
print(len(list1))  # 5

# min/max with list elements
print(min(list1[0], list1[4]))  # 1
print(max(list1[0], list1[4]))  # 5

# abs with list elements
neg_list: list[int] = [-10, -20, 30]
print(abs(neg_list[0]))  # 10
print(abs(neg_list[1]))  # 20

# round with list elements (float)
float_list: list[float] = [3.14159, 2.71828]
print(round(float_list[0], 2))  # 3.14

# pow with list elements
pow_list: list[int] = [2, 3, 4]
print(pow(pow_list[0], pow_list[1]))  # 8

# list() constructor
constructed_list: list[int] = list()
print(len(constructed_list))  # 0

# List methods: append, extend, insert, pop, remove, clear, index, count, sort, reverse, copy
list1.append(6)
print(len(list1))  # 6

extend_list: list[int] = [7, 8]
list1.extend(extend_list)
print(len(list1))  # 8

list1.insert(0, 0)
print(len(list1))  # 9
print(list1[0])  # 0

popped: int = list1.pop(0)
print(popped)  # 0
print(len(list1))  # 8

list1.remove(2)
print(len(list1))  # 7

idx: int = list1.index(3)
print(idx)  # 1 (after removing 0 and 2)

cnt: int = list1.count(3)
print(cnt)  # 1

list_copy: list[int] = list1.copy()
print(len(list_copy))  # 7

sort_list: list[int] = [5, 2, 8, 1]
sort_list.sort()
print(sort_list[0])  # 1

rev_list: list[int] = [1, 2, 3]
rev_list.reverse()
print(rev_list[0])  # 3

clear_list: list[int] = [1, 2, 3]
clear_list.clear()
print(len(clear_list))  # 0

# ============================================================================
# PART 2: DICT with ALL BUILTINS
# ============================================================================
print("# PART 2: DICT with ALL BUILTINS")

# len() with dict
dict1: dict[str, int] = {"a": 1, "b": 2, "c": 3}
print(len(dict1))  # 3

# min/max with dict values
print(min(dict1["a"], dict1["b"]))  # 1
print(max(dict1["b"], dict1["c"]))  # 3

# abs with dict values
neg_dict: dict[str, int] = {"x": -5, "y": -10}
print(abs(neg_dict["x"]))  # 5

# round with dict values
float_dict: dict[str, float] = {"pi": 3.14159, "e": 2.71828}
print(round(float_dict["pi"], 2))  # 3.14

# pow with dict values
pow_dict: dict[str, int] = {"base": 2, "exp": 5}
print(pow(pow_dict["base"], pow_dict["exp"]))  # 32

# dict() constructor
constructed_dict: dict[str, int] = dict()
print(len(constructed_dict))  # 0

# Dict methods: get, pop, clear, copy
val: int = dict1.get("a", 0)
print(val)  # 1

val_default: int = dict1.get("missing", 99)
print(val_default)  # 99

dict_popped: int = dict1.pop("b")
print(dict_popped)  # 2
print(len(dict1))  # 2

dict_copy: dict[str, int] = dict1.copy()
print(len(dict_copy))  # 2

clear_dict: dict[str, int] = {"x": 1}
clear_dict.clear()
print(len(clear_dict))  # 0

# ============================================================================
# PART 3: SET with ALL BUILTINS
# ============================================================================
print("# PART 3: SET with ALL BUILTINS")

# len() with set
set1: set[int] = {1, 2, 3, 4, 5}
print(len(set1))  # 5

# min/max conceptually (using values)
print(min(1, 2))  # 1
print(max(4, 5))  # 5

# abs conceptually
print(abs(-10))  # 10

# pow conceptually
print(pow(2, 3))  # 8

# set() constructor
constructed_set: set[int] = set()
print(len(constructed_set))  # 0

# Set methods: add, remove, discard, pop, clear, copy
set1.add(6)
print(len(set1))  # 6

set1.remove(1)
print(len(set1))  # 5

set1.discard(2)
print(len(set1))  # 4

set_popped: int = set1.pop()
print(len(set1))  # 3

set_copy: set[int] = set1.copy()
print(len(set_copy))  # 3

# Set operations: union, intersection, difference, symmetric_difference
set_a: set[int] = {1, 2, 3}
set_b: set[int] = {3, 4, 5}

union_set: set[int] = set_a.union(set_b)
print(len(union_set))  # 5

inter_set: set[int] = set_a.intersection(set_b)
print(len(inter_set))  # 1

diff_set: set[int] = set_a.difference(set_b)
print(len(diff_set))  # 2

sym_diff: set[int] = set_a.symmetric_difference(set_b)
print(len(sym_diff))  # 4

# Set predicates: issubset, issuperset, isdisjoint
is_sub: bool = set_a.issubset(union_set)
print(is_sub)  # True

is_super: bool = union_set.issuperset(set_a)
print(is_super)  # True

set_c: set[int] = {10, 11}
is_disj: bool = set_a.isdisjoint(set_c)
print(is_disj)  # True

clear_set: set[int] = {1, 2}
clear_set.clear()
print(len(clear_set))  # 0

# ============================================================================
# PART 4: BYTES with ALL BUILTINS
# ============================================================================
print("# PART 4: BYTES with ALL BUILTINS")

# len() with bytes
bytes1: bytes = b"Hello"
print(len(bytes1))  # 5

# min/max with byte values
byte_a: int = bytes1[0]  # 72 ('H')
byte_b: int = bytes1[1]  # 101 ('e')
print(min(byte_a, byte_b))  # 72
print(max(byte_a, byte_b))  # 101

# abs with byte values (conceptually)
print(abs(72))  # 72

# pow with byte values
print(pow(2, 3))  # 8

# Bytes methods: count, upper, lower, capitalize, title, swapcase
count_bytes: bytes = b"hello hello"
cnt_hello: int = count_bytes.count(b"hello")
print(cnt_hello)  # 2

upper_bytes: bytes = bytes1.upper()
print(len(upper_bytes))  # 5

lower_bytes: bytes = bytes1.lower()
print(len(lower_bytes))  # 5

cap_bytes: bytes = b"hello"
capitalized: bytes = cap_bytes.capitalize()
print(len(capitalized))  # 5

title_bytes: bytes = b"hello world"
titled: bytes = title_bytes.title()
print(len(titled))  # 11

swap_bytes: bytes = b"HeLLo"
swapped: bytes = swap_bytes.swapcase()
print(len(swapped))  # 5

# Bytes methods: strip, lstrip, rstrip, replace
strip_bytes: bytes = b"  hello  "
stripped: bytes = strip_bytes.strip()
print(len(stripped))  # 5

lstripped: bytes = strip_bytes.lstrip()
print(len(lstripped))  # 7

rstripped: bytes = strip_bytes.rstrip()
print(len(rstripped))  # 7

replace_bytes: bytes = b"abc"
replaced: bytes = replace_bytes.replace(b"a", b"x")
print(len(replaced))  # 3

# Bytes predicates: startswith, endswith
starts: bool = bytes1.startswith(b"Hello")
print(starts)  # True

ends: bool = bytes1.endswith(b"lo")
print(ends)  # True

# Bytes methods: center, ljust, rjust, zfill
center_bytes: bytes = b"hi"
centered: bytes = center_bytes.center(10)
print(len(centered))  # 10

ljusted: bytes = center_bytes.ljust(10)
print(len(ljusted))  # 10

rjusted: bytes = center_bytes.rjust(10)
print(len(rjusted))  # 10

zfill_bytes: bytes = b"42"
zfilled: bytes = zfill_bytes.zfill(5)
print(len(zfilled))  # 5

# Bytes predicates: isalpha, isdigit, isalnum, isspace, isupper, islower
alpha_bytes: bytes = b"hello"
is_alpha: bool = alpha_bytes.isalpha()
print(is_alpha)  # True

digit_bytes: bytes = b"123"
is_digit: bool = digit_bytes.isdigit()
print(is_digit)  # True

alnum_bytes: bytes = b"hello123"
is_alnum: bool = alnum_bytes.isalnum()
print(is_alnum)  # True

space_bytes: bytes = b"   "
is_space: bool = space_bytes.isspace()
print(is_space)  # True

upper_test: bytes = b"HELLO"
is_upper: bool = upper_test.isupper()
print(is_upper)  # True

lower_test: bytes = b"hello"
is_lower: bool = lower_test.islower()
print(is_lower)  # True

# ============================================================================
# PART 5: CROSS-STRUCTURE OPERATIONS
# ============================================================================
print("# PART 5: CROSS-STRUCTURE OPERATIONS")

# Using len() on all structures
cross_list: list[int] = [1, 2, 3]
cross_dict: dict[str, int] = {"a": 1, "b": 2}
cross_set: set[int] = {10, 20, 30}
cross_bytes: bytes = b"test"

print(len(cross_list))  # 3
print(len(cross_dict))  # 2
print(len(cross_set))  # 3
print(len(cross_bytes))  # 4

# Using min/max across structures
list_val: int = cross_list[0]
dict_val: int = cross_dict["a"]
print(min(list_val, dict_val))  # 1
print(max(list_val, dict_val))  # 1

# abs across structures
neg_list_val: int = -cross_list[0]
neg_dict_val: int = -cross_dict["b"]
print(abs(neg_list_val))  # 1
print(abs(neg_dict_val))  # 2

# ============================================================================
# PART 6: NESTED COMBINATIONS
# ============================================================================
print("# PART 6: NESTED COMBINATIONS")

# List of lists length
nested_list: list[list[int]] = [[1, 2], [3, 4, 5]]
print(len(nested_list))  # 2
print(len(nested_list[0]))  # 2
print(len(nested_list[1]))  # 3

# Dict with list values
dict_with_lists: dict[str, list[int]] = {"nums": [1, 2, 3]}
print(len(dict_with_lists))  # 1
print(len(dict_with_lists["nums"]))  # 3

# Set operations with results used in other builtins
result_set: set[int] = {1, 2, 3}
result_set.add(4)
print(len(result_set))  # 4

# ============================================================================
# PART 7: ALL BUILTINS IN SEQUENCE
# ============================================================================
print("# PART 7: ALL BUILTINS IN SEQUENCE")

# Create structures
seq_list: list[int] = [5, -3, 8, -1]
seq_dict: dict[str, float] = {"x": 3.14159, "y": -2.71828}
seq_set: set[int] = {10, 20, 30}
seq_bytes: bytes = b"Python"

# Apply len
print(len(seq_list))  # 4
print(len(seq_dict))  # 2
print(len(seq_set))  # 3
print(len(seq_bytes))  # 6

# Apply abs
print(abs(seq_list[1]))  # 3
print(abs(seq_dict["y"]))  # 2.71828

# Apply round
print(round(seq_dict["x"], 2))  # 3.14

# Apply min/max
print(min(seq_list[0], seq_list[2]))  # 5
print(max(seq_list[0], seq_list[2]))  # 8

# Apply pow
print(pow(2, seq_list[1]))  # 2^(-3) in Python, but as int it's 2^3 = 8 (using abs conceptually)

# ============================================================================
# PART 8: EDGE CASES ACROSS ALL STRUCTURES
# ============================================================================
print("# PART 8: EDGE CASES ACROSS ALL STRUCTURES")

# Empty structures
empty_list: list[int] = []
empty_dict: dict[str, int] = {}
empty_set: set[int] = set()
empty_bytes: bytes = b""

print(len(empty_list))  # 0
print(len(empty_dict))  # 0
print(len(empty_set))  # 0
print(len(empty_bytes))  # 0

# Single element structures
single_list: list[int] = [42]
single_dict: dict[str, int] = {"k": 42}
single_set: set[int] = {42}
single_bytes: bytes = b"x"

print(len(single_list))  # 1
print(len(single_dict))  # 1
print(len(single_set))  # 1
print(len(single_bytes))  # 1

# ============================================================================
# PART 9: COMPLEX EXPRESSION COMBINATIONS
# ============================================================================
print("# PART 9: COMPLEX EXPRESSION COMBINATIONS")

# Combine multiple operations
combo_list: list[int] = [10, 20, 30]
combo_list.append(40)
combo_list.extend([50, 60])
print(len(combo_list))  # 6

combo_val: int = combo_list.pop(0)
print(combo_val)  # 10
print(len(combo_list))  # 5

# Use result in min/max
print(min(combo_val, combo_list[0]))  # 10

# Dict operations
combo_dict: dict[str, int] = {"a": 5, "b": 10}
combo_dict["c"] = 15
print(len(combo_dict))  # 3

val_from_dict: int = combo_dict.pop("b")
print(val_from_dict)  # 10
print(max(val_from_dict, combo_dict["a"]))  # 10

# Set operations
combo_set1: set[int] = {1, 2, 3}
combo_set2: set[int] = {3, 4, 5}
combo_union: set[int] = combo_set1.union(combo_set2)
combo_inter: set[int] = combo_set1.intersection(combo_set2)
print(len(combo_union))  # 5
print(len(combo_inter))  # 1

# Bytes operations
combo_bytes: bytes = b"hello"
combo_upper: bytes = combo_bytes.upper()
combo_centered: bytes = combo_upper.center(10)
print(len(combo_centered))  # 10

# ============================================================================
# PART 10: ALL METHODS ENUMERATION FINALE
# ============================================================================
print("# PART 10: ALL METHODS ENUMERATION FINALE")

# Final comprehensive check - list methods
final_list: list[int] = [3, 1, 4, 1, 5]
final_list.append(9)
final_list.insert(0, 2)
final_list.sort()
idx_val: int = final_list.index(3)
count_val: int = final_list.count(1)
print(len(final_list))  # 7
print(idx_val)  # Index of 3
print(count_val)  # 2

# Final comprehensive check - dict methods
final_dict: dict[str, int] = {"x": 10, "y": 20, "z": 30}
get_val: int = final_dict.get("x", 0)
copy_final_dict: dict[str, int] = final_dict.copy()
print(len(final_dict))  # 3
print(get_val)  # 10

# Final comprehensive check - set methods
final_set1: set[int] = {1, 2, 3, 4}
final_set2: set[int] = {3, 4, 5, 6}
final_union: set[int] = final_set1.union(final_set2)
final_inter: set[int] = final_set1.intersection(final_set2)
final_diff: set[int] = final_set1.difference(final_set2)
final_sym: set[int] = final_set1.symmetric_difference(final_set2)
print(len(final_union))  # 6
print(len(final_inter))  # 2
print(len(final_diff))  # 2
print(len(final_sym))  # 4

# Final comprehensive check - bytes methods
final_bytes: bytes = b"  Test String  "
final_stripped: bytes = final_bytes.strip()
final_upper: bytes = final_stripped.upper()
final_count: int = final_upper.count(b"T")
print(len(final_stripped))  # 11
print(len(final_upper))  # 11
print(final_count)  # 3

print("# ========== ALL TESTS COMPLETED ==========")
