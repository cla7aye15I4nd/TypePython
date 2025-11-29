# Test comparison operations with mixed types (int and float)

# Int == Float
result1: bool = 5 == 5.0
print(result1)

result2: bool = 5 == 5.5
print(result2)

# Int != Float
result3: bool = 5 != 5.5
print(result3)

result4: bool = 5 != 5.0
print(result4)

# Int < Float
result5: bool = 5 < 5.5
print(result5)

result6: bool = 5 < 4.5
print(result6)

# Int <= Float
result7: bool = 5 <= 5.0
print(result7)

result8: bool = 5 <= 4.5
print(result8)

# Int > Float
result9: bool = 5 > 4.5
print(result9)

result10: bool = 5 > 5.5
print(result10)

# Int >= Float
result11: bool = 5 >= 5.0
print(result11)

result12: bool = 5 >= 5.5
print(result12)

# Float == Int
result13: bool = 5.0 == 5
print(result13)

# Float < Int
result14: bool = 4.5 < 5
print(result14)
