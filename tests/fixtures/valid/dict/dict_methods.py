# Test dict methods

d: dict[int, int] = {1: 10, 2: 20, 3: 30}

# Test len
print(len(d))

# Test contains (key in dict)
b1: bool = 2 in d
print(b1)

b2: bool = 5 in d
print(b2)

# Test not in
b3: bool = 5 not in d
print(b3)

# Test copy
d2: dict[int, int] = d.copy()
print(len(d2))
print(d2[1])

# Test clear
d.clear()
print(len(d))

# Original copy should be intact
print(len(d2))

# Test get with default
d3: dict[int, int] = {1: 100, 2: 200}
v1: int = d3.get(1, 0)
print(v1)

v2: int = d3.get(99, -1)
print(v2)

# Test setdefault - returns existing value if key exists
d4: dict[int, int] = {1: 100, 2: 200}
v3: int = d4.setdefault(1, 999)
print(v3)
print(len(d4))

# Test setdefault - sets and returns default if key doesn't exist
v4: int = d4.setdefault(3, 300)
print(v4)
print(len(d4))
print(d4[3])
