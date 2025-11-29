# Cannot use 'not in' with Dict[str, int] and None
x: bool = {"a": 1} not in None
