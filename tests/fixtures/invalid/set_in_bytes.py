# Cannot use 'in' with Set[int] and Bytes
x: bool = {1, 2, 3} in b"hello"
