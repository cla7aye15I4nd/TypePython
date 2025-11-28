# Deeply nested conditional statements
def classify_number(n: int) -> int:
    result: int = 0
    if n > 0:
        if n < 10:
            if n % 2 == 0:
                result = 1
            else:
                result = 2
        else:
            if n < 100:
                if n % 10 == 0:
                    result = 3
                else:
                    result = 4
            else:
                result = 5
    else:
        if n < 0:
            if n > -10:
                result = 6
            else:
                result = 7
        else:
            result = 8
    return result

def test_all_paths() -> int:
    sum: int = 0
    sum = sum + classify_number(5)
    sum = sum + classify_number(8)
    sum = sum + classify_number(15)
    sum = sum + classify_number(50)
    sum = sum + classify_number(150)
    sum = sum + classify_number(-5)
    sum = sum + classify_number(-15)
    sum = sum + classify_number(0)
    return sum

result: int = test_all_paths()
print(b"Nested conditionals result:", result)
