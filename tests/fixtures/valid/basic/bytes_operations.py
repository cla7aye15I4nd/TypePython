# Test bytes operations

# Concatenation
b1: bytes = b"hello"
b2: bytes = b"world"
print(b1 + b" " + b2)

# Repetition
print(b"ab" * 3)
print(b"x" * 5)
print(b"hi" * True)
print(b"test" * False)

# Comparison
print(b"abc" == b"abc")
print(b"abc" == b"xyz")
print(b"abc" != b"xyz")
print(b"abc" != b"abc")

# Ordering
print(b"apple" < b"banana")
print(b"banana" < b"apple")
print(b"apple" <= b"apple")
print(b"apple" <= b"banana")
print(b"banana" > b"apple")
print(b"apple" > b"banana")
print(b"apple" >= b"apple")
print(b"banana" >= b"apple")

# In operator (byte value membership)
print(104 in b"hello")
print(200 in b"hello")
print(104 not in b"hello")
print(200 not in b"hello")

# In operator (substring membership)
print(b"ell" in b"hello")
print(b"xyz" in b"hello")
print(b"ell" not in b"hello")
print(b"xyz" not in b"hello")

# Indexing
msg: bytes = b"hello"
print(msg[0])
print(msg[1])
print(msg[-1])

# Slicing
print(msg[1:4])
print(msg[:3])
print(msg[2:])
print(msg[::2])
print(msg[::-1])
