# Cannot divide Dict[str, int] and Set[int]
x: float = {"a": 1} / {1, 2, 3}
