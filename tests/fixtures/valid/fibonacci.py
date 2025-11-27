# Fibonacci sequence using iteration
def fibonacci(n: int) -> int:
    if n <= 1:
        return n
    else:
        a: int = 0
        b: int = 1
        i: int = 2
        while i <= n:
            temp: int = a + b
            a = b
            b = temp
            i = i + 1
        return b

# Calculate 10th fibonacci number
fib10: int = fibonacci(10)
print("10th Fibonacci number:", fib10)
