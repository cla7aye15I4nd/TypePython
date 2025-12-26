# Exception type matching tests - covers various exception type scenarios

# Multi-level exception hierarchy
class BaseError(Exception):
    code: int

class MiddleError(BaseError):
    code: int

class LeafError(MiddleError):
    code: int

# Another hierarchy
class NetworkError(Exception):
    code: int

class TimeoutError(NetworkError):
    code: int

def test_exception_no_message() -> int:
    """Exception raised with no message (empty args)"""
    try:
        raise Exception()
    except:
        print(1)
    print(2)
    return 0

def test_custom_no_message() -> int:
    """Custom exception with no message"""
    try:
        raise BaseError()
    except BaseError:
        print(1)
    print(2)
    return 0

def test_deep_hierarchy() -> int:
    """Multi-level exception inheritance (3 levels deep)"""
    try:
        raise LeafError("leaf")
    except LeafError:
        print(1)
    print(2)
    return 0

def test_catch_middle_level() -> int:
    """Catch exception at middle of hierarchy"""
    try:
        raise LeafError("leaf")
    except MiddleError:
        print(1)
    print(2)
    return 0

def test_catch_base_level() -> int:
    """Catch exception at base of hierarchy"""
    try:
        raise LeafError("leaf")
    except BaseError:
        print(1)
    print(2)
    return 0

def test_catch_builtin_exception() -> int:
    """Catch any exception with Exception base class"""
    try:
        raise LeafError("leaf")
    except Exception:
        print(1)
    print(2)
    return 0

def test_specific_before_general() -> int:
    """More specific exception type matched first"""
    try:
        raise TimeoutError("timeout")
    except TimeoutError:
        print(1)
    except NetworkError:
        print(0)
    except Exception:
        print(0)
    print(2)
    return 0

def test_fallthrough_to_general() -> int:
    """Exception falls through to general handler"""
    try:
        raise NetworkError("network")
    except TimeoutError:
        print(0)
    except NetworkError:
        print(1)
    print(2)
    return 0

def test_multiple_handlers_same_level() -> int:
    """Multiple handlers at same inheritance level"""
    try:
        raise BaseError("base")
    except NetworkError:
        print(0)
    except BaseError:
        print(1)
    print(2)
    return 0

def test() -> int:
    print("=== Exception Types Tests ===")

    print("Test: exception no message")
    test_exception_no_message()

    print("Test: custom exception no message")
    test_custom_no_message()

    print("Test: deep hierarchy")
    test_deep_hierarchy()

    print("Test: catch middle level")
    test_catch_middle_level()

    print("Test: catch base level")
    test_catch_base_level()

    print("Test: catch builtin Exception")
    test_catch_builtin_exception()

    print("Test: specific before general")
    test_specific_before_general()

    print("Test: fallthrough to general")
    test_fallthrough_to_general()

    print("Test: multiple handlers same level")
    test_multiple_handlers_same_level()

    print("=== Exception Types Tests Complete ===")
    return 0
