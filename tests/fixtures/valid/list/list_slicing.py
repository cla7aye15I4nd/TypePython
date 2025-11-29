# Test list slicing operations

x: list[int] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

# Basic slicing
y: list[int] = x[2:5]
print(len(y))
print(y[0])
print(y[2])

# Slice from start
z: list[int] = x[:3]
print(len(z))
print(z[0])

# Slice to end
w: list[int] = x[7:]
print(len(w))
print(w[0])

# Negative indices in slice
v: list[int] = x[-3:]
print(len(v))
print(v[0])

# Slice with step
s: list[int] = x[::2]
print(len(s))
print(s[0])
print(s[1])
print(s[2])

# Reverse with step
r: list[int] = x[::-1]
print(len(r))
print(r[0])
print(r[9])
