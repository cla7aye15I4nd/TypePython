# For loop over tuple
t: tuple[int, int, int, int, int] = (10, 20, 30, 40, 50)
total: int = 0
for x in t:
    total = total + x
print(total)
# Expected: 150
