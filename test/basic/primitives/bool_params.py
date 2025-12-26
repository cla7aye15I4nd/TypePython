# Test bool as function parameter and return type
def negate(b: bool) -> bool:
    if b:
        return False
    return True

def identity(b: bool) -> bool:
    return b

def and_func(a: bool, b: bool) -> bool:
    if a:
        if b:
            return True
    return False

def test_bool_param() -> int:
    x: bool = True
    y: bool = negate(x)
    if y:
        return 0
    return 1

def test_bool_identity() -> int:
    x: bool = False
    y: bool = identity(x)
    if y:
        return 0
    return 1

def test_bool_and_func() -> int:
    result: bool = and_func(True, True)
    if result:
        return 1
    return 0
