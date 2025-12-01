# Generator used in for loop
def count(n: int) -> int:
    i: int = 0
    while i < n:
        yield i
        i = i + 1

total: int = 0
for x in count(5):
    total = total + x
print(total)
# Expected: 10 (0+1+2+3+4)
