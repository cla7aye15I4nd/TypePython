# Test basic generator functions

def count_up(n: int) -> int:
    i: int = 0
    while i < n:
        yield i
        i = i + 1

# Use generator in for loop
for x in count_up(5):
    print(b"Generated:", x)

# Generator for even numbers
def evens(limit: int) -> int:
    i: int = 0
    while i < limit:
        yield i
        i = i + 2

for e in evens(10):
    print(b"Even:", e)

# Generator with early return
def first_n_positive(nums: list[int], n: int) -> int:
    count: int = 0
    for x in nums:
        if x > 0:
            yield x
            count = count + 1
            if count >= n:
                return

data: list[int] = [-1, 2, -3, 4, 5, -6, 7]
for val in first_n_positive(data, 3):
    print(b"Positive:", val)

# Nested generator usage
def squares(n: int) -> int:
    for i in range(n):
        yield i * i

total: int = 0
for sq in squares(5):
    total = total + sq
print(b"Sum of squares:", total)
