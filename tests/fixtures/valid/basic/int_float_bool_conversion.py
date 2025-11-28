# Test int and float conversion to bool in conditionals

# Int as condition - non-zero is true
x: int = 5
if x:
    print(b"int 5 is truthy")
else:
    print(b"int 5 is falsy")

# Int zero as condition
y: int = 0
if y:
    print(b"int 0 is truthy")
else:
    print(b"int 0 is falsy")

# Float as condition - non-zero is true
f1: float = 3.14
if f1:
    print(b"float 3.14 is truthy")
else:
    print(b"float 3.14 is falsy")

# Float zero as condition
f2: float = 0.0
if f2:
    print(b"float 0.0 is truthy")
else:
    print(b"float 0.0 is falsy")

# Negative values are also truthy
neg: int = -10
if neg:
    print(b"int -10 is truthy")
else:
    print(b"int -10 is falsy")

# Using int in while condition
counter: int = 3
while counter:
    print(counter)
    counter = counter - 1
