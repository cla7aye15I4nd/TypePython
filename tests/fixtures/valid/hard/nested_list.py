# Test nested list support: List[List[int]]

# Create a nested list
nested: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

# Access outer list elements
first_row: list[int] = nested[0]
print(first_row[0])  # 1
print(first_row[1])  # 2
print(first_row[2])  # 3

# Access nested elements directly
print(nested[1][0])  # 4
print(nested[1][1])  # 5
print(nested[2][2])  # 9

# Iterate over nested list
for row in nested:
    for item in row:
        print(item)

# Create nested list with append
result: list[list[int]] = []
for i in range(3):
    inner: list[int] = []
    for j in range(3):
        inner.append(i * 3 + j)
    result.append(inner)

# Verify appended nested list
print(result[0][0])  # 0
print(result[1][1])  # 4
print(result[2][2])  # 8
