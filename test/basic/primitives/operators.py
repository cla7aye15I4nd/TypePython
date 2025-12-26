# Test all binary operators
def test_add(a: int, b: int) -> int:
    print("Adding", a, "+", b)
    result: int = a + b
    print("Result:", result)
    return result

def test_sub(a: int, b: int) -> int:
    print("Subtracting", a, "-", b)
    result: int = a - b
    print("Result:", result)
    return result

def test_mult(a: int, b: int) -> int:
    print("Multiplying", a, "*", b)
    result: int = a * b
    print("Result:", result)
    return result

def test_mod(a: int, b: int) -> int:
    print("Modulo", a, "%", b)
    result: int = a % b
    print("Result:", result)
    return result

# Test all comparison operators
def test_eq(a: int, b: int) -> int:
    print("Testing", a, "==", b)
    if a == b:
        print("True")
        return 1
    print("False")
    return 0

def test_neq(a: int, b: int) -> int:
    print("Testing", a, "!=", b)
    if a != b:
        print("True")
        return 1
    print("False")
    return 0

def test_lt(a: int, b: int) -> int:
    print("Testing", a, "<", b)
    if a < b:
        print("True")
        return 1
    print("False")
    return 0

def test_lte(a: int, b: int) -> int:
    print("Testing", a, "<=", b)
    if a <= b:
        print("True")
        return 1
    print("False")
    return 0

def test_gt(a: int, b: int) -> int:
    print("Testing", a, ">", b)
    if a > b:
        print("True")
        return 1
    print("False")
    return 0

def test_gte(a: int, b: int) -> int:
    print("Testing", a, ">=", b)
    if a >= b:
        print("True")
        return 1
    print("False")
    return 0
