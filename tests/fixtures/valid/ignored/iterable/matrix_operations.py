# Test iteration patterns in matrix operations

# Create identity matrix
def identity(n: int) -> list[list[int]]:
    result: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(n):
            if i == j:
                row.append(1)
            else:
                row.append(0)
        result.append(row)
    return result

print(b"Identity 3x3:")
for row in identity(3):
    print(row)

# Create zero matrix
def zeros(rows: int, cols: int) -> list[list[int]]:
    result: list[list[int]] = []
    for _ in range(rows):
        row: list[int] = []
        for _ in range(cols):
            row.append(0)
        result.append(row)
    return result

print(b"Zeros 2x4:")
for row in zeros(2, 4):
    print(row)

# Matrix addition
def add_matrices(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    rows: int = len(a)
    cols: int = len(a[0])
    result: list[list[int]] = []
    for i in range(rows):
        row: list[int] = []
        for j in range(cols):
            row.append(a[i][j] + b[i][j])
        result.append(row)
    return result

m1: list[list[int]] = [[1, 2], [3, 4]]
m2: list[list[int]] = [[5, 6], [7, 8]]
print(b"Matrix addition:")
for row in add_matrices(m1, m2):
    print(row)

# Matrix multiplication
def multiply_matrices(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
    rows_a: int = len(a)
    cols_a: int = len(a[0])
    cols_b: int = len(b[0])
    result: list[list[int]] = []
    for i in range(rows_a):
        row: list[int] = []
        for j in range(cols_b):
            sum_val: int = 0
            for k in range(cols_a):
                sum_val = sum_val + a[i][k] * b[k][j]
            row.append(sum_val)
        result.append(row)
    return result

print(b"Matrix multiplication:")
for row in multiply_matrices(m1, m2):
    print(row)

# Transpose
def transpose(m: list[list[int]]) -> list[list[int]]:
    rows: int = len(m)
    cols: int = len(m[0])
    result: list[list[int]] = []
    for j in range(cols):
        row: list[int] = []
        for i in range(rows):
            row.append(m[i][j])
        result.append(row)
    return result

m3: list[list[int]] = [[1, 2, 3], [4, 5, 6]]
print(b"Transpose:")
for row in transpose(m3):
    print(row)

# Scalar multiplication
def scalar_mult(m: list[list[int]], k: int) -> list[list[int]]:
    result: list[list[int]] = []
    for row in m:
        new_row: list[int] = []
        for val in row:
            new_row.append(val * k)
        result.append(new_row)
    return result

print(b"Scalar mult x3:")
for row in scalar_mult(m1, 3):
    print(row)

# Row sum
def row_sums(m: list[list[int]]) -> list[int]:
    result: list[int] = []
    for row in m:
        total: int = 0
        for val in row:
            total = total + val
        result.append(total)
    return result

print(b"Row sums:", row_sums(m3))

# Column sum
def col_sums(m: list[list[int]]) -> list[int]:
    cols: int = len(m[0])
    result: list[int] = []
    for j in range(cols):
        total: int = 0
        for row in m:
            total = total + row[j]
        result.append(total)
    return result

print(b"Column sums:", col_sums(m3))

# Diagonal sum
def diagonal_sum(m: list[list[int]]) -> int:
    total: int = 0
    for i in range(len(m)):
        total = total + m[i][i]
    return total

print(b"Diagonal sum:", diagonal_sum(m1))

# Anti-diagonal sum
def anti_diagonal_sum(m: list[list[int]]) -> int:
    n: int = len(m)
    total: int = 0
    for i in range(n):
        total = total + m[i][n - 1 - i]
    return total

print(b"Anti-diagonal sum:", anti_diagonal_sum(m1))

# Flatten matrix
def flatten(m: list[list[int]]) -> list[int]:
    result: list[int] = []
    for row in m:
        for val in row:
            result.append(val)
    return result

print(b"Flattened:", flatten(m3))

# Reshape
def reshape(flat: list[int], rows: int, cols: int) -> list[list[int]]:
    result: list[list[int]] = []
    idx: int = 0
    for _ in range(rows):
        row: list[int] = []
        for _ in range(cols):
            row.append(flat[idx])
            idx = idx + 1
        result.append(row)
    return result

flat: list[int] = [1, 2, 3, 4, 5, 6]
print(b"Reshaped 2x3:")
for row in reshape(flat, 2, 3):
    print(row)

# Rotate 90 degrees clockwise
def rotate_90(m: list[list[int]]) -> list[list[int]]:
    n: int = len(m)
    result: list[list[int]] = []
    for j in range(n):
        row: list[int] = []
        for i in range(n - 1, -1, -1):
            row.append(m[i][j])
        result.append(row)
    return result

m4: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
print(b"Rotated 90:")
for row in rotate_90(m4):
    print(row)

# Spiral traversal
def spiral(m: list[list[int]]) -> list[int]:
    if len(m) == 0:
        return []

    result: list[int] = []
    top: int = 0
    bottom: int = len(m) - 1
    left: int = 0
    right: int = len(m[0]) - 1

    while top <= bottom and left <= right:
        for j in range(left, right + 1):
            result.append(m[top][j])
        top = top + 1

        for i in range(top, bottom + 1):
            result.append(m[i][right])
        right = right - 1

        if top <= bottom:
            for j in range(right, left - 1, -1):
                result.append(m[bottom][j])
            bottom = bottom - 1

        if left <= right:
            for i in range(bottom, top - 1, -1):
                result.append(m[i][left])
            left = left + 1

    return result

print(b"Spiral:", spiral(m4))

# Find element in sorted matrix
def search_matrix(m: list[list[int]], target: int) -> tuple[int, int]:
    if len(m) == 0:
        return (-1, -1)
    rows: int = len(m)
    cols: int = len(m[0])
    row: int = 0
    col: int = cols - 1

    while row < rows and col >= 0:
        if m[row][col] == target:
            return (row, col)
        elif m[row][col] > target:
            col = col - 1
        else:
            row = row + 1

    return (-1, -1)

sorted_m: list[list[int]] = [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
print(b"Search 5:", search_matrix(sorted_m, 5))
print(b"Search 10:", search_matrix(sorted_m, 10))
