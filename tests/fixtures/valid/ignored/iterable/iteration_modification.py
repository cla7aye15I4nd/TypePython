# Test iteration with modification patterns

# Build new list while iterating
nums: list[int] = [1, 2, 3, 4, 5]
doubled: list[int] = []
for n in nums:
    doubled.append(n * 2)
print(b"Doubled:", doubled)

# Filter into new list
mixed: list[int] = [-2, -1, 0, 1, 2, 3]
positives: list[int] = []
for n in mixed:
    if n > 0:
        positives.append(n)
print(b"Positives:", positives)

# Modify list via index
values: list[int] = [1, 2, 3, 4, 5]
for i in range(len(values)):
    values[i] = values[i] * values[i]
print(b"Squared in place:", values)

# Iterate copy while modifying original
original: list[int] = [1, 2, 3, 4, 5]
for n in original[:]:  # Iterate over copy
    if n % 2 == 0:
        original.remove(n)
print(b"Odds only:", original)

# Build dict while iterating list
names: list[str] = ["alice", "bob", "charlie"]
name_lengths: dict[str, int] = {}
for name in names:
    name_lengths[name] = len(name)
print(b"Name lengths:", name_lengths)

# Accumulator pattern
numbers: list[int] = [1, 2, 3, 4, 5]
running_sum: list[int] = []
total: int = 0
for n in numbers:
    total = total + n
    running_sum.append(total)
print(b"Running sum:", running_sum)

# Counter pattern
text: str = "hello world"
char_count: dict[str, int] = {}
for c in text:
    if c in char_count:
        char_count[c] = char_count[c] + 1
    else:
        char_count[c] = 1
print(b"Char counts:", char_count)

# Grouping pattern
data: list[tuple[str, int]] = [("a", 1), ("b", 2), ("a", 3), ("b", 4), ("a", 5)]
groups: dict[str, list[int]] = {}
for key, val in data:
    if key not in groups:
        groups[key] = []
    groups[key].append(val)
print(b"Groups:", groups)

# Build set while iterating
nums2: list[int] = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
seen: set[int] = set()
unique_order: list[int] = []
for n in nums2:
    if n not in seen:
        seen.add(n)
        unique_order.append(n)
print(b"Unique order:", unique_order)
