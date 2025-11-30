# Test list() from dict with int keys
d: dict[int, int] = {1: 10, 2: 20, 3: 30}
keys: list[int] = list(d)
print(len(keys))

# Empty dict
d2: dict[int, int] = dict()
keys2: list[int] = list(d2)
print(len(keys2))

print("list from dict tests passed!")
