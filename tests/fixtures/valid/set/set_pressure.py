# Pressure test: set with 10^7 elements
n: int = 10000000

# Build set
s: set[int] = set()
i: int = 0
while i < n:
    s.add(i)
    i = i + 1

print(len(s))

# Check membership for first and last 1000 elements
c: int = 0
j: int = 0
while j < 1000:
    if j in s:
        c = c + 1
    if (n - 1 - j) in s:
        c = c + 1
    j = j + 1
print(c)

# Check non-existent elements
m: int = 0
k: int = n
while k < n + 1000:
    if k in s:
        m = m + 1
    k = k + 1
print(m)
