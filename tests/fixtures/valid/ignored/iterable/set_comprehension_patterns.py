# Test comprehensive set comprehension patterns

# Basic set comprehension
squares: set[int] = {x * x for x in range(5)}
print(b"Squares:", squares)

# With transform
doubled: set[int] = {x * 2 for x in range(5)}
print(b"Doubled:", doubled)

# With condition
evens: set[int] = {x for x in range(10) if x % 2 == 0}
print(b"Evens:", evens)

# Removes duplicates
nums: list[int] = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
unique: set[int] = {n for n in nums}
print(b"Unique:", unique)

# From string (unique chars)
chars: set[str] = {c for c in "hello"}
print(b"Unique chars:", chars)

# Modulo set
mods: set[int] = {x % 5 for x in range(20)}
print(b"Mods:", mods)

# Multiple conditions
filtered: set[int] = {x for x in range(30) if x % 2 == 0 if x % 3 == 0}
print(b"Div 2 and 3:", filtered)

# From list with condition
positives: set[int] = {x for x in [-1, 2, -3, 4, -5, 6] if x > 0}
print(b"Positives:", positives)

# Nested iteration
pairs: set[tuple[int, int]] = {(i, j) for i in range(3) for j in range(3) if i != j}
print(b"Pairs:", pairs)

# From enumerate
indices: set[int] = {i for i, c in enumerate("hello") if c in "aeiou"}
print(b"Vowel indices:", indices)

# From dict values
d: dict[str, int] = {"a": 1, "b": 2, "c": 1, "d": 3, "e": 2}
unique_vals: set[int] = {v for v in d.values()}
print(b"Unique vals:", unique_vals)

# From dict keys
keys_set: set[str] = {k for k in d}
print(b"Keys set:", keys_set)

# String manipulation
words: list[str] = ["Hello", "World", "hello", "world"]
lower_unique: set[str] = {w.lower() for w in words}
print(b"Lower unique:", lower_unique)

# First letters
first_letters: set[str] = {w[0] for w in ["apple", "banana", "cherry", "apricot"]}
print(b"First letters:", first_letters)

# Absolute values
absolutes: set[int] = {abs(x) for x in [-3, -2, -1, 0, 1, 2, 3]}
print(b"Absolutes:", absolutes)

# Lengths
lengths: set[int] = {len(w) for w in ["hi", "hello", "hey", "howdy"]}
print(b"Unique lengths:", lengths)

# With function
def square(x: int) -> int:
    return x * x

via_func: set[int] = {square(x) for x in range(-5, 6)}
print(b"Via function:", via_func)

# Conditional expression
labels: set[str] = {"even" if x % 2 == 0 else "odd" for x in range(10)}
print(b"Labels:", labels)

# From zip
zipped: set[tuple[int, int]] = {(a, b) for a, b in zip([1, 2, 3], [4, 5, 6])}
print(b"Zipped:", zipped)

# Tuple unpacking
points: list[tuple[int, int]] = [(1, 2), (3, 4), (5, 6), (1, 2)]
x_coords: set[int] = {x for x, y in points}
print(b"X coords:", x_coords)

sums: set[int] = {x + y for x, y in points}
print(b"Point sums:", sums)

# From nested list
nested: list[list[int]] = [[1, 2], [2, 3], [3, 4], [4, 5]]
all_vals: set[int] = {x for inner in nested for x in inner}
print(b"All vals:", all_vals)

# Intersection via comprehension
s1: set[int] = {1, 2, 3, 4, 5}
s2: set[int] = {4, 5, 6, 7, 8}
intersection: set[int] = {x for x in s1 if x in s2}
print(b"Intersection:", intersection)

# Difference via comprehension
difference: set[int] = {x for x in s1 if x not in s2}
print(b"Difference:", difference)

# Complex condition
complex_set: set[int] = {x for x in range(100) if x % 2 == 0 if x % 5 == 0 if x > 0}
print(b"Complex:", complex_set)

# Boolean operations
bool_results: set[bool] = {x > 5 for x in range(10)}
print(b"Bool results:", bool_results)
