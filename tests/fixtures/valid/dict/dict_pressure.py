# Pressure test: dict with 10^7 elements
n: int = 10000000

# Build dict
d: dict[int, int] = {}
i: int = 0
while i < n:
    d[i] = i * 2
    i = i + 1

print(len(d))
print(d[0])
print(d[n - 1])
print(d[5000000])

# Check some values
c: int = 0
j: int = 0
while j < 1000:
    if d[j] == j * 2:
        c = c + 1
    j = j + 1
print(c)
