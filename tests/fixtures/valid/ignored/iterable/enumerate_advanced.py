# Test advanced enumerate patterns

# Enumerate with different start values
items: list[str] = ["a", "b", "c", "d", "e"]

print(b"Start at 0:")
for i, item in enumerate(items):
    print(b"  ", i, item)

print(b"Start at 1:")
for i, item in enumerate(items, 1):
    print(b"  ", i, item)

print(b"Start at 100:")
for i, item in enumerate(items, 100):
    print(b"  ", i, item)

print(b"Start at -2:")
for i, item in enumerate(items, -2):
    print(b"  ", i, item)

# Enumerate over string
for i, c in enumerate("hello"):
    print(b"Char", i, b":", c)

# Enumerate over dict keys
ages: dict[str, int] = {"alice": 30, "bob": 25, "charlie": 35}
for i, name in enumerate(ages):
    print(b"Person", i, b":", name)

# Enumerate over dict items
for i, (name, age) in enumerate(ages.items()):
    print(b"Entry", i, b":", name, age)

# Enumerate with condition
for i, val in enumerate([10, 20, 30, 40, 50]):
    if i % 2 == 0:
        print(b"Even index:", i, val)

# Enumerate to find indices
text: str = "banana"
for i, c in enumerate(text):
    if c == "a":
        print(b"Found 'a' at index:", i)

# Enumerate in comprehension
indexed: list[tuple[int, str]] = [(i, v) for i, v in enumerate(["x", "y", "z"])]
print(b"Indexed pairs:", indexed)

# Enumerate with zip
names: list[str] = ["alice", "bob"]
scores: list[int] = [95, 87]
for i, (name, score) in enumerate(zip(names, scores), 1):
    print(b"Rank", i, b":", name, score)

# Enumerate to build dict
items2: list[str] = ["first", "second", "third"]
position_map: dict[str, int] = {item: i for i, item in enumerate(items2, 1)}
print(b"Position map:", position_map)

# Enumerate on range
for i, val in enumerate(range(10, 15)):
    print(b"Enum range:", i, val)

# Enumerate on reversed
for i, val in enumerate(reversed([1, 2, 3, 4, 5])):
    print(b"Enum reversed:", i, val)

# Enumerate on generator
def gen() -> int:
    yield 100
    yield 200
    yield 300

for i, val in enumerate(gen()):
    print(b"Enum gen:", i, val)
