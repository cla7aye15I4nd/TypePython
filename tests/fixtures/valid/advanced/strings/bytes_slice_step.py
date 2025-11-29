# Test bytes slicing with step parameter
s: bytes = b"Hello World"
print(s[::2])
print(s[1::2])
print(s[::-1])
print(s[::3])
print(s[1:8:2])
