# Callback-like patterns with function pointers simulation
def apply_twice(n: int) -> int:
    return n * 2

def apply_thrice(n: int) -> int:
    return n * 3

def process_with_function(x: int, use_twice: bool) -> int:
    if use_twice:
        return apply_twice(x)
    else:
        return apply_thrice(x)

def map_values(a: int, b: int, c: int, double: bool) -> int:
    result: int = 0

    if double:
        result = apply_twice(a) + apply_twice(b) + apply_twice(c)
    else:
        result = apply_thrice(a) + apply_thrice(b) + apply_thrice(c)

    return result

result1: int = process_with_function(10, True)
result2: int = process_with_function(10, False)
result3: int = map_values(1, 2, 3, True)
result4: int = map_values(1, 2, 3, False)

print("Process twice:", result1)
print("Process thrice:", result2)
print("Map double:", result3)
print("Map triple:", result4)
