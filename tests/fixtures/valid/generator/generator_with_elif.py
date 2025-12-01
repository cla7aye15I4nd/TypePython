# Generator with elif clauses - tests generator variable collection in elif branches
def classify_numbers(n: int) -> int:
    i: int = 0
    while i < n:
        if i % 3 == 0:
            yield i * 10  # multiple of 3
        elif i % 2 == 0:
            yield i * 2   # even but not multiple of 3
        else:
            yield i       # odd
        i = i + 1

for x in classify_numbers(10):
    print(x)
