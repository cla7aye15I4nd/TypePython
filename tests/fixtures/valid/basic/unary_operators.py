# Test unary operators

# Integer negation
a: int = 42
neg_a: int = -a
print(b"Negation of 42:", neg_a)

# Double negation
double_neg: int = -(-a)
print(b"Double negation of 42:", double_neg)

# Positive operator (no-op but valid)
pos_a: int = +a
print(b"Positive of 42:", pos_a)

# Negation of negative
b: int = -10
neg_b: int = -b
print(b"Negation of -10:", neg_b)

# Float negation
x: float = 3.14
neg_x: float = -x
print(b"Negation of 3.14:", neg_x)

# Float positive
pos_x: float = +x
print(b"Positive of 3.14:", pos_x)

# Boolean NOT
t: bool = True
f: bool = False
not_t: bool = not t
not_f: bool = not f
print(b"not True:", not_t)
print(b"not False:", not_f)

# Double NOT
double_not: bool = not not t
print(b"not not True:", double_not)

# Bitwise NOT on integers
c: int = 0
bitnot_c: int = ~c
print(b"~0 =", bitnot_c)

d: int = -1
bitnot_d: int = ~d
print(b"~(-1) =", bitnot_d)

# Combining unary operators
e: int = 5
combined: int = -(-e) + (+e)
print(b"-(-5) + (+5) =", combined)
