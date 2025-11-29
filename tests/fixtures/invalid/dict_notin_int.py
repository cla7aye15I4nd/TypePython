# Cannot use 'not in' with Dict[str, int] and Int
x: bool = {"a": 1} not in 1
