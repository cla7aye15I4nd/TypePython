# Test bool with float operations (bool coerced to int, then to float)

# Bool + Float
result1: float = True + 1.5
print(result1)

result2: float = False + 2.5
print(result2)

# Bool - Float
result3: float = True - 0.5
print(result3)

result4: float = False - 1.0
print(result4)

# Bool * Float
result5: float = True * 2.5
print(result5)

result6: float = False * 100.0
print(result6)

# Bool / Float
result7: float = True / 2.0
print(result7)

result8: float = True / 0.5
print(result8)

# Bool // Float (floor division)
result9: float = True // 0.3
print(result9)

# Bool % Float (modulo)
result10: float = True % 0.3
print(result10)

# Bool ** Float (power)
result11: float = True ** 0.5
print(result11)

result12: float = False ** 2.0
print(result12)
