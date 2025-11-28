# Various parameter passing patterns
def sum_two(a: int, b: int) -> int:
    return a + b

def sum_three(a: int, b: int, c: int) -> int:
    return a + b + c

def sum_four(a: int, b: int, c: int, d: int) -> int:
    return a + b + c + d

def sum_five(a: int, b: int, c: int, d: int, e: int) -> int:
    return a + b + c + d + e

def compute(x: int, y: int, z: int) -> int:
    temp1: int = x * y
    temp2: int = y * z
    temp3: int = x * z
    return temp1 + temp2 + temp3

def mixed_types_func(i: int, f: float, b: bool) -> int:
    result: int = i
    if b:
        result = result + 10
    return result

result1: int = sum_two(5, 10)
result2: int = sum_three(1, 2, 3)
result3: int = sum_four(10, 20, 30, 40)
result4: int = sum_five(1, 1, 1, 1, 1)
result5: int = compute(2, 3, 4)
result6: int = mixed_types_func(100, 3.14, True)

print("Sum two:", result1)
print("Sum three:", result2)
print("Sum four:", result3)
print("Sum five:", result4)
print("Compute:", result5)
print("Mixed types:", result6)
