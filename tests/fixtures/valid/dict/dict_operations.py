# Test dict operations (merge, equality)

a: dict[int, int] = {1: 10, 2: 20}
b: dict[int, int] = {3: 30, 4: 40}

# Dict merge with |
c: dict[int, int] = a | b
print(len(c))
print(c[1])
print(c[3])

# Merge with overlapping keys (right takes precedence)
d: dict[int, int] = {1: 100, 5: 50}
e: dict[int, int] = a | d
print(len(e))
print(e[1])
print(e[5])

# Equality
f: dict[int, int] = {1: 10, 2: 20}
b1: bool = a == f
print(b1)

g: dict[int, int] = {1: 10, 2: 99}
b2: bool = a == g
print(b2)

# Inequality
b3: bool = a != g
print(b3)
