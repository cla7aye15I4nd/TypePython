# Tuple iteration with for loop
t: tuple[int, int, int, int] = (1, 2, 3, 4)
total: int = 0
for x in t:
    total += x
print(total)
# Expected output: 10
