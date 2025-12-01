# Generator with pass statement
def gen_with_pass(n: int) -> int:
    i: int = 0
    while i < n:
        if i == 1:
            pass
        yield i
        i = i + 1

for x in gen_with_pass(3):
    print(x)
