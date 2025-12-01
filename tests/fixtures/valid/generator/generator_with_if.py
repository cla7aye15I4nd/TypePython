# Generator with conditionals
def even_only(n: int) -> int:
    i: int = 0
    while i < n:
        if i % 2 == 0:
            yield i
        i = i + 1

for x in even_only(10):
    print(x)
