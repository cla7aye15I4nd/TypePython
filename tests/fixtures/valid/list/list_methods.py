# Test list methods: insert, extend, remove, clear, reverse, sort, pop, index, count, copy

# ============================================================================
# 1. insert - insert element at position
# ============================================================================
print(b"1. insert:")
lst1: list[int] = [1, 2, 3]
lst1.insert(0, 0)
print(lst1)
lst1.insert(2, 99)
print(lst1)
lst1.insert(10, 100)
print(lst1)

# ============================================================================
# 2. extend - extend list with another iterable
# ============================================================================
print(b"2. extend:")
lst2: list[int] = [1, 2, 3]
lst2.extend([4, 5, 6])
print(lst2)
lst2.extend([])
print(lst2)
lst2.extend([7])
print(lst2)

# ============================================================================
# 3. remove - remove first occurrence of value
# ============================================================================
print(b"3. remove:")
lst3: list[int] = [1, 2, 3, 2, 4]
lst3.remove(2)
print(lst3)
lst3.remove(4)
print(lst3)

# ============================================================================
# 4. clear - remove all elements
# ============================================================================
print(b"4. clear:")
lst4: list[int] = [1, 2, 3, 4, 5]
print(len(lst4))
lst4.clear()
print(len(lst4))
print(lst4)

# ============================================================================
# 5. reverse - reverse in place
# ============================================================================
print(b"5. reverse:")
lst5: list[int] = [1, 2, 3, 4, 5]
lst5.reverse()
print(lst5)
lst5a: list[int] = [1]
lst5a.reverse()
print(lst5a)
lst5b: list[int] = []
lst5b.reverse()
print(lst5b)

# ============================================================================
# 6. sort - sort in place
# ============================================================================
print(b"6. sort:")
lst6: list[int] = [3, 1, 4, 1, 5, 9, 2, 6]
lst6.sort()
print(lst6)
lst6a: list[int] = [5, 4, 3, 2, 1]
lst6a.sort()
print(lst6a)
lst6b: list[int] = []
lst6b.sort()
print(lst6b)

# ============================================================================
# 7. pop - remove and return element at index (-1 for last)
# ============================================================================
print(b"7. pop:")
lst7: list[int] = [1, 2, 3, 4, 5]
x: int = lst7.pop(-1)
print(x)
print(lst7)
y: int = lst7.pop(-1)
print(y)
print(lst7)
z: int = lst7.pop(0)
print(z)
print(lst7)

# ============================================================================
# 8. index - find index of first occurrence
# ============================================================================
print(b"8. index:")
lst8: list[int] = [10, 20, 30, 20, 40]
print(lst8.index(20))
print(lst8.index(30))
print(lst8.index(10))

# ============================================================================
# 9. count - count occurrences
# ============================================================================
print(b"9. count:")
lst9: list[int] = [1, 2, 2, 3, 2, 4, 2]
print(lst9.count(2))
print(lst9.count(1))
print(lst9.count(5))
print(lst9.count(3))

# ============================================================================
# 10. copy - shallow copy
# ============================================================================
print(b"10. copy:")
lst10: list[int] = [1, 2, 3]
lst10_copy: list[int] = lst10.copy()
print(lst10_copy)
lst10.append(4)
print(lst10)
print(lst10_copy)

# ============================================================================
# 11. Combined operations
# ============================================================================
print(b"11. combined:")
lst11: list[int] = [3, 1, 2]
lst11.append(4)
lst11.insert(0, 0)
lst11.extend([5, 6])
print(lst11)
lst11.sort()
print(lst11)
lst11.reverse()
print(lst11)
