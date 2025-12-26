# Exception handling tests module

# Import additional test modules
from . import nested_try
from . import exception_reraise
from . import finally_always
from . import except_types
from . import raise_from_except

# Custom exception classes for testing
class MyError(Exception):
    code: int

class ErrorA(Exception):
    code: int

class ErrorB(Exception):
    code: int

def test_try_no_exception() -> int:
    """Test try/except when no exception is raised"""
    try:
        print(1)
    except:
        print(0)
    print(2)
    return 0

def test_try_with_raise() -> int:
    """Test try/except with raise"""
    try:
        raise Exception("test")
    except:
        print(1)
    print(2)
    return 0

def test_try_else() -> int:
    """Test else clause - runs when no exception"""
    try:
        print(1)
    except:
        print(0)
    else:
        print(2)
    print(3)
    return 0

def test_try_else_with_exception() -> int:
    """Test else clause - should NOT run when exception occurs"""
    try:
        raise Exception("test")
    except:
        print(1)
    else:
        print(0)
    print(2)
    return 0

def test_try_finally() -> int:
    """Test finally clause - always runs"""
    try:
        print(1)
    finally:
        print(2)
    print(3)
    return 0

def test_try_except_finally() -> int:
    """Test try/except/finally with exception"""
    try:
        raise Exception("test")
    except:
        print(1)
    finally:
        print(2)
    print(3)
    return 0

def test_raise_in_function() -> int:
    """Test raise inside a function"""
    try:
        raise Exception("error")
    except:
        print(1)
    print(2)
    return 0

def test_custom_exception() -> int:
    """Test raising and catching custom exception class"""
    try:
        raise MyError("custom error")
    except MyError:
        print(1)
    print(2)
    return 0

def test_exception_type_matching() -> int:
    """Test that specific exception types are matched correctly"""
    try:
        raise ErrorB("B error")
    except ErrorA:
        print(0)
    except ErrorB:
        print(1)
    print(2)
    return 0

def test_base_exception_catches_subclass() -> int:
    """Test that catching Exception catches subclasses"""
    try:
        raise MyError("subclass error")
    except Exception:
        print(1)
    print(2)
    return 0

def test_nested_try_catch() -> int:
    """Test nested try/except blocks"""
    try:
        print(1)
        try:
            raise Exception("inner")
        except:
            print(2)
        print(3)
    except:
        print(0)
    print(4)
    return 0

def test_nested_exception_propagation() -> int:
    """Test exception propagation through nested try blocks"""
    try:
        try:
            raise Exception("inner")
        finally:
            print(1)
    except:
        print(2)
    print(3)
    return 0

def test_nested_finally() -> int:
    """Test nested finally clauses"""
    try:
        try:
            print(1)
        finally:
            print(2)
    finally:
        print(3)
    print(4)
    return 0

def test() -> int:
    print("=== Exception Handling Tests ===")

    print("Test: try without exception")
    test_try_no_exception()

    print("Test: try with raise")
    test_try_with_raise()

    print("Test: else clause (no exception)")
    test_try_else()

    print("Test: else clause (with exception)")
    test_try_else_with_exception()

    print("Test: finally clause")
    test_try_finally()

    print("Test: except and finally")
    test_try_except_finally()

    print("Test: raise in function")
    test_raise_in_function()

    print("Test: custom exception class")
    test_custom_exception()

    print("Test: exception type matching")
    test_exception_type_matching()

    print("Test: base Exception catches subclass")
    test_base_exception_catches_subclass()

    print("Test: nested try/catch")
    test_nested_try_catch()

    print("Test: nested exception propagation")
    test_nested_exception_propagation()

    print("Test: nested finally")
    test_nested_finally()

    # Run additional test modules
    nested_try.test()
    exception_reraise.test()
    finally_always.test()
    except_types.test()
    raise_from_except.test()

    print("=== Exception Tests Complete ===")
    return 0
