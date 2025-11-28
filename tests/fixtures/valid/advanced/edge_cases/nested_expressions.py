# Deeply nested expressions
def test_nested_arithmetic() -> int:
    result: int = (((10 + 5) * 2 - 3) * ((8 - 2) + 4)) + (((20 - 5) * 3) // 9)
    return result

def test_nested_boolean() -> int:
    count: int = 0

    if ((True and (False or True)) and (not False)):
        count = count + 1

    if (((10 > 5) and (5 < 10)) or ((10 == 10) and (5 != 10))):
        count = count + 1

    if not ((False and True) or (False and False)):
        count = count + 1

    return count

def nested_function_calls(x: int) -> int:
    return ((x + 1) * 2 - 3) * ((x - 1) + 4)

def compute_nested(a: int, b: int) -> int:
    result1: int = nested_function_calls(a + b)
    result2: int = nested_function_calls(a - b)
    result3: int = nested_function_calls(a * b)
    return result1 + result2 + result3

result1: int = test_nested_arithmetic()
result2: int = test_nested_boolean()
result3: int = compute_nested(5, 3)

print("Nested arithmetic:", result1)
print("Nested boolean:", result2)
print("Compute nested:", result3)
