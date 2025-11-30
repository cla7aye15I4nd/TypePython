# Test comprehensive generator patterns

# Simple generator
def simple_gen() -> int:
    yield 1
    yield 2
    yield 3

for x in simple_gen():
    print(b"Simple:", x)

# Generator with parameter
def count_up(n: int) -> int:
    i: int = 0
    while i < n:
        yield i
        i = i + 1

for x in count_up(5):
    print(b"Count:", x)

# Generator with loop
def squares(n: int) -> int:
    for i in range(n):
        yield i * i

for x in squares(5):
    print(b"Square:", x)

# Generator with condition
def evens(n: int) -> int:
    for i in range(n):
        if i % 2 == 0:
            yield i

for x in evens(10):
    print(b"Even:", x)

# Generator filter
def filter_positive(nums: list[int]) -> int:
    for n in nums:
        if n > 0:
            yield n

data: list[int] = [-1, 2, -3, 4, -5, 6]
for x in filter_positive(data):
    print(b"Positive:", x)

# Generator transform
def double(nums: list[int]) -> int:
    for n in nums:
        yield n * 2

for x in double([1, 2, 3, 4, 5]):
    print(b"Doubled:", x)

# Chained generators
def add_one(nums: list[int]) -> int:
    for n in nums:
        yield n + 1

def times_two(gen) -> int:
    for n in gen:
        yield n * 2

for x in times_two(add_one([1, 2, 3])):
    print(b"Chained:", x)

# Generator with early return
def first_n(iterable, n: int) -> int:
    count: int = 0
    for item in iterable:
        if count >= n:
            return
        yield item
        count = count + 1

for x in first_n(range(100), 5):
    print(b"First n:", x)

# Infinite generator (with limit)
def infinite() -> int:
    i: int = 0
    while True:
        yield i
        i = i + 1

count: int = 0
for x in infinite():
    print(b"Infinite:", x)
    count = count + 1
    if count >= 5:
        break

# Fibonacci generator
def fibonacci(limit: int) -> int:
    a: int = 0
    b: int = 1
    count: int = 0
    while count < limit:
        yield a
        a, b = b, a + b
        count = count + 1

for x in fibonacci(10):
    print(b"Fib:", x)

# Generator yielding tuples
def enumerate_gen(items: list[str]) -> tuple[int, str]:
    i: int = 0
    for item in items:
        yield (i, item)
        i = i + 1

for idx, val in enumerate_gen(["a", "b", "c"]):
    print(b"Enum gen:", idx, val)

# Generator with state
def running_sum(nums: list[int]) -> int:
    total: int = 0
    for n in nums:
        total = total + n
        yield total

for x in running_sum([1, 2, 3, 4, 5]):
    print(b"Running sum:", x)

# Flatten generator
def flatten(nested: list[list[int]]) -> int:
    for inner in nested:
        for item in inner:
            yield item

matrix: list[list[int]] = [[1, 2], [3, 4], [5, 6]]
for x in flatten(matrix):
    print(b"Flat:", x)

# Repeat generator
def repeat(value: int, times: int) -> int:
    for _ in range(times):
        yield value

for x in repeat(42, 5):
    print(b"Repeat:", x)

# Cycle generator (limited)
def cycle(items: list[int], times: int) -> int:
    for _ in range(times):
        for item in items:
            yield item

for x in cycle([1, 2, 3], 2):
    print(b"Cycle:", x)

# Pairwise generator
def pairwise(items: list[int]) -> tuple[int, int]:
    it = iter(items)
    prev = next(it, None)
    if prev is None:
        return
    for curr in it:
        yield (prev, curr)
        prev = curr

for a, b in pairwise([1, 2, 3, 4, 5]):
    print(b"Pair:", a, b)

# Window generator
def sliding_window(items: list[int], size: int) -> list[int]:
    for i in range(len(items) - size + 1):
        window: list[int] = []
        for j in range(size):
            window.append(items[i + j])
        yield window

for w in sliding_window([1, 2, 3, 4, 5], 3):
    print(b"Window:", w)

# Prime generator
def primes(limit: int) -> int:
    for n in range(2, limit):
        is_prime: bool = True
        for i in range(2, n):
            if n % i == 0:
                is_prime = False
                break
        if is_prime:
            yield n

for p in primes(30):
    print(b"Prime:", p)

# Generator to list
gen_list: list[int] = list(squares(5))
print(b"Gen to list:", gen_list)

# Generator to sum
total: int = sum(count_up(10))
print(b"Gen sum:", total)

# Generator in any/all
print(b"Any even:", any(x % 2 == 0 for x in range(10)))
print(b"All positive:", all(x > 0 for x in [1, 2, 3, 4, 5]))
