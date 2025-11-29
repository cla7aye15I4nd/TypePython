# Cannot use 'not in' with None and Bytes
x: bool = None not in b"hello"
