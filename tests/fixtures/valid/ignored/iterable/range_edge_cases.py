# Test range edge cases

# Empty ranges
for i in range(0):
    print(b"Never prints")

for i in range(5, 5):
    print(b"Never prints")

for i in range(5, 0):
    print(b"Never prints")

for i in range(0, 10, -1):
    print(b"Never prints")

# Single element ranges
for i in range(1):
    print(b"Single:", i)

for i in range(5, 6):
    print(b"Single:", i)

# Large step
for i in range(0, 100, 50):
    print(b"Large step:", i)

# Step larger than range
for i in range(0, 5, 10):
    print(b"Step > range:", i)

# Negative step edge cases
for i in range(0, -1, -1):
    print(b"Neg single:", i)

for i in range(10, 0, -3):
    print(b"Neg step 3:", i)

# Boolean in range
for i in range(True, 5):
    print(b"Bool start:", i)

for i in range(0, True):
    print(b"Bool stop:", i)

for i in range(0, 10, True):
    print(b"Bool step:", i)

# Range operations
r1: range = range(10)
print(b"len(range(10)):", len(r1))
print(b"5 in range(10):", 5 in r1)
print(b"10 in range(10):", 10 in r1)
print(b"range(10)[5]:", r1[5])
print(b"range(10)[-1]:", r1[-1])

r2: range = range(2, 20, 3)
print(b"len(range(2,20,3)):", len(r2))
print(b"8 in range(2,20,3):", 8 in r2)
print(b"9 in range(2,20,3):", 9 in r2)

# Range equality
print(b"range(10) == range(0,10):", range(10) == range(0, 10))
print(b"range(10) == range(0,10,1):", range(10) == range(0, 10, 1))
print(b"range(0) == range(5,5):", range(0) == range(5, 5))
