# Mixed type output with bytes
def report_all_types() -> None:
    i: int = 42
    f: float = 3.14159
    b: bool = True
    s: bytes = b"TypePython"

    print(b"Integer:", i)
    print(b"Float:", f)
    print(b"Boolean:", b)
    print(b"Bytes:", s)

def formatted_results(x: int, y: int) -> None:
    sum: int = x + y
    diff: int = x - y
    prod: int = x * y
    quot: int = x // y

    print(b"Sum:", sum)
    print(b"Difference:", diff)
    print(b"Product:", prod)
    print(b"Quotient:", quot)

def conditional_output(value: int) -> None:
    if value > 100:
        print(b"Large:", value)
    else:
        if value > 50:
            print(b"Medium:", value)
        else:
            print(b"Small:", value)

report_all_types()
formatted_results(20, 4)
conditional_output(150)
conditional_output(75)
conditional_output(25)
