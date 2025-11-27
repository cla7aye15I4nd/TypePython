# Simple if-else statements
def check_positive(n: int) -> int:
    if n > 0:
        return 1
    else:
        return 0

result1: int = check_positive(10)
result2: int = check_positive(-5)
result3: int = check_positive(0)

print("10 is positive:", result1)
print("-5 is positive:", result2)
print("0 is positive:", result3)
