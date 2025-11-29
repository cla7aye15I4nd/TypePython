# Cannot use 'not in' with Dict[str, int] and Str
x: bool = {"a": 1} not in "hello"
