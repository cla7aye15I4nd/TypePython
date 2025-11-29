# Test set operations (union, intersection, difference, etc.)

a: set[int] = {1, 2, 3, 4, 5}
b: set[int] = {3, 4, 5, 6, 7}

# Union
c: set[int] = a | b
print(len(c))

# Intersection  
d: set[int] = a & b
print(len(d))

# Difference
e: set[int] = a - b
print(len(e))

# Symmetric difference
f: set[int] = a ^ b
print(len(f))

# Subset
s1: set[int] = {1, 2}
b1: bool = s1 < a
print(b1)

b2: bool = s1 <= a
print(b2)

# Not subset
b3: bool = a < s1
print(b3)

# Superset
b4: bool = a > s1
print(b4)

b5: bool = a >= s1
print(b5)

# Equality
g: set[int] = {1, 2, 3, 4, 5}
b6: bool = a == g
print(b6)

h: set[int] = {1, 2, 3}
b7: bool = a == h
print(b7)

# Inequality
b8: bool = a != h
print(b8)
