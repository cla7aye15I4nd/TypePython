# Nested try/except/finally blocks - more complex nesting patterns

class DeepError(Exception):
    code: int

def test_triple_nested() -> int:
    """Triple nested try blocks"""
    try:
        print(1)
        try:
            print(2)
            try:
                raise Exception("deep")
            except:
                print(3)
            print(4)
        except:
            print(0)
        print(5)
    except:
        print(0)
    print(6)
    return 0

def test_nested_finally_chain() -> int:
    """Nested finally blocks all execute in order"""
    try:
        try:
            try:
                print(1)
            finally:
                print(2)
        finally:
            print(3)
    finally:
        print(4)
    print(5)
    return 0

def test_nested_exception_in_finally() -> int:
    """Exception in inner finally, caught by outer except"""
    try:
        try:
            print(1)
        finally:
            print(2)
            raise Exception("from finally")
    except:
        print(3)
    print(4)
    return 0

def test_nested_else_clauses() -> int:
    """Nested try with else clauses"""
    try:
        print(1)
        try:
            print(2)
        except:
            print(0)
        else:
            print(3)
    except:
        print(0)
    else:
        print(4)
    print(5)
    return 0

def test_nested_mixed() -> int:
    """Mixed nesting with all clauses"""
    try:
        print(1)
        try:
            raise DeepError("inner")
        except DeepError:
            print(2)
        else:
            print(0)
        finally:
            print(3)
        print(4)
    except:
        print(0)
    finally:
        print(5)
    print(6)
    return 0

def test() -> int:
    print("=== Nested Try Tests ===")

    print("Test: triple nested try")
    test_triple_nested()

    print("Test: nested finally chain")
    test_nested_finally_chain()

    print("Test: exception in finally")
    test_nested_exception_in_finally()

    print("Test: nested else clauses")
    test_nested_else_clauses()

    print("Test: nested mixed clauses")
    test_nested_mixed()

    print("=== Nested Try Tests Complete ===")
    return 0
