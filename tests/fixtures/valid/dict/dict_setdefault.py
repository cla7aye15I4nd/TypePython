# Test dict setdefault method

d: dict[int, int] = {1: 10, 2: 20}

# Key exists - returns existing value
v1: int = d.setdefault(1, 99)
print(v1)
print(d[1])

# Key doesn't exist - inserts and returns default
v2: int = d.setdefault(3, 30)
print(v2)
print(d[3])

# Verify key was inserted
print(len(d))
print(3 in d)

# Another new key
v3: int = d.setdefault(4, 40)
print(v3)
print(len(d))
