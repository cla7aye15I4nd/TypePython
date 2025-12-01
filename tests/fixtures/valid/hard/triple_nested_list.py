# Test List[List[List[int]]] - 3-level nesting

# Create a 3D nested list (2x2x2 for simplicity)
cube: list[list[list[int]]] = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]

# Access at different levels
print(cube[0][0][0])  # 1
print(cube[0][0][1])  # 2
print(cube[0][1][0])  # 3
print(cube[0][1][1])  # 4
print(cube[1][0][0])  # 5
print(cube[1][1][1])  # 8

# Get a slice (2D matrix)
matrix: list[list[int]] = cube[1]
print(matrix[0][0])  # 5
print(matrix[1][1])  # 8

# Get a row (1D list)
row: list[int] = cube[0][1]
print(row[0])  # 3
print(row[1])  # 4

# Build nested structure with append
result: list[list[list[int]]] = []
for i in range(2):
    plane: list[list[int]] = []
    for j in range(2):
        line: list[int] = []
        for k in range(2):
            line.append(i * 4 + j * 2 + k)
        plane.append(line)
    result.append(plane)

# Verify
print(result[0][0][0])  # 0
print(result[0][0][1])  # 1
print(result[0][1][0])  # 2
print(result[1][0][0])  # 4
print(result[1][1][1])  # 7
