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

# Update with empty dict (should not change)
d4: dict[int, int] = {7: 70}
empty: dict[int, int] = {}
d4.update(empty)
print(len(d4))
print(d4[7])

# Update empty dict with non-empty
empty2: dict[int, int] = {}
d5: dict[int, int] = {8: 80, 9: 90}
empty2.update(d5)
print(len(empty2))
print(empty2[8])
print(empty2[9])

# Update multiple times
d6: dict[int, int] = {10: 100}
d7: dict[int, int] = {20: 200}
d6.update(d7)
print(len(d6))
print(d6[10])
print(d6[20])

# Update again with more keys
d8: dict[int, int] = {30: 300, 40: 400}
d6.update(d8)
print(len(d6))
print(d6[30])
print(d6[40])
