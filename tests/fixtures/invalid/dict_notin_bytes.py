# Cannot use 'not in' with Dict[str, int] and Bytes
x: bool = {"a": 1} not in b"hello"
