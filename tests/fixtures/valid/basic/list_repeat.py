# Test list repetition operator
a: list[int] = [1, 2, 3]
b: list[int] = a * 2
print(len(b))
print(b)

c: list[int] = [10] * 5
print(len(c))
print(c)

# Repeat empty list
empty: list[int] = []
repeated_empty: list[int] = empty * 3
print(len(repeated_empty))

# Repeat with 0
d: list[int] = [1, 2, 3] * 0
print(len(d))

# Repeat with 1 (same as original)
e: list[int] = [100, 200] * 1
print(len(e))
print(e)

print("list repeat test passed!")
