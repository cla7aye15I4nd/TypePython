# Test advanced range() patterns

# Large range
total: int = 0
for i in range(1000):
    total = total + 1
print(b"Count to 1000:", total)

# Range with large step
for i in range(0, 100, 25):
    print(b"By 25:", i)

# Negative range
for i in range(-5, 0):
    print(b"Negative:", i)

# Range from negative to positive
for i in range(-3, 4):
    print(b"Crossing zero:", i)

# Range with negative start, stop, step
for i in range(-1, -10, -2):
    print(b"Neg step:", i)

# Range in boolean context
r: range = range(5)
if r:
    print(b"Non-empty range is truthy")

empty_r: range = range(0)
if not empty_r:
    print(b"Empty range is falsy")

# Range length
print(b"Len range(10):", len(range(10)))
print(b"Len range(5,15):", len(range(5, 15)))
print(b"Len range(0,10,2):", len(range(0, 10, 2)))

# Range membership
r2: range = range(0, 100, 5)
print(b"25 in range:", 25 in r2)
print(b"27 in range:", 27 in r2)

# Range indexing
r3: range = range(10, 20)
print(b"r3[0]:", r3[0])
print(b"r3[5]:", r3[5])
print(b"r3[-1]:", r3[-1])
