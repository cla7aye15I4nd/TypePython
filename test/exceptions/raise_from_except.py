# Raise from except block tests

class WrapperError(Exception):
    code: int

class InnerError(Exception):
    code: int

def test_raise_new_in_except() -> int:
    """Raise a new exception from except block"""
    try:
        try:
            raise InnerError("inner")
        except InnerError:
            print(1)
            raise WrapperError("wrapper")
    except WrapperError:
        print(2)
    print(3)
    return 0

def test_raise_same_type_in_except() -> int:
    """Raise same exception type from except"""
    try:
        try:
            raise Exception("first")
        except:
            print(1)
            raise Exception("second")
    except:
        print(2)
    print(3)
    return 0

def test_raise_in_except_with_finally() -> int:
    """Raise in except, finally still runs"""
    try:
        try:
            raise Exception("original")
        except:
            print(1)
            raise Exception("new")
        finally:
            print(2)
    except:
        print(3)
    print(4)
    return 0

def test_conditional_raise_in_except() -> int:
    """Conditionally raise in except block"""
    x: int = 1
    try:
        try:
            raise Exception("test")
        except:
            print(1)
            if x > 0:
                raise WrapperError("conditional")
            print(0)
    except WrapperError:
        print(2)
    print(3)
    return 0

def test_no_raise_in_except() -> int:
    """Handle without re-raising"""
    try:
        raise Exception("handled")
    except:
        print(1)
    print(2)
    return 0

def test_raise_in_nested_except() -> int:
    """Raise in deeply nested except block"""
    try:
        try:
            try:
                raise InnerError("deep")
            except InnerError:
                print(1)
                raise WrapperError("mid")
        except WrapperError:
            print(2)
            raise Exception("outer")
    except:
        print(3)
    print(4)
    return 0

def test() -> int:
    print("=== Raise From Except Tests ===")

    print("Test: raise new in except")
    test_raise_new_in_except()

    print("Test: raise same type in except")
    test_raise_same_type_in_except()

    print("Test: raise in except with finally")
    test_raise_in_except_with_finally()

    print("Test: conditional raise in except")
    test_conditional_raise_in_except()

    print("Test: no raise in except")
    test_no_raise_in_except()

    print("Test: raise in nested except")
    test_raise_in_nested_except()

    print("=== Raise From Except Tests Complete ===")
    return 0
