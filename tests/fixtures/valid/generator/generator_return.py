# Generator with early return
def early_return(n: int) -> int:
    i: int = 0
    while i < n:
        if i == 3:
            return
        yield i
        i = i + 1

for x in early_return(10):
    print(x)
