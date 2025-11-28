# Simple function definition and call
def greet(name: bytes) -> None:
    print(b"Hello,", name)

greet(b"World")
greet(b"TypePython")
