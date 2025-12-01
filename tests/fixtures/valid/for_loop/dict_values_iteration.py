# Iteration over dict.values()
d: dict[int, int] = {1: 10, 2: 20, 3: 30}
total: int = 0
for v in d.values():
    total = total + v
print(total)
# Expected: 60 (sum of values 10+20+30)
