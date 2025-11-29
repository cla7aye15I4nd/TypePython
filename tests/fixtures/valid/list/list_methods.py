# Test list method calls (TypePython specific - methods return the list)

# Test len
x: list[int] = [1, 2, 3, 4, 5]
print(len(x))

# Test append returns modified list
x = x.append(6)
print(len(x))
print(x[5])

# Test pop
y: int = x.pop()
print(y)
print(len(x))

# Test insert
x = x.insert(0, 99)
print(x[0])

# Test index
idx: int = x.index(3)
print(idx)

# Test count
z: list[int] = [1, 1, 2, 1, 3]
c: int = z.count(1)
print(c)

# Test copy
z2: list[int] = z.copy()
print(len(z2))

# Test reverse (in place)
z.reverse()
print(z[0])

# Test clear
z.clear()
print(len(z))
