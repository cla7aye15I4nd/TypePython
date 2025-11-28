# Test bitwise operators on integers
a: int = 12
b: int = 10

# Bitwise OR
or_result: int = a | b
print(b"12 | 10 =", or_result)

# Bitwise AND
and_result: int = a & b
print(b"12 & 10 =", and_result)

# Bitwise XOR
xor_result: int = a ^ b
print(b"12 ^ 10 =", xor_result)

# Left shift
lshift_result: int = a << 2
print(b"12 << 2 =", lshift_result)

# Right shift
rshift_result: int = a >> 2
print(b"12 >> 2 =", rshift_result)

# Bitwise NOT
not_result: int = ~a
print(b"~12 =", not_result)

# Combined bitwise operations
c: int = 255
d: int = 15
combined: int = (c & d) | (a << 4)
print(b"(255 & 15) | (12 << 4) =", combined)

# Shift with larger values
e: int = 1
big_shift: int = e << 10
print(b"1 << 10 =", big_shift)
