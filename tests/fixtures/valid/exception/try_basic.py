# Basic try/except test
x: int = 10

try:
    y: int = x + 5
    print(y)
except ValueError:
    print(b"caught error")
finally:
    print(b"cleanup")
