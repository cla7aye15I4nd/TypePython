# Test for-else construct

# for-else without break (else executes)
for i in range(5):
    print(b"Checking:", i)
else:
    print(b"Loop completed normally")

# for-else with break (else skipped)
for i in range(10):
    if i == 5:
        print(b"Found 5, breaking")
        break
else:
    print(b"This should not print")

# Search pattern with for-else
nums: list[int] = [1, 3, 5, 7, 9]
target: int = 6
for n in nums:
    if n == target:
        print(b"Found target:", target)
        break
else:
    print(b"Target not found:", target)

# Find first match pattern
items: list[str] = ["apple", "banana", "cherry"]
for item in items:
    if item.startswith("b"):
        print(b"First b-word:", item)
        break
else:
    print(b"No b-word found")

# Prime check with for-else
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    else:
        return True
    return False

for num in range(10, 20):
    if is_prime(num):
        print(b"Prime:", num)

# Nested for-else
matrix: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
target_val: int = 5
found: bool = False
for row in matrix:
    for val in row:
        if val == target_val:
            print(b"Found in matrix:", val)
            found = True
            break
    if found:
        break
else:
    print(b"Not found in matrix")

# Empty iterable (else executes)
empty: list[int] = []
for x in empty:
    print(b"This won't print")
else:
    print(b"Empty list, else executed")
