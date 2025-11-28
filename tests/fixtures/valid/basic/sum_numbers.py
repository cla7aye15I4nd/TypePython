# Sum numbers from 1 to n
def sum_to_n(n: int) -> int:
    sum: int = 0
    i: int = 1
    while i <= n:
        sum = sum + i
        i = i + 1
    return sum

result: int = sum_to_n(10)
print(b"Sum from 1 to 10:", result)
