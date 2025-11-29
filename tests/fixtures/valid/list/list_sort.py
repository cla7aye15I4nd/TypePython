# Test list sort method
x: list[int] = [5, 2, 8, 1, 9, 3]
x.sort()
print(x[0])
print(x[1])
print(x[2])
print(x[3])
print(x[4])
print(x[5])

# Already sorted
y: list[int] = [1, 2, 3]
y.sort()
print(y[0])
print(y[1])
print(y[2])

# Reverse sorted
z: list[int] = [5, 4, 3, 2, 1]
z.sort()
print(z[0])
print(z[4])

# Single element
w: list[int] = [42]
w.sort()
print(w[0])

# Empty list
e: list[int] = []
e.sort()
print(len(e))
