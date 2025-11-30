# Test generator expressions

# Basic generator expression
gen = (x * 2 for x in range(5))
for val in gen:
    print(b"Doubled:", val)

# Generator expression with condition
evens = (x for x in range(10) if x % 2 == 0)
for e in evens:
    print(b"Even:", e)

# Pass generator to sum
total: int = sum(x for x in range(5))
print(b"Sum:", total)

# Pass generator to list
squares: list[int] = list(x * x for x in range(5))
print(b"Squares:", squares)

# Generator expression over list
nums: list[int] = [1, 2, 3, 4, 5]
doubled = (n * 2 for n in nums)
for d in doubled:
    print(b"Val:", d)

# Nested generator expressions
matrix: list[list[int]] = [[1, 2], [3, 4], [5, 6]]
flattened = (item for row in matrix for item in row)
for val in flattened:
    print(b"Flat:", val)
