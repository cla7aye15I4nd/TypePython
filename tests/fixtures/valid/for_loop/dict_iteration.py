# Iteration over dict (iterates over keys)
d: dict[int, int] = {1: 10, 2: 20, 3: 30}
total: int = 0
for k in d:
    total = total + k
print(total)
# Expected: 6 (sum of keys 1+2+3)
