# Extra tests for coverage - unary minus, division, print edge cases

def test_unary_minus() -> int:
    x: int = 5
    y: int = -x
    return y

def test_division() -> int:
    x: int = 20
    y: int = 4
    z: int = x // y
    return z

def test_print_newline() -> int:
    print()
    return 1

def test_print_bool() -> int:
    b: bool = True
    print(b)
    return 1

def test_both_branches_return() -> int:
    x: int = 5
    if x > 0:
        return 1
    else:
        return 0

def test_bool_to_int() -> int:
    b: bool = True
    x: int = 0
    if b:
        x = 1
    return x
