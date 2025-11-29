# Cannot use 'not in' with Float and Bytes
x: bool = 1.0 not in b"hello"
