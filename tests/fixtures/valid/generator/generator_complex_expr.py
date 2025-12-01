# Generator with complex expressions - tests expression evaluation in yield
def gen_complex(n: int) -> int:
    i: int = 0
    while i < n:
        # Complex expression in yield
        result: int = (i * 2) + 1
        yield result
        i = i + 1

for x in gen_complex(3):
    print(x)
