# Test int operations with bool (bool is treated as 0/1)

# Addition
x1: int = 5 + True
print(x1)  # 6
x2: int = 5 + False
print(x2)  # 5

# Subtraction
y1: int = 5 - True
print(y1)  # 4
y2: int = 5 - False
print(y2)  # 5

# Multiplication
z1: int = 5 * True
print(z1)  # 5
z2: int = 5 * False
print(z2)  # 0

# Division
d1: float = 5 / True
print(d1)  # 5.0

# Floor division
f1: int = 5 // True
print(f1)  # 5

# Modulo
m1: int = 5 % True
print(m1)  # 0

# Power
p1: int = 2 ** True
print(p1)  # 2
p2: int = 2 ** False
print(p2)  # 1
