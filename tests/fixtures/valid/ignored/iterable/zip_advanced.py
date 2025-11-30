# Test advanced zip patterns

# Zip multiple iterables
a: list[int] = [1, 2, 3]
b: list[str] = ["a", "b", "c"]
c: list[float] = [1.1, 2.2, 3.3]
d: list[bool] = [True, False, True]

for w, x, y, z in zip(a, b, c, d):
    print(b"Four:", w, x, y, z)

# Zip with unequal lengths (truncates)
short: list[int] = [1, 2]
long: list[int] = [10, 20, 30, 40, 50]
count: int = 0
for s, l in zip(short, long):
    count = count + 1
    print(b"Pair:", s, l)
print(b"Pairs produced:", count)

# Zip strings
for c1, c2, c3 in zip("abc", "def", "ghi"):
    print(b"Chars:", c1, c2, c3)

# Zip to create dict
keys: list[str] = ["name", "age", "city"]
values: list[str] = ["Alice", "30", "NYC"]
result: dict[str, str] = {}
for k, v in zip(keys, values):
    result[k] = v
print(b"Dict from zip:", result)

# Dict comprehension with zip
d2: dict[str, int] = {k: v for k, v in zip(["a", "b", "c"], [1, 2, 3])}
print(b"Dict comp zip:", d2)

# Zip with enumerate
names: list[str] = ["alice", "bob", "charlie"]
scores: list[int] = [95, 87, 92]
for i, (name, score) in enumerate(zip(names, scores)):
    print(b"Index", i, b":", name, score)

# Zip with range
for i, letter in zip(range(100), "abcde"):
    print(b"Range zip:", i, letter)

# Parallel iteration with zip
list1: list[int] = [1, 2, 3, 4, 5]
list2: list[int] = [10, 20, 30, 40, 50]
sums: list[int] = []
for a, b in zip(list1, list2):
    sums.append(a + b)
print(b"Parallel sums:", sums)

# Zip for comparison
old_vals: list[int] = [1, 2, 3, 4, 5]
new_vals: list[int] = [1, 2, 4, 4, 5]
for i, (old, new) in enumerate(zip(old_vals, new_vals)):
    if old != new:
        print(b"Diff at", i, b":", old, b"->", new)

# Zip with reversed
for a, b in zip([1, 2, 3], reversed([10, 20, 30])):
    print(b"With reversed:", a, b)

# Zip to transpose
matrix: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
transposed: list[tuple[int, ...]] = list(zip(*matrix))
print(b"Transposed:", transposed)

# Zip self for pairs
items: list[int] = [1, 2, 3, 4, 5, 6]
for a, b in zip(items[::2], items[1::2]):
    print(b"Paired:", a, b)
