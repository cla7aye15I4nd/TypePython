# Test floor division operator (//)
# Integer floor division
a: int = 10
b: int = 3
result1: int = a // b
print(b"10 // 3 =", result1)

# Float floor division
x: float = 10.0
y: float = 3.0
result2: float = x // y
print(b"10.0 // 3.0 =", result2)

# Negative floor division (float)
x2: float = -10.0
y2: float = 3.0
result3: float = x2 // y2
print(b"-10.0 // 3.0 =", result3)

# Mixed types (should convert to float)
m: int = 7
n: float = 2.0
result4: float = m // n
print(b"7 // 2.0 =", result4)
