# Bool type tests

# Test bool literals - return 1 if True works, 0 if False
def test_bool_true() -> int:
    if True:
        return 1
    return 0

def test_bool_false() -> int:
    if False:
        return 0
    return 1

# Test bool variables
def test_bool_var_true() -> int:
    x: bool = True
    if x:
        return 1
    return 0

def test_bool_var_false() -> int:
    x: bool = False
    if x:
        return 0
    return 1

# Test comparison returns bool and can be stored
def test_compare_to_bool() -> int:
    x: bool = 5 < 10
    if x:
        return 1
    return 0

# Test and operator
def test_and_tt() -> int:
    if True and True:
        return 1
    return 0

def test_and_tf() -> int:
    if True and False:
        return 0
    return 1

def test_and_ff() -> int:
    if False and False:
        return 0
    return 1

# Test or operator
def test_or_tt() -> int:
    if True or True:
        return 1
    return 0

def test_or_tf() -> int:
    if True or False:
        return 1
    return 0

def test_or_ff() -> int:
    if False or False:
        return 0
    return 1

# Test not operator
def test_not_true() -> int:
    if not True:
        return 0
    return 1

def test_not_false() -> int:
    if not False:
        return 1
    return 0

# Test bool with variables in logical ops
def test_and_vars() -> int:
    a: bool = True
    b: bool = False
    if a and b:
        return 0
    return 1

def test_or_vars() -> int:
    a: bool = True
    b: bool = False
    if a or b:
        return 1
    return 0

def test_not_var() -> int:
    a: bool = True
    if not a:
        return 0
    return 1

# Test chained comparison with bool result
def test_chained_compare_bool() -> int:
    x: bool = 1 < 5 < 10
    if x:
        return 1
    return 0
