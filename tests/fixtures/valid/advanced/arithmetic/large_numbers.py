# Large number calculations
def factorial_iterative(n: int) -> int:
    result: int = 1
    i: int = 2
    while i <= n:
        result = result * i
        i = i + 1
    return result

def sum_of_squares(n: int) -> int:
    sum: int = 0
    i: int = 1
    while i <= n:
        sum = sum + (i * i)
        i = i + 1
    return sum

def sum_of_cubes(n: int) -> int:
    sum: int = 0
    i: int = 1
    while i <= n:
        sum = sum + (i * i * i)
        i = i + 1
    return sum

fact10: int = factorial_iterative(10)
squares20: int = sum_of_squares(20)
cubes10: int = sum_of_cubes(10)

print("10! =", fact10)
print("Sum of squares 1-20:", squares20)
print("Sum of cubes 1-10:", cubes10)
