# Test list operations (concat, repeat, equality)

a: list[int] = [1, 2, 3]
b: list[int] = [4, 5, 6]

# Concatenation
c: list[int] = a + b
print(len(c))
print(c[0])
print(c[3])
print(c[5])

# Repeat
d: list[int] = a * 3
print(len(d))
print(d[0])
print(d[3])
print(d[6])

# Equality
e: list[int] = [1, 2, 3]
b1: bool = a == e
print(b1)

f: list[int] = [1, 2, 4]
b2: bool = a == f
print(b2)

# Inequality
b3: bool = a != f
print(b3)

# Membership
b4: bool = 2 in a
print(b4)

b5: bool = 10 in a
print(b5)

b6: bool = 10 not in a
print(b6)
