# Test min(), max(), sum() builtins with iterables

# Basic min/max/sum
nums: list[int] = [3, 1, 4, 1, 5, 9, 2, 6]
print(b"Min:", min(nums))
print(b"Max:", max(nums))
print(b"Sum:", sum(nums))

# With floats
floats: list[float] = [1.5, 2.5, 0.5, 3.5]
print(b"Min float:", min(floats))
print(b"Max float:", max(floats))
print(b"Sum float:", sum(floats))

# Min/max with strings
words: list[str] = ["banana", "apple", "cherry"]
print(b"Min word:", min(words))
print(b"Max word:", max(words))

# Min/max with key function
def by_length(s: str) -> int:
    return len(s)

print(b"Shortest:", min(words, key=by_length))
print(b"Longest:", max(words, key=by_length))

# Sum with start value
nums2: list[int] = [1, 2, 3]
print(b"Sum with start:", sum(nums2, 10))

# Min/max with two arguments
print(b"Min of 2:", min(5, 3))
print(b"Max of 2:", max(5, 3))

# Min/max with multiple arguments
print(b"Min of many:", min(5, 3, 8, 1, 9))
print(b"Max of many:", max(5, 3, 8, 1, 9))

# Sum of generator expression
print(b"Sum squares:", sum(x * x for x in range(5)))

# Empty with default
empty: list[int] = []
print(b"Min empty default:", min(empty, default=-1))
print(b"Max empty default:", max(empty, default=-1))
