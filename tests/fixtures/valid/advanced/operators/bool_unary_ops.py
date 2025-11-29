# Test bool unary operators that coerce to int

# Bitwise NOT on bool (coerces to int)
result1: int = ~True
print(result1)

result2: int = ~False
print(result2)

# Unary minus on bool (coerces to int)
result3: int = -True
print(result3)

result4: int = -False
print(result4)

# Unary plus on bool (coerces to int)
result5: int = +True
print(result5)

result6: int = +False
print(result6)

# Logical NOT on bool (stays bool)
result7: bool = not True
print(result7)

result8: bool = not False
print(result8)
