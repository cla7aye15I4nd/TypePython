# Test boolean operations with other types
# Tests Bool op Int and Bool op Float coercion paths

# Bool + Int should coerce bool to int
print(b"Bool + Int:")
b1: bool = True
i1: int = 5
r1: int = b1 + i1  # True (1) + 5 = 6
print(r1)

b2: bool = False
r2: int = b2 + i1  # False (0) + 5 = 5
print(r2)

# Bool * Int
print(b"Bool * Int:")
r3: int = b1 * i1  # True (1) * 5 = 5
print(r3)
r4: int = b2 * i1  # False (0) * 5 = 0
print(r4)

# Bool - Int
print(b"Bool - Int:")
r5: int = b1 - 3  # True (1) - 3 = -2
print(r5)

# Bool + Float should coerce bool to float
print(b"Bool + Float:")
f1: float = 2.5
r6: float = b1 + f1  # True (1.0) + 2.5 = 3.5
print(r6)
r7: float = b2 + f1  # False (0.0) + 2.5 = 2.5
print(r7)

# Bool * Float
print(b"Bool * Float:")
r8: float = b1 * f1  # True (1.0) * 2.5 = 2.5
print(r8)
r9: float = b2 * f1  # False (0.0) * 2.5 = 0.0
print(r9)

# Bool / Float
print(b"Bool / Float:")
r10: float = b1 / f1  # True (1.0) / 2.5 = 0.4
print(r10)
