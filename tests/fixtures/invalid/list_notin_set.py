# Cannot use 'not in' with List[int] and Set[int]
x: bool = [1, 2, 3] not in {1, 2, 3}
