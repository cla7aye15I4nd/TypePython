# Test comprehensive yield patterns including yield from and send

# Basic yield in function
def simple_yield() -> int:
    yield 1
    yield 2
    yield 3

for x in simple_yield():
    print(b"Simple yield:", x)

# Yield with local state
def stateful_yield() -> int:
    state: int = 0
    while state < 5:
        yield state
        state = state + 1

for x in stateful_yield():
    print(b"Stateful:", x)

# Yield expression value
def yield_expr() -> int:
    x: int = yield 1
    yield x if x is not None else 10
    y: int = yield 2
    yield y if y is not None else 20

gen = yield_expr()
print(b"First yield:", next(gen))
print(b"After send:", gen.send(100))
print(b"Second yield:", next(gen))
print(b"After send 2:", gen.send(200))

# Yield from with list
def yield_from_list() -> int:
    yield from [1, 2, 3, 4, 5]

for x in yield_from_list():
    print(b"From list:", x)

# Yield from with range
def yield_from_range(n: int) -> int:
    yield from range(n)

for x in yield_from_range(5):
    print(b"From range:", x)

# Yield from with tuple
def yield_from_tuple() -> int:
    yield from (10, 20, 30)

for x in yield_from_tuple():
    print(b"From tuple:", x)

# Yield from with generator
def inner_gen() -> int:
    yield 1
    yield 2
    yield 3

def outer_gen() -> int:
    yield 0
    yield from inner_gen()
    yield 4

for x in outer_gen():
    print(b"Outer/inner:", x)

# Nested yield from
def level3() -> int:
    yield 3

def level2() -> int:
    yield 2
    yield from level3()

def level1() -> int:
    yield 1
    yield from level2()
    yield 4

for x in level1():
    print(b"Nested levels:", x)

# Yield in try block
def yield_in_try() -> int:
    try:
        yield 1
        yield 2
    finally:
        print(b"Cleanup in finally")
    yield 3

for x in yield_in_try():
    print(b"Try yield:", x)

# Yield with exception handling
def yield_with_except() -> int:
    for i in range(5):
        try:
            if i == 2:
                raise ValueError("skip 2")
            yield i
        except ValueError:
            yield -1

for x in yield_with_except():
    print(b"Except yield:", x)

# Generator delegation with yield from
def gen_a() -> int:
    yield 1
    yield 2

def gen_b() -> int:
    yield 3
    yield 4

def combined() -> int:
    yield from gen_a()
    yield from gen_b()

for x in combined():
    print(b"Combined:", x)

# Yield from with filter
def filter_gen(items: list[int]) -> int:
    for item in items:
        if item > 0:
            yield item

def yield_from_filter() -> int:
    data: list[int] = [-1, 2, -3, 4, -5, 6]
    yield from filter_gen(data)

for x in yield_from_filter():
    print(b"Filtered:", x)

# Recursive generator with yield from
def tree_walk(depth: int, prefix: str) -> str:
    if depth <= 0:
        return
    yield prefix
    yield from tree_walk(depth - 1, prefix + "L")
    yield from tree_walk(depth - 1, prefix + "R")

for path in tree_walk(3, ""):
    print(b"Tree path:", path)

# Yield from string characters
def chars(s: str) -> str:
    yield from s

for c in chars("hello"):
    print(b"Char:", c)

# Generator with close
def closeable_gen() -> int:
    try:
        yield 1
        yield 2
        yield 3
    except GeneratorExit:
        print(b"Generator closed")

g = closeable_gen()
print(b"Got:", next(g))
print(b"Got:", next(g))
g.close()

# Yield from with map result
def mapped_gen() -> int:
    yield from map(lambda x: x * 2, [1, 2, 3, 4, 5])

for x in mapped_gen():
    print(b"Mapped:", x)

# Yield from with filter result
def filtered_gen() -> int:
    yield from filter(lambda x: x % 2 == 0, range(10))

for x in filtered_gen():
    print(b"Filter gen:", x)

# Yield from dict keys
def dict_keys_gen() -> str:
    d: dict[str, int] = {"a": 1, "b": 2, "c": 3}
    yield from d.keys()

for k in dict_keys_gen():
    print(b"Dict key:", k)

# Yield from dict values
def dict_values_gen() -> int:
    d: dict[str, int] = {"a": 1, "b": 2, "c": 3}
    yield from d.values()

for v in dict_values_gen():
    print(b"Dict value:", v)

# Yield from dict items
def dict_items_gen() -> tuple[str, int]:
    d: dict[str, int] = {"a": 1, "b": 2, "c": 3}
    yield from d.items()

for k, v in dict_items_gen():
    print(b"Dict item:", k, v)

# Yield from set
def set_gen() -> int:
    s: set[int] = {1, 2, 3, 4, 5}
    yield from s

for x in set_gen():
    print(b"Set elem:", x)

# Chained yield from
def chain(*iterables) -> int:
    for it in iterables:
        yield from it

for x in chain([1, 2], [3, 4], [5, 6]):
    print(b"Chained:", x)

# Yield from with enumerate
def enum_gen(items: list[str]) -> tuple[int, str]:
    yield from enumerate(items)

for i, v in enum_gen(["a", "b", "c"]):
    print(b"Enum:", i, v)

# Yield from with zip
def zip_gen(a: list[int], b: list[str]) -> tuple[int, str]:
    yield from zip(a, b)

for x, y in zip_gen([1, 2, 3], ["a", "b", "c"]):
    print(b"Zipped:", x, y)

# Yield from with reversed
def reversed_gen(items: list[int]) -> int:
    yield from reversed(items)

for x in reversed_gen([1, 2, 3, 4, 5]):
    print(b"Reversed:", x)

# Yield from with sorted
def sorted_gen(items: list[int]) -> int:
    yield from sorted(items)

for x in sorted_gen([5, 2, 8, 1, 9]):
    print(b"Sorted:", x)

# Generator pipeline with yield from
def source() -> int:
    yield from range(10)

def transform(gen) -> int:
    for x in gen:
        yield x * 2

def sink(gen) -> int:
    for x in gen:
        if x > 5:
            yield x

for x in sink(transform(source())):
    print(b"Pipeline:", x)

# Yield from empty iterable
def empty_gen() -> int:
    yield from []

for x in empty_gen():
    print(b"Empty:", x)
print(b"Empty gen complete")

# Yield from with condition
def conditional_yield_from(flag: bool) -> int:
    if flag:
        yield from [1, 2, 3]
    else:
        yield from [4, 5, 6]

for x in conditional_yield_from(True):
    print(b"Cond true:", x)

for x in conditional_yield_from(False):
    print(b"Cond false:", x)

# Multiple yield from in sequence
def multi_yield_from() -> int:
    yield from [1, 2]
    yield 100
    yield from [3, 4]
    yield 200
    yield from [5, 6]

for x in multi_yield_from():
    print(b"Multi:", x)
