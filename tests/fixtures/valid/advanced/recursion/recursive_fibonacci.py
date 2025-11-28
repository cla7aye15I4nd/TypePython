# Fibonacci using pure recursion (inefficient but tests recursion)
def fib_recursive(n: int) -> int:
    if n <= 1:
        return n
    else:
        return fib_recursive(n - 1) + fib_recursive(n - 2)

def sum_fibs(n: int) -> int:
    sum: int = 0
    i: int = 0
    while i <= n:
        sum = sum + fib_recursive(i)
        i = i + 1
    return sum

result1: int = fib_recursive(10)
result2: int = sum_fibs(10)

print("Fib(10):", result1)
print("Sum of first 11 fibs:", result2)
print("Fib(12):", fib_recursive(12))
