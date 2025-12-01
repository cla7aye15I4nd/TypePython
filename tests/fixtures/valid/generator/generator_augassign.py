# Generator with augmented assignment - tests AugAssignment in generator
def gen_with_augassign(n: int) -> int:
    total: int = 0
    i: int = 0
    while i < n:
        total += i
        yield total
        i += 1

for val in gen_with_augassign(4):
    print(val)
