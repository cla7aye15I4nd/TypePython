def triangular(n: int) -> int:
    return (n * (n + 1)) // 2

def pentagonal(n: int) -> int:
    return (n * (3 * n - 1)) // 2

def hexagonal(n: int) -> int:
    return n * (2 * n - 1)

def sum_sequence(n: int) -> int:
    sum: int = 0
    i: int = 1
    while i <= n:
        sum = sum + triangular(i)
        i = i + 1
    return sum
