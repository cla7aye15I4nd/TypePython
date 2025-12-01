# Generator with if-else in body
def odd_or_double(n: int) -> int:
    i: int = 0
    while i < n:
        if i % 2 == 0:
            yield i * 2
        else:
            yield i
        i = i + 1

for x in odd_or_double(5):
    print(x)
