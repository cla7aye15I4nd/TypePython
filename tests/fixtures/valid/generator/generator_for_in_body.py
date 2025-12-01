# Generator with for loop in body
def squares(n: int) -> int:
    for i in range(n):
        yield i * i

for x in squares(5):
    print(x)
