# Test comprehensive dict comprehension patterns

# Basic dict comprehension
squares: dict[int, int] = {x: x * x for x in range(5)}
print(b"Squares:", squares)

# With transform
doubled: dict[int, int] = {x: x * 2 for x in range(5)}
print(b"Doubled:", doubled)

# With condition
even_squares: dict[int, int] = {x: x * x for x in range(10) if x % 2 == 0}
print(b"Even squares:", even_squares)

# From two lists with zip
keys: list[str] = ["a", "b", "c"]
vals: list[int] = [1, 2, 3]
zipped: dict[str, int] = {k: v for k, v in zip(keys, vals)}
print(b"Zipped:", zipped)

# Invert dict
original: dict[str, int] = {"a": 1, "b": 2, "c": 3}
inverted: dict[int, str] = {v: k for k, v in original.items()}
print(b"Inverted:", inverted)

# From enumerate
indexed: dict[int, str] = {i: v for i, v in enumerate(["x", "y", "z"])}
print(b"Indexed:", indexed)

# String to length
words: list[str] = ["hello", "world", "python"]
lengths: dict[str, int] = {w: len(w) for w in words}
print(b"Lengths:", lengths)

# Filter dict
source: dict[str, int] = {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
filtered: dict[str, int] = {k: v for k, v in source.items() if v > 2}
print(b"Filtered:", filtered)

# Transform values
doubled_vals: dict[str, int] = {k: v * 2 for k, v in source.items()}
print(b"Doubled vals:", doubled_vals)

# Transform keys
upper_keys: dict[str, int] = {k.upper(): v for k, v in source.items()}
print(b"Upper keys:", upper_keys)

# Conditional value
conditional: dict[int, str] = {x: "even" if x % 2 == 0 else "odd" for x in range(5)}
print(b"Conditional:", conditional)

# Nested structure
nested: dict[int, list[int]] = {i: [j for j in range(i)] for i in range(1, 5)}
print(b"Nested:", nested)

# From string chars
char_pos: dict[str, int] = {c: i for i, c in enumerate("abcde")}
print(b"Char positions:", char_pos)

# Multiple conditions
multi_cond: dict[int, int] = {x: x * x for x in range(20) if x % 2 == 0 if x % 3 == 0}
print(b"Multi condition:", multi_cond)

# Tuple keys
tuple_keys: dict[tuple[int, int], int] = {(i, j): i * j for i in range(3) for j in range(3)}
print(b"Tuple keys:", tuple_keys)

# From set
s: set[int] = {1, 2, 3, 4, 5}
from_set: dict[int, int] = {x: x * x for x in s}
print(b"From set:", from_set)

# Grade mapping
scores: dict[str, int] = {"alice": 85, "bob": 92, "charlie": 78}
grades: dict[str, str] = {
    name: "A" if score >= 90 else "B" if score >= 80 else "C"
    for name, score in scores.items()
}
print(b"Grades:", grades)

# Frequency count from list
items: list[str] = ["a", "b", "a", "c", "b", "a"]
unique: set[str] = set(items)
freq: dict[str, int] = {x: items.count(x) for x in unique}
print(b"Frequency:", freq)

# Default values
defaults: dict[str, int] = {k: 0 for k in ["x", "y", "z"]}
print(b"Defaults:", defaults)

# Merge with transform
d1: dict[str, int] = {"a": 1, "b": 2}
d2: dict[str, int] = {"c": 3, "d": 4}
merged: dict[str, int] = {k: v * 10 for d in [d1, d2] for k, v in d.items()}
print(b"Merged:", merged)

# Computed keys
computed: dict[str, int] = {"key" + str(i): i * i for i in range(5)}
print(b"Computed keys:", computed)

# Bool values
bool_map: dict[int, bool] = {x: x % 2 == 0 for x in range(5)}
print(b"Bool map:", bool_map)

# Range as values
range_vals: dict[int, range] = {i: range(i) for i in range(1, 5)}
print(b"Range vals:", range_vals)

# Complex transform
complex_t: dict[str, int] = {
    k.upper(): v ** 2 + v + 1
    for k, v in {"a": 1, "b": 2, "c": 3}.items()
    if v > 1
}
print(b"Complex:", complex_t)
