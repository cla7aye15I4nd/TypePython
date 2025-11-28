# Various sequence generators
def geometric_sum(a: int, r: int, n: int) -> int:
    sum: int = 0
    term: int = a
    i: int = 0

    while i < n:
        sum = sum + term
        term = term * r
        i = i + 1

    return sum

def arithmetic_sum(a: int, d: int, n: int) -> int:
    sum: int = 0
    term: int = a
    i: int = 0

    while i < n:
        sum = sum + term
        term = term + d
        i = i + 1

    return sum

def pell_number(n: int) -> int:
    if n == 0:
        return 0
    if n == 1:
        return 1

    p0: int = 0
    p1: int = 1
    i: int = 2

    while i <= n:
        current: int = 2 * p1 + p0
        p0 = p1
        p1 = current
        i = i + 1

    return p1

def lucas_number(n: int) -> int:
    if n == 0:
        return 2
    if n == 1:
        return 1

    l0: int = 2
    l1: int = 1
    i: int = 2

    while i <= n:
        current: int = l0 + l1
        l0 = l1
        l1 = current
        i = i + 1

    return l1

result1: int = geometric_sum(2, 3, 5)
result2: int = arithmetic_sum(5, 3, 10)
result3: int = pell_number(7)
result4: int = lucas_number(7)

print(b"Geometric sum:", result1)
print(b"Arithmetic sum:", result2)
print(b"Pell(7):", result3)
print(b"Lucas(7):", result4)
