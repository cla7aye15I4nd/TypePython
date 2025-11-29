# Pressure test: list with 10^7 elements
n: int = 10000000

# Build list by appending
x: list[int] = []
i: int = 0
while i < n:
    x.append(i)
    i = i + 1

print(len(x))
print(x[0])
print(x[n - 1])
print(x[5000000])

# Sum first and last 1000 elements
s: int = 0
j: int = 0
while j < 1000:
    s = s + x[j] + x[n - 1 - j]
    j = j + 1
print(s)
