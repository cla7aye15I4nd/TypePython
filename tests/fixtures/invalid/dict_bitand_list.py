# Cannot bitwise AND Dict[str, int] and List[int]
x: dict[str, int] = {"a": 1} & [1, 2, 3]
