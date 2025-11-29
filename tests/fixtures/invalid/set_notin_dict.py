# Cannot use 'not in' with Set[int] and Dict[str, int]
x: bool = {1, 2, 3} not in {"a": 1}
