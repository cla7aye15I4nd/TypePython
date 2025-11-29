# Test dict pop method

d: dict[int, int] = {1: 10, 2: 20, 3: 30}
print(len(d))

# Pop a key - returns value and removes key
v1: int = d.pop(2)
print(v1)
print(len(d))

# Verify key was removed
b1: bool = 2 in d
print(b1)

# Pop another existing key
v2: int = d.pop(1)
print(v2)
print(len(d))

# Only key 3 should remain
print(d[3])
