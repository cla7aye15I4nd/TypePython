# Test all mutating set methods with integer sets
# Tests: remove, discard, pop, clear, update, difference_update,
#        intersection_update, symmetric_difference_update

# ============================================================================
# SECTION 1: remove()
# ============================================================================
print("# remove()")

remove_set: set[int] = {1, 2, 3, 4, 5}
print(len(remove_set))  # 5

remove_set.remove(3)
print(len(remove_set))  # 4

remove_set.remove(1)
print(len(remove_set))  # 3

remove_set.remove(5)
print(len(remove_set))  # 2

# ============================================================================
# SECTION 2: discard()
# ============================================================================
print("# discard()")

discard_set: set[int] = {10, 20, 30, 40}
print(len(discard_set))  # 4

discard_set.discard(20)
print(len(discard_set))  # 3

# Discard non-existent element (no error, no change)
discard_set.discard(999)
print(len(discard_set))  # 3

discard_set.discard(10)
print(len(discard_set))  # 2

# ============================================================================
# SECTION 3: pop()
# ============================================================================
print("# pop()")

pop_set: set[int] = {100, 200, 300}
print(len(pop_set))  # 3

v1: int = pop_set.pop()
print(len(pop_set))  # 2

v2: int = pop_set.pop()
print(len(pop_set))  # 1

v3: int = pop_set.pop()
print(len(pop_set))  # 0

# ============================================================================
# SECTION 4: clear()
# ============================================================================
print("# clear()")

clear_set: set[int] = {1, 2, 3, 4, 5}
print(len(clear_set))  # 5

clear_set.clear()
print(len(clear_set))  # 0

# Clear already empty set (no error)
empty_set: set[int] = set()
empty_set.clear()
print(len(empty_set))  # 0

# ============================================================================
# SECTION 5: update()
# ============================================================================
print("# update()")

update_set1: set[int] = {1, 2, 3}
update_set2: set[int] = {3, 4, 5}
print(len(update_set1))  # 3

update_set1.update(update_set2)
print(len(update_set1))  # 5 (1, 2, 3, 4, 5)

# Update with disjoint set
update_set3: set[int] = {10, 20, 30}
update_set4: set[int] = {40, 50}
update_set3.update(update_set4)
print(len(update_set3))  # 5

# Update with empty set
update_set5: set[int] = {1, 2}
empty_update: set[int] = set()
update_set5.update(empty_update)
print(len(update_set5))  # 2 (unchanged)

# ============================================================================
# SECTION 6: difference_update()
# ============================================================================
print("# difference_update()")

diff_set1: set[int] = {1, 2, 3, 4, 5}
diff_set2: set[int] = {3, 4, 5, 6}
print(len(diff_set1))  # 5

diff_set1.difference_update(diff_set2)
print(len(diff_set1))  # 2 (only 1, 2 remain)

# difference_update with disjoint set (no change)
diff_set3: set[int] = {10, 20, 30}
diff_set4: set[int] = {40, 50}
diff_set3.difference_update(diff_set4)
print(len(diff_set3))  # 3 (unchanged)

# difference_update removes all common elements
diff_set5: set[int] = {1, 2, 3}
diff_set6: set[int] = {1, 2, 3}
diff_set5.difference_update(diff_set6)
print(len(diff_set5))  # 0

# ============================================================================
# SECTION 7: intersection_update()
# ============================================================================
print("# intersection_update()")

inter_set1: set[int] = {1, 2, 3, 4, 5}
inter_set2: set[int] = {3, 4, 5, 6}
print(len(inter_set1))  # 5

inter_set1.intersection_update(inter_set2)
print(len(inter_set1))  # 3 (only 3, 4, 5 remain)

# intersection_update with disjoint set (empty result)
inter_set3: set[int] = {1, 2, 3}
inter_set4: set[int] = {4, 5, 6}
inter_set3.intersection_update(inter_set4)
print(len(inter_set3))  # 0

# intersection_update with identical set (no change)
inter_set5: set[int] = {10, 20, 30}
inter_set6: set[int] = {10, 20, 30}
inter_set5.intersection_update(inter_set6)
print(len(inter_set5))  # 3 (unchanged)

# ============================================================================
# SECTION 8: symmetric_difference_update()
# ============================================================================
print("# symmetric_difference_update()")

sym_set1: set[int] = {1, 2, 3}
sym_set2: set[int] = {3, 4, 5}
print(len(sym_set1))  # 3

sym_set1.symmetric_difference_update(sym_set2)
print(len(sym_set1))  # 4 (1, 2, 4, 5)

# symmetric_difference_update with disjoint sets (union)
sym_set3: set[int] = {10, 20}
sym_set4: set[int] = {30, 40}
sym_set3.symmetric_difference_update(sym_set4)
print(len(sym_set3))  # 4

# symmetric_difference_update with identical sets (empty)
sym_set5: set[int] = {1, 2, 3}
sym_set6: set[int] = {1, 2, 3}
sym_set5.symmetric_difference_update(sym_set6)
print(len(sym_set5))  # 0

# ============================================================================
# SECTION 9: Combined operations
# ============================================================================
print("# Combined operations")

combo_set: set[int] = set()
combo_set.add(1)
combo_set.add(2)
combo_set.add(3)
combo_set.add(4)
combo_set.add(5)
print(len(combo_set))  # 5

combo_set.remove(3)
print(len(combo_set))  # 4

combo_set.discard(999)  # no effect
print(len(combo_set))  # 4

popped: int = combo_set.pop()
print(len(combo_set))  # 3

other_set: set[int] = {10, 20}
combo_set.update(other_set)
print(len(combo_set))  # 5

combo_set.clear()
print(len(combo_set))  # 0

print("# All tests passed!")
