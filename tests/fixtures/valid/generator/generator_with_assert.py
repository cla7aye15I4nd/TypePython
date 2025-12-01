# Generator with assert statement - tests contains_yield for Assert block
def checked_range(n: int) -> int:
    i: int = 0
    assert n > 0
    while i < n:
        yield i
        i = i + 1

# Test the generator
for x in checked_range(3):
    print(x)
