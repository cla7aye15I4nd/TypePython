# Cannot compare with < Dict[str, int] and List[int]
x: bool = {"a": 1} < [1, 2, 3]
