# Cannot use 'in' with Dict[str, int] and None
x: bool = {"a": 1} in None
