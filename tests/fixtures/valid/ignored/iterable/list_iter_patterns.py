# Test various list iteration patterns

# Empty list
empty: list[int] = []
for x in empty:
    print(b"Never")
print(b"Empty done")

# Single element
single: list[int] = [42]
for x in single:
    print(b"Single:", x)

# Large list
large: list[int] = []
for i in range(100):
    large.append(i)
total: int = 0
for x in large:
    total = total + x
print(b"Sum 0-99:", total)

# Nested lists - various depths
depth2: list[list[int]] = [[1, 2], [3, 4], [5, 6]]
for inner in depth2:
    for val in inner:
        print(b"D2:", val)

depth3: list[list[list[int]]] = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
for d1 in depth3:
    for d2 in d1:
        for val in d2:
            print(b"D3:", val)

# Mixed type lists
mixed: list[int | str | bool] = [1, "hello", True, 2, "world", False]
for item in mixed:
    print(b"Mixed:", item)

# List of tuples
pairs: list[tuple[int, int]] = [(1, 2), (3, 4), (5, 6)]
for a, b in pairs:
    print(b"Pair:", a, b)

triples: list[tuple[int, int, int]] = [(1, 2, 3), (4, 5, 6)]
for a, b, c in triples:
    print(b"Triple:", a, b, c)

# List of dicts
dicts: list[dict[str, int]] = [{"a": 1}, {"b": 2}, {"c": 3}]
for d in dicts:
    for k, v in d.items():
        print(b"Dict item:", k, v)

# Modify during iteration (via copy)
nums: list[int] = [1, 2, 3, 4, 5]
for n in nums[:]:
    if n % 2 == 0:
        nums.remove(n)
print(b"After remove evens:", nums)

# Index-based modification
vals: list[int] = [1, 2, 3, 4, 5]
for i in range(len(vals)):
    vals[i] = vals[i] ** 2
print(b"Squared:", vals)

# Enumerate patterns
items: list[str] = ["a", "b", "c"]
for i, v in enumerate(items):
    items[i] = v.upper()
print(b"Upper:", items)

# Reverse iteration
rev: list[int] = [1, 2, 3, 4, 5]
for x in reversed(rev):
    print(b"Rev:", x)

# List with None
with_none: list[int | None] = [1, None, 2, None, 3]
for x in with_none:
    if x is not None:
        print(b"Not none:", x)
