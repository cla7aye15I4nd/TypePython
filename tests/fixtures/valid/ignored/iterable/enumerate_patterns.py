# Test comprehensive enumerate patterns

# Basic enumerate
items: list[str] = ["a", "b", "c", "d", "e"]
for i, item in enumerate(items):
    print(b"Index:", i, b"Item:", item)

# Enumerate with start
for i, item in enumerate(items, 1):
    print(b"One-indexed:", i, item)

for i, item in enumerate(items, 100):
    print(b"Start 100:", i, item)

for i, item in enumerate(items, -2):
    print(b"Negative start:", i, item)

# Enumerate string
for i, c in enumerate("hello"):
    print(b"Char at", i, c)

# Enumerate bytes
for i, b in enumerate(b"abc"):
    print(b"Byte at", i, b)

# Enumerate range
for i, val in enumerate(range(10, 15)):
    print(b"Range at", i, val)

# Enumerate dict keys
d: dict[str, int] = {"x": 1, "y": 2, "z": 3}
for i, k in enumerate(d):
    print(b"Dict key", i, k)

# Enumerate dict items
for i, (k, v) in enumerate(d.items()):
    print(b"Dict item", i, k, v)

# Enumerate set
s: set[int] = {10, 20, 30}
for i, val in enumerate(s):
    print(b"Set at", i, val)

# Use index for modification
nums: list[int] = [1, 2, 3, 4, 5]
for i, n in enumerate(nums):
    nums[i] = n * n
print(b"Squared:", nums)

# Find index of first match
target: int = 3
found_idx: int = -1
for i, n in enumerate([1, 2, 3, 4, 5]):
    if n == target:
        found_idx = i
        break
print(b"Found at:", found_idx)

# Find all indices
text: str = "banana"
indices: list[int] = []
for i, c in enumerate(text):
    if c == "a":
        indices.append(i)
print(b"All 'a' indices:", indices)

# Enumerate with condition
for i, val in enumerate([10, 20, 30, 40, 50]):
    if i % 2 == 0:
        print(b"Even index:", i, val)

# Build dict from enumerate
items2: list[str] = ["first", "second", "third"]
pos_map: dict[str, int] = {}
for i, item in enumerate(items2, 1):
    pos_map[item] = i
print(b"Position map:", pos_map)

# Enumerate in comprehension
indexed: list[tuple[int, str]] = [(i, v) for i, v in enumerate(["x", "y", "z"])]
print(b"Indexed list:", indexed)

# Enumerate with zip
names: list[str] = ["alice", "bob"]
scores: list[int] = [95, 87]
for i, (name, score) in enumerate(zip(names, scores), 1):
    print(b"Rank", i, name, score)

# Enumerate reversed
for i, val in enumerate(reversed([1, 2, 3, 4, 5])):
    print(b"Rev at", i, val)

# Enumerate generator
def gen() -> int:
    yield 100
    yield 200
    yield 300

for i, val in enumerate(gen()):
    print(b"Gen at", i, val)

# Enumerate nested
matrix: list[list[int]] = [[1, 2], [3, 4], [5, 6]]
for row_idx, row in enumerate(matrix):
    for col_idx, val in enumerate(row):
        print(b"Position:", row_idx, col_idx, b"Value:", val)

# Enumerate with early break
for i, val in enumerate([1, 2, 3, 4, 5]):
    if val > 3:
        print(b"Stopped at index:", i)
        break

# Enumerate with continue
for i, val in enumerate([1, 2, 3, 4, 5]):
    if val % 2 == 0:
        continue
    print(b"Odd at", i, val)
