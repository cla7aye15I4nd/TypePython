# Test identity operators (is, is not)
# Integer identity
a: int = 5
b: int = 5
c: int = 10

result1: bool = a is b
print("5 is 5:", result1)

result2: bool = a is c
print("5 is 10:", result2)

result3: bool = a is not c
print("5 is not 10:", result3)

# Float identity
x: float = 3.14
y: float = 3.14
z: float = 2.71

result4: bool = x is y
print("3.14 is 3.14:", result4)

result5: bool = x is not z
print("3.14 is not 2.71:", result5)

# Note: In TypePython, 'is' compares values for primitives
# This is different from Python where 'is' compares object identity
