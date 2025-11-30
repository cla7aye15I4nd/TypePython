# Test advanced any() and all() patterns

# any/all with generator expressions
nums: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

print(b"Any even:", any(x % 2 == 0 for x in nums))
print(b"Any > 10:", any(x > 10 for x in nums))
print(b"All positive:", all(x > 0 for x in nums))
print(b"All < 100:", all(x < 100 for x in nums))
print(b"All even:", all(x % 2 == 0 for x in nums))

# any/all short-circuit behavior
def check_and_print(x: int, threshold: int) -> bool:
    print(b"Checking:", x)
    return x > threshold

# any stops at first True
print(b"--- any short-circuit ---")
result: bool = any(check_and_print(x, 3) for x in [1, 2, 3, 4, 5])
print(b"Result:", result)

# all stops at first False
print(b"--- all short-circuit ---")
result2: bool = all(check_and_print(x, 0) for x in [5, 4, 3, 0, 1])
print(b"Result:", result2)

# any/all with complex conditions
data: list[dict[str, int]] = [
    {"name": "alice", "age": 30},
    {"name": "bob", "age": 25},
    {"name": "charlie", "age": 35}
]

print(b"Any over 30:", any(d["age"] > 30 for d in data))
print(b"All over 20:", all(d["age"] > 20 for d in data))

# any/all with string checks
words: list[str] = ["hello", "world", "python"]
print(b"Any starts with p:", any(w.startswith("p") for w in words))
print(b"All len > 3:", all(len(w) > 3 for w in words))

# any/all with None checks
values: list[int | None] = [1, 2, None, 4, 5]
print(b"Any None:", any(v is None for v in values))
print(b"All not None:", all(v is not None for v in values))

# Nested any/all
matrix: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
print(b"Any row all even:", any(all(x % 2 == 0 for x in row) for row in matrix))
print(b"All rows have even:", all(any(x % 2 == 0 for x in row) for row in matrix))

# any/all with set
s: set[int] = {1, 2, 3, 4, 5}
print(b"Any in set > 3:", any(x > 3 for x in s))
print(b"All in set > 0:", all(x > 0 for x in s))

# any/all with dict
ages: dict[str, int] = {"alice": 30, "bob": 25, "charlie": 35}
print(b"Any age > 30:", any(age > 30 for age in ages.values()))
print(b"All names start lower:", all(name[0].islower() for name in ages.keys()))

# Combining any and all
nums2: list[int] = [2, 4, 6, 8, 10]
print(b"All even and any > 5:", all(x % 2 == 0 for x in nums2) and any(x > 5 for x in nums2))
