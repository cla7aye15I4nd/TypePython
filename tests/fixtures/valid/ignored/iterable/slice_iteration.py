# Test iteration over slices

# Iterate slice of list
nums: list[int] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

print(b"First 5:")
for n in nums[:5]:
    print(b"  ", n)

print(b"Last 5:")
for n in nums[-5:]:
    print(b"  ", n)

print(b"Middle (2:7):")
for n in nums[2:7]:
    print(b"  ", n)

print(b"Every other:")
for n in nums[::2]:
    print(b"  ", n)

print(b"Reversed via slice:")
for n in nums[::-1]:
    print(b"  ", n)

print(b"Every third from end:")
for n in nums[::-3]:
    print(b"  ", n)

# Slice of string
text: str = "hello world"
for c in text[6:]:
    print(b"Char:", c)

# Slice with enumerate
for i, n in enumerate(nums[3:8]):
    print(b"Slice index:", i, b"Value:", n)

# Iterate pairs using slices
for a, b in zip(nums[:-1], nums[1:]):
    print(b"Adjacent:", a, b)

# Sliding window via slices
window_size: int = 3
for i in range(len(nums) - window_size + 1):
    window: list[int] = nums[i:i + window_size]
    print(b"Window:", window)

# Compare slices
list1: list[int] = [1, 2, 3, 4, 5]
list2: list[int] = [1, 2, 3, 4, 5]
all_equal: bool = True
for a, b in zip(list1[:3], list2[:3]):
    if a != b:
        all_equal = False
print(b"First 3 equal:", all_equal)
