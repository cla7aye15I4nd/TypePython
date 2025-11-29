# Cannot use 'not in' with List[int] and Dict[str, int]
x: bool = [1, 2, 3] not in {"a": 1}
