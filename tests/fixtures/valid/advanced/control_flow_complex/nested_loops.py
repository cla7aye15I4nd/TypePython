# Nested loop patterns
def multiplication_table(n: int) -> int:
    sum: int = 0
    i: int = 1
    while i <= n:
        j: int = 1
        while j <= n:
            sum = sum + (i * j)
            j = j + 1
        i = i + 1
    return sum

def triangle_pattern(n: int) -> int:
    sum: int = 0
    row: int = 1
    while row <= n:
        col: int = 1
        while col <= row:
            sum = sum + col
            col = col + 1
        row = row + 1
    return sum

def three_level_nesting(n: int) -> int:
    count: int = 0
    i: int = 1
    while i <= n:
        j: int = 1
        while j <= n:
            k: int = 1
            while k <= n:
                count = count + 1
                k = k + 1
            j = j + 1
        i = i + 1
    return count

result1: int = multiplication_table(5)
result2: int = triangle_pattern(10)
result3: int = three_level_nesting(3)

print("Multiplication table sum:", result1)
print("Triangle pattern sum:", result2)
print("Three level nesting count:", result3)
