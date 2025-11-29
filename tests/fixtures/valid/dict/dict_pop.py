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

# Pop another dict
d2: dict[int, int] = {5: 50, 6: 60, 7: 70}
v3: int = d2.pop(5)
print(v3)
print(len(d2))

# Pop another key
v4: int = d2.pop(6)
print(v4)
print(len(d2))

# Verify remaining key exists
print(d2[7])

# Pop multiple items
d3: dict[int, int] = {10: 100, 20: 200, 30: 300}
v5: int = d3.pop(20)
print(v5)
print(len(d3))

# Verify key was removed
b2: bool = 20 in d3
print(b2)

# Check remaining keys exist
b3: bool = 10 in d3
print(b3)
b4: bool = 30 in d3
print(b4)

# Pop remaining keys
v7: int = d3.pop(10)
print(v7)
v8: int = d3.pop(30)
print(v8)
print(len(d3))
