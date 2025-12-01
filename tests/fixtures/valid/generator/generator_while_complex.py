# Generator with while loop and multiple yields
def countdown_up(n: int) -> int:
    i: int = n
    while i > 0:
        yield i
        i = i - 1
    # After countdown, count up
    i = 1
    while i <= n:
        yield i * 10
        i = i + 1

for x in countdown_up(3):
    print(x)
