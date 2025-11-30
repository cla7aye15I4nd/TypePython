# Test various set iteration patterns

# Empty set
empty: set[int] = set()
for x in empty:
    print(b"Never")
print(b"Empty set done")

# Single element
single: set[int] = {42}
for x in single:
    print(b"Single:", x)

# Multiple elements (order not guaranteed)
nums: set[int] = {3, 1, 4, 1, 5, 9, 2, 6}
count: int = 0
for n in nums:
    count = count + 1
print(b"Unique count:", count)

# String set
words: set[str] = {"apple", "banana", "cherry"}
for w in words:
    print(b"Word:", w)

# Build list from set
s: set[int] = {5, 3, 8, 1, 9}
as_list: list[int] = []
for x in s:
    as_list.append(x)
print(b"Set as list:", as_list)

# Set operations during iteration
original: set[int] = {1, 2, 3, 4, 5}
new_set: set[int] = set()
for x in original:
    new_set.add(x * 2)
print(b"Doubled set:", new_set)

# Filter set
evens: set[int] = set()
for x in original:
    if x % 2 == 0:
        evens.add(x)
print(b"Evens:", evens)

# Set of tuples
pairs: set[tuple[int, int]] = {(1, 2), (3, 4), (5, 6)}
for a, b in pairs:
    print(b"Pair:", a, b)

# Set intersection via iteration
s1: set[int] = {1, 2, 3, 4, 5}
s2: set[int] = {4, 5, 6, 7, 8}
intersection: set[int] = set()
for x in s1:
    if x in s2:
        intersection.add(x)
print(b"Intersection:", intersection)

# Set union via iteration
union: set[int] = set()
for x in s1:
    union.add(x)
for x in s2:
    union.add(x)
print(b"Union:", union)

# Set difference via iteration
diff: set[int] = set()
for x in s1:
    if x not in s2:
        diff.add(x)
print(b"Difference:", diff)

# Symmetric difference
sym_diff: set[int] = set()
for x in s1:
    if x not in s2:
        sym_diff.add(x)
for x in s2:
    if x not in s1:
        sym_diff.add(x)
print(b"Symmetric diff:", sym_diff)

# Set with frozen iteration
frozen: frozenset[int] = frozenset({1, 2, 3})
for x in frozen:
    print(b"Frozen:", x)

# Count elements
letters: set[str] = {"a", "b", "c", "d", "e"}
vowels: set[str] = {"a", "e", "i", "o", "u"}
vowel_count: int = 0
for letter in letters:
    if letter in vowels:
        vowel_count = vowel_count + 1
print(b"Vowel count:", vowel_count)
