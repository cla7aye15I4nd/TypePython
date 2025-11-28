# Zero handling and edge cases
def test_zero_arithmetic() -> int:
    a: int = 0 + 5
    b: int = 10 - 0
    c: int = 0 * 100
    d: int = 0 // 5

    return a + b + c + d

def test_zero_comparisons() -> int:
    count: int = 0

    if 0 == 0:
        count = count + 1
    if 0 < 1:
        count = count + 1
    if 0 <= 0:
        count = count + 1
    if 0 != 1:
        count = count + 1

    return count

def test_zero_in_loops() -> int:
    sum: int = 0
    i: int = 0

    while i < 0:
        sum = sum + i
        i = i + 1

    return sum

def factorial_with_zero(n: int) -> int:
    if n <= 0:
        return 1
    else:
        return n * factorial_with_zero(n - 1)

result1: int = test_zero_arithmetic()
result2: int = test_zero_comparisons()
result3: int = test_zero_in_loops()
result4: int = factorial_with_zero(0)
result5: int = factorial_with_zero(5)

print("Zero arithmetic:", result1)
print("Zero comparisons:", result2)
print("Zero in loops:", result3)
print("Factorial(0):", result4)
print("Factorial(5):", result5)
