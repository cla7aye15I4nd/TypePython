# Cannot bitwise OR Dict[str, int] and Set[int]
x: dict[str, int] = {"a": 1} | {1, 2, 3}
