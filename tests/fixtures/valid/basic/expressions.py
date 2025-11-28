# Test various expression types

# Arithmetic expressions
sum: int = 10 + 20
diff: int = 50 - 15
product: int = 6 * 7
quotient: int = 100 // 5
remainder: int = 17 % 5

# Complex arithmetic with precedence
complex_calc: int = 2 + 3 * 4 - 10 // 2

# Boolean expressions
true_val: bool = True
false_val: bool = False
and_result: bool = True and False
or_result: bool = True or False
not_result: bool = not True

# Comparison expressions
gt: bool = 10 > 5
lt: bool = 3 < 7
ge: bool = 10 >= 10
le: bool = 5 <= 5
eq: bool = 42 == 42
ne: bool = 10 != 20

# Combined logical expressions
complex_bool: bool = 5 > 3 and 10 < 20 or not False

# Parenthesized expressions
paren_calc: int = (2 + 3) * (4 + 5)

# Unary operators
negative: int = -42
negated_bool: bool = not True

# Float expressions
pi: float = 3.14159
radius: float = 5.0
area: float = pi * radius * radius

# Print results
print(b"Sum:", sum)
print(b"Diff:", diff)
print(b"Product:", product)
print(b"Quotient:", quotient)
print(b"Remainder:", remainder)
print(b"Complex calc:", complex_calc)
print(b"And result:", and_result)
print(b"Or result:", or_result)
print(b"Not result:", not_result)
print(b"Greater than:", gt)
print(b"Less than:", lt)
print(b"Complex bool:", complex_bool)
print(b"Paren calc:", paren_calc)
print(b"Negative:", negative)
print(b"Area:", area)
