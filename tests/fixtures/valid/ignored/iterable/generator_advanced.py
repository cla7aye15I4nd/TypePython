# Test advanced generator patterns

# Generator with multiple yields
def multi_yield() -> int:
    yield 1
    yield 2
    yield 3
    yield 4
    yield 5

for val in multi_yield():
    print(b"Multi:", val)

# Generator with yield in loop
def range_gen(n: int) -> int:
    i: int = 0
    while i < n:
        yield i
        i = i + 1

for val in range_gen(5):
    print(b"Range gen:", val)

# Generator that filters
def even_filter(nums: list[int]) -> int:
    for n in nums:
        if n % 2 == 0:
            yield n

data: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
for e in even_filter(data):
    print(b"Even:", e)

# Generator that transforms
def double_gen(nums: list[int]) -> int:
    for n in nums:
        yield n * 2

for d in double_gen([1, 2, 3, 4, 5]):
    print(b"Doubled:", d)

# Chained generators
def add_one(nums: list[int]) -> int:
    for n in nums:
        yield n + 1

def multiply_two(gen) -> int:
    for n in gen:
        yield n * 2

base: list[int] = [1, 2, 3]
for val in multiply_two(add_one(base)):
    print(b"Chained:", val)

# Generator with early termination
def take(gen, n: int) -> int:
    count: int = 0
    for val in gen:
        if count >= n:
            return
        yield val
        count = count + 1

def infinite_counter() -> int:
    i: int = 0
    while True:
        yield i
        i = i + 1

for val in take(infinite_counter(), 5):
    print(b"Taken:", val)

# Generator yielding tuples
def enumerate_gen(items: list[str]) -> tuple[int, str]:
    i: int = 0
    for item in items:
        yield (i, item)
        i = i + 1

for idx, val in enumerate_gen(["a", "b", "c"]):
    print(b"Enum:", idx, val)
