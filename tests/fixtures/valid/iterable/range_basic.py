# Test basic range() iteration

# range with single argument (stop)
total: int = 0
for i in range(5):
    total = total + i
print(b"Sum 0-4:", total)

# range with start and stop
total = 0
for i in range(2, 6):
    total = total + i
print(b"Sum 2-5:", total)

# range with start, stop, step
total = 0
for i in range(0, 10, 2):
    total = total + i
print(b"Sum evens 0-8:", total)

# range with negative step
for i in range(5, 0, -1):
    print(b"Countdown:", i)

# Empty range
count: int = 0
for i in range(0):
    count = count + 1
print(b"Empty range iterations:", count)

# Nested range loops
for i in range(3):
    for j in range(3):
        print(b"i,j:", i, j)
