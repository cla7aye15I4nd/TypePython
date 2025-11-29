# Invalid: list() constructor cannot be called with direct arguments
# Should use list literals like [1, 2, 3] instead

x: list[int] = list(1, 2, 3)
print(x)
