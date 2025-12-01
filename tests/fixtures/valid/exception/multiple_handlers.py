# Test multiple exception handlers
x: int = 0

try:
    print(b"raising TypeError")
    raise TypeError
except ValueError:
    print(b"caught ValueError")
    x = 1
except TypeError:
    print(b"caught TypeError")
    x = 2
except RuntimeError:
    print(b"caught RuntimeError")
    x = 3
finally:
    print(b"finally")

print(x)
