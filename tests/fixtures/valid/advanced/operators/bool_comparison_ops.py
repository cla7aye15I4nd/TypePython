# Test comparison operations on booleans

# Bool equality
result1: bool = True == True
print(result1)

result2: bool = True == False
print(result2)

result3: bool = False == False
print(result3)

# Bool inequality
result4: bool = True != True
print(result4)

result5: bool = True != False
print(result5)

result6: bool = False != False
print(result6)

# Bool with int comparison (True == 1, False == 0)
result7: bool = True == 1
print(result7)

result8: bool = False == 0
print(result8)

result9: bool = True == 0
print(result9)

result10: bool = True != 1
print(result10)

result11: bool = False != 0
print(result11)

# Bool identity
result12: bool = True is True
print(result12)

result13: bool = True is False
print(result13)

result14: bool = False is not True
print(result14)

result15: bool = True is not True
print(result15)

# Bool ordering (True=1, False=0)
result16: bool = True > False
print(result16)

result17: bool = False < True
print(result17)

result18: bool = True >= True
print(result18)

result19: bool = False <= False
print(result19)

# Bool arithmetic (coerced to int)
result20: int = True + True
print(result20)

result21: int = True + 1
print(result21)

result22: int = True - False
print(result22)

result23: int = True * 5
print(result23)
