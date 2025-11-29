# Test list extend() method

# Basic extend
lst1: list[int] = [1, 2, 3]
lst2: list[int] = [4, 5, 6]
print(len(lst1))
lst1.extend(lst2)
print(len(lst1))
print(lst1[0])
print(lst1[1])
print(lst1[2])
print(lst1[3])
print(lst1[4])
print(lst1[5])

# Extend with empty list
lst3: list[int] = [10, 20]
empty: list[int] = []
lst3.extend(empty)
print(len(lst3))
print(lst3[0])
print(lst3[1])

# Extend empty list with non-empty
empty2: list[int] = []
lst4: list[int] = [100, 200, 300]
empty2.extend(lst4)
print(len(empty2))
print(empty2[0])
print(empty2[1])
print(empty2[2])

# Extend multiple times
lst5: list[int] = [1]
lst6: list[int] = [2, 3]
lst7: list[int] = [4, 5, 6]
lst5.extend(lst6)
print(len(lst5))
lst5.extend(lst7)
print(len(lst5))
print(lst5[0])
print(lst5[1])
print(lst5[2])
print(lst5[3])
print(lst5[4])
print(lst5[5])

# Extend with single element list
lst8: list[int] = [7, 8, 9]
single: list[int] = [10]
lst8.extend(single)
print(len(lst8))
print(lst8[3])

# Extend with larger list
lst9: list[int] = [1, 2]
large: list[int] = [3, 4, 5, 6, 7, 8, 9, 10]
lst9.extend(large)
print(len(lst9))
print(lst9[0])
print(lst9[9])

# Extend doesn't modify source list
lst10: list[int] = [11, 12]
lst11: list[int] = [13, 14]
print(len(lst10))
print(len(lst11))
lst10.extend(lst11)
print(len(lst10))
print(len(lst11))
