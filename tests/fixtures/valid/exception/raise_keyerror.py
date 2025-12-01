# Test KeyError exception
x: int = 0

try:
    raise KeyError
except KeyError:
    print(b"caught KeyError")
    x = 42

print(x)
