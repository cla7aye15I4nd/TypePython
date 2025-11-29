# Test dict len() and 'in' operator

# Empty dict
empty: dict[int, int] = {}
print(len(empty))

# Single element
d1: dict[int, int] = {1: 10}
print(len(d1))

# Multiple elements
d2: dict[int, int] = {1: 10, 2: 20, 3: 30, 4: 40, 5: 50}
print(len(d2))

# After adding elements
d3: dict[int, int] = {1: 100}
print(len(d3))
d3[2] = 200
print(len(d3))
d3[3] = 300
print(len(d3))

# After updating existing key (len should not change)
d3[2] = 999
print(len(d3))

# Test 'in' operator - key exists
b1: bool = 1 in d2
print(b1)

b2: bool = 3 in d2
print(b2)

b3: bool = 5 in d2
print(b3)

# Test 'in' operator - key does not exist
b4: bool = 10 in d2
print(b4)

b5: bool = 0 in d2
print(b5)

# Test 'not in' operator
b6: bool = 10 not in d2
print(b6)

b7: bool = 1 not in d2
print(b7)

# Test with larger dict
large_d: dict[int, int] = {100: 1, 200: 2, 300: 3, 400: 4, 500: 5}
print(len(large_d))

b8: bool = 100 in large_d
print(b8)

b9: bool = 999 in large_d
print(b9)

b10: bool = 999 not in large_d
print(b10)

# Test after removal
large_d2: dict[int, int] = {10: 10, 20: 20, 30: 30}
print(len(large_d2))
b11: bool = 20 in large_d2
print(b11)

removed2: int = large_d2.pop(20)
print(len(large_d2))
b12: bool = 20 in large_d2
print(b12)

# Test with empty dict
empty2: dict[int, int] = {}
b13: bool = 100 in empty2
print(b13)

b14: bool = 100 not in empty2
print(b14)

# Test len after clear
d4: dict[int, int] = {1: 1, 2: 2, 3: 3}
print(len(d4))
d4.clear()
print(len(d4))

# Test 'in' after clear
b15: bool = 1 in d4
print(b15)

# Test with update
d5: dict[int, int] = {1: 10}
d6: dict[int, int] = {2: 20, 3: 30}
print(len(d5))
d5.update(d6)
print(len(d5))
b16: bool = 2 in d5
print(b16)
b17: bool = 3 in d5
print(b17)

# Test 'in' checks value not found in keys
d7: dict[int, int] = {1: 100, 2: 200, 3: 300}
print(len(d7))
b18: bool = 1 in d7
print(b18)
b19: bool = 100 in d7
print(b19)
b20: bool = 99 not in d7
print(b20)
