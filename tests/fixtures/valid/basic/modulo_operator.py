# Test modulo operator (%)

# Integer modulo
a: int = 17
b: int = 5
result1: int = a % b
print(b"17 % 5 =", result1)

# Modulo with zero remainder
c: int = 20
d: int = 4
result2: int = c % d
print(b"20 % 4 =", result2)

# Modulo with larger divisor
e: int = 3
f: int = 7
result3: int = e % f
print(b"3 % 7 =", result3)

# Negative number modulo
g: int = -17
h: int = 5
result4: int = g % h
print(b"-17 % 5 =", result4)

# Modulo in loop (common use case)
i: int = 0
count: int = 0
while i < 10:
    if i % 2 == 0:
        count = count + 1
    i = i + 1
print(b"Even numbers 0-9:", count)

# Float modulo
x: float = 7.5
y: float = 2.5
result5: float = x % y
print(b"7.5 % 2.5 =", result5)

# Float modulo with non-exact division
m: float = 10.0
n: float = 3.0
result6: float = m % n
print(b"10.0 % 3.0 =", result6)
