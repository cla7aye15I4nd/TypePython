# Comprehensive test for all set operations with all applicable builtin functions
# Tests: len, set() constructor, min/max on set elements
# Set methods: add, remove, discard, pop, clear, copy, union, intersection,
#              difference, symmetric_difference, issubset, issuperset, isdisjoint

# ============================================================================
# SECTION 1: len() with sets
# ============================================================================
print("# len() with sets")

empty_set: set[int] = set()
print(len(empty_set))  # 0

single_set: set[int] = {42}
print(len(single_set))  # 1

multi_set: set[int] = {1, 2, 3, 4, 5}
print(len(multi_set))  # 5

# ============================================================================
# SECTION 2: set() constructor
# ============================================================================
print("# set() constructor")

new_set: set[int] = set()
print(len(new_set))  # 0

# ============================================================================
# SECTION 3: Set method - add()
# ============================================================================
print("# add() method")

add_set: set[int] = set()
add_set.add(10)
add_set.add(20)
add_set.add(30)

print(len(add_set))  # 3

# Adding duplicate (no effect on length)
add_set.add(10)
print(len(add_set))  # 3 (still)

# ============================================================================
# SECTION 4: Set method - remove()
# ============================================================================
print("# remove() method")

remove_set: set[int] = {1, 2, 3, 4, 5}
print(len(remove_set))  # 5

remove_set.remove(3)
print(len(remove_set))  # 4

remove_set.remove(1)
print(len(remove_set))  # 3

# ============================================================================
# SECTION 5: Set method - discard()
# ============================================================================
print("# discard() method")

discard_set: set[int] = {10, 20, 30, 40}
print(len(discard_set))  # 4

discard_set.discard(20)
print(len(discard_set))  # 3

# Discard non-existent element (no error)
discard_set.discard(999)
print(len(discard_set))  # 3 (unchanged)

# ============================================================================
# SECTION 6: Set method - pop()
# ============================================================================
print("# pop() method")

pop_set: set[int] = {100, 200, 300}
print(len(pop_set))  # 3

popped1: int = pop_set.pop()
print(len(pop_set))  # 2

popped2: int = pop_set.pop()
print(len(pop_set))  # 1

# ============================================================================
# SECTION 7: Set method - clear()
# ============================================================================
print("# clear() method")

clear_set: set[int] = {1, 2, 3, 4, 5}
print(len(clear_set))  # 5
clear_set.clear()
print(len(clear_set))  # 0

# ============================================================================
# SECTION 8: Set method - copy()
# ============================================================================
print("# copy() method")

orig_set: set[int] = {1, 2, 3}
copy_set: set[int] = orig_set.copy()

print(len(copy_set))  # 3

# Modify copy to verify independence
copy_set.add(4)
print(len(orig_set))  # 3 (unchanged)
print(len(copy_set))  # 4 (modified)

# ============================================================================
# SECTION 9: Set method - union()
# ============================================================================
print("# union() method")

set_a: set[int] = {1, 2, 3}
set_b: set[int] = {3, 4, 5}
union_set: set[int] = set_a.union(set_b)

print(len(union_set))  # 5 (elements: 1, 2, 3, 4, 5)

# Union with non-overlapping sets
set_c: set[int] = {10, 20}
set_d: set[int] = {30, 40}
union_cd: set[int] = set_c.union(set_d)
print(len(union_cd))  # 4

# ============================================================================
# SECTION 10: Set method - intersection()
# ============================================================================
print("# intersection() method")

inter_a: set[int] = {1, 2, 3, 4}
inter_b: set[int] = {3, 4, 5, 6}
inter_set: set[int] = inter_a.intersection(inter_b)

print(len(inter_set))  # 2 (elements: 3, 4)

# No intersection
inter_c: set[int] = {1, 2}
inter_d: set[int] = {3, 4}
inter_cd: set[int] = inter_c.intersection(inter_d)
print(len(inter_cd))  # 0

# ============================================================================
# SECTION 11: Set method - difference()
# ============================================================================
print("# difference() method")

diff_a: set[int] = {1, 2, 3, 4}
diff_b: set[int] = {3, 4, 5}
diff_set: set[int] = diff_a.difference(diff_b)

print(len(diff_set))  # 2 (elements: 1, 2)

# All elements different
diff_c: set[int] = {10, 20}
diff_d: set[int] = {30, 40}
diff_cd: set[int] = diff_c.difference(diff_d)
print(len(diff_cd))  # 2

# ============================================================================
# SECTION 12: Set method - symmetric_difference()
# ============================================================================
print("# symmetric_difference() method")

sym_a: set[int] = {1, 2, 3}
sym_b: set[int] = {3, 4, 5}
sym_diff: set[int] = sym_a.symmetric_difference(sym_b)

print(len(sym_diff))  # 4 (elements: 1, 2, 4, 5)

# No overlap
sym_c: set[int] = {10, 20}
sym_d: set[int] = {30, 40}
sym_cd: set[int] = sym_c.symmetric_difference(sym_d)
print(len(sym_cd))  # 4

# ============================================================================
# SECTION 13: Set method - issubset()
# ============================================================================
print("# issubset() method")

subset_a: set[int] = {1, 2}
subset_b: set[int] = {1, 2, 3, 4}
is_sub1: bool = subset_a.issubset(subset_b)
print(is_sub1)  # True

is_sub2: bool = subset_b.issubset(subset_a)
print(is_sub2)  # False

# ============================================================================
# SECTION 14: Set method - issuperset()
# ============================================================================
print("# issuperset() method")

super_a: set[int] = {1, 2, 3, 4}
super_b: set[int] = {2, 3}
is_super1: bool = super_a.issuperset(super_b)
print(is_super1)  # True

is_super2: bool = super_b.issuperset(super_a)
print(is_super2)  # False

# ============================================================================
# SECTION 15: Set method - isdisjoint()
# ============================================================================
print("# isdisjoint() method")

disj_a: set[int] = {1, 2, 3}
disj_b: set[int] = {4, 5, 6}
is_disj1: bool = disj_a.isdisjoint(disj_b)
print(is_disj1)  # True

disj_c: set[int] = {1, 2, 3}
disj_d: set[int] = {3, 4, 5}
is_disj2: bool = disj_c.isdisjoint(disj_d)
print(is_disj2)  # False (3 is common)

# ============================================================================
# SECTION 16: min() and max() conceptually (set elements via iteration)
# Note: We can't directly iterate, but we can use converted elements
# ============================================================================
print("# min() and max() concepts")

val_set: set[int] = {5, 2, 9, 1, 7}
# We'll demonstrate using known values
print(min(5, 2))  # 2
print(max(9, 7))  # 9
print(min(1, 2, 5))  # 1
print(max(5, 7, 9))  # 9

# ============================================================================
# SECTION 17: Complex combinations
# ============================================================================
print("# Complex combinations")

combo_set: set[int] = set()
combo_set.add(10)
combo_set.add(20)
combo_set.add(30)

print(len(combo_set))  # 3

# Copy and modify
combo_copy: set[int] = combo_set.copy()
combo_copy.add(40)
print(len(combo_set))  # 3
print(len(combo_copy))  # 4

# Union
combo_union: set[int] = combo_set.union(combo_copy)
print(len(combo_union))  # 4

# ============================================================================
# SECTION 18: Nested operations with len()
# ============================================================================
print("# Nested operations with len()")

nested_set: set[int] = {1, 2, 3}
print(len(nested_set))  # 3

nested_set.add(4)
print(len(nested_set))  # 4

nested_set.remove(2)
print(len(nested_set))  # 3

nested_set.clear()
print(len(nested_set))  # 0

# ============================================================================
# SECTION 19: Set with different element types
# ============================================================================
print("# Sets with different element types")

# String sets
str_set: set[str] = {"hello", "world"}
print(len(str_set))  # 2

str_set.add("foo")
print(len(str_set))  # 3

# Float sets
float_set: set[float] = {1.1, 2.2, 3.3}
print(len(float_set))  # 3

float_set.add(4.4)
print(len(float_set))  # 4

# ============================================================================
# SECTION 20: Multiple set operations in sequence
# ============================================================================
print("# Multiple set operations in sequence")

seq_set: set[int] = set()
seq_set.add(5)
seq_set.add(10)
seq_set.add(15)
seq_set.add(20)

print(len(seq_set))  # 4

# Remove some
seq_set.remove(10)
seq_set.remove(20)
print(len(seq_set))  # 2

# Add back
seq_set.add(100)
print(len(seq_set))  # 3

# Copy
seq_copy: set[int] = seq_set.copy()
print(len(seq_copy))  # 3

# ============================================================================
# SECTION 21: Set operations with overlaps
# ============================================================================
print("# Set operations with overlaps")

overlap_a: set[int] = {1, 2, 3, 4, 5}
overlap_b: set[int] = {4, 5, 6, 7, 8}

# Union
overlap_union: set[int] = overlap_a.union(overlap_b)
print(len(overlap_union))  # 8 (1,2,3,4,5,6,7,8)

# Intersection
overlap_inter: set[int] = overlap_a.intersection(overlap_b)
print(len(overlap_inter))  # 2 (4,5)

# Difference
overlap_diff_ab: set[int] = overlap_a.difference(overlap_b)
print(len(overlap_diff_ab))  # 3 (1,2,3)

overlap_diff_ba: set[int] = overlap_b.difference(overlap_a)
print(len(overlap_diff_ba))  # 3 (6,7,8)

# Symmetric difference
overlap_sym: set[int] = overlap_a.symmetric_difference(overlap_b)
print(len(overlap_sym))  # 6 (1,2,3,6,7,8)

# ============================================================================
# SECTION 22: Edge cases
# ============================================================================
print("# Edge cases")

# Empty set operations
edge_set: set[int] = set()
print(len(edge_set))  # 0

edge_set.add(42)
print(len(edge_set))  # 1

# Pop until empty
edge_set.add(100)
edge_set.add(200)
print(len(edge_set))  # 3

v1: int = edge_set.pop()
v2: int = edge_set.pop()
v3: int = edge_set.pop()
print(len(edge_set))  # 0

# Union with empty
edge_empty: set[int] = set()
edge_full: set[int] = {1, 2, 3}
edge_union: set[int] = edge_empty.union(edge_full)
print(len(edge_union))  # 3

# ============================================================================
# SECTION 23: Set relationship tests
# ============================================================================
print("# Set relationship tests")

rel_a: set[int] = {1, 2, 3, 4, 5}
rel_b: set[int] = {2, 3}
rel_c: set[int] = {6, 7}

# Subset/superset relationships
print(rel_b.issubset(rel_a))  # True
print(rel_a.issuperset(rel_b))  # True
print(rel_c.issubset(rel_a))  # False

# Disjoint relationships
print(rel_a.isdisjoint(rel_c))  # True
print(rel_a.isdisjoint(rel_b))  # False

# ============================================================================
# SECTION 24: Using abs() and other builtins with set elements
# ============================================================================
print("# abs() with set elements")

# We can't iterate directly, but we can use values
print(abs(-5))  # 5
print(abs(-10))  # 10

# ============================================================================
# SECTION 25: Using pow() conceptually
# ============================================================================
print("# pow() conceptually")

print(pow(2, 3))  # 8
print(pow(3, 2))  # 9

# ============================================================================
# SECTION 26: Chained set operations
# ============================================================================
print("# Chained set operations")

chain_a: set[int] = {1, 2, 3}
chain_b: set[int] = {2, 3, 4}
chain_c: set[int] = {3, 4, 5}

# Multiple unions
step1: set[int] = chain_a.union(chain_b)
step2: set[int] = step1.union(chain_c)
print(len(step2))  # 5 (1,2,3,4,5)

# Intersection then union
inter_ab: set[int] = chain_a.intersection(chain_b)
union_ic: set[int] = inter_ab.union(chain_c)
print(len(inter_ab))  # 2 (2,3)
print(len(union_ic))  # 4 (2,3,4,5)

print("# All tests completed")
