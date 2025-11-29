# Cannot use 'not in' with Bytes and Str
x: bool = b"hello" not in "hello"
