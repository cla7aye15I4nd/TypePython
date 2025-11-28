# Test type coercion between types

# Int to float in expressions
a: int = 5
b: float = 2.5

# When used in mixed-type context via function
def add_int_to_float(i: int, f: float) -> float:
    result: float = f + f
    return result

result1: float = add_int_to_float(5, 2.5)
print(b"Float addition result:", result1)

# Boolean to int conversion scenarios
def bool_math(flag: bool) -> int:
    count: int = 0
    if flag:
        count = 1
    else:
        count = 0
    return count

bool_as_int: int = bool_math(True)
print(b"True as count:", bool_as_int)

bool_as_int2: int = bool_math(False)
print(b"False as count:", bool_as_int2)

# Int arithmetic producing large values
big1: int = 1000000
big2: int = 1000000
big_result: int = big1 * big2
print(b"1000000 * 1000000 =", big_result)

# Float precision test - print as scaled value to avoid precision issues
precise: float = 0.1
sum_floats: float = precise + precise + precise
sum_floats_scaled: float = sum_floats * 10000 // 1
print(b"0.1 + 0.1 + 0.1 * 10000 =", sum_floats_scaled)

# Comparing different magnitudes
small: float = 0.0001
large: float = 10000.0
ratio: float = small * large
print(b"0.0001 * 10000.0 =", ratio)

# Bool in conditional always works
flag: bool = True
x: int = 10
if flag:
    x = 20
print(b"After bool conditional:", x)

# Multiple type usage in function
def process_types(i: int, f: float, b: bool) -> int:
    result: int = i
    if b:
        result = result + 10
    if f > 1.0:
        result = result + 5
    return result

final: int = process_types(100, 2.5, True)
print(b"Process types result:", final)
