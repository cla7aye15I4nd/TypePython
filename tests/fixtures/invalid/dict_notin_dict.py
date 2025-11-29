# Cannot use 'not in' with Dict[str, int] and Dict[str, int]
x: bool = {"a": 1} not in {"a": 1}
