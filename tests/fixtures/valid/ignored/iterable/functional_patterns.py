# Test functional programming iteration patterns

# Map pattern
def my_map(func, iterable: list[int]) -> list[int]:
    result: list[int] = []
    for x in iterable:
        result.append(func(x))
    return result

def square(x: int) -> int:
    return x * x

print(b"Map square:", my_map(square, [1, 2, 3, 4, 5]))

# Filter pattern
def my_filter(pred, iterable: list[int]) -> list[int]:
    result: list[int] = []
    for x in iterable:
        if pred(x):
            result.append(x)
    return result

def is_even(x: int) -> bool:
    return x % 2 == 0

print(b"Filter even:", my_filter(is_even, [1, 2, 3, 4, 5, 6, 7, 8]))

# Reduce pattern
def my_reduce(func, iterable: list[int], initial: int) -> int:
    acc: int = initial
    for x in iterable:
        acc = func(acc, x)
    return acc

def add(a: int, b: int) -> int:
    return a + b

print(b"Reduce sum:", my_reduce(add, [1, 2, 3, 4, 5], 0))

def mul(a: int, b: int) -> int:
    return a * b

print(b"Reduce product:", my_reduce(mul, [1, 2, 3, 4, 5], 1))

# Chain operations
nums: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
# Filter evens, square, sum
evens: list[int] = my_filter(is_even, nums)
squared: list[int] = my_map(square, evens)
total: int = my_reduce(add, squared, 0)
print(b"Chain result:", total)

# Takewhile pattern
def takewhile(pred, iterable: list[int]) -> list[int]:
    result: list[int] = []
    for x in iterable:
        if not pred(x):
            break
        result.append(x)
    return result

def less_than_5(x: int) -> bool:
    return x < 5

print(b"Takewhile <5:", takewhile(less_than_5, [1, 2, 3, 4, 5, 6, 2, 1]))

# Dropwhile pattern
def dropwhile(pred, iterable: list[int]) -> list[int]:
    result: list[int] = []
    dropping: bool = True
    for x in iterable:
        if dropping and not pred(x):
            dropping = False
        if not dropping:
            result.append(x)
    return result

print(b"Dropwhile <5:", dropwhile(less_than_5, [1, 2, 3, 4, 5, 6, 2, 1]))

# Partition pattern
def partition(pred, iterable: list[int]) -> tuple[list[int], list[int]]:
    true_list: list[int] = []
    false_list: list[int] = []
    for x in iterable:
        if pred(x):
            true_list.append(x)
        else:
            false_list.append(x)
    return (true_list, false_list)

evens_part, odds_part = partition(is_even, [1, 2, 3, 4, 5, 6])
print(b"Partition evens:", evens_part)
print(b"Partition odds:", odds_part)

# Groupby pattern (simplified)
def groupby(iterable: list[int], key_func) -> dict[int, list[int]]:
    groups: dict[int, list[int]] = {}
    for x in iterable:
        k: int = key_func(x)
        if k not in groups:
            groups[k] = []
        groups[k].append(x)
    return groups

def mod3(x: int) -> int:
    return x % 3

print(b"Groupby mod3:", groupby([1, 2, 3, 4, 5, 6, 7, 8, 9], mod3))

# Scan/accumulate pattern
def scan(func, iterable: list[int], initial: int) -> list[int]:
    result: list[int] = [initial]
    acc: int = initial
    for x in iterable:
        acc = func(acc, x)
        result.append(acc)
    return result

print(b"Scan sum:", scan(add, [1, 2, 3, 4, 5], 0))

# Unfold pattern
def unfold(func, seed: int, n: int) -> list[int]:
    result: list[int] = []
    current: int = seed
    for _ in range(n):
        result.append(current)
        current = func(current)
    return result

def double(x: int) -> int:
    return x * 2

print(b"Unfold double:", unfold(double, 1, 10))

# Compose functions
def compose(f, g):
    def composed(x: int) -> int:
        return f(g(x))
    return composed

def inc(x: int) -> int:
    return x + 1

square_then_inc = compose(inc, square)
result_composed: list[int] = my_map(square_then_inc, [1, 2, 3, 4, 5])
print(b"Composed:", result_composed)

# Curry pattern simulation
def add_n(n: int):
    def adder(x: int) -> int:
        return x + n
    return adder

add5 = add_n(5)
print(b"Add 5:", my_map(add5, [1, 2, 3, 4, 5]))

# Flatten pattern
def flatten(nested: list) -> list[int]:
    result: list[int] = []
    for item in nested:
        if isinstance(item, list):
            for sub in flatten(item):
                result.append(sub)
        else:
            result.append(item)
    return result

nested: list = [1, [2, 3], [4, [5, 6]], 7]
print(b"Flatten:", flatten(nested))

# Zip with function
def zipwith(func, a: list[int], b: list[int]) -> list[int]:
    result: list[int] = []
    for x, y in zip(a, b):
        result.append(func(x, y))
    return result

print(b"Zipwith add:", zipwith(add, [1, 2, 3], [10, 20, 30]))

# Any/all patterns
def my_any(pred, iterable: list[int]) -> bool:
    for x in iterable:
        if pred(x):
            return True
    return False

def my_all(pred, iterable: list[int]) -> bool:
    for x in iterable:
        if not pred(x):
            return False
    return True

print(b"Any even:", my_any(is_even, [1, 3, 5, 6, 7]))
print(b"All even:", my_all(is_even, [2, 4, 6, 8]))
print(b"All positive:", my_all(lambda x: x > 0, [1, 2, 3, 4, 5]))

# None pattern
def my_none(pred, iterable: list[int]) -> bool:
    for x in iterable:
        if pred(x):
            return False
    return True

def is_negative(x: int) -> bool:
    return x < 0

print(b"None negative:", my_none(is_negative, [1, 2, 3, 4, 5]))
