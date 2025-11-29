# Test bitwise operations on integers

# Bitwise AND
result1: int = 12 & 10  # 1100 & 1010 = 1000 = 8
print(result1)

# Bitwise OR
result2: int = 12 | 10  # 1100 | 1010 = 1110 = 14
print(result2)

# Bitwise XOR
result3: int = 12 ^ 10  # 1100 ^ 1010 = 0110 = 6
print(result3)

# Left shift
result4: int = 5 << 2  # 101 << 2 = 10100 = 20
print(result4)

# Right shift (positive number)
result5: int = 20 >> 2  # 10100 >> 2 = 101 = 5
print(result5)

# Right shift (negative number - arithmetic shift)
result6: int = -8 >> 1  # Should preserve sign bit
print(result6)

# Bitwise NOT (unary)
result7: int = ~5  # -(5 + 1) = -6
print(result7)

# Complex expression
result8: int = (12 & 10) | (5 << 1)
print(result8)

# Bitwise with zero
result9: int = 42 & 0
print(result9)

result10: int = 42 | 0
print(result10)
