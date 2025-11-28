# Guard clause patterns
def validate_and_compute(x: int) -> int:
    if x < 0:
        return -1

    if x == 0:
        return 0

    if x > 100:
        return 100

    return x * 2

def safe_divide(a: int, b: int) -> int:
    if b == 0:
        return -1

    return a // b

def categorize_age(age: int) -> int:
    if age < 0:
        return 0

    if age < 13:
        return 1

    if age < 20:
        return 2

    if age < 65:
        return 3

    return 4

result1: int = validate_and_compute(50)
result2: int = validate_and_compute(-10)
result3: int = validate_and_compute(150)
result4: int = safe_divide(10, 2)
result5: int = safe_divide(10, 0)
result6: int = categorize_age(25)

print(b"Validate 50:", result1)
print(b"Validate -10:", result2)
print(b"Validate 150:", result3)
print(b"Safe divide 10/2:", result4)
print(b"Safe divide 10/0:", result5)
print(b"Categorize age 25:", result6)
