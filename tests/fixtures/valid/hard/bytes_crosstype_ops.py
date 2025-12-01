# TypePython-specific cross-type operations for bytes
# These don't work in Python3 but are supported in TypePython

a: bytes = b"hello"

# Cross-type equality with non-bytes (always false)
print(a == 42)       # False (bytes vs int)
print(a != 42)       # True

# Cross-type membership with containers
ints: list[int] = [1, 2, 3]
print(b"hi" in ints)     # False
print(b"hi" not in ints) # True

# Identity cross-type
print(a is 42)       # False
print(a is not 42)   # True
