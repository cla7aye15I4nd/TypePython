# Test basic dict operations

# Create a simple dict
d: dict[int, int] = {1: 10, 2: 20, 3: 30}

# Access elements by key
print(d[1])
print(d[2])
print(d[3])

# Modify element
d[2] = 200
print(d[2])

# Add new element
d[4] = 40
print(d[4])

# Test len
print(len(d))
