# Bytes operations and manipulations
def test_bytes_literals() -> None:
    s1: bytes = b"Hello"
    s2: bytes = b"World"
    s3: bytes = b"TypePython"
    s4: bytes = b"Compiler"

    print(s1)
    print(s2)
    print(s3)
    print(s4)

def test_bytes_with_numbers() -> None:
    msg1: bytes = b"Number: "
    msg2: bytes = b"Value: "
    msg3: bytes = b"Result: "

    print(msg1, 42)
    print(msg2, 3.14)
    print(msg3, 100)

def test_multiple_bytes() -> None:
    greeting: bytes = b"Hello"
    name: bytes = b"Alice"
    farewell: bytes = b"Goodbye"

    print(greeting, name)
    print(farewell, name)

test_bytes_literals()
test_bytes_with_numbers()
test_multiple_bytes()
