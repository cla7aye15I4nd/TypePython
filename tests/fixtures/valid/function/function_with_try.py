# Functions containing try/except - exercises contains_yield for Try statement
def safe_divide(a: int, b: int) -> int:
    result: int = 0
    try:
        result = a + b  # Division not supported yet, using addition
    except:
        result = -1
    return result

print(safe_divide(10, 5))

def with_finally(x: int) -> int:
    y: int = 0
    try:
        y = x * 2
    finally:
        y = y + 1
    return y

print(with_finally(5))
