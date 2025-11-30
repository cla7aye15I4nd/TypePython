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

# Dict with int keys
scores: dict[int, str] = {1: "first", 2: "second", 3: "third"}
for pos in scores:
    print(b"Position:", pos)

for pos, label in scores.items():
    print(b"Pos:", pos, b"Label:", label)
