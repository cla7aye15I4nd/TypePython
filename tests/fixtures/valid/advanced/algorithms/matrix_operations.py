# Matrix operations (simulated with scalars)
def matrix_2x2_determinant(a: int, b: int, c: int, d: int) -> int:
    return a * d - b * c

def matrix_multiply_2x2_scalar(a: int, b: int, c: int, d: int, scalar: int) -> int:
    # Returns sum of all elements after multiplication
    return (a + b + c + d) * scalar

def matrix_transpose_sum(a: int, b: int, c: int, d: int) -> int:
    # For 2x2 matrix [[a, b], [c, d]], transpose is [[a, c], [b, d]]
    # Return sum showing they're different arrangements
    original: int = (a + b) * 10 + (c + d)
    transposed: int = (a + c) * 10 + (b + d)
    return original + transposed

result1: int = matrix_2x2_determinant(3, 4, 2, 5)
result2: int = matrix_multiply_2x2_scalar(1, 2, 3, 4, 5)
result3: int = matrix_transpose_sum(1, 2, 3, 4)

print(b"Matrix determinant:", result1)
print(b"Matrix scalar multiply:", result2)
print(b"Matrix transpose sum:", result3)
