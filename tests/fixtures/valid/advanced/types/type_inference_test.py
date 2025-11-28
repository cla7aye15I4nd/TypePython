# Type inference and compatibility tests
def return_int_literal() -> int:
    return 42

def return_float_literal() -> float:
    return 3.14

def return_bool_literal() -> bool:
    return True

def return_string_literal() -> str:
    return "Hello"

def use_all_literals() -> None:
    i: int = return_int_literal()
    f: float = return_float_literal()
    b: bool = return_bool_literal()
    s: str = return_string_literal()

    print("Int:", i)
    print("Float:", f)
    print("Bool:", b)
    print("String:", s)

def literal_arithmetic() -> int:
    a: int = 10
    b: int = 20
    c: int = 30

    result: int = a + b + c
    return result

use_all_literals()
result: int = literal_arithmetic()
print("Literal arithmetic:", result)
