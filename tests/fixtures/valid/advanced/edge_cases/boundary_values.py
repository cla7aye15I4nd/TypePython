# Boundary value testing
def test_one() -> int:
    a: int = 1 + 0
    b: int = 1 * 100
    c: int = 100 // 1
    d: int = 1 - 1

    return a + b + c + d

def test_small_numbers() -> int:
    sum: int = 0
    i: int = 1

    while i <= 1:
        sum = sum + i
        i = i + 1

    return sum

def single_iteration_loop() -> int:
    count: int = 0
    i: int = 0

    while i < 1:
        count = count + 1
        i = i + 1

    return count

def boundary_factorial(n: int) -> int:
    if n <= 1:
        return 1
    else:
        return n * boundary_factorial(n - 1)

result1: int = test_one()
result2: int = test_small_numbers()
result3: int = single_iteration_loop()
result4: int = boundary_factorial(1)
result5: int = boundary_factorial(2)

print(b"Test one:", result1)
print(b"Small numbers:", result2)
print(b"Single iteration:", result3)
print(b"Factorial(1):", result4)
print(b"Factorial(2):", result5)
