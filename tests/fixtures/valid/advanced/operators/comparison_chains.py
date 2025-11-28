# Test comparison operator chains and edge cases

# Integer comparisons with all operators
def test_int_comparisons(a: int, b: int) -> None:
    print(b"Testing:", a)
    print(b"Against:", b)

    eq: bool = a == b
    ne: bool = a != b
    lt: bool = a < b
    le: bool = a <= b
    gt: bool = a > b
    ge: bool = a >= b

    print(b"  == :", eq)
    print(b"  != :", ne)
    print(b"  <  :", lt)
    print(b"  <= :", le)
    print(b"  >  :", gt)
    print(b"  >= :", ge)

# Test with equal values
print(b"=== Equal values ===")
test_int_comparisons(5, 5)

# Test with different values
print(b"=== Different values ===")
test_int_comparisons(3, 7)

# Test with negative values
print(b"=== Negative values ===")
test_int_comparisons(-5, 3)

# Float comparisons
def test_float_comparisons(a: float, b: float) -> None:
    print(b"Float testing:", a)
    print(b"Float against:", b)

    eq: bool = a == b
    ne: bool = a != b
    lt: bool = a < b
    le: bool = a <= b
    gt: bool = a > b
    ge: bool = a >= b

    print(b"  == :", eq)
    print(b"  != :", ne)
    print(b"  <  :", lt)
    print(b"  <= :", le)
    print(b"  >  :", gt)
    print(b"  >= :", ge)

print(b"=== Float equal ===")
test_float_comparisons(3.14, 3.14)

print(b"=== Float different ===")
test_float_comparisons(2.71, 3.14)

# Boolean comparisons
t: bool = True
f: bool = False

eq_bool: bool = t == t
ne_bool: bool = t != f
print(b"True == True:", eq_bool)
print(b"True != False:", ne_bool)

# Bytes comparisons
s1: bytes = b"hello"
s2: bytes = b"hello"
s3: bytes = b"world"

eq_str: bool = s1 == s2
ne_str: bool = s1 != s3
print(b"hello == hello:", eq_str)
print(b"hello != world:", ne_str)

# Identity operators on integers
x: int = 100
y: int = 100
is_same: bool = x is y
is_not_same: bool = x is not y
print(b"100 is 100:", is_same)
print(b"100 is not 100:", is_not_same)
