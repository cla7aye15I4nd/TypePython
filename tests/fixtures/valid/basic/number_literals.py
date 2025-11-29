# Test various number literal formats

# Binary literals
b1: int = 0b1010
print(b1)

b2: int = 0b1111
print(b2)

b3: int = 0B0001
print(b3)

# Octal literals
o1: int = 0o755
print(o1)

o2: int = 0o10
print(o2)

o3: int = 0O77
print(o3)

# Hexadecimal literals
h1: int = 0xFF
print(h1)

h2: int = 0xAB
print(h2)

h3: int = 0X10
print(h3)

h4: int = 0xdeadbeef
print(h4)

# Regular decimal
d1: int = 42
print(d1)

d2: int = 0
print(d2)

d3: int = 999
print(d3)

# Float literals
f1: float = 3.14
print(f1)

f2: float = 0.5
print(f2)

f3: float = 10.0
print(f3)
