# Test bytes comparison operations

# Bytes equality
result1: bool = b"hello" == b"hello"
print(result1)

result2: bool = b"hello" == b"world"
print(result2)

# Bytes inequality
result3: bool = b"hello" != b"world"
print(result3)

result4: bool = b"hello" != b"hello"
print(result4)

# Bytes ordering
result5: bool = b"abc" < b"abd"
print(result5)

result6: bool = b"abc" < b"abc"
print(result6)

result7: bool = b"abc" <= b"abc"
print(result7)

result8: bool = b"abd" > b"abc"
print(result8)

result9: bool = b"abc" >= b"abc"
print(result9)

# Bytes membership
result10: bool = b"el" in b"hello"
print(result10)

result11: bool = b"xy" in b"hello"
print(result11)

result12: bool = b"xy" not in b"hello"
print(result12)

result13: bool = b"el" not in b"hello"
print(result13)

# Empty bytes
result14: bool = b"" == b""
print(result14)

result15: bool = b"" < b"a"
print(result15)
