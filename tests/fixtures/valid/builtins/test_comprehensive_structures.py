# Comprehensive test: Data structures with builtin functions
# This test enumerates combinations of list, dict, set, bytes with builtins

# ============================================================================
# LIST TESTS
# ============================================================================

# len() with list
list1: list[int] = [1, 2, 3, 4, 5]
print(len(list1))  # 5

# list() constructor
empty_list: list[int] = list()
print(len(empty_list))  # 0

# append
list1.append(6)
print(len(list1))  # 6

# extend
ext_list: list[int] = [7, 8]
list1.extend(ext_list)
print(len(list1))  # 8

# insert
list1.insert(0, 0)
print(list1[0])  # 0
print(len(list1))  # 9

# pop
val1: int = list1.pop(0)
print(val1)  # 0
print(len(list1))  # 8

# remove
list1.remove(2)
print(len(list1))  # 7

# index
idx1: int = list1.index(3)
print(idx1)  # 1

# count
cnt_list: list[int] = [1, 2, 2, 3, 2]
cnt1: int = cnt_list.count(2)
print(cnt1)  # 3

# copy
copy_list: list[int] = list1.copy()
print(len(copy_list))  # 7

# sort
sort_list: list[int] = [5, 2, 8, 1]
sort_list.sort()
print(sort_list[0])  # 1
print(sort_list[3])  # 8

# reverse
rev_list: list[int] = [1, 2, 3]
rev_list.reverse()
print(rev_list[0])  # 3

# clear
clear_list: list[int] = [1, 2, 3]
clear_list.clear()
print(len(clear_list))  # 0

# min/max with list elements
minmax_list: list[int] = [10, 20, 30]
print(min(minmax_list[0], minmax_list[1]))  # 10
print(max(minmax_list[1], minmax_list[2]))  # 30

# abs with list elements
abs_list: list[int] = [-5, -10, 3]
print(abs(abs_list[0]))  # 5
print(abs(abs_list[1]))  # 10

# pow with list elements
pow_list: list[int] = [2, 3, 4]
print(pow(pow_list[0], pow_list[1]))  # 8

# ============================================================================
# DICT TESTS
# ============================================================================

# len() with dict
dict1: dict[str, int] = {"a": 1, "b": 2, "c": 3}
print(len(dict1))  # 3

# dict() constructor
empty_dict: dict[str, int] = dict()
print(len(empty_dict))  # 0

# get
val2: int = dict1.get("a", 0)
print(val2)  # 1

val3: int = dict1.get("missing", 99)
print(val3)  # 99

# pop
dict_val: int = dict1.pop("b")
print(dict_val)  # 2
print(len(dict1))  # 2

# copy
copy_dict: dict[str, int] = dict1.copy()
print(len(copy_dict))  # 2

# clear
clear_dict: dict[str, int] = {"x": 1}
clear_dict.clear()
print(len(clear_dict))  # 0

# min/max with dict values
minmax_dict: dict[str, int] = {"x": 5, "y": 10, "z": 15}
print(min(minmax_dict["x"], minmax_dict["y"]))  # 5
print(max(minmax_dict["y"], minmax_dict["z"]))  # 15

# abs with dict values
abs_dict: dict[str, int] = {"neg": -42}
print(abs(abs_dict["neg"]))  # 42

# pow with dict values
pow_dict: dict[str, int] = {"base": 2, "exp": 10}
print(pow(pow_dict["base"], pow_dict["exp"]))  # 1024

# ============================================================================
# SET TESTS
# ============================================================================

# len() with set
set1: set[int] = {1, 2, 3, 4, 5}
print(len(set1))  # 5

# set() constructor
empty_set: set[int] = set()
print(len(empty_set))  # 0

# add
set1.add(6)
print(len(set1))  # 6

# remove
set1.remove(1)
print(len(set1))  # 5

# discard
set1.discard(2)
print(len(set1))  # 4

# pop
set_val: int = set1.pop()
print(len(set1))  # 3

# copy
copy_set: set[int] = set1.copy()
print(len(copy_set))  # 3

# union
set_a: set[int] = {1, 2, 3}
set_b: set[int] = {3, 4, 5}
union_set: set[int] = set_a.union(set_b)
print(len(union_set))  # 5

# intersection
inter_set: set[int] = set_a.intersection(set_b)
print(len(inter_set))  # 1

# difference
diff_set: set[int] = set_a.difference(set_b)
print(len(diff_set))  # 2

# symmetric_difference
sym_set: set[int] = set_a.symmetric_difference(set_b)
print(len(sym_set))  # 4

# issubset
is_sub: bool = set_a.issubset(union_set)
print(is_sub)  # True

# issuperset
is_super: bool = union_set.issuperset(set_a)
print(is_super)  # True

# isdisjoint
set_c: set[int] = {10, 11}
is_disj: bool = set_a.isdisjoint(set_c)
print(is_disj)  # True

# clear
clear_set: set[int] = {1, 2}
clear_set.clear()
print(len(clear_set))  # 0

# ============================================================================
# BYTES TESTS
# ============================================================================

# len() with bytes
bytes1: bytes = b"Hello"
print(len(bytes1))  # 5

# indexing
byte_val1: int = bytes1[0]
byte_val2: int = bytes1[1]
print(min(byte_val1, byte_val2))  # 72
print(max(byte_val1, byte_val2))  # 101

# count
count_bytes: bytes = b"hello hello"
cnt2: int = count_bytes.count(b"hello")
print(cnt2)  # 2

# upper
upper_bytes: bytes = bytes1.upper()
print(len(upper_bytes))  # 5

# lower
lower_bytes: bytes = bytes1.lower()
print(len(lower_bytes))  # 5

# capitalize
cap_bytes: bytes = b"hello"
cap_result: bytes = cap_bytes.capitalize()
print(len(cap_result))  # 5

# title
title_bytes: bytes = b"hello world"
title_result: bytes = title_bytes.title()
print(len(title_result))  # 11

# swapcase
swap_bytes: bytes = b"HeLLo"
swap_result: bytes = swap_bytes.swapcase()
print(len(swap_result))  # 5

# strip
strip_bytes: bytes = b"  hello  "
strip_result: bytes = strip_bytes.strip()
print(len(strip_result))  # 5

# lstrip
lstrip_result: bytes = strip_bytes.lstrip()
print(len(lstrip_result))  # 7

# rstrip
rstrip_result: bytes = strip_bytes.rstrip()
print(len(rstrip_result))  # 7

# replace
replace_bytes: bytes = b"abc"
replace_result: bytes = replace_bytes.replace(b"a", b"x")
print(len(replace_result))  # 3

# startswith
starts: bool = bytes1.startswith(b"Hello")
print(starts)  # True

# endswith
ends: bool = bytes1.endswith(b"lo")
print(ends)  # True

# center
center_bytes: bytes = b"hi"
center_result: bytes = center_bytes.center(10)
print(len(center_result))  # 10

# ljust
ljust_result: bytes = center_bytes.ljust(10)
print(len(ljust_result))  # 10

# rjust
rjust_result: bytes = center_bytes.rjust(10)
print(len(rjust_result))  # 10

# zfill
zfill_bytes: bytes = b"42"
zfill_result: bytes = zfill_bytes.zfill(5)
print(len(zfill_result))  # 5

# isalpha
alpha_bytes: bytes = b"hello"
is_alpha: bool = alpha_bytes.isalpha()
print(is_alpha)  # True

# isdigit
digit_bytes: bytes = b"123"
is_digit: bool = digit_bytes.isdigit()
print(is_digit)  # True

# isalnum
alnum_bytes: bytes = b"hello123"
is_alnum: bool = alnum_bytes.isalnum()
print(is_alnum)  # True

# isspace
space_bytes: bytes = b"   "
is_space: bool = space_bytes.isspace()
print(is_space)  # True

# isupper
upper_test: bytes = b"HELLO"
is_upper: bool = upper_test.isupper()
print(is_upper)  # True

# islower
lower_test: bytes = b"hello"
is_lower: bool = lower_test.islower()
print(is_lower)  # True

# ============================================================================
# CROSS-STRUCTURE TESTS
# ============================================================================

# len() on all structures
cross_list: list[int] = [1, 2, 3]
cross_dict: dict[str, int] = {"a": 1, "b": 2}
cross_set: set[int] = {10, 20, 30}
cross_bytes: bytes = b"test"

print(len(cross_list))  # 3
print(len(cross_dict))  # 2
print(len(cross_set))  # 3
print(len(cross_bytes))  # 4

# min/max across structures
list_val_x: int = cross_list[0]
dict_val_x: int = cross_dict["a"]
print(min(list_val_x, dict_val_x))  # 1
print(max(list_val_x, dict_val_x))  # 1

# abs across structures
print(abs(-cross_list[0]))  # 1
print(abs(-cross_dict["b"]))  # 2

# ============================================================================
# BUILTIN FUNCTIONS
# ============================================================================

# abs
print(abs(-42))  # 42
print(abs(42))  # 42

# round
print(round(3.7))  # 4
print(round(3.2))  # 3
print(round(3.14159, 2))  # 3.14

# min
print(min(5, 3))  # 3
print(min(1, 2, 3))  # 1

# max
print(max(5, 3))  # 5
print(max(1, 2, 3))  # 3

# pow
print(pow(2, 3))  # 8
print(pow(2, 10))  # 1024
print(pow(2, 3, 5))  # 3

print(999)  # Final marker
