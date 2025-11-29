# Test unary operations on integers

# Unary negation
result1: int = -5
print(result1)

result2: int = -(-5)
print(result2)

# Unary positive
result3: int = +5
print(result3)

result4: int = +(-5)
print(result4)

# Unary NOT (bitwise)
result5: int = ~0
print(result5)

result6: int = ~1
print(result6)

result7: int = ~(-1)
print(result7)

# Combined unary operations
result8: int = -~5
print(result8)

result9: int = ~-5
print(result9)

# Unary on expressions
result10: int = -(5 + 3)
print(result10)

result11: int = ~(8 - 1)
print(result11)
