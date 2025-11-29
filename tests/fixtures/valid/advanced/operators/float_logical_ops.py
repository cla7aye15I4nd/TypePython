# Test float 'and' and 'or' operators
# Python's and/or return actual values, not booleans

# Float and Float
result1: float = 3.5 and 2.1
print(result1)

result2: float = 0.0 and 5.5
print(result2)

result3: float = 3.5 and 0.0
print(result3)

result4: float = 0.0 and 0.0
print(result4)

# Float or Float
result5: float = 3.5 or 2.1
print(result5)

result6: float = 0.0 or 5.5
print(result6)

result7: float = 3.5 or 0.0
print(result7)

result8: float = 0.0 or 0.0
print(result8)

# Mixed and scenarios
result9: float = 1.5 and 2.5 and 3.5
print(result9)

result10: float = 1.5 and 0.0 and 3.5
print(result10)

# Mixed or scenarios
result11: float = 0.0 or 0.0 or 5.5
print(result11)

result12: float = 1.5 or 2.5 or 3.5
print(result12)
