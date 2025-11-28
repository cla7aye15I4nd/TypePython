# Test integer true division (/)
# In Python 3, int / int always returns float

a: int = 10
b: int = 3
result1: float = a / b
print(b"10 / 3 =", result1)

# Exact division
c: int = 12
d: int = 4
result2: float = c / d
print(b"12 / 4 =", result2)

# Larger numbers
e: int = 100
f: int = 7
result3: float = e / f
print(b"100 / 7 =", result3)

# Division with negative
g: int = -15
h: int = 4
result4: float = g / h
print(b"-15 / 4 =", result4)

# Both negative
i: int = -20
j: int = -6
result5: float = i / j
print(b"-20 / -6 =", result5)
