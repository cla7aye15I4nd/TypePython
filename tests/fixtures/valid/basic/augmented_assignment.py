# Test augmented assignment operators

# Addition assignment
a: int = 10
a += 5
print(b"10 += 5:", a)

# Subtraction assignment
b: int = 20
b -= 8
print(b"20 -= 8:", b)

# Multiplication assignment
c: int = 6
c *= 7
print(b"6 *= 7:", c)

# Floor division assignment
d: int = 100
d //= 9
print(b"100 //= 9:", d)

# Modulo assignment
e: int = 17
e %= 5
print(b"17 %= 5:", e)

# Power assignment
f: int = 2
f **= 8
print(b"2 **= 8:", f)

# Bitwise OR assignment
g: int = 12
g |= 3
print(b"12 |= 3:", g)

# Bitwise AND assignment
h: int = 15
h &= 9
print(b"15 &= 9:", h)

# Bitwise XOR assignment
i: int = 10
i ^= 7
print(b"10 ^= 7:", i)

# Left shift assignment
j: int = 4
j <<= 3
print(b"4 <<= 3:", j)

# Right shift assignment
k: int = 64
k >>= 2
print(b"64 >>= 2:", k)

# Float augmented assignments
x: float = 10.5
x += 2.5
print(b"10.5 += 2.5:", x)

y: float = 20.0
y -= 5.5
print(b"20.0 -= 5.5:", y)

z: float = 3.0
z *= 4.0
print(b"3.0 *= 4.0:", z)

w: float = 15.0
w /= 4.0
print(b"15.0 /= 4.0:", w)

# Float floor division
ff: float = 17.0
ff //= 5.0
print(b"17.0 //= 5.0:", ff)

# Float modulo
fm: float = 10.0
fm %= 3.0
print(b"10.0 %= 3.0:", fm)

# Float power
fp: float = 2.0
fp **= 3.0
print(b"2.0 **= 3.0:", fp)

# Chained augmented assignment in loop
counter: int = 0
i2: int = 0
while i2 < 5:
    counter += i2
    i2 += 1
print(b"Sum 0+1+2+3+4:", counter)
