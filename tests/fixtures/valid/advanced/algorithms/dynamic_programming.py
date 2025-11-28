# Dynamic programming patterns
def fibonacci_dp(n: int) -> int:
    if n <= 1:
        return n

    prev2: int = 0
    prev1: int = 1
    i: int = 2

    while i <= n:
        current: int = prev1 + prev2
        prev2 = prev1
        prev1 = current
        i = i + 1

    return prev1

def climbing_stairs(n: int) -> int:
    # Ways to climb n stairs (1 or 2 steps at a time)
    if n <= 2:
        return n

    dp1: int = 1
    dp2: int = 2
    i: int = 3

    while i <= n:
        current: int = dp1 + dp2
        dp1 = dp2
        dp2 = current
        i = i + 1

    return dp2

def tribonacci(n: int) -> int:
    if n == 0:
        return 0
    if n <= 2:
        return 1

    t0: int = 0
    t1: int = 1
    t2: int = 1
    i: int = 3

    while i <= n:
        current: int = t0 + t1 + t2
        t0 = t1
        t1 = t2
        t2 = current
        i = i + 1

    return t2

result1: int = fibonacci_dp(20)
result2: int = climbing_stairs(10)
result3: int = tribonacci(10)

print("Fibonacci(20):", result1)
print("Climbing stairs(10):", result2)
print("Tribonacci(10):", result3)
