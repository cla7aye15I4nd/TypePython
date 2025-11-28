# Test different types of division

# Integer floor division
a: int = 17
b: int = 5
floor_div: int = a // b
print(b"17 // 5 =", floor_div)

# Integer floor division with exact result
c: int = 20
d: int = 4
exact_div: int = c // d
print(b"20 // 4 =", exact_div) # Corrected comment

# Integer division (returns int)
e: int = 100
f: int = 7
int_div: int = e // f
print(b"100 // 7 =", int_div)

# Negative floor division
g: int = -17
h: int = 5
neg_floor: int = g // h
print(b"-17 // 5 =", neg_floor)

# Float true division
x: float = 17.0
y: float = 5.0
true_div: float = x / y
print(b"17.0 / 5.0 =", true_div)

# Float floor division
floor_float: float = x // y
print(b"17.0 // 5.0 =", floor_float)

# Division resulting in repeating decimal - print as scaled value
m: float = 10.0
n: float = 3.0
repeat_div: float = m / n
repeat_div_scaled: float = repeat_div * 10000 // 1
print(b"10.0 / 3.0 * 10000 =", repeat_div_scaled)

# Division with small numbers
p: float = 1.0
q: float = 8.0
small_div: float = p / q
print(b"1.0 / 8.0 =", small_div)

# Combining divisions - print as scaled value
r: float = 100.0
s: float = 7.0
t: float = 2.0
combined: float = (r / s) / t
combined_scaled: float = combined * 10000 // 1
print(b"(100.0 / 7.0) / 2.0 * 10000 =", combined_scaled)

# Floor division of negative floats
u: float = -17.0
v: float = 5.0
neg_floor_float: float = u // v
print(b"-17.0 // 5.0 =", neg_floor_float)
