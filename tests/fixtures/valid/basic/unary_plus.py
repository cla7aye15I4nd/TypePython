# Test unary plus operator
x: int = 5
y: int = +x
print(y)   # 5

f: float = -3.14
g: float = +f
print(g)   # -3.14

# Multiple unary operators
h: int = +-+-5
print(h)   # 5
