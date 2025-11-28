# Test various numeric literal formats

# Binary literals
print(0b1010)
print(0b11111111)
print(0b0)

# Octal literals
print(0o755)
print(0o777)
print(0o10)

# Hexadecimal literals
print(0xFF)
print(0x10)
print(0xDEAD)
print(0xBEEF)

# Arithmetic with different bases
x: int = 0b1010
y: int = 0o12
z: int = 0xA
print(x + y + z)

# Comparisons
print(0b1010 == 10)
print(0o12 == 10)
print(0xA == 10)
