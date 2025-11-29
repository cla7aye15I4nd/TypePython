# Cannot compare with >= Dict[str, int] and Set[int]
x: bool = {"a": 1} >= {1, 2, 3}
