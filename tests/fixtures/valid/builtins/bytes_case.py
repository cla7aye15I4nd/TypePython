# Test bytes case conversion methods: upper(), lower()

# Basic upper/lower
print(b"hello".upper())
print(b"HELLO".lower())
print(b"MiXeD".upper())
print(b"MiXeD".lower())

# Empty bytes
print(b"".upper())
print(b"".lower())

# Only uppercase
print(b"ABC".upper())
print(b"ABC".lower())

# Only lowercase
print(b"xyz".upper())
print(b"xyz".lower())

# With numbers and special chars
print(b"hello123".upper())
print(b"WORLD456".lower())
print(b"test!@#".upper())
print(b"TEST!@#".lower())

# Mixed case
print(b"HeLLo WoRLd".upper())
print(b"HeLLo WoRLd".lower())

# Single character
print(b"a".upper())
print(b"Z".lower())

# With spaces
print(b"  hello  ".upper())
print(b"  WORLD  ".lower())

# Chained operations
print(b"test".upper().lower())
print(b"TEST".lower().upper())

# Long strings
print(b"the quick brown fox".upper())
print(b"THE QUICK BROWN FOX".lower())
