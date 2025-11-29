# Test max() with bytes, dict, list, and set (compares by length)

# Test max with bytes
b1: bytes = b"hi"
b2: bytes = b"hello"
b3: bytes = b"a"
max_bytes: bytes = max(b1, b2, b3)
print(max_bytes)  # b"hello" (longest)

# Test max with two bytes
shorter: bytes = b"ab"
longer: bytes = b"abcde"
result: bytes = max(shorter, longer)
print(result)  # b"abcde"

# Test max with lists
list1: list[int] = [1, 2]
list2: list[int] = [3, 4, 5, 6]
list3: list[int] = [7]
max_list: list[int] = max(list1, list2, list3)
print(max_list)  # [3, 4, 5, 6] (longest)

# Test max with dicts
dict1: dict[int, int] = {1: 10}
dict2: dict[int, int] = {2: 20, 3: 30, 4: 40}
dict3: dict[int, int] = {5: 50, 6: 60}
max_dict: dict[int, int] = max(dict1, dict2, dict3)
print(max_dict)  # {2: 20, 3: 30, 4: 40} (longest)

# Test max with sets
set1: set[int] = {1, 2}
set2: set[int] = {3}
set3: set[int] = {4, 5, 6}
max_set: set[int] = max(set1, set2, set3)
print(max_set)  # {4, 5, 6} (longest)

# Test min with bytes
min_bytes: bytes = min(b1, b2, b3)
print(min_bytes)  # b"a" (shortest)

# Test min with lists
min_list: list[int] = min(list1, list2, list3)
print(min_list)  # [7] (shortest)
