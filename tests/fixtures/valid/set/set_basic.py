# Test basic set operations

# Create a simple set
s: set[int] = {1, 2, 3, 4, 5}

# Test len
print(len(s))

# Test contains via 'in' operator
x: int = 3
b1: bool = x in s
print(b1)

# Test not in
y: int = 10
b2: bool = y in s
print(b2)

# Test add (void in TypePython like Python)
s.add(6)
print(len(s))

# Test remove
s.remove(1)
print(len(s))

# Test clear
s.clear()
print(len(s))
