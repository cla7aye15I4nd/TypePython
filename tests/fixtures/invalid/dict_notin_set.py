# Cannot use 'not in' with Dict[str, int] and Set[int]
x: bool = {"a": 1} not in {1, 2, 3}
