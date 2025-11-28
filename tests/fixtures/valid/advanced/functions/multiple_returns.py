# Functions with multiple return paths
def abs_value(n: int) -> int:
    if n < 0:
        return -n
    else:
        return n

def max_of_three(a: int, b: int, c: int) -> int:
    if a >= b and a >= c:
        return a
    else:
        if b >= c:
            return b
        else:
            return c

def min_of_three(a: int, b: int, c: int) -> int:
    if a <= b and a <= c:
        return a
    else:
        if b <= c:
            return b
        else:
            return c

def sign(n: int) -> int:
    if n > 0:
        return 1
    else:
        if n < 0:
            return -1
        else:
            return 0

result1: int = abs_value(-42)
result2: int = abs_value(37)
result3: int = max_of_three(10, 25, 15)
result4: int = min_of_three(10, 25, 15)
result5: int = sign(100) + sign(-50) + sign(0)

print(b"Absolute value -42:", result1)
print(b"Absolute value 37:", result2)
print(b"Max of 10,25,15:", result3)
print(b"Min of 10,25,15:", result4)
print(b"Sign sum:", result5)
