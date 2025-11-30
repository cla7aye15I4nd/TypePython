# Test map() and filter() builtins

def double(x: int) -> int:
    return x * 2

def square(x: int) -> int:
    return x * x

def is_even(x: int) -> bool:
    return x % 2 == 0

def is_positive(x: int) -> bool:
    return x > 0

# Basic map
nums: list[int] = [1, 2, 3, 4, 5]
for val in map(double, nums):
    print(b"Doubled:", val)

# Map to list
squared: list[int] = list(map(square, nums))
print(b"Squared:", squared)

# Basic filter
for val in filter(is_even, nums):
    print(b"Even:", val)

# Filter to list
evens: list[int] = list(filter(is_even, nums))
print(b"Evens list:", evens)

# Chained map and filter
result: list[int] = list(map(square, filter(is_even, range(10))))
print(b"Squared evens:", result)

# Filter with negative numbers
mixed: list[int] = [-2, -1, 0, 1, 2, 3]
positives: list[int] = list(filter(is_positive, mixed))
print(b"Positives:", positives)

# Map on strings
def upper(s: str) -> str:
    return s.upper()

words: list[str] = ["hello", "world"]
uppers: list[str] = list(map(upper, words))
print(b"Uppers:", uppers)
