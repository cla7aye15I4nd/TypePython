# Test integer operations

# Arithmetic
print(10 + 3)
print(10 - 3)
print(10 * 3)
print(10 / 3)
print(10 // 3)
print(10 % 3)
print(2 ** 10)

# Unary
print(-42)
print(+42)
print(~42)

# Bitwise
print(12 & 10)
print(12 | 10)
print(12 ^ 10)
print(1 << 4)
print(16 >> 2)

# Comparison
print(5 < 10)
print(10 < 5)
print(5 <= 5)
print(5 <= 10)
print(10 > 5)
print(5 > 10)
print(5 >= 5)
print(10 >= 5)
print(5 == 5)
print(5 == 10)
print(5 != 10)
print(5 != 5)

# Int with float
print(5 + 2.5)
print(5 - 2.5)
print(5 * 2.5)
print(5 / 2.5)
print(5 // 2.5)
print(5 % 2.5)
print(2 ** 0.5)

# Int with bool
print(5 + True)
print(5 - True)
print(5 * True)
print(5 / True)
print(5 // True)
print(5 % True)
print(2 ** True)
print(5 & True)
print(5 | True)
print(5 ^ True)
print(5 << True)
print(5 >> True)

# Comparisons with float
print(5 < 5.5)
print(5.5 < 5)
print(5 <= 5.0)
print(5 >= 5.0)

# Comparisons with bool
print(1 < True)
print(1 <= True)
print(1 > True)
print(1 >= True)
print(1 == True)
print(0 == False)

# Identity with same type
print(5 is 5)
print(5 is not 5)
print(5 is not 10)

# Membership
print(3 in [1, 2, 3])
print(5 in [1, 2, 3])
print(3 not in [1, 2, 3])
print(5 not in [1, 2, 3])

# In bytes
print(104 in b"hello")
print(200 in b"hello")

# In set
print(2 in {1, 2, 3})
print(5 in {1, 2, 3})
