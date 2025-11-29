# Test logical and/or operators with floats

# and operator - returns first falsy or last value
x1: float = 1.0 and 2.0
print(x1)  # 2.0

x2: float = 0.0 and 2.0
print(x2)  # 0.0

x3: float = 1.0 and 0.0
print(x3)  # 0.0

# or operator - returns first truthy or last value
y1: float = 1.0 or 2.0
print(y1)  # 1.0

y2: float = 0.0 or 2.0
print(y2)  # 2.0

y3: float = 0.0 or 0.0
print(y3)  # 0.0

# Mixed with int
z1: int = 1.5 and 3
print(z1)  # 3

z2: float = 0 or 2.5
print(z2)  # 2.5

# Chained operations
c1: float = 1.0 and 2.0 and 3.0
print(c1)  # 3.0

c2: float = 0.0 or 0.0 or 5.0
print(c2)  # 5.0
