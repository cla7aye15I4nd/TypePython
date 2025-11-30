# Test bool operations

# Arithmetic with bool
print(True + True)
print(True - False)
print(True * True)
print(True / True)
print(True // True)
print(True % True)
print(True ** True)

# Bitwise with bool
print(True & True)
print(True & False)
print(True | False)
print(True | True)
print(True ^ True)
print(True ^ False)
print(True << True)
print(True >> True)

# Unary
print(-True)
print(+True)
print(~True)
print(-False)
print(+False)
print(~False)

# Comparison with bool
print(True < True)
print(True <= True)
print(True > False)
print(True >= True)
print(True == True)
print(True == False)
print(True != False)
print(True != True)

# Bool with int
print(True + 5)
print(True - 5)
print(True * 5)
print(True / 5)
print(True // 5)
print(True % 5)
print(True ** 5)
print(True & 5)
print(True | 5)
print(True ^ 5)
print(True << 2)
print(True >> 1)

# Bool with float
print(True + 2.5)
print(True - 2.5)
print(True * 2.5)
print(True / 2.5)
print(True // 2.5)
print(True % 2.5)
print(True ** 2.5)

# Comparisons with int
print(True < 2)
print(True <= 1)
print(True > 0)
print(True >= 1)
print(True == 1)
print(False == 0)

# Comparisons with float
print(True < 1.5)
print(True <= 1.0)
print(True > 0.5)
print(True >= 1.0)

# Identity
print(True is True)
print(True is not True)
print(True is not False)

# String multiplication
print(True * "hi")
print(False * "hi")

# Bytes multiplication
print(True * b"hi")
print(False * b"hi")

# List multiplication
print(True * [1, 2])
print(False * [1, 2])

# Membership
print(True in [1, 2, 3])
print(False in [0, 1, 2])
print(True not in [0, 2, 3])

# In bytes
print(True in b"hello")
print(False in b"hello")

# In set
print(True in {0, 1, 2})
print(False in {0, 1, 2})
