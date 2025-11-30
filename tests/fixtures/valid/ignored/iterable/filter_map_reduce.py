# Test filter/map/reduce patterns without builtins

# Filter pattern
nums: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
evens: list[int] = []
for n in nums:
    if n % 2 == 0:
        evens.append(n)
print(b"Evens:", evens)

# Map pattern
doubled: list[int] = []
for n in nums:
    doubled.append(n * 2)
print(b"Doubled:", doubled)

# Filter + Map
squared_evens: list[int] = []
for n in nums:
    if n % 2 == 0:
        squared_evens.append(n * n)
print(b"Squared evens:", squared_evens)

# Map + Filter
large_squares: list[int] = []
for n in nums:
    sq: int = n * n
    if sq > 25:
        large_squares.append(sq)
print(b"Large squares:", large_squares)

# Reduce sum
total: int = 0
for n in nums:
    total = total + n
print(b"Sum:", total)

# Filter + Reduce
even_sum: int = 0
for n in nums:
    if n % 2 == 0:
        even_sum = even_sum + n
print(b"Even sum:", even_sum)

# Map + Reduce
square_sum: int = 0
for n in nums:
    square_sum = square_sum + n * n
print(b"Square sum:", square_sum)

# Complex pipeline: filter, map, reduce
# Sum of squares of even numbers
result: int = 0
for n in nums:
    if n % 2 == 0:
        result = result + n * n
print(b"Sum of even squares:", result)

# String pipeline
words: list[str] = ["hello", "world", "python", "is", "great"]
# Filter long words, map to uppercase, reduce to concat
long_upper: str = ""
for w in words:
    if len(w) > 3:
        long_upper = long_upper + w.upper() + " "
print(b"Long upper:", long_upper)

# Nested filter
matrix: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
flat_evens: list[int] = []
for row in matrix:
    for val in row:
        if val % 2 == 0:
            flat_evens.append(val)
print(b"Flat evens:", flat_evens)
