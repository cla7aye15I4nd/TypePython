# Deep recursion tests
def sum_recursive(n: int) -> int:
    if n <= 0:
        return 0
    else:
        return n + sum_recursive(n - 1)

def product_recursive(n: int) -> int:
    if n <= 1:
        return 1
    else:
        return n * product_recursive(n - 1)

def power_recursive(base: int, exp: int) -> int:
    if exp == 0:
        return 1
    else:
        return base * power_recursive(base, exp - 1)

def sum_range(start: int, end: int) -> int:
    if start > end:
        return 0
    else:
        return start + sum_range(start + 1, end)

result1: int = sum_recursive(10)
result2: int = product_recursive(5)
result3: int = power_recursive(2, 10)
result4: int = sum_range(1, 20)

print(b"Sum recursive 1-10:", result1)
print(b"Product recursive 1-5:", result2)
print(b"Power 2^10:", result3)
print(b"Sum range 1-20:", result4)
