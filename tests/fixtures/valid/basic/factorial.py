# Factorial function using recursion
def factorial(n: int) -> int:
    if n <= 1:
        return 1
    else:
        return n * factorial(n - 1)

# Main execution
result: int = factorial(5)
print(b"Factorial of 5:", result)
