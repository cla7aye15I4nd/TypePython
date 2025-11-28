# Mixed type operations and conversions
def work_with_all_types(i: int, f: float, b: bool, s: str) -> int:
    result: int = i

    if b:
        result = result + 10

    if f > 5.0:
        result = result + 5

    return result

def boolean_to_numeric() -> int:
    t: bool = True
    f: bool = False

    count: int = 0
    if t:
        count = count + 1
    if not f:
        count = count + 1

    return count

def compare_mixed_types(i: int, f: float) -> int:
    count: int = 0

    if i > 5:
        count = count + 1
    if f > 5.0:
        count = count + 1
    if i == 10:
        count = count + 1

    return count

def complex_type_logic(x: int, y: float, flag: bool) -> float:
    result: float = 0.0

    if flag and x > 0:
        result = y * 2.0
    else:
        if not flag or x < 0:
            result = y + 10.0
        else:
            result = y

    return result

result1: int = work_with_all_types(10, 6.5, True, "test")
result2: int = boolean_to_numeric()
result3: int = compare_mixed_types(10, 7.5)
result4: float = complex_type_logic(5, 3.0, True)

print("Work with all types:", result1)
print("Boolean to numeric:", result2)
print("Compare mixed:", result3)
print("Complex type logic:", result4)
