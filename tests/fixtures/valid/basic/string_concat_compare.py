# Test bytes operations (C-style null-terminated strings)
# Note: TypePython uses bytes type for C-style null-terminated strings (char*)

# Bytes concatenation
a: bytes = b"Hello"
b: bytes = b" "
c: bytes = b"World"
result: bytes = a + b + c
print(result)

# Multiple concatenations
greeting: bytes = b"Type" + b"Python" + b" " + b"Compiler"
print(greeting)

# Bytes equality
s1: bytes = b"test"
s2: bytes = b"test"
s3: bytes = b"other"

eq1: bool = s1 == s2
print(b"test == test:", eq1)

eq2: bool = s1 == s3
print(b"test == other:", eq2)

# Bytes inequality
ne1: bool = s1 != s3
print(b"test != other:", ne1)

ne2: bool = s1 != s2
print(b"test != test:", ne2)
