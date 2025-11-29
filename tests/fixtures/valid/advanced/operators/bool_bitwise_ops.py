# Test bitwise operations on booleans

# Bool bitwise AND
result1: bool = True & True
print(result1)

result2: bool = True & False
print(result2)

result3: bool = False & False
print(result3)

# Bool bitwise OR
result4: bool = True | True
print(result4)

result5: bool = True | False
print(result5)

result6: bool = False | False
print(result6)

# Bool bitwise XOR
result7: bool = True ^ True
print(result7)

result8: bool = True ^ False
print(result8)

result9: bool = False ^ False
print(result9)

# Bool with Int bitwise ops
result10: int = True & 5
print(result10)

result11: int = True | 4
print(result11)

result12: int = False ^ 7
print(result12)

# Bool logical AND/OR
result13: bool = True and True
print(result13)

result14: bool = True and False
print(result14)

result15: bool = True or False
print(result15)

result16: bool = False or False
print(result16)
