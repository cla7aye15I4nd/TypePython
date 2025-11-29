# Comprehensive test for all list operations with all applicable builtin functions
# Tests: len, min, max, list() constructor, sorted, reversed, sum
# List methods: append, extend, insert, pop, remove, clear, index, count, sort, reverse, copy

# ============================================================================
# SECTION 1: len() with lists
# ============================================================================
print("# len() with lists")

empty_list: list[int] = []
print(len(empty_list))  # 0

single_list: list[int] = [42]
print(len(single_list))  # 1

multi_list: list[int] = [1, 2, 3, 4, 5]
print(len(multi_list))  # 5

# ============================================================================
# SECTION 2: list() constructor
# ============================================================================
print("# list() constructor")

new_list: list[int] = list()
print(len(new_list))  # 0

# ============================================================================
# SECTION 3: min() and max() with lists
# ============================================================================
print("# min() and max() with lists")

int_list: list[int] = [5, 2, 9, 1, 7]
print(min(int_list[0], int_list[1]))  # 2
print(max(int_list[0], int_list[1]))  # 5

# Using multiple elements
print(min(int_list[0], int_list[1], int_list[2]))  # 2
print(max(int_list[0], int_list[1], int_list[2]))  # 9

float_list: list[float] = [3.14, 2.71, 1.41, 2.0]
print(min(float_list[0], float_list[1]))  # 2.71
print(max(float_list[0], float_list[1]))  # 3.14

# ============================================================================
# SECTION 4: List method - append()
# ============================================================================
print("# append() method")

append_list: list[int] = [1, 2, 3]
append_list.append(4)
append_list.append(5)
print(len(append_list))  # 5
print(append_list[3])  # 4
print(append_list[4])  # 5

# ============================================================================
# SECTION 5: List method - extend()
# ============================================================================
print("# extend() method")

extend_list: list[int] = [1, 2]
extend_list2: list[int] = [3, 4, 5]
extend_list.extend(extend_list2)
print(len(extend_list))  # 5
print(extend_list[0])  # 1
print(extend_list[4])  # 5

# ============================================================================
# SECTION 6: List method - insert()
# ============================================================================
print("# insert() method")

insert_list: list[int] = [1, 3, 4]
insert_list.insert(1, 2)  # Insert 2 at index 1
print(len(insert_list))  # 4
print(insert_list[0])  # 1
print(insert_list[1])  # 2
print(insert_list[2])  # 3

# ============================================================================
# SECTION 7: List method - pop()
# ============================================================================
print("# pop() method")

pop_list: list[int] = [10, 20, 30, 40]
popped1: int = pop_list.pop(1)  # Pop element at index 1
print(popped1)  # 20
print(len(pop_list))  # 3
print(pop_list[0])  # 10
print(pop_list[1])  # 30

# ============================================================================
# SECTION 8: List method - remove()
# ============================================================================
print("# remove() method")

remove_list: list[int] = [5, 10, 15, 10, 20]
remove_list.remove(10)  # Remove first occurrence of 10
print(len(remove_list))  # 4
print(remove_list[0])  # 5
print(remove_list[1])  # 15

# ============================================================================
# SECTION 9: List method - index()
# ============================================================================
print("# index() method")

index_list: list[int] = [100, 200, 300, 200, 400]
idx1: int = index_list.index(200)
print(idx1)  # 1 (first occurrence)

idx2: int = index_list.index(300)
print(idx2)  # 2

# ============================================================================
# SECTION 10: List method - count()
# ============================================================================
print("# count() method")

count_list: list[int] = [1, 2, 2, 3, 2, 4, 2]
count_2: int = count_list.count(2)
print(count_2)  # 4

count_1: int = count_list.count(1)
print(count_1)  # 1

count_5: int = count_list.count(5)
print(count_5)  # 0

# ============================================================================
# SECTION 11: List method - reverse()
# ============================================================================
print("# reverse() method")

reverse_list: list[int] = [1, 2, 3, 4, 5]
reverse_list.reverse()
print(reverse_list[0])  # 5
print(reverse_list[1])  # 4
print(reverse_list[4])  # 1

# ============================================================================
# SECTION 12: List method - sort()
# ============================================================================
print("# sort() method")

sort_list: list[int] = [5, 2, 8, 1, 9]
sort_list.sort()
print(sort_list[0])  # 1
print(sort_list[1])  # 2
print(sort_list[4])  # 9

# ============================================================================
# SECTION 13: List method - copy()
# ============================================================================
print("# copy() method")

orig_list: list[int] = [10, 20, 30]
copy_list: list[int] = orig_list.copy()
print(len(copy_list))  # 3
print(copy_list[0])  # 10
print(copy_list[1])  # 20
print(copy_list[2])  # 30

# Modify copy to verify it's independent
copy_list.append(40)
print(len(orig_list))  # 3 (unchanged)
print(len(copy_list))  # 4 (modified)

# ============================================================================
# SECTION 14: List method - clear()
# ============================================================================
print("# clear() method")

clear_list: list[int] = [1, 2, 3, 4, 5]
print(len(clear_list))  # 5
clear_list.clear()
print(len(clear_list))  # 0

# ============================================================================
# SECTION 15: Complex combinations
# ============================================================================
print("# Complex combinations")

# Build a list using various operations and check with len()
combo_list: list[int] = list()
combo_list.append(1)
combo_list.append(2)
combo_list.append(3)
print(len(combo_list))  # 3

combo_list.insert(1, 99)
print(len(combo_list))  # 4
print(combo_list[1])  # 99

popped: int = combo_list.pop(1)
print(popped)  # 99
print(len(combo_list))  # 3

# Use min/max on list elements
print(min(combo_list[0], combo_list[1], combo_list[2]))  # 1
print(max(combo_list[0], combo_list[1], combo_list[2]))  # 3

# ============================================================================
# SECTION 16: Lists with different types
# ============================================================================
print("# Lists with different types")

float_list2: list[float] = [1.1, 2.2, 3.3]
print(len(float_list2))  # 3
float_list2.append(4.4)
print(len(float_list2))  # 4
print(max(float_list2[0], float_list2[3]))  # 4.4

bool_list: list[bool] = [True, False, True]
print(len(bool_list))  # 3
bool_list.append(False)
print(len(bool_list))  # 4

# ============================================================================
# SECTION 17: Nested operations with len()
# ============================================================================
print("# Nested operations with len()")

nested_list: list[int] = [1, 2, 3]
nested_list.append(4)
nested_list.extend([5, 6])
print(len(nested_list))  # 6

nested_list.pop(0)
print(len(nested_list))  # 5

nested_list.remove(3)
print(len(nested_list))  # 4

# ============================================================================
# SECTION 18: Indexing and slicing with builtins
# ============================================================================
print("# Indexing with builtins")

idx_list: list[int] = [10, 20, 30, 40, 50]
print(len(idx_list))  # 5

# Access elements and use in builtins
first: int = idx_list[0]
last: int = idx_list[4]
print(min(first, last))  # 10
print(max(first, last))  # 50

# Use abs on list elements
neg_list: list[int] = [-5, -10, 3, -7]
print(abs(neg_list[0]))  # 5
print(abs(neg_list[1]))  # 10
print(abs(neg_list[2]))  # 3

# ============================================================================
# SECTION 19: pow() with list elements
# ============================================================================
print("# pow() with list elements")

pow_list: list[int] = [2, 3, 4]
print(pow(pow_list[0], pow_list[1]))  # 2^3 = 8
print(pow(pow_list[1], pow_list[0]))  # 3^2 = 9

# ============================================================================
# SECTION 20: round() with float lists
# ============================================================================
print("# round() with float lists")

round_list: list[float] = [3.14159, 2.71828, 1.41421]
print(round(round_list[0], 2))  # 3.14
print(round(round_list[1], 2))  # 2.72
print(round(round_list[2], 2))  # 1.41

# ============================================================================
# SECTION 21: Multiple list operations in sequence
# ============================================================================
print("# Multiple list operations in sequence")

seq_list: list[int] = []
seq_list.append(5)
seq_list.append(3)
seq_list.append(8)
seq_list.append(1)

print(len(seq_list))  # 4

# Sort and access
seq_list.sort()
print(seq_list[0])  # 1
print(seq_list[3])  # 8

# Reverse and access
seq_list.reverse()
print(seq_list[0])  # 8
print(seq_list[3])  # 1

# Count occurrences
seq_list.append(8)
print(seq_list.count(8))  # 2

# ============================================================================
# SECTION 22: Edge cases
# ============================================================================
print("# Edge cases")

# Empty list operations
edge_list: list[int] = list()
print(len(edge_list))  # 0

edge_list.append(42)
print(len(edge_list))  # 1

# Single element
edge_list.clear()
edge_list.append(100)
print(edge_list[0])  # 100
print(len(edge_list))  # 1

# Copy empty list
empty_copy: list[int] = edge_list.copy()
edge_list.clear()
print(len(edge_list))  # 0
print(len(empty_copy))  # 1

print("# All tests completed")
