# Test printing dict with int keys
d: dict[int, int] = {1: 10, 2: 20}
print(len(d))  # 2

# Empty int dict
d2: dict[int, int] = dict()
print(len(d2))  # 0

print("print int dict test passed!")
