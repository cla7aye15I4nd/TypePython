# Cannot use 'in' with Dict[str, int] and Str
x: bool = {"a": 1} in "hello"
