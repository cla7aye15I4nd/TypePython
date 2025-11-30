# Test dict comprehensions

# Basic dict comprehension
squares: dict[int, int] = {x: x * x for x in range(5)}
print(b"Squares:", squares)

# Dict comprehension with condition
even_squares: dict[int, int] = {x: x * x for x in range(10) if x % 2 == 0}
print(b"Even squares:", even_squares)

# Dict from two lists using zip
keys: list[str] = ["a", "b", "c"]
vals: list[int] = [1, 2, 3]
d: dict[str, int] = {k: v for k, v in zip(keys, vals)}
print(b"Zipped dict:", d)

# Dict comprehension with string manipulation
words: list[str] = ["hello", "world", "python"]
lengths: dict[str, int] = {w: len(w) for w in words}
print(b"Lengths:", lengths)

# Invert a dict
original: dict[str, int] = {"a": 1, "b": 2, "c": 3}
inverted: dict[int, str] = {v: k for k, v in original.items()}
print(b"Inverted:", inverted)

# Dict comprehension with complex values
data: dict[int, list[int]] = {i: [j for j in range(i)] for i in range(1, 4)}
print(b"Complex:", data)
