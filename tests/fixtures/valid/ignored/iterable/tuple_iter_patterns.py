# Test comprehensive tuple iteration patterns

# Empty tuple
empty: tuple[()] = ()
for x in empty:
    print(b"Never")
print(b"Empty tuple done")

# Single element
single: tuple[int] = (42,)
for x in single:
    print(b"Single:", x)

# Homogeneous tuple
nums: tuple[int, int, int, int, int] = (1, 2, 3, 4, 5)
total: int = 0
for n in nums:
    total = total + n
print(b"Sum:", total)

# Heterogeneous tuple
mixed: tuple[int, str, float, bool] = (42, "hello", 3.14, True)
for item in mixed:
    print(b"Mixed:", item)

# Tuple of tuples
nested: tuple[tuple[int, int], tuple[int, int], tuple[int, int]] = ((1, 2), (3, 4), (5, 6))
for inner in nested:
    for val in inner:
        print(b"Nested:", val)

# Unpacking in iteration
pairs: tuple[tuple[int, int], tuple[int, int], tuple[int, int]] = ((1, 2), (3, 4), (5, 6))
for a, b in pairs:
    print(b"Pair:", a, b)

# Tuple with enumerate
coords: tuple[str, str, str] = ("x", "y", "z")
for i, c in enumerate(coords):
    print(b"Coord", i, c)

# Tuple with zip
t1: tuple[int, int, int] = (1, 2, 3)
t2: tuple[int, int, int] = (10, 20, 30)
for a, b in zip(t1, t2):
    print(b"Zipped:", a, b)

# Reversed tuple
for x in reversed((1, 2, 3, 4, 5)):
    print(b"Reversed:", x)

# Build list from tuple
t: tuple[int, int, int, int] = (10, 20, 30, 40)
as_list: list[int] = []
for x in t:
    as_list.append(x)
print(b"As list:", as_list)

# Tuple membership check
targets: tuple[int, int, int] = (2, 4, 6)
for i in range(10):
    if i in targets:
        print(b"Found:", i)

# Tuple indexing via iteration
data: tuple[str, str, str, str] = ("a", "b", "c", "d")
for i in range(len(data)):
    print(b"Index", i, data[i])

# Tuple comparison
t3: tuple[int, int, int] = (1, 2, 3)
t4: tuple[int, int, int] = (1, 2, 4)
same: bool = True
for i in range(len(t3)):
    if t3[i] != t4[i]:
        same = False
        break
print(b"Same:", same)

# Named tuple-like pattern
Point = tuple[int, int]
points: list[Point] = [(0, 0), (1, 1), (2, 4), (3, 9)]
for x, y in points:
    print(b"Point:", x, y)

# Tuple with None
with_none: tuple[int, None, int, None] = (1, None, 2, None)
for x in with_none:
    if x is not None:
        print(b"Not none:", x)

# Sliding window with tuples
data2: tuple[int, int, int, int, int] = (1, 2, 3, 4, 5)
for i in range(len(data2) - 1):
    print(b"Adjacent:", data2[i], data2[i + 1])

# Tuple unpacking with *
def process(*args: int) -> int:
    total: int = 0
    for arg in args:
        total = total + arg
    return total

result: int = process(1, 2, 3, 4, 5)
print(b"Sum via *args:", result)

# Tuple from generator
gen_tuple: tuple[int, ...] = tuple(x * x for x in range(5))
for x in gen_tuple:
    print(b"Gen tuple:", x)
