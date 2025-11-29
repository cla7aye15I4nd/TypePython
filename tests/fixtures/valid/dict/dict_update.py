# Test dict update method

d1: dict[int, int] = {1: 10, 2: 20}
d2: dict[int, int] = {3: 30, 4: 40}

print(len(d1))

# Update d1 with d2
d1.update(d2)

print(len(d1))
print(d1[1])
print(d1[2])
print(d1[3])
print(d1[4])

# Update with overlapping keys (right takes precedence)
d3: dict[int, int] = {1: 100, 5: 50}
d1.update(d3)

print(len(d1))
print(d1[1])
print(d1[5])
