# Cannot use 'not in' with Dict[str, int] and Float
x: bool = {"a": 1} not in 1.0
