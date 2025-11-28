print(b"=== Test 1: Arithmetic Precedence ===")
result1: int = 2 + 3 * 4
print(b"2 + 3 * 4 =", result1)

result2: int = 10 - 8 // 2
print(b"10 - 8 // 2 =", result2)

result3: int = 10 + 5 % 3
print(b"10 + 5 % 3 =", result3)

result4: int = 2 + 3 * 4 - 5
print(b"2 + 3 * 4 - 5 =", result4)

print()
print(b"=== Test 2: Comparison and Arithmetic ===")
check1: bool = 2 + 3 == 5
print(b"2 + 3 == 5 is", check1)

check2: bool = 10 - 5 > 3
print(b"10 - 5 > 3 is", check2)

check3: bool = 2 * 3 <= 6
print(b"2 * 3 <= 6 is", check3)

check4: bool = 10 // 2 != 4
print(b"10 // 2 != 4 is", check4)

print()
print(b"=== Test 3: Logical Operator Precedence ===")
bool1: bool = True or False and False
print(b"True or False and False is", bool1)

bool2: bool = False and False or True
print(b"False and False or True is", bool2)

bool3: bool = not False and True
print(b"not False and True is", bool3)

bool4: bool = not True or False
print(b"not True or False is", bool4)

print()
print(b"=== Test 4: Comparison Chaining Context ===")
check5: bool = 3 * 2 > 4 + 1
print(b"3 * 2 > 4 + 1 is", check5)

check6: bool = 10 - 3 <= 2 * 4
print(b"10 - 3 <= 2 * 4 is", check6)

check7: bool = 15 // 3 == 10 - 5
print(b"15 // 3 == 10 - 5 is", check7)

print()
print(b"=== Test 5: Complex Mixed Expressions ===")
a: int = 5
b: int = 3
c: int = 2
result5: int = a + b * c - 1
print(b"5 + 3 * 2 - 1 =", result5)

d: bool = a > b and b > c
print(b"5 > 3 and 3 > 2 is", d)

e: bool = a == b or c * 2 == 4
print(b"5 == 3 or 2 * 2 == 4 is", e)

f: bool = not a < b
print(b"not 5 < 3 is", f)

print()
print(b"=== Test 6: Float Operations Precedence ===")
x: float = 2.5 + 3.0 * 2.0
print(b"2.5 + 3.0 * 2.0 =", x)

y: float = 10.0 - 2.0 * 1.0
print(b"10.0 - 2.0 * 1.0 =", y)

compare: bool = x > y
print(b"8.5 > 8.0 is", compare)

print()
print(b"=== Test 7: Modulo with Other Operations ===")
mod1: int = 10 + 7 % 3
print(b"10 + 7 % 3 =", mod1)

mod2: int = 15 % 4 * 2
print(b"15 % 4 * 2 =", mod2)

mod3: bool = 17 % 5 == 2
print(b"17 % 5 == 2 is", mod3)
