# Cannot use 'in' with Dict[str, int] and Bytes
x: bool = {"a": 1} in b"hello"
