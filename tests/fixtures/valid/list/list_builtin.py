# Test list() builtin function

# Create empty list using list() builtin
empty: list[int] = list()
print(len(empty))

# Add some elements
empty.append(10)
empty.append(20)
empty.append(30)

print(len(empty))
print(empty[0])
print(empty[1])
print(empty[2])
