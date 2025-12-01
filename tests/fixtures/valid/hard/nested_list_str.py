# Test List[List[str]] nested type support

# Create a nested list of strings
grid: list[list[str]] = [["a", "b", "c"], ["d", "e", "f"]]

# Access elements
print(grid[0][0])  # a
print(grid[0][2])  # c
print(grid[1][1])  # e

# Build with append
result: list[list[str]] = []
words: list[str] = ["hello", "world"]
result.append(words)
result.append(["foo", "bar"])

print(result[0][0])  # hello
print(result[0][1])  # world
print(result[1][0])  # foo
print(result[1][1])  # bar
