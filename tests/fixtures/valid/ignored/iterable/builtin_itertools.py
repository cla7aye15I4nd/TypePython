# Test itertools-like patterns (manual implementations)

# Chain multiple iterables
def chain(*iterables) -> any:
    for it in iterables:
        for item in it:
            yield item

list1: list[int] = [1, 2, 3]
list2: list[int] = [4, 5, 6]
list3: list[int] = [7, 8, 9]
for val in chain(list1, list2, list3):
    print(b"Chained:", val)

# Repeat value n times
def repeat(value: int, n: int) -> int:
    for _ in range(n):
        yield value

for val in repeat(42, 5):
    print(b"Repeated:", val)

# Take first n elements
def take(iterable, n: int) -> any:
    count: int = 0
    for item in iterable:
        if count >= n:
            return
        yield item
        count = count + 1

for val in take(range(100), 5):
    print(b"Taken:", val)

# Drop first n elements
def drop(iterable, n: int) -> any:
    count: int = 0
    for item in iterable:
        if count >= n:
            yield item
        else:
            count = count + 1

for val in drop(range(10), 5):
    print(b"After drop:", val)

# Takewhile
def takewhile(predicate, iterable) -> any:
    for item in iterable:
        if not predicate(item):
            return
        yield item

def less_than_5(x: int) -> bool:
    return x < 5

for val in takewhile(less_than_5, range(10)):
    print(b"Takewhile:", val)

# Dropwhile
def dropwhile(predicate, iterable) -> any:
    dropping: bool = True
    for item in iterable:
        if dropping:
            if not predicate(item):
                dropping = False
                yield item
        else:
            yield item

for val in dropwhile(less_than_5, range(10)):
    print(b"Dropwhile:", val)

# Accumulate (running sum)
def accumulate(iterable) -> int:
    total: int = 0
    for item in iterable:
        total = total + item
        yield total

for val in accumulate([1, 2, 3, 4, 5]):
    print(b"Accumulated:", val)

# Pairwise iteration
def pairwise(iterable) -> tuple[any, any]:
    it = iter(iterable)
    prev = next(it, None)
    if prev is None:
        return
    for curr in it:
        yield (prev, curr)
        prev = curr

for a, b in pairwise([1, 2, 3, 4, 5]):
    print(b"Pair:", a, b)
