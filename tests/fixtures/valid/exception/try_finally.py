# Simple try-finally pattern
x: int = 0

try:
    x = 10
    print(x)
finally:
    x = 0
    print(x)
