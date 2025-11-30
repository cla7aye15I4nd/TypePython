# Test unpacking in iteration

# Tuple unpacking in for loop
pairs: list[tuple[int, int]] = [(1, 2), (3, 4), (5, 6)]
for a, b in pairs:
    print(b"Pair:", a, b)

# Triple unpacking
triples: list[tuple[int, int, int]] = [(1, 2, 3), (4, 5, 6), (7, 8, 9)]
for x, y, z in triples:
    print(b"Triple:", x, y, z)

# Unpacking with dict.items()
ages: dict[str, int] = {"alice": 30, "bob": 25}
for name, age in ages.items():
    print(b"Person:", name, age)

# Unpacking enumerate
items: list[str] = ["a", "b", "c"]
for i, item in enumerate(items):
    print(b"Index:", i, b"Item:", item)

# Unpacking zip
names: list[str] = ["alice", "bob", "charlie"]
scores: list[int] = [95, 87, 92]
for name, score in zip(names, scores):
    print(b"Name:", name, b"Score:", score)

# Nested tuple unpacking
nested: list[tuple[int, tuple[int, int]]] = [(1, (2, 3)), (4, (5, 6))]
for a, (b, c) in nested:
    print(b"Nested:", a, b, c)

# Unpacking in list comprehension
pairs2: list[tuple[int, int]] = [(1, 2), (3, 4), (5, 6)]
sums: list[int] = [a + b for a, b in pairs2]
print(b"Sums:", sums)

# Unpacking in dict comprehension
swapped: dict[int, int] = {b: a for a, b in pairs2}
print(b"Swapped:", swapped)

# Star unpacking (extended unpacking)
data: list[int] = [1, 2, 3, 4, 5]
first, *middle, last = data
print(b"First:", first)
print(b"Middle:", middle)
print(b"Last:", last)

# Star in loop (rest of tuple)
records: list[tuple[str, int, int, int]] = [
    ("alice", 90, 85, 92),
    ("bob", 78, 82, 80)
]
for name, *scores in records:
    print(b"Name:", name, b"Scores:", scores)
