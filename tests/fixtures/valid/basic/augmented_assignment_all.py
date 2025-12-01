# Test all augmented assignment operators
x: int = 10
x += 5
print(x)   # 15

x -= 3
print(x)   # 12

x *= 2
print(x)   # 24

x //= 5
print(x)   # 4

x %= 3
print(x)   # 1

y: int = 2
y **= 3
print(y)   # 8

# Bitwise augmented assignment
z: int = 12
z |= 3
print(z)   # 15

z ^= 5
print(z)   # 10

z &= 7
print(z)   # 2

z <<= 2
print(z)   # 8

z >>= 1
print(z)   # 4

# Float augmented assignment
f: float = 10.0
f /= 4
print(f)   # 2.5
