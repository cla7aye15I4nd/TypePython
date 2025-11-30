# Test raise and catch
x: int = 0

try:
    print(b"before raise")
    raise ValueError
    print(b"after raise - should not print")
except ValueError:
    print(b"caught ValueError")
    x = 1
finally:
    print(b"finally block")

print(x)
