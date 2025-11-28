# String operations and manipulations
def test_string_literals() -> None:
    s1: str = "Hello"
    s2: str = "World"
    s3: str = "TypePython"
    s4: str = "Compiler"

    print(s1)
    print(s2)
    print(s3)
    print(s4)

def test_string_with_numbers() -> None:
    msg1: str = "Number: "
    msg2: str = "Value: "
    msg3: str = "Result: "

    print(msg1, 42)
    print(msg2, 3.14)
    print(msg3, 100)

def test_multiple_strings() -> None:
    greeting: str = "Hello"
    name: str = "Alice"
    farewell: str = "Goodbye"

    print(greeting, name)
    print(farewell, name)

test_string_literals()
test_string_with_numbers()
test_multiple_strings()
