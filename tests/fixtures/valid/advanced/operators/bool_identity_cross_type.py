# Test bool 'is' and 'is not' operators with different types

# Bool is with int (should be False for different types)
result1: bool = True is 1
print(result1)

result2: bool = False is 0
print(result2)

# Bool is with bytes (should always be False)
result3: bool = True is b"hello"
print(result3)

result4: bool = False is b""
print(result4)

# Bool is not with int (should be True for different types)
result5: bool = True is not 1
print(result5)

result6: bool = False is not 0
print(result6)

# Bool is not with bytes (should always be True)
result7: bool = True is not b"hello"
print(result7)

result8: bool = False is not b""
print(result8)
