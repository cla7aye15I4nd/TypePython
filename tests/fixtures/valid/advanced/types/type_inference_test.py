# Type inference and compatibility tests
def return_int_literal() -> int:
    return 42

def return_float_literal() -> float:
    return 3.14

def return_bool_literal() -> bool:
    return True

def return_bytes_literal() -> bytes:
    return b"Hello"

def use_all_literals() -> None:
    i: int = return_int_literal()
    f: float = return_float_literal()
    b: bool = return_bool_literal()
    s: bytes = return_bytes_literal()

    print(b"Int:", i)
    print(b"Float:", f)
    print(b"Bool:", b)
    print(b"Bytes:", s)

def literal_arithmetic() -> int:
    a: int = 10
    b: int = 20
    c: int = 30

    result: int = a + b + c
    return result

use_all_literals()
result: int = literal_arithmetic()
print(b"Literal arithmetic:", result)
