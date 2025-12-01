# Test for loop over dictionaries

# Iterate over dict keys
ages: dict[str, int] = {"alice": 30, "bob": 25, "charlie": 35}
for name in ages:
    print(b"Name:", name)

# Iterate using .keys()
for key in ages.keys():
    print(b"Key:", key)

# Iterate using .values()
total: int = 0
for age in ages.values():
    total = total + age
print(b"Total age:", total)

# Iterate using .items()
for name, age in ages.items():
    print(b"Person:", name, b"Age:", age)

# Dict with int keys - use sum to avoid order dependency
scores: dict[int, str] = {1: "first", 2: "second", 3: "third"}
pos_sum: int = 0
for pos in scores:
    pos_sum = pos_sum + pos
print(b"Position sum:", pos_sum)

items_sum: int = 0
for pos, label in scores.items():
    items_sum = items_sum + pos
print(b"Items sum:", items_sum)
