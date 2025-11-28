# Test logical operators and short-circuit behavior

# Function that prints and returns value (to test evaluation)
def check_true() -> bool:
    print(b"check_true called")
    return True

def check_false() -> bool:
    print(b"check_false called")
    return False

# AND short-circuit: False and X should not evaluate X
print(b"=== AND short-circuit ===")
print(b"Testing: False and check_true()")
r1: bool = False and check_true()
print(b"Result:", r1)

print(b"Testing: True and check_true()")
r2: bool = True and check_true()
print(b"Result:", r2)

# OR short-circuit: True or X should not evaluate X
print(b"=== OR short-circuit ===")
print(b"Testing: True or check_false()")
r3: bool = True or check_false()
print(b"Result:", r3)

print(b"Testing: False or check_false()")
r4: bool = False or check_false()
print(b"Result:", r4)

# Chained logical operators
print(b"=== Chained logical ===")
a: bool = True
b: bool = False
c: bool = True

chain1: bool = a and b and c
print(b"True and False and True:", chain1)

chain2: bool = a or b or c
print(b"True or False or True:", chain2)

chain3: bool = a and b or c
print(b"True and False or True:", chain3)

chain4: bool = a or b and c
print(b"True or False and True:", chain4)

# NOT combinations
print(b"=== NOT combinations ===")
not1: bool = not True
not2: bool = not False
not3: bool = not not True
not4: bool = not (True and False)
not5: bool = not (True or False)

print(b"not True:", not1)
print(b"not False:", not2)
print(b"not not True:", not3)
print(b"not (True and False):", not4)
print(b"not (True or False):", not5)

# Practical examples
print(b"=== Practical examples ===")
def is_valid_range(x: int, low: int, high: int) -> bool:
    return x >= low and x <= high

def is_out_of_range(x: int, low: int, high: int) -> bool:
    return x < low or x > high

print(b"5 in [0,10]:", is_valid_range(5, 0, 10))
print(b"15 in [0,10]:", is_valid_range(15, 0, 10))
print(b"5 out of [0,10]:", is_out_of_range(5, 0, 10))
print(b"15 out of [0,10]:", is_out_of_range(15, 0, 10))

# Complex boolean expression
x: int = 5
y: int = 10
z: int = 15

complex_bool: bool = (x < y and y < z) or (x > z)
print(b"(5<10 and 10<15) or (5>15):", complex_bool)
