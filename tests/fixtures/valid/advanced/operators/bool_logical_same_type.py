# Test bool 'and' and 'or' operators with same types
# Python's and/or return actual values, not booleans

# Bool and Bool
result1: bool = True and True
print(result1)

result2: bool = True and False
print(result2)

result3: bool = False and True
print(result3)

result4: bool = False and False
print(result4)

# Bool or Bool
result5: bool = True or True
print(result5)

result6: bool = True or False
print(result6)

result7: bool = False or True
print(result7)

result8: bool = False or False
print(result8)

# Int and Int
result9: int = 5 and 10
print(result9)

result10: int = 0 and 10
print(result10)

result11: int = 5 and 0
print(result11)

# Int or Int
result12: int = 5 or 10
print(result12)

result13: int = 0 or 10
print(result13)

result14: int = 5 or 0
print(result14)
