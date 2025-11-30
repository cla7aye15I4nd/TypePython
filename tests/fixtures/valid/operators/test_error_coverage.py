# Test file to increase error handling coverage
# This file tests various operations to improve coverage

# Test set operations
s: set[int] = {1, 2, 3}
if 1 in s:
    print(1)

# Test float comparisons with int and bool
f: float = 3.14
i: int = 3
b: bool = True

# Float comparisons with int
if f > i:
    if f >= i:
        if f != i:
            # Float comparisons with bool
            if f > b:
                print(2)

# Test logical and/or operations with mixed types
# Int and int
x: int = 5
y: int = 0
z = x and y  # Should return 0

# Int and bool
a: int = 10
c: bool = True
d = a and c  # Should return 1 (converted from bool)

# Int or bool
e: int = 0
g: bool = True
h = e or g  # Should return 1

# Int and float
j: int = 5
k: float = 2.5
m = j and k  # Should return 2.5

# Int or float
n: int = 0
p: float = 3.5
q = n or p  # Should return 3.5

print(3)

# Test int operations with bool
r: int = 10
t: bool = True

# Bitwise operations with bool
r1 = r & t  # bitand
r2 = r | t  # bitor
r3 = r ^ t  # bitxor
r4 = r << t  # lshift
r5 = r >> t  # rshift

# Comparisons with bool
if r != t:
    if r > t:
        if r >= t:
            print(4)
