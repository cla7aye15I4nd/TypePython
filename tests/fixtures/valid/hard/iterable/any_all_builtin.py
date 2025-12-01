# Test any() and all() builtins with iterables

# any() with list
nums: list[int] = [0, 0, 0, 1, 0]
print(b"Any non-zero:", any(nums))

all_zero: list[int] = [0, 0, 0]
print(b"Any in all zeros:", any(all_zero))

# all() with list
all_positive: list[int] = [1, 2, 3, 4, 5]
print(b"All positive:", all(all_positive))

has_zero: list[int] = [1, 2, 0, 4]
print(b"All non-zero:", all(has_zero))

# any/all with booleans
bools: list[bool] = [True, False, True]
print(b"Any true:", any(bools))
print(b"All true:", all(bools))

all_true: list[bool] = [True, True, True]
print(b"All true list:", all(all_true))

# any/all with generator expression
nums2: list[int] = [1, 2, 3, 4, 5]
print(b"Any > 3:", any(x > 3 for x in nums2))
print(b"All > 0:", all(x > 0 for x in nums2))
print(b"All > 3:", all(x > 3 for x in nums2))

# any/all with empty
empty: list[int] = []
print(b"Any empty:", any(empty))
print(b"All empty:", all(empty))

# any/all with strings (non-empty = truthy)
strings: list[str] = ["hello", "", "world"]
print(b"Any non-empty:", any(strings))
print(b"All non-empty:", all(strings))
