# Test unary not operator on integers
# This tests IntType.unary_op with UnaryOp::Not

print(b"Int unary not (bitwise):")
i1: int = 0
r1: int = ~i1  # bitwise not of 0 = -1
print(r1)

i2: int = 1
r2: int = ~i2  # bitwise not of 1 = -2
print(r2)

i3: int = -1
r3: int = ~i3  # bitwise not of -1 = 0
print(r3)

i4: int = 255
r4: int = ~i4  # bitwise not of 255 = -256
print(r4)

# Double bitwise not should return original
i5: int = 42
r5: int = ~~i5  # double bitwise not = original
print(r5)
