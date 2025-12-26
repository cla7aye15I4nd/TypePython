# Test augmented assignments
def test_add_assign() -> int:
    x: int = 10
    x += 5
    return x

def test_sub_assign() -> int:
    x: int = 10
    x -= 3
    return x

def test_mult_assign() -> int:
    x: int = 10
    x *= 2
    return x

def test_mod_assign() -> int:
    x: int = 10
    x %= 3
    return x

def test_compound_aug() -> int:
    x: int = 5
    x += 10
    x *= 2
    x -= 5
    return x
