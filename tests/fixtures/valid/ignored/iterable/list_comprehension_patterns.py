# Test comprehensive list comprehension patterns

# Basic comprehension
squares: list[int] = [x * x for x in range(5)]
print(b"Squares:", squares)

# With transform
doubled: list[int] = [x * 2 for x in range(5)]
print(b"Doubled:", doubled)

cubed: list[int] = [x ** 3 for x in range(5)]
print(b"Cubed:", cubed)

# With condition
evens: list[int] = [x for x in range(10) if x % 2 == 0]
print(b"Evens:", evens)

odds: list[int] = [x for x in range(10) if x % 2 == 1]
print(b"Odds:", odds)

# Transform and filter
even_squares: list[int] = [x * x for x in range(10) if x % 2 == 0]
print(b"Even squares:", even_squares)

# From list
nums: list[int] = [1, 2, 3, 4, 5]
tripled: list[int] = [n * 3 for n in nums]
print(b"Tripled:", tripled)

# From string
chars: list[str] = [c for c in "hello"]
print(b"Chars:", chars)

upper: list[str] = [c.upper() for c in "hello"]
print(b"Upper:", upper)

# Nested comprehension
matrix: list[list[int]] = [[i * j for j in range(3)] for i in range(3)]
print(b"Matrix:", matrix)

# Flatten
flat: list[int] = [val for row in matrix for val in row]
print(b"Flat:", flat)

# Conditional expression
labels: list[str] = ["even" if x % 2 == 0 else "odd" for x in range(5)]
print(b"Labels:", labels)

# Multiple conditions
filtered: list[int] = [x for x in range(30) if x % 2 == 0 if x % 3 == 0]
print(b"Div 2 and 3:", filtered)

# From enumerate
indexed: list[tuple[int, str]] = [(i, c) for i, c in enumerate("abc")]
print(b"Indexed:", indexed)

# From zip
paired: list[tuple[int, str]] = [(a, b) for a, b in zip([1, 2, 3], ["x", "y", "z"])]
print(b"Paired:", paired)

# From dict
d: dict[str, int] = {"a": 1, "b": 2, "c": 3}
keys: list[str] = [k for k in d]
print(b"Keys:", keys)

values: list[int] = [v for v in d.values()]
print(b"Values:", values)

items: list[tuple[str, int]] = [(k, v) for k, v in d.items()]
print(b"Items:", items)

# String manipulation
words: list[str] = ["  hello  ", "  world  ", "  python  "]
stripped: list[str] = [w.strip() for w in words]
print(b"Stripped:", stripped)

# With function call
def square(x: int) -> int:
    return x * x

result: list[int] = [square(x) for x in range(5)]
print(b"Via function:", result)

# Boolean filter
data: list[int | None] = [1, None, 2, None, 3]
non_none: list[int] = [x for x in data if x is not None]
print(b"Non-none:", non_none)

# Complex expression
polynomial: list[int] = [x ** 2 + 2 * x + 1 for x in range(5)]
print(b"Polynomial:", polynomial)

# From set
s: set[int] = {3, 1, 4, 1, 5}
from_set: list[int] = [x for x in s]
print(b"From set:", from_set)

# Tuple unpacking
points: list[tuple[int, int]] = [(1, 2), (3, 4), (5, 6)]
sums: list[int] = [a + b for a, b in points]
print(b"Point sums:", sums)

# With string methods
names: list[str] = ["alice", "bob", "charlie"]
lengths: list[int] = [len(name) for name in names]
print(b"Lengths:", lengths)

# Nested with filter
nested: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
evens_nested: list[int] = [x for row in nested for x in row if x % 2 == 0]
print(b"Nested evens:", evens_nested)

# Creating range values
ranges: list[list[int]] = [[i for i in range(n)] for n in range(5)]
print(b"Ranges:", ranges)

# Cartesian product
product: list[tuple[int, int]] = [(x, y) for x in range(3) for y in range(3)]
print(b"Cartesian:", product)

# With conditional skip
skip_some: list[int] = [x for x in range(10) if x != 3 if x != 7]
print(b"Skip 3 and 7:", skip_some)

# Boolean values
bools: list[bool] = [x > 5 for x in range(10)]
print(b"Bools:", bools)

# Index access
indices: list[str] = ["abc"[i] for i in range(3)]
print(b"Indices:", indices)
