# Mixed type output with strings
def report_all_types() -> None:
    i: int = 42
    f: float = 3.14159
    b: bool = True
    s: str = "TypePython"

    print("Integer:", i)
    print("Float:", f)
    print("Boolean:", b)
    print("String:", s)

def formatted_results(x: int, y: int) -> None:
    sum: int = x + y
    diff: int = x - y
    prod: int = x * y
    quot: int = x // y

    print("Sum:", sum)
    print("Difference:", diff)
    print("Product:", prod)
    print("Quotient:", quot)

def conditional_output(value: int) -> None:
    if value > 100:
        print("Large:", value)
    else:
        if value > 50:
            print("Medium:", value)
        else:
            print("Small:", value)

report_all_types()
formatted_results(20, 4)
conditional_output(150)
conditional_output(75)
conditional_output(25)
