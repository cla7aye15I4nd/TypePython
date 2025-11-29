# Cannot use 'in' with Set[int] and Dict[str, int]
x: bool = {1, 2, 3} in {"a": 1}
