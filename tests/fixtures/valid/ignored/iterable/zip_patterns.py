# Test comprehensive zip patterns

# Basic zip
a: list[int] = [1, 2, 3]
b: list[str] = ["a", "b", "c"]
for x, y in zip(a, b):
    print(b"Pair:", x, y)

# Zip three lists
c: list[float] = [1.1, 2.2, 3.3]
for x, y, z in zip(a, b, c):
    print(b"Triple:", x, y, z)

# Zip four lists
d: list[bool] = [True, False, True]
for w, x, y, z in zip(a, b, c, d):
    print(b"Quad:", w, x, y, z)

# Unequal lengths (truncates)
short: list[int] = [1, 2]
long: list[int] = [10, 20, 30, 40]
count: int = 0
for s, l in zip(short, long):
    count = count + 1
    print(b"Unequal:", s, l)
print(b"Pairs:", count)

# Zip strings
for c1, c2 in zip("abc", "xyz"):
    print(b"Chars:", c1, c2)

# Zip string and list
for c, n in zip("abc", [1, 2, 3]):
    print(b"Mixed:", c, n)

# Build dict from zip
keys: list[str] = ["name", "age", "city"]
values: list[str] = ["Alice", "30", "NYC"]
result: dict[str, str] = {}
for k, v in zip(keys, values):
    result[k] = v
print(b"Dict:", result)

# Dict comprehension with zip
d2: dict[str, int] = {k: v for k, v in zip(["a", "b", "c"], [1, 2, 3])}
print(b"Dict comp:", d2)

# Zip with enumerate
names: list[str] = ["alice", "bob", "charlie"]
scores: list[int] = [95, 87, 92]
for i, (name, score) in enumerate(zip(names, scores)):
    print(b"Enum zip:", i, name, score)

# Parallel sum
list1: list[int] = [1, 2, 3, 4, 5]
list2: list[int] = [10, 20, 30, 40, 50]
sums: list[int] = []
for x, y in zip(list1, list2):
    sums.append(x + y)
print(b"Sums:", sums)

# Element-wise operations
products: list[int] = []
for x, y in zip(list1, list2):
    products.append(x * y)
print(b"Products:", products)

# Difference detection
old: list[int] = [1, 2, 3, 4, 5]
new: list[int] = [1, 2, 4, 4, 5]
for i, (o, n) in enumerate(zip(old, new)):
    if o != n:
        print(b"Diff at", i, o, n)

# Zip with reversed
for x, y in zip([1, 2, 3], reversed([10, 20, 30])):
    print(b"With reversed:", x, y)

# Zip with range
for i, c in zip(range(100), "abcde"):
    print(b"Range zip:", i, c)

# Transpose via zip
matrix: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
transposed: list[tuple[int, ...]] = list(zip(*matrix))
print(b"Transposed:", transposed)

# Pair adjacent elements
items: list[int] = [1, 2, 3, 4, 5, 6]
for x, y in zip(items[::2], items[1::2]):
    print(b"Adjacent:", x, y)

# Pairwise iteration
for x, y in zip(items[:-1], items[1:]):
    print(b"Pairwise:", x, y)

# Zip with generators
def gen1() -> int:
    yield 1
    yield 2
    yield 3

def gen2() -> int:
    yield 10
    yield 20
    yield 30

for x, y in zip(gen1(), gen2()):
    print(b"Gen zip:", x, y)

# Zip empty
for x, y in zip([], [1, 2, 3]):
    print(b"Never")
print(b"Empty zip done")

# Zip with sets (order not guaranteed)
s1: set[int] = {1, 2, 3}
s2: set[int] = {10, 20, 30}
for x, y in zip(s1, s2):
    print(b"Set zip:", x, y)

# Zip with tuples
t1: tuple[int, int, int] = (1, 2, 3)
t2: tuple[int, int, int] = (10, 20, 30)
for x, y in zip(t1, t2):
    print(b"Tuple zip:", x, y)

# Complex unpacking
records: list[tuple[str, int, float]] = [("alice", 30, 1.65), ("bob", 25, 1.80)]
heights: list[float] = [1.70, 1.75]
for (name, age, h1), h2 in zip(records, heights):
    print(b"Record:", name, age, h1, h2)
