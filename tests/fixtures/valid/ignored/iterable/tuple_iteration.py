# Test tuple iteration patterns

# Basic tuple iteration
t: tuple[int, int, int, int, int] = (1, 2, 3, 4, 5)
for val in t:
    print(b"Tuple val:", val)

# Tuple of strings
names: tuple[str, str, str] = ("alice", "bob", "charlie")
for name in names:
    print(b"Name:", name)

# Mixed type tuple
mixed: tuple[int, str, float, bool] = (42, "hello", 3.14, True)
for item in mixed:
    print(b"Mixed item:", item)

# Tuple in enumerate
coords: tuple[int, int, int] = (10, 20, 30)
for i, val in enumerate(coords):
    print(b"Index:", i, b"Coord:", val)

# Tuple in zip
x_vals: tuple[int, int, int] = (1, 2, 3)
y_vals: tuple[int, int, int] = (4, 5, 6)
for x, y in zip(x_vals, y_vals):
    print(b"Point:", x, y)

# Reversed tuple
for val in reversed((1, 2, 3, 4, 5)):
    print(b"Reversed:", val)

# Tuple unpacking in iteration
points: list[tuple[int, int]] = [(0, 0), (1, 1), (2, 4), (3, 9)]
for x, y in points:
    print(b"x:", x, b"y:", y)

# Nested tuples
nested: tuple[tuple[int, int], tuple[int, int]] = ((1, 2), (3, 4))
for pair in nested:
    for val in pair:
        print(b"Nested val:", val)

# Tuple comparison during iteration
pairs: list[tuple[int, int]] = [(1, 2), (2, 1), (3, 3), (4, 5)]
for a, b in pairs:
    if a < b:
        print(b"a < b:", a, b)
    elif a > b:
        print(b"a > b:", a, b)
    else:
        print(b"a == b:", a, b)

# Build tuple from iteration
result: tuple[int, ...] = tuple(x * x for x in range(5))
print(b"Squared tuple:", result)

# Tuple membership during iteration
targets: tuple[int, int, int] = (2, 4, 6)
for i in range(10):
    if i in targets:
        print(b"Found target:", i)
