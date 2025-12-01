# Test shift operators
x: int = 4
y: int = x << 2
print(y)   # 16

z: int = 16
w: int = z >> 2
print(w)   # 4

# Chained shifts
a: int = 1
b: int = a << 3 << 2  # (1 << 3) << 2 = 8 << 2 = 32
print(b)

# Combined with other ops
c: int = 2 + (3 << 1)
print(c)   # 2 + 6 = 8
