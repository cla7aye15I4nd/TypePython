# Test nested iteration patterns

# Triple nested loop
for i in range(2):
    for j in range(2):
        for k in range(2):
            print(b"Triple:", i, j, k)

# Nested list iteration
matrix: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
for row in matrix:
    for val in row:
        print(b"Val:", val)

# Nested dict iteration
data: dict[str, dict[str, int]] = {
    "alice": {"age": 30, "score": 95},
    "bob": {"age": 25, "score": 87}
}
for name in data:
    for key in data[name]:
        print(b"Person:", name, b"Key:", key, b"Val:", data[name][key])

# Mixed nesting: list of dicts
records: list[dict[str, int]] = [
    {"x": 1, "y": 2},
    {"x": 3, "y": 4},
    {"x": 5, "y": 6}
]
for record in records:
    for key, val in record.items():
        print(b"Record:", key, val)

# Dict of lists
groups: dict[str, list[int]] = {
    "evens": [2, 4, 6],
    "odds": [1, 3, 5]
}
for group_name, numbers in groups.items():
    print(b"Group:", group_name)
    for n in numbers:
        print(b"  Num:", n)

# Nested with break
found: bool = False
for i in range(5):
    for j in range(5):
        if i * j == 6:
            print(b"Found at:", i, j)
            found = True
            break
    if found:
        break

# Nested with continue
for i in range(3):
    for j in range(3):
        if j == 1:
            continue
        print(b"Skipped j=1:", i, j)
