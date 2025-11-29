# Cannot use 'not in' with Dict[str, int] and Bool
x: bool = {"a": 1} not in True
