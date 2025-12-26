# Finally clause tests - verify finally always executes

class FinallyError(Exception):
    code: int

def test_finally_no_exception() -> int:
    """Finally runs when no exception"""
    try:
        print(1)
    finally:
        print(2)
    print(3)
    return 0

def test_finally_with_exception() -> int:
    """Finally runs when exception is caught"""
    try:
        raise Exception("test")
    except:
        print(1)
    finally:
        print(2)
    print(3)
    return 0

def test_finally_exception_propagates() -> int:
    """Finally runs even when exception propagates"""
    try:
        try:
            raise Exception("propagate")
        finally:
            print(1)
    except:
        print(2)
    print(3)
    return 0

def test_finally_with_else() -> int:
    """Finally runs with else clause (no exception)"""
    try:
        print(1)
    except:
        print(0)
    else:
        print(2)
    finally:
        print(3)
    print(4)
    return 0

def test_finally_else_exception() -> int:
    """Finally runs when exception, else skipped"""
    try:
        raise Exception("test")
    except:
        print(1)
    else:
        print(0)
    finally:
        print(2)
    print(3)
    return 0

def test_finally_multiple_except() -> int:
    """Finally runs with multiple except clauses"""
    try:
        raise FinallyError("specific")
    except Exception:
        print(1)
    finally:
        print(2)
    print(3)
    return 0

def test_finally_bare_except() -> int:
    """Finally with bare except clause"""
    try:
        raise Exception("bare")
    except:
        print(1)
    finally:
        print(2)
    print(3)
    return 0

def test() -> int:
    print("=== Finally Always Tests ===")

    print("Test: finally no exception")
    test_finally_no_exception()

    print("Test: finally with exception")
    test_finally_with_exception()

    print("Test: finally exception propagates")
    test_finally_exception_propagates()

    print("Test: finally with else")
    test_finally_with_else()

    print("Test: finally else with exception")
    test_finally_else_exception()

    print("Test: finally multiple except")
    test_finally_multiple_except()

    print("Test: finally bare except")
    test_finally_bare_except()

    print("=== Finally Always Tests Complete ===")
    return 0
