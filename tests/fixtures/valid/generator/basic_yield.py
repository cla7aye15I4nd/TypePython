# Basic yield patterns

# Test 1: Simple yield with while loop
def count_up(n: int) -> int:
    i: int = 0
    while i < n:
        yield i
        i = i + 1

print(b"Test 1: count_up(5)")
for x in count_up(5):
    print(x)

# Test 2: Multiple yields without loop
def simple_yield() -> int:
    yield 1
    yield 2
    yield 3

print(b"Test 2: simple_yield()")
for x in simple_yield():
    print(x)

print(b"Done!")
