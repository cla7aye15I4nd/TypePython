# Cannot use 'in' with Dict[str, int] and Set[int]
x: bool = {"a": 1} in {1, 2, 3}
