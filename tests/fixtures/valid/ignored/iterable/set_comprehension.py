# Test set comprehensions

# Basic set comprehension
squares: set[int] = {x * x for x in range(5)}
print(b"Squares:", squares)

# Set comprehension with condition
evens: set[int] = {x for x in range(10) if x % 2 == 0}
print(b"Evens:", evens)

# Set comprehension from list (removes duplicates)
nums: list[int] = [1, 2, 2, 3, 3, 3, 4, 4, 4, 4]
unique: set[int] = {n for n in nums}
print(b"Unique:", unique)

# Set comprehension with string
chars: set[str] = {c for c in "hello"}
print(b"Unique chars:", chars)

# Set comprehension with modulo (limited values)
mod_set: set[int] = {x % 3 for x in range(10)}
print(b"Modulo 3 values:", mod_set)

# Nested set comprehension
pairs: set[tuple[int, int]] = {(i, j) for i in range(3) for j in range(3) if i != j}
print(b"Non-diagonal pairs:", pairs)
