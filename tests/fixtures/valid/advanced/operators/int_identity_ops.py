# Test int identity operations

# Int is Int
result1: bool = 1 is 1
print(result1)

result2: bool = 0 is 0
print(result2)

# Int is not Int
result3: bool = 1 is not 2
print(result3)

result4: bool = 1 is not 1
print(result4)

# Negative numbers
result5: bool = -1 is -1
print(result5)

result6: bool = -1 is not -2
print(result6)

# Int logical and/or (returns int, not bool)
result7: int = 1 and 2
print(result7)

result8: int = 0 and 2
print(result8)

result9: int = 1 or 2
print(result9)

result10: int = 0 or 2
print(result10)
