# Test IndexError exception
x: int = 0

try:
    raise IndexError
except IndexError:
    print(b"caught IndexError")
    x = 99

print(x)
