# Test basic float support

# Test float literals
def test_float_literal() -> int:
    x: float = 3.14
    if x > 3.0:
        return 1
    return 0

# Test float addition
def test_float_add() -> int:
    a: float = 2.5
    b: float = 1.5
    result: float = a + b
    if result == 4.0:
        return 1
    return 0

# Test float subtraction
def test_float_sub() -> int:
    a: float = 5.5
    b: float = 2.5
    result: float = a - b
    if result == 3.0:
        return 1
    return 0

# Test float multiplication
def test_float_mult() -> int:
    a: float = 2.5
    b: float = 4.0
    result: float = a * b
    if result == 10.0:
        return 1
    return 0

# Test float division
def test_float_div() -> int:
    a: float = 10.0
    b: float = 4.0
    result: float = a / b
    if result == 2.5:
        return 1
    return 0

# Test int division returns float
def test_int_div_returns_float() -> int:
    result: float = 10 / 4
    if result == 2.5:
        return 1
    return 0

# Test mixed int/float addition
def test_mixed_add() -> int:
    result: float = 5 + 1.5
    if result == 6.5:
        return 1
    return 0

# Test unary minus on float
def test_float_neg() -> int:
    x: float = 3.14
    result: float = -x
    if result < 0.0:
        return 1
    return 0

# Test float comparison greater than
def test_float_gt() -> int:
    a: float = 3.14
    b: float = 2.5
    if a > b:
        return 1
    return 0

# Test float comparison less than
def test_float_lt() -> int:
    a: float = 2.5
    b: float = 3.14
    if a < b:
        return 1
    return 0

# Test float comparison equal
def test_float_eq() -> int:
    a: float = 3.14
    b: float = 3.14
    if a == b:
        return 1
    return 0

# Test float truthiness (non-zero is true)
def test_float_truthy() -> int:
    x: float = 1.0
    if x:
        return 1
    return 0

# Test float falsiness (zero is false)
def test_float_falsy() -> int:
    x: float = 0.0
    if x:
        return 0
    return 1

# Test float floor division
def test_float_floordiv() -> int:
    result: float = 7.5 // 2.0
    if result == 3.0:
        return 1
    return 0

# Test float modulo
def test_float_mod() -> int:
    result: float = 7.5 % 2.0
    if result == 1.5:
        return 1
    return 0

# Test float power
def test_float_pow() -> int:
    result: float = 2.0 ** 3.0
    if result == 8.0:
        return 1
    return 0

# Test print float (returns 1 if no crash)
def test_print_float() -> int:
    x: float = 3.14
    print(x)
    return 1
