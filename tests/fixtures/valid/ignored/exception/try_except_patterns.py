# Test comprehensive try/except/finally patterns

# Basic try-except
try:
    x: int = 1 / 0
except ZeroDivisionError:
    print(b"Caught division by zero")

# Try-except with message
try:
    result: int = 10 / 0
except ZeroDivisionError as e:
    print(b"Error:", str(e))

# Multiple except blocks
def multi_except(val: int) -> None:
    try:
        if val == 0:
            raise ZeroDivisionError("zero")
        elif val == 1:
            raise ValueError("one")
        elif val == 2:
            raise TypeError("two")
        print(b"No error for:", val)
    except ZeroDivisionError:
        print(b"Got ZeroDivisionError")
    except ValueError:
        print(b"Got ValueError")
    except TypeError:
        print(b"Got TypeError")

multi_except(0)
multi_except(1)
multi_except(2)
multi_except(3)

# Except with tuple of exceptions
def tuple_except(val: int) -> None:
    try:
        if val == 0:
            raise ValueError("val error")
        elif val == 1:
            raise TypeError("type error")
        print(b"No error")
    except (ValueError, TypeError) as e:
        print(b"Caught one of:", str(e))

tuple_except(0)
tuple_except(1)
tuple_except(2)

# Try-except-else
def with_else(val: int) -> None:
    try:
        result: int = 100 / val
    except ZeroDivisionError:
        print(b"Cannot divide by zero")
    else:
        print(b"Division result:", result)

with_else(0)
with_else(10)

# Try-except-finally
def with_finally(val: int) -> None:
    try:
        result: int = 100 / val
        print(b"Result:", result)
    except ZeroDivisionError:
        print(b"Division error")
    finally:
        print(b"Cleanup done")

with_finally(0)
with_finally(10)

# Try-except-else-finally
def full_try(val: int) -> None:
    try:
        result: int = 100 / val
    except ZeroDivisionError:
        print(b"Caught zero division")
    else:
        print(b"Success:", result)
    finally:
        print(b"Always runs")

full_try(0)
full_try(5)

# Nested try-except
def nested_try() -> None:
    try:
        try:
            x: int = 1 / 0
        except ZeroDivisionError:
            print(b"Inner caught")
            raise ValueError("re-raised as ValueError")
    except ValueError as e:
        print(b"Outer caught:", str(e))

nested_try()

# Try in loop
def try_in_loop() -> None:
    for i in range(-2, 3):
        try:
            result: int = 10 / i
            print(b"10 /", i, "=", result)
        except ZeroDivisionError:
            print(b"Skipping division by zero")

try_in_loop()

# Loop in try
def loop_in_try() -> None:
    try:
        for i in range(5):
            if i == 3:
                raise ValueError("stopped at 3")
            print(b"Processing:", i)
    except ValueError as e:
        print(b"Loop error:", str(e))

loop_in_try()

# Re-raise exception
def reraise_exception() -> None:
    try:
        try:
            x: list[int] = [1, 2, 3]
            print(x[10])
        except IndexError:
            print(b"Caught and re-raising")
            raise
    except IndexError:
        print(b"Re-caught IndexError")

reraise_exception()

# Raise with custom message
def custom_raise(val: int) -> None:
    try:
        if val < 0:
            raise ValueError("Value must be non-negative")
        print(b"Value is:", val)
    except ValueError as e:
        print(b"Validation error:", str(e))

custom_raise(-5)
custom_raise(10)

# Exception in function call
def may_fail(x: int) -> int:
    if x < 0:
        raise ValueError("negative")
    return x * 2

def call_may_fail() -> None:
    try:
        result: int = may_fail(-1)
        print(b"Result:", result)
    except ValueError as e:
        print(b"Function failed:", str(e))

call_may_fail()

# Exception propagation
def level3_ex() -> None:
    raise RuntimeError("deep error")

def level2_ex() -> None:
    level3_ex()

def level1_ex() -> None:
    try:
        level2_ex()
    except RuntimeError as e:
        print(b"Caught from deep:", str(e))

level1_ex()

# Finally with return
def finally_with_return(val: int) -> int:
    try:
        if val == 0:
            raise ValueError("zero")
        return val * 2
    except ValueError:
        return -1
    finally:
        print(b"Finally runs before return")

print(b"Return val 0:", finally_with_return(0))
print(b"Return val 5:", finally_with_return(5))

# Finally with break
def finally_with_break() -> None:
    for i in range(5):
        try:
            if i == 3:
                break
            print(b"Iteration:", i)
        finally:
            print(b"Finally in loop:", i)
    print(b"Loop ended")

finally_with_break()

# Finally with continue
def finally_with_continue() -> None:
    for i in range(5):
        try:
            if i == 2:
                continue
            print(b"Processing:", i)
        finally:
            print(b"Finally:", i)

finally_with_continue()

# Bare except (catch all)
def bare_except() -> None:
    try:
        x: dict[str, int] = {}
        print(x["missing"])
    except:
        print(b"Caught something")

bare_except()

# Exception type checking
def check_exception_type(val: int) -> None:
    try:
        if val == 0:
            raise ValueError("value")
        elif val == 1:
            raise TypeError("type")
        elif val == 2:
            raise KeyError("key")
    except Exception as e:
        if isinstance(e, ValueError):
            print(b"It was ValueError")
        elif isinstance(e, TypeError):
            print(b"It was TypeError")
        else:
            print(b"It was something else:", type(e).__name__)

check_exception_type(0)
check_exception_type(1)
check_exception_type(2)

# Try with resource management
def resource_management() -> None:
    resource: list[str] = []
    try:
        resource.append("opened")
        print(b"Resource:", resource)
        raise ValueError("operation failed")
    except ValueError as e:
        print(b"Error during operation:", str(e))
    finally:
        resource.clear()
        print(b"Resource cleaned up")

resource_management()

# Exception in list comprehension
def except_in_comprehension() -> None:
    data: list[int] = [1, 2, 0, 4, 5]
    results: list[int] = []
    for x in data:
        try:
            results.append(10 / x)
        except ZeroDivisionError:
            results.append(-1)
    print(b"Results:", results)

except_in_comprehension()

# Exception in dict access
def except_dict_access() -> None:
    d: dict[str, int] = {"a": 1, "b": 2}
    for key in ["a", "b", "c", "d"]:
        try:
            val: int = d[key]
            print(b"Key", key, "=", val)
        except KeyError:
            print(b"Key", key, "not found")

except_dict_access()

# Exception in list access
def except_list_access() -> None:
    lst: list[int] = [10, 20, 30]
    for i in range(5):
        try:
            print(b"Index", i, "=", lst[i])
        except IndexError:
            print(b"Index", i, "out of range")

except_list_access()

# Raise from (chained exceptions)
def raise_from() -> None:
    try:
        try:
            x: int = int("not a number")
        except ValueError as e:
            raise RuntimeError("conversion failed") from e
    except RuntimeError as e:
        print(b"Caught RuntimeError:", str(e))
        if e.__cause__ is not None:
            print(b"Caused by:", str(e.__cause__))

raise_from()

# Exception attributes
def exception_attrs() -> None:
    try:
        raise ValueError("test error", 42, "extra")
    except ValueError as e:
        print(b"Args:", e.args)

exception_attrs()

# Assert as exception
def assert_exception(val: int) -> None:
    try:
        assert val > 0, "value must be positive"
        print(b"Value is positive:", val)
    except AssertionError as e:
        print(b"Assertion failed:", str(e))

assert_exception(10)
assert_exception(-5)

# StopIteration handling
def stop_iter_handling() -> None:
    gen = iter([1, 2, 3])
    while True:
        try:
            val: int = next(gen)
            print(b"Next:", val)
        except StopIteration:
            print(b"Iterator exhausted")
            break

stop_iter_handling()

# Multiple exceptions same handler
def multi_same_handler() -> None:
    errors: list[Exception] = [
        ValueError("val"),
        TypeError("type"),
        KeyError("key")
    ]
    for err in errors:
        try:
            raise err
        except (ValueError, TypeError, KeyError) as e:
            print(b"Handled:", type(e).__name__, str(e))

multi_same_handler()

# Exception with context manager pattern
class SimpleContext:
    def __init__(self, name: str) -> None:
        self.name: str = name

    def __enter__(self) -> SimpleContext:
        print(b"Entering:", self.name)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb) -> bool:
        print(b"Exiting:", self.name)
        if exc_type is not None:
            print(b"Exception:", str(exc_val))
        return False

def context_exception() -> None:
    try:
        with SimpleContext("test") as ctx:
            print(b"Inside context")
            raise ValueError("error in context")
    except ValueError as e:
        print(b"Caught outside:", str(e))

context_exception()

# Generator with exception
def gen_with_exception() -> int:
    for i in range(5):
        if i == 3:
            raise ValueError("stopped at 3")
        yield i

def use_gen_exception() -> None:
    try:
        for x in gen_with_exception():
            print(b"Gen value:", x)
    except ValueError as e:
        print(b"Generator error:", str(e))

use_gen_exception()

# Throw into generator
def gen_with_throw() -> int:
    try:
        yield 1
        yield 2
        yield 3
    except ValueError:
        yield -1

def use_gen_throw() -> None:
    g = gen_with_throw()
    print(b"First:", next(g))
    print(b"After throw:", g.throw(ValueError, "injected"))

use_gen_throw()

# Finally in generator
def gen_with_finally() -> int:
    try:
        yield 1
        yield 2
    finally:
        print(b"Generator cleanup")

def use_gen_finally() -> None:
    g = gen_with_finally()
    print(b"Got:", next(g))
    g.close()

use_gen_finally()

# Recursive function with exception
def factorial_safe(n: int) -> int:
    if n < 0:
        raise ValueError("negative factorial")
    if n <= 1:
        return 1
    return n * factorial_safe(n - 1)

def test_factorial() -> None:
    for val in [5, 0, -3]:
        try:
            result: int = factorial_safe(val)
            print(b"Factorial of", val, "=", result)
        except ValueError as e:
            print(b"Error for", val, ":", str(e))

test_factorial()

# Exception in class method
class Calculator:
    def divide(self, a: int, b: int) -> float:
        if b == 0:
            raise ZeroDivisionError("cannot divide by zero")
        return a / b

def test_calculator() -> None:
    calc: Calculator = Calculator()
    for a, b in [(10, 2), (5, 0), (20, 4)]:
        try:
            result: float = calc.divide(a, b)
            print(b"Divide", a, "/", b, "=", result)
        except ZeroDivisionError as e:
            print(b"Calc error:", str(e))

test_calculator()

# Custom exception class
class CustomError(Exception):
    def __init__(self, code: int, message: str) -> None:
        super().__init__(message)
        self.code: int = code
        self.message: str = message

def raise_custom() -> None:
    try:
        raise CustomError(404, "Not found")
    except CustomError as e:
        print(b"Custom error code:", e.code)
        print(b"Custom error msg:", e.message)

raise_custom()

# Exception hierarchy
class BaseAppError(Exception):
    pass

class NetworkError(BaseAppError):
    pass

class DatabaseError(BaseAppError):
    pass

def test_hierarchy() -> None:
    errors: list[BaseAppError] = [
        NetworkError("connection failed"),
        DatabaseError("query failed"),
        BaseAppError("generic error")
    ]
    for err in errors:
        try:
            raise err
        except NetworkError:
            print(b"Network issue")
        except DatabaseError:
            print(b"Database issue")
        except BaseAppError:
            print(b"Generic app error")

test_hierarchy()
