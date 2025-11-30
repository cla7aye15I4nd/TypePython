# Test bound method extraction and calling
# Expected output:
# b'hello     '
# WORLD

# Test bytes method binding
f = b"hello".ljust
result: bytes = f(10)
print(result)

# Test str method binding
s: str = "world"
g = s.upper
result2: str = g()
print(result2)
