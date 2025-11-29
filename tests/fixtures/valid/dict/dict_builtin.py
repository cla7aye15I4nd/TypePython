# Test dict() builtin function

# Create empty dict using dict() builtin
empty: dict[int, int] = dict()
print(len(empty))

# Add some key-value pairs
empty[1] = 100
empty[2] = 200
empty[3] = 300

print(len(empty))
print(empty[1])
print(empty[2])
print(empty[3])
