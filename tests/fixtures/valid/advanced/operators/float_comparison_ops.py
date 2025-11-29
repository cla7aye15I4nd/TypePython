# Test float comparison operations

# Float == Float
result1: bool = 1.0 == 1.0
print(result1)

result2: bool = 1.0 == 2.0
print(result2)

# Float != Float
result3: bool = 1.0 != 2.0
print(result3)

result4: bool = 1.0 != 1.0
print(result4)

# Float < Float
result5: bool = 1.0 < 2.0
print(result5)

result6: bool = 2.0 < 1.0
print(result6)

# Float <= Float
result7: bool = 1.0 <= 1.0
print(result7)

result8: bool = 2.0 <= 1.0
print(result8)

# Float > Float
result9: bool = 2.0 > 1.0
print(result9)

result10: bool = 1.0 > 2.0
print(result10)

# Float >= Float
result11: bool = 2.0 >= 2.0
print(result11)

result12: bool = 1.0 >= 2.0
print(result12)

# Float with Int comparisons
result13: bool = 1.5 > 1
print(result13)

result14: bool = 1.0 == 1
print(result14)

result15: bool = 2.5 < 3
print(result15)

result16: bool = 2.0 <= 2
print(result16)

result17: bool = 3.0 >= 3
print(result17)
