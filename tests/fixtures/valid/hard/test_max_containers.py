# Test max() with bytes and list (these are comparable in Python)

# Test max with bytes (lexicographic comparison)
b1: bytes = b"hi"
b2: bytes = b"hello"
b3: bytes = b"a"
max_bytes: bytes = max(b1, b2, b3)
print(max_bytes)  # b"hi" (lexicographically largest)

# Test max with two bytes
shorter: bytes = b"ab"
longer: bytes = b"abcde"
result: bytes = max(shorter, longer)
print(result)  # b"abcde" (lexicographically larger)

# Test max with lists (element-by-element comparison)
list1: list[int] = [1, 2]
list2: list[int] = [3, 4, 5, 6]
list3: list[int] = [7]
max_list: list[int] = max(list1, list2, list3)
print(max_list)  # [7] (7 > 3 > 1)

# Test min with bytes
min_bytes: bytes = min(b1, b2, b3)
print(min_bytes)  # b"a" (lexicographically smallest)

# Test min with lists
min_list: list[int] = min(list1, list2, list3)
print(min_list)  # [1, 2] (1 < 3 < 7)
