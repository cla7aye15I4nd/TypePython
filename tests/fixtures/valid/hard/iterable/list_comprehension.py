# Test list comprehensions

# Basic list comprehension
squares: list[int] = [x * x for x in range(5)]
print(b"Squares:", squares)

# List comprehension with condition
evens: list[int] = [x for x in range(10) if x % 2 == 0]
print(b"Evens:", evens)

# List comprehension from list
nums: list[int] = [1, 2, 3, 4, 5]
doubled: list[int] = [n * 2 for n in nums]
print(b"Doubled:", doubled)

# String list comprehension
chars: list[str] = [c for c in "hello"]
print(b"Chars:", chars)

# Nested list comprehension
matrix: list[list[int]] = [[i * j for j in range(3)] for i in range(3)]
print(b"Matrix:", matrix)

# Flatten with list comprehension
flat: list[int] = [x for row in matrix for x in row]
print(b"Flattened:", flat)

# List comprehension with function call
def square(n: int) -> int:
    return n * n

result: list[int] = [square(x) for x in range(5)]
print(b"Squares via fn:", result)

# List comprehension with complex condition
filtered: list[int] = [x for x in range(20) if x % 2 == 0 if x % 3 == 0]
print(b"Divisible by 2 and 3:", filtered)
