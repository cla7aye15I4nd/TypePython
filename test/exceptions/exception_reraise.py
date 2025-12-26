# Exception re-raising tests

class RaiseError(Exception):
    code: int

def helper_that_throws() -> int:
    raise RaiseError("from helper")
    return 0

def test_catch_and_reraise() -> int:
    """Catch an exception, do something, then re-raise"""
    try:
        try:
            raise Exception("original")
        except:
            print(1)
            raise Exception("re-raised")
    except:
        print(2)
    print(3)
    return 0

def test_reraise_different_type() -> int:
    """Catch one exception type, raise a different one"""
    try:
        try:
            raise RaiseError("first")
        except RaiseError:
            print(1)
            raise Exception("converted")
    except Exception:
        print(2)
    print(3)
    return 0

def test_reraise_from_function() -> int:
    """Exception propagates from helper function"""
    try:
        helper_that_throws()
    except RaiseError:
        print(1)
    print(2)
    return 0

def test_multiple_reraise() -> int:
    """Multiple levels of catch and re-raise"""
    try:
        try:
            try:
                raise Exception("deep")
            except:
                print(1)
                raise Exception("level1")
        except:
            print(2)
            raise Exception("level2")
    except:
        print(3)
    print(4)
    return 0

def test() -> int:
    print("=== Exception Re-raise Tests ===")

    print("Test: catch and re-raise")
    test_catch_and_reraise()

    print("Test: re-raise different type")
    test_reraise_different_type()

    print("Test: re-raise from function")
    test_reraise_from_function()

    print("Test: multiple re-raise levels")
    test_multiple_reraise()

    print("=== Exception Re-raise Tests Complete ===")
    return 0
