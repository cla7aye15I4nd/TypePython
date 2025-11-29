# Test int-float comparison operations

# Int == Float
result1: bool = 1 == 1.0
print(result1)

result2: bool = 2 == 2.5
print(result2)

# Int != Float
result3: bool = 1 != 1.0
print(result3)

result4: bool = 2 != 2.5
print(result4)

# Int < Float
result5: bool = 1 < 1.5
print(result5)

result6: bool = 2 < 1.5
print(result6)

# Int <= Float
result7: bool = 1 <= 1.0
print(result7)

result8: bool = 2 <= 1.5
print(result8)

# Int > Float
result9: bool = 2 > 1.5
print(result9)

result10: bool = 1 > 1.5
print(result10)

# Int >= Float
result11: bool = 2 >= 2.0
print(result11)

result12: bool = 1 >= 1.5
print(result12)

# Negative numbers
result13: bool = -1 < 0.0
print(result13)

result14: bool = -2 > -1.5
print(result14)
