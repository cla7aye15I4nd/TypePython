# Generator with multiple sequential yields
def multi() -> int:
    yield 1
    yield 2
    yield 3
    yield 4
    yield 5

total: int = 0
for x in multi():
    total = total + x
print(total)
# Expected: 15
