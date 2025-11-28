# Float type operations and precision
def test_float_arithmetic() -> float:
    a: float = 10.5
    b: float = 3.2
    sum: float = a + b
    diff: float = a - b
    prod: float = a * b
    return sum + diff + prod

def test_float_comparisons() -> int:
    x: float = 10.0
    y: float = 10.5
    z: float = 9.5

    count: int = 0
    if x > z:
        count = count + 1
    if y > x:
        count = count + 1
    if x + z == 19.5:
        count = count + 1
    if y - x == 0.5:
        count = count + 1

    return count

def mixed_int_float(i: int, f: float) -> float:
    result1: float = 1.0
    result2: float = 2.0
    if i > 5:
        result1 = f * 2.0
    else:
        result2 = f + 10.0
    return result1 + result2

result1: float = test_float_arithmetic()
result2: int = test_float_comparisons()
result3: float = mixed_int_float(10, 3.5)

print(b"Float arithmetic:", result1)
print(b"Float comparisons:", result2)
print(b"Mixed int float:", result3)
