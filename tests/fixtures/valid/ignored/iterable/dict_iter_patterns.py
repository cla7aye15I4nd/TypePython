# Test various dict iteration patterns

# Empty dict
empty: dict[str, int] = {}
for k in empty:
    print(b"Never")
print(b"Empty dict done")

# Single entry
single: dict[str, int] = {"one": 1}
for k in single:
    print(b"Single key:", k)

# Large dict
large: dict[int, int] = {}
for i in range(50):
    large[i] = i * i
total: int = 0
for v in large.values():
    total = total + v
print(b"Sum of squares:", total)

# String keys
names: dict[str, int] = {"alice": 30, "bob": 25, "charlie": 35}
for name in names:
    print(b"Name:", name)

for name in names.keys():
    print(b"Key:", name)

for age in names.values():
    print(b"Age:", age)

for name, age in names.items():
    print(b"Entry:", name, age)

# Int keys
scores: dict[int, str] = {1: "first", 2: "second", 3: "third"}
for pos in scores:
    print(b"Position:", pos)

for pos, label in scores.items():
    print(b"Pos-Label:", pos, label)

# Tuple keys
coords: dict[tuple[int, int], str] = {(0, 0): "origin", (1, 0): "right", (0, 1): "up"}
for coord in coords:
    print(b"Coord:", coord)

for (x, y), name in coords.items():
    print(b"Point:", x, y, name)

# Nested dicts
nested: dict[str, dict[str, int]] = {
    "group1": {"a": 1, "b": 2},
    "group2": {"c": 3, "d": 4}
}
for group in nested:
    for key in nested[group]:
        print(b"Nested:", group, key, nested[group][key])

for group, inner in nested.items():
    for key, val in inner.items():
        print(b"Items:", group, key, val)

# Dict of lists
lists: dict[str, list[int]] = {"evens": [2, 4, 6], "odds": [1, 3, 5]}
for name, nums in lists.items():
    for n in nums:
        print(b"List item:", name, n)

# Bool keys
flags: dict[bool, str] = {True: "yes", False: "no"}
for flag in flags:
    print(b"Flag:", flag, flags[flag])

# Modify during iteration (via list of keys)
data: dict[str, int] = {"a": 1, "b": 2, "c": 3}
for k in list(data.keys()):
    if data[k] < 2:
        del data[k]
print(b"After delete:", data)

# Build new dict from iteration
source: dict[str, int] = {"x": 10, "y": 20, "z": 30}
doubled: dict[str, int] = {}
for k, v in source.items():
    doubled[k] = v * 2
print(b"Doubled:", doubled)

# Filter dict
filtered: dict[str, int] = {}
for k, v in source.items():
    if v > 15:
        filtered[k] = v
print(b"Filtered:", filtered)

# Invert dict
original: dict[str, int] = {"a": 1, "b": 2, "c": 3}
inverted: dict[int, str] = {}
for k, v in original.items():
    inverted[v] = k
print(b"Inverted:", inverted)
