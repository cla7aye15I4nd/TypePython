# Bytes formatting with % operator
b1: bytes = b"Value: %d"
result1: bytes = b1 % 42
print(result1)

b2: bytes = b"Pi is %f"
result2: bytes = b2 % 3.14
print(result2)

b3: bytes = b"Flag: %s"
result3: bytes = b3 % True
print(result3)

b4: bytes = b"Name: %s"
result4: bytes = b4 % "Alice"
print(result4)
