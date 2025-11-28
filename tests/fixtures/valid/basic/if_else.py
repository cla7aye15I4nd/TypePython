# Simple if-else statements
def check_positive(n: int) -> int:
    if n > 0:
        return 1
    else:
        return 0

result1: int = check_positive(10)
result2: int = check_positive(-5)
result3: int = check_positive(0)

print(b"10 is positive:", result1)
print(b"-5 is positive:", result2)
print(b"0 is positive:", result3)
