# Arithmetic operator edge cases

# Testing power operator edge cases
def test_power() -> None:
    # Zero exponent
    a: int = 999
    zero_pow: int = a ** 0
    print(b"999 ** 0 =", zero_pow)

    # One as base
    one_pow: int = 1 ** 100
    print(b"1 ** 100 =", one_pow)

    # Large power
    two_pow: int = 2 ** 20
    print(b"2 ** 20 =", two_pow)

    # Float powers - print as scaled value to avoid precision issues
    f: float = 2.0
    fp: float = f ** 0.5
    fp_scaled: float = fp * 10000 // 1
    print(b"2.0 ** 0.5 * 10000 =", fp_scaled)

test_power()

# Testing modulo edge cases
def test_modulo() -> None:
    # Modulo with 1
    mod1: int = 100 % 1
    print(b"100 % 1 =", mod1)

    # Self modulo
    self_mod: int = 7 % 7
    print(b"7 % 7 =", self_mod)

    # Negative modulo
    neg_mod: int = -10 % 3
    print(b"-10 % 3 =", neg_mod)

    # Float modulo
    fmod: float = 5.5 % 2.0
    print(b"5.5 % 2.0 =", fmod)

test_modulo()

# Testing division edge cases
def test_division() -> None:
    # Division by 1
    div1: int = 42 // 1
    print(b"42 // 1 =", div1)

    # Self division
    self_div: int = 13 // 13
    print(b"13 // 13 =", self_div)

    # Negative division
    neg_div: int = -20 // 3
    print(b"-20 // 3 =", neg_div)

    # Float division
    fdiv: float = 7.0 / 2.0
    print(b"7.0 / 2.0 =", fdiv)

    # Float floor division
    ffloor: float = 7.0 // 2.0
    print(b"7.0 // 2.0 =", ffloor)

test_division()

# Testing multiplication edge cases
def test_multiplication() -> None:
    # Multiply by zero
    zero_mul: int = 999 * 0
    print(b"999 * 0 =", zero_mul)

    # Multiply by one
    one_mul: int = 42 * 1
    print(b"42 * 1 =", one_mul)

    # Multiply by negative one
    neg_one_mul: int = 42 * -1
    print(b"42 * -1 =", neg_one_mul)

    # Large multiplication
    big_mul: int = 12345 * 6789
    print(b"12345 * 6789 =", big_mul)

test_multiplication()

# Testing subtraction edge cases
def test_subtraction() -> None:
    # Self subtraction
    self_sub: int = 100 - 100
    print(b"100 - 100 =", self_sub)

    # Subtract from zero
    from_zero: int = 0 - 50
    print(b"0 - 50 =", from_zero)

    # Double negative
    double_neg: int = -5 - -10
    print(b"-5 - -10 =", double_neg)

test_subtraction()

# Order of operations
def test_precedence() -> None:
    # Power before multiplication
    r1: int = 2 * 3 ** 2
    print(b"2 * 3 ** 2 =", r1)

    # Multiplication before addition
    r2: int = 2 + 3 * 4
    print(b"2 + 3 * 4 =", r2)

    # Parentheses override
    r3: int = (2 + 3) * 4
    print(b"(2 + 3) * 4 =", r3)

    # Complex expression
    r4: int = 2 ** 3 + 4 * 5 - 6 // 2
    print(b"2**3 + 4*5 - 6//2 =", r4)

test_precedence()
