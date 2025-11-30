# Test comprehensive generator expression patterns

# Basic generator expression
gen = (x for x in range(5))
for val in gen:
    print(b"Basic:", val)

# Generator expression with transform
doubled = (x * 2 for x in range(5))
for val in doubled:
    print(b"Doubled:", val)

squared = (x * x for x in range(5))
for val in squared:
    print(b"Squared:", val)

# Generator expression with condition
evens = (x for x in range(10) if x % 2 == 0)
for val in evens:
    print(b"Even:", val)

positives = (x for x in [-1, 2, -3, 4, -5] if x > 0)
for val in positives:
    print(b"Positive:", val)

# Transform and filter
even_squares = (x * x for x in range(10) if x % 2 == 0)
for val in even_squares:
    print(b"Even square:", val)

# Generator from list
nums: list[int] = [1, 2, 3, 4, 5]
tripled = (n * 3 for n in nums)
for val in tripled:
    print(b"Tripled:", val)

# Generator from string
chars = (c for c in "hello")
for c in chars:
    print(b"Char:", c)

upper_chars = (c.upper() for c in "hello")
for c in upper_chars:
    print(b"Upper:", c)

# Nested generator expression
matrix: list[list[int]] = [[1, 2], [3, 4], [5, 6]]
flat = (val for row in matrix for val in row)
for val in flat:
    print(b"Flat:", val)

# Generator with conditional expression
labels = ("even" if x % 2 == 0 else "odd" for x in range(5))
for label in labels:
    print(b"Label:", label)

# Pass to sum
total: int = sum(x for x in range(10))
print(b"Sum:", total)

total_sq: int = sum(x * x for x in range(5))
print(b"Sum squares:", total_sq)

# Pass to list
squares_list: list[int] = list(x * x for x in range(5))
print(b"Squares list:", squares_list)

# Pass to any/all
has_even: bool = any(x % 2 == 0 for x in [1, 3, 5, 6, 7])
print(b"Has even:", has_even)

all_positive: bool = all(x > 0 for x in [1, 2, 3, 4, 5])
print(b"All positive:", all_positive)

# Pass to min/max
min_val: int = min(x * x for x in range(-5, 5))
print(b"Min square:", min_val)

max_val: int = max(x for x in range(10))
print(b"Max:", max_val)

# Pass to tuple
as_tuple: tuple[int, ...] = tuple(x * 2 for x in range(5))
print(b"Tuple:", as_tuple)

# Pass to set
as_set: set[int] = set(x % 3 for x in range(10))
print(b"Set:", as_set)

# Pass to dict (with zip)
keys: list[str] = ["a", "b", "c"]
as_dict: dict[str, int] = dict((k, i) for i, k in enumerate(keys))
print(b"Dict:", as_dict)

# Multiple conditions
filtered = (x for x in range(30) if x % 2 == 0 if x % 3 == 0)
for val in filtered:
    print(b"Div 2 and 3:", val)

# Generator from enumerate
indexed = ((i, v) for i, v in enumerate("abc"))
for i, v in indexed:
    print(b"Indexed:", i, v)

# Generator from zip
paired = ((a, b) for a, b in zip([1, 2, 3], ["x", "y", "z"]))
for a, b in paired:
    print(b"Paired:", a, b)

# Generator from dict items
d: dict[str, int] = {"a": 1, "b": 2, "c": 3}
doubled_vals = ((k, v * 2) for k, v in d.items())
for k, v in doubled_vals:
    print(b"Dict doubled:", k, v)

# Chained generator expressions
step1 = (x + 1 for x in range(5))
step2 = (x * 2 for x in step1)
for val in step2:
    print(b"Chained:", val)

# Complex expression
result = (x ** 2 + x + 1 for x in range(5))
for val in result:
    print(b"Polynomial:", val)

# Generator expression with function call
def process(x: int) -> int:
    return x * x + 1

processed = (process(x) for x in range(5))
for val in processed:
    print(b"Processed:", val)

# Generator for string join
words: list[str] = ["hello", "world", "python"]
gen_words = (w.upper() for w in words)
result_str: str = " ".join(gen_words)
print(b"Joined:", result_str)
