# Cannot power List[int] and Dict[str, int]
x: list[int] = [1, 2, 3] ** {"a": 1}
