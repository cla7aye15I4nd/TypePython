# Test comprehensive iter/next patterns

# Basic iter and next
nums: list[int] = [1, 2, 3, 4, 5]
it = iter(nums)
print(b"Next 1:", next(it))
print(b"Next 2:", next(it))
print(b"Next 3:", next(it))

# Next with default
it2 = iter([10, 20])
print(b"Val:", next(it2))
print(b"Val:", next(it2))
print(b"Default:", next(it2, -1))
print(b"Still default:", next(it2, -1))

# Iter on string
s_it = iter("hello")
print(b"Char:", next(s_it))
print(b"Char:", next(s_it))
print(b"Char:", next(s_it))

# Iter on range
r_it = iter(range(3))
print(b"Range:", next(r_it))
print(b"Range:", next(r_it))
print(b"Range:", next(r_it))
print(b"Range default:", next(r_it, -1))

# Iter on dict
d: dict[str, int] = {"a": 1, "b": 2, "c": 3}
d_it = iter(d)
print(b"Dict key:", next(d_it))
print(b"Dict key:", next(d_it))

# Iter on set
s: set[int] = {10, 20, 30}
s_it = iter(s)
print(b"Set val:", next(s_it))

# Multiple iterators on same collection
data: list[int] = [1, 2, 3, 4, 5]
it1 = iter(data)
it2 = iter(data)
print(b"it1:", next(it1))
print(b"it2:", next(it2))
print(b"it1:", next(it1))
print(b"it2:", next(it2))

# Iterator exhaustion
short: list[int] = [1]
short_it = iter(short)
print(b"First:", next(short_it))
print(b"After exhaustion:", next(short_it, "DONE"))

# Partial consumption then loop
partial: list[int] = [1, 2, 3, 4, 5]
p_it = iter(partial)
print(b"Manual 1:", next(p_it))
print(b"Manual 2:", next(p_it))
for remaining in p_it:
    print(b"Loop:", remaining)

# Skip first n elements
to_skip: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
skip_it = iter(to_skip)
for _ in range(3):
    next(skip_it)
for x in skip_it:
    print(b"After skip:", x)

# Take first n elements
def take_n(iterable, n: int) -> list:
    it = iter(iterable)
    result: list = []
    for _ in range(n):
        val = next(it, None)
        if val is None:
            break
        result.append(val)
    return result

print(b"First 3:", take_n([1, 2, 3, 4, 5], 3))

# Interleave two iterators
a: list[int] = [1, 2, 3]
b: list[int] = [10, 20, 30]
a_it = iter(a)
b_it = iter(b)
interleaved: list[int] = []
for _ in range(6):
    if len(interleaved) % 2 == 0:
        val = next(a_it, None)
    else:
        val = next(b_it, None)
    if val is not None:
        interleaved.append(val)
print(b"Interleaved:", interleaved)

# Peek pattern (consume and store)
peek_data: list[int] = [1, 2, 3, 4, 5]
peek_it = iter(peek_data)
peeked: int = next(peek_it)
print(b"Peeked:", peeked)
all_vals: list[int] = [peeked]
for x in peek_it:
    all_vals.append(x)
print(b"All vals:", all_vals)

# Iter on iter (returns same)
base: list[int] = [1, 2, 3]
it_base = iter(base)
it_same = iter(it_base)
print(b"Same iter:", it_base is it_same)

# Different default types
empty_it1 = iter([])
print(b"Default int:", next(empty_it1, 0))
empty_it2 = iter([])
print(b"Default str:", next(empty_it2, "none"))
empty_it3 = iter([])
print(b"Default None:", next(empty_it3, None))
empty_it4 = iter([])
print(b"Default list:", next(empty_it4, []))
empty_it5 = iter([])
print(b"Default bool:", next(empty_it5, False))

# Sentinel pattern (two-arg iter)
# Note: iter(callable, sentinel) - keep calling until sentinel returned
counter: int = 0
def increment() -> int:
    global counter
    counter = counter + 1
    return counter

# Manual sentinel simulation
counter = 0
vals: list[int] = []
while True:
    v: int = increment()
    if v > 5:
        break
    vals.append(v)
print(b"Sentinel vals:", vals)

# Chunking with iter
def chunks(data: list[int], size: int) -> list[list[int]]:
    it = iter(data)
    result: list[list[int]] = []
    while True:
        chunk: list[int] = []
        for _ in range(size):
            val = next(it, None)
            if val is None:
                break
            chunk.append(val)
        if len(chunk) == 0:
            break
        result.append(chunk)
    return result

print(b"Chunks:", chunks([1, 2, 3, 4, 5, 6, 7, 8], 3))
