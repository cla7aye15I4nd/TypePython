# Test enumerate() builtin

# Basic enumerate
fruits: list[str] = ["apple", "banana", "cherry"]
for i, fruit in enumerate(fruits):
    print(b"Index:", i, b"Fruit:", fruit)

# Enumerate with start
for i, fruit in enumerate(fruits, 1):
    print(b"Position:", i, b"Fruit:", fruit)

# Enumerate string
for i, c in enumerate("abc"):
    print(b"Index:", i, b"Char:", c)

# Enumerate with different types
numbers: list[int] = [10, 20, 30]
for idx, val in enumerate(numbers):
    print(b"nums[", idx, b"] =", val)

# Use enumerate index for modification
values: list[int] = [1, 2, 3, 4, 5]
for i, v in enumerate(values):
    values[i] = v * 2
print(b"Doubled:", values)
