# Cannot use 'not in' with Set[int] and Bytes
x: bool = {1, 2, 3} not in b"hello"
