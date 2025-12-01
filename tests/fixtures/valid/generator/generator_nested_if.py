# Generator with nested if/else
def categorize(n: int) -> int:
    i: int = 0
    while i < n:
        if i % 3 == 0:
            if i % 2 == 0:
                yield i  # divisible by 6
        i = i + 1

for x in categorize(20):
    print(x)
