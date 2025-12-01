# Iteration over a set
s: set[int] = {3, 1, 2}
total: int = 0
for x in s:
    total = total + x
print(total)
# Expected: 6 (sum of 1+2+3)
