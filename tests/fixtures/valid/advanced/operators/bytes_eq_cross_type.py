# Test bytes equality operators with different types

# Bytes == int (should always be False)
result1: bool = b"test" == 5
print(result1)

result2: bool = b"" == 0
print(result2)

result3: bool = b"hello" == 123
print(result3)

# Bytes != int (should always be True)
result4: bool = b"test" != 5
print(result4)

result5: bool = b"" != 0
print(result5)

result6: bool = b"hello" != 123
print(result6)

# Bytes == bool (should always be False)
result7: bool = b"" == False
print(result7)

result8: bool = b"x" == True
print(result8)

# Bytes != bool (should always be True)
result9: bool = b"" != False
print(result9)

result10: bool = b"x" != True
print(result10)

# Bytes == Bytes (normal comparison)
result11: bool = b"hello" == b"hello"
print(result11)

result12: bool = b"hello" == b"world"
print(result12)

# Bytes != Bytes (normal comparison)
result13: bool = b"hello" != b"hello"
print(result13)

result14: bool = b"hello" != b"world"
print(result14)
