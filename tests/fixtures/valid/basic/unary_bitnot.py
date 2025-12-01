# Test bitwise NOT operator
x: int = 5
y: int = ~x
print(y)   # -6

z: int = 0
w: int = ~z
print(w)   # -1

# Combined with other unary
a: int = -~5
print(a)   # 6 (double negation of sorts)
