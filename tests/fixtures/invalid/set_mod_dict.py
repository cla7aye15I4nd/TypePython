# Cannot modulo Set[int] and Dict[str, int]
x: set[int] = {1, 2, 3} % {"a": 1}
