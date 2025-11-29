# Cannot use 'not in' with List[int] and Float
x: bool = [1, 2, 3] not in 1.0
